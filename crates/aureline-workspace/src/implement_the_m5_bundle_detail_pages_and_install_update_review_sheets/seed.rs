// Canonical seed for the M5 bundle detail / review primitive. Included from `mod.rs` so the
// seeded builder, its worked cases, the fixture generator, and the on-disk support export all
// stay byte-aligned.

/// Builds a reviewable component diff row (never an opaque blob).
fn diff_row(
    component_kind: BundleComponentKind,
    component_id: &str,
    label: &str,
    lifecycle_stage: LifecycleStage,
    diff_action: DiffAction,
    ownership: AssetOwnership,
    resolution: ResolutionChoice,
) -> ComponentDiffEntry {
    ComponentDiffEntry {
        component_kind,
        component_id: component_id.to_owned(),
        lifecycle_stage,
        requires_review: lifecycle_stage.is_non_stable(),
        diff_action,
        ownership,
        resolution,
        diffable: true,
        label: label.to_owned(),
        diff_preview_ref: format!("diff:preview:{component_id}"),
        local_override_ref: match ownership {
            AssetOwnership::LocallyOverridden => Some(format!("override:{component_id}")),
            _ => None,
        },
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

/// Builds a detail-page component-inventory summary.
fn summary(
    component_kind: BundleComponentKind,
    component_id: &str,
    label: &str,
    lifecycle_stage: LifecycleStage,
) -> M5BundleComponentSummary {
    M5BundleComponentSummary {
        component_kind,
        component_id: component_id.to_owned(),
        label: label.to_owned(),
        lifecycle_stage,
    }
}

/// A certified first-party bundle detail page + install review: full inventory, fresh
/// evidence, ready to apply.
fn detail_certified_input() -> M5BundleReviewInput {
    M5BundleReviewInput {
        review_id: "review:certified-rust-service:0001".to_owned(),
        surface_label: "Bundle detail page for a certified Rust service stack".to_owned(),
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
        changelog_ref: "changelog:rust-service:0001".to_owned(),
        evidence_link_refs: vec![
            "evidence:cert-proof:0001".to_owned(),
            "evidence:test-run:0001".to_owned(),
        ],
        component_inventory: vec![
            summary(
                BundleComponentKind::Extension,
                "ext.rust-analyzer",
                "Rust Analyzer extension",
                LifecycleStage::Stable,
            ),
            summary(
                BundleComponentKind::SettingsPreset,
                "preset.rustfmt",
                "rustfmt settings preset",
                LifecycleStage::Stable,
            ),
            summary(
                BundleComponentKind::TaskRecipe,
                "task.cargo-check",
                "cargo check task recipe",
                LifecycleStage::Stable,
            ),
            summary(
                BundleComponentKind::DocsPack,
                "docs.axum-guide",
                "Axum service docs pack",
                LifecycleStage::Stable,
            ),
            summary(
                BundleComponentKind::TemplateRef,
                "template.axum-service",
                "Axum service template",
                LifecycleStage::Stable,
            ),
        ],
        dependency_markers: vec![],
        operation: BundleReviewOperation::Install,
        diff_rows: vec![
            diff_row(
                BundleComponentKind::Extension,
                "ext.rust-analyzer",
                "Rust Analyzer extension",
                LifecycleStage::Stable,
                DiffAction::Added,
                AssetOwnership::BundleOwned,
                ResolutionChoice::AdoptBundle,
            ),
            diff_row(
                BundleComponentKind::SettingsPreset,
                "preset.rustfmt",
                "rustfmt settings preset",
                LifecycleStage::Stable,
                DiffAction::Added,
                AssetOwnership::BundleOwned,
                ResolutionChoice::AdoptBundle,
            ),
        ],
        side_effects: vec![
            M5BundleSideEffectClass::ExtensionInstall,
            M5BundleSideEffectClass::SettingsProfileWrite,
            M5BundleSideEffectClass::TaskRecipeRegistration,
        ],
        rollback_checkpoint: Some(checkpoint("checkpoint:install:0001", 5)),
        claims_no_change_despite_diff: false,
        claims_current_despite_stale: false,
        degraded: None,
    }
}

/// A managed, org-approved web-app install review: entitlement + policy-gated dependency
/// markers disclosed, ready to apply.
fn install_web_input() -> M5BundleReviewInput {
    M5BundleReviewInput {
        review_id: "review:managed-web-app:0002".to_owned(),
        surface_label: "Install review sheet for a managed-approved web-app stack".to_owned(),
        bundle_id_ref: "bundle:web-app:0002".to_owned(),
        bundle_name: "Managed Web App".to_owned(),
        bundle_class: BundleClass::OrgManagedBundle,
        signer_source: SourceTrust::TrustedRemote,
        support_class: LifecycleStage::PolicyGated,
        source_class: CertificationTarget::ManagedApproved,
        scorecard_class: BundleScorecardClass::Certified,
        certification_freshness: EvidenceFreshness::Fresh,
        imported_confidence: ImportedVsNativeConfidence::Native,
        compatible_aureline_range: ">=2026.7, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Live,
        changelog_ref: "changelog:web-app:0002".to_owned(),
        evidence_link_refs: vec!["evidence:policy-approval:0002".to_owned()],
        component_inventory: vec![
            summary(
                BundleComponentKind::Extension,
                "ext.web-tools",
                "Web tooling extension",
                LifecycleStage::PolicyGated,
            ),
            summary(
                BundleComponentKind::ProfilePreset,
                "preset.web-profile",
                "Web workspace profile preset",
                LifecycleStage::Stable,
            ),
            summary(
                BundleComponentKind::LaunchRecipe,
                "launch.dev-server",
                "Dev server launch recipe",
                LifecycleStage::Stable,
            ),
        ],
        dependency_markers: vec![
            M5BundleDependencyMarker::EntitlementRequired,
            M5BundleDependencyMarker::PolicyGated,
        ],
        operation: BundleReviewOperation::Install,
        diff_rows: vec![
            diff_row(
                BundleComponentKind::Extension,
                "ext.web-tools",
                "Web tooling extension",
                LifecycleStage::PolicyGated,
                DiffAction::Added,
                AssetOwnership::BundleOwned,
                ResolutionChoice::AdoptBundle,
            ),
            diff_row(
                BundleComponentKind::ProfilePreset,
                "preset.web-profile",
                "Web workspace profile preset",
                LifecycleStage::Stable,
                DiffAction::Added,
                AssetOwnership::BundleOwned,
                ResolutionChoice::AdoptBundle,
            ),
        ],
        side_effects: vec![
            M5BundleSideEffectClass::ExtensionInstall,
            M5BundleSideEffectClass::SettingsProfileWrite,
        ],
        rollback_checkpoint: Some(checkpoint("checkpoint:install:0002", 3)),
        claims_no_change_despite_diff: false,
        claims_current_despite_stale: false,
        degraded: None,
    }
}

/// A community framework-pack update review: a preview-stage dependency marker disclosed, a
/// user-adopted asset rebased, ready to apply.
fn update_framework_input() -> M5BundleReviewInput {
    M5BundleReviewInput {
        review_id: "review:framework-pack-update:0003".to_owned(),
        surface_label: "Update review sheet for a community framework pack".to_owned(),
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
        changelog_ref: "changelog:framework-pack:0003".to_owned(),
        evidence_link_refs: vec!["evidence:community-review:0003".to_owned()],
        component_inventory: vec![
            summary(
                BundleComponentKind::Extension,
                "ext.framework-lint",
                "Framework lint extension",
                LifecycleStage::Preview,
            ),
            summary(
                BundleComponentKind::TaskRecipe,
                "task.framework-build",
                "Framework build task recipe",
                LifecycleStage::Stable,
            ),
            summary(
                BundleComponentKind::TourPack,
                "tour.framework-intro",
                "Framework intro tour pack",
                LifecycleStage::Stable,
            ),
        ],
        dependency_markers: vec![M5BundleDependencyMarker::PreviewCapability],
        operation: BundleReviewOperation::Update,
        diff_rows: vec![
            diff_row(
                BundleComponentKind::Extension,
                "ext.framework-lint",
                "Framework lint extension",
                LifecycleStage::Preview,
                DiffAction::Modified,
                AssetOwnership::BundleOwned,
                ResolutionChoice::AdoptBundle,
            ),
            diff_row(
                BundleComponentKind::TaskRecipe,
                "task.framework-build",
                "Framework build task recipe",
                LifecycleStage::Stable,
                DiffAction::Modified,
                AssetOwnership::Adopted,
                ResolutionChoice::Rebase,
            ),
        ],
        side_effects: vec![
            M5BundleSideEffectClass::ExtensionInstall,
            M5BundleSideEffectClass::TaskRecipeRegistration,
            M5BundleSideEffectClass::DocsTourPackInstall,
        ],
        rollback_checkpoint: Some(checkpoint("checkpoint:update:0003", 3)),
        claims_no_change_despite_diff: false,
        claims_current_despite_stale: false,
        degraded: None,
    }
}

/// A managed update served from a mirror with a policy-blocked asset: still intelligible, but
/// constrained by policy.
fn policy_constrained_update_input() -> M5BundleReviewInput {
    M5BundleReviewInput {
        review_id: "review:policy-constrained-update:0004".to_owned(),
        surface_label: "Update review sheet blocked by org policy, served from a mirror".to_owned(),
        bundle_id_ref: "bundle:web-app:0004".to_owned(),
        bundle_name: "Managed Web App (mirror)".to_owned(),
        bundle_class: BundleClass::OrgManagedBundle,
        signer_source: SourceTrust::TrustedRemote,
        support_class: LifecycleStage::MirrorOnly,
        source_class: CertificationTarget::ManagedApproved,
        scorecard_class: BundleScorecardClass::Certified,
        certification_freshness: EvidenceFreshness::Aging,
        imported_confidence: ImportedVsNativeConfidence::Native,
        compatible_aureline_range: ">=2026.7, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Mirrored,
        changelog_ref: "changelog:web-app:0004".to_owned(),
        evidence_link_refs: vec!["evidence:policy-approval:0004".to_owned()],
        component_inventory: vec![
            summary(
                BundleComponentKind::Extension,
                "ext.web-tools",
                "Web tooling extension",
                LifecycleStage::PolicyGated,
            ),
            summary(
                BundleComponentKind::SettingsPreset,
                "preset.web-secure",
                "Secure web settings preset",
                LifecycleStage::PolicyGated,
            ),
        ],
        dependency_markers: vec![
            M5BundleDependencyMarker::PolicyGated,
            M5BundleDependencyMarker::MirrorOnlySource,
        ],
        operation: BundleReviewOperation::Update,
        diff_rows: vec![
            diff_row(
                BundleComponentKind::SettingsPreset,
                "preset.web-secure",
                "Secure web settings preset",
                LifecycleStage::PolicyGated,
                DiffAction::Modified,
                AssetOwnership::BlockedByPolicy,
                ResolutionChoice::Compare,
            ),
            diff_row(
                BundleComponentKind::Extension,
                "ext.web-tools",
                "Web tooling extension",
                LifecycleStage::PolicyGated,
                DiffAction::Modified,
                AssetOwnership::BundleOwned,
                ResolutionChoice::AdoptBundle,
            ),
        ],
        side_effects: vec![
            M5BundleSideEffectClass::ExtensionInstall,
            M5BundleSideEffectClass::SettingsProfileWrite,
        ],
        rollback_checkpoint: Some(checkpoint("checkpoint:update:0004", 2)),
        claims_no_change_despite_diff: false,
        claims_current_despite_stale: false,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::MirrorStale,
            degraded_label:
                "This managed update is served from a mirror whose evidence is aging and one asset is blocked by org policy; the review stays intelligible and keeps the blocked asset's compare-only resolution"
                    .to_owned(),
        }),
    }
}

