//! Inline unit tests binding the typed register to the checked-in artifact and
//! exercising the parity ceiling, disclosure narrowing, per-axis state, and the
//! publication verdict against mutated copies.

use super::*;

fn register() -> BoundaryManifestRegister {
    current_m5_versioned_boundary_manifests().expect("checked-in register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        M5_VERSIONED_BOUNDARY_MANIFESTS_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_VERSIONED_BOUNDARY_MANIFESTS_RECORD_KIND);
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.manifests.is_empty());
}

#[test]
fn every_family_has_exactly_one_versioned_manifest() {
    let r = register();
    for family in M5Family::ALL {
        let m = r
            .manifest_for_family(family)
            .unwrap_or_else(|| panic!("family {} must have a manifest", family.as_str()));
        assert!(
            !m.manifest_version.trim().is_empty(),
            "family {} manifest must be versioned",
            family.as_str()
        );
    }
}

#[test]
fn every_manifest_declares_all_guardrails_and_lane_detail() {
    let r = register();
    for m in &r.manifests {
        for kind in GuardrailKind::ALL {
            assert!(
                m.guardrails.iter().any(|g| g.kind == kind),
                "manifest {} must declare guardrail {}",
                m.manifest_id,
                kind.as_str()
            );
        }
        assert!(
            !m.lane_entries.is_empty(),
            "manifest {} must carry per-lane asset detail (no vague open-core copy)",
            m.manifest_id
        );
    }
}

#[test]
fn states_are_per_axis_not_one_global_flag() {
    let r = register();
    let states: BTreeSet<ManifestState> = r.manifests.iter().map(|m| m.manifest_state).collect();
    assert!(states.contains(&ManifestState::Published));
    assert!(
        states.len() >= 3,
        "expected several distinct manifest states"
    );
    let reasons: BTreeSet<ManifestReason> = r
        .manifests
        .iter()
        .flat_map(|m| m.active_reasons.iter().copied())
        .collect();
    assert!(!reasons.is_empty(), "narrowed manifests must name reasons");
}

#[test]
fn no_manifest_publishes_greener_than_its_release_evidence() {
    let r = register();
    for m in &r.manifests {
        if m.is_published() {
            assert!(
                !m.over_claims_release_evidence(),
                "published manifest {} over-claims its release evidence",
                m.manifest_id
            );
        }
        assert!(
            m.effective_label.rank() <= m.declared_label.rank(),
            "manifest {} effective label is wider than declared",
            m.manifest_id
        );
    }
}

#[test]
fn summary_and_parity_match_manifests() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(r.release_link_parity, r.computed_release_link_parity());
    assert_eq!(
        r.summary.manifests_published + r.summary.manifests_narrowed + r.summary.state_withdrawn,
        r.manifests.len()
    );
}

#[test]
fn reuse_projection_covers_every_manifest() {
    let r = register();
    let projection = r.reuse_projection();
    assert_eq!(projection.len(), r.manifests.len());
    for projected in &projection {
        assert!(
            !projected.surfaces.is_empty(),
            "projected manifest {} must carry reuse surfaces",
            projected.manifest_id
        );
    }
}

#[test]
fn manifest_layer_failure_holds_promotion_inherited_does_not() {
    let r = register();
    assert_eq!(r.publication.decision, r.computed_decision());
    let blocking = r.computed_blocking_manifest_ids();
    for id in &blocking {
        let m = r.manifest(id).expect("blocking manifest exists");
        assert!(m.release_blocking);
        assert!(m.declares_at_or_above_cutline());
        assert!(!m.is_waived());
        assert!(m.manifest_state.is_narrowed());
    }
    // An inherited (below-cutline) or waived narrowing is gated upstream.
    for m in &r.manifests {
        if m.manifest_state.is_narrowed() && (!m.declares_at_or_above_cutline() || m.is_waived()) {
            assert!(
                !blocking.contains(&m.manifest_id),
                "inherited/waived narrowing on {} must not hold promotion",
                m.manifest_id
            );
        }
    }
}

#[test]
fn validate_flags_a_published_over_claim() {
    let mut r = register();
    let m = r
        .manifests
        .iter_mut()
        .find(|m| m.is_published() && m.declared_label == LifecycleLabel::Stable)
        .expect("a published stable manifest exists");
    // Drop the release evidence to beta but leave the manifest published at stable.
    m.release_link.train_label = LifecycleLabel::Beta;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::PublishedOverClaimsReleaseEvidence { .. }
    )));
}

#[test]
fn validate_flags_a_published_manifest_with_a_gap() {
    let mut r = register();
    let m = r
        .manifests
        .iter_mut()
        .find(|m| m.is_published())
        .expect("a published manifest exists");
    m.active_reasons.push(ManifestReason::OwnerSignoffMissing);
    assert!(r
        .validate()
        .iter()
        .any(|x| matches!(x, RegisterViolation::PublishedWithActiveReason { .. })));
}

#[test]
fn validate_flags_an_undisclosed_dependency_without_a_reason() {
    let mut r = register();
    let m = r
        .manifests
        .iter_mut()
        .find(|m| m.is_published() && !m.residual_dependencies.is_empty())
        .expect("a published manifest with a disclosed dependency exists");
    // Hide a disclosed dependency without narrowing on disclosure.
    m.residual_dependencies[0].disclosed = false;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::GapWithoutReason {
            reason: ManifestReason::UndisclosedResidualDependency,
            ..
        }
    )));
}

#[test]
fn validate_flags_a_narrowed_manifest_above_the_cutline() {
    let mut r = register();
    let m = r
        .manifests
        .iter_mut()
        .find(|m| m.manifest_state.is_narrowed())
        .expect("a narrowed manifest exists");
    m.effective_label = LifecycleLabel::Stable;
    assert!(r.validate().iter().any(|x| matches!(
        x,
        RegisterViolation::NarrowedAboveCutline { .. }
            | RegisterViolation::EffectiveLabelMismatch { .. }
    )));
}

#[test]
fn validate_flags_a_proceed_verdict_while_a_rule_fires() {
    let mut r = register();
    if r.computed_decision() == PublicationDecision::Hold {
        r.publication.decision = PublicationDecision::Proceed;
        assert!(r
            .validate()
            .iter()
            .any(|x| matches!(x, RegisterViolation::PublicationDecisionInconsistent)));
    }
}
