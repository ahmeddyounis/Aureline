use super::*;

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::current_m5_ecosystem_governance_matrix;

fn packet() -> M5SideloadReview {
    current_m5_sideload_review().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_SIDELOAD_REVIEW_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_SIDELOAD_REVIEW_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_sheets() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_sheet_gate_is_consistent() {
    let packet = packet();
    assert!(packet.all_gates_consistent());
    for sheet in &packet.review_sheets {
        assert_eq!(
            sheet.rendered_trust_tier,
            sheet.computed_rendered_trust_tier(),
            "sheet {} rendered trust tier diverges from the recomputed cap",
            sheet.sheet_id
        );
        assert_eq!(
            sheet.review_triggers,
            sheet.computed_review_triggers(),
            "sheet {} review triggers diverge from the recomputed set",
            sheet.sheet_id
        );
        assert_eq!(
            sheet.disposition,
            sheet.computed_disposition(),
            "sheet {} disposition diverges from the recomputed gate",
            sheet.sheet_id
        );
    }
}

#[test]
fn no_sideload_inherits_a_trusted_badge() {
    // The core anti-abuse guardrail: a locally-built or side-loaded artifact can never
    // render a verified-publisher or enterprise-approved badge, even when signed by a
    // trusted key on the same machine.
    let packet = packet();
    for sheet in &packet.review_sheets {
        assert!(
            !sheet.rendered_trust_tier.is_trusted_badge(),
            "sheet {} renders a trusted badge {} for a side-load",
            sheet.sheet_id,
            sheet.rendered_trust_tier.as_str()
        );
    }
    // A signed-verified artifact that stays local still renders local-only.
    let framework = packet
        .review_sheet("sheet:framework_pack_first_sideload")
        .expect("framework sheet present");
    assert_eq!(framework.signature_state, SignatureState::SignedVerified);
    assert_eq!(framework.update_binding, UpdateBinding::StayLocal);
    assert_eq!(
        framework.rendered_trust_tier,
        TrustPosture::UnsignedLocalOnly
    );
}

#[test]
fn binding_to_registry_lifts_the_cap_only_to_registry_bound() {
    let packet = packet();
    let template = packet
        .review_sheet("sheet:template_artifact_bound_to_registry")
        .expect("template sheet present");
    assert_eq!(
        template.update_binding,
        UpdateBinding::BoundToRegistryIdentity
    );
    assert_eq!(template.rendered_trust_tier, TrustPosture::RegistryBound);
    assert!(!template.rendered_trust_tier.is_trusted_badge());
}

#[test]
fn widening_or_rebinding_forces_a_fresh_review() {
    // The lane guardrail: a permission widening, runtime/host change, executable
    // introduction, rebinding, or channel change on an installed side-load must force a
    // fresh review rather than a silent hot reload.
    let packet = packet();
    for sheet in &packet.review_sheets {
        let widens_or_rebinds = sheet.widens_permissions()
            || sheet.runtime_class_changed()
            || sheet.host_or_abi_rebound()
            || sheet.introduces_external_executable()
            || sheet.update_binding_changed()
            || sheet.release_channel_changed();
        if widens_or_rebinds {
            assert!(
                !sheet.allows_local_install(),
                "sheet {} allows a silent local install despite a widening/rebinding trigger",
                sheet.sheet_id
            );
        }
    }
}

#[test]
fn revoked_or_quarantined_sideloads_are_blocked() {
    let packet = packet();
    for sheet in &packet.review_sheets {
        if sheet.is_signature_revoked() || sheet.is_quarantined() {
            assert_eq!(
                sheet.disposition,
                SideloadDisposition::Blocked,
                "sheet {} is revoked/quarantined but is not blocked",
                sheet.sheet_id
            );
        }
    }
}

#[test]
fn non_ready_sheets_never_expose_an_enabled_accept_action() {
    let packet = packet();
    for sheet in &packet.review_sheets {
        if sheet.disposition != SideloadDisposition::ReviewedInstallReady {
            if let Some(accept) = sheet.accept_action() {
                assert!(
                    !accept.enabled,
                    "sheet {} disposition {} exposes an enabled accept action",
                    sheet.sheet_id,
                    sheet.disposition.as_str()
                );
            }
        }
    }
}

#[test]
fn installed_rows_preserve_limited_trust_continuity() {
    let packet = packet();
    assert!(packet.all_trust_continuity_preserved());
}

#[test]
fn every_disposition_is_exercised() {
    let packet = packet();
    for disposition in SideloadDisposition::ALL {
        assert!(
            packet
                .review_sheets
                .iter()
                .any(|s| s.disposition == disposition),
            "no sheet exercises disposition {}",
            disposition.as_str()
        );
    }
}

#[test]
fn every_review_trigger_is_exercised() {
    let packet = packet();
    for trigger in SideloadReviewTrigger::ALL {
        assert!(
            packet
                .review_sheets
                .iter()
                .any(|s| s.computed_review_triggers().contains(&trigger)),
            "no sheet exercises review trigger {}",
            trigger.as_str()
        );
    }
}

#[test]
fn every_update_binding_and_source_kind_is_exercised() {
    let packet = packet();
    for binding in UpdateBinding::ALL {
        assert!(
            packet
                .review_sheets
                .iter()
                .any(|s| s.update_binding == binding),
            "no sheet exercises update binding {}",
            binding.as_str()
        );
    }
    for kind in SourceKind::ALL {
        assert!(
            packet.review_sheets.iter().any(|s| s.source.kind == kind),
            "no sheet exercises source kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn both_install_kinds_are_exercised() {
    let packet = packet();
    for install_kind in SideloadInstallKind::ALL {
        assert!(
            packet
                .review_sheets
                .iter()
                .any(|s| s.install_kind == install_kind),
            "no sheet exercises install kind {}",
            install_kind.as_str()
        );
    }
}

#[test]
fn governance_family_refs_resolve_to_known_families() {
    // Every sheet resolves through a governance-matrix family, so the sideload review
    // shares one family vocabulary with the install-governance matrix.
    let packet = packet();
    let matrix = current_m5_ecosystem_governance_matrix().expect("matrix parses");
    let known: BTreeSet<ArtifactFamily> =
        matrix.families.iter().map(|r| r.artifact_family).collect();
    for sheet in &packet.review_sheets {
        assert!(
            known.contains(&sheet.package_kind),
            "sheet {} package kind {} is not a known governance family",
            sheet.sheet_id,
            sheet.package_kind.as_str()
        );
        assert!(
            !sheet.governance_family_ref.trim().is_empty(),
            "sheet {} has an empty governance family ref",
            sheet.sheet_id
        );
    }
}

#[test]
fn export_projection_mirrors_the_sheets() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.rows.len(), packet.review_sheets.len());
    assert!(projection.all_gates_consistent);
    assert_eq!(projection.blocked_count, 2);
    assert_eq!(projection.fresh_review_required_count, 3);
    for (row, sheet) in projection.rows.iter().zip(&packet.review_sheets) {
        assert_eq!(row.sheet_id, sheet.sheet_id);
        assert_eq!(row.disposition, sheet.disposition.as_str());
        assert_eq!(row.rendered_trust_tier, sheet.rendered_trust_tier.as_str());
        assert_eq!(row.allows_local_install, sheet.allows_local_install());
    }
}

#[test]
fn no_source_hint_is_an_absolute_path() {
    // Export-safety: no record carries an absolute machine path.
    let packet = packet();
    for sheet in &packet.review_sheets {
        assert!(
            !sheet.source.hint_looks_absolute(),
            "sheet {} source hint looks absolute",
            sheet.sheet_id
        );
    }
}

#[test]
fn detects_an_overstated_trust_tier() {
    // A locally-built side-load that claims a stronger rendered badge than its signing
    // state and binding allow is flagged by the gate recompute.
    let mut packet = packet();
    let sheet = packet
        .review_sheets
        .iter_mut()
        .find(|s| s.sheet_id == "sheet:framework_pack_first_sideload")
        .expect("framework sheet present");
    sheet.rendered_trust_tier = TrustPosture::EnterpriseApproved;
    let violations = packet.validate();
    assert!(
        violations
            .iter()
            .any(|v| matches!(v, M5SideloadReviewViolation::RenderedTrustMismatch { .. })),
        "expected a rendered-trust mismatch, got {violations:?}"
    );
    assert!(
        violations.iter().any(|v| matches!(
            v,
            M5SideloadReviewViolation::SideloadInheritsTrustedBadge { .. }
        )),
        "expected a trusted-badge inheritance violation, got {violations:?}"
    );
}

#[test]
fn detects_a_silently_elevated_trust_continuity() {
    // A reload that does not rebind to the registry must not raise the installed row's
    // limited-trust badge. Construct an installed row that was bound and rendered
    // local-only, then keep the binding unchanged while the recompute lifts the badge.
    let mut packet = packet();
    let sheet = packet
        .review_sheets
        .iter_mut()
        .find(|s| s.sheet_id == "sheet:recipe_pack_widen_and_rehost")
        .expect("recipe sheet present");
    sheet.update_binding = UpdateBinding::BoundToRegistryIdentity;
    sheet.rendered_trust_tier = sheet.computed_rendered_trust_tier();
    if let Some(baseline) = sheet.installed_baseline.as_mut() {
        baseline.update_binding = UpdateBinding::BoundToRegistryIdentity;
        baseline.rendered_trust_tier = TrustPosture::UnsignedLocalOnly;
    }
    let violations = packet.validate();
    assert!(
        violations
            .iter()
            .any(|v| matches!(v, M5SideloadReviewViolation::TrustContinuityElevated { .. })),
        "expected a trust-continuity violation, got {violations:?}"
    );
}

#[test]
fn detects_a_hidden_widening_trigger() {
    // Hiding a widening by clearing the trigger set must fail validation.
    let mut packet = packet();
    let sheet = packet
        .review_sheets
        .iter_mut()
        .find(|s| s.sheet_id == "sheet:model_pack_executable_introduced")
        .expect("model sheet present");
    sheet.review_triggers.clear();
    sheet.disposition = SideloadDisposition::ReviewedInstallReady;
    let violations = packet.validate();
    assert!(
        violations
            .iter()
            .any(|v| matches!(v, M5SideloadReviewViolation::ReviewTriggersMismatch { .. })),
        "expected a review-triggers mismatch, got {violations:?}"
    );
}

#[test]
fn detects_an_inconsistent_archive_source() {
    // An archive bundle must carry a content address; dropping it is flagged.
    let mut packet = packet();
    let sheet = packet
        .review_sheets
        .iter_mut()
        .find(|s| s.sheet_id == "sheet:docs_pack_archive_bind_later")
        .expect("docs sheet present");
    sheet.source.content_address_ref = None;
    let violations = packet.validate();
    assert!(
        violations.iter().any(|v| matches!(
            v,
            M5SideloadReviewViolation::SourceIdentityInconsistent { .. }
        )),
        "expected a source-identity inconsistency, got {violations:?}"
    );
}

#[test]
fn paths_and_record_kind_are_stable() {
    assert_eq!(
        M5_SIDELOAD_REVIEW_PATH,
        "artifacts/ecosystem/m5/m5-sideload-review.json"
    );
    assert_eq!(M5_SIDELOAD_REVIEW_RECORD_KIND, "m5_sideload_review");
}