/// A read-only drift review of a certified bundle with a locally overridden asset kept: a
/// comparison that mutates nothing (no rollback checkpoint).
fn drift_review_input() -> M5BundleReviewInput {
    M5BundleReviewInput {
        review_id: "review:drift-review:0005".to_owned(),
        surface_label: "Drift-review sheet comparing local state against a certified bundle"
            .to_owned(),
        bundle_id_ref: "bundle:rust-service:0005".to_owned(),
        bundle_name: "Rust Service Starter".to_owned(),
        bundle_class: BundleClass::LaunchBundle,
        signer_source: SourceTrust::FirstParty,
        support_class: LifecycleStage::Stable,
        source_class: CertificationTarget::Certified,
        scorecard_class: BundleScorecardClass::Certified,
        certification_freshness: EvidenceFreshness::Aging,
        imported_confidence: ImportedVsNativeConfidence::Native,
        compatible_aureline_range: ">=2026.6, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Live,
        changelog_ref: "changelog:rust-service:0005".to_owned(),
        evidence_link_refs: vec!["evidence:drift-scan:0005".to_owned()],
        component_inventory: vec![
            summary(
                BundleComponentKind::SettingsPreset,
                "preset.rustfmt",
                "rustfmt settings preset",
                LifecycleStage::Stable,
            ),
            summary(
                BundleComponentKind::TaskRecipe,
                "task.cargo-check",
                "cargo check task recipe",
                LifecycleStage::Stable,
            ),
        ],
        dependency_markers: vec![],
        operation: BundleReviewOperation::DriftReview,
        diff_rows: vec![diff_row(
            BundleComponentKind::SettingsPreset,
            "preset.rustfmt",
            "rustfmt settings preset",
            LifecycleStage::Stable,
            DiffAction::Modified,
            AssetOwnership::LocallyOverridden,
            ResolutionChoice::KeepLocal,
        )],
        side_effects: vec![],
        rollback_checkpoint: None,
        claims_no_change_despite_diff: false,
        claims_current_despite_stale: false,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
            degraded_label:
                "This bundle's local settings preset has diverged from the certified base; the drift review keeps the local override visible and mutates nothing"
                    .to_owned(),
        }),
    }
}

