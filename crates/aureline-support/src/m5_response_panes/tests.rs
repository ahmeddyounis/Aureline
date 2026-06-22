//! Unit tests for the response-pane builder: the computed strip state, the
//! runbook-step admission rule, the mutating-step preview/approval gate, and the
//! local-outage continuity truth.

use super::*;

#[test]
fn set_validates_and_all_invariants_hold() {
    let set = response_pane_set();
    set.validate()
        .expect("canonical response-pane set validates");
    assert!(set.all_invariants_hold());
    assert!(!set.invariants.is_empty());
}

#[test]
fn set_is_deterministic() {
    assert_eq!(response_pane_set(), response_pane_set());
}

#[test]
fn set_is_support_export_safe() {
    let set = response_pane_set();
    assert!(set.raw_payload_excluded);
    assert!(set.is_support_export_safe());
}

#[test]
fn every_surface_binds_a_canonical_matrix_surface() {
    let set = response_pane_set();
    let matrix = crate::m5_operator_surfaces::operator_surface_matrix();
    for s in &set.service_strips {
        assert_eq!(s.surface, OperatorSurfaceClass::ServiceOwnershipStrip);
        assert_eq!(s.surface_id, s.surface.surface_id());
        assert!(matrix.surface(s.surface).is_some());
    }
    for p in &set.runbook_panes {
        assert_eq!(p.surface, OperatorSurfaceClass::RunbookStepCard);
        assert_eq!(p.surface_id, p.surface.surface_id());
        assert!(matrix.surface(p.surface).is_some());
    }
    for v in &set.continuity_views {
        assert_eq!(v.surface, v.kind.surface());
        assert_eq!(v.surface_id, v.surface.surface_id());
        assert!(matrix.surface(v.surface).is_some());
    }
}

#[test]
fn strips_name_owner_oncall_and_authority() {
    let set = response_pane_set();
    for s in &set.service_strips {
        assert!(!s.primary_owner.is_empty());
        assert!(!s.on_call_lane.is_empty());
        assert!(!s.decision_right.is_empty());
        assert!(!s.escalation.routes_to_ref.is_empty());
        assert!(AuthoritySourceClass::ALL.contains(&s.authority_source));
    }
}

#[test]
fn stale_or_advisory_strip_never_reads_clear() {
    let set = response_pane_set();
    for s in &set.service_strips {
        if !s.freshness.green_eligible() {
            assert_ne!(
                s.effective_state,
                OperatorStateClass::Clear,
                "{} must not read clear when stale",
                s.strip_id
            );
        }
    }
}

#[test]
fn strip_effective_state_is_computed() {
    let set = response_pane_set();
    for s in &set.service_strips {
        assert_eq!(
            s.effective_state,
            compute_effective_state(s.displayed_state, s.freshness, BlockerWaiverClass::None)
        );
    }
}

#[test]
fn mutating_step_is_never_run_locally() {
    let set = response_pane_set();
    let mut saw_mutating = false;
    for p in &set.runbook_panes {
        for st in &p.steps {
            if st.intent.is_mutating() {
                saw_mutating = true;
                assert_ne!(
                    st.execution,
                    StepExecutionClass::RunLocal,
                    "{} mutates and must not run locally without preview",
                    st.step_id
                );
                assert!(st.dry_run_available, "{} must offer a dry run", st.step_id);
                assert!(
                    !st.rollback_note.is_empty(),
                    "{} must carry a rollback note",
                    st.step_id
                );
            }
        }
    }
    assert!(saw_mutating, "fixture must exercise mutating steps");
}

#[test]
fn step_admission_matches_compute_for_every_path() {
    let set = response_pane_set();
    for p in &set.runbook_panes {
        for st in &p.steps {
            assert_eq!(
                st.execution,
                compute_step_execution(
                    st.intent,
                    st.boundary,
                    st.approval_gate,
                    st.approval_state,
                    st.boundary_state,
                    st.live_target_present,
                )
            );
        }
    }
}

#[test]
fn every_admission_path_is_exercised() {
    let set = response_pane_set();
    let observed: std::collections::BTreeSet<&str> = set
        .runbook_panes
        .iter()
        .flat_map(|p| p.steps.iter().map(|s| s.execution.as_str()))
        .collect();
    for expected in [
        StepExecutionClass::RunLocal,
        StepExecutionClass::PreviewBeforeApply,
        StepExecutionClass::BlockedAwaitingApproval,
        StepExecutionClass::BlockedByBoundary,
        StepExecutionClass::ExternalBrowserHandoff,
        StepExecutionClass::ReadOnlyImportedSnapshot,
    ] {
        assert!(
            observed.contains(expected.as_str()),
            "fixture must exercise the {} admission",
            expected.as_str()
        );
    }
}

