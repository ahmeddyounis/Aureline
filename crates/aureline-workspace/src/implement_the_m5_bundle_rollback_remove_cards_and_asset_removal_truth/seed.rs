// Canonical seed for the M5 bundle rollback / remove primitive. Included from `mod.rs` so the
// seeded builder, its worked cases, the fixture generator, and the on-disk support export all stay
// byte-aligned.

/// Builds one removal-inventory asset. The safe-to-remove class and disposition are supplied so a
/// row can be built at any granularity; the resolver still enforces that they are honest for the
/// asset's origin and ownership.
#[allow(clippy::too_many_arguments)]
fn asset(
    origin: M5RemovalAssetOrigin,
    component_kind: BundleComponentKind,
    target_ref: &str,
    label: &str,
    ownership: AssetOwnership,
    safe_to_remove_class: M5SafeToRemoveClass,
    disposition: M5RemovalDisposition,
    explicitly_selected_for_removal: bool,
) -> M5RemovalAsset {
    M5RemovalAsset {
        origin,
        component_kind,
        target_ref: target_ref.to_owned(),
        label: label.to_owned(),
        ownership,
        safe_to_remove_class,
        disposition,
        explicitly_selected_for_removal,
    }
}

/// A bundle-created asset that is reverted with the bundle.
fn bundle_created(
    component_kind: BundleComponentKind,
    target_ref: &str,
    label: &str,
    ownership: AssetOwnership,
) -> M5RemovalAsset {
    asset(
        M5RemovalAssetOrigin::BundleCreated,
        component_kind,
        target_ref,
        label,
        ownership,
        M5SafeToRemoveClass::SafeToRemove,
        M5RemovalDisposition::Reverted,
        false,
    )
}

/// A user-owned asset kept local across the removal.
fn kept_local(
    origin: M5RemovalAssetOrigin,
    component_kind: BundleComponentKind,
    target_ref: &str,
    label: &str,
    ownership: AssetOwnership,
) -> M5RemovalAsset {
    asset(
        origin,
        component_kind,
        target_ref,
        label,
        ownership,
        M5SafeToRemoveClass::KeepLocal,
        M5RemovalDisposition::KeptLocal,
        false,
    )
}

/// A user-owned asset whose removal has dependents and must be handled manually.
fn manual_follow_up(
    origin: M5RemovalAssetOrigin,
    component_kind: BundleComponentKind,
    target_ref: &str,
    label: &str,
    ownership: AssetOwnership,
) -> M5RemovalAsset {
    asset(
        origin,
        component_kind,
        target_ref,
        label,
        ownership,
        M5SafeToRemoveClass::RequiresManualHandling,
        M5RemovalDisposition::ManualFollowUp,
        false,
    )
}

/// Builds a one-step, pre-mutation checkpoint restore path.
fn checkpoint(checkpoint_ref: &str, captured_component_count: usize) -> RollbackCheckpoint {
    RollbackCheckpoint {
        checkpoint_ref: checkpoint_ref.to_owned(),
        one_step: true,
        reversible: true,
        captured_before_mutation: true,
        captured_component_count,
    }
}

/// Builds an available export-before-remove action that captures user-owned state.
fn export_before_remove(export_ref: &str) -> M5ExportBeforeRemove {
    M5ExportBeforeRemove {
        export_ref: export_ref.to_owned(),
        format_label: "support_bundle_json".to_owned(),
        captures_user_assets: true,
        available: true,
    }
}

