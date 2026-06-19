use super::*;

use crate::maintenance::DocsPublishBoundaryState;

#[test]
fn seeded_contract_validates() {
    validate_seeded_release_docs_maintenance()
        .expect("seeded release-docs maintenance records validate");
}

#[test]
fn contract_record_identity_is_stable() {
    let contract = seeded_release_docs_maintenance_contract();
    assert_eq!(
        contract.record_kind,
        RELEASE_DOCS_MAINTENANCE_CONTRACT_RECORD_KIND
    );
    assert_eq!(
        contract.schema_version,
        RELEASE_DOCS_MAINTENANCE_SCHEMA_VERSION
    );
    assert_eq!(contract.contract_id, RELEASE_DOCS_MAINTENANCE_CONTRACT_ID);
    for surface in &contract.surfaces {
        assert_eq!(
            surface.record_kind,
            RELEASE_DOCS_MAINTENANCE_SURFACE_RECORD_KIND
        );
        assert_eq!(
            surface.schema_version,
            RELEASE_DOCS_MAINTENANCE_SCHEMA_VERSION
        );
    }
}

#[test]
fn scope_is_visible_before_edit_on_every_surface() {
    let contract = seeded_release_docs_maintenance_contract();
    for surface in &contract.surfaces {
        assert!(
            surface.scope_visible_before_edit,
            "{} hides scope before edit",
            surface.surface_id
        );
        assert!(
            !surface.active_scope_summary.trim().is_empty(),
            "{} has an empty scope summary",
            surface.surface_id
        );
    }
}

#[test]
fn every_evidence_scope_is_exercised() {
    let contract = seeded_release_docs_maintenance_contract();
    for scope in [
        ReleaseDocsEvidenceScope::LocalDraft,
        ReleaseDocsEvidenceScope::PrivateBranch,
        ReleaseDocsEvidenceScope::SharedReview,
        ReleaseDocsEvidenceScope::SharedPrerelease,
        ReleaseDocsEvidenceScope::InstalledStable,
    ] {
        assert!(
            contract
                .surfaces
                .iter()
                .any(|surface| surface.evidence_scope == scope),
            "evidence scope {} is exercised",
            scope.as_str()
        );
    }
}

#[test]
fn non_stable_docs_carry_explicit_scope_unless_blocked() {
    let contract = seeded_release_docs_maintenance_contract();
    for surface in &contract.surfaces {
        if surface.evidence_scope.is_installed_stable() {
            continue;
        }
        if surface.publish_boundary_state == DocsPublishBoundaryState::BlockedUnscoped {
            // Blocked surfaces are explicitly stopped for lacking a scope.
            assert!(surface.apply_export_action.is_none());
            continue;
        }
        assert!(
            surface.publish_scope.is_scoped(),
            "{} is non-stable but unscoped",
            surface.surface_id
        );
    }
}

#[test]
fn installed_stable_surface_matches_running_build() {
    let contract = seeded_release_docs_maintenance_contract();
    let stable = contract
        .surfaces
        .iter()
        .find(|surface| surface.evidence_scope.is_installed_stable())
        .expect("an installed-stable surface exists");
    assert_eq!(
        stable.source_version_badge.version_match_state,
        VersionMatchState::ExactBuildMatch
    );
    assert_eq!(
        stable.source_version_badge.freshness_class,
        DocsFreshnessClass::AuthoritativeLive
    );
}

#[test]
fn masquerade_guard_flags_unscoped_prerelease() {
    let mut contract = seeded_release_docs_maintenance_contract();
    let surface = contract
        .surfaces
        .iter_mut()
        .find(|surface| surface.evidence_scope == ReleaseDocsEvidenceScope::SharedPrerelease)
        .expect("a prerelease surface exists");
    // Strip the scope and relax the boundary so the masquerade guard is the
    // only thing standing between a beta note and the installed stable truth.
    surface.publish_scope = DocsPublishScope::default();
    surface.publish_boundary_state = DocsPublishBoundaryState::LocalOnly;
    surface.publish_boundary_notes = Vec::new();
    surface.apply_export_action = None;
    let findings = contract.validate();
    assert!(
        findings
            .iter()
            .any(|finding| finding.check_id == "surface.unscoped_nonstable"),
        "expected masquerade guard finding, got {findings:?}"
    );
}

#[test]
fn falsely_installed_stable_is_flagged() {
    let mut contract = seeded_release_docs_maintenance_contract();
    let surface = contract
        .surfaces
        .iter_mut()
        .find(|surface| surface.evidence_scope == ReleaseDocsEvidenceScope::LocalDraft)
        .expect("a local-draft surface exists");
    // A drifted local draft must not be relabeled as installed stable.
    surface.evidence_scope = ReleaseDocsEvidenceScope::InstalledStable;
    let findings = contract.validate();
    assert!(
        findings
            .iter()
            .any(|finding| finding.check_id == "surface.stable_claim"),
        "expected stable-claim finding, got {findings:?}"
    );
}

#[test]
fn compare_history_stays_reopenable() {
    let contract = seeded_release_docs_maintenance_contract();
    let mut compare_kinds = std::collections::BTreeSet::new();
    for surface in &contract.surfaces {
        for entry in &surface.compare_history {
            assert!(
                entry.reopenable,
                "{} compare {} is not reopenable",
                surface.surface_id, entry.compare_id
            );
            assert_ne!(entry.base_ref, entry.target_ref);
            compare_kinds.insert(entry.compare_kind);
        }
    }
    for kind in [
        ReleaseDocsCompareKind::WorkingVsInstalled,
        ReleaseDocsCompareKind::BranchVsRelease,
        ReleaseDocsCompareKind::RevisionVsRevision,
        ReleaseDocsCompareKind::ChannelVsChannel,
    ] {
        assert!(
            compare_kinds.contains(&kind),
            "compare kind {} is exercised",
            kind.as_str()
        );
    }
}

