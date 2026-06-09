use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_command_discoverability_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn empty_protected_corpus_is_rejected() {
    let mut packet = seeded_command_discoverability_packet();
    packet.commands.clear();
    assert!(packet
        .validate()
        .contains(&DiscoverabilitySupportViolation::EmptyProtectedCorpus));
}

#[test]
fn stable_command_missing_help_anchor_is_rejected() {
    let mut packet = seeded_command_discoverability_packet();
    let command = packet
        .commands
        .iter_mut()
        .find(|command| command.stable_line_required)
        .expect("seeded packet must contain a stable command");
    let command_id = command.command_id.clone();
    command.docs_help_anchor_ref.clear();
    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        DiscoverabilitySupportViolation::StableCommandMissingHelpAnchor(found)
            if found == &command_id
    )));
}

#[test]
fn deprecated_alias_without_replacement_is_rejected() {
    let mut packet = seeded_command_discoverability_packet();
    let command = packet
        .commands
        .iter_mut()
        .find(|command| {
            command
                .alias_records
                .iter()
                .any(|alias| alias.alias_state != "active")
        })
        .expect("seeded packet must contain a deprecated alias");
    let command_id = command.command_id.clone();
    let alias = command
        .alias_records
        .iter_mut()
        .find(|alias| alias.alias_state != "active")
        .expect("seeded packet must contain a deprecated alias");
    let alias_id = alias.alias_id.clone();
    alias.replacement_command_id = None;
    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        DiscoverabilitySupportViolation::AliasLifecycleIncomplete(found_command, found_alias)
            if found_command == &command_id && found_alias == &alias_id
    )));
}

#[test]
fn query_session_must_remain_local_first() {
    let mut packet = seeded_command_discoverability_packet();
    packet.query_session_policy.sync_posture = QuerySessionSyncPostureClass::GovernedSync;
    assert!(packet
        .validate()
        .contains(&DiscoverabilitySupportViolation::QuerySessionNotLocalFirst));
}

#[test]
fn query_session_controls_must_forbid_raw_export() {
    let mut packet = seeded_command_discoverability_packet();
    packet.query_session_policy.raw_query_export_allowed = true;
    assert!(packet
        .validate()
        .contains(&DiscoverabilitySupportViolation::QuerySessionControlsIncomplete));
}

#[test]
fn checked_artifact_validates() {
    let packet =
        current_command_discoverability_export().expect("checked discoverability export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = seeded_command_discoverability_packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir = format!(
        "{root}/artifacts/commands/m4/stabilize_command_discoverability_records_alias_history"
    );
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        format!("{dir}/support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
    std::fs::write(
        format!("{dir}/summary.md"),
        packet.render_markdown_summary(),
    )
    .unwrap();
    let fixture_dir = format!(
        "{root}/fixtures/commands/m4/stabilize_command_discoverability_records_alias_history"
    );
    std::fs::create_dir_all(&fixture_dir).unwrap();
    std::fs::write(
        format!("{fixture_dir}/discoverability_support_packet.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
}