/// An imported bundle migration review: migration-mapping components with imported-not-native
/// provenance preserved.
fn imported_migration_input() -> M5BundleReviewInput {
    M5BundleReviewInput {
        review_id: "review:imported-migration:0006".to_owned(),
        surface_label: "Migration review view reconstructing an imported bundle's diff".to_owned(),
        bundle_id_ref: "bundle:monorepo:0006".to_owned(),
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
        changelog_ref: "changelog:monorepo:0006".to_owned(),
        evidence_link_refs: vec!["evidence:import-map:0006".to_owned()],
        component_inventory: vec![
            summary(
                BundleComponentKind::MigrationMapping,
                "migration.settings-map",
                "Imported settings migration mapping",
                LifecycleStage::Labs,
            ),
            summary(
                BundleComponentKind::MigrationMapping,
                "migration.task-map",
                "Imported task migration mapping",
                LifecycleStage::Stable,
            ),
            summary(
                BundleComponentKind::ScaffoldRef,
                "scaffold.monorepo",
                "Monorepo scaffold generator",
                LifecycleStage::Stable,
            ),
        ],
        dependency_markers: vec![M5BundleDependencyMarker::LabsCapability],
        operation: BundleReviewOperation::Update,
        diff_rows: vec![
            diff_row(
                BundleComponentKind::MigrationMapping,
                "migration.settings-map",
                "Imported settings migration mapping",
                LifecycleStage::Labs,
                DiffAction::Added,
                AssetOwnership::BundleOwned,
                ResolutionChoice::AdoptBundle,
            ),
            diff_row(
                BundleComponentKind::ScaffoldRef,
                "scaffold.monorepo",
                "Monorepo scaffold generator",
                LifecycleStage::Stable,
                DiffAction::Modified,
                AssetOwnership::LocallyOverridden,
                ResolutionChoice::KeepLocal,
            ),
        ],
        side_effects: vec![
            M5BundleSideEffectClass::ScaffoldGeneration,
            M5BundleSideEffectClass::SettingsProfileWrite,
        ],
        rollback_checkpoint: Some(checkpoint("checkpoint:migration:0006", 3)),
        claims_no_change_despite_diff: false,
        claims_current_despite_stale: false,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::ImportedNotNative,
            degraded_label:
                "This bundle was imported from another setup and its certification is stale; the migration review keeps the imported-not-native provenance and preserves the local override"
                    .to_owned(),
        }),
    }
}

