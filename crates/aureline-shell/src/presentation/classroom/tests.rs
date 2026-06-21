//! Unit tests for the teaching / classroom role and exercise-packet model.

use super::corpus::{
    classroom_role_example, classroom_role_support_export, exercise_packet_example,
    seeded_classroom_role_corpus, validate_classroom_role_corpus, ClassroomRoleCorpus,
};
use super::roles::{
    project_classroom_profile, ClassroomMember, ClassroomSupportViolation, ClassroomViolation,
    ClientClass, ExercisePacket, ExerciseTarget, ExerciseTargetKind, ExpectedAction,
    MemberCapabilitySummary, ProductAuthorityAttribution, TeachingRole,
};

fn member(
    id: &str,
    role: TeachingRole,
    client: ClientClass,
    authority: ProductAuthorityAttribution,
) -> ClassroomMember {
    ClassroomMember::new(id, role, client, authority)
}

const ALL_ROLES: [TeachingRole; 5] = [
    TeachingRole::Moderator,
    TeachingRole::Participant,
    TeachingRole::Observer,
    TeachingRole::Approver,
    TeachingRole::Scribe,
];

#[test]
fn no_classroom_role_grants_product_authority() {
    for role in ALL_ROLES {
        assert!(!role.grants_terminal_or_debug_control());
        assert!(!role.implies_broader_authority());
        let m = member(
            "m:1",
            role,
            ClientClass::Full,
            ProductAuthorityAttribution::none(),
        );
        assert!(!m.role_grants_terminal_or_debug_control);
        assert!(!m.role_implies_broader_authority);
        assert!(!m.product_authority.granted_by_classroom_role);
        assert!(m.is_consistent());
    }
}

#[test]
fn approval_authority_comes_from_an_external_grant_not_the_badge() {
    // A classroom approver who actually holds product approval sources it from a
    // separate grant; the badge alone confers nothing.
    let teaching_only = member(
        "m:approver-teaching",
        TeachingRole::Approver,
        ClientClass::Full,
        ProductAuthorityAttribution::none(),
    );
    assert!(!teaching_only.product_authority.holds_external_authority);
    assert!(teaching_only.is_consistent());

    let with_grant = member(
        "m:approver-with-grant",
        TeachingRole::Approver,
        ClientClass::Full,
        ProductAuthorityAttribution::external("policy:grant:approval:lead"),
    );
    assert!(with_grant.product_authority.holds_external_authority);
    assert!(!with_grant.product_authority.granted_by_classroom_role);
    assert!(with_grant.is_consistent());
}

#[test]
fn low_bandwidth_moderator_loses_drive_and_joins_as_note_taker() {
    let m = member(
        "m:low",
        TeachingRole::Moderator,
        ClientClass::LowBandwidth,
        ProductAuthorityAttribution::none(),
    );
    // Drive and mutation are omitted on a constrained client; notes remain.
    assert!(!m.capability.can_drive_session);
    assert!(!m.capability.may_expose_mutation_affordance);
    assert!(m.capability.can_take_notes);
    assert!(!m.capability.observer_only);
    assert!(m.capability.joins_safely);
    assert!(m.degrades_honestly);
}

#[test]
fn limited_participant_joins_as_note_taker_with_no_mutation() {
    let m = member(
        "m:limited",
        TeachingRole::Participant,
        ClientClass::Limited,
        ProductAuthorityAttribution::none(),
    );
    assert!(!m.capability.can_drive_session);
    assert!(!m.capability.may_expose_mutation_affordance);
    assert!(m.capability.can_take_notes);
    assert!(m.capability.joins_safely);
    assert!(m.degrades_honestly);
}

#[test]
fn observer_is_observer_only_on_any_client() {
    for client in [
        ClientClass::Full,
        ClientClass::Limited,
        ClientClass::LowBandwidth,
    ] {
        let cap = MemberCapabilitySummary::for_seat(TeachingRole::Observer, client);
        assert!(cap.observer_only);
        assert!(!cap.can_drive_session);
        assert!(!cap.can_take_notes);
        assert!(!cap.may_expose_mutation_affordance);
        assert!(cap.joins_safely);
    }
}

