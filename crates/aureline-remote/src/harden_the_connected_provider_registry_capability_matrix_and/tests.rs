use super::*;

fn page() -> ProviderRegistryPage {
    seeded_provider_registry_page()
}

#[test]
fn seeded_page_seeds_zero_defects_and_qualifies_stable() {
    let page = page();
    assert_eq!(page.defects.len(), 0, "seeded page must be clean: {:?}", page.defects);
    assert!(validate_provider_registry_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        ProviderRegistryQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_passes_all_six_stability_conditions() {
    let page = page();
    assert!(page.covers_all_required_pairs(), "all nine (family, actor) pairs must be covered");
    assert!(
        page.all_descriptors_declare_local_core_continuity(),
        "all descriptors must declare local-core continuity"
    );
    assert!(
        page.all_descriptors_declare_callback_path(),
        "all descriptors must declare their callback path class"
    );
    assert!(
        page.all_descriptors_declare_object_support(),
        "all descriptors must cover at least one object kind"
    );
}

#[test]
fn seeded_page_covers_all_required_pairs() {
    let page = page();
    let covered = page.descriptor_snapshot.covered_pairs();
    for (family, actor) in &REQUIRED_DESCRIPTOR_PAIRS {
        assert!(
            covered.contains(&(family.as_str().to_owned(), actor.as_str().to_owned())),
            "required pair '{}:{}' must be covered",
            family.as_str(),
            actor.as_str()
        );
    }
}

#[test]
fn seeded_page_rows_are_all_stable() {
    let page = page();
    assert!(!page.rows.is_empty(), "page must have at least one row");
    for row in &page.rows {
        assert_eq!(
            row.qualification_token,
            ProviderRegistryQualificationClass::Stable.as_str(),
            "row '{}:{}' must qualify stable; got '{}'",
            row.provider_family_token,
            row.actor_identity_token,
            row.qualification_token
        );
        assert_eq!(
            row.narrow_reason_token,
            ProviderRegistryNarrowReasonClass::NotNarrowed.as_str(),
            "row '{}:{}' must have not_narrowed reason; got '{}'",
            row.provider_family_token,
            row.actor_identity_token,
            row.narrow_reason_token
        );
    }
}

#[test]
fn seeded_page_summary_counts_match_rows() {
    let page = page();
    assert_eq!(page.summary.row_count, page.rows.len());
    assert_eq!(page.summary.stable_row_count, page.rows.len());
    assert_eq!(page.summary.beta_row_count, 0);
    assert_eq!(page.summary.preview_row_count, 0);
    assert_eq!(page.summary.withdrawn_row_count, 0);
}

#[test]
fn seeded_page_covers_nine_pairs() {
    let page = page();
    assert_eq!(
        page.summary.pairs_covered.len(),
        REQUIRED_DESCRIPTOR_PAIRS.len(),
        "summary must cover all nine required (family, actor) pairs"
    );
}

#[test]
fn seeded_page_all_rows_have_local_core_continuity_declared() {
    let page = page();
    for row in &page.rows {
        assert!(
            row.local_core_continuity_declared,
            "row '{}:{}' must declare local-core continuity",
            row.provider_family_token,
            row.actor_identity_token
        );
    }
    assert_eq!(
        page.summary.local_core_continuity_declared_count,
        page.rows.len()
    );
}

#[test]
fn seeded_page_all_rows_have_object_support_declared() {
    let page = page();
    for row in &page.rows {
        assert!(
            row.object_support_declared,
            "row '{}:{}' must declare object support",
            row.provider_family_token,
            row.actor_identity_token
        );
    }
    assert_eq!(
        page.summary.object_support_declared_count,
        page.rows.len()
    );
}

#[test]
fn support_export_wraps_clean_page() {
    let page = page();
    let export = ProviderRegistrySupportExport::from_page(
        "remote:provider-registry-export:stable:0001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(export.page.summary.withdrawn_row_count, 0);
}

#[test]
fn raw_private_material_on_any_descriptor_withdraws_packet() {
    let mut snapshot = seeded_provider_descriptor_snapshot();
    snapshot.records[0].raw_private_material_excluded = false;
    let page = ProviderRegistryPage::new(
        "remote:provider_registry:test",
        "test page",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.qualifies_stable());
    assert!(!page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        ProviderRegistryQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == ProviderRegistryNarrowReasonClass::RawPrivateMaterialExposed
    }));
    // Hard guardrail: only the withdrawal defect should be present.
    assert_eq!(page.defects.len(), 1);
}

#[test]
fn missing_required_pair_narrows_to_preview() {
    let mut snapshot = seeded_provider_descriptor_snapshot();
    // Remove the code_host × human_account pair.
    snapshot.records.retain(|r| {
        !(r.provider_family == ProviderFamilyClass::CodeHost
            && r.actor_identity == ActorIdentityClass::HumanAccount)
    });
    let page = ProviderRegistryPage::new(
        "remote:provider_registry:test",
        "test page",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        ProviderRegistryQualificationClass::Preview.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == ProviderRegistryNarrowReasonClass::RequiredRowMissing
            && d.source == "code_host:human_account"
    }));
}

#[test]
fn descriptor_without_local_core_continuity_narrows_to_beta() {
    let mut snapshot = seeded_provider_descriptor_snapshot();
    snapshot.records[0].local_core_continuity_allowed = false;
    let page = ProviderRegistryPage::new(
        "remote:provider_registry:test",
        "test page",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        ProviderRegistryQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == ProviderRegistryNarrowReasonClass::LocalCoreContinuityUndeclared
    }));
}

