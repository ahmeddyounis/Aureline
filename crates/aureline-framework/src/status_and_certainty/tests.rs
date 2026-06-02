use super::*;

fn healthy_pack_strip() -> FrameworkSupportStrip {
    FrameworkSupportStrip {
        record_kind: FRAMEWORK_SUPPORT_STRIP_RECORD_KIND.to_owned(),
        framework_support_strip_schema_version: FRAMEWORK_SUPPORT_STRIP_SCHEMA_VERSION,
        framework_support_strip_id: "framework.strip.tests.healthy_pack".to_owned(),
        captured_at: "2026-05-18T09:00:00Z".to_owned(),
        surface_class: SurfaceClass::RouteExplorerSurface,
        framework_identity_block: FrameworkIdentityBlock {
            framework_family_class: FrameworkFamilyClass::ReactOrJsxFamily,
            framework_name_label: "Next.js".to_owned(),
            framework_version_label: Some("16.x".to_owned()),
            framework_version_range_label: Some("16.0–16.x".to_owned()),
        },
        support_class: SupportClass::FrameworkPack,
        pack_or_bridge_source_block: PackOrBridgeSourceBlock {
            pack_source_class: PackSourceClass::GovernedFrameworkPack,
            pack_ref: Some("framework.pack.next_app_router.v3".to_owned()),
            pack_version_label: Some("v3.4".to_owned()),
            bridge_ref: None,
            bridge_label: None,
        },
        health_block: HealthBlock {
            health_class: HealthClass::HealthyLive,
            freshness_class: FreshnessClass::AuthoritativeLive,
            last_refresh_at: Some("2026-05-18T08:59:00Z".to_owned()),
        },
        scope_block: ScopeBlock {
            locality_class: LocalityClass::LocalWorkspace,
            workspace_scope_label: Some("Workspace HEAD".to_owned()),
            workspace_scope_ref: Some("workspace.head".to_owned()),
        },
        compatibility_block: CompatibilityBlock {
            version_compatibility_class: VersionCompatibilityClass::WithinSupportedRange,
            supported_version_range_label: Some("16.0–16.x".to_owned()),
            downgraded_behavior_summary: None,
        },
        actions: vec![
            FrameworkSupportActionClass::OpenCompatibilityDetails,
            FrameworkSupportActionClass::OpenPackDocs,
            FrameworkSupportActionClass::OpenPackStatus,
        ],
        framework_certainty_row_record_ref: Some("framework.row.route.proven.0001".to_owned()),
        source_sync_chip_record_ref: Some("framework.chip.in_sync.0001".to_owned()),
        summary: "Next.js App Router framework pack v3.4 is healthy on workspace HEAD.".to_owned(),
        notes: None,
    }
}

#[test]
fn healthy_pack_strip_validates_clean() {
    let strip = healthy_pack_strip();
    let findings = strip.validate();
    assert!(
        findings.is_empty(),
        "expected clean strip, got {findings:?}"
    );
}

#[test]
fn healthy_pack_strip_rejects_stale_freshness() {
    let mut strip = healthy_pack_strip();
    strip.health_block.freshness_class = FreshnessClass::Stale;
    let findings = strip.validate();
    assert!(
        findings
            .iter()
            .any(|finding| finding.check_id == "framework_support_strip.healthy_live_freshness"),
        "expected healthy_live_freshness finding, got {findings:?}"
    );
}

#[test]
fn heuristic_strip_rejects_pack_update_action() {
    let mut strip = healthy_pack_strip();
    strip.support_class = SupportClass::HeuristicConventionMode;
    strip.pack_or_bridge_source_block.pack_source_class = PackSourceClass::HeuristicConventionOnly;
    strip.pack_or_bridge_source_block.pack_ref = None;
    strip.pack_or_bridge_source_block.pack_version_label = None;
    strip.compatibility_block.version_compatibility_class =
        VersionCompatibilityClass::UnknownVersion;
    strip.compatibility_block.supported_version_range_label = None;
    strip.compatibility_block.downgraded_behavior_summary =
        Some("Routes inferred from file layout only.".to_owned());
    strip
        .actions
        .push(FrameworkSupportActionClass::RequestPackUpdate);
    let findings = strip.validate();
    assert!(
        findings
            .iter()
            .any(|finding| finding.check_id == "framework_support_strip.pack_update_without_pack"),
        "expected pack_update_without_pack finding, got {findings:?}"
    );
}

