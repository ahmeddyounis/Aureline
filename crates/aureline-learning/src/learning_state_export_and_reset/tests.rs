use super::*;

// ── Seed integrity ──────────────────────────────────────────────────────────

#[test]
fn seeded_manifest_validates() {
    let manifest = seeded_m5_learning_state_export_and_reset();
    validate_m5_learning_state_export_and_reset(&manifest)
        .expect("seeded learning-state portability manifest must pass validation");
}

#[test]
fn seeded_manifest_serializes_and_roundtrips() {
    let manifest = seeded_m5_learning_state_export_and_reset();
    let json = serde_json::to_string_pretty(&manifest).expect("serialize");
    let back = reopen_portability_manifest_from_json(&json).expect("deserialize");
    assert_eq!(manifest, back);
    // Provenance, redaction, continuity, source-language escape, and reset scope
    // all survive export + reopen unchanged.
    for (orig, reopened) in manifest
        .export_bundles
        .iter()
        .zip(back.export_bundles.iter())
    {
        assert_eq!(orig.source_state_refs, reopened.source_state_refs);
        assert_eq!(orig.redaction, reopened.redaction);
        assert_eq!(orig.cached_pack, reopened.cached_pack);
        assert_eq!(orig.source_language, reopened.source_language);
    }
    for (orig, reopened) in manifest.reset_plans.iter().zip(back.reset_plans.iter()) {
        assert_eq!(orig.target_state_kinds, reopened.target_state_kinds);
        assert_eq!(orig.protected_classes, reopened.protected_classes);
        assert_eq!(orig.restore_command_ref, reopened.restore_command_ref);
    }
}

#[test]
fn ships_bundles_and_plans_across_families_and_flow_kinds() {
    let manifest = seeded_m5_learning_state_export_and_reset();
    assert!(manifest.export_bundles.len() >= 3);
    assert!(manifest.reset_plans.len() >= 2);
    let families: BTreeSet<_> = manifest.export_bundles.iter().map(|b| b.family).collect();
    assert!(families.len() >= 3, "bundles should span several families");
    let kinds: BTreeSet<_> = manifest
        .export_bundles
        .iter()
        .map(|b| b.state_kind)
        .collect();
    assert!(kinds.len() >= 3, "bundles should span several state kinds");
}

// ── Acceptance: inspect/reset/export without losing provenance or privacy ────

#[test]
fn every_export_preserves_provenance_redacts_payloads_and_is_user_initiated() {
    let manifest = seeded_m5_learning_state_export_and_reset();
    for bundle in &manifest.export_bundles {
        assert!(
            bundle.provenance_preserved,
            "{} must preserve provenance",
            bundle.bundle_id
        );
        assert!(
            !bundle.source_state_refs.is_empty(),
            "{} must carry a provenance trail",
            bundle.bundle_id
        );
        assert!(bundle.redaction.qualifies_stable(), "{}", bundle.bundle_id);
        assert!(bundle.user_initiated, "{}", bundle.bundle_id);
        assert!(bundle.safe_for_support_export, "{}", bundle.bundle_id);
    }
}

#[test]
fn no_export_widens_data_sharing_or_leaves_user_owned_local_first() {
    let manifest = seeded_m5_learning_state_export_and_reset();
    for bundle in &manifest.export_bundles {
        assert!(
            !bundle.redaction.widens_data_sharing,
            "{}",
            bundle.bundle_id
        );
        assert_eq!(
            bundle.data_ownership,
            DataOwnershipClass::UserOwnedLocalFirst,
            "{}",
            bundle.bundle_id
        );
    }
}

// ── Acceptance: source-language escapes and cached-pack continuity visible ───

#[test]
fn localized_exports_keep_a_command_backed_source_language_escape() {
    let manifest = seeded_m5_learning_state_export_and_reset();
    let mut saw_localized = false;
    for bundle in &manifest.export_bundles {
        if bundle.source_language.presented_localized {
            saw_localized = true;
            assert!(
                bundle.source_language.escape_to_source_available,
                "{} localized without an escape",
                bundle.bundle_id
            );
            assert!(
                bundle.source_language.escape_command_ref.is_some(),
                "{} escape is not command-backed",
                bundle.bundle_id
            );
            assert_ne!(
                bundle.source_language.source_locale, bundle.source_language.presented_locale,
                "{} claims localization but locales match",
                bundle.bundle_id
            );
        }
        assert!(
            bundle.source_language.preserves_provenance,
            "{} localization drops provenance",
            bundle.bundle_id
        );
    }
    assert!(saw_localized, "seed should exercise a localized export");
}

#[test]
fn cached_or_mirrored_packs_disclose_their_continuity() {
    let manifest = seeded_m5_learning_state_export_and_reset();
    let mut saw_non_live = false;
    for bundle in &manifest.export_bundles {
        if !bundle.cached_pack.freshness.is_live() {
            saw_non_live = true;
            assert!(
                bundle.cached_pack.continuity_disclosed,
                "{} serves a non-live pack without disclosure",
                bundle.bundle_id
            );
        }
        assert!(
            bundle.cached_pack.disclosure_is_honest(),
            "{}",
            bundle.bundle_id
        );
    }
    assert!(saw_non_live, "seed should exercise a non-live pack");
}