#[test]
fn non_reopenable_compare_is_flagged() {
    let mut contract = seeded_release_docs_maintenance_contract();
    contract.surfaces[0].compare_history[0].reopenable = false;
    let findings = contract.validate();
    assert!(
        findings
            .iter()
            .any(|finding| finding.check_id == "compare_entry.reopenable"),
        "expected reopenable finding, got {findings:?}"
    );
}

#[test]
fn integration_targets_cover_in_product_surfaces() {
    let contract = seeded_release_docs_maintenance_contract();
    let mut targets = std::collections::BTreeSet::new();
    for surface in &contract.surfaces {
        assert!(
            surface.in_product_maintenance_path,
            "{} left the in-product path",
            surface.surface_id
        );
        for anchor in &surface.integration_anchors {
            targets.insert(anchor.target);
        }
    }
    for target in [
        ReleaseDocsIntegrationTarget::ReleaseCenter,
        ReleaseDocsIntegrationTarget::HelpBrowser,
        ReleaseDocsIntegrationTarget::AboutPanel,
        ReleaseDocsIntegrationTarget::SupportExport,
    ] {
        assert!(
            targets.contains(&target),
            "integration target {} is covered",
            target.as_str()
        );
    }
}

#[test]
fn browser_only_path_is_flagged() {
    let mut contract = seeded_release_docs_maintenance_contract();
    contract.surfaces[0].in_product_maintenance_path = false;
    let findings = contract.validate();
    assert!(
        findings
            .iter()
            .any(|finding| finding.check_id == "surface.in_product_path"),
        "expected in-product-path finding, got {findings:?}"
    );
}

#[test]
fn publish_boundaries_are_fully_covered() {
    let contract = seeded_release_docs_maintenance_contract();
    for boundary in [
        DocsPublishBoundaryState::LocalOnly,
        DocsPublishBoundaryState::ReviewHandoffScoped,
        DocsPublishBoundaryState::PublishHandoffScoped,
        DocsPublishBoundaryState::BlockedUnscoped,
    ] {
        assert!(
            contract
                .surfaces
                .iter()
                .any(|surface| surface.publish_boundary_state == boundary),
            "publish boundary {} is covered",
            boundary.as_str()
        );
    }
    // The blocked surface never exposes an apply or export action.
    assert!(contract
        .surfaces
        .iter()
        .any(|surface| surface.publish_boundary_state
            == DocsPublishBoundaryState::BlockedUnscoped
            && surface.apply_export_action.is_none()));
}

#[test]
fn pending_suggestions_resolve_to_evidence_backed_cards() {
    let contract = seeded_release_docs_maintenance_contract();
    for surface in &contract.surfaces {
        assert_eq!(
            surface.pending_suggestion_count,
            surface.pending_suggestion_refs.len()
        );
        for card_ref in &surface.pending_suggestion_refs {
            let card = contract
                .suggestion_card(card_ref)
                .expect("pending suggestion ref resolves to a card");
            assert!(card.silent_rewrite_blocked);
            assert!(!card.evidence_refs.is_empty());
        }
    }
}

#[test]
fn unknown_suggestion_ref_is_flagged() {
    let mut contract = seeded_release_docs_maintenance_contract();
    let surface = contract
        .surfaces
        .iter_mut()
        .find(|surface| surface.pending_suggestion_count == 1)
        .expect("a surface with a pending suggestion exists");
    surface.pending_suggestion_refs = vec!["release-docs-suggestion:does-not-exist".to_owned()];
    let findings = contract.validate();
    assert!(
        findings
            .iter()
            .any(|finding| finding.check_id == "surface.unknown_suggestion_ref"),
        "expected unknown-suggestion finding, got {findings:?}"
    );
}

#[test]
fn review_packet_is_metadata_only_and_inspectable() {
    let contract = seeded_release_docs_maintenance_contract();
    let packet = seeded_release_docs_review_packet();
    packet
        .validate_against_contract(&contract)
        .expect("review packet validates");
    assert!(!packet.raw_document_bodies_exported);
    assert!(packet.handoff_banner.screenshot_free_review);

    // Pending suggestions, compare history, and publish boundaries survive the
    // export so they stay inspectable after the user leaves the surface.
    assert_eq!(packet.surfaces, contract.surfaces);
    assert!(packet
        .surfaces
        .iter()
        .any(|surface| !surface.compare_history.is_empty()));
    assert!(packet
        .surfaces
        .iter()
        .any(|surface| surface.pending_suggestion_count > 0));

    let json = packet.export_safe_json();
    assert!(!json.contains("://"), "export must omit raw URLs");
}

#[test]
fn coverage_summary_counts_match() {
    let contract = seeded_release_docs_maintenance_contract();
    let projection = contract.surface_projection();
    assert_eq!(projection.coverage.surface_count, contract.surfaces.len());
    let compare_total: usize = contract
        .surfaces
        .iter()
        .map(|surface| surface.compare_history.len())
        .sum();
    assert_eq!(projection.coverage.compare_entry_count, compare_total);
    let anchor_total: usize = contract
        .surfaces
        .iter()
        .map(|surface| surface.integration_anchors.len())
        .sum();
    assert_eq!(projection.coverage.integration_anchor_count, anchor_total);
    assert_eq!(
        projection.coverage.suggestion_card_count,
        contract.suggestion_cards.len()
    );
}
