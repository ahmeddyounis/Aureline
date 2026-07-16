//! Canonical seed for the constrained-state drill corpus.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV, Markdown
//! summary, health dashboard, and narrowed fixtures. Every binding is derived from one per-fixture
//! [`ConstrainedStateGrammar`] so the same seeded fixture always carries the same constrained-state grammar across
//! surfaces, and every drill derives its object class, co-applicable class, blocked-write reason, chosen fallback
//! path, write disposition, checkpoint / undo class, parity, denial expectation, and action set from
//! [`resolve_drill_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

/// The full accessibility route set every drill offers so the state class, canonical source, and write target are
/// discoverable without pointer-only chrome.
fn all_accessibility_routes() -> Vec<M5ConstrainedFileStateAccessibilityRoute> {
    M5ConstrainedFileStateAccessibilityRoute::ALL.to_vec()
}

fn grammar(
    state_role: &str,
    state_class_label: &str,
    blocked_write_reason: &str,
    canonical_source: &str,
    exact_write_target: &str,
    write_disposition: &str,
) -> ConstrainedStateGrammar {
    ConstrainedStateGrammar {
        state_role_word: state_role.to_owned(),
        state_class_label_word: state_class_label.to_owned(),
        blocked_write_reason_word: blocked_write_reason.to_owned(),
        canonical_source_word: canonical_source.to_owned(),
        exact_write_target_word: exact_write_target.to_owned(),
        write_disposition_word: write_disposition.to_owned(),
    }
}

fn canonical_source_join_for(object_class: M5ConstrainedFileStateObject) -> CanonicalSourceJoin {
    CanonicalSourceJoin {
        canonical_source_ref: format!("canonical-source-{}", object_class.as_str()),
        exact_write_target_ref: format!("exact-write-target-{}", object_class.as_str()),
        owning_authority_ref: format!("owning-authority-{}", object_class.as_str()),
        preserved_versus_lost_sync_ref: format!(
            "preserved-versus-lost-sync-{}",
            object_class.as_str()
        ),
    }
}

fn denial_note_for(drill: DrillScenario) -> String {
    match drill {
        DrillScenario::SymlinkAliasSaveDenied => {
            "Denied, read-only alias path: in-place save is blocked; duplicate to an editable copy as a reviewed transition before commit."
        }
        DrillScenario::GeneratedArtifactDriftDenied => {
            "Denied, generated artifact regenerate-only: the artifact has drifted from its generator; regenerate from the canonical source with a preview instead of a lossy direct edit."
        }
        DrillScenario::PolicyLockedManagedMirrorDenied => {
            "Denied, policy lock plus managed mirror: an in-place write is gated; request approval, and both the policy-lock and managed-mirror facets stay visible."
        }
        DrillScenario::ProjectionExportDenied => {
            "Denied, projection over a captured snapshot: writing requires an overlay patch or detach; both the projection and captured-snapshot facets stay visible."
        }
        DrillScenario::CapturedSnapshotInWorkspaceDenied => {
            "Denied, captured snapshot restore-only: the snapshot is not the live object; duplicate to an editable copy rather than mutating the snapshot in place."
        }
        DrillScenario::ManagedMirrorRoundTripDenied => {
            "Denied, managed source requires detach: this unsupported round trip is blocked; detach from the managed source as a reviewed transition."
        }
        DrillScenario::ReadOnlyGeneratedOverlayDenied => {
            "Denied, read-only plus generated: an in-place save is blocked; duplicate to an editable copy, and both the read-only and generated facets stay visible."
        }
        DrillScenario::GeneratedPolicyLockedRegenDenied => {
            "Denied, generated plus policy-locked: a direct edit is blocked; regenerate with a preview, and both the generated and policy-locked facets stay visible."
        }
        DrillScenario::ManagedCapturedSnapshotRestoreDenied => {
            "Denied, managed plus captured snapshot: a direct write is blocked; detach from the managed source, and both the managed and captured-snapshot facets stay visible."
        }
    }
    .to_owned()
}

fn make_denial_expectation(
    object_class: M5ConstrainedFileStateObject,
    drill: DrillScenario,
    disclosure: DrillDisclosure,
) -> DenialExpectation {
    let co_applicable_state_ref = disclosure
        .co_applicable_object_class
        .map(|co| format!("co-applicable-state-{}", co.as_str()));
    DenialExpectation {
        blocked_write_reason: disclosure.blocked_write_reason,
        chosen_fallback_path: disclosure.chosen_fallback_path,
        required_write_disposition: disclosure.required_write_disposition,
        checkpoint_undo_class: disclosure.checkpoint_undo_class,
        reviewed_fallback_ref: Some(format!(
            "reviewed-fallback-{}-{}",
            object_class.as_str(),
            disclosure.chosen_fallback_path.as_str()
        )),
        co_applicable_state_ref,
        denial_note: denial_note_for(drill),
    }
}

