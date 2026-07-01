use super::*;

#[test]
fn seeded_set_validates() {
    let set = seeded_m5_reproduction_packet_set();
    assert!(set.validate().is_ok(), "{:?}", set.validate());
    assert_eq!(set.packet_set_id, M5_REPRODUCTION_PACKET_SET_ID);
}

#[test]
fn seeded_set_names_every_surface_once() {
    let set = seeded_m5_reproduction_packet_set();
    assert_eq!(set.packets.len(), OriginatingSurfaceClass::ALL.len());
    for surface in OriginatingSurfaceClass::ALL {
        let count = set
            .packets
            .iter()
            .filter(|p| p.originating_surface == surface)
            .count();
        assert_eq!(count, 1, "surface {} not named once", surface.as_str());
    }
}

#[test]
fn every_flow_is_exercised() {
    let set = seeded_m5_reproduction_packet_set();
    for flow in PacketFlowClass::ALL {
        assert!(
            set.packets.iter().any(|p| p.flow == flow),
            "flow {} not exercised",
            flow.as_str()
        );
    }
}

#[test]
fn every_redactable_field_is_covered() {
    let set = seeded_m5_reproduction_packet_set();
    for field in RedactableFieldClass::ALL {
        let covered = set
            .packets
            .iter()
            .any(|p| p.redaction_preview.iter().any(|r| r.field_class == field));
        assert!(covered, "field {} not covered", field.as_str());
    }
}

#[test]
fn tokens_are_always_removed() {
    let set = seeded_m5_reproduction_packet_set();
    for packet in &set.packets {
        for row in &packet.redaction_preview {
            if row.field_class == RedactableFieldClass::Token {
                assert_eq!(row.default_action, RedactionActionClass::RemovedEntirely);
                assert_eq!(row.chosen_action, RedactionActionClass::RemovedEntirely);
                assert!(row.mandatory_redaction);
            }
        }
    }
}

#[test]
fn every_packet_excludes_secrets_and_never_auto_submits() {
    let set = seeded_m5_reproduction_packet_set();
    for packet in &set.packets {
        assert!(packet.raw_secrets_excluded);
        assert!(packet.raw_screenshots_excluded);
        assert!(packet.hidden_approvals_excluded);
        assert!(packet.unmanaged_capture_excluded);
        assert!(!packet.auto_submit_on_create_allowed);
    }
}

#[test]
fn save_local_packets_never_leave_and_stay_offline() {
    let set = seeded_m5_reproduction_packet_set();
    for packet in &set.packets {
        if packet.flow == PacketFlowClass::SaveLocal {
            assert_eq!(
                packet.data_exit_boundary,
                DataExitBoundary::NoPayloadLeavesProduct
            );
            assert!(packet.offline_reusable);
        }
    }
}

#[test]
fn shared_packets_require_preview_confirmation() {
    let set = seeded_m5_reproduction_packet_set();
    for packet in &set.packets {
        let leaves = packet.flow.leaves_product_on_share()
            || packet.data_exit_boundary != DataExitBoundary::NoPayloadLeavesProduct;
        if leaves {
            assert!(
                packet.preview_confirmed_before_share,
                "packet {} shares without preview confirmation",
                packet.packet_id
            );
        }
    }
}

#[test]
fn loosening_a_redaction_fails() {
    let mut set = seeded_m5_reproduction_packet_set();
    // Default for the hostname row is generalized; try to loosen the chosen
    // action to an object ref (more exposed).
    let row = set.packets[0]
        .redaction_preview
        .iter_mut()
        .find(|r| r.field_class == RedactableFieldClass::Hostname)
        .expect("hostname row present");
    row.chosen_action = RedactionActionClass::IncludedAsObjectRef;
    assert!(matches!(
        set.validate(),
        Err(ReproductionPacketError::ChosenLoosensRedaction { .. })
    ));
}

#[test]
fn token_kept_as_ref_fails() {
    let mut set = seeded_m5_reproduction_packet_set();
    let row = set.packets[1]
        .redaction_preview
        .iter_mut()
        .find(|r| r.field_class == RedactableFieldClass::Token)
        .expect("token row present");
    row.default_action = RedactionActionClass::IncludedAsObjectRef;
    row.chosen_action = RedactionActionClass::IncludedAsObjectRef;
    assert!(matches!(
        set.validate(),
        Err(ReproductionPacketError::FieldActionNotAllowed { .. })
    ));
}

#[test]
fn auto_submit_on_create_fails() {
    let mut set = seeded_m5_reproduction_packet_set();
    set.packets[0].auto_submit_on_create_allowed = true;
    assert!(matches!(
        set.validate(),
        Err(ReproductionPacketError::AutoSubmitOnCreate { .. })
    ));
}

#[test]
fn missing_secret_exclusion_fails() {
    let mut set = seeded_m5_reproduction_packet_set();
    set.packets[0].raw_secrets_excluded = false;
    assert!(matches!(
        set.validate(),
        Err(ReproductionPacketError::GuardrailExclusionMissing { .. })
    ));
}

