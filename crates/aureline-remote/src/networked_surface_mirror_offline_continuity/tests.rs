use super::*;

fn page() -> MirrorOfflineContinuityPage {
    seeded_mirror_offline_continuity_page()
}

#[test]
fn seeded_page_seeds_zero_defects_and_qualifies_stable() {
    let page = page();
    assert_eq!(
        page.defects.len(),
        0,
        "seeded page must be clean: {:?}",
        page.defects
    );
    assert!(validate_mirror_offline_continuity_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_covers_all_required_families() {
    let page = page();
    assert!(page.covers_all_required_families());
    assert_eq!(page.rows.len(), REQUIRED_ARTIFACT_FAMILIES.len());
    assert_eq!(
        page.continuity_snapshot.records.len(),
        REQUIRED_ARTIFACT_FAMILIES.len()
    );
    assert_eq!(
        page.summary.stable_row_count,
        REQUIRED_ARTIFACT_FAMILIES.len()
    );
}

#[test]
fn seeded_page_distinguishes_all_five_route_classes() {
    let page = page();
    assert!(page.distinguishes_all_route_classes());
    // Each of the five route-handling behaviors appears exactly once.
    for route in [
        ContinuityRouteClass::MirrorRoute,
        ContinuityRouteClass::LocalFileBundle,
        ContinuityRouteClass::PublicDirect,
        ContinuityRouteClass::Blocked,
        ContinuityRouteClass::Deferred,
    ] {
        assert_eq!(
            page.summary.route_counts.get(route.as_str()).copied(),
            Some(1),
            "route '{}' must appear exactly once",
            route.as_str()
        );
    }
}

#[test]
fn seeded_page_satisfies_core_continuity_invariants() {
    let page = page();
    assert!(page.no_record_allows_silent_public_fallback());
    assert!(page.replay_queues_are_idempotent_only());
    assert!(page.all_records_preserve_local_core());
    assert!(page.blocked_records_carry_reasons());
    assert!(page.all_fallback_rules_consistent());
    assert!(page.all_records_at_field_parity());
}

#[test]
fn records_render_at_field_catalog_parity() {
    let page = page();
    for record in &page.continuity_snapshot.records {
        assert!(record.fields_at_parity());
        let names: Vec<String> = record.render_fields().into_iter().map(|(n, _)| n).collect();
        assert_eq!(names, CONTINUITY_FIELD_NAMES.to_vec());
    }
}

#[test]
fn cli_and_support_views_quote_identical_codes_and_field_names() {
    let page = page();
    for record in &page.continuity_snapshot.records {
        let cli = record.render_cli_lines();
        let support = record.render_support_lines();
        assert_eq!(cli.len(), CONTINUITY_FIELD_NAMES.len());
        assert_eq!(support.len(), CONTINUITY_FIELD_NAMES.len());
        for ((name, value), (cli_line, support_line)) in record
            .render_fields()
            .into_iter()
            .zip(cli.iter().zip(support.iter()))
        {
            assert_eq!(cli_line, &format!("{name}={value}"));
            assert_eq!(support_line, &format!("{name}: {value}"));
        }
    }
}

#[test]
fn route_handling_maps_to_consistent_fallback_rule() {
    use ContinuityRouteClass::*;
    use PublicFallbackRuleClass::*;
    assert_eq!(
        MirrorRoute.consistent_public_fallback_rule(),
        MirrorOnlyNoFallback
    );
    assert_eq!(
        LocalFileBundle.consistent_public_fallback_rule(),
        NoPublicFallback
    );
    assert_eq!(
        PublicDirect.consistent_public_fallback_rule(),
        ExplicitPublicDirectAllowed
    );
    assert_eq!(Blocked.consistent_public_fallback_rule(), DenyAllNoFallback);
    assert_eq!(Deferred.consistent_public_fallback_rule(), NoPublicFallback);
}

#[test]
fn missing_family_narrows_to_preview() {
    let mut snapshot = seeded_mirror_offline_continuity_snapshot();
    snapshot
        .records
        .retain(|r| r.artifact_family != ArtifactFamilyClass::DocsPack);
    let page = MirrorOfflineContinuityPage::new(
        "remote:networked_surface_mirror_offline_continuity:test:missing",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityQualificationClass::Preview.as_str()
    );
    assert!(!page.covers_all_required_families());
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == ContinuityNarrowReasonClass::RequiredArtifactFamilyMissing));
}