#[test]
fn full_client_runs_each_role_natively() {
    let mod_cap = MemberCapabilitySummary::for_seat(TeachingRole::Moderator, ClientClass::Full);
    assert!(mod_cap.can_drive_session);
    let part_cap = MemberCapabilitySummary::for_seat(TeachingRole::Participant, ClientClass::Full);
    assert!(part_cap.may_expose_mutation_affordance);
    assert!(part_cap.can_take_notes);
    let appr_cap = MemberCapabilitySummary::for_seat(TeachingRole::Approver, ClientClass::Full);
    assert!(appr_cap.may_expose_mutation_affordance);
    let scribe_cap = MemberCapabilitySummary::for_seat(TeachingRole::Scribe, ClientClass::Full);
    assert!(scribe_cap.can_take_notes);
    assert!(!scribe_cap.can_drive_session);
}

#[test]
fn a_well_formed_exercise_packet_is_authority_bounded() {
    let packet = exercise_packet_example();
    assert!(packet.is_well_formed());
    assert!(packet.is_authority_bounded());
    assert!(packet.authority_bound.all_actions_command_backed);
    assert!(packet.authority_bound.constrained_to_declared_targets);
    assert!(!packet.authority_bound.opens_hidden_mutation_path);
    assert!(!packet.authority_bound.widens_product_authority);
}

#[test]
fn a_packet_action_outside_its_targets_is_not_bounded() {
    let packet = ExercisePacket::new(
        "p:1",
        "Exercise",
        vec![ExerciseTarget::new("file:a.rs", ExerciseTargetKind::File)],
        vec![ExpectedAction::new(
            "a.1",
            "Open something else",
            "cmd:editor.open_file",
            "key:editor.open_file",
            "file:not-declared.rs",
        )],
    );
    assert!(!packet.authority_bound.constrained_to_declared_targets);
    assert!(!packet.is_authority_bounded());
}

#[test]
fn a_packet_action_without_a_command_is_not_command_backed() {
    let packet = ExercisePacket::new(
        "p:1",
        "Exercise",
        vec![ExerciseTarget::new("file:a.rs", ExerciseTargetKind::File)],
        vec![ExpectedAction::new(
            "a.1",
            "Do a raw thing",
            "raw:mutate",
            "key:x",
            "file:a.rs",
        )],
    );
    assert!(!packet.authority_bound.all_actions_command_backed);
    assert!(!packet.is_authority_bounded());
}

#[test]
fn profile_validation_catches_a_role_claiming_authority() {
    let mut profile = project_classroom_profile(
        "s:1",
        vec![member(
            "m:1",
            TeachingRole::Moderator,
            ClientClass::Full,
            ProductAuthorityAttribution::none(),
        )],
        Vec::new(),
    );
    profile.members[0].role_implies_broader_authority = true;
    let violations = profile.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ClassroomViolation::RoleImpliesBroaderAuthority { .. })));
}

#[test]
fn profile_validation_catches_a_dishonest_capability_claim() {
    let mut profile = project_classroom_profile(
        "s:1",
        vec![member(
            "m:1",
            TeachingRole::Participant,
            ClientClass::LowBandwidth,
            ProductAuthorityAttribution::none(),
        )],
        Vec::new(),
    );
    // Force a drive/mutation capability the constrained client cannot use.
    profile.members[0].capability.may_expose_mutation_affordance = true;
    let violations = profile.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ClassroomViolation::CapabilityDerivationMismatch { .. }
            | ClassroomViolation::MemberDegradesDishonestly { .. }
    )));
}

#[test]
fn profile_validation_catches_authority_from_the_classroom() {
    let mut profile = project_classroom_profile(
        "s:1",
        vec![member(
            "m:1",
            TeachingRole::Approver,
            ClientClass::Full,
            ProductAuthorityAttribution::none(),
        )],
        Vec::new(),
    );
    profile.members[0]
        .product_authority
        .granted_by_classroom_role = true;
    let violations = profile.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ClassroomViolation::AuthorityAttributionInconsistent { .. }
    )));
}

