//! Unit tests for the triage-inbox builder, the reason-for-attention contract,
//! the no-silent-green row rule, grouped saved-view application, batch-review
//! truth, and handoff parity.

use super::*;

#[test]
fn set_validates_and_all_invariants_hold() {
    let set = triage_inbox_set();
    set.validate()
        .expect("canonical triage-inbox set validates");
    assert!(set.all_invariants_hold());
    assert!(!set.invariants.is_empty());
}

#[test]
fn set_is_deterministic() {
    assert_eq!(triage_inbox_set(), triage_inbox_set());
}

#[test]
fn set_is_support_export_safe() {
    let set = triage_inbox_set();
    assert!(set.raw_payload_excluded);
    assert!(set.is_support_export_safe());
}

#[test]
fn every_inbox_is_present_once() {
    let set = triage_inbox_set();
    assert_eq!(set.inboxes.len(), InboxClass::ALL.len());
    for class in InboxClass::ALL {
        let inbox = set.inbox(class).expect("inbox present");
        assert_eq!(inbox.inbox_id, class.inbox_id());
        assert!(!inbox.rows.is_empty());
        assert!(!inbox.saved_views.is_empty());
    }
}

#[test]
fn inboxes_bind_the_triage_matrix_surface() {
    let set = triage_inbox_set();
    let matrix = crate::m5_operator_surfaces::operator_surface_matrix();
    for inbox in &set.inboxes {
        assert_eq!(inbox.surface, OperatorSurfaceClass::TriageInbox);
        assert_eq!(inbox.surface_id, inbox.surface.surface_id());
        assert!(
            matrix.surface(inbox.surface).is_some(),
            "inbox {} must bind a matrix surface",
            inbox.inbox.as_str()
        );
    }
}

#[test]
fn rows_point_at_canonical_objects_not_queue_ids() {
    let set = triage_inbox_set();
    for inbox in &set.inboxes {
        for row in &inbox.rows {
            assert!(
                row.object_ref.starts_with("aureline://"),
                "{} must carry a canonical object handle",
                row.row_id
            );
            assert_eq!(
                row.open_detail_ref, row.object_ref,
                "{} open-detail must route to its canonical object",
                row.row_id
            );
        }
    }
}

#[test]
fn every_attention_class_is_distinct_and_present() {
    let set = triage_inbox_set();
    let rows: Vec<&TriageRow> = set.inboxes.iter().flat_map(|i| i.rows.iter()).collect();
    for class in AttentionClass::ALL {
        assert!(
            rows.iter().any(|r| r.attention_class == class),
            "attention class {} must appear so it never collapses into one badge",
            class.as_str()
        );
    }
}

#[test]
fn every_row_explains_its_reason_for_attention() {
    let set = triage_inbox_set();
    for inbox in &set.inboxes {
        for row in &inbox.rows {
            assert!(
                !row.reason_for_attention.is_empty(),
                "{} must carry a written reason-for-attention",
                row.row_id
            );
        }
    }
}

#[test]
fn priority_and_sla_are_first_class() {
    let set = triage_inbox_set();
    for inbox in &set.inboxes {
        for row in &inbox.rows {
            if row.sla_state.requires_reason() {
                assert!(
                    !row.sla_reason.is_empty(),
                    "{} at-risk/breached SLA must carry a reason",
                    row.row_id
                );
            }
        }
    }
    // Every SLA state is proven somewhere in the corpus.
    let rows: Vec<&TriageRow> = set.inboxes.iter().flat_map(|i| i.rows.iter()).collect();
    for sla in SlaState::ALL {
        assert!(
            rows.iter().any(|r| r.sla_state == sla),
            "SLA state {} must appear",
            sla.as_str()
        );
    }
}

#[test]
fn source_and_provider_truth_is_preserved() {
    let set = triage_inbox_set();
    for inbox in &set.inboxes {
        for row in &inbox.rows {
            assert!(
                !row.provider.is_empty(),
                "{} must name a provider",
                row.row_id
            );
            if row.source.requires_external_provider() {
                assert_ne!(
                    row.provider, INTERNAL_PROVIDER,
                    "{} is provider-raised and must name an external provider",
                    row.row_id
                );
            }
        }
    }
}

#[test]
fn local_shared_deferred_state_is_consistent() {
    let set = triage_inbox_set();
    // Each sync state appears at least once.
    let rows: Vec<&TriageRow> = set.inboxes.iter().flat_map(|i| i.rows.iter()).collect();
    for sync in SyncStateClass::ALL {
        assert!(
            rows.iter().any(|r| r.sync_state == sync),
            "sync state {} must appear",
            sync.as_str()
        );
    }
    for row in rows {
        assert_eq!(row.batch_reviewable, row.sync_state.batch_reviewable());
        // batch_reviewable == true <=> no exclusion reason.
        assert_eq!(row.batch_reviewable, row.batch_excluded_reason.is_empty());
        if row.sync_state == SyncStateClass::ImportedSnapshot {
            assert_eq!(row.boundary, OperatorPathClass::ImportedSnapshot);
            assert!(!row.batch_reviewable);
        }
    }
}

