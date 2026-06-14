use super::*;

fn corpus() -> StorageInspectorCorpus {
    current_storage_inspector_corpus().expect("corpus parses")
}

#[test]
fn corpus_parses_every_fixture() {
    let corpus = corpus();
    assert_eq!(corpus.cards.len(), CARD_FIXTURES.len());
    assert_eq!(corpus.breakdown_rows.len(), BREAKDOWN_FIXTURES.len());
    assert_eq!(corpus.detail_rows.len(), DETAIL_FIXTURES.len());
}

#[test]
fn corpus_validates_against_the_safety_contract() {
    let corpus = corpus();
    let violations = corpus.validate();
    assert_eq!(violations, Vec::new(), "{violations:#?}");
}

#[test]
fn every_breakdown_row_resolves_to_a_loaded_card() {
    let corpus = corpus();
    for entry in &corpus.breakdown_rows {
        assert!(
            corpus.card(&entry.row.card_id_ref).is_some(),
            "{} references missing card {}",
            entry.row.row_id,
            entry.row.card_id_ref
        );
    }
}

#[test]
fn class_breakdown_distinguishes_disposable_and_authoritative() {
    let corpus = corpus();
    let rows = corpus.class_breakdown_for("card.storage.single_workspace_local_profile");
    // The single-workspace card carries both a disposable knowledge cache and an
    // authoritative user-owned recovery row, and the two are not collapsed.
    let knowledge = rows
        .iter()
        .find(|r| r.storage_class_id == StorageClassId::KnowledgeCache)
        .expect("knowledge row present");
    let recovery = rows
        .iter()
        .find(|r| r.storage_class_id == StorageClassId::UserOwnedRecoveryState)
        .expect("recovery row present");
    assert_eq!(
        knowledge.authority_class,
        AuthorityClass::DisposableDerivedCache
    );
    assert!(knowledge.reclaimable_bytes_estimate > 0);
    assert_eq!(
        recovery.authority_class,
        AuthorityClass::UserOwnedRecoveryState
    );
    assert_eq!(
        recovery.rebuild_cost_class,
        RebuildCostClass::AuthoritativeNoRebuild
    );
    assert_eq!(recovery.reclaimable_bytes_estimate, 0);
}

#[test]
fn top_consumers_sort_by_bytes_descending() {
    let corpus = corpus();
    let card = corpus
        .card("card.storage.single_workspace_local_profile")
        .expect("card present");
    let top = card.top_consumers_by_bytes();
    let mut prev = u64::MAX;
    for consumer in &top {
        assert!(
            consumer.consumer_used_bytes <= prev,
            "must be non-increasing"
        );
        prev = consumer.consumer_used_bytes;
    }
    // The persisted authority-aware order leads with the recovery journal even
    // though it is not the largest by bytes; the by-bytes view re-sorts so the
    // search index leads.
    assert_eq!(
        card.largest_consumers.first().unwrap().consumer_class,
        ConsumerClass::WorkspaceRecoveryJournal
    );
    assert_eq!(
        top.first().unwrap().consumer_class,
        ConsumerClass::WorkspaceIndexCorpus
    );
}

#[test]
fn protected_detail_rows_never_admit_a_generic_clear() {
    let corpus = corpus();
    for entry in &corpus.detail_rows {
        let row = &entry.row;
        if !matches!(
            row.storage_class_id,
            StorageClassId::EvidenceSupportCache | StorageClassId::UserOwnedRecoveryState
        ) {
            continue;
        }
        assert!(
            !matches!(row.clear_action, ClearActionClass::ClearAdmissibleGeneric),
            "{} must not admit a generic clear",
            row.row_id
        );
        assert_ne!(
            row.clear_cache_protection_class,
            ClearCacheProtectionClass::GenericClearAlwaysAllowed,
            "{} must not allow a generic always-allowed clear",
            row.row_id
        );
        assert!(
            row.linked_class_specific_review_ref.is_some(),
            "{} must link the class-specific review",
            row.row_id
        );
    }
}