#[test]
fn raw_material_withdraws_packet() {
    let mut snapshot = seeded_mirror_offline_continuity_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.artifact_family == ArtifactFamilyClass::Registry {
            r.raw_private_material_excluded = false;
        }
    }
    let page = MirrorOfflineContinuityPage::new(
        "remote:networked_surface_mirror_offline_continuity:test:raw",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityQualificationClass::Withdrawn.as_str()
    );
    assert!(!page.no_withdrawn_rows());
}

#[test]
fn bypass_withdraws_packet() {
    let mut snapshot = seeded_mirror_offline_continuity_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.artifact_family == ArtifactFamilyClass::ModelPack {
            r.no_bypass = false;
        }
    }
    let page = MirrorOfflineContinuityPage::new(
        "remote:networked_surface_mirror_offline_continuity:test:bypass",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityQualificationClass::Withdrawn.as_str()
    );
}

#[test]
fn mirror_only_profile_silent_public_fallback_withdraws_packet() {
    // A mirror-only profile that flips to a public-direct route is a silent
    // public fall-through and must withdraw the packet.
    let mut snapshot = seeded_mirror_offline_continuity_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.artifact_family == ArtifactFamilyClass::Registry {
            r.continuity_route = ContinuityRouteClass::PublicDirect;
            r.continuity_route_token = ContinuityRouteClass::PublicDirect.as_str().to_owned();
        }
    }
    let page = MirrorOfflineContinuityPage::new(
        "remote:networked_surface_mirror_offline_continuity:test:silent-fallback",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == ContinuityNarrowReasonClass::SilentPublicFallbackResolved));
}

#[test]
fn non_idempotent_deferred_action_withdraws_packet() {
    let mut snapshot = seeded_mirror_offline_continuity_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.artifact_family == ArtifactFamilyClass::CompanionHandoff {
            r.action_is_idempotent = false;
        }
    }
    let page = MirrorOfflineContinuityPage::new(
        "remote:networked_surface_mirror_offline_continuity:test:non-idempotent",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == ContinuityNarrowReasonClass::NonIdempotentReplayQueued));
}

#[test]
fn blocked_route_without_reason_narrows_to_beta() {
    let mut snapshot = seeded_mirror_offline_continuity_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.artifact_family == ArtifactFamilyClass::ModelPack {
            r.denial_reason = None;
            r.denial_reason_token = String::new();
        }
    }
    let page = MirrorOfflineContinuityPage::new(
        "remote:networked_surface_mirror_offline_continuity:test:no-reason",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityQualificationClass::Beta.as_str()
    );
    let row = page
        .rows
        .iter()
        .find(|r| r.artifact_family_token == ArtifactFamilyClass::ModelPack.as_str())
        .expect("model_pack row present");
    assert_eq!(
        row.narrow_reason_token,
        ContinuityNarrowReasonClass::DenialReasonMissing.as_str()
    );
}

#[test]
fn stale_proof_narrows_to_beta() {
    let mut snapshot = seeded_mirror_offline_continuity_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.artifact_family == ArtifactFamilyClass::DocsPack {
            r.trust_proof_freshness = ProofFreshnessClass::ExpiredBeyondWindow;
            r.trust_proof_freshness_token =
                ProofFreshnessClass::ExpiredBeyondWindow.as_str().to_owned();
        }
    }
    let page = MirrorOfflineContinuityPage::new(
        "remote:networked_surface_mirror_offline_continuity:test:stale",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityQualificationClass::Beta.as_str()
    );
    let row = page
        .rows
        .iter()
        .find(|r| r.artifact_family_token == ArtifactFamilyClass::DocsPack.as_str())
        .expect("docs_pack row present");
    assert_eq!(
        row.narrow_reason_token,
        ContinuityNarrowReasonClass::ProofStaleBeyondWindow.as_str()
    );
}

