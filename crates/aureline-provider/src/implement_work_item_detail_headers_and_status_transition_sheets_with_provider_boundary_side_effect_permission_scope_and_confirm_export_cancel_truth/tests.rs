use super::*;

const PACKET_ID: &str = DETAIL_HEADER_TRANSITION_PACKET_ID;

fn packet() -> DetailHeaderTransitionControlsPacket {
    seeded_detail_header_transition_controls()
}

#[test]
fn seed_packet_validates() {
    let packet = packet();
    assert!(
        packet.validate().is_empty(),
        "seed packet failed validation: {:?}",
        packet.validate()
    );
    assert_eq!(packet.packet_id, PACKET_ID);
    assert_eq!(packet.record_kind, DETAIL_HEADER_TRANSITION_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        DETAIL_HEADER_TRANSITION_SCHEMA_VERSION
    );
}

#[test]
fn header_boundary_is_derived_not_asserted() {
    use HeaderFreshnessClass as Fresh;
    use HeaderWriteScope as Scope;
    use M5WorkItemLocalState as Local;
    use M5WorkItemProviderAuthority as Authority;

    // Provider-owned, current, known → provider-writable, live-synced, provider-backed.
    let d = resolve_detail_header(
        Authority::ProviderOwned,
        Local::SyncedWithProvider,
        true,
        true,
    );
    assert_eq!(d.write_scope, Scope::ProviderWritable);
    assert_eq!(d.freshness_class, Fresh::LiveSynced);
    assert!(d.is_provider_backed);
    assert!(!d.is_local_draft);
    assert!(!d.needs_write_scope_note);
    assert!(!d.needs_freshness_note);

    // Local draft → local-only scope + freshness, not provider-backed.
    let d = resolve_detail_header(Authority::LocalDraft, Local::LocalOnlyDraft, false, true);
    assert_eq!(d.write_scope, Scope::LocalDraftOnly);
    assert_eq!(d.freshness_class, Fresh::LocalOnly);
    assert!(!d.is_provider_backed);
    assert!(d.is_local_draft);
    assert!(d.needs_write_scope_note);
    assert!(d.needs_freshness_note);

    // Imported snapshot → read-only mirror, stale snapshot.
    let d = resolve_detail_header(
        Authority::ImportedSnapshot,
        Local::SyncedWithProvider,
        true,
        true,
    );
    assert_eq!(d.write_scope, Scope::ReadOnlyMirror);
    assert_eq!(d.freshness_class, Fresh::StaleSnapshot);
    assert!(d.is_provider_backed);

    // Policy-pinned → policy-blocked write, needs policy note.
    let d = resolve_detail_header(
        Authority::PolicyPinned,
        Local::SyncedWithProvider,
        true,
        true,
    );
    assert_eq!(d.write_scope, Scope::PolicyBlockedWrite);
    assert!(d.needs_policy_note);

    // Unknown freshness overrides even for a reachable mirror.
    let d = resolve_detail_header(
        Authority::MirroredReadOnly,
        Local::SyncedWithProvider,
        true,
        false,
    );
    assert_eq!(d.freshness_class, Fresh::UnknownFreshness);
    assert!(d.needs_freshness_note);
}

#[test]
fn transition_publish_is_derived_not_asserted() {
    use M5WorkItemTransitionEffect as Effect;
    use TransitionPublishClass as Class;

    // Local-only transition → nothing publishes.
    let d = resolve_transition_publish(Effect::LocalOnlyTransition, false);
    assert_eq!(d.publish_class, Class::LocalDraftOnly);
    assert!(d.is_local_only);
    assert!(!d.publishes_externally);
    assert!(!d.needs_notification_note);

    // Comment side effect → publishes to provider, notifies.
    let d = resolve_transition_publish(Effect::CommentSideEffect, false);
    assert_eq!(d.publish_class, Class::PublishesToProvider);
    assert!(d.publishes_externally);
    assert!(d.needs_notification_note);

    // Open in provider → opens externally.
    let d = resolve_transition_publish(Effect::OpenInProvider, false);
    assert_eq!(d.publish_class, Class::OpensInProvider);
    assert!(d.publishes_externally);

    // Blocked transition → needs permission, no external mutation.
    let d = resolve_transition_publish(Effect::BlockedTransition, false);
    assert_eq!(d.publish_class, Class::BlockedNeedsPermission);
    assert!(d.is_blocked);
    assert!(!d.publishes_externally);

    // Policy-block overrides everything, no external mutation.
    let d = resolve_transition_publish(Effect::PublishNowTransition, true);
    assert_eq!(d.publish_class, Class::PolicyBlockedTransition);
    assert!(d.is_blocked);
    assert!(!d.publishes_externally);
    assert!(d.needs_policy_note);
}

