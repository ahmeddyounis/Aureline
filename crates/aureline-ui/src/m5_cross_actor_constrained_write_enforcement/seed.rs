//! Canonical seed for the cross-actor constrained-write enforcement packet.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV, Markdown
//! summary, and narrowed fixtures. Every binding is derived from one per-profile [`GateResolution`] — a pure
//! function of the object's state class — so the same constrained-object profile always carries the same blocked
//! reason and safe next step across every actor routed against it, and every narrowed posture derives its
//! disclosure and action set from [`resolve_gate_render_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

/// The full accessibility route set every gate view offers so the reason, write target, and safe next step are
/// discoverable without pointer-only chrome.
fn all_accessibility_routes() -> Vec<M5ConstrainedFileStateAccessibilityRoute> {
    M5ConstrainedFileStateAccessibilityRoute::ALL.to_vec()
}

/// Builds the gate resolution for a constrained-object class — a pure function of the state class.
fn resolution_for(
    object_class: M5ConstrainedFileStateObject,
    co_applicable_state_labels: &[&str],
) -> GateResolution {
    let reason = BlockedWriteReason::for_object_class(object_class);
    let safe_next_step = reason.safe_next_step();
    let (exact_write_target, canonical_source, explanation) = match object_class {
        M5ConstrainedFileStateObject::ReadOnly => (
            "new_editable_copy_beside_the_read_only_source",
            "vendored_source_of_truth",
            "read_only_current_object_cannot_be_written_in_place_duplicate_to_an_editable_copy_beside_it",
        ),
        M5ConstrainedFileStateObject::Generated => (
            "regenerated_artifact_rendered_from_the_generator_input",
            "generator_source_of_truth",
            "generated_artifact_truth_lives_in_its_generator_regenerate_from_source_with_preview_not_a_direct_edit",
        ),
        M5ConstrainedFileStateObject::PolicyLocked => (
            "protected_config_pending_an_approved_change_request",
            "policy_owner_of_record",
            "policy_locked_object_write_is_gated_behind_an_approval_request_approval_from_the_policy_owner",
        ),
        M5ConstrainedFileStateObject::Managed => (
            "detached_local_copy_forked_from_the_managed_source",
            "upstream_managing_owner",
            "managed_object_is_owned_upstream_detach_from_the_managed_source_to_edit_a_local_copy",
        ),
        M5ConstrainedFileStateObject::Projection => (
            "overlay_patch_layered_over_the_backing_source",
            "backing_source_object",
            "projection_resolves_back_to_a_backing_source_record_edits_as_an_overlay_patch_not_in_place",
        ),
        M5ConstrainedFileStateObject::CapturedSnapshot => (
            "new_editable_copy_materialized_from_the_captured_snapshot",
            "live_object_of_record",
            "captured_snapshot_preserves_a_past_state_duplicate_it_into_a_new_editable_copy_leave_it_immutable",
        ),
    };
    GateResolution {
        blocked_write_reason: reason,
        write_disposition_word: safe_next_step
            .required_write_disposition()
            .as_str()
            .to_owned(),
        safe_next_step,
        checkpoint_undo_class: safe_next_step.required_checkpoint_undo_class(),
        exact_write_target_word: exact_write_target.to_owned(),
        canonical_source_word: canonical_source.to_owned(),
        structured_reason_explanation: explanation.to_owned(),
        co_applicable_state_labels: co_applicable_state_labels
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
    }
}

fn preserved_note_for(reason: GateNarrowReason) -> String {
    match reason {
        GateNarrowReason::FailedClosedOnActorContextDrift => {
            "blocked reason, exact write target, canonical source, and safe next step preserved; the gate failed closed and offers no write path until the actor context is resolved"
        }
        GateNarrowReason::ExportRedactionNarrowed => {
            "all gate-resolution content preserved; only surrounding detail is redacted export-safe"
        }
    }
    .to_owned()
}

fn next_action_label_for(action: GateNarrowNextAction) -> String {
    match action {
        GateNarrowNextAction::ResolveActorContextThenRetry => {
            "Resolve the actor context, then retry"
        }
        GateNarrowNextAction::OpenFullGateDetail => "Open the full gate detail",
    }
    .to_owned()
}

fn allowed_actions_for(disclosure: GateRenderDisclosure) -> Vec<GateAction> {
    let mut actions = GateAction::SAFE_BASE.to_vec();
    if disclosure.offers_open_safe_next_step {
        actions.push(GateAction::OpenSafeNextStepReview);
    }
    actions
}