/// A full guided-stack rollback: bundle-created toolchain config reverted while the user's profile
/// and an authored file survive — bundle-created cleanup separated from preserved user work.
fn workspace_rollback_input() -> M5BundleRemovalInput {
    M5BundleRemovalInput {
        removal_id: "removal:rust-service:0001".to_owned(),
        surface_label: "Workspace rollback card backing out a certified Rust service stack"
            .to_owned(),
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
        operation: BundleReviewOperation::Remove,
        assets: vec![
            bundle_created(
                BundleComponentKind::SettingsPreset,
                "asset:bundle.rustfmt-preset",
                "Bundle-provided rustfmt preset is reverted with the bundle",
                AssetOwnership::Removable,
            ),
            kept_local(
                M5RemovalAssetOrigin::UserProfile,
                BundleComponentKind::ProfilePreset,
                "asset:user.editor-profile",
                "User editor profile is kept local",
                AssetOwnership::LocallyOverridden,
            ),
            kept_local(
                M5RemovalAssetOrigin::UserCreatedFile,
                BundleComponentKind::TemplateRef,
                "asset:user.scratch-notes",
                "User-authored scratch notes file is kept local",
                AssetOwnership::Adopted,
            ),
        ],
        side_effects: vec![
            M5BundleSideEffectClass::SettingsProfileWrite,
            M5BundleSideEffectClass::ExtensionInstall,
        ],
        rollback_checkpoint: Some(checkpoint("checkpoint:rollback:0001", 3)),
        export_before_remove: Some(export_before_remove("export:rollback:0001")),
        reads_like_destructive_cleanup: false,
        claims_current_despite_stale: false,
        forces_reset_to_export: false,
        degraded: None,
    }
}

/// Removing a managed bundle where an adopted package has dependents and must be handled manually:
/// bundle-created extension reverted, adopted package flagged for manual follow-up.
fn detail_remove_input() -> M5BundleRemovalInput {
    M5BundleRemovalInput {
        removal_id: "removal:web-app:0002".to_owned(),
        surface_label: "Bundle detail remove panel for a managed web app".to_owned(),
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
        operation: BundleReviewOperation::Remove,
        assets: vec![
            bundle_created(
                BundleComponentKind::Extension,
                "asset:bundle.web-tools-ext",
                "Bundle-owned web tooling extension is reverted with the bundle",
                AssetOwnership::Removable,
            ),
            manual_follow_up(
                M5RemovalAssetOrigin::AdoptedPackage,
                BundleComponentKind::Extension,
                "asset:adopted.db-driver",
                "Adopted database driver has dependents; handle removal manually",
                AssetOwnership::Adopted,
            ),
        ],
        side_effects: vec![M5BundleSideEffectClass::ExtensionInstall],
        rollback_checkpoint: Some(checkpoint("checkpoint:remove:0002", 2)),
        export_before_remove: Some(export_before_remove("export:remove:0002")),
        reads_like_destructive_cleanup: false,
        claims_current_despite_stale: false,
        forces_reset_to_export: false,
        degraded: None,
    }
}

/// Removing a single extension while the user's local history survives: a small bundle-created
/// revert that never touches the user's accrued history.
fn extension_remove_input() -> M5BundleRemovalInput {
    M5BundleRemovalInput {
        removal_id: "removal:framework-lint:0003".to_owned(),
        surface_label: "Extension remove row for a community framework lint extension".to_owned(),
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
        operation: BundleReviewOperation::Remove,
        assets: vec![
            bundle_created(
                BundleComponentKind::Extension,
                "asset:bundle.framework-lint-ext",
                "Bundle-owned framework lint extension is reverted with the bundle",
                AssetOwnership::Removable,
            ),
            kept_local(
                M5RemovalAssetOrigin::LocalHistory,
                BundleComponentKind::TemplateRef,
                "asset:user.local-history",
                "User local history / timeline is kept local",
                AssetOwnership::Adopted,
            ),
        ],
        side_effects: vec![M5BundleSideEffectClass::ExtensionInstall],
        rollback_checkpoint: Some(checkpoint("checkpoint:remove:0003", 2)),
        export_before_remove: Some(export_before_remove("export:remove:0003")),
        reads_like_destructive_cleanup: false,
        claims_current_despite_stale: false,
        forces_reset_to_export: false,
        degraded: None,
    }
}

