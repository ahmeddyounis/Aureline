use std::fs;
use std::path::PathBuf;

use aureline_commands::PolicyContext;
use aureline_input::keybindings::{
    CandidateContext, CommandSemanticsSnapshot, InspectionScope, KeySequence, KeybindingResolver,
    ResolverLayerClass, SequenceResolutionState, WinningResolutionKind,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct KeybindingCaseFixture {
    inspection_scope: InspectionScope,
    reserved_sequences: Vec<String>,
    #[serde(default)]
    admin_locked_sequences: Vec<String>,
    emergency_block_active: bool,
    bindings: Vec<BindingFixture>,
    inspected_sequence: String,
    expected: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct BindingFixture {
    resolver_layer: ResolverLayerClass,
    candidate_ref: String,
    command: CommandSemanticsSnapshot,
    sequence: String,
    #[serde(default)]
    source_provenance_ref: Option<String>,
    #[serde(default)]
    scope_note: Option<String>,
    #[serde(default)]
    context: CandidateContext,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    sequence_state: SequenceResolutionState,
    winner_kind: WinningResolutionKind,
    winner_layer: Option<ResolverLayerClass>,
    winner_command_id: Option<String>,
}

#[test]
fn keybinding_cases_fixture_set_stays_deterministic() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixtures_dir = repo_root.join("fixtures/input/keybinding_cases");

    let mut fixture_paths: Vec<PathBuf> = fs::read_dir(&fixtures_dir)
        .expect("fixture directory must exist")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixture_paths.sort();

    assert!(
        !fixture_paths.is_empty(),
        "expected at least one fixture under {fixtures_dir:?}"
    );

    for path in fixture_paths {
        let raw = fs::read_to_string(&path).expect("fixture should be readable");
        let fixture: KeybindingCaseFixture =
            serde_json::from_str(&raw).expect("fixture should be valid JSON");

        let mut resolver = KeybindingResolver::new(PolicyContext {
            policy_epoch: "pe:test:01".to_string(),
            trust_state: "trusted".to_string(),
            execution_context_id: Some("exec:keybindings:test".to_string()),
        });
        resolver.set_emergency_block_active(fixture.emergency_block_active);

        let reserved = fixture
            .reserved_sequences
            .iter()
            .map(|s| KeySequence::parse_literal_sequence(s).expect("reserved sequence must parse"))
            .collect::<Vec<_>>();
        resolver.set_reserved_sequences(reserved);

        let admin_locked = fixture
            .admin_locked_sequences
            .iter()
            .map(|s| {
                KeySequence::parse_literal_sequence(s).expect("admin locked sequence must parse")
            })
            .collect::<Vec<_>>();
        resolver.set_admin_locked_sequences(admin_locked);

        for binding in fixture.bindings {
            let sequence = KeySequence::parse_literal_sequence(&binding.sequence)
                .expect("binding sequence must parse");
            resolver.push_candidate(
                binding.resolver_layer,
                binding.candidate_ref,
                binding.command,
                sequence,
                binding.source_provenance_ref,
                binding.scope_note,
                binding.context,
            );
        }

        let inspected = KeySequence::parse_literal_sequence(&fixture.inspected_sequence)
            .expect("inspected sequence must parse");
        let packet = resolver.resolve(&inspected, &fixture.inspection_scope);

        assert_eq!(
            packet.sequence_state, fixture.expected.sequence_state,
            "sequence_state mismatch for {path:?}"
        );
        assert_eq!(
            packet.winning_resolution.winner_kind, fixture.expected.winner_kind,
            "winner_kind mismatch for {path:?}"
        );
        assert_eq!(
            packet.winning_resolution.resolver_layer, fixture.expected.winner_layer,
            "winner_layer mismatch for {path:?}"
        );

        match fixture.expected.winner_command_id.as_deref() {
            Some(expected_id) => {
                let Some(candidate) = packet.winning_resolution.command_candidate.as_ref() else {
                    panic!("expected a command_candidate for {path:?}");
                };
                assert_eq!(
                    candidate.command.command_id.as_str(),
                    expected_id,
                    "winner_command_id mismatch for {path:?}"
                );
            }
            None => {
                assert!(
                    packet.winning_resolution.command_candidate.is_none(),
                    "expected no command_candidate for {path:?}"
                );
            }
        }
    }
}