#[test]
fn header_coverage_is_complete() {
    let packet = packet();
    let scopes: std::collections::BTreeSet<_> = packet
        .detail_headers
        .iter()
        .map(|h| h.boundary_disclosure().write_scope)
        .collect();
    for scope in HeaderWriteScope::ALL {
        assert!(scopes.contains(&scope), "missing write scope {scope:?}");
    }
    let fresh: std::collections::BTreeSet<_> = packet
        .detail_headers
        .iter()
        .map(|h| h.boundary_disclosure().freshness_class)
        .collect();
    for class in HeaderFreshnessClass::ALL {
        assert!(fresh.contains(&class), "missing freshness class {class:?}");
    }
}

#[test]
fn transition_coverage_is_complete() {
    let packet = packet();
    let classes: std::collections::BTreeSet<_> = packet
        .status_transition_sheets
        .iter()
        .map(|s| s.publish_disclosure().publish_class)
        .collect();
    for class in TransitionPublishClass::ALL {
        assert!(classes.contains(&class), "missing publish class {class:?}");
    }
    let mutations: std::collections::BTreeSet<_> = packet
        .status_transition_sheets
        .iter()
        .flat_map(|s| s.mutation_kinds.iter().copied())
        .collect();
    for kind in TransitionMutationKind::ALL {
        assert!(mutations.contains(&kind), "missing mutation kind {kind:?}");
    }
    let scopes: std::collections::BTreeSet<_> = packet
        .status_transition_sheets
        .iter()
        .map(|s| s.permission_scope)
        .collect();
    for scope in PermissionScopeClass::ALL {
        assert!(
            scopes.contains(&scope),
            "missing permission scope {scope:?}"
        );
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::MissingSourceContracts));
}

#[test]
fn empty_detail_headers_fails() {
    let mut packet = packet();
    packet.detail_headers.clear();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::DetailHeadersMissing));
}

#[test]
fn empty_transition_sheets_fails() {
    let mut packet = packet();
    packet.status_transition_sheets.clear();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::StatusTransitionSheetsMissing));
}

#[test]
fn header_wrong_component_class_fails() {
    let mut packet = packet();
    packet.detail_headers[0].component = M5WorkItemComponentFamily::StatusTransitionSheet;
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::DetailHeaderWrongComponentClass));
}

#[test]
fn header_write_scope_misrepresented_fails() {
    let mut packet = packet();
    let header = packet
        .detail_headers
        .iter_mut()
        .find(|h| h.boundary_disclosure().write_scope == HeaderWriteScope::LocalDraftOnly)
        .expect("local-draft header present");
    header.write_scope = HeaderWriteScope::ProviderWritable;
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::HeaderWriteScopeMisrepresented));
}

#[test]
fn header_freshness_misrepresented_fails() {
    let mut packet = packet();
    let header = packet
        .detail_headers
        .iter_mut()
        .find(|h| h.boundary_disclosure().freshness_class == HeaderFreshnessClass::StaleSnapshot)
        .expect("stale header present");
    header.freshness_class = HeaderFreshnessClass::LiveSynced;
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::HeaderFreshnessMisrepresented));
}

#[test]
fn local_draft_claiming_provider_backed_fails() {
    let mut packet = packet();
    let header = packet
        .detail_headers
        .iter_mut()
        .find(|h| h.boundary_disclosure().is_local_draft)
        .expect("local-draft header present");
    header.claims_provider_backed = true;
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::LocalDraftMisrepresentedAsProviderBacked));
}

#[test]
fn missing_write_scope_note_fails() {
    let mut packet = packet();
    let header = packet
        .detail_headers
        .iter_mut()
        .find(|h| h.boundary_disclosure().needs_write_scope_note)
        .expect("non-writable header present");
    header.write_scope_note.clear();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::WriteScopeNoteMissing));
}

#[test]
fn missing_freshness_note_fails() {
    let mut packet = packet();
    let header = packet
        .detail_headers
        .iter_mut()
        .find(|h| h.boundary_disclosure().needs_freshness_note)
        .expect("non-live header present");
    header.freshness_note.clear();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::FreshnessNoteMissing));
}

#[test]
fn missing_header_policy_note_fails() {
    let mut packet = packet();
    let header = packet
        .detail_headers
        .iter_mut()
        .find(|h| h.boundary_disclosure().needs_policy_note)
        .expect("policy-pinned header present");
    header.policy_block_note.clear();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::HeaderPolicyBlockNoteMissing));
}

#[test]
fn missing_header_escape_hatch_fails() {
    let mut packet = packet();
    packet.detail_headers[0].actions = vec![DetailHeaderAction::RevealScope];
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::HeaderOpenExternalOrCopyMissing));
}