/// A read-only removal preview of an imported bundle: imported settings kept, an adopted package
/// flagged for manual handling, export-before-remove available even without a mutation.
fn migration_rollback_preview_input() -> M5BundleRemovalInput {
    M5BundleRemovalInput {
        removal_id: "removal:imported-monorepo:0004".to_owned(),
        surface_label: "Migration rollback view previewing removal of an imported bundle".to_owned(),
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
        operation: BundleReviewOperation::DriftReview,
        assets: vec![
            kept_local(
                M5RemovalAssetOrigin::ImportedSetting,
                BundleComponentKind::SettingsPreset,
                "asset:imported.editor-settings",
                "Imported editor settings are kept local",
                AssetOwnership::LocallyOverridden,
            ),
            manual_follow_up(
                M5RemovalAssetOrigin::AdoptedPackage,
                BundleComponentKind::Extension,
                "asset:adopted.legacy-linter",
                "Adopted legacy linter has dependents; handle removal manually",
                AssetOwnership::Adopted,
            ),
        ],
        side_effects: vec![],
        rollback_checkpoint: None,
        export_before_remove: Some(export_before_remove("export:preview:0004")),
        reads_like_destructive_cleanup: false,
        claims_current_despite_stale: false,
        forces_reset_to_export: false,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::ImportedNotNative,
            degraded_label:
                "This bundle was imported from another setup and its certification is stale; the preview keeps imported settings local and flags the adopted linter for manual handling before anything is removed"
                    .to_owned(),
        }),
    }
}

/// A mirror-served managed removal where the user explicitly selects an adopted package for
/// removal: the explicit-selection carve-out reverts a user-owned asset honestly.
fn diagnostics_explicit_remove_input() -> M5BundleRemovalInput {
    M5BundleRemovalInput {
        removal_id: "removal:managed-mirror:0005".to_owned(),
        surface_label: "Diagnostics removal report served from a mirror with an explicit selection"
            .to_owned(),
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
        operation: BundleReviewOperation::Remove,
        assets: vec![
            bundle_created(
                BundleComponentKind::Extension,
                "asset:bundle.web-secure-ext",
                "Bundle-owned secure web extension is reverted with the bundle",
                AssetOwnership::Removable,
            ),
            asset(
                M5RemovalAssetOrigin::AdoptedPackage,
                BundleComponentKind::Extension,
                "asset:adopted.metrics-agent",
                "Adopted metrics agent is explicitly selected for removal",
                AssetOwnership::Adopted,
                M5SafeToRemoveClass::KeepLocal,
                M5RemovalDisposition::Reverted,
                true,
            ),
        ],
        side_effects: vec![M5BundleSideEffectClass::ExtensionInstall],
        rollback_checkpoint: Some(checkpoint("checkpoint:remove:0005", 2)),
        export_before_remove: Some(export_before_remove("export:remove:0005")),
        reads_like_destructive_cleanup: false,
        claims_current_despite_stale: false,
        forces_reset_to_export: false,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::MirrorStale,
            degraded_label:
                "This managed bundle is served from a mirror whose evidence is aging; the report reverts the bundle-owned extension and the adopted metrics agent the user explicitly selected, and keeps the checkpoint restore and export before removing anything"
                    .to_owned(),
        }),
    }
}

/// A read-only removal preview from a diagnostics report: bundle-created revert plus a user
/// profile kept local, export available without a mutation.
fn diagnostics_preview_input() -> M5BundleRemovalInput {
    M5BundleRemovalInput {
        removal_id: "removal:diagnostics-preview:0006".to_owned(),
        surface_label: "Diagnostics removal report previewing a bundle removal".to_owned(),
        bundle_id_ref: "bundle:rust-service:0006".to_owned(),
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
        operation: BundleReviewOperation::DriftReview,
        assets: vec![
            bundle_created(
                BundleComponentKind::TaskRecipe,
                "asset:bundle.cargo-check-task",
                "Bundle-owned cargo check task is reverted with the bundle",
                AssetOwnership::BundleOwned,
            ),
            kept_local(
                M5RemovalAssetOrigin::UserProfile,
                BundleComponentKind::LayoutPreset,
                "asset:user.layout-preset",
                "User layout preset is kept local",
                AssetOwnership::LocallyOverridden,
            ),
        ],
        side_effects: vec![],
        rollback_checkpoint: None,
        export_before_remove: Some(export_before_remove("export:preview:0006")),
        reads_like_destructive_cleanup: false,
        claims_current_despite_stale: false,
        forces_reset_to_export: false,
        degraded: None,
    }
}

