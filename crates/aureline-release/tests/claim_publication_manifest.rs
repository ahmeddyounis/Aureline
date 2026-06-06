//! Protected tests binding the claim-publication manifest to the checked-in
//! stable-line artifact.
//!
//! The positive case is the checked-in manifest. The mutation cases prove that
//! stale/missing evidence must downgrade, surfaces may not overclaim, Certified
//! rows require current reference-workspace reports, and private evaluation
//! filters cannot widen beyond the public effective claim.

use aureline_release::claim_publication_manifest::{
    current_claim_publication_manifest, ClaimNarrowingReason, ClaimPublicationDecision,
    ClaimPublicationManifest, ClaimPublicationViolation, ClaimSurface, EffectiveClaim,
    EvidenceState, ReportFamily, SupportClass, CLAIM_PUBLICATION_MANIFEST_RECORD_KIND,
    CLAIM_PUBLICATION_MANIFEST_SCHEMA_VERSION,
};

fn manifest() -> ClaimPublicationManifest {
    current_claim_publication_manifest().expect("checked-in claim-publication manifest parses")
}

#[test]
fn checked_in_manifest_parses_and_validates() {
    let manifest = manifest();
    assert_eq!(
        manifest.schema_version,
        CLAIM_PUBLICATION_MANIFEST_SCHEMA_VERSION
    );
    assert_eq!(manifest.record_kind, CLAIM_PUBLICATION_MANIFEST_RECORD_KIND);
    let violations = manifest.validate();
    assert!(
        violations.is_empty(),
        "checked-in manifest must validate cleanly: {violations:#?}"
    );
}

#[test]
fn all_required_surfaces_consume_the_same_manifest() {
    let manifest = manifest();
    for entry in &manifest.entries {
        for surface in ClaimSurface::ALL {
            let projection = entry
                .surface_projections
                .iter()
                .find(|projection| projection.surface_id == surface)
                .unwrap_or_else(|| {
                    panic!("{} must project to {:?}", entry.entry_id, surface);
                });
            assert_eq!(projection.source_manifest_ref, manifest.manifest_id);
            assert!(
                !projection.linked_report_refs.is_empty(),
                "{} projection {:?} must carry report refs",
                entry.entry_id,
                surface
            );
        }
    }
}

#[test]
fn checked_in_manifest_records_automatic_downgrades() {
    let manifest = manifest();
    assert_eq!(
        manifest.publication.decision,
        ClaimPublicationDecision::ProceedWithDowngrades
    );
    let downgraded = manifest.downgraded_entries();
    assert_eq!(downgraded.len(), 2);
    assert!(downgraded.iter().any(|entry| entry
        .active_narrowing_reasons
        .contains(&ClaimNarrowingReason::ReferenceWorkspaceStale)));
    assert!(downgraded
        .iter()
        .any(|entry| entry.effective_claim == EffectiveClaim::Unsupported));
}

#[test]
fn every_certified_entry_has_current_reference_workspace_report() {
    let manifest = manifest();
    let certified_entries: Vec<_> = manifest
        .entries
        .iter()
        .filter(|entry| entry.declared_support_class == SupportClass::Certified)
        .collect();
    assert!(!certified_entries.is_empty());
    for entry in certified_entries {
        assert!(
            entry.has_current_certified_reference_report(),
            "{} must have a current signed reference-workspace report",
            entry.entry_id
        );
    }
}

#[test]
fn surface_overclaim_holds_publication() {
    let mut manifest = manifest();
    let entry = manifest
        .entries
        .iter_mut()
        .find(|entry| entry.effective_claim == EffectiveClaim::Limited)
        .expect("manifest has a Limited row");
    entry.surface_projections[0].rendered_claim = EffectiveClaim::Supported;
    manifest.publication.decision = manifest.computed_publication_decision();
    manifest.publication.blocking_entry_ids = manifest.computed_blocking_entry_ids();
    manifest.publication.blocking_surface_refs = manifest.computed_blocking_surface_refs();
    manifest.summary = manifest.computed_summary();

    assert!(manifest.validate().iter().any(|violation| matches!(
        violation,
        ClaimPublicationViolation::SurfaceOverclaims { .. }
    )));
    assert_eq!(
        manifest.computed_publication_decision(),
        ClaimPublicationDecision::Hold
    );
}

#[test]
fn stale_report_without_downgrade_fails() {
    let mut manifest = manifest();
    let entry = manifest
        .entries
        .iter_mut()
        .find(|entry| entry.effective_claim == EffectiveClaim::Limited)
        .expect("manifest has a Limited row");
    entry.effective_claim = EffectiveClaim::Supported;
    for projection in &mut entry.surface_projections {
        projection.rendered_claim = EffectiveClaim::Supported;
    }
    manifest.summary = manifest.computed_summary();

    assert!(manifest.validate().iter().any(|violation| matches!(
        violation,
        ClaimPublicationViolation::NarrowingEntryNotDowngraded { .. }
            | ClaimPublicationViolation::EffectiveClaimWiderThanDeclared { .. }
    )));
}

#[test]
fn certified_entry_without_current_reference_report_fails() {
    let mut manifest = manifest();
    let entry = manifest
        .entries
        .iter_mut()
        .find(|entry| entry.declared_support_class == SupportClass::Certified)
        .expect("manifest has a Certified row");
    let report = entry
        .linked_report_refs
        .iter_mut()
        .find(|report| report.report_family == ReportFamily::ReferenceWorkspaceReport)
        .expect("certified entry has reference report");
    report.evidence_state = EvidenceState::Stale;
    entry
        .active_narrowing_reasons
        .push(ClaimNarrowingReason::ReferenceWorkspaceStale);
    manifest.summary = manifest.computed_summary();

    assert!(manifest.validate().iter().any(|violation| matches!(
        violation,
        ClaimPublicationViolation::CertifiedEntryWithoutCurrentReport { .. }
    )));
}

#[test]
fn private_evaluation_filter_cannot_widen_public_claim() {
    let mut manifest = manifest();
    let filter = manifest
        .evaluation_filters
        .iter_mut()
        .find(|filter| filter.filter_id == "evaluation_filter:pilot_remote_review")
        .expect("pilot filter exists");
    filter.max_effective_claim = EffectiveClaim::Certified;

    assert!(manifest.validate().iter().any(|violation| matches!(
        violation,
        ClaimPublicationViolation::EvaluationFilterOverclaims { .. }
    )));
}
