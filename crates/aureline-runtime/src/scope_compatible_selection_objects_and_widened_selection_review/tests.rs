use super::*;

const PACKET_ID: &str = "portable-selection:stable:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn target(target_id: &str, node_kind: DurableTestNodeKind) -> SelectionTarget {
    SelectionTarget {
        target_id: target_id.to_owned(),
        node_kind,
        target_fingerprint_token: format!("fingerprint:{target_id}"),
        identity_class: TestItemIdentityClass::Stable,
    }
}

fn imported_target(target_id: &str) -> SelectionTarget {
    SelectionTarget {
        target_id: target_id.to_owned(),
        node_kind: DurableTestNodeKind::ConcreteCase,
        target_fingerprint_token: format!("fingerprint:{target_id}"),
        identity_class: TestItemIdentityClass::ImportedReadOnly,
    }
}

fn snapshot(snapshot_id: &str, digest: &str, consumer: &str) -> SnapshotFingerprint {
    SnapshotFingerprint {
        snapshot_id: snapshot_id.to_owned(),
        snapshot_digest: digest.to_owned(),
        consumer_token: consumer.to_owned(),
    }
}

fn ui_rerun_all() -> SelectionObject {
    SelectionObject {
        selection_id: "selection:ui:rerun".to_owned(),
        label: "UI rerun-all".to_owned(),
        origin_channel: SelectorChannel::Ui,
        intent: SelectionIntentKind::RerunAll,
        expansion_policy: ExpansionPolicy::ReresolveWithinSnapshot,
        snapshot_fingerprint: snapshot("snapshot:fw", "digest:v1", "framework_pack"),
        query: SelectionQuery {
            include_query_tokens: refs(&["suite:checkout"]),
            ..SelectionQuery::default()
        },
        pinned_targets: vec![
            target("case:add", DurableTestNodeKind::ConcreteCase),
            target(
                "template:totals",
                DurableTestNodeKind::ParameterizedTemplate,
            ),
            target("invocation:usd", DurableTestNodeKind::ConcreteInvocation),
        ],
        evidence_refs: refs(&["evidence:ui"]),
    }
}

fn cli_rerun_failed() -> SelectionObject {
    SelectionObject {
        selection_id: "selection:cli:failed".to_owned(),
        label: "CLI rerun-failed".to_owned(),
        origin_channel: SelectorChannel::Cli,
        intent: SelectionIntentKind::RerunFailed,
        expansion_policy: ExpansionPolicy::PinnedExact,
        snapshot_fingerprint: snapshot("snapshot:fw", "digest:v1", "framework_pack"),
        query: SelectionQuery::default(),
        pinned_targets: vec![target(
            "invocation:usd",
            DurableTestNodeKind::ConcreteInvocation,
        )],
        evidence_refs: refs(&["evidence:cli"]),
    }
}

fn ai_changed_since() -> SelectionObject {
    SelectionObject {
        selection_id: "selection:ai:changed".to_owned(),
        label: "AI changed-since".to_owned(),
        origin_channel: SelectorChannel::Ai,
        intent: SelectionIntentKind::ChangedSince,
        expansion_policy: ExpansionPolicy::AllowWidenWithReview,
        snapshot_fingerprint: snapshot("snapshot:tree", "digest:v3", "test_tree"),
        query: SelectionQuery {
            include_query_tokens: refs(&["path:src"]),
            changed_since_ref: Some("ref:base".to_owned()),
            ..SelectionQuery::default()
        },
        pinned_targets: vec![target("tree:login", DurableTestNodeKind::ConcreteCase)],
        evidence_refs: refs(&["evidence:ai"]),
    }
}

fn support_snapshot_scoped() -> SelectionObject {
    SelectionObject {
        selection_id: "selection:support:scoped".to_owned(),
        label: "Support snapshot-scoped".to_owned(),
        origin_channel: SelectorChannel::Support,
        intent: SelectionIntentKind::SnapshotScoped,
        expansion_policy: ExpansionPolicy::ReresolveWithinSnapshot,
        snapshot_fingerprint: snapshot("snapshot:tree", "digest:v3", "test_tree"),
        query: SelectionQuery {
            include_query_tokens: refs(&["scope:tree"]),
            ..SelectionQuery::default()
        },
        pinned_targets: vec![target("tree:login", DurableTestNodeKind::ConcreteCase)],
        evidence_refs: refs(&["evidence:support"]),
    }
}