#[test]
fn save_local_that_leaves_product_fails() {
    let mut set = seeded_m5_reproduction_packet_set();
    let packet = set
        .packets
        .iter_mut()
        .find(|p| p.flow == PacketFlowClass::SaveLocal)
        .expect("save-local packet present");
    packet.data_exit_boundary = DataExitBoundary::RedactedSupportPacket;
    assert!(matches!(
        set.validate(),
        Err(ReproductionPacketError::FlowDataExitMismatch { .. })
            | Err(ReproductionPacketError::PostureDataExitMismatch { .. })
    ));
}

#[test]
fn share_without_preview_confirmation_fails() {
    let mut set = seeded_m5_reproduction_packet_set();
    let packet = set
        .packets
        .iter_mut()
        .find(|p| p.flow == PacketFlowClass::CopySummary)
        .expect("copy-summary packet present");
    packet.preview_confirmed_before_share = false;
    assert!(matches!(
        set.validate(),
        Err(ReproductionPacketError::ShareBeforePreviewConfirmed { .. })
    ));
}

#[test]
fn missing_surface_fails() {
    let mut set = seeded_m5_reproduction_packet_set();
    set.packets
        .retain(|p| p.originating_surface != OriginatingSurfaceClass::OtherSurface);
    assert!(matches!(
        set.validate(),
        Err(ReproductionPacketError::SurfaceMissing { .. })
    ));
}

#[test]
fn missing_source_contract_fails() {
    let mut set = seeded_m5_reproduction_packet_set();
    set.source_contract_refs
        .retain(|r| r != M5_REPRODUCTION_PACKET_PUBLIC_MATRIX_REF);
    assert!(matches!(
        set.validate(),
        Err(ReproductionPacketError::MissingSourceContracts)
    ));
}

#[test]
fn raw_ref_leak_fails() {
    let mut set = seeded_m5_reproduction_packet_set();
    set.packets[0].object_anchor.object_ref = "https://example.com/object".to_owned();
    assert!(matches!(
        set.validate(),
        Err(ReproductionPacketError::RawRefLeak { .. })
            | Err(ReproductionPacketError::RawMaterialInExport)
    ));
}

#[test]
fn duplicate_packet_id_fails() {
    let mut set = seeded_m5_reproduction_packet_set();
    let dup = set.packets[0].clone();
    set.packets.push(dup);
    assert!(matches!(
        set.validate(),
        Err(ReproductionPacketError::DuplicatePacketId { .. })
            | Err(ReproductionPacketError::SurfaceMissing { .. })
    ));
}

#[test]
fn matrix_csv_has_a_row_per_packet() {
    let set = seeded_m5_reproduction_packet_set();
    let csv = set.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + set.packets.len());
    assert!(lines[0].starts_with("packet,originating_surface,"));
    for packet in &set.packets {
        assert!(
            csv.contains(&packet.packet_id),
            "csv missing {}",
            packet.packet_id
        );
    }
}

#[test]
fn markdown_summary_lists_every_packet() {
    let set = seeded_m5_reproduction_packet_set();
    let summary = set.render_markdown_summary();
    for packet in &set.packets {
        assert!(
            summary.contains(&packet.packet_id),
            "summary missing {}",
            packet.packet_id
        );
    }
}

#[test]
fn copy_summary_lists_every_redaction_row() {
    let set = seeded_m5_reproduction_packet_set();
    for packet in &set.packets {
        let text = packet.render_copy_summary();
        for row in &packet.redaction_preview {
            assert!(
                text.contains(row.field_class.as_str()),
                "copy summary for {} missing {}",
                packet.packet_id,
                row.field_class.as_str()
            );
        }
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_reproduction_packet_set().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}

#[test]
fn narrowed_fixture_packets_validate() {
    for packet in [
        seeded_save_local_offline_draft_packet(),
        seeded_tokens_and_approvals_removed_packet(),
    ] {
        assert!(packet.validate().is_ok(), "{:?}", packet.validate());
    }
}

#[test]
fn save_local_offline_draft_stays_local_and_unconfirmed() {
    let packet = seeded_save_local_offline_draft_packet();
    assert_eq!(packet.flow, PacketFlowClass::SaveLocal);
    assert_eq!(
        packet.data_exit_boundary,
        DataExitBoundary::NoPayloadLeavesProduct
    );
    assert!(packet.offline_reusable);
    assert!(!packet.preview_confirmed_before_share);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_reproduction_packet_set()
        .expect("checked reproduction packet set validates");
    assert_eq!(
        from_disk,
        seeded_m5_reproduction_packet_set(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_fixtures_match_seed_builders() {
    let draft: ReproductionPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/help/reproduction-packets/save_local_offline_draft.json"
    )))
    .expect("offline-draft fixture parses");
    assert_eq!(draft, seeded_save_local_offline_draft_packet());

    let removed: ReproductionPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/help/reproduction-packets/tokens_and_approvals_removed.json"
    )))
    .expect("secrets-removed fixture parses");
    assert_eq!(removed, seeded_tokens_and_approvals_removed_packet());
}