#[test]
fn seeded_corpus_validates() {
    let corpus = seeded_classroom_role_corpus();
    validate_classroom_role_corpus(&corpus).expect("seeded corpus validates");
    assert!(corpus.summary.no_role_grants_product_authority);
    assert!(corpus.summary.all_authority_externally_sourced);
    assert!(corpus.summary.all_constrained_clients_join_safely);
    assert!(corpus.summary.all_packets_authority_bounded);
    assert!(corpus.summary.constrained_client_demonstrated);
    assert!(corpus.summary.exercise_packet_demonstrated);
    // All five roles are exercised somewhere in the corpus.
    assert_eq!(corpus.summary.roles_covered.len(), ALL_ROLES.len());
}

#[test]
fn corpus_round_trips_through_json() {
    let corpus = seeded_classroom_role_corpus();
    let json = serde_json::to_string_pretty(&corpus).unwrap();
    let parsed: ClassroomRoleCorpus = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, corpus);
}

#[test]
fn support_export_validates_and_excludes_prose() {
    let corpus = seeded_classroom_role_corpus();
    let export = classroom_role_support_export(
        "support-export:presentation-classroom-role:001",
        "2026-06-21T00:00:00Z",
        &corpus,
    );
    assert!(export.validate().is_empty(), "{:?}", export.validate());
    assert!(export.no_role_grants_control_authority);
    assert!(export.no_authority_granted_by_classroom_role);
    assert!(export.all_constrained_clients_join_safely);
    assert!(export.all_packets_authority_bounded);
    assert!(export.raw_instructional_prose_excluded);
    assert_eq!(export.member_rows.len() as u32, corpus.summary.member_count);
    assert_eq!(export.packet_rows.len() as u32, corpus.summary.packet_count);

    // No packet title, action label, command id, target ref, or authority grant
    // ref ever leaks into the support export.
    let export_json = serde_json::to_string(&export).unwrap();
    assert!(!export_json.contains("\"title\""));
    assert!(!export_json.contains("\"label\""));
    assert!(!export_json.contains("\"command_id\""));
    assert!(!export_json.contains("\"target_ref\""));
    assert!(!export_json.contains("\"external_authority_ref\""));
    for profile in corpus.all_profiles() {
        for packet in &profile.exercise_packets {
            assert!(!export_json.contains(&packet.title));
        }
    }
}

#[test]
fn support_export_flags_a_role_claiming_authority() {
    let corpus = seeded_classroom_role_corpus();
    let mut export = classroom_role_support_export("x", "t", &corpus);
    export.member_rows[0].role_grants_terminal_or_debug_control = true;
    let violations = export.validate();
    assert!(violations.contains(&ClassroomSupportViolation::RowClaimsRoleAuthority));
}

#[test]
fn checked_in_fixtures_match_the_seed_projection() {
    let corpus = seeded_classroom_role_corpus();
    let fixture = include_str!(
        "../../../../../fixtures/presentation/classroom-role-and-authority/classroom-role-and-authority-corpus.json"
    );
    let parsed: ClassroomRoleCorpus = serde_json::from_str(fixture).expect("fixture parses");
    assert_eq!(
        parsed, corpus,
        "fixtures/presentation/classroom-role-and-authority drifted from the seed corpus; \
         regenerate with the dump_presentation_classroom_roles example"
    );
}

#[test]
fn checked_in_example_artifacts_match_the_seed() {
    let profile_fixture =
        include_str!("../../../../../artifacts/presentation/classroom-role.example.json");
    let parsed_profile: super::roles::ClassroomProfile =
        serde_json::from_str(profile_fixture).expect("profile example parses");
    assert_eq!(parsed_profile, classroom_role_example());

    let packet_fixture =
        include_str!("../../../../../artifacts/presentation/exercise-packet.example.json");
    let parsed_packet: ExercisePacket =
        serde_json::from_str(packet_fixture).expect("packet example parses");
    assert_eq!(parsed_packet, exercise_packet_example());
}