fn imported_overlay() -> SelectionObject {
    SelectionObject {
        selection_id: "selection:imported".to_owned(),
        label: "Imported overlay".to_owned(),
        origin_channel: SelectorChannel::Support,
        intent: SelectionIntentKind::ExplicitItems,
        expansion_policy: ExpansionPolicy::FrozenImportedReadOnly,
        snapshot_fingerprint: snapshot("snapshot:ci", "digest:ci", "imported_ci"),
        query: SelectionQuery::default(),
        pinned_targets: vec![imported_target("ci:smoke")],
        evidence_refs: refs(&["evidence:imported"]),
    }
}

fn guardrails() -> SelectionGuardrails {
    SelectionGuardrails {
        templates_distinct_from_invocations: true,
        imported_never_rerun_as_local: true,
        widening_opens_review: true,
        snapshot_drift_opens_review: true,
        origin_preserved_or_reviewed: true,
        selection_reconstructable_from_export: true,
    }
}

fn channel_projection() -> SelectionChannelProjection {
    SelectionChannelProjection {
        ui_consumes_selection: true,
        cli_consumes_selection: true,
        ai_plan_consumes_selection: true,
        support_export_reconstructs_selection: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        PORTABLE_SELECTION_SCHEMA_REF,
        PORTABLE_SELECTION_DOC_REF,
        PORTABLE_SELECTION_ARTIFACT_REF,
    ])
}

