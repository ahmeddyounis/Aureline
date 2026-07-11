use super::*;

const PACKET_ID: &str = NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_PACKET_ID;

fn packet() -> NotebookDocumentHeaderKernelStateStripControlsPacket {
    seeded_notebook_document_header_kernel_state_strip_controls()
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
    assert_eq!(
        packet.record_kind,
        NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        NOTEBOOK_DOCUMENT_HEADER_KERNEL_STATE_STRIP_SCHEMA_VERSION
    );
}

#[test]
fn document_origin_is_derived_not_asserted() {
    use DocumentOriginClass as Class;
    use M5NotebookDocumentIdentityState as Id;
    use M5NotebookDocumentSourceClass as Src;

    // Local / remote / managed → canonical settled source.
    for (src, class) in [
        (Src::LocalIpynb, Class::LocalDocument),
        (Src::RemoteIpynb, Class::RemoteDocument),
        (Src::ManagedWorkspaceIpynb, Class::ManagedDocument),
    ] {
        let d = resolve_document_header(src, Id::SavedClean);
        assert_eq!(d.origin_class, class);
        assert!(d.is_canonical_source);
    }

    // Imported → imported, not canonical, needs imported note.
    let d = resolve_document_header(Src::ImportedIpynb, Id::SavedClean);
    assert_eq!(d.origin_class, Class::ImportedDocument);
    assert!(!d.is_canonical_source);
    assert!(d.needs_imported_note);

    // Scratch → scratch, not canonical, needs scratch note.
    let d = resolve_document_header(Src::ScratchUntitled, Id::SavedClean);
    assert_eq!(d.origin_class, Class::ScratchDocument);
    assert!(!d.is_canonical_source);
    assert!(d.needs_scratch_note);

    // Unknown → unknown, not canonical, needs unknown note.
    let d = resolve_document_header(Src::UnknownSource, Id::SavedClean);
    assert_eq!(d.origin_class, Class::UnknownDocument);
    assert!(!d.is_canonical_source);
    assert!(d.needs_unknown_source_note);

    // Identity notes are derived independently of the source.
    let d = resolve_document_header(Src::LocalIpynb, Id::Conflicted);
    assert!(d.needs_conflict_note);
    let d = resolve_document_header(Src::LocalIpynb, Id::ReadOnly);
    assert!(d.needs_readonly_note);
    let d = resolve_document_header(Src::LocalIpynb, Id::Recovered);
    assert!(d.needs_recovered_note);
    let d = resolve_document_header(Src::LocalIpynb, Id::UnsavedChanges);
    assert!(d.needs_unsaved_note);
}

#[test]
fn kernel_state_is_derived_not_asserted() {
    use KernelLiveClass as Class;
    use M5KernelConnectionState as Conn;
    use M5KernelExecutionState as Exec;

    // Idle + connected → ready-live.
    let d = resolve_kernel_state(Exec::IdleReady, Conn::ConnectedLocal);
    assert_eq!(d.live_class, Class::ReadyLive);
    assert!(d.is_live);

    // Busy → busy-live.
    let d = resolve_kernel_state(Exec::BusyRunning, Conn::ConnectedRemote);
    assert_eq!(d.live_class, Class::BusyLive);
    assert!(d.is_live);

    // Queued → queued-live.
    let d = resolve_kernel_state(Exec::QueuedPending, Conn::Reconnecting);
    assert_eq!(d.live_class, Class::QueuedLive);
    assert!(d.is_live);

    // Dead-no-kernel → no-kernel-editable, not live, editing preserved, needs no-kernel note.
    let d = resolve_kernel_state(Exec::DeadNoKernel, Conn::NeverConnected);
    assert_eq!(d.live_class, Class::NoKernelEditable);
    assert!(!d.is_live);
    assert!(d.preserves_no_kernel_editing);
    assert!(d.needs_no_kernel_note);

    // Idle but never connected → no-kernel-editable (never reads as live).
    let d = resolve_kernel_state(Exec::IdleReady, Conn::NeverConnected);
    assert_eq!(d.live_class, Class::NoKernelEditable);
    assert!(!d.is_live);

    // Idle but connection dropped → disconnected-recoverable, needs reconnect note.
    for conn in [Conn::Reconnecting, Conn::Disconnected, Conn::ConnectionLost] {
        let d = resolve_kernel_state(Exec::IdleReady, conn);
        assert_eq!(d.live_class, Class::DisconnectedRecoverable);
        assert!(!d.is_live);
        assert!(d.needs_reconnect_note);
    }

    // Disconnected/reconnecting execution → disconnected-recoverable.
    let d = resolve_kernel_state(Exec::DisconnectedReconnecting, Conn::Disconnected);
    assert_eq!(d.live_class, Class::DisconnectedRecoverable);
    assert!(!d.is_live);

    // Interrupted → inspect-only, not live, needs inspect-only note.
    let d = resolve_kernel_state(Exec::Interrupted, Conn::ConnectionLost);
    assert_eq!(d.live_class, Class::InspectOnly);
    assert!(!d.is_live);
    assert!(d.needs_inspect_only_note);
}