#[test]
fn descriptor_with_empty_object_support_narrows_to_beta() {
    let mut snapshot = seeded_provider_descriptor_snapshot();
    snapshot.records[0].object_support.clear();
    let page = ProviderRegistryPage::new(
        "remote:provider_registry:test",
        "test page",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        ProviderRegistryQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == ProviderRegistryNarrowReasonClass::ObjectSupportUndeclared
    }));
}

#[test]
fn qualification_class_checks() {
    assert!(ProviderRegistryQualificationClass::Stable.is_stable());
    assert!(!ProviderRegistryQualificationClass::Beta.is_stable());
    assert!(!ProviderRegistryQualificationClass::Withdrawn.is_stable());
    assert!(ProviderRegistryQualificationClass::Stable.is_claimable());
    assert!(ProviderRegistryQualificationClass::Beta.is_claimable());
    assert!(!ProviderRegistryQualificationClass::Withdrawn.is_claimable());
    assert!(!ProviderRegistryQualificationClass::Preview.is_claimable());
}

#[test]
fn narrow_reason_sentinel_checks() {
    assert!(
        ProviderRegistryNarrowReasonClass::RawPrivateMaterialExposed.is_withdrawal_reason()
    );
    assert!(!ProviderRegistryNarrowReasonClass::RequiredRowMissing.is_withdrawal_reason());
    assert!(ProviderRegistryNarrowReasonClass::RequiredRowMissing.is_preview_reason());
    assert!(
        !ProviderRegistryNarrowReasonClass::LocalCoreContinuityUndeclared.is_preview_reason()
    );
    assert!(!ProviderRegistryNarrowReasonClass::NotNarrowed.is_withdrawal_reason());
}

#[test]
fn provider_family_tokens_are_stable() {
    assert_eq!(ProviderFamilyClass::CodeHost.as_str(), "code_host");
    assert_eq!(ProviderFamilyClass::IssueTracker.as_str(), "issue_tracker");
    assert_eq!(ProviderFamilyClass::CiChecks.as_str(), "ci_checks");
}

#[test]
fn actor_identity_tokens_are_stable() {
    assert_eq!(ActorIdentityClass::HumanAccount.as_str(), "human_account");
    assert_eq!(ActorIdentityClass::InstallationGrant.as_str(), "installation_grant");
    assert_eq!(ActorIdentityClass::DelegatedCredential.as_str(), "delegated_credential");
    assert_eq!(ActorIdentityClass::LocalOnlyNoAccount.as_str(), "local_only_no_account");
}

#[test]
fn callback_path_tokens_are_stable() {
    assert_eq!(CallbackPathClass::PublicSaas.as_str(), "public_saas");
    assert_eq!(CallbackPathClass::MirroredIngress.as_str(), "mirrored_ingress");
    assert_eq!(CallbackPathClass::CustomerControlled.as_str(), "customer_controlled");
    assert_eq!(CallbackPathClass::PollingOrImportOnly.as_str(), "polling_or_import_only");
}

#[test]
fn snapshot_freshness_usability_checks() {
    assert!(SnapshotFreshnessClass::Fresh.is_usable());
    assert!(SnapshotFreshnessClass::StaleWithinWindow.is_usable());
    assert!(!SnapshotFreshnessClass::ExpiredBeyondWindow.is_usable());
    assert!(!SnapshotFreshnessClass::RevokedOrDisconnected.is_usable());
}

#[test]
fn record_kind_constants_match_schema_tokens() {
    assert_eq!(
        PROVIDER_REGISTRY_PAGE_RECORD_KIND,
        "remote_provider_registry_page_record"
    );
    assert_eq!(
        PROVIDER_DESCRIPTOR_RECORD_KIND,
        "remote_provider_descriptor_record"
    );
    assert_eq!(
        PROVIDER_REGISTRY_DEFECT_RECORD_KIND,
        "remote_provider_registry_defect_record"
    );
    assert_eq!(PROVIDER_REGISTRY_SCHEMA_VERSION, 1);
}

#[test]
fn audit_function_returns_same_defects_as_page() {
    let page = page();
    let audit_defects = audit_provider_registry_page(&page);
    assert_eq!(audit_defects.len(), page.defects.len());
}

#[test]
fn required_descriptor_pairs_count_is_nine() {
    assert_eq!(REQUIRED_DESCRIPTOR_PAIRS.len(), 9);
}

#[test]
fn seeded_snapshot_has_no_raw_private_material() {
    let snapshot = seeded_provider_descriptor_snapshot();
    for record in &snapshot.records {
        assert!(
            record.raw_private_material_excluded,
            "descriptor '{}:{}' must exclude raw private material",
            record.provider_family_token,
            record.actor_identity_token
        );
    }
}

#[test]
fn object_support_entries_have_stable_tokens() {
    let snapshot = seeded_provider_descriptor_snapshot();
    for record in &snapshot.records {
        for entry in &record.object_support {
            assert_eq!(
                entry.object_kind_token,
                entry.object_kind.as_str(),
                "object_kind_token must match object_kind enum for '{}:{}' entry '{}'",
                record.provider_family_token,
                record.actor_identity_token,
                entry.object_kind_token
            );
            assert_eq!(
                entry.mutation_posture_token,
                entry.mutation_posture.as_str(),
                "mutation_posture_token must match mutation_posture enum for '{}:{}' entry '{}'",
                record.provider_family_token,
                record.actor_identity_token,
                entry.object_kind_token
            );
        }
    }
}