#[test]
fn cached_pack_export_narrows_to_beta_but_stays_disclosed() {
    let manifest = seeded_m5_learning_state_export_and_reset();
    let cached = manifest
        .export_bundles
        .iter()
        .find(|b| b.cached_pack.freshness == FreshnessState::CachedDisclosed)
        .expect("a cached export bundle");
    assert_eq!(cached.verdict, QualificationVerdict::NarrowedBeta);
    assert!(cached
        .narrowing_reasons
        .iter()
        .any(|r| r.contains("cached_pack_content_may_lag")));
    // The narrowest member propagates to the overall verdict.
    assert_eq!(manifest.overall_verdict, QualificationVerdict::NarrowedBeta);
}

#[test]
fn mirror_synced_disclosed_export_is_stable() {
    let manifest = seeded_m5_learning_state_export_and_reset();
    let mirror = manifest
        .export_bundles
        .iter()
        .find(|b| b.cached_pack.freshness == FreshnessState::MirrorSyncedDisclosed)
        .expect("a mirror-synced export bundle");
    assert_eq!(mirror.verdict, QualificationVerdict::QualifiedStable);
    assert!(mirror.cached_pack.continuity_disclosed);
}

// ── Acceptance: reset preserves unrelated user state and is reversible ───────

#[test]
fn every_reset_protects_docs_packs_bookmarks_and_notes() {
    let manifest = seeded_m5_learning_state_export_and_reset();
    for plan in &manifest.reset_plans {
        assert!(
            plan.protects_required_classes(),
            "{} fails to protect a required class",
            plan.plan_id
        );
        let protected = plan.protected_set();
        assert!(protected.contains(&ProtectedStateClass::DocsPack));
        assert!(protected.contains(&ProtectedStateClass::Bookmark));
        assert!(protected.contains(&ProtectedStateClass::UserAuthoredNote));
    }
}

#[test]
fn every_reset_is_reversible_and_scoped() {
    let manifest = seeded_m5_learning_state_export_and_reset();
    for plan in &manifest.reset_plans {
        assert!(!plan.target_state_kinds.is_empty(), "{}", plan.plan_id);
        assert!(!plan.silently_deletes_outside_scope, "{}", plan.plan_id);
        assert!(plan.restore_available, "{}", plan.plan_id);
        assert!(plan.restore_window_disclosed, "{}", plan.plan_id);
        assert!(plan.restore_command_ref.is_some(), "{}", plan.plan_id);
        assert!(plan.user_initiated, "{}", plan.plan_id);
        assert_eq!(
            plan.verdict,
            QualificationVerdict::QualifiedStable,
            "{}",
            plan.plan_id
        );
    }
}

// ── Guardrails: no hidden mutating tutorial path ─────────────────────────────

#[test]
fn no_export_or_reset_opens_a_mutating_tutorial_shortcut() {
    let manifest = seeded_m5_learning_state_export_and_reset();
    for bundle in &manifest.export_bundles {
        assert!(
            bundle.mutation_fence.qualifies_stable(),
            "{}",
            bundle.bundle_id
        );
        assert!(
            !bundle
                .mutation_fence
                .introduces_tutorial_only_mutating_shortcut
        );
        assert!(!bundle.mutation_fence.bypasses_preview_approval);
        assert!(!bundle.mutation_fence.authority_boundary_change_allowed);
        assert!(bundle.mutation_fence.command_graph_unchanged);
    }
    for plan in &manifest.reset_plans {
        assert!(plan.mutation_fence.qualifies_stable(), "{}", plan.plan_id);
        assert!(
            !plan
                .mutation_fence
                .introduces_tutorial_only_mutating_shortcut
        );
        assert!(!plan.mutation_fence.bypasses_preview_approval);
    }
}

#[test]
fn stored_verdicts_agree_with_derived_verdicts() {
    let manifest = seeded_m5_learning_state_export_and_reset();
    for bundle in &manifest.export_bundles {
        let (derived, reasons) = derive_export_bundle_verdict(bundle);
        assert_eq!(derived, bundle.verdict, "{}", bundle.bundle_id);
        assert_eq!(reasons, bundle.narrowing_reasons, "{}", bundle.bundle_id);
    }
    for plan in &manifest.reset_plans {
        let (derived, reasons) = derive_reset_plan_verdict(plan);
        assert_eq!(derived, plan.verdict, "{}", plan.plan_id);
        assert_eq!(reasons, plan.narrowing_reasons, "{}", plan.plan_id);
    }
}

// ── Negative: validation catches every invariant breach ──────────────────────

#[test]
fn validation_catches_export_that_does_not_redact_payloads() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.export_bundles[0].redaction.redacts_raw_payloads = false;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("redact raw payloads")));
}

#[test]
fn validation_catches_export_that_widens_data_sharing() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.export_bundles[0].redaction.widens_data_sharing = true;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("widens data sharing")));
}