#[test]
fn compute_step_execution_priority_order() {
    use ActionBoundaryClass as B;
    use ApprovalGateClass as G;
    use ApprovalStateClass as AS;
    use OperatorStateClass as S;
    use StepExecutionClass as E;
    use StepIntentClass as I;

    // Imported snapshot wins over everything.
    assert_eq!(
        compute_step_execution(
            I::Mitigate,
            B::ManagedControlPlane,
            G::None,
            AS::NotRequired,
            S::Clear,
            false
        ),
        E::ReadOnlyImportedSnapshot
    );
    // Browser handoff next.
    assert_eq!(
        compute_step_execution(
            I::Communicate,
            B::BrowserHandoff,
            G::None,
            AS::NotRequired,
            S::Clear,
            true
        ),
        E::ExternalBrowserHandoff
    );
    // Read-only intents run locally.
    assert_eq!(
        compute_step_execution(
            I::Observe,
            B::ManagedControlPlane,
            G::None,
            AS::NotRequired,
            S::Clear,
            true
        ),
        E::RunLocal
    );
    // Mutating + blocking boundary blocks before approval is even checked.
    assert_eq!(
        compute_step_execution(
            I::Mitigate,
            B::ManagedControlPlane,
            G::SingleApproval,
            AS::Granted,
            S::FailoverInProgress,
            true
        ),
        E::BlockedByBoundary
    );
    // Mutating + approval pending blocks awaiting approval.
    assert_eq!(
        compute_step_execution(
            I::Mitigate,
            B::ManagedControlPlane,
            G::SingleApproval,
            AS::Pending,
            S::Clear,
            true
        ),
        E::BlockedAwaitingApproval
    );
    // Mutating + approved + clear boundary previews before apply.
    assert_eq!(
        compute_step_execution(
            I::Mitigate,
            B::ManagedControlPlane,
            G::SingleApproval,
            AS::Granted,
            S::Clear,
            true
        ),
        E::PreviewBeforeApply
    );
    // A local mutation with no gate still previews before apply (never run_local).
    assert_eq!(
        compute_step_execution(
            I::Rollback,
            B::LocalOnly,
            G::None,
            AS::NotRequired,
            S::Clear,
            true
        ),
        E::PreviewBeforeApply
    );
}

#[test]
fn continuity_views_keep_local_work_and_publish_later() {
    let set = response_pane_set();
    for v in &set.continuity_views {
        assert!(
            !v.local_capabilities.is_empty(),
            "{} must list local capabilities",
            v.view_id
        );
        if v.blocks_managed_writes() {
            assert!(
                v.publish_later_capture,
                "{} blocks managed writes and must offer publish-later",
                v.view_id
            );
        }
    }
}

#[test]
fn continuity_with_failed_boundary_never_total_outage() {
    let set = response_pane_set();
    let mut saw_failed = false;
    for v in &set.continuity_views {
        if v.failed_boundary.failed() {
            saw_failed = true;
            assert!(
                !v.local_capabilities.is_empty(),
                "{} failed a boundary but must still keep local work",
                v.view_id
            );
        }
    }
    assert!(saw_failed, "fixture must exercise a failed boundary");
}

#[test]
fn ids_are_unique() {
    let set = response_pane_set();
    let mut step_ids = std::collections::BTreeSet::new();
    for p in &set.runbook_panes {
        for st in &p.steps {
            assert!(
                step_ids.insert(st.step_id.clone()),
                "duplicate step id {}",
                st.step_id
            );
        }
    }
}

#[test]
fn projection_renders_for_support() {
    let set = response_pane_set();
    let lines = response_pane_lines(&set);
    assert!(lines.iter().any(|l| l.contains("Operator response panes")));
    assert!(lines
        .iter()
        .any(|l| l.contains("Service ownership / on-call strips")));
    assert!(lines
        .iter()
        .any(|l| l.contains("Runbook-guided response panes")));
    assert!(lines
        .iter()
        .any(|l| l.contains("Local-outage continuity views")));
}

#[test]
fn round_trips_through_json() {
    let set = response_pane_set();
    let json = serde_json::to_string(&set).expect("serialize");
    let back: ResponsePaneSet = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(set, back);
}