fn binding_refs(object_class: M5ConstrainedFileStateObject) -> Vec<String> {
    vec![
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF.to_owned(),
        object_class.canonical_domain_schema_ref().to_owned(),
    ]
}

#[allow(clippy::too_many_arguments)]
fn make_binding(
    binding_id: &str,
    object_profile_id: &str,
    object_profile_label: &str,
    object_class: M5ConstrainedFileStateObject,
    co_applicable_states: Vec<M5ConstrainedFileStateObject>,
    actor: MutationActor,
    posture: GateEnforcementPosture,
    fail_closed_reason: Option<FailClosedReason>,
    resolution: GateResolution,
) -> CrossActorGateBinding {
    let disclosure = resolve_gate_render_disclosure(posture);
    let narrow_note = disclosure.narrow_reason.map(|reason| {
        let next_action = disclosure
            .narrow_next_action
            .expect("a narrow reason always carries a next action");
        GateNarrowNote {
            reason,
            preserved_content_note: preserved_note_for(reason),
            next_action,
            next_action_label: next_action_label_for(next_action),
        }
    });
    let export_detail_note = if disclosure.needs_export_detail_note {
        "surrounding detail redacted export-safe in this packet; full detail available on request"
            .to_owned()
    } else {
        String::new()
    };
    let effective_fail_closed_reason = if disclosure.is_fail_closed {
        Some(fail_closed_reason.unwrap_or(FailClosedReason::ActorContextDrifted))
    } else {
        None
    };
    let trace = ActorGateTrace {
        actor,
        blocked_write_reason: resolution.blocked_write_reason,
        chosen_fallback_path: resolution.safe_next_step,
        gate_posture: posture,
    };

    CrossActorGateBinding {
        binding_id: binding_id.to_owned(),
        object_profile_id: object_profile_id.to_owned(),
        object_profile_label: object_profile_label.to_owned(),
        object_class,
        co_applicable_states,
        actor,
        posture,
        resolution,
        parity_state: disclosure.parity_state,
        fail_closed_reason: effective_fail_closed_reason,
        trace,
        allowed_actions: allowed_actions_for(disclosure),
        accessibility_routes: all_accessibility_routes(),
        narrow_note,
        export_detail_note,
        routed_through_shared_gate: true,
        safe_next_step_keyed_to_state_class: true,
        silently_writes_constrained_object_bypassing_direct_typing: false,
        gives_ai_automation_import_or_repair_flows_a_hidden_bypass: false,
        uses_actor_specific_free_form_blocked_reason: false,
        leaves_exact_write_target_or_canonical_source_unstated: false,
        lets_one_state_class_hide_another_when_both_materially_affect_behavior: false,
        source_contract_refs: binding_refs(object_class),
    }
}

/// One actor routing of a constrained-object profile, before any posture override.
struct BindingSpec {
    binding_id: &'static str,
    actor: MutationActor,
    posture: GateEnforcementPosture,
    fail_closed_reason: Option<FailClosedReason>,
}

/// One constrained-object profile routed by several actors at one gate resolution.
struct ProfileSpec {
    profile_id: &'static str,
    profile_label: &'static str,
    object_class: M5ConstrainedFileStateObject,
    co_applicable_states: Vec<M5ConstrainedFileStateObject>,
    resolution: GateResolution,
    bindings: Vec<BindingSpec>,
}

fn bs(
    binding_id: &'static str,
    actor: MutationActor,
    posture: GateEnforcementPosture,
    fail_closed_reason: Option<FailClosedReason>,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        actor,
        posture,
        fail_closed_reason,
    }
}