/// A support / export replay reconstructed from a certified imported snapshot: full inventory,
/// fresh snapshot evidence, ready to apply.
fn support_replay_input() -> M5BundleReviewInput {
    M5BundleReviewInput {
        review_id: "review:support-replay:0007".to_owned(),
        surface_label: "Support / export replay reconstructing a review snapshot".to_owned(),
        bundle_id_ref: "bundle:snapshot:0007".to_owned(),
        bundle_name: "Snapshot Rust Service".to_owned(),
        bundle_class: BundleClass::LaunchBundle,
        signer_source: SourceTrust::FirstParty,
        support_class: LifecycleStage::Stable,
        source_class: CertificationTarget::Certified,
        scorecard_class: BundleScorecardClass::Certified,
        certification_freshness: EvidenceFreshness::Fresh,
        imported_confidence: ImportedVsNativeConfidence::Native,
        compatible_aureline_range: ">=2026.6, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::Imported,
        changelog_ref: "changelog:snapshot:0007".to_owned(),
        evidence_link_refs: vec!["evidence:snapshot-proof:0007".to_owned()],
        component_inventory: vec![
            summary(
                BundleComponentKind::Extension,
                "ext.rust-analyzer",
                "Rust Analyzer extension",
                LifecycleStage::Stable,
            ),
            summary(
                BundleComponentKind::DebugRecipe,
                "debug.lldb",
                "LLDB debug recipe",
                LifecycleStage::Stable,
            ),
        ],
        dependency_markers: vec![],
        operation: BundleReviewOperation::Install,
        diff_rows: vec![diff_row(
            BundleComponentKind::Extension,
            "ext.rust-analyzer",
            "Rust Analyzer extension",
            LifecycleStage::Stable,
            DiffAction::Added,
            AssetOwnership::BundleOwned,
            ResolutionChoice::AdoptBundle,
        )],
        side_effects: vec![M5BundleSideEffectClass::ExtensionInstall],
        rollback_checkpoint: Some(checkpoint("checkpoint:install:0007", 2)),
        claims_no_change_despite_diff: false,
        claims_current_despite_stale: false,
        degraded: None,
    }
}