/// A community bundle removal reconstructed from an offline cache for support replay: bundle-owned
/// revert plus imported settings kept local, replayed offline with export available.
fn support_replay_input() -> M5BundleRemovalInput {
    M5BundleRemovalInput {
        removal_id: "removal:offline-replay:0007".to_owned(),
        surface_label: "Support / export replay reconstructing removal truth from an offline cache"
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
        operation: BundleReviewOperation::DriftReview,
        assets: vec![
            bundle_created(
                BundleComponentKind::DocsPack,
                "asset:bundle.framework-docs",
                "Bundle-owned framework docs pack is reverted with the bundle",
                AssetOwnership::Removable,
            ),
            kept_local(
                M5RemovalAssetOrigin::ImportedSetting,
                BundleComponentKind::SettingsPreset,
                "asset:imported.build-settings",
                "Imported build settings are kept local",
                AssetOwnership::LocallyOverridden,
            ),
        ],
        side_effects: vec![],
        rollback_checkpoint: None,
        export_before_remove: Some(export_before_remove("export:replay:0007")),
        reads_like_destructive_cleanup: false,
        claims_current_despite_stale: false,
        forces_reset_to_export: false,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::OfflineCacheOnly,
            degraded_label:
                "This community bundle's removal is reconstructed from an offline cache with stale certification; the replay stays intelligible, keeps the imported build settings local, and names the offline-cache provenance"
                    .to_owned(),
        }),
    }
}

