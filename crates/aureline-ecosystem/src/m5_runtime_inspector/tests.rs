use super::*;

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::current_m5_ecosystem_governance_matrix;

fn packet() -> M5RuntimeInspector {
    current_m5_runtime_inspector().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_RUNTIME_INSPECTOR_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_RUNTIME_INSPECTOR_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_cards() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_card_gate_is_consistent() {
    let packet = packet();
    assert!(packet.all_gates_consistent());
    for card in &packet.inspector_cards {
        assert_eq!(
            card.rendered_trust_tier,
            card.computed_rendered_trust_tier(),
            "card {} rendered trust tier diverges from the recomputed cap",
            card.card_id
        );
        assert_eq!(
            card.review_triggers,
            card.computed_review_triggers(),
            "card {} review triggers diverge from the recomputed set",
            card.card_id
        );
        assert_eq!(
            card.disposition,
            card.computed_disposition(),
            "card {} disposition diverges from the recomputed gate",
            card.card_id
        );
    }
}

#[test]
fn inspector_shows_the_required_runtime_facts() {
    // The acceptance surface: every card carries activation time, current host, granted
    // capabilities, logs, and recent failures, and offers a quarantine/disable path.
    let packet = packet();
    for card in &packet.inspector_cards {
        assert!(
            !card.granted_capabilities.is_empty(),
            "card {} surfaces no granted capabilities",
            card.card_id
        );
        assert!(
            !card.recent_logs.is_empty(),
            "card {} surfaces no logs",
            card.card_id
        );
        // The inspector always exposes its logs, so crash history is never hidden.
        assert!(
            card.offers_action(InspectorActionKind::ViewLogs),
            "card {} hides its logs",
            card.card_id
        );
        // Every card offers at least one quarantine, disable, or re-enable control.
        let offers_control = card.offers_action(InspectorActionKind::Quarantine)
            || card.offers_action(InspectorActionKind::DisableForWorkspace)
            || card.offers_action(InspectorActionKind::DisableGlobally)
            || card.offers_action(InspectorActionKind::ReEnable);
        assert!(
            offers_control,
            "card {} offers no quarantine/disable/re-enable control",
            card.card_id
        );
    }
}

#[test]
fn local_or_untrusted_packages_never_inherit_a_trusted_badge() {
    // The non-inheritance guardrail: an unsigned local-dev, side-loaded, or revoked
    // artifact renders local-only even when built on a machine that holds a trusted key.
    let packet = packet();
    for card in &packet.inspector_cards {
        if card.signature_state.is_local_or_untrusted() {
            assert_eq!(
                card.rendered_trust_tier,
                TrustPosture::UnsignedLocalOnly,
                "card {} renders {} for a local/untrusted artifact",
                card.card_id,
                card.rendered_trust_tier.as_str()
            );
            assert!(!card.rendered_trust_tier.is_trusted_badge());
        }
    }
    // A genuinely verified package may carry a real badge.
    let framework = packet
        .inspector_card("card:framework_pack_running_healthy")
        .expect("framework card present");
    assert_eq!(framework.signature_state, SignatureState::SignedVerified);
    assert_eq!(
        framework.rendered_trust_tier,
        TrustPosture::EnterpriseApproved
    );
}

#[test]
fn failing_or_missing_source_cards_keep_last_known_good_visible() {
    // The keep-useful-when-failing guardrail.
    let packet = packet();
    assert!(packet.all_required_last_known_good_present());
    for card in &packet.inspector_cards {
        if card.requires_last_known_good() {
            assert!(
                card.last_known_good_visible(),
                "card {} failed/lost source but hides its last-known-good state",
                card.card_id
            );
            // A stale good state never overstates the current trust cap.
            assert!(card.last_known_good_within_cap());
        }
    }
}

#[test]
fn widening_hot_reload_forces_a_fresh_review() {
    // The lane guardrail: a hot reload that widens authority must force a fresh review
    // rather than applying silently, and the restart/reload actions must be held.
    let packet = packet();
    let recipe = packet
        .inspector_card("card:recipe_pack_fresh_review_required")
        .expect("recipe card present");
    assert_eq!(
        recipe.hot_reload_posture,
        HotReloadPosture::PermissionsWidenedPendingReview
    );
    assert_eq!(
        recipe.disposition,
        InspectorDisposition::FreshReviewRequired
    );
    assert!(recipe.offers_action(InspectorActionKind::RequestFreshReview));
    assert!(!recipe.offers_enabled_action(InspectorActionKind::Restart));
    assert!(!recipe.offers_enabled_action(InspectorActionKind::ReloadSource));
}

#[test]
fn undeclared_capability_is_surfaced_as_a_trigger() {
    let packet = packet();
    let sideload = packet
        .inspector_card("card:side_loaded_operator_disabled")
        .expect("side-load card present");
    assert!(sideload.has_undeclared_capability());
    assert!(sideload
        .review_triggers
        .contains(&InspectorReviewTrigger::UndeclaredCapabilityExercised));
    // The operator-disabled hold outranks the fresh-review trigger but never hides it.
    assert_eq!(sideload.disposition, InspectorDisposition::OperatorDisabled);
    assert!(sideload.offers_action(InspectorActionKind::ReEnable));
}

#[test]
fn quarantined_card_stays_inspectable() {
    let packet = packet();
    let mirror = packet
        .inspector_card("card:mirrored_variant_quarantined")
        .expect("mirror card present");
    assert!(mirror.is_quarantined());
    assert_eq!(mirror.disposition, InspectorDisposition::Quarantined);
    // Crash history and logs remain visible on a quarantined card.
    assert!(mirror.has_crash_loop());
    assert!(mirror.offers_action(InspectorActionKind::ViewLogs));
    assert!(mirror.offers_action(InspectorActionKind::ReEnable));
    assert!(!mirror.offers_enabled_action(InspectorActionKind::Restart));
}

#[test]
fn every_disposition_and_load_state_is_exercised() {
    let packet = packet();
    for disposition in InspectorDisposition::ALL {
        assert!(
            packet
                .inspector_cards
                .iter()
                .any(|c| c.disposition == disposition),
            "no card exercises disposition {}",
            disposition.as_str()
        );
    }
    for load_state in LoadState::ALL {
        assert!(
            packet
                .inspector_cards
                .iter()
                .any(|c| c.load_state == load_state),
            "no card exercises load state {}",
            load_state.as_str()
        );
    }
}

#[test]
fn governance_family_refs_resolve_to_known_families() {
    // Every card resolves through a governance-matrix family, so the inspector shares
    // one family vocabulary with the install-governance matrix.
    let packet = packet();
    let matrix = current_m5_ecosystem_governance_matrix().expect("matrix parses");
    let known: BTreeSet<ArtifactFamily> =
        matrix.families.iter().map(|r| r.artifact_family).collect();
    for card in &packet.inspector_cards {
        assert!(
            known.contains(&card.package_kind),
            "card {} package kind {} is not a known governance family",
            card.card_id,
            card.package_kind.as_str()
        );
        assert!(
            !card.governance_family_ref.trim().is_empty(),
            "card {} has an empty governance family ref",
            card.card_id
        );
    }
}

#[test]
fn export_projection_mirrors_the_cards() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.rows.len(), packet.inspector_cards.len());
    assert!(projection.all_gates_consistent);
    assert_eq!(projection.fresh_review_required_count, 1);
    assert_eq!(projection.held_count, 2);
    for (row, card) in projection.rows.iter().zip(&packet.inspector_cards) {
        assert_eq!(row.card_id, card.card_id);
        assert_eq!(row.disposition, card.disposition.as_str());
        assert_eq!(row.rendered_trust_tier, card.rendered_trust_tier.as_str());
        assert_eq!(row.activation_millis, card.activation.activation_millis);
        assert_eq!(row.last_known_good_visible, card.last_known_good_visible());
    }
}