/// A community update served from an offline cache: still intelligible under offline, ready to
/// apply against the cached snapshot.
fn offline_update_input() -> M5BundleReviewInput {
    M5BundleReviewInput {
        review_id: "review:offline-update:0008".to_owned(),
        surface_label: "Update review sheet reconstructed from an offline cache".to_owned(),
        bundle_id_ref: "bundle:framework-pack:0008".to_owned(),
        bundle_name: "Community Framework Pack (offline)".to_owned(),
        bundle_class: BundleClass::FrameworkPack,
        signer_source: SourceTrust::UnverifiedRemote,
        support_class: LifecycleStage::Preview,
        source_class: CertificationTarget::CommunityReviewed,
        scorecard_class: BundleScorecardClass::Community,
        certification_freshness: EvidenceFreshness::Aging,
        imported_confidence: ImportedVsNativeConfidence::Bridged,
        compatible_aureline_range: ">=2026.4, <2027.0".to_owned(),
        truth_mode: M5BundleTruthMode::CachedOffline,
        changelog_ref: "changelog:framework-pack:0008".to_owned(),
        evidence_link_refs: vec!["evidence:offline-snapshot:0008".to_owned()],
        component_inventory: vec![summary(
            BundleComponentKind::TaskRecipe,
            "task.framework-build",
            "Framework build task recipe",
            LifecycleStage::Stable,
        )],
        dependency_markers: vec![M5BundleDependencyMarker::PreviewCapability],
        operation: BundleReviewOperation::Update,
        diff_rows: vec![diff_row(
            BundleComponentKind::TaskRecipe,
            "task.framework-build",
            "Framework build task recipe",
            LifecycleStage::Stable,
            DiffAction::Modified,
            AssetOwnership::BundleOwned,
            ResolutionChoice::AdoptBundle,
        )],
        side_effects: vec![M5BundleSideEffectClass::TaskRecipeRegistration],
        rollback_checkpoint: Some(checkpoint("checkpoint:update:0008", 1)),
        claims_no_change_despite_diff: false,
        claims_current_despite_stale: false,
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::OfflineCacheOnly,
            degraded_label:
                "This community update is reconstructed from an offline cache; the review stays intelligible and names the offline-cache provenance before applying"
                    .to_owned(),
        }),
    }
}