#[test]
fn core_native_rejects_unsupported_version() {
    let mut strip = healthy_pack_strip();
    strip.support_class = SupportClass::CoreNative;
    strip.pack_or_bridge_source_block.pack_source_class = PackSourceClass::FirstPartyNative;
    strip.compatibility_block.version_compatibility_class =
        VersionCompatibilityClass::UnsupportedVersion;
    let findings = strip.validate();
    assert!(
        findings
            .iter()
            .any(|finding| finding.check_id
                == "framework_support_strip.core_native_version_compat"),
        "expected core_native_version_compat finding, got {findings:?}"
    );
}

fn exact_route_row() -> FrameworkObjectCertainty {
    FrameworkObjectCertainty {
        record_kind: FRAMEWORK_OBJECT_CERTAINTY_RECORD_KIND.to_owned(),
        framework_object_certainty_schema_version: FRAMEWORK_OBJECT_CERTAINTY_SCHEMA_VERSION,
        framework_object_certainty_id: "framework.cert.tests.exact_route".to_owned(),
        captured_at: "2026-05-18T09:01:00Z".to_owned(),
        surface_class: SurfaceClass::RouteExplorerSurface,
        framework_object_kind: FrameworkObjectKind::FrameworkObjectRow,
        certainty_label_class: CertaintyLabelClass::ExactPackBacked,
        support_class: SupportClass::FrameworkPack,
        pack_or_bridge_source_block: PackOrBridgeSourceBlock {
            pack_source_class: PackSourceClass::GovernedFrameworkPack,
            pack_ref: Some("framework.pack.next_app_router.v3".to_owned()),
            pack_version_label: Some("v3.4".to_owned()),
            bridge_ref: None,
            bridge_label: None,
        },
        framework_object_row_block: Some(FrameworkObjectRowBlock {
            framework_object_row_kind: FrameworkObjectRowKind::RouteRow,
            object_ref: Some("framework.route.dashboard.id".to_owned()),
            object_label: "/dashboard".to_owned(),
            authored_origin_class: AuthoredOriginClass::AuthoredByUser,
            parent_object_ref: None,
            evidence_anchors: vec![
                EvidenceAnchor {
                    evidence_anchor_kind_class: EvidenceAnchorKindClass::SourceFileAnchor,
                    anchor_ref: "src.file.app.dashboard.page".to_owned(),
                    anchor_label: "app/dashboard/page.tsx".to_owned(),
                },
                EvidenceAnchor {
                    evidence_anchor_kind_class: EvidenceAnchorKindClass::PackProvingArtifactAnchor,
                    anchor_ref: "framework.proving_artifact.next_app_router.routes_manifest"
                        .to_owned(),
                    anchor_label: "App Router routes manifest".to_owned(),
                },
            ],
            partial_or_derived_note: None,
        }),
        convention_diagnostic_block: None,
        generator_preview_block: None,
        actions: vec![
            RowActionClass::OpenCanonicalSource,
            RowActionClass::OpenPackDocs,
        ],
        support_strip_ref: Some("framework.strip.next_app_router.healthy".to_owned()),
        framework_certainty_row_record_ref: Some("framework.row.route.proven.0001".to_owned()),
        source_sync_chip_record_ref: Some("framework.chip.in_sync.0001".to_owned()),
        summary: "/dashboard resolves exactly to app/dashboard/page.tsx under Next.js App Router."
            .to_owned(),
        notes: None,
    }
}