#[test]
fn document_source_identity_and_origin_coverage_is_complete() {
    let packet = packet();
    let sources: std::collections::BTreeSet<_> = packet
        .document_headers
        .iter()
        .map(|h| h.source_class)
        .collect();
    for source in M5NotebookDocumentSourceClass::ALL {
        assert!(sources.contains(&source), "missing source {source:?}");
    }
    let identities: std::collections::BTreeSet<_> = packet
        .document_headers
        .iter()
        .map(|h| h.identity_state)
        .collect();
    for identity in M5NotebookDocumentIdentityState::ALL {
        assert!(
            identities.contains(&identity),
            "missing identity {identity:?}"
        );
    }
    let origin: std::collections::BTreeSet<_> = packet
        .document_headers
        .iter()
        .map(|h| h.document_disclosure().origin_class)
        .collect();
    for class in DocumentOriginClass::ALL {
        assert!(origin.contains(&class), "missing origin class {class:?}");
    }
}

#[test]
fn kernel_execution_connection_and_live_coverage_is_complete() {
    let packet = packet();
    let executions: std::collections::BTreeSet<_> = packet
        .kernel_strips
        .iter()
        .map(|s| s.execution_state)
        .collect();
    for exec in M5KernelExecutionState::ALL {
        assert!(executions.contains(&exec), "missing execution {exec:?}");
    }
    let connections: std::collections::BTreeSet<_> = packet
        .kernel_strips
        .iter()
        .map(|s| s.connection_state)
        .collect();
    for conn in M5KernelConnectionState::ALL {
        assert!(connections.contains(&conn), "missing connection {conn:?}");
    }
    let live: std::collections::BTreeSet<_> = packet
        .kernel_strips
        .iter()
        .map(|s| s.kernel_disclosure().live_class)
        .collect();
    for class in KernelLiveClass::ALL {
        assert!(live.contains(&class), "missing live class {class:?}");
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::MissingSourceContracts));
}

#[test]
fn empty_headers_fails() {
    let mut packet = packet();
    packet.document_headers.clear();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::HeadersMissing));
}

#[test]
fn empty_strips_fails() {
    let mut packet = packet();
    packet.kernel_strips.clear();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::StripsMissing));
}

#[test]
fn header_wrong_component_class_fails() {
    let mut packet = packet();
    packet.document_headers[0].component = M5NotebookKernelOutputComponentFamily::KernelStateStrip;
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::HeaderWrongComponentClass));
}

#[test]
fn strip_wrong_component_class_fails() {
    let mut packet = packet();
    packet.kernel_strips[0].component =
        M5NotebookKernelOutputComponentFamily::NotebookDocumentHeader;
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::StripWrongComponentClass));
}

#[test]
fn imported_header_claiming_canonical_fails() {
    let mut packet = packet();
    let header = packet
        .document_headers
        .iter_mut()
        .find(|h| h.origin_class == DocumentOriginClass::ImportedDocument)
        .expect("imported header present");
    header.claims_canonical_source = true;
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::DocumentSourceMisrepresented));
}

#[test]
fn no_kernel_strip_claiming_live_fails() {
    let mut packet = packet();
    let strip = packet
        .kernel_strips
        .iter_mut()
        .find(|s| s.live_class == KernelLiveClass::NoKernelEditable)
        .expect("no-kernel strip present");
    strip.claims_live = true;
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::KernelStateMisrepresented));
}

#[test]
fn missing_imported_note_fails() {
    let mut packet = packet();
    let header = packet
        .document_headers
        .iter_mut()
        .find(|h| h.origin_class == DocumentOriginClass::ImportedDocument)
        .expect("imported header present");
    header.imported_note.clear();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::ImportedNoteMissing));
}

#[test]
fn missing_unknown_source_note_fails() {
    let mut packet = packet();
    let header = packet
        .document_headers
        .iter_mut()
        .find(|h| h.origin_class == DocumentOriginClass::UnknownDocument)
        .expect("unknown-source header present");
    header.unknown_source_note.clear();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::UnknownSourceNoteMissing));
}

#[test]
fn missing_no_kernel_note_fails() {
    let mut packet = packet();
    let strip = packet
        .kernel_strips
        .iter_mut()
        .find(|s| s.live_class == KernelLiveClass::NoKernelEditable)
        .expect("no-kernel strip present");
    strip.no_kernel_note.clear();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::NoKernelNoteMissing));
}