/// The six constrained-object profiles — one per B150 constrained-file-state object class — each carrying one
/// state-class blocked reason and safe next step, routed through the shared gate by the direct-edit / save,
/// AI-apply, automation-recipe, importer, repair, and code-action actors. The managed profile is routed by an AI,
/// an importer, a repair, and a direct-save actor so the same blocked reason is proven across all four (AC1). Two
/// profiles are multi-state (`Generated` plus `Policy locked`, `Managed` plus `Captured snapshot`) so both facts
/// stay visible across actors.
fn profile_specs() -> Vec<ProfileSpec> {
    use FailClosedReason::*;
    use GateEnforcementPosture::*;
    use M5ConstrainedFileStateObject::*;
    use MutationActor::*;

    vec![
        ProfileSpec {
            profile_id: "read-only/vendored-source",
            profile_label: "Read-only vendored source (duplicate into an editable copy)",
            object_class: ReadOnly,
            co_applicable_states: vec![],
            resolution: resolution_for(ReadOnly, &[]),
            bindings: vec![
                bs(
                    "caw-readonly-directsave",
                    DirectEditSave,
                    EnforcedGate,
                    None,
                ),
                bs(
                    "caw-readonly-codeaction",
                    CodeAction,
                    FailClosedOnActorDrift,
                    Some(ExactWriteTargetNotTruthfullyExplainable),
                ),
                bs("caw-readonly-importer", Importer, ExportRedacted, None),
            ],
        },
        ProfileSpec {
            profile_id: "generated/policy-locked-artifact",
            profile_label:
                "Generated artifact that is also policy locked (regenerate with preview)",
            object_class: Generated,
            co_applicable_states: vec![PolicyLocked],
            resolution: resolution_for(Generated, &["policy_locked"]),
            bindings: vec![
                bs("caw-generated-aiapply", AiApply, EnforcedGate, None),
                bs(
                    "caw-generated-automation",
                    AutomationRecipe,
                    FailClosedOnActorDrift,
                    Some(ActorContextDrifted),
                ),
                bs("caw-generated-repair", Repair, ExportRedacted, None),
            ],
        },
        ProfileSpec {
            profile_id: "policy-locked/protected-config",
            profile_label: "Policy-locked protected config (request approval)",
            object_class: PolicyLocked,
            co_applicable_states: vec![],
            resolution: resolution_for(PolicyLocked, &[]),
            bindings: vec![
                bs("caw-policy-directsave", DirectEditSave, EnforcedGate, None),
                bs(
                    "caw-policy-repair",
                    Repair,
                    FailClosedOnActorDrift,
                    Some(ActorContextDrifted),
                ),
            ],
        },
        ProfileSpec {
            profile_id: "managed/captured-snapshot-mirror",
            profile_label:
                "Managed mirror that is also a captured snapshot (detach from managed source)",
            object_class: Managed,
            co_applicable_states: vec![CapturedSnapshot],
            resolution: resolution_for(Managed, &["captured_snapshot"]),
            bindings: vec![
                bs("caw-managed-aiapply", AiApply, EnforcedGate, None),
                bs("caw-managed-importer", Importer, EnforcedGate, None),
                bs("caw-managed-repair", Repair, EnforcedGate, None),
                bs(
                    "caw-managed-directsave",
                    DirectEditSave,
                    ExportRedacted,
                    None,
                ),
            ],
        },
        ProfileSpec {
            profile_id: "projection/virtual-view",
            profile_label: "Projection / virtual view (create overlay patch)",
            object_class: Projection,
            co_applicable_states: vec![],
            resolution: resolution_for(Projection, &[]),
            bindings: vec![
                bs("caw-projection-codeaction", CodeAction, EnforcedGate, None),
                bs(
                    "caw-projection-automation",
                    AutomationRecipe,
                    ExportRedacted,
                    None,
                ),
            ],
        },
        ProfileSpec {
            profile_id: "captured-snapshot/preserved-state",
            profile_label:
                "Captured snapshot of a preserved past state (duplicate into an editable copy)",
            object_class: CapturedSnapshot,
            co_applicable_states: vec![],
            resolution: resolution_for(CapturedSnapshot, &[]),
            bindings: vec![
                bs(
                    "caw-snapshot-directsave",
                    DirectEditSave,
                    EnforcedGate,
                    None,
                ),
                bs(
                    "caw-snapshot-aiapply",
                    AiApply,
                    FailClosedOnActorDrift,
                    Some(ActorContextDrifted),
                ),
            ],
        },
    ]
}