fn valid_packet() -> PortableSelectionPacket {
    let cli = cli_rerun_failed();
    let ai = ai_changed_since();
    let imported = imported_overlay();

    let compatible = cli.assess_against(
        "assessment:compatible".to_owned(),
        cli.snapshot_fingerprint.clone(),
        &cli.pinned_targets,
        refs(&["evidence:assessment:compatible"]),
    );
    let widened = ai.assess_against(
        "assessment:widened".to_owned(),
        ai.snapshot_fingerprint.clone(),
        &[
            target("tree:login", DurableTestNodeKind::ConcreteCase),
            target("tree:logout", DurableTestNodeKind::ConcreteCase),
        ],
        refs(&["evidence:assessment:widened"]),
    );
    let blocked = imported.assess_against(
        "assessment:blocked".to_owned(),
        imported.snapshot_fingerprint.clone(),
        &imported.pinned_targets,
        refs(&["evidence:assessment:blocked"]),
    );

    PortableSelectionPacket::new(PortableSelectionPacketInput {
        packet_id: PACKET_ID.to_owned(),
        label: "M5 Scope-Compatible Selection Objects".to_owned(),
        selections: vec![ui_rerun_all(), cli, ai, support_snapshot_scoped(), imported],
        assessments: vec![compatible, widened, blocked],
        guardrails: guardrails(),
        channel_projection: channel_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

#[test]
fn valid_packet_has_no_violations() {
    let packet = valid_packet();
    assert!(
        packet.validate().is_empty(),
        "expected clean packet: {:?}",
        packet.validate()
    );
}

#[test]
fn all_channels_and_intents_are_represented() {
    let packet = valid_packet();
    assert_eq!(
        packet.represented_channels().len(),
        SelectorChannel::ALL.len()
    );
    assert_eq!(
        packet.represented_intents().len(),
        SelectionIntentKind::ALL.len()
    );
}

#[test]
fn compatible_selection_preserves_origin_and_dispatches() {
    let cli = cli_rerun_failed();
    let assessment = cli.assess_against(
        "assessment:compatible".to_owned(),
        cli.snapshot_fingerprint.clone(),
        &cli.pinned_targets,
        refs(&["evidence:a"]),
    );
    assert_eq!(
        assessment.compatibility_class,
        TargetCompatibilityClass::Compatible
    );
    assert_eq!(
        assessment.review_state,
        WidenedSelectionReviewState::NotRequired
    );
    assert!(assessment.preserves_origin);
    assert!(assessment.dispatch_allowed());
    assert!(assessment.is_consistent());
}

#[test]
fn widening_opens_review_and_blocks_dispatch() {
    let ai = ai_changed_since();
    let widened = ai.assess_against(
        "assessment:widened".to_owned(),
        ai.snapshot_fingerprint.clone(),
        &[
            target("tree:login", DurableTestNodeKind::ConcreteCase),
            target("tree:logout", DurableTestNodeKind::ConcreteCase),
        ],
        refs(&["evidence:a"]),
    );
    assert_eq!(
        widened.compatibility_class,
        TargetCompatibilityClass::WidenedNeedsReview
    );
    assert_eq!(widened.added_target_ids, vec!["tree:logout".to_owned()]);
    assert_eq!(widened.review_state, WidenedSelectionReviewState::Pending);
    assert!(!widened.preserves_origin);
    assert!(
        !widened.dispatch_allowed(),
        "a pending widening must not silently dispatch"
    );
}

#[test]
fn narrowing_opens_review() {
    let ui = ui_rerun_all();
    let narrowed = ui.assess_against(
        "assessment:narrowed".to_owned(),
        ui.snapshot_fingerprint.clone(),
        &[target("case:add", DurableTestNodeKind::ConcreteCase)],
        refs(&["evidence:a"]),
    );
    assert_eq!(
        narrowed.compatibility_class,
        TargetCompatibilityClass::NarrowedNeedsReview
    );
    assert!(!narrowed.removed_target_ids.is_empty());
    assert!(narrowed.review_state.is_open_or_decided());
    assert!(!narrowed.dispatch_allowed());
}

#[test]
fn snapshot_drift_forces_review_even_with_same_targets() {
    let ui = ui_rerun_all();
    let drifted = ui.assess_against(
        "assessment:drifted".to_owned(),
        snapshot("snapshot:fw", "digest:v2", "framework_pack"),
        &ui.pinned_targets,
        refs(&["evidence:a"]),
    );
    assert_eq!(
        drifted.compatibility_class,
        TargetCompatibilityClass::SnapshotDrifted
    );
    assert!(drifted.snapshot_drifted);
    assert!(!drifted.dispatch_allowed());
}

#[test]
fn target_fingerprint_mismatch_forces_review() {
    let cli = cli_rerun_failed();
    let mut changed = target("invocation:usd", DurableTestNodeKind::ConcreteInvocation);
    changed.target_fingerprint_token = "fingerprint:invocation:usd:v2".to_owned();
    let assessment = cli.assess_against(
        "assessment:mismatch".to_owned(),
        cli.snapshot_fingerprint.clone(),
        &[changed],
        refs(&["evidence:a"]),
    );
    assert_eq!(
        assessment.compatibility_class,
        TargetCompatibilityClass::TargetFingerprintMismatch
    );
    assert_eq!(
        assessment.fingerprint_mismatch_target_ids,
        vec!["invocation:usd".to_owned()]
    );
    assert!(!assessment.dispatch_allowed());
}

#[test]
fn imported_selection_is_never_a_local_rerun() {
    let imported = imported_overlay();
    let blocked = imported.assess_against(
        "assessment:blocked".to_owned(),
        imported.snapshot_fingerprint.clone(),
        &imported.pinned_targets,
        refs(&["evidence:a"]),
    );
    assert_eq!(
        blocked.compatibility_class,
        TargetCompatibilityClass::ImportedNotRerunnable
    );
    assert_eq!(blocked.review_state, WidenedSelectionReviewState::Blocked);
    assert!(!blocked.dispatch_allowed());
}

#[test]
fn keeping_original_scope_preserves_origin_and_dispatches() {
    let ai = ai_changed_since();
    let widened = ai
        .assess_against(
            "assessment:widened".to_owned(),
            ai.snapshot_fingerprint.clone(),
            &[
                target("tree:login", DurableTestNodeKind::ConcreteCase),
                target("tree:logout", DurableTestNodeKind::ConcreteCase),
            ],
            refs(&["evidence:a"]),
        )
        .with_decision(WidenedSelectionReviewState::RejectedKeepOriginal);
    assert!(widened.preserves_origin);
    assert!(widened.dispatch_allowed());
    assert!(widened.is_consistent());
}

#[test]
fn approving_adjusted_scope_does_not_preserve_origin_but_dispatches() {
    let ai = ai_changed_since();
    let widened = ai
        .assess_against(
            "assessment:widened".to_owned(),
            ai.snapshot_fingerprint.clone(),
            &[
                target("tree:login", DurableTestNodeKind::ConcreteCase),
                target("tree:logout", DurableTestNodeKind::ConcreteCase),
            ],
            refs(&["evidence:a"]),
        )
        .with_decision(WidenedSelectionReviewState::ApprovedAsAdjusted);
    assert!(!widened.preserves_origin);
    assert!(widened.dispatch_allowed());
    assert!(widened.is_consistent());
}

#[test]
fn widening_hidden_behind_no_review_is_rejected() {
    let mut packet = valid_packet();
    // Force the widened assessment to claim no review is required.
    let widened = packet
        .assessments
        .iter_mut()
        .find(|a| a.compatibility_class == TargetCompatibilityClass::WidenedNeedsReview)
        .expect("widened assessment");
    widened.review_state = WidenedSelectionReviewState::NotRequired;
    let violations = packet.validate();
    assert!(violations.contains(&PortableSelectionViolation::WideningHidesReview));
    assert!(violations.contains(&PortableSelectionViolation::AssessmentInconsistent));
}

#[test]
fn target_fingerprint_cannot_substitute_id() {
    let mut packet = valid_packet();
    let target = &mut packet.selections[0].pinned_targets[0];
    target.target_fingerprint_token = target.target_id.clone();
    let violations = packet.validate();
    assert!(violations.contains(&PortableSelectionViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn template_and_invocation_kinds_must_both_appear() {
    let mut packet = valid_packet();
    // Strip every invocation target across selections.
    for selection in &mut packet.selections {
        selection
            .pinned_targets
            .retain(|t| t.node_kind != DurableTestNodeKind::ConcreteInvocation);
    }
    let violations = packet.validate();
    assert!(violations.contains(&PortableSelectionViolation::TemplateCollapsedWithInvocation));
}

#[test]
fn imported_selection_with_rerunnable_policy_is_rejected() {
    let mut packet = valid_packet();
    let imported = packet
        .selections
        .iter_mut()
        .find(|s| s.selection_id == "selection:imported")
        .expect("imported selection");
    imported.expansion_policy = ExpansionPolicy::ReresolveWithinSnapshot;
    assert!(!imported.imported_policy_ok());
    let violations = packet.validate();
    assert!(violations.contains(&PortableSelectionViolation::ImportedRerunAsLocal));
}

#[test]
fn snapshot_drift_flag_must_agree_with_selection() {
    let mut packet = valid_packet();
    // Flip the drift flag on the compatible assessment without changing the
    // snapshot fingerprint it references.
    let compatible = packet
        .assessments
        .iter_mut()
        .find(|a| a.compatibility_class == TargetCompatibilityClass::Compatible)
        .expect("compatible assessment");
    compatible.snapshot_drifted = true;
    let violations = packet.validate();
    assert!(violations.contains(&PortableSelectionViolation::SnapshotDriftFlagMismatch));
}

#[test]
fn assessment_referencing_unknown_selection_is_rejected() {
    let mut packet = valid_packet();
    packet.assessments[0].selection_ref = "selection:does-not-exist".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&PortableSelectionViolation::AssessmentSelectionUnresolved));
}

#[test]
fn missing_required_channel_is_rejected() {
    let mut packet = valid_packet();
    packet
        .selections
        .retain(|s| s.origin_channel != SelectorChannel::Cli);
    // Dropping the only CLI selection also drops the compatible assessment's
    // referent, but coverage independently flags the missing channel.
    let violations = packet.validate();
    assert!(violations.contains(&PortableSelectionViolation::ChannelMissing));
}

#[test]
fn changed_since_without_ref_is_invalid() {
    let mut selection = ai_changed_since();
    selection.query.changed_since_ref = None;
    assert!(!selection.intent_query_consistent());
    assert!(!selection.is_valid());
}

#[test]
fn export_safe_json_round_trips() {
    let packet = valid_packet();
    let json = packet.export_safe_json();
    let parsed: PortableSelectionPacket = serde_json::from_str(&json).expect("round trip parses");
    assert_eq!(parsed, packet);
    assert!(parsed.validate().is_empty());
}

#[test]
fn markdown_summary_mentions_review_and_dispatch() {
    let packet = valid_packet();
    let md = packet.render_markdown_summary();
    assert!(md.contains("widened_needs_review"));
    assert!(md.contains("dispatch blocked"));
    assert!(md.contains("would add"));
}

#[test]
fn guardrails_must_all_hold() {
    let mut packet = valid_packet();
    packet.guardrails.widening_opens_review = false;
    assert!(packet
        .validate()
        .contains(&PortableSelectionViolation::GuardrailsIncomplete));
}