#[test]
fn broad_scope_cards_disclose_both_protected_classes() {
    let corpus = corpus();
    for entry in &corpus.cards {
        let card = &entry.card;
        if !card.inspector_scope.scope_class.is_broad() {
            continue;
        }
        assert!(
            card.protected_class_visibility
                .contains(&ProtectedClassVisibility::EvidenceSupportCacheVisible)
                && card
                    .protected_class_visibility
                    .contains(&ProtectedClassVisibility::UserOwnedRecoveryStateVisible),
            "{} must disclose both protected classes",
            card.card_id
        );
    }
}

#[test]
fn mutating_user_owned_detail_to_generic_clear_is_rejected() {
    let mut corpus = corpus();
    let entry = corpus
        .detail_rows
        .iter_mut()
        .find(|e| e.row.storage_class_id == StorageClassId::UserOwnedRecoveryState)
        .expect("user-owned detail row present");
    entry.row.clear_action = ClearActionClass::ClearAdmissibleGeneric;
    entry.row.clear_cache_protection_class = ClearCacheProtectionClass::GenericClearAlwaysAllowed;
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "detail.user_owned_clear_action"
                || v.check_id == "detail.user_owned_protection"),
        "expected a user-owned clear/protection violation, got {violations:#?}"
    );
}

#[test]
fn mutating_a_rebuild_hint_to_an_inconsistent_summary_is_rejected() {
    let mut corpus = corpus();
    // Force the knowledge-cache search-index detail (expensive_to_rebuild_but_safe)
    // to claim it is cheap to rebuild while keeping the expensive axes.
    let entry = corpus
        .detail_rows
        .iter_mut()
        .find(|e| e.row.storage_class_id == StorageClassId::KnowledgeCache)
        .expect("knowledge detail row present");
    entry.row.rebuild_cost_hint.rebuild_safety_summary_class =
        RebuildSafetySummaryClass::CheapToRebuildSafeToRemove;
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "rebuild.cheap_pairing"),
        "expected a cheap-pairing violation, got {violations:#?}"
    );
}

#[test]
fn mutating_an_evidence_row_to_disposable_authority_is_rejected() {
    let mut corpus = corpus();
    let entry = corpus
        .breakdown_rows
        .iter_mut()
        .find(|e| e.row.storage_class_id == StorageClassId::EvidenceSupportCache)
        .expect("evidence breakdown row present");
    entry.row.authority_class = AuthorityClass::DisposableDerivedCache;
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "class.evidence_authority"),
        "expected an evidence-authority violation, got {violations:#?}"
    );
}

#[test]
fn quota_ceiling_round_trips_through_yaml_and_json() {
    for (yaml, expected) in [
        ("5368709120", QuotaCeilingBytes::Bytes(5_368_709_120)),
        ("not_applicable", QuotaCeilingBytes::NotApplicable),
    ] {
        let from_yaml: QuotaCeilingBytes = serde_yaml::from_str(yaml).expect("yaml parses");
        assert_eq!(from_yaml, expected);
        let json = serde_json::to_string(&expected).expect("serialize");
        let from_json: QuotaCeilingBytes = serde_json::from_str(&json).expect("json parses");
        assert_eq!(from_json, expected);
    }
}

#[test]
fn support_export_is_metadata_safe_and_complete() {
    let corpus = corpus();
    let export = corpus.support_export(
        "support_export.m5_storage_inspector.v1",
        "2026-06-14T00:00:00Z",
    );
    assert!(export.is_export_safe());
    assert_eq!(export.card_count as usize, corpus.cards.len());
    assert_eq!(export.cards.len(), corpus.cards.len());
    assert!(!export.raw_content_exported);
    // Every protected class checked in is counted.
    assert!(export.protected_class_row_count >= 1);
    // The envelope round-trips through serde without losing fields.
    let json = serde_json::to_string(&export).expect("serialize");
    let back: StorageInspectorSupportExport = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(export, back);
}

#[test]
fn support_export_matches_checked_in_golden() {
    const GOLDEN: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/storage/m5_storage_inspector/support_export.golden.json"
    ));
    let corpus = corpus();
    let export = corpus.support_export(
        "support_export.m5_storage_inspector.v1",
        "2026-06-14T00:00:00Z",
    );
    let golden: StorageInspectorSupportExport =
        serde_json::from_str(GOLDEN).expect("golden parses");
    assert_eq!(
        export, golden,
        "projected support export drifted from the checked-in golden; \
         regenerate with `cargo run -p aureline-support \
         --example dump_m5_storage_inspector_support_export`"
    );
}
