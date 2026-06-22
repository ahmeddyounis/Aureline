//! Freeze gate for the M5 triage-inbox set.
//!
//! The checked-in fixture `fixtures/ops/m5-triage-inbox/canonical_triage.json` is
//! the published triage-inbox set. This gate rebuilds the set in code and asserts
//! it equals the fixture after a serialize round-trip, so the triage contract
//! cannot drift from the published artifact without failing CI. It also re-proves
//! support-export safety, full inbox coverage, canonical object identity, the
//! reason-for-attention contract, the no-silent-green row rule, batch-review and
//! handoff parity, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_support::m5_triage_inbox::{
    batch_review_view, export_triage_view, triage_inbox_lines, triage_inbox_set, AttentionClass,
    InboxClass, SlaState, SyncStateClass, TriageInboxSet, M5_TRIAGE_INBOX_MATRIX_REF,
    M5_TRIAGE_INBOX_RECORD_KIND, M5_TRIAGE_INBOX_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/ops/m5-triage-inbox/canonical_triage.json")
}

fn load_fixture() -> TriageInboxSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_set_matches_checked_in_fixture() {
    let built = triage_inbox_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code triage-inbox set drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-support --example dump_m5_triage_inbox`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_TRIAGE_INBOX_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_TRIAGE_INBOX_SCHEMA_REF);
    assert_eq!(fixture.matrix_ref, M5_TRIAGE_INBOX_MATRIX_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: TriageInboxSet =
        serde_json::from_str(&serde_json::to_string(&fixture).expect("serializes"))
            .expect("round-trips");
    assert_eq!(roundtrip, fixture);
}

#[test]
fn every_frozen_invariant_holds() {
    let fixture = load_fixture();
    assert!(!fixture.invariants.is_empty());
    for invariant in &fixture.invariants {
        assert!(
            invariant.holds,
            "frozen invariant must hold: {}",
            invariant.invariant_id
        );
    }
    assert!(fixture.all_invariants_hold());
}

#[test]
fn set_covers_every_inbox() {
    let fixture = load_fixture();
    assert_eq!(fixture.inboxes.len(), InboxClass::ALL.len());
    for inbox in InboxClass::ALL {
        let entry = fixture.inbox(inbox).expect("inbox present");
        assert!(!entry.rows.is_empty());
        assert!(!entry.saved_views.is_empty());
        assert_eq!(entry.surface_id, inbox.surface().surface_id());
    }
}

#[test]
fn fixture_rows_explain_attention_and_carry_canonical_objects() {
    let fixture = load_fixture();
    for inbox in &fixture.inboxes {
        for row in &inbox.rows {
            assert!(row.object_ref.starts_with("aureline://"));
            assert_eq!(row.open_detail_ref, row.object_ref);
            assert!(!row.owner.is_empty());
            assert!(!row.decision_right.is_empty());
            assert!(!row.reason_for_attention.is_empty());
            assert!(!row.provider.is_empty());
            if row.sla_state.requires_reason() {
                assert!(!row.sla_reason.is_empty());
            }
            if row.blocker_waiver.requires_reason() {
                assert!(!row.blocker_reason.is_empty());
            }
        }
    }
    // The six attention classes, five SLA states, and four sync states all appear.
    let rows: Vec<_> = fixture.inboxes.iter().flat_map(|i| i.rows.iter()).collect();
    for class in AttentionClass::ALL {
        assert!(rows.iter().any(|r| r.attention_class == class));
    }
    for sla in SlaState::ALL {
        assert!(rows.iter().any(|r| r.sla_state == sla));
    }
    for sync in SyncStateClass::ALL {
        assert!(rows.iter().any(|r| r.sync_state == sync));
    }
}

#[test]
fn fixture_handoff_and_batch_parity_hold() {
    let fixture = load_fixture();
    for inbox in &fixture.inboxes {
        let handoff = export_triage_view(inbox, &inbox.default_view).expect("default exports");
        assert_eq!(
            handoff,
            inbox.handoff,
            "{} frozen handoff must equal re-applying its default view",
            inbox.inbox.as_str()
        );
        let batch = batch_review_view(inbox, &inbox.default_view).expect("default previews");
        assert_eq!(batch, inbox.batch_review);
        assert!(inbox.batch_review.preserves_object_identity);
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = triage_inbox_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Operator triage inboxes")));
    for inbox in InboxClass::ALL {
        assert!(
            lines.iter().any(|line| line.contains(inbox.as_str())),
            "projection must mention inbox {}",
            inbox.as_str()
        );
    }
}