#[test]
fn no_silent_green_is_computed() {
    // A stale clear row downgrades to unconfirmed.
    assert_eq!(
        compute_effective_state(
            OperatorStateClass::Clear,
            FreshnessClass::Stale,
            BlockerWaiverClass::None
        ),
        OperatorStateClass::Unconfirmed
    );
    // A waived row is never green.
    assert_eq!(
        compute_effective_state(
            OperatorStateClass::Clear,
            FreshnessClass::Fresh,
            BlockerWaiverClass::Waived
        ),
        OperatorStateClass::Attention
    );
    // Every row's effective state matches the computed rule.
    let set = triage_inbox_set();
    for inbox in &set.inboxes {
        for row in &inbox.rows {
            assert_eq!(
                row.effective_state,
                compute_effective_state(row.displayed_state, row.freshness, row.blocker_waiver),
                "{} effective state must be the computed no-silent-green state",
                row.row_id
            );
        }
    }
}

#[test]
fn at_least_one_row_proves_each_downgrade_path() {
    let set = triage_inbox_set();
    let rows: Vec<&TriageRow> = set.inboxes.iter().flat_map(|i| i.rows.iter()).collect();
    // A stale would-be-green row downgraded to unconfirmed.
    assert!(rows.iter().any(|r| {
        r.displayed_state == OperatorStateClass::Clear
            && r.effective_state == OperatorStateClass::Unconfirmed
    }));
    // A waived row rendered as attention.
    assert!(rows
        .iter()
        .any(|r| r.blocker_waiver == BlockerWaiverClass::Waived
            && r.effective_state == OperatorStateClass::Attention));
    // A blocked row.
    assert!(rows
        .iter()
        .any(|r| r.blocker_waiver == BlockerWaiverClass::Blocked
            && r.effective_state == OperatorStateClass::Blocked));
}

#[test]
fn owner_and_blocker_reason_are_first_class() {
    let set = triage_inbox_set();
    for inbox in &set.inboxes {
        for row in &inbox.rows {
            assert!(!row.owner.is_empty());
            assert!(!row.decision_right.is_empty());
            if row.blocker_waiver.requires_reason() {
                assert!(
                    !row.blocker_reason.is_empty(),
                    "{} must show a visible blocker/waiver reason",
                    row.row_id
                );
            }
        }
    }
}

#[test]
fn saved_views_name_grouping_and_order() {
    let set = triage_inbox_set();
    for inbox in &set.inboxes {
        assert!(
            inbox
                .saved_views
                .iter()
                .any(|v| v.token == inbox.default_view),
            "{} default view must be a saved view",
            inbox.inbox.as_str()
        );
        for view in &inbox.saved_views {
            assert!(
                !view.group_by.reason.is_empty(),
                "{} grouping must be named",
                view.view_id
            );
            assert!(
                !view.order.reason.is_empty(),
                "{} order must be named",
                view.view_id
            );
        }
    }
}