fn corpus_evidence_for(binding_id: &str) -> CorpusEvidenceBindings {
    CorpusEvidenceBindings {
        screenshot_ref: format!(
            "artifacts/support/m5-constrained-state-drills/screenshots/{binding_id}.png"
        ),
        accessibility_check_ref: format!(
            "artifacts/support/m5-constrained-state-drills/accessibility/{binding_id}.json"
        ),
        cli_support_export_ref: M5_CONSTRAINED_STATE_DRILL_ARTIFACT_REF.to_owned(),
        health_dashboard_ref: M5_CONSTRAINED_STATE_DRILL_DASHBOARD_REF.to_owned(),
    }
}

fn allowed_actions() -> Vec<DrillDenialAction> {
    let mut actions = DrillDenialAction::BASE.to_vec();
    actions.push(DrillDenialAction::OpenReviewedFallbackPath);
    actions
}

fn binding_refs(object_class: M5ConstrainedFileStateObject) -> Vec<String> {
    vec![
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF.to_owned(),
        object_class.canonical_domain_schema_ref().to_owned(),
    ]
}

fn make_binding(
    binding_id: &str,
    fixture_id: &str,
    fixture_label: &str,
    object_class: M5ConstrainedFileStateObject,
    consumer: M5ConstrainedFileStateConsumerSurface,
    drill: DrillScenario,
    constrained_grammar: ConstrainedStateGrammar,
) -> DrillCorpusBinding {
    let disclosure = resolve_drill_disclosure(drill);
    DrillCorpusBinding {
        binding_id: binding_id.to_owned(),
        fixture_id: fixture_id.to_owned(),
        fixture_label: fixture_label.to_owned(),
        object_class,
        co_applicable_object_class: disclosure.co_applicable_object_class,
        consumer,
        drill,
        drill_label: drill.stable_label().to_owned(),
        blocked_write_reason: disclosure.blocked_write_reason,
        chosen_fallback_path: disclosure.chosen_fallback_path,
        write_disposition: disclosure.required_write_disposition,
        checkpoint_undo_class: disclosure.checkpoint_undo_class,
        constrained_grammar,
        is_mixed_state: disclosure.is_mixed_state,
        parity_state: disclosure.parity,
        allowed_actions: allowed_actions(),
        accessibility_routes: all_accessibility_routes(),
        canonical_source_join: canonical_source_join_for(object_class),
        denial_expectation: make_denial_expectation(object_class, drill, disclosure),
        corpus_evidence: corpus_evidence_for(binding_id),
        constrained_state_explicitly_classified: true,
        both_state_facets_visible_when_mixed: true,
        lets_one_constrained_state_class_hide_another: false,
        silently_falls_back_to_lossy_direct_write: false,
        gives_ai_automation_import_or_repair_a_hidden_bypass: false,
        leaves_canonical_source_or_exact_write_target_unstated: false,
        presents_as_directly_writable_or_hides_recovery_path: false,
        source_contract_refs: binding_refs(object_class),
    }
}

/// One consumer-surface adoption of a seeded fixture, before any drill override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5ConstrainedFileStateConsumerSurface,
    drill: DrillScenario,
}

/// One seeded constrained-object fixture exercised across several consumer surfaces at one constrained-state grammar.
struct FixtureSpec {
    fixture_id: &'static str,
    fixture_label: &'static str,
    object_class: M5ConstrainedFileStateObject,
    grammar: ConstrainedStateGrammar,
    bindings: Vec<BindingSpec>,
}

fn spec(
    fixture_id: &'static str,
    fixture_label: &'static str,
    object_class: M5ConstrainedFileStateObject,
    grammar: ConstrainedStateGrammar,
    bindings: Vec<BindingSpec>,
) -> FixtureSpec {
    FixtureSpec {
        fixture_id,
        fixture_label,
        object_class,
        grammar,
        bindings,
    }
}

fn bs(
    binding_id: &'static str,
    consumer: M5ConstrainedFileStateConsumerSurface,
    drill: DrillScenario,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        drill,
    }
}