#[test]
fn exact_route_row_validates_clean() {
    let record = exact_route_row();
    let findings = record.validate();
    assert!(
        findings.is_empty(),
        "expected clean record, got {findings:?}"
    );
}

#[test]
fn exact_label_rejects_heuristic_support_class() {
    let mut record = exact_route_row();
    record.support_class = SupportClass::HeuristicConventionMode;
    record.pack_or_bridge_source_block.pack_source_class = PackSourceClass::HeuristicConventionOnly;
    record.pack_or_bridge_source_block.pack_ref = None;
    record.pack_or_bridge_source_block.pack_version_label = None;
    let findings = record.validate();
    assert!(
        findings.iter().any(|finding| finding.check_id
            == "framework_object_certainty.exact_requires_pack_or_native"),
        "expected exact_requires_pack_or_native finding, got {findings:?}"
    );
}

#[test]
fn heuristic_row_requires_visible_note() {
    let mut record = exact_route_row();
    record.support_class = SupportClass::HeuristicConventionMode;
    record.pack_or_bridge_source_block.pack_source_class = PackSourceClass::HeuristicConventionOnly;
    record.pack_or_bridge_source_block.pack_ref = None;
    record.pack_or_bridge_source_block.pack_version_label = None;
    record.certainty_label_class = CertaintyLabelClass::HeuristicSuspicion;
    if let Some(row) = record.framework_object_row_block.as_mut() {
        row.partial_or_derived_note = None;
        row.evidence_anchors = vec![EvidenceAnchor {
            evidence_anchor_kind_class: EvidenceAnchorKindClass::ConventionPatternAnchor,
            anchor_ref: "convention.pattern.app_dir".to_owned(),
            anchor_label: "app/ directory convention".to_owned(),
        }];
        row.authored_origin_class = AuthoredOriginClass::OriginUnknown;
    }
    let findings = record.validate();
    assert!(
        findings.iter().any(|finding| finding.check_id
            == "framework_object_certainty.partial_or_derived_note_required"),
        "expected partial_or_derived_note_required finding, got {findings:?}"
    );
}

#[test]
fn user_authored_route_requires_source_round_trip() {
    let mut record = exact_route_row();
    record.certainty_label_class = CertaintyLabelClass::DerivedByConvention;
    record.support_class = SupportClass::HeuristicConventionMode;
    record.pack_or_bridge_source_block.pack_source_class = PackSourceClass::HeuristicConventionOnly;
    record.pack_or_bridge_source_block.pack_ref = None;
    record.pack_or_bridge_source_block.pack_version_label = None;
    if let Some(row) = record.framework_object_row_block.as_mut() {
        row.partial_or_derived_note =
            Some("Inferred from file layout; runtime evidence missing.".to_owned());
        row.evidence_anchors = vec![EvidenceAnchor {
            evidence_anchor_kind_class: EvidenceAnchorKindClass::ConventionPatternAnchor,
            anchor_ref: "convention.pattern.app_dir".to_owned(),
            anchor_label: "app/ directory convention".to_owned(),
        }];
    }
    let findings = record.validate();
    assert!(
        findings
            .iter()
            .any(|finding| finding.check_id
                == "framework_object_certainty.source_round_trip_required"),
        "expected source_round_trip_required finding, got {findings:?}"
    );
}