fn case(input: M5BundleReviewInput) -> M5BundleReviewCase {
    M5BundleReviewCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5BundleReviewSurfaceRow> {
    let base_source_refs = vec![
        M5_BUNDLE_REVIEW_SCHEMA_REF.to_owned(),
        M5_BUNDLE_REVIEW_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5BundleReviewExportField::ALL.to_vec();

    vec![
        M5BundleReviewSurfaceRow {
            surface_family: M5BundleReviewSurfaceFamily::BundleDetailPage,
            owner_role: "Bundle detail guild".to_owned(),
            scope_summary: "Bundle detail page listing extensions, presets, tasks, docs, templates, dependency markers, mirror/offline posture, and changelog"
                .to_owned(),
            operations: vec![BundleReviewOperation::Install],
            source_classes: vec![CertificationTarget::Certified],
            truth_modes: vec![M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::StaleCertification,
                M5BundleComponentDowngradeTrigger::IncompatibleAureline,
            ],
            consumer_surfaces: vec!["bundle_detail".to_owned(), "docs_onboarding".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_reviews: vec![case(detail_certified_input())],
            hides_diff_scope: false,
            applies_before_review: false,
            hides_dependency_markers: false,
        },
        M5BundleReviewSurfaceRow {
            surface_family: M5BundleReviewSurfaceFamily::InstallReviewSheet,
            owner_role: "Install-review guild".to_owned(),
            scope_summary: "Install review sheet enumerating added components, side effects, entitlement/policy dependency markers, and rollback checkpoint creation"
                .to_owned(),
            operations: vec![BundleReviewOperation::Install],
            source_classes: vec![CertificationTarget::ManagedApproved],
            truth_modes: vec![M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::EntitlementDependencyUnmet,
                M5BundleComponentDowngradeTrigger::StaleCertification,
            ],
            consumer_surfaces: vec!["install_review".to_owned(), "start_center".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_reviews: vec![case(install_web_input())],
            hides_diff_scope: false,
            applies_before_review: false,
            hides_dependency_markers: false,
        },
        M5BundleReviewSurfaceRow {
            surface_family: M5BundleReviewSurfaceFamily::UpdateReviewSheet,
            owner_role: "Update-review guild".to_owned(),
            scope_summary: "Update review sheet enumerating changed components, preserving policy-blocked and adopted assets, and deriving a review posture"
                .to_owned(),
            operations: vec![BundleReviewOperation::Update],
            source_classes: vec![
                CertificationTarget::CommunityReviewed,
                CertificationTarget::ManagedApproved,
            ],
            truth_modes: vec![M5BundleTruthMode::Live, M5BundleTruthMode::Mirrored],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::MirrorStale,
                M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
            ],
            consumer_surfaces: vec!["update_review".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_reviews: vec![
                case(update_framework_input()),
                case(policy_constrained_update_input()),
            ],
            hides_diff_scope: false,
            applies_before_review: false,
            hides_dependency_markers: false,
        },
        M5BundleReviewSurfaceRow {
            surface_family: M5BundleReviewSurfaceFamily::DriftReviewSheet,
            owner_role: "Drift-review guild".to_owned(),
            scope_summary: "Drift-review sheet comparing local state against the bundle read-only and keeping local overrides visible"
                .to_owned(),
            operations: vec![BundleReviewOperation::DriftReview],
            source_classes: vec![CertificationTarget::Certified],
            truth_modes: vec![M5BundleTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::LocalOverrideDrift,
                M5BundleComponentDowngradeTrigger::StaleCertification,
            ],
            consumer_surfaces: vec!["drift_review".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_reviews: vec![case(drift_review_input())],
            hides_diff_scope: false,
            applies_before_review: false,
            hides_dependency_markers: false,
        },
        M5BundleReviewSurfaceRow {
            surface_family: M5BundleReviewSurfaceFamily::MigrationReviewView,
            owner_role: "Migration-review guild".to_owned(),
            scope_summary: "Migration review view reconstructing an imported bundle's diffed truth and preserving imported-not-native provenance"
                .to_owned(),
            operations: vec![BundleReviewOperation::Update],
            source_classes: vec![CertificationTarget::ImportedPendingReview],
            truth_modes: vec![M5BundleTruthMode::Imported],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::ImportedNotNative,
                M5BundleComponentDowngradeTrigger::StaleCertification,
            ],
            consumer_surfaces: vec!["migration_review".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_reviews: vec![case(imported_migration_input())],
            hides_diff_scope: false,
            applies_before_review: false,
            hides_dependency_markers: false,
        },
        M5BundleReviewSurfaceRow {
            surface_family: M5BundleReviewSurfaceFamily::SupportExportReplay,
            owner_role: "Support / export guild".to_owned(),
            scope_summary: "Offline replay reconstructing review truth from a certified snapshot and an offline-cache update"
                .to_owned(),
            operations: vec![BundleReviewOperation::Install, BundleReviewOperation::Update],
            source_classes: vec![
                CertificationTarget::Certified,
                CertificationTarget::CommunityReviewed,
            ],
            truth_modes: vec![M5BundleTruthMode::Imported, M5BundleTruthMode::CachedOffline],
            export_fields: all_export_fields,
            downgrade_triggers: vec![
                M5BundleComponentDowngradeTrigger::OfflineCacheOnly,
                M5BundleComponentDowngradeTrigger::ImportedNotNative,
            ],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs,
            example_reviews: vec![case(support_replay_input()), case(offline_update_input())],
            hides_diff_scope: false,
            applies_before_review: false,
            hides_dependency_markers: false,
        },
    ]
}

fn seeded_governance_review() -> M5BundleReviewGovernanceReview {
    M5BundleReviewGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        review_identity_preserved_across_surfaces: true,
        change_disclosed_before_apply: true,
        review_intelligible_under_constraint: true,
        rollback_checkpoint_created_before_mutation: true,
        support_export_reconstructs_review: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5BundleReviewConsumerProjection {
    M5BundleReviewConsumerProjection {
        review_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        review_sheet_reads_single_diff_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5BundleReviewReleasePosture {
    M5BundleReviewReleasePosture {
        release_packet_ref: M5_BUNDLE_REVIEW_ARTIFACT_REF.to_owned(),
        review_audit_ref: M5_BUNDLE_REVIEW_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 bundle detail / review primitive packet. This is the one
/// source of truth shared by the tests, the fixture generator, and the on-disk support export so
/// all three stay byte-aligned.
pub fn seeded_m5_bundle_detail_review_packet() -> M5BundleDetailReviewPacket {
    M5BundleDetailReviewPacket::new(M5BundleDetailReviewPacketInput {
        packet_id: "m5-bundle-detail-review-primitive:stable:0001".to_owned(),
        matrix_label:
            "M5 Bundle Detail / Review Primitive: Bundle Detail Page and Install / Update Review Sheet"
                .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5BundleReviewVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_BUNDLE_REVIEW_SCHEMA_REF.to_owned(),
            M5_BUNDLE_REVIEW_DOC_REF.to_owned(),
            M5_BUNDLE_REVIEW_COMPONENT_MATRIX_REF.to_owned(),
            M5_BUNDLE_REVIEW_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "workflow_bundle_component_boundary_v1".to_owned(),
        minted_at: "2026-07-06T00:00:00Z".to_owned(),
    })
}