/// The six seeded fixture families — one per constrained-object class — and the drills that exercise each across the
/// nine shared consumer surfaces (tab chrome, breadcrumb trail, status bar, command palette, editor banner, diff /
/// review header, write-review sheet, AI / automation path, and support / export packet).
fn fixture_specs() -> Vec<FixtureSpec> {
    use DrillScenario::*;
    use M5ConstrainedFileStateConsumerSurface::*;
    use M5ConstrainedFileStateObject::*;

    vec![
        spec(
            "read-only-alias-path",
            "Read-only symlink / alias path fixture",
            ReadOnly,
            grammar(
                "state_badge_classification",
                "read_only",
                "read_only_path_not_directly_writable",
                "canonical_owning_object",
                "editable_copy_write_target",
                "read_only_blocked",
            ),
            vec![
                bs("csd-ro-tab", TabChrome, SymlinkAliasSaveDenied),
                bs(
                    "csd-ro-editor",
                    EditorBanner,
                    ReadOnlyGeneratedOverlayDenied,
                ),
                bs(
                    "csd-ro-support",
                    SupportExportPacket,
                    SymlinkAliasSaveDenied,
                ),
            ],
        ),
        spec(
            "generated-derived-artifact",
            "Generated / derived artifact fixture",
            Generated,
            grammar(
                "blocked_write_reason",
                "generated",
                "generated_artifact_regenerate_only",
                "generator_canonical_source",
                "regenerated_artifact_write_target",
                "regenerate_only",
            ),
            vec![
                bs("csd-gen-status", StatusBar, GeneratedArtifactDriftDenied),
                bs(
                    "csd-gen-diff",
                    DiffReviewHeader,
                    GeneratedPolicyLockedRegenDenied,
                ),
                bs(
                    "csd-gen-palette",
                    CommandPalette,
                    GeneratedArtifactDriftDenied,
                ),
            ],
        ),
        spec(
            "policy-locked-managed-mirror",
            "Policy-locked managed mirror fixture",
            PolicyLocked,
            grammar(
                "canonical_source_relation",
                "policy_locked",
                "policy_lock_requires_approval",
                "policy_owner_authority",
                "approval_gated_write_target",
                "approval_gated",
            ),
            vec![
                bs(
                    "csd-pol-breadcrumb",
                    BreadcrumbTrail,
                    PolicyLockedManagedMirrorDenied,
                ),
                bs(
                    "csd-pol-write",
                    WriteReviewSheet,
                    PolicyLockedManagedMirrorDenied,
                ),
            ],
        ),
        spec(
            "projection-virtual-view",
            "Projection / virtual view fixture",
            Projection,
            grammar(
                "exact_write_target",
                "projection",
                "projection_requires_overlay_or_detach",
                "backing_source_object",
                "overlay_patch_write_target",
                "detach_required",
            ),
            vec![
                bs("csd-proj-ai", AiAutomationPath, ProjectionExportDenied),
                bs("csd-proj-tab", TabChrome, ProjectionExportDenied),
            ],
        ),
        spec(
            "managed-external-source",
            "Managed, externally-owned source fixture",
            Managed,
            grammar(
                "safe_next_step_guidance",
                "managed",
                "managed_source_requires_detach",
                "managing_owner_authority",
                "detached_copy_write_target",
                "detach_required",
            ),
            vec![
                bs(
                    "csd-man-write",
                    WriteReviewSheet,
                    ManagedMirrorRoundTripDenied,
                ),
                bs(
                    "csd-man-support",
                    SupportExportPacket,
                    ManagedCapturedSnapshotRestoreDenied,
                ),
                bs("csd-man-status", StatusBar, ManagedMirrorRoundTripDenied),
            ],
        ),
        spec(
            "captured-workspace-snapshot",
            "Captured snapshot in current workspace fixture",
            CapturedSnapshot,
            grammar(
                "allowed_blocked_action_set",
                "captured_snapshot",
                "captured_snapshot_restore_only",
                "live_object_source",
                "editable_copy_write_target",
                "read_only_blocked",
            ),
            vec![
                bs(
                    "csd-snap-editor",
                    EditorBanner,
                    CapturedSnapshotInWorkspaceDenied,
                ),
                bs(
                    "csd-snap-palette",
                    CommandPalette,
                    CapturedSnapshotInWorkspaceDenied,
                ),
            ],
        ),
    ]
}