fn diagnostic_with_generator_action() -> FrameworkObjectCertainty {
    FrameworkObjectCertainty {
        record_kind: FRAMEWORK_OBJECT_CERTAINTY_RECORD_KIND.to_owned(),
        framework_object_certainty_schema_version: FRAMEWORK_OBJECT_CERTAINTY_SCHEMA_VERSION,
        framework_object_certainty_id: "framework.cert.tests.diag.gen_action".to_owned(),
        captured_at: "2026-05-18T09:02:00Z".to_owned(),
        surface_class: SurfaceClass::ConventionDiagnosticsSurface,
        framework_object_kind: FrameworkObjectKind::ConventionDiagnosticRow,
        certainty_label_class: CertaintyLabelClass::DerivedByConvention,
        support_class: SupportClass::FrameworkPack,
        pack_or_bridge_source_block: PackOrBridgeSourceBlock {
            pack_source_class: PackSourceClass::GovernedFrameworkPack,
            pack_ref: Some("framework.pack.next_app_router.v3".to_owned()),
            pack_version_label: Some("v3.4".to_owned()),
            bridge_ref: None,
            bridge_label: None,
        },
        framework_object_row_block: None,
        convention_diagnostic_block: Some(ConventionDiagnosticBlock {
            convention_diagnostic_class: ConventionDiagnosticClass::MissingRegistration,
            convention_certainty_class: ConventionCertaintyClass::CertaintyProvenViolation,
            affected_object_ref: "framework.route.missing.id".to_owned(),
            affected_object_label: "/settings".to_owned(),
            evidence_anchors: vec![EvidenceAnchor {
                evidence_anchor_kind_class: EvidenceAnchorKindClass::PackProvingArtifactAnchor,
                anchor_ref: "framework.proving_artifact.next_app_router.routes_manifest".to_owned(),
                anchor_label: "Routes manifest reports no handler for /settings".to_owned(),
            }],
            fix_actions: vec![ConventionFixActionClass::OpenGeneratorPreview],
            generator_preview_ref: None,
            suppressibility_note: None,
        }),
        generator_preview_block: None,
        actions: vec![RowActionClass::OpenConventionDiagnostic],
        support_strip_ref: Some("framework.strip.next_app_router.healthy".to_owned()),
        framework_certainty_row_record_ref: None,
        source_sync_chip_record_ref: None,
        summary: "/settings has no registered handler under the Next.js App Router pack."
            .to_owned(),
        notes: None,
    }
}

#[test]
fn open_generator_preview_requires_paired_ref() {
    let record = diagnostic_with_generator_action();
    let findings = record.validate();
    assert!(
        findings.iter().any(|finding| finding.check_id
            == "framework_object_certainty.generator_preview_ref_required"),
        "expected generator_preview_ref_required finding, got {findings:?}"
    );
}

fn safe_pack_scaffold_preview() -> FrameworkObjectCertainty {
    FrameworkObjectCertainty {
        record_kind: FRAMEWORK_OBJECT_CERTAINTY_RECORD_KIND.to_owned(),
        framework_object_certainty_schema_version: FRAMEWORK_OBJECT_CERTAINTY_SCHEMA_VERSION,
        framework_object_certainty_id: "framework.cert.tests.gen.preview".to_owned(),
        captured_at: "2026-05-18T09:03:00Z".to_owned(),
        surface_class: SurfaceClass::GeneratorPreviewSurface,
        framework_object_kind: FrameworkObjectKind::GeneratorPreviewRow,
        certainty_label_class: CertaintyLabelClass::ExactPackBacked,
        support_class: SupportClass::FrameworkPack,
        pack_or_bridge_source_block: PackOrBridgeSourceBlock {
            pack_source_class: PackSourceClass::GovernedFrameworkPack,
            pack_ref: Some("framework.pack.next_app_router.v3".to_owned()),
            pack_version_label: Some("v3.4".to_owned()),
            bridge_ref: None,
            bridge_label: None,
        },
        framework_object_row_block: None,
        convention_diagnostic_block: None,
        generator_preview_block: Some(GeneratorPreviewBlock {
            generator_kind_class: GeneratorKindClass::FrameworkPackScaffold,
            generator_id_ref: "framework.generator.next_app_router.new_route".to_owned(),
            generator_label: "New route scaffold".to_owned(),
            generator_version_label: "v3.4".to_owned(),
            input_summary: "Route slug 'settings'; default layout; no auth guard.".to_owned(),
            file_effect_rows: vec![
                GeneratorFileEffectRow {
                    file_effect_class: FileEffectClass::CreateFile,
                    file_ownership_class: FileOwnershipClass::FrameworkGeneratedOverwritable,
                    file_path_handle_ref: "file.handle.app.settings.page".to_owned(),
                    file_label: "app/settings/page.tsx".to_owned(),
                    requires_user_confirmation: false,
                },
                GeneratorFileEffectRow {
                    file_effect_class: FileEffectClass::ModifyFile,
                    file_ownership_class: FileOwnershipClass::ManagedByPackOverwritable,
                    file_path_handle_ref: "file.handle.app.routes_manifest".to_owned(),
                    file_label: "app/_routes.generated.ts".to_owned(),
                    requires_user_confirmation: false,
                },
            ],
            dependency_impact_class: DependencyImpactClass::NoDependencyChange,
            rollback_class: RollbackClass::RollbackViaCheckpoint,
            checkpoint_ref: Some("checkpoint.cp_2026_05_18_0903".to_owned()),
            regenerate_path_available: true,
        }),
        actions: vec![
            RowActionClass::OpenGeneratorPreview,
            RowActionClass::OpenCanonicalSource,
        ],
        support_strip_ref: Some("framework.strip.next_app_router.healthy".to_owned()),
        framework_certainty_row_record_ref: None,
        source_sync_chip_record_ref: None,
        summary: "Scaffolds /settings route with managed routes manifest update.".to_owned(),
        notes: None,
    }
}

