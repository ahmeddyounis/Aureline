use super::*;

fn register() -> M5ProvenanceCardRegister {
    current_m5_provenance_cards().expect("register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(r.schema_version, M5_PROVENANCE_CARDS_SCHEMA_VERSION);
    assert_eq!(r.record_kind, M5_PROVENANCE_CARDS_RECORD_KIND);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "register must validate cleanly: {violations:#?}"
    );
    assert!(!r.rows.is_empty());
}

#[test]
fn embedded_json_matches_builder() {
    // The checked-in JSON must be exactly what the in-code builder produces, so
    // the embedded consumer and the artifact never drift.
    assert_eq!(register(), build_m5_provenance_cards());
}

#[test]
fn builder_validates_cleanly() {
    assert_eq!(build_m5_provenance_cards().validate(), Vec::new());
}

#[test]
fn covers_every_family_kind() {
    let r = register();
    for kind in M5ArtifactFamilyKind::ALL {
        assert!(
            !r.rows_for_kind(kind).is_empty(),
            "family kind {} must have at least one card",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_artifact() {
    let r = register();
    assert!(!r.release_blocking_artifact_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_rows()
        .iter()
        .map(|row| row.artifact_ref.as_str())
        .collect();
    for declared in &r.release_blocking_artifact_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking card"
        );
    }
}

#[test]
fn register_narrows_at_least_one_family() {
    let r = register();
    assert!(
        !r.rows_narrowed().is_empty(),
        "the register must narrow at least one family below the cutline"
    );
}

#[test]
fn every_gap_reason_has_a_stop_rule() {
    let r = register();
    let covered: std::collections::BTreeSet<CardGapReason> = r
        .stop_rules
        .iter()
        .map(|rule| rule.trigger_reason)
        .collect();
    for reason in CardGapReason::ALL {
        assert!(covered.contains(&reason), "{}", reason.as_str());
    }
}

#[test]
fn converged_cards_converge_every_surface_on_one_build_identity() {
    // Acceptance: About/Help/service-health and support/export surfaces agree on
    // the same build identity and artifact provenance for every claimed family.
    let r = register();
    for row in r.rows_converged() {
        assert!(row.has_all_surfaces(), "{} misses a surface", row.entry_id);
        assert!(row.surfaces_converge(), "{} diverges", row.entry_id);
        for binding in &row.surface_bindings {
            assert_eq!(
                binding.build_identity_ref,
                row.exact_build.build_identity_ref
            );
            assert_eq!(binding.provenance_ref, row.exact_build.provenance_ref);
        }
    }
}

#[test]
fn converged_cards_render_help_about_chrome() {
    // Guardrail: no release-center-only truth that Help and About cannot explain.
    let r = register();
    for row in &r.rows {
        if row.surface(ProvenanceSurfaceKind::ReleaseCenter).is_some() {
            assert!(
                row.has_help_about_chrome(),
                "{} renders release-center without Help/About chrome",
                row.entry_id
            );
        }
    }
}

#[test]
fn converged_provenance_survives_offline_and_mirror() {
    // Acceptance: user-visible provenance survives offline and mirror profiles
    // without needing live vendor connectivity.
    let r = register();
    for row in r.rows_converged() {
        assert!(
            row.offline_provenance_survives(),
            "{} provenance does not survive offline",
            row.entry_id
        );
        for surface in ProvenanceSurfaceKind::HELP_ABOUT_CHROME {
            assert!(
                row.surface(surface)
                    .map(|b| b.offline_available)
                    .unwrap_or(false),
                "{} chrome surface {} is not offline-available",
                row.entry_id,
                surface.as_str()
            );
        }
    }
}

#[test]
fn badges_are_copy_safe_and_machine_readable() {
    // Acceptance: signature/attestation/SBOM/export/mirror/official/partial/
    // not-provided states are machine-readable and copy-safe.
    let r = register();
    for row in &r.rows {
        for kind in ProvenanceBadgeKind::ALL {
            let badge = row
                .badge(kind)
                .unwrap_or_else(|| panic!("{} missing badge {}", row.entry_id, kind.as_str()));
            assert!(
                badge.copyable,
                "{} badge {} not copy-safe",
                row.entry_id,
                kind.as_str()
            );
            assert_eq!(
                badge.machine_token,
                badge.canonical_token(),
                "{} badge {} token not canonical",
                row.entry_id,
                kind.as_str()
            );
            assert_eq!(
                badge.machine_token,
                format!("{}:{}", kind.as_str(), badge.state.as_str())
            );
        }
    }
}

#[test]
fn badges_never_overclaim_trust() {
    // Guardrail: a badge may not imply a stronger trust posture than the actual
    // signature/attestation/SBOM state available.
    let r = register();
    for row in &r.rows {
        assert!(
            !row.badge_overclaims(),
            "{} has an overclaiming badge: {:?}",
            row.entry_id,
            row.overclaiming_badge_kind()
        );
        for badge in &row.badges {
            assert_eq!(
                badge.state,
                row.canonical_badge_state(badge.kind),
                "{} badge {} state is not the honest canonical state",
                row.entry_id,
                badge.kind.as_str()
            );
        }
    }
}

#[test]
fn exact_build_converges_across_publication_matrix_vocabulary() {
    // The card reuses the publication-matrix exact-build identity (signature,
    // attestation, SBOM, symbols, mirror, rollback, evidence) rather than a local
    // synonym set; a converged card has intact exact-build linkage.
    let r = register();
    for row in r.rows_converged() {
        assert!(
            row.exact_build.linkage_intact(),
            "{} converges without intact exact-build linkage",
            row.entry_id
        );
        assert!(row.exact_build.required_gap_reasons().is_empty());
    }
}

#[test]
fn narrowed_family_surfaces_its_gaps_in_export() {
    let r = register();
    let projection = r.support_export_projection();
    let narrowed = projection
        .rows
        .iter()
        .find(|row| !row.publishes_stable)
        .expect("a narrowed family exists");
    assert!(
        !narrowed.active_gap_reasons.is_empty(),
        "a narrowed family must surface its gap reasons"
    );
    assert!(
        !narrowed.surfaces_converge || !narrowed.offline_verifiable,
        "a narrowed family must surface its concrete provenance gap"
    );
    // Even a narrowed family keeps its badges honest and copy-safe.
    for badge in &narrowed.badges {
        assert_eq!(
            badge.machine_token,
            format!("{}:{}", badge.kind.as_str(), badge.state.as_str())
        );
    }
}

#[test]
fn export_projection_mirrors_rows() {
    let r = register();
    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.rows.len());
    for (row, proj) in r.rows.iter().zip(&projection.rows) {
        assert_eq!(row.entry_id, proj.entry_id);
        assert_eq!(row.publishes_stable(), proj.publishes_stable);
        assert_eq!(row.exact_build.build_identity_ref, proj.build_identity_ref);
        assert_eq!(row.surface_bindings.len(), proj.surfaces.len());
        assert_eq!(row.badges.len(), proj.badges.len());
        assert_eq!(row.surfaces_converge(), proj.surfaces_converge);
    }
}

#[test]
fn summary_counts_match_rows() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(
        r.summary.entries_converged + r.summary.entries_narrowed,
        r.rows.len()
    );
    assert!(r.summary.signatures_verified > 0);
    assert!(r.summary.total_badges == r.rows.len() * ProvenanceBadgeKind::ALL.len());
}