/// Builds all gate bindings, applying `posture_override` to override a binding's posture.
fn build_bindings<F>(posture_override: F) -> Vec<CrossActorGateBinding>
where
    F: Fn(&str, GateEnforcementPosture) -> GateEnforcementPosture,
{
    let mut bindings = Vec::new();
    for profile in profile_specs() {
        for spec in &profile.bindings {
            let posture = posture_override(spec.binding_id, spec.posture);
            bindings.push(make_binding(
                spec.binding_id,
                profile.profile_id,
                profile.profile_label,
                profile.object_class,
                profile.co_applicable_states.clone(),
                spec.actor,
                posture,
                spec.fail_closed_reason,
                profile.resolution.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> M5CrossActorConstrainedWriteEnforcementTrustReview {
    M5CrossActorConstrainedWriteEnforcementTrustReview {
        gate_reuse_proven_by_fixtures: true,
        same_object_same_blocked_reason_across_actors: true,
        blocked_reason_keyed_to_state_class_never_actor_free_form: true,
        no_bypass_actor_silently_writes_generated_managed_projection_or_archived: true,
        no_hidden_bypass_for_ai_automation_import_repair: true,
        exact_write_target_and_canonical_source_always_stated: true,
        safe_next_step_offered_before_any_write: true,
        gate_fails_closed_on_actor_context_drift: true,
        support_trace_preserves_actor_reason_and_fallback: true,
        multi_state_objects_keep_every_state_visible: true,
        accessibility_routes_present_for_reason_target_and_safe_next_step: true,
        narrowing_disclosed_across_postures: true,
        export_views_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn actor_projection() -> M5CrossActorConstrainedWriteEnforcementActorProjection {
    M5CrossActorConstrainedWriteEnforcementActorProjection {
        direct_edit_save_routed_through_gate: true,
        ai_apply_routed_through_gate: true,
        automation_recipe_routed_through_gate: true,
        importer_routed_through_gate: true,
        repair_routed_through_gate: true,
        code_action_routed_through_gate: true,
        at_least_one_object_hit_by_ai_repair_importer_and_direct_save: true,
        blocked_reason_identical_for_same_object: true,
        blocked_reason_keyed_to_state_class_not_actor: true,
        no_bypass_actor_silently_writes_constrained_object: true,
        gate_fails_closed_on_actor_context_drift: true,
        gate_fails_closed_when_write_target_not_truthfully_explainable: true,
        trace_preserves_actor_reason_and_fallback: true,
        multi_state_objects_keep_both_facts_visible: true,
        narrowing_disclosed_not_hidden: true,
        export_maps_back_to_one_constrained_file_state_object: true,
    }
}

fn proof_freshness() -> M5CrossActorConstrainedWriteEnforcementProofFreshness {
    M5CrossActorConstrainedWriteEnforcementProofFreshness {
        proof_freshness_slo_hours: M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_SCHEMA_REF.to_owned(),
        M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_DOC_REF.to_owned(),
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
    gate_bindings: Vec<CrossActorGateBinding>,
) -> M5CrossActorConstrainedWriteEnforcementPacket {
    M5CrossActorConstrainedWriteEnforcementPacket::new(
        M5CrossActorConstrainedWriteEnforcementPacketInput {
            packet_id: packet_id.to_owned(),
            surface_label: surface_label.to_owned(),
            gate_bindings,
            downgrade_triggers: M5CrossActorConstrainedWriteEnforcementDowngradeTrigger::ALL
                .to_vec(),
            actors: MutationActor::ALL.to_vec(),
            trust_review: trust_review(),
            actor_projection: actor_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// The canonical, checked-in cross-actor constrained-write enforcement packet.
pub fn seeded_m5_cross_actor_constrained_write_enforcement(
) -> M5CrossActorConstrainedWriteEnforcementPacket {
    packet_from_bindings(
        M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_PACKET_ID,
        "M5 cross-actor constrained-write enforcement (one gate across actors)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: the same profiles with two more enforced gates narrowed to a fail-closed rendering, proving the
/// state-class reason survives when the gate fails closed on actor-context drift.
pub fn seeded_m5_cross_actor_constrained_write_enforcement_fail_closed_narrowed(
) -> M5CrossActorConstrainedWriteEnforcementPacket {
    packet_from_bindings(
        "m5-cross-actor-constrained-write-enforcement:fail-closed:0001",
        "M5 cross-actor constrained-write enforcement (fail-closed narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "caw-projection-codeaction" => GateEnforcementPosture::FailClosedOnActorDrift,
            "caw-snapshot-directsave" => GateEnforcementPosture::FailClosedOnActorDrift,
            _ => default,
        }),
    )
}

/// Fixture: the same profiles with two more enforced gates narrowed to export-safe redaction, proving the
/// state-class reason survives into exported forms.
pub fn seeded_m5_cross_actor_constrained_write_enforcement_export_redacted_narrowed(
) -> M5CrossActorConstrainedWriteEnforcementPacket {
    packet_from_bindings(
        "m5-cross-actor-constrained-write-enforcement:export-redacted:0001",
        "M5 cross-actor constrained-write enforcement (export redacted narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "caw-readonly-directsave" => GateEnforcementPosture::ExportRedacted,
            "caw-policy-directsave" => GateEnforcementPosture::ExportRedacted,
            _ => default,
        }),
    )
}
