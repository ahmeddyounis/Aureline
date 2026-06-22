//! Unit tests for the action-plan builder, the controlled item vocabulary, the
//! local-versus-external separation, approval/policy preservation, due/expiry
//! visibility, scope/boundary truth, progress, and handoff parity.

use super::*;

#[test]
fn set_validates_and_all_invariants_hold() {
    let set = action_plan_set();
    set.validate().expect("canonical action-plan set validates");
    assert!(set.all_invariants_hold());
    assert!(!set.invariants.is_empty());
}

#[test]
fn set_is_deterministic() {
    assert_eq!(action_plan_set(), action_plan_set());
}

#[test]
fn set_is_support_export_safe() {
    let set = action_plan_set();
    assert!(set.raw_payload_excluded);
    assert!(set.is_support_export_safe());
}

#[test]
fn every_plan_is_present_once_and_binds_the_matrix() {
    let set = action_plan_set();
    let matrix = crate::m5_operator_surfaces::operator_surface_matrix();
    assert_eq!(set.plans.len(), PlanClass::ALL.len());
    for class in PlanClass::ALL {
        let plan = set.plan(class).expect("plan present");
        assert_eq!(plan.plan_id, class.plan_id());
        assert_eq!(plan.surface, OperatorSurfaceClass::ActionPlan);
        assert_eq!(plan.surface_id, plan.surface.surface_id());
        assert!(matrix.surface(plan.surface).is_some());
        assert!(!plan.items.is_empty());
    }
}

#[test]
fn plans_address_canonical_subjects() {
    let set = action_plan_set();
    for plan in &set.plans {
        assert!(
            plan.subject_object_ref.starts_with("aureline://"),
            "{} must address a canonical subject",
            plan.plan.as_str()
        );
    }
}

#[test]
fn items_are_a_contiguous_order() {
    let set = action_plan_set();
    for plan in &set.plans {
        for (idx, item) in plan.items.iter().enumerate() {
            assert_eq!(
                item.ordinal,
                (idx as u32) + 1,
                "{} items must be ordered 1..n",
                plan.plan.as_str()
            );
        }
    }
}

#[test]
fn every_item_class_is_distinct_and_present() {
    let set = action_plan_set();
    let items: Vec<&PlanItem> = set.plans.iter().flat_map(|p| p.items.iter()).collect();
    for class in PlanItemClass::ALL {
        assert!(
            items.iter().any(|i| i.item_class == class),
            "item class {} must appear so the closed vocabulary is proven",
            class.as_str()
        );
    }
}

#[test]
fn local_checkoff_never_resolves_an_external_object() {
    let set = action_plan_set();
    let items: Vec<&PlanItem> = set.plans.iter().flat_map(|p| p.items.iter()).collect();

    // resolves_external_object is computed only from the mutation state.
    for item in &items {
        assert_eq!(
            item.resolves_external_object,
            compute_resolves_external(item.external_mutation_state),
            "{} resolves flag must be the computed value",
            item.item_id
        );
        if item.resolves_external_object {
            assert_eq!(
                item.external_mutation_state,
                ExternalMutationState::ExecutedConfirmed,
                "{} can only resolve when executed and confirmed",
                item.item_id
            );
        }
    }

    // At least one item is checked off locally while its external object is NOT
    // resolved — the lived proof that the two states are distinct.
    assert!(
        items.iter().any(|i| {
            i.local_state == ItemLocalState::DoneLocal
                && i.external_link.is_external()
                && !i.resolves_external_object
        }),
        "a locally-done item must leave its external object unresolved"
    );
}

#[test]
fn external_linkage_is_explicit_and_consistent() {
    let set = action_plan_set();
    for plan in &set.plans {
        for item in &plan.items {
            if item.external_link.is_external() {
                assert!(item.external_object_ref.starts_with("aureline://"));
                assert_ne!(
                    item.external_mutation_state,
                    ExternalMutationState::NotApplicable
                );
                assert!(!item.mutation_note.is_empty());
            } else {
                assert!(item.external_object_ref.is_empty());
                assert_eq!(
                    item.external_mutation_state,
                    ExternalMutationState::NotApplicable
                );
                assert!(item.mutation_note.is_empty());
            }
        }
    }
}

#[test]
fn confirmed_mutations_held_authority() {
    let set = action_plan_set();
    for plan in &set.plans {
        for item in &plan.items {
            if item.external_mutation_state == ExternalMutationState::ExecutedConfirmed {
                assert!(
                    item.approval_state.is_authorized(),
                    "{} reached a confirmed mutation without authority",
                    item.item_id
                );
            }
        }
    }
    // The confirmed-execution path is actually exercised somewhere.
    assert!(set
        .plans
        .iter()
        .flat_map(|p| p.items.iter())
        .any(|i| i.external_mutation_state == ExternalMutationState::ExecutedConfirmed));
}