fn case(input: M5BundleRemovalInput) -> M5BundleRemovalCase {
    M5BundleRemovalCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5BundleRemovalSurfaceRow> {
    let base_source_refs = vec![
        M5_BUNDLE_REMOVAL_SCHEMA_REF.to_owned(),
        M5_BUNDLE_REMOVAL_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5BundleRemovalExportField::ALL.to_vec();

    vec![
        M5BundleRemovalSurfaceRow {
            surface_family: M5BundleRemovalSurfaceFamily::WorkspaceRollbackCard,
            owner_role: "Workspace rollback guild".to_owned(),
            scope_summary: "Workspace rollback card reverting bundle-created config while keeping the user's profile and authored files local"
                .to_owned(),
            operations: vec![BundleReviewOperation::Remove],
            source_classes: vec![CertificationTarget::Certified],
            truth_modes: vec![M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::RollbackOnlyPath,
                M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
            ],
            consumer_surfaces: vec!["workspace_shell".to_owned(), "start_center".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_removals: vec![case(workspace_rollback_input())],
            implies_destructive_cleanup: false,
            collapses_to_opaque_removal: false,
            forces_reset_to_export: false,
        },
        M5BundleRemovalSurfaceRow {
            surface_family: M5BundleRemovalSurfaceFamily::BundleDetailRemovePanel,
            owner_role: "Bundle detail guild".to_owned(),
            scope_summary: "Bundle detail remove panel reverting a bundle-owned extension and flagging an adopted package with dependents for manual follow-up"
                .to_owned(),
            operations: vec![BundleReviewOperation::Remove],
            source_classes: vec![CertificationTarget::ManagedApproved],
            truth_modes: vec![M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::RollbackOnlyPath,
                M5BundleComponentDowngradeTrigger::EntitlementDependencyUnmet,
            ],
            consumer_surfaces: vec!["bundle_detail".to_owned(), "docs_bundles".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_removals: vec![case(detail_remove_input())],
            implies_destructive_cleanup: false,
            collapses_to_opaque_removal: false,
            forces_reset_to_export: false,
        },
        M5BundleRemovalSurfaceRow {
            surface_family: M5BundleRemovalSurfaceFamily::ExtensionRemoveRow,
            owner_role: "Extension lifecycle guild".to_owned(),
            scope_summary: "Extension remove row reverting a bundle-owned extension while keeping the user's local history"
                .to_owned(),
            operations: vec![BundleReviewOperation::Remove],
            source_classes: vec![CertificationTarget::CommunityReviewed],
            truth_modes: vec![M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::RollbackOnlyPath,
                M5BundleComponentDowngradeTrigger::UnverifiedSigner,
            ],
            consumer_surfaces: vec!["extension_list".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_removals: vec![case(extension_remove_input())],
            implies_destructive_cleanup: false,
            collapses_to_opaque_removal: false,
            forces_reset_to_export: false,
        },
        M5BundleRemovalSurfaceRow {
            surface_family: M5BundleRemovalSurfaceFamily::MigrationRollbackView,
            owner_role: "Migration rollback guild".to_owned(),
            scope_summary: "Migration rollback view previewing removal of an imported bundle, keeping imported settings and flagging an adopted package for manual handling"
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
            example_removals: vec![case(migration_rollback_preview_input())],
            implies_destructive_cleanup: false,
            collapses_to_opaque_removal: false,
            forces_reset_to_export: false,
        },
        M5BundleRemovalSurfaceRow {
            surface_family: M5BundleRemovalSurfaceFamily::DiagnosticsRemovalReport,
            owner_role: "Diagnostics removal guild".to_owned(),
            scope_summary: "Diagnostics removal report covering a mirror-served explicit removal and a read-only removal preview, both keeping checkpoint restore and export before removing anything"
                .to_owned(),
            operations: vec![BundleReviewOperation::Remove, BundleReviewOperation::DriftReview],
            source_classes: vec![
                CertificationTarget::ManagedApproved,
                CertificationTarget::Certified,
            ],
            truth_modes: vec![M5BundleTruthMode::Mirrored, M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::MirrorStale,
                M5BundleComponentDowngradeTrigger::RollbackOnlyPath,
            ],
            consumer_surfaces: vec!["diagnostics".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_removals: vec![
                case(diagnostics_explicit_remove_input()),
                case(diagnostics_preview_input()),
            ],
            implies_destructive_cleanup: false,
            collapses_to_opaque_removal: false,
            forces_reset_to_export: false,
        },
        M5BundleRemovalSurfaceRow {
            surface_family: M5BundleRemovalSurfaceFamily::SupportExportReplay,
            owner_role: "Support / export guild".to_owned(),
            scope_summary: "Offline replay reconstructing removal truth from an offline cache with a stale-certification narrowing, keeping imported settings local"
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
            example_removals: vec![case(support_replay_input())],
            implies_destructive_cleanup: false,
            collapses_to_opaque_removal: false,
            forces_reset_to_export: false,
        },
    ]
}

fn seeded_governance_review() -> M5BundleRemovalGovernanceReview {
    M5BundleRemovalGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        removal_identity_preserved_across_surfaces: true,
        removal_non_destructive_of_user_work: true,
        states_remains_reverted_manual: true,
        created_versus_adopted_distinguished: true,
        checkpoint_restore_before_mutation: true,
        export_before_remove_available_when_narrowing: true,
        support_export_reconstructs_removal: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5BundleRemovalConsumerProjection {
    M5BundleRemovalConsumerProjection {
        removal_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        inventory_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5BundleRemovalReleasePosture {
    M5BundleRemovalReleasePosture {
        release_packet_ref: M5_BUNDLE_REMOVAL_ARTIFACT_REF.to_owned(),
        removal_audit_ref: M5_BUNDLE_REMOVAL_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 bundle rollback / remove primitive packet. This is the one
/// source of truth shared by the tests, the fixture generator, and the on-disk support export so
/// all three stay byte-aligned.
pub fn seeded_m5_bundle_rollback_remove_packet() -> M5BundleRollbackRemovePacket {
    M5BundleRollbackRemovePacket::new(M5BundleRollbackRemovePacketInput {
        packet_id: "m5-bundle-rollback-remove-primitive:stable:0001".to_owned(),
        matrix_label:
            "M5 Bundle Rollback / Remove Primitive: Rollback / Remove Card, Created-versus-Adopted Asset Inventory, and Restore Path"
                .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5BundleRemovalVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_BUNDLE_REMOVAL_SCHEMA_REF.to_owned(),
            M5_BUNDLE_REMOVAL_DOC_REF.to_owned(),
            M5_BUNDLE_REMOVAL_COMPONENT_MATRIX_REF.to_owned(),
            M5_BUNDLE_REMOVAL_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "workflow_bundle_component_boundary_v1".to_owned(),
        minted_at: "2026-07-06T00:00:00Z".to_owned(),
    })
}
