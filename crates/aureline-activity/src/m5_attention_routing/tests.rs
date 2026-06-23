//! Unit tests for the attention-routing matrix builder, invariants, and
//! export-safety rules.

use super::*;

#[test]
fn matrix_validates_and_all_invariants_hold() {
    let matrix = attention_routing_matrix();
    matrix.validate().expect("canonical matrix validates");
    assert!(matrix.all_invariants_hold());
    assert!(!matrix.invariants.is_empty());
}

#[test]
fn matrix_is_deterministic() {
    assert_eq!(attention_routing_matrix(), attention_routing_matrix());
}

#[test]
fn matrix_is_support_export_safe() {
    let matrix = attention_routing_matrix();
    assert!(matrix.raw_payload_excluded);
    assert!(matrix.is_support_export_safe());
}

#[test]
fn every_object_family_is_present_once() {
    let matrix = attention_routing_matrix();
    assert_eq!(matrix.objects.len(), AttentionObjectClass::ALL.len());
    for class in AttentionObjectClass::ALL {
        let entry = matrix.object(class).expect("object present");
        assert_eq!(entry.object_id, class.object_id());
        assert!(!entry.canonical_schema_refs.is_empty());
        assert!(!entry.produced_by_refs.is_empty());
        assert!(!entry.proof_packet_ref.is_empty());
        assert!(!entry.applicable_states.is_empty());
        assert!(!entry.controlled_vocabularies.is_empty());
        assert!(entry.required_fields.iter().any(|f| f.required));
        assert!(!entry.reopen_targets.is_empty());
    }
}

#[test]
fn every_fanout_channel_is_present_once() {
    let matrix = attention_routing_matrix();
    assert_eq!(matrix.channels.len(), FanoutChannelClass::ALL.len());
    for class in FanoutChannelClass::ALL {
        let entry = matrix.channel(class).expect("channel present");
        assert_eq!(entry.channel_id, class.channel_id());
        assert!(!entry.can_bypass_preview_approval);
        assert!(entry.quiet_hours_respected);
    }
}

#[test]
fn state_vocabulary_is_complete_and_unique() {
    let matrix = attention_routing_matrix();
    assert_eq!(
        matrix.state_vocabulary.len(),
        AttentionStateClass::ALL.len()
    );
    let mut tokens = std::collections::BTreeSet::new();
    for term in &matrix.state_vocabulary {
        assert_eq!(term.token, term.state.as_str());
        assert!(!term.derived_from_refs.is_empty());
        assert!(tokens.insert(term.token.clone()), "duplicate state token");
    }
}

#[test]
fn every_applicable_state_is_a_defined_vocabulary_term() {
    let matrix = attention_routing_matrix();
    for object in &matrix.objects {
        for state in &object.applicable_states {
            assert!(
                matrix.state_term(*state).is_some(),
                "object {} references undefined state {}",
                object.object.as_str(),
                state.as_str()
            );
        }
    }
}

#[test]
fn every_controlled_vocabulary_is_bound_by_some_object() {
    let matrix = attention_routing_matrix();
    for vocab in ControlledVocabulary::ALL {
        assert!(
            matrix.objects.iter().any(|o| o.binds(vocab)),
            "controlled vocabulary {} bound by no object",
            vocab.as_str()
        );
    }
}

#[test]
fn no_long_running_work_is_toast_only() {
    let matrix = attention_routing_matrix();
    for object in &matrix.objects {
        let needs_durable = object
            .applicable_states
            .iter()
            .any(|s| s.requires_durable_object());
        if needs_durable {
            assert!(
                object.carries_durable_record,
                "{} shows durable-requiring states but is not a durable record",
                object.object.as_str()
            );
        }
    }
}

#[test]
fn fanout_receipt_labels_stale_and_undelivered() {
    let matrix = attention_routing_matrix();
    let receipt = matrix
        .object(AttentionObjectClass::FanoutReceipt)
        .expect("fanout receipt present");
    assert!(receipt.can_show(AttentionStateClass::FanoutStale));
    assert!(receipt.can_show(AttentionStateClass::FanoutUndelivered));
    assert!(receipt.binds(ControlledVocabulary::FanoutDelivery));
}

#[test]
fn no_channel_can_bypass_preview_approval() {
    let matrix = attention_routing_matrix();
    for channel in &matrix.channels {
        assert!(
            !channel.can_bypass_preview_approval,
            "{} must not bypass preview/approval",
            channel.channel.as_str()
        );
    }
}

#[test]
fn suppression_states_stay_separate_from_history() {
    let matrix = attention_routing_matrix();
    for object in &matrix.objects {
        let shows_suppressed = object.can_show(AttentionStateClass::Suppressed);
        let shows_quiet_hours = object.can_show(AttentionStateClass::QuietHoursDeferred);
        if shows_suppressed {
            assert!(
                object.binds(ControlledVocabulary::Suppression),
                "{} must bind the suppression vocabulary",
                object.object.as_str()
            );
        }
        if shows_quiet_hours {
            assert!(
                object.binds(ControlledVocabulary::QuietHours),
                "{} must bind the quiet-hours vocabulary",
                object.object.as_str()
            );
        }
        if shows_suppressed || shows_quiet_hours {
            assert!(
                object.retention_rule.separate_from_audit_history,
                "{} must keep suppression state separate from audit history",
                object.object.as_str()
            );
        }
    }
}

#[test]
fn every_object_reopens_an_authoritative_target() {
    let matrix = attention_routing_matrix();
    for object in &matrix.objects {
        assert!(
            object.binds(ControlledVocabulary::ReopenRouting),
            "{} must bind reopen routing",
            object.object.as_str()
        );
        assert!(
            !object.reopen_targets.is_empty(),
            "{} must name a reopen target",
            object.object.as_str()
        );
    }
}

#[test]
fn badge_aggregate_derives_from_deduped_durable_items() {
    let matrix = attention_routing_matrix();
    let badge = matrix
        .object(AttentionObjectClass::BadgeAggregate)
        .expect("badge aggregate present");
    assert!(badge.binds(ControlledVocabulary::DedupeRule));
    assert!(badge.carries_durable_record);
    assert!(badge.can_reopen(ReopenTargetClass::ActivityJobRow));
}

#[test]
fn validate_rejects_a_raw_payload_flag_flip() {
    let mut matrix = attention_routing_matrix();
    matrix.raw_payload_excluded = false;
    assert!(matrix.validate().is_err());
    assert!(!matrix.is_support_export_safe());
}

#[test]
fn validate_rejects_an_unsafe_ref() {
    let mut matrix = attention_routing_matrix();
    matrix.objects[0]
        .produced_by_refs
        .push("https://internal.example.com/secret".to_owned());
    assert!(!matrix.is_support_export_safe());
    assert!(matrix.validate().is_err());
}

#[test]
fn validate_rejects_a_missing_proof_packet() {
    let mut matrix = attention_routing_matrix();
    matrix.objects[0].proof_packet_ref = String::new();
    assert!(matrix.validate().is_err());
}

#[test]
fn human_readable_projection_renders() {
    let matrix = attention_routing_matrix();
    let lines = attention_routing_lines(&matrix);
    assert!(lines.iter().any(|l| l.contains("Attention-routing matrix")));
    assert!(lines.iter().any(|l| l.contains("Objects:")));
    assert!(lines.iter().any(|l| l.contains("Channels:")));
    for class in AttentionObjectClass::ALL {
        assert!(
            lines.iter().any(|l| l.contains(class.as_str())),
            "projection must mention object {}",
            class.as_str()
        );
    }
}