#[test]
fn validation_catches_non_user_owned_export() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.export_bundles[0].data_ownership = DataOwnershipClass::RepoVisibleShared;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("user-owned local-first")));
}

#[test]
fn validation_catches_export_that_drops_provenance() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.export_bundles[0].provenance_preserved = false;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("drops provenance")));
}

#[test]
fn validation_catches_silent_export() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.export_bundles[0].user_initiated = false;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("silent export")));
}

#[test]
fn validation_catches_localized_export_without_source_language_escape() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    let bundle = manifest
        .export_bundles
        .iter_mut()
        .find(|b| b.source_language.presented_localized)
        .expect("a localized export");
    bundle.source_language.escape_to_source_available = false;
    bundle.source_language.escape_command_ref = None;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("source-language escape")));
}

#[test]
fn validation_catches_undisclosed_non_live_pack_masquerade() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    let bundle = manifest
        .export_bundles
        .iter_mut()
        .find(|b| !b.cached_pack.freshness.is_live())
        .expect("a non-live pack export");
    bundle.cached_pack.continuity_disclosed = false;
    bundle.sync_verdict();
    // An undisclosed non-live pack is a masquerade: it narrows to Preview, not Beta.
    assert_eq!(bundle.verdict, QualificationVerdict::NarrowedPreview);
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("masquerade")));
}

#[test]
fn validation_catches_export_mutating_tutorial_shortcut() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.export_bundles[0]
        .mutation_fence
        .introduces_tutorial_only_mutating_shortcut = true;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("tutorial-only mutating shortcut")));
}

#[test]
fn validation_catches_export_that_touches_workspace_outside_standard_model() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    let fence = &mut manifest.export_bundles[0].mutation_fence;
    fence.touches_real_workspace_state = true;
    fence.uses_standard_preview_approval_when_touching_workspace = false;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e
        .message
        .contains("outside the standard preview/approval model")));
}

#[test]
fn validation_catches_reset_that_deletes_outside_scope() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.reset_plans[0].silently_deletes_outside_scope = true;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("delete state outside its scope")));
}

#[test]
fn validation_catches_reset_that_does_not_protect_docs_packs() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.reset_plans[0]
        .protected_classes
        .retain(|c| *c != ProtectedStateClass::DocsPack);
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("does not protect docs_pack")));
}

#[test]
fn validation_catches_reset_that_does_not_protect_bookmarks_or_notes() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.reset_plans[0].protected_classes.retain(|c| {
        *c != ProtectedStateClass::Bookmark && *c != ProtectedStateClass::UserAuthoredNote
    });
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("does not protect bookmark")));
    assert!(errors
        .iter()
        .any(|e| e.message.contains("does not protect user_authored_note")));
}

#[test]
fn validation_catches_irreversible_reset() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.reset_plans[0].restore_available = false;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("not reversible")));
}

#[test]
fn validation_catches_reset_with_uncommand_backed_restore() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.reset_plans[0].restore_command_ref = None;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("restore is not command-backed")));
}

#[test]
fn validation_catches_reset_with_empty_scope() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.reset_plans[0].target_state_kinds.clear();
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("no target scope")));
}

#[test]
fn validation_catches_silent_reset() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.reset_plans[0].user_initiated = false;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("silent reset")));
}

#[test]
fn validation_catches_duplicate_bundle_id() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    let clone = manifest.export_bundles[0].clone();
    manifest.export_bundles.push(clone);
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("duplicate bundle id")));
}

#[test]
fn validation_catches_duplicate_plan_id() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    let clone = manifest.reset_plans[0].clone();
    manifest.reset_plans.push(clone);
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("duplicate plan id")));
}

#[test]
fn validation_catches_manifest_with_no_export_bundle() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.export_bundles.clear();
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("no export bundle")));
}

#[test]
fn validation_catches_manifest_with_no_reset_plan() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.reset_plans.clear();
    manifest.sync_verdicts();
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("no reset plan")));
}

#[test]
fn validation_catches_stored_verdict_divergence() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    // Introduce a hard violation but do NOT re-derive: the stored verdict lies.
    manifest.export_bundles[0].provenance_preserved = false;
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("disagrees with derived verdict")));
}

#[test]
fn validation_catches_manifest_overall_verdict_drift() {
    let mut manifest = seeded_m5_learning_state_export_and_reset();
    manifest.overall_verdict = QualificationVerdict::QualifiedStable;
    let errors = validate_m5_learning_state_export_and_reset(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("overall verdict")));
}

// ── Lookups ──────────────────────────────────────────────────────────────────

#[test]
fn bundle_and_plan_lookups_resolve_known_ids_and_reject_unknown() {
    let manifest = seeded_m5_learning_state_export_and_reset();
    let first_bundle = manifest.export_bundles[0].bundle_id.clone();
    assert!(manifest.export_bundle(&first_bundle).is_some());
    assert!(manifest.export_bundle("learning:m5:export:nope").is_none());
    let first_plan = manifest.reset_plans[0].plan_id.clone();
    assert!(manifest.reset_plan(&first_plan).is_some());
    assert!(manifest.reset_plan("learning:m5:reset:nope").is_none());
}