#[test]
fn saved_view_filters_use_the_shared_vocabulary() {
    let set = triage_inbox_set();
    for inbox in &set.inboxes {
        for view in &inbox.saved_views {
            for clause in &view.filters {
                let facet = set.facet(clause.facet).expect("facet defined");
                if facet.closed_vocabulary {
                    for value in &clause.include_tokens {
                        assert!(
                            facet.allowed_tokens.contains(value),
                            "value {value} must be in facet {}",
                            clause.facet.as_str()
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn handoff_parity_holds_for_every_inbox_default_view() {
    let set = triage_inbox_set();
    for inbox in &set.inboxes {
        let recomputed = export_triage_view(inbox, &inbox.default_view).expect("default exports");
        assert_eq!(
            recomputed,
            inbox.handoff,
            "{} frozen handoff must equal re-applying its default view",
            inbox.inbox.as_str()
        );
    }
}

#[test]
fn batch_review_parity_holds_and_preserves_identity() {
    let set = triage_inbox_set();
    for inbox in &set.inboxes {
        let recomputed = batch_review_view(inbox, &inbox.default_view).expect("default previews");
        assert_eq!(recomputed, inbox.batch_review);
        assert!(inbox.batch_review.preserves_object_identity);
        for c in &inbox.batch_review.candidates {
            assert!(
                inbox
                    .rows
                    .iter()
                    .any(|r| r.row_id == c.row_id && r.object_ref == c.object_ref),
                "candidate {} must resolve to a row's exact object",
                c.row_id
            );
        }
        for e in &inbox.batch_review.excluded {
            assert!(
                !e.reason.is_empty(),
                "{} exclusion must state a reason",
                e.row_id
            );
        }
    }
}

#[test]
fn imported_snapshot_is_excluded_from_batch_review() {
    let set = triage_inbox_set();
    let incident = set.inbox(InboxClass::IncidentTriage).expect("inbox");
    // The imported replay row is excluded from the batch action with a reason.
    let imported = incident
        .rows
        .iter()
        .find(|r| r.sync_state == SyncStateClass::ImportedSnapshot)
        .expect("an imported row exists");
    assert!(!imported.batch_reviewable);
    assert!(incident
        .batch_review
        .excluded
        .iter()
        .any(|e| e.object_ref == imported.object_ref && !e.reason.is_empty()));
    assert!(incident
        .batch_review
        .candidates
        .iter()
        .all(|c| c.object_ref != imported.object_ref));
}

#[test]
fn applying_a_filter_view_narrows_and_preserves_truth() {
    let set = triage_inbox_set();
    let support = set.inbox(InboxClass::SupportTriage).expect("inbox");
    let handoff = export_triage_view(support, "breaching_only").expect("view exports");
    assert!(handoff.row_count >= 1);
    // Only breached / at-risk cases survive.
    assert!(handoff
        .rows
        .iter()
        .all(|r| matches!(r.sla_state, SlaState::Breached | SlaState::AtRisk)));
    // The handoff preserves source/provider/sla and the blocker reason verbatim.
    for er in &handoff.rows {
        let row = support
            .rows
            .iter()
            .find(|r| r.row_id == er.row_id)
            .expect("export row maps to a row");
        assert_eq!(er.source, row.source);
        assert_eq!(er.provider, row.provider);
        assert_eq!(er.sla_reason, row.sla_reason);
        assert_eq!(er.blocker_reason, row.blocker_reason);
        assert_eq!(er.open_detail_ref, row.object_ref);
    }
}

#[test]
fn grouping_orders_deterministically_by_group_then_order() {
    let set = triage_inbox_set();
    let incident = set.inbox(InboxClass::IncidentTriage).expect("inbox");
    // The default view groups by severity, so the blocked row sorts above clear.
    let handoff = &incident.handoff;
    let blocked_pos = handoff
        .rows
        .iter()
        .position(|r| r.effective_state == OperatorStateClass::Blocked);
    let imported_pos = handoff
        .rows
        .iter()
        .position(|r| r.effective_state == OperatorStateClass::ImportedSnapshotNoLive);
    if let (Some(b), Some(im)) = (blocked_pos, imported_pos) {
        assert!(
            b < im,
            "blocked must sort above the low-severity imported row"
        );
    }
}

#[test]
fn export_and_batch_return_none_for_unknown_view() {
    let set = triage_inbox_set();
    let inbox = set.inbox(InboxClass::AdminTriage).expect("inbox");
    assert!(export_triage_view(inbox, "no_such_view").is_none());
    assert!(batch_review_view(inbox, "no_such_view").is_none());
}

#[test]
fn validate_rejects_a_raw_payload_flag_flip() {
    let mut set = triage_inbox_set();
    set.raw_payload_excluded = false;
    assert!(set.validate().is_err());
    assert!(!set.is_support_export_safe());
}

#[test]
fn validate_rejects_an_unsafe_object_ref() {
    let mut set = triage_inbox_set();
    set.inboxes[0].rows[0].object_ref = "https://internal.example.com/incident".to_owned();
    assert!(!set.is_support_export_safe());
    assert!(set.validate().is_err());
}

#[test]
fn validate_rejects_a_silent_green_row() {
    let mut set = triage_inbox_set();
    let row = &mut set.inboxes[0].rows[1];
    row.freshness = FreshnessClass::VeryStale;
    row.displayed_state = OperatorStateClass::Clear;
    row.effective_state = OperatorStateClass::Clear;
    assert!(set.validate().is_err());
}

#[test]
fn validate_rejects_a_collapsed_reason_for_attention() {
    let mut set = triage_inbox_set();
    set.inboxes[0].rows[0].reason_for_attention = String::new();
    assert!(set.validate().is_err());
}

#[test]
fn validate_rejects_a_provider_raised_row_without_a_provider() {
    let mut set = triage_inbox_set();
    // case-8802 is provider-raised; blanking the provider to the internal sentinel
    // must fail.
    for inbox in &mut set.inboxes {
        for row in &mut inbox.rows {
            if row.source.requires_external_provider() {
                row.provider = INTERNAL_PROVIDER.to_owned();
            }
        }
    }
    assert!(set.validate().is_err());
}

#[test]
fn human_readable_projection_renders() {
    let set = triage_inbox_set();
    let lines = triage_inbox_lines(&set);
    assert!(lines.iter().any(|l| l.contains("Operator triage inboxes")));
    for class in InboxClass::ALL {
        assert!(
            lines.iter().any(|l| l.contains(class.as_str())),
            "projection must mention inbox {}",
            class.as_str()
        );
    }
}