/// Builds all drill bindings, applying `drill_override` to override a binding's drill.
fn build_bindings<F>(drill_override: F) -> Vec<DrillCorpusBinding>
where
    F: Fn(&str, DrillScenario) -> DrillScenario,
{
    let mut bindings = Vec::new();
    for fixture in fixture_specs() {
        for spec in &fixture.bindings {
            let drill = drill_override(spec.binding_id, spec.drill);
            bindings.push(make_binding(
                spec.binding_id,
                fixture.fixture_id,
                fixture.fixture_label,
                fixture.object_class,
                spec.consumer,
                drill,
                fixture.grammar.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> DrillCorpusTrustReview {
    DrillCorpusTrustReview {
        covers_every_object_class_as_primary: true,
        covers_five_or_more_mixed_state_combinations: true,
        exact_blocked_write_reasons_are_distinguishable: true,
        every_object_class_seeded_by_two_or_more_consumers: true,
        constrained_grammar_identical_for_same_fixture: true,
        state_role_words_stay_in_frozen_vocabulary: true,
        canonical_source_and_write_target_present_on_every_binding: true,
        every_denial_routes_to_reviewed_fallback_keyed_to_state_class: true,
        no_drill_silently_falls_back_to_lossy_direct_write: true,
        no_mixed_state_drill_hides_second_state: true,
        no_cross_surface_disagreement: true,
        no_ai_automation_import_or_repair_bypass: true,
        support_export_can_replay_denial_and_fallback: true,
        corpus_bound_to_screenshots_accessibility_cli_and_dashboards: true,
        corpus_referenced_by_release_and_support_not_ad_hoc: true,
        accessibility_routes_present_for_state_source_and_target: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> DrillCorpusProjection {
    DrillCorpusProjection {
        tab_chrome_consumes_corpus: true,
        breadcrumb_trail_consumes_corpus: true,
        status_bar_consumes_corpus: true,
        command_palette_consumes_corpus: true,
        editor_banner_consumes_corpus: true,
        diff_review_header_consumes_corpus: true,
        write_review_sheet_consumes_corpus: true,
        ai_automation_path_consumes_corpus: true,
        support_export_packet_consumes_corpus: true,
        every_object_class_stated_by_two_or_more_consumers: true,
        constrained_grammar_identical_for_same_fixture: true,
        constrained_state_disclosed_not_hidden: true,
        drill_maps_back_to_one_constrained_object: true,
    }
}

fn proof_freshness() -> DrillCorpusProofFreshness {
    DrillCorpusProofFreshness {
        proof_freshness_slo_hours: M5_CONSTRAINED_STATE_DRILL_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_CONSTRAINED_STATE_DRILL_SCHEMA_REF.to_owned(),
        M5_CONSTRAINED_STATE_DRILL_DOC_REF.to_owned(),
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF.to_owned(),
        M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF.to_owned(),
    ];
    // The six object classes map to three canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object_class in M5ConstrainedFileStateObject::ALL {
        domains.insert(object_class.canonical_domain_schema_ref());
    }
    for domain in domains {
        refs.push(domain.to_owned());
    }
    refs
}

fn packet_from_bindings(
    packet_id: &str,
    surface_label: &str,
    drill_bindings: Vec<DrillCorpusBinding>,
) -> M5ConstrainedStateDrillCorpusPacket {
    M5ConstrainedStateDrillCorpusPacket::new(M5ConstrainedStateDrillCorpusPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        drill_bindings,
        downgrade_triggers: ConstrainedStateDrillDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5ConstrainedFileStateConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in constrained-state drill corpus.
pub fn seeded_m5_constrained_state_drill_corpus() -> M5ConstrainedStateDrillCorpusPacket {
    packet_from_bindings(
        M5_CONSTRAINED_STATE_DRILL_PACKET_ID,
        "M5 constrained-state drill corpus (mixed-state fixtures and regression drills)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: two single-state denials narrowed to their mixed-state siblings (read-only + generated and managed +
/// captured-snapshot), proving the corpus surfaces the co-applicable second state instead of hiding it behind the
/// primary badge. The original single-state drills stay covered on their other bindings.
pub fn seeded_m5_constrained_state_drill_corpus_mixed_state_narrowed(
) -> M5ConstrainedStateDrillCorpusPacket {
    packet_from_bindings(
        "m5-constrained-state-drill:mixed-state:0001",
        "M5 constrained-state drill corpus (mixed-state narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "csd-ro-tab" => DrillScenario::ReadOnlyGeneratedOverlayDenied,
            "csd-man-status" => DrillScenario::ManagedCapturedSnapshotRestoreDenied,
            _ => default,
        }),
    )
}

/// Fixture: a read-only single-state denial and a generated single-state denial narrowed to their mixed-state
/// siblings (read-only + generated and generated + policy-locked), proving the same denial-and-fallback replay stays
/// honest when a second state class becomes material. The original single-state drills stay covered on their other
/// bindings.
pub fn seeded_m5_constrained_state_drill_corpus_read_only_generated_narrowed(
) -> M5ConstrainedStateDrillCorpusPacket {
    packet_from_bindings(
        "m5-constrained-state-drill:read-only-generated:0001",
        "M5 constrained-state drill corpus (read-only + generated narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "csd-ro-support" => DrillScenario::ReadOnlyGeneratedOverlayDenied,
            "csd-gen-status" => DrillScenario::GeneratedPolicyLockedRegenDenied,
            _ => default,
        }),
    )
}