#[test]
fn detects_an_overstated_trust_tier() {
    // A local/untrusted package that claims a stronger rendered badge than its signing
    // state allows is flagged by the gate recompute.
    let mut packet = packet();
    let card = packet
        .inspector_cards
        .iter_mut()
        .find(|c| c.card_id == "card:model_pack_running_degraded")
        .expect("model card present");
    card.rendered_trust_tier = TrustPosture::EnterpriseApproved;
    let violations = packet.validate();
    assert!(
        violations
            .iter()
            .any(|v| matches!(v, M5RuntimeInspectorViolation::RenderedTrustMismatch { .. })),
        "expected a rendered-trust mismatch, got {violations:?}"
    );
    assert!(
        violations.iter().any(|v| matches!(
            v,
            M5RuntimeInspectorViolation::LocalPackageInheritsTrustedBadge { .. }
        )),
        "expected a trusted-badge inheritance violation, got {violations:?}"
    );
}

#[test]
fn detects_a_dropped_last_known_good() {
    // Hiding the last-known-good state on a failed-load card must fail validation.
    let mut packet = packet();
    let card = packet
        .inspector_cards
        .iter_mut()
        .find(|c| c.card_id == "card:bridge_pack_load_failed")
        .expect("bridge card present");
    card.last_known_good = None;
    let violations = packet.validate();
    assert!(
        violations
            .iter()
            .any(|v| matches!(v, M5RuntimeInspectorViolation::MissingLastKnownGood { .. })),
        "expected a missing-last-known-good violation, got {violations:?}"
    );
}