#[test]
fn transition_wrong_component_class_fails() {
    let mut packet = packet();
    packet.status_transition_sheets[0].component = M5WorkItemComponentFamily::WorkItemDetailHeader;
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::StatusTransitionSheetWrongComponentClass));
}

#[test]
fn transition_publish_class_misrepresented_fails() {
    let mut packet = packet();
    packet.status_transition_sheets[0].publish_class = TransitionPublishClass::PublishesToProvider;
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::TransitionPublishClassMisrepresented));
}

#[test]
fn local_transition_implying_external_mutation_fails() {
    let mut packet = packet();
    let sheet = packet
        .status_transition_sheets
        .iter_mut()
        .find(|s| s.publish_disclosure().is_local_only)
        .expect("local-only sheet present");
    sheet.implies_external_mutation = true;
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::ExternalMutationMisrepresented));
}

#[test]
fn external_transition_denying_mutation_fails() {
    let mut packet = packet();
    let sheet = packet
        .status_transition_sheets
        .iter_mut()
        .find(|s| s.publish_disclosure().publishes_externally)
        .expect("external sheet present");
    sheet.implies_external_mutation = false;
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::ExternalMutationMisrepresented));
}

#[test]
fn missing_side_effect_preview_fails() {
    let mut packet = packet();
    packet.status_transition_sheets[0]
        .side_effect_preview_label
        .clear();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::SideEffectPreviewMissing));
}

#[test]
fn missing_linked_context_note_fails() {
    let mut packet = packet();
    packet.status_transition_sheets[0]
        .linked_context_note
        .clear();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::LinkedContextNoteMissing));
}

#[test]
fn missing_notification_note_fails() {
    let mut packet = packet();
    let sheet = packet
        .status_transition_sheets
        .iter_mut()
        .find(|s| s.publish_disclosure().needs_notification_note)
        .expect("externally-publishing sheet present");
    sheet.notification_side_effect_note.clear();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::NotificationSideEffectNoteMissing));
}

#[test]
fn missing_permission_scope_note_fails() {
    let mut packet = packet();
    packet.status_transition_sheets[0]
        .permission_scope_note
        .clear();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::PermissionScopeNoteMissing));
}

#[test]
fn missing_confirm_export_cancel_fails() {
    let mut packet = packet();
    packet.status_transition_sheets[0].actions = vec![TransitionSheetAction::Confirm];
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::ConfirmExportCancelIncomplete));
}

#[test]
fn missing_export_fallback_note_fails() {
    let mut packet = packet();
    packet.status_transition_sheets[0]
        .export_fallback_note
        .clear();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::ExportFallbackNoteMissing));
}

#[test]
fn missing_transition_policy_note_fails() {
    let mut packet = packet();
    let sheet = packet
        .status_transition_sheets
        .iter_mut()
        .find(|s| s.publish_disclosure().needs_policy_note)
        .expect("policy-blocked sheet present");
    sheet.policy_block_note.clear();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::TransitionPolicyBlockNoteMissing));
}

#[test]
fn missing_transition_status_labels_fails() {
    let mut packet = packet();
    packet.status_transition_sheets[0].to_status.clear();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::TransitionStatusLabelsMissing));
}

#[test]
fn empty_mutation_kinds_fails() {
    let mut packet = packet();
    packet.status_transition_sheets[0].mutation_kinds.clear();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::TransitionMutationKindsMissing));
}

#[test]
fn generic_ticket_wording_fails() {
    let mut packet = packet();
    packet.detail_headers[0].uses_generic_ticket_wording = true;
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::GenericTicketWordingUsed));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .local_transition_never_implies_external_mutation = false;
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .transition_surface_previews_before_publish = false;
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.detail_headers[0].title = "see https://internal.example/board".to_owned();
    assert!(packet
        .validate()
        .contains(&DetailHeaderTransitionViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Detail headers"));
    assert!(summary.contains("## Status-transition sheets"));
    assert!(summary.contains("policy_blocked_transition"));
    assert!(summary.contains("local_only"));
}

#[test]
fn matrix_csv_has_a_line_per_control() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 5 detail headers + 5 transition sheets
    assert_eq!(lines, 1 + 5 + 5);
    assert!(csv.contains("detail_header"));
    assert!(csv.contains("status_transition_sheet"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_detail_header_transition_export()
        .expect("checked detail header transition export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-work-item-detail-header-status-transition-controls/detail_header_local_draft.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-work-item-detail-header-status-transition-controls/status_transition_publish_now.json"
        )),
    ] {
        let packet: DetailHeaderTransitionControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as detail header transition packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn scenario_fixtures_stay_valid_and_covered() {
    for packet in [
        seeded_detail_header_transition_controls_detail_header_local_draft(),
        seeded_detail_header_transition_controls_status_transition_publish_now(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