#[test]
fn stale_mirror_served_beyond_grace_narrows_to_beta() {
    // A registry mirror with a blocking stale-mirror warning that is still
    // served (route not blocked) narrows to beta.
    let mut snapshot = seeded_mirror_offline_continuity_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.artifact_family == ArtifactFamilyClass::Registry {
            r.stale_mirror_warning = StaleMirrorWarningClass::StaleBeyondGrace;
            r.stale_mirror_warning_token = StaleMirrorWarningClass::StaleBeyondGrace
                .as_str()
                .to_owned();
        }
    }
    let page = MirrorOfflineContinuityPage::new(
        "remote:networked_surface_mirror_offline_continuity:test:stale-mirror",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == ContinuityNarrowReasonClass::StaleMirrorServedBeyondGrace));
}

#[test]
fn inconsistent_fallback_rule_narrows_to_beta() {
    let mut snapshot = seeded_mirror_offline_continuity_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.artifact_family == ArtifactFamilyClass::DocsPack {
            // Local file bundle should imply no_public_fallback, not mirror-only.
            r.public_fallback_rule = PublicFallbackRuleClass::MirrorOnlyNoFallback;
            r.public_fallback_rule_token = PublicFallbackRuleClass::MirrorOnlyNoFallback
                .as_str()
                .to_owned();
        }
    }
    let page = MirrorOfflineContinuityPage::new(
        "remote:networked_surface_mirror_offline_continuity:test:inconsistent",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == ContinuityNarrowReasonClass::FallbackRuleInconsistent));
}

#[test]
fn support_export_rolls_up_defects_without_raw_material() {
    let page = page();
    let export = MirrorOfflineContinuitySupportExport::from_page(
        "remote:networked_surface_mirror_offline_continuity:support-export:test",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(
        export.shared_contract_ref,
        MIRROR_OFFLINE_CONTINUITY_SHARED_CONTRACT_REF
    );
}

#[test]
fn record_kinds_and_contract_refs_are_stable() {
    let page = page();
    assert_eq!(page.record_kind, MIRROR_OFFLINE_CONTINUITY_PAGE_RECORD_KIND);
    assert_eq!(
        page.schema_version,
        MIRROR_OFFLINE_CONTINUITY_SCHEMA_VERSION
    );
    assert_eq!(
        page.evidence_index_ref,
        MIRROR_OFFLINE_CONTINUITY_EVIDENCE_INDEX_REF
    );
    for record in &page.continuity_snapshot.records {
        assert_eq!(record.record_kind, MIRROR_OFFLINE_CONTINUITY_RECORD_KIND);
        assert_eq!(
            record.shared_contract_ref,
            MIRROR_OFFLINE_CONTINUITY_SHARED_CONTRACT_REF
        );
    }
    for row in &page.rows {
        assert_eq!(row.record_kind, MIRROR_OFFLINE_CONTINUITY_ROW_RECORD_KIND);
    }
}

#[test]
fn records_never_carry_raw_material() {
    let page = page();
    for record in &page.continuity_snapshot.records {
        assert!(record.raw_private_material_excluded);
        // Closed-vocabulary tokens only: no URLs/hosts in the routed fields.
        assert!(!record.continuity_route_token.contains("://"));
        assert!(!record.origin_scope_token.contains('.'));
    }
}

#[test]
fn page_round_trips_through_json() {
    let page = page();
    let json = serde_json::to_string(&page).expect("serialize");
    let back: MirrorOfflineContinuityPage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(page, back);
}