#[test]
fn detects_a_silent_widening_hot_reload() {
    // Enabling restart on a fresh-review-required card would apply the widening through
    // a silent hot reload and must fail validation.
    let mut packet = packet();
    let card = packet
        .inspector_cards
        .iter_mut()
        .find(|c| c.card_id == "card:recipe_pack_fresh_review_required")
        .expect("recipe card present");
    for action in &mut card.actions {
        if action.action_kind == InspectorActionKind::Restart {
            action.enabled = true;
        }
    }
    let violations = packet.validate();
    assert!(
        violations.iter().any(|v| matches!(
            v,
            M5RuntimeInspectorViolation::WideningAppliedWithoutReview { .. }
        )),
        "expected a widening-without-review violation, got {violations:?}"
    );
}

#[test]
fn detects_a_hidden_review_trigger() {
    // Clearing the recomputed trigger set must fail validation.
    let mut packet = packet();
    let card = packet
        .inspector_cards
        .iter_mut()
        .find(|c| c.card_id == "card:recipe_pack_fresh_review_required")
        .expect("recipe card present");
    card.review_triggers.clear();
    card.disposition = InspectorDisposition::RunningHealthy;
    let violations = packet.validate();
    assert!(
        violations.iter().any(|v| matches!(
            v,
            M5RuntimeInspectorViolation::ReviewTriggersMismatch { .. }
        )),
        "expected a review-triggers mismatch, got {violations:?}"
    );
}

#[test]
fn detects_a_last_known_good_badge_above_the_cap() {
    // A stale good state that renders a stronger badge than the current cap is flagged.
    let mut packet = packet();
    let card = packet
        .inspector_cards
        .iter_mut()
        .find(|c| c.card_id == "card:side_loaded_operator_disabled")
        .expect("side-load card present");
    if let Some(good) = card.last_known_good.as_mut() {
        good.rendered_trust_tier = TrustPosture::EnterpriseApproved;
    }
    let violations = packet.validate();
    assert!(
        violations.iter().any(|v| matches!(
            v,
            M5RuntimeInspectorViolation::LastKnownGoodTrustExceedsCap { .. }
        )),
        "expected a last-known-good cap violation, got {violations:?}"
    );
}

#[test]
fn runtime_class_and_external_executable_widenings_are_recognized() {
    // The two hot-reload widenings not present in the corpus are still recomputed into
    // their triggers and force a fresh review.
    let mut packet = packet();
    let card = packet
        .inspector_cards
        .iter_mut()
        .find(|c| c.card_id == "card:framework_pack_running_healthy")
        .expect("framework card present");
    card.hot_reload_posture = HotReloadPosture::RuntimeClassWidenedPendingReview;
    assert_eq!(
        card.computed_review_triggers(),
        vec![InspectorReviewTrigger::RuntimeClassWidened]
    );
    assert_eq!(
        card.computed_disposition(),
        InspectorDisposition::FreshReviewRequired
    );

    card.hot_reload_posture = HotReloadPosture::ExternalExecutableAddedPendingReview;
    assert_eq!(
        card.computed_review_triggers(),
        vec![InspectorReviewTrigger::ExternalExecutableAdded]
    );
    assert_eq!(
        card.computed_disposition(),
        InspectorDisposition::FreshReviewRequired
    );
}

#[test]
fn paths_and_record_kind_are_stable() {
    assert_eq!(
        M5_RUNTIME_INSPECTOR_PATH,
        "artifacts/ecosystem/m5/m5-runtime-inspector.json"
    );
    assert_eq!(M5_RUNTIME_INSPECTOR_RECORD_KIND, "m5_runtime_inspector");
}