#[test]
fn non_authorized_approvals_and_blocked_items_state_a_reason() {
    let set = action_plan_set();
    for plan in &set.plans {
        for item in &plan.items {
            if item.approval_state.requires_reason() {
                assert!(!item.approval_reason.is_empty(), "{}", item.item_id);
            }
            if item.local_state.requires_note() {
                assert!(!item.local_note.is_empty(), "{}", item.item_id);
            }
        }
    }
}

#[test]
fn due_and_expiry_are_consistent_and_visible() {
    let set = action_plan_set();
    for plan in &set.plans {
        for item in &plan.items {
            if item.time_state.forbids_deadline() {
                assert!(item.due.is_empty() && item.expiry.is_empty());
            }
            if item.time_state.requires_due() {
                assert!(!item.due.is_empty());
            }
            if item.time_state.requires_expiry() {
                assert!(!item.expiry.is_empty());
            }
            if item.time_state.requires_reason() {
                assert!(!item.time_reason.is_empty());
            }
        }
    }
}

#[test]
fn verification_steps_link_evidence() {
    let set = action_plan_set();
    for plan in &set.plans {
        for item in &plan.items {
            for ev in &item.linked_evidence {
                assert!(ev.starts_with("aureline://"));
            }
            if item.item_class.requires_evidence() {
                assert!(
                    !item.linked_evidence.is_empty(),
                    "{} is a verify step with no evidence",
                    item.item_id
                );
            }
        }
    }
}

#[test]
fn every_share_posture_is_proven() {
    let set = action_plan_set();
    for posture in SharePosture::ALL {
        assert!(
            set.plans.iter().any(|p| p.share_posture == posture),
            "share posture {} must be proven",
            posture.as_str()
        );
    }
}

#[test]
fn export_gate_states_boundary_truth() {
    let set = action_plan_set();
    for plan in &set.plans {
        let gate = &plan.export_gate;
        assert_eq!(gate.scope, plan.scope);
        assert_eq!(gate.share_posture, plan.share_posture);
        assert_eq!(gate.scope, plan.share_posture.scope());
        assert_eq!(
            gate.requires_boundary_ack,
            plan.share_posture.requires_boundary_ack()
        );
        assert_eq!(gate.redaction_class, plan.default_redaction);
        assert!(!gate.crosses_on_share.is_empty());
        assert!(gate.raw_payload_excluded);
    }
}

#[test]
fn progress_reports_local_and_external_separately() {
    let set = action_plan_set();
    for plan in &set.plans {
        let recomputed = compute_progress(&plan.items);
        assert_eq!(plan.progress, recomputed);
        let resolved = plan
            .items
            .iter()
            .filter(|i| i.resolves_external_object)
            .count() as u32;
        let done = plan
            .items
            .iter()
            .filter(|i| i.local_state == ItemLocalState::DoneLocal)
            .count() as u32;
        assert_eq!(plan.progress.externally_resolved, resolved);
        assert_eq!(plan.progress.done_local, done);
    }
    // At least one plan has more local check-offs than confirmed external
    // resolutions, proving the counts never silently merge.
    assert!(set
        .plans
        .iter()
        .any(|p| p.progress.done_local > p.progress.externally_resolved));
}

#[test]
fn handoff_is_a_snapshot_that_preserves_truth() {
    let set = action_plan_set();
    for plan in &set.plans {
        let exported = export_plan(plan);
        assert_eq!(exported, plan.handoff);
        assert_eq!(
            plan.handoff.live_vs_snapshot,
            LiveSnapshotClass::SnapshotOnly
        );
        assert_eq!(plan.handoff.items, plan.items);
        assert_eq!(plan.handoff.progress, compute_progress(&plan.items));
        assert_eq!(
            plan.handoff.crosses_on_share,
            plan.export_gate.crosses_on_share
        );
    }
}

#[test]
fn mark_done_local_action_is_local_safe_and_preview_is_separate() {
    let set = action_plan_set();
    for plan in &set.plans {
        let mark = plan
            .actions
            .iter()
            .find(|a| a.action == PlanActionClass::MarkItemDoneLocal)
            .expect("mark-done-local offered");
        assert!(mark.local_safe);
        assert!(plan
            .actions
            .iter()
            .any(|a| a.action == PlanActionClass::PreviewMutation));
        for action in &plan.actions {
            assert_eq!(action.local_safe, action.action.local_safe());
        }
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let set = action_plan_set();
    let lines = action_plan_lines(&set);
    assert!(lines.iter().any(|l| l.contains("Operator action plans")));
    for plan in PlanClass::ALL {
        assert!(
            lines.iter().any(|l| l.contains(plan.as_str())),
            "projection must mention plan {}",
            plan.as_str()
        );
    }
    // The progress headline never conflates local and external resolution.
    assert!(lines
        .iter()
        .any(|l| l.contains("never resolves a provider-owned object")));
}