#[test]
fn missing_reconnect_note_fails() {
    let mut packet = packet();
    let strip = packet
        .kernel_strips
        .iter_mut()
        .find(|s| s.live_class == KernelLiveClass::DisconnectedRecoverable)
        .expect("disconnected strip present");
    strip.reconnect_note.clear();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::ReconnectNoteMissing));
}

#[test]
fn missing_notebook_identity_fails() {
    let mut packet = packet();
    packet.document_headers[0].notebook_identity_label.clear();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::NotebookIdentityMissing));
}

#[test]
fn missing_export_state_fails() {
    let mut packet = packet();
    packet.document_headers[0].export_state_label.clear();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::ExportStateMissing));
}

#[test]
fn missing_kernel_origin_label_fails() {
    let mut packet = packet();
    packet.kernel_strips[0].kernel_origin_label.clear();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::KernelOriginLabelMissing));
}

#[test]
fn missing_kernel_free_edit_note_fails() {
    let mut packet = packet();
    packet.kernel_strips[0].kernel_free_edit_note.clear();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::KernelFreeEditNoteMissing));
}

#[test]
fn header_missing_review_action_fails() {
    let mut packet = packet();
    packet.document_headers[0].header_actions = vec![DocumentHeaderAction::OpenDocument];
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::HeaderActionsIncomplete));
}

#[test]
fn strip_missing_continue_action_fails() {
    let mut packet = packet();
    packet.kernel_strips[0].strip_actions = vec![KernelStripAction::SelectKernel];
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::StripActionsIncomplete));
}

#[test]
fn deep_link_action_without_target_fails() {
    let mut packet = packet();
    // The first header offers OpenDeepLink; blank its kind to NoDeepLink.
    packet.document_headers[0].deep_link_kind = DeepLinkKind::NoDeepLink;
    packet.document_headers[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::DeepLinkUnresolved));
}

#[test]
fn resolvable_deep_link_without_ref_fails() {
    let mut packet = packet();
    packet.document_headers[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::DeepLinkRefMissing));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = packet();
    packet.kernel_strips[0].context_note.clear();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::ContextNoteMissing));
}

#[test]
fn missing_dispositions_fails() {
    let mut packet = packet();
    packet.document_headers[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::DispositionsMissing));
}

#[test]
fn header_pretending_kernel_free_is_live_fails() {
    let mut packet = packet();
    packet.document_headers[0].pretends_kernel_free_is_live = true;
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::KernelFreePresentedAsLive));
}

#[test]
fn strip_collapsing_kernel_origins_fails() {
    let mut packet = packet();
    packet.kernel_strips[0].collapses_kernel_origins_into_one_badge = true;
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::KernelOriginsCollapsed));
}

#[test]
fn strip_conflating_document_and_runtime_fails() {
    let mut packet = packet();
    packet.kernel_strips[0].conflates_document_and_runtime_truth = true;
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::DocumentAndRuntimeConflated));
}

#[test]
fn header_hiding_state_behind_hover_only_fails() {
    let mut packet = packet();
    packet.document_headers[0].hides_state_behind_hover_only = true;
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::StateHiddenBehindHoverOnly));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.document_headers[0].required_labels =
        vec![M5NotebookKernelOutputRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.kernel_strips[0].accessibility_routes =
        vec![M5NotebookKernelOutputAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::AccessibilityRouteMissing));
}

#[test]
fn notebook_review_incomplete_fails() {
    let mut packet = packet();
    packet.notebook_review.kernel_free_never_shown_as_live = false;
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::NotebookReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .kernel_state_visible_before_trusting_output = false;
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.document_headers[0].deep_link_ref = "see https://internal.example/notebook".to_owned();
    assert!(packet
        .validate()
        .contains(&NotebookDocumentHeaderKernelStateStripViolation::RawMaterialInExport));
}

#[test]
fn markdown_summary_lists_components() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Notebook document headers"));
    assert!(summary.contains("## Kernel-state strips"));
    assert!(summary.contains("no_kernel_editable"));
    assert!(summary.contains("scratch_document"));
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 document headers + 6 kernel strips
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("notebook_document_header"));
    assert!(csv.contains("kernel_state_strip"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_notebook_document_header_kernel_state_strip_export()
        .expect("checked notebook document header kernel strip export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-notebook-document-header-kernel-state-strip-controls/document_header_scratch.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-notebook-document-header-kernel-state-strip-controls/kernel_state_strip_no_kernel.json"
        )),
    ] {
        let packet: NotebookDocumentHeaderKernelStateStripControlsPacket =
            serde_json::from_str(raw)
                .expect("fixture parses as notebook document header kernel strip packet");
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
        seeded_notebook_document_header_kernel_state_strip_controls_document_header_scratch(),
        seeded_notebook_document_header_kernel_state_strip_controls_kernel_state_strip_no_kernel(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