#[test]
fn safe_pack_scaffold_preview_validates_clean() {
    let record = safe_pack_scaffold_preview();
    let findings = record.validate();
    assert!(
        findings.is_empty(),
        "expected clean preview, got {findings:?}"
    );
}

#[test]
fn delete_user_owned_file_requires_confirmation() {
    let mut record = safe_pack_scaffold_preview();
    if let Some(gen) = record.generator_preview_block.as_mut() {
        gen.file_effect_rows.push(GeneratorFileEffectRow {
            file_effect_class: FileEffectClass::DeleteFile,
            file_ownership_class: FileOwnershipClass::UserOwnedAuthored,
            file_path_handle_ref: "file.handle.app.legacy_page".to_owned(),
            file_label: "app/legacy/page.tsx".to_owned(),
            requires_user_confirmation: false,
        });
    }
    let findings = record.validate();
    assert!(
        findings.iter().any(|finding| finding.check_id
            == "framework_object_certainty.delete_user_owned_requires_confirmation"),
        "expected delete_user_owned_requires_confirmation finding, got {findings:?}"
    );
}

#[test]
fn checkpoint_rollback_requires_checkpoint_ref() {
    let mut record = safe_pack_scaffold_preview();
    if let Some(gen) = record.generator_preview_block.as_mut() {
        gen.checkpoint_ref = None;
    }
    let findings = record.validate();
    assert!(
        findings.iter().any(|finding| finding.check_id
            == "framework_object_certainty.checkpoint_ref_required"),
        "expected checkpoint_ref_required finding, got {findings:?}"
    );
}

#[test]
fn unsupported_forces_no_admissible_evidence() {
    let mut record = exact_route_row();
    record.support_class = SupportClass::UnsupportedOrUnclaimed;
    record.pack_or_bridge_source_block.pack_source_class = PackSourceClass::NoPackOrBridge;
    record.pack_or_bridge_source_block.pack_ref = None;
    record.pack_or_bridge_source_block.pack_version_label = None;
    record.certainty_label_class = CertaintyLabelClass::PartialEvidence;
    if let Some(row) = record.framework_object_row_block.as_mut() {
        row.partial_or_derived_note = Some("partial".to_owned());
    }
    let findings = record.validate();
    assert!(
        findings.iter().any(|finding| finding.check_id
            == "framework_object_certainty.unsupported_requires_no_admissible_evidence"),
        "expected unsupported_requires_no_admissible_evidence finding, got {findings:?}"
    );
}