#[test]
fn publication_decision_matches_computed() {
    let r = register();
    assert_eq!(r.publication.decision, r.computed_publication_decision());
    assert_eq!(
        r.publication.blocking_rule_ids,
        r.computed_blocking_rule_ids()
    );
    assert_eq!(
        r.publication.blocking_claim_ids,
        r.computed_blocking_entry_ids()
    );
}

#[test]
fn validate_flags_a_converged_family_with_active_gap() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a converged family exists");
    row.active_gap_reasons
        .push(CardGapReason::ProofPacketMissing);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, M5ProvenanceCardViolation::HeldWithActiveGap { .. })));
}

#[test]
fn validate_flags_a_divergent_surface_on_a_converged_family() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a converged family exists");
    row.surface_bindings[0].build_identity_ref = "exact_build/tampered".to_owned();
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ProvenanceCardViolation::HeldWithoutSurfaceConvergence { .. }
    )));
}

#[test]
fn validate_flags_a_badge_that_overclaims_trust() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a converged family exists");
    // Force the build-identity badge to claim a verified signature it does not hold.
    let badge = row
        .badges
        .iter_mut()
        .find(|b| b.kind == ProvenanceBadgeKind::BuildIdentity)
        .expect("build-identity badge exists");
    badge.state = ProvenanceBadgeState::Verified;
    badge.machine_token = badge.canonical_token();
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ProvenanceCardViolation::BadgeOverclaimsTrust {
            kind: ProvenanceBadgeKind::BuildIdentity,
            ..
        }
    )));
}

#[test]
fn validate_flags_release_center_only_truth() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a converged family exists");
    // Strip the About chrome while keeping the release-center surface.
    row.surface_bindings
        .retain(|b| b.surface != ProvenanceSurfaceKind::About);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, M5ProvenanceCardViolation::ReleaseCenterOnlyTruth { .. })));
}

#[test]
fn validate_flags_a_converged_family_without_offline_provenance() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a converged family exists");
    row.mirror_offline.offline_verifiable = false;
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ProvenanceCardViolation::HeldWithoutOfflineProvenance { .. }
    )));
}

#[test]
fn validate_flags_a_converged_family_without_signoff() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a converged family exists");
    row.owner_signoff.signed_off = false;
    row.owner_signoff.signed_at = None;
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, M5ProvenanceCardViolation::HeldWithoutSignoff { .. })));
}
