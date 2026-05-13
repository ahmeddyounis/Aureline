//! Fixture-driven coverage for Git/VFS conflict handoff packets.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use aureline_git::{GitConflictHandoffRequest, GitConflictHandoffService, SystemGitStatusBackend};
use aureline_vfs::save::open_save_target;
use aureline_vfs::{
    compare_external_change, Alias, AliasKind, CapabilityFlags, CaseSensitivity, HookCounters,
    NormalizationForm, PermissionSnapshot, RootClass, SymlinkEscapePolicy, SyntheticRootBuilder,
    TrustState, VfsUri,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ConflictHandoffFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    setup_mode: String,
    path: PathBuf,
    vfs: Option<VfsInput>,
    expected: ExpectedHandoff,
}

#[derive(Debug, Deserialize)]
struct VfsInput {
    workspace_id: String,
    root_id: String,
    root_class: String,
    trust_state: String,
    presentation_uri: String,
    canonical_uri: String,
    logical_uri: String,
    display_label: String,
    normalization_form: String,
    strongest_token_base: String,
    initial_generation: u64,
    initial_content: String,
    local_content: String,
    external_content_after_open: Option<String>,
    permission_writable: bool,
    permission_mode: String,
    permission_owner: Option<String>,
    permission_group: Option<String>,
    read_only: bool,
    policy_constrained: bool,
    review_required_before_save: bool,
    review_required_before_rename: bool,
    case_sensitivity: String,
    symlink_escape_policy: String,
    observed_at: String,
    aliases: Vec<VfsAlias>,
    presentation_alias_kind: Option<String>,
    presentation_resolution_chain: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct VfsAlias {
    alias_uri: String,
    alias_kind: String,
    resolution_chain: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedHandoff {
    divergence_source: String,
    editor_surface_state: String,
    git_surface_state: String,
    filesystem_identity_ref: Option<String>,
    min_unresolved_count: u32,
    external_compare_outcome: Option<String>,
    diff_availability: Option<String>,
    rollback_path_class: String,
    rollback_available: bool,
    no_write_committed: bool,
    safe_actions: Vec<String>,
    support_omits: Vec<String>,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/git/conflict_handoff_alpha")
}

fn run_git(root: &Path, args: &[&str]) {
    let status = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .status()
        .expect("git command launches");
    assert!(
        status.success(),
        "git {args:?} failed in {}",
        root.display()
    );
}

fn run_git_expect_failure(root: &Path, args: &[&str]) {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .expect("git command launches");
    assert!(
        !output.status.success(),
        "git {args:?} unexpectedly succeeded in {}",
        root.display()
    );
}

fn init_repo(root: &Path) {
    let status = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["init", "-q", "-b", "main"])
        .status()
        .expect("git init launches");
    if !status.success() {
        run_git(root, &["init", "-q"]);
        run_git(root, &["checkout", "-q", "-b", "main"]);
    }
    run_git(root, &["config", "user.email", "fixture@example.invalid"]);
    run_git(root, &["config", "user.name", "Fixture"]);
}

fn build_git_conflict_root() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    init_repo(dir.path());
    fs::create_dir_all(dir.path().join("src")).expect("create src dir");
    fs::write(
        dir.path().join("src/lib.rs"),
        "pub fn answer() -> u32 {\n    1\n}\n",
    )
    .expect("write base source");
    run_git(dir.path(), &["add", "src/lib.rs"]);
    run_git(dir.path(), &["commit", "-q", "-m", "base"]);
    run_git(dir.path(), &["checkout", "-q", "-b", "incoming"]);
    fs::write(
        dir.path().join("src/lib.rs"),
        "pub fn answer() -> u32 {\n    2\n}\n",
    )
    .expect("write incoming source");
    run_git(dir.path(), &["commit", "-q", "-am", "incoming"]);
    run_git(dir.path(), &["checkout", "-q", "main"]);
    fs::write(
        dir.path().join("src/lib.rs"),
        "pub fn answer() -> u32 {\n    3\n}\n",
    )
    .expect("write current source");
    run_git(dir.path(), &["commit", "-q", "-am", "current"]);
    run_git_expect_failure(dir.path(), &["merge", "incoming"]);
    dir
}

fn parse_root_class(value: &str) -> RootClass {
    match value {
        "local_posix_like" => RootClass::LocalPosixLike,
        "local_windows_like" => RootClass::LocalWindowsLike,
        "remote_agent_mount" => RootClass::RemoteAgentMount,
        "container_mount" => RootClass::ContainerMount,
        "virtual_generated_document" => RootClass::VirtualGeneratedDocument,
        "archive_like_view" => RootClass::ArchiveLikeView,
        other => panic!("unsupported root_class in fixture: {other}"),
    }
}

fn parse_trust_state(value: &str) -> TrustState {
    match value {
        "trusted" => TrustState::Trusted,
        "restricted" => TrustState::Restricted,
        "pending_evaluation" => TrustState::PendingEvaluation,
        other => panic!("unsupported trust_state in fixture: {other}"),
    }
}

fn parse_normalization_form(value: &str) -> NormalizationForm {
    match value {
        "none" => NormalizationForm::None,
        "nfc" => NormalizationForm::Nfc,
        "nfd" => NormalizationForm::Nfd,
        "mixed_observed" => NormalizationForm::MixedObserved,
        other => panic!("unsupported normalization_form: {other}"),
    }
}

fn parse_case_sensitivity(value: &str) -> CaseSensitivity {
    match value {
        "sensitive" => CaseSensitivity::Sensitive,
        "insensitive_preserving" => CaseSensitivity::InsensitivePreserving,
        "insensitive_non_preserving" => CaseSensitivity::InsensitiveNonPreserving,
        other => panic!("unsupported case_sensitivity: {other}"),
    }
}

fn parse_symlink_escape_policy(value: &str) -> SymlinkEscapePolicy {
    match value {
        "allow" => SymlinkEscapePolicy::Allow,
        "warn" => SymlinkEscapePolicy::Warn,
        "block" => SymlinkEscapePolicy::Block,
        other => panic!("unsupported symlink_escape_policy: {other}"),
    }
}

fn parse_alias_kind(value: &str) -> AliasKind {
    match value {
        "symlink" => AliasKind::Symlink,
        "junction" => AliasKind::Junction,
        "hardlink_sibling" => AliasKind::HardlinkSibling,
        "case_only_variant" => AliasKind::CaseOnlyVariant,
        "unicode_normalization_variant" => AliasKind::UnicodeNormalizationVariant,
        "remote_alias" => AliasKind::RemoteAlias,
        "bind_mount_alias" => AliasKind::BindMountAlias,
        "container_mount_alias" => AliasKind::ContainerMountAlias,
        "archive_inner_alias" => AliasKind::ArchiveInnerAlias,
        other => panic!("unsupported alias_kind: {other}"),
    }
}

fn capability_flags(input: &VfsInput) -> CapabilityFlags {
    CapabilityFlags {
        supports_atomic_replace: !input.read_only,
        supports_in_place_write: !input.read_only,
        supports_conditional_remote_write: false,
        case_sensitivity: parse_case_sensitivity(&input.case_sensitivity),
        unicode_normalization: parse_normalization_form(&input.normalization_form),
        supports_case_only_rename: true,
        supports_unicode_normalization_rename: true,
        symlink_escape_policy: parse_symlink_escape_policy(&input.symlink_escape_policy),
        read_only: input.read_only,
        policy_constrained: input.policy_constrained,
        review_required_before_save: input.review_required_before_save,
        review_required_before_rename: input.review_required_before_rename,
        remote_container_adaptation: false,
    }
}

fn build_vfs_root(input: &VfsInput) -> aureline_vfs::SyntheticRoot {
    let aliases: Vec<Alias> = input
        .aliases
        .iter()
        .map(|alias| Alias {
            alias_uri: VfsUri::parse(alias.alias_uri.clone()).unwrap(),
            alias_kind: parse_alias_kind(&alias.alias_kind),
            resolution_chain: alias.resolution_chain.clone(),
        })
        .collect();

    SyntheticRootBuilder::new(
        input.root_id.clone(),
        parse_root_class(&input.root_class),
        capability_flags(input),
    )
    .with_workspace_id(input.workspace_id.clone())
    .with_trust_state(parse_trust_state(&input.trust_state))
    .add_canonical_object(
        input.canonical_uri.clone(),
        input.logical_uri.clone(),
        parse_normalization_form(&input.normalization_form),
        input.strongest_token_base.clone(),
        input.initial_generation,
        vec![],
        PermissionSnapshot {
            writable: input.permission_writable,
            mode: input.permission_mode.clone(),
            owner: input.permission_owner.clone(),
            group: input.permission_group.clone(),
            acl_summary: None,
        },
        aliases,
        input.initial_content.clone().into_bytes(),
    )
    .add_presentation(
        input.presentation_uri.clone(),
        input.display_label.clone(),
        input.canonical_uri.clone(),
        input
            .presentation_alias_kind
            .as_deref()
            .map(parse_alias_kind),
        input.presentation_resolution_chain.clone(),
    )
    .build()
}

fn assert_packet_matches(
    packet: &aureline_git::GitConflictHandoffPacket,
    fixture: &ConflictHandoffFixture,
) {
    assert_eq!(packet.record_kind, "git_conflict_handoff_packet");
    assert_eq!(packet.schema_version, 1);
    assert_eq!(
        packet.divergence_source.as_str(),
        fixture.expected.divergence_source,
        "{}: divergence source",
        fixture.case_name
    );
    assert_eq!(
        packet.editor_surface.state_class.as_str(),
        fixture.expected.editor_surface_state,
        "{}: editor surface state",
        fixture.case_name
    );
    assert_eq!(
        packet.git_surface.state_class.as_str(),
        fixture.expected.git_surface_state,
        "{}: git surface state",
        fixture.case_name
    );
    assert_eq!(
        packet.path_identity.filesystem_identity_ref, fixture.expected.filesystem_identity_ref,
        "{}: filesystem identity",
        fixture.case_name
    );
    assert!(
        packet.git_state.unresolved_count >= fixture.expected.min_unresolved_count,
        "{}: unresolved count",
        fixture.case_name
    );
    assert_eq!(
        packet.rollback_checkpoint.rollback_path_class, fixture.expected.rollback_path_class,
        "{}: rollback class",
        fixture.case_name
    );
    assert_eq!(
        packet.rollback_checkpoint.rollback_available, fixture.expected.rollback_available,
        "{}: rollback availability",
        fixture.case_name
    );
    assert_eq!(
        packet.rollback_checkpoint.no_write_committed, fixture.expected.no_write_committed,
        "{}: no-write state",
        fixture.case_name
    );
    assert!(
        packet.conflict_state_visible_in_editor_and_git(),
        "{}: state visible in both surfaces",
        fixture.case_name
    );
    assert!(
        packet.preserves_identity_and_recovery(),
        "{}: identity and recovery posture",
        fixture.case_name
    );

    let safe_actions = packet
        .safe_action_tokens()
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    for expected in &fixture.expected.safe_actions {
        assert!(
            safe_actions.contains(expected),
            "{}: missing safe action {expected}",
            fixture.case_name
        );
    }
    for omitted in &fixture.expected.support_omits {
        assert!(
            packet.support_export.omitted_fields.contains(omitted),
            "{}: missing omitted field {omitted}",
            fixture.case_name
        );
    }
    if let Some(expected) = &fixture.expected.external_compare_outcome {
        let external = packet
            .external_compare
            .as_ref()
            .expect("external compare projection exists");
        assert_eq!(&external.compare_outcome, expected);
        assert_eq!(
            Some(external.diff_availability.as_str()),
            fixture.expected.diff_availability.as_deref()
        );
    }
}

fn run_fixture(path: &Path) {
    let text = fs::read_to_string(path).expect("read fixture");
    let fixture: ConflictHandoffFixture = serde_yaml::from_str(&text).expect("parse fixture");
    assert_eq!(fixture.record_kind, "git_conflict_handoff_alpha_case");
    assert_eq!(fixture.schema_version, 1);

    match fixture.setup_mode.as_str() {
        "git_merge_conflict" => {
            let dir = build_git_conflict_root();
            let request = GitConflictHandoffRequest::with_observed_at(
                format!("workspace.fixture.{}", fixture.case_name),
                dir.path(),
                fixture.path.clone(),
                "2026-05-13T00:00:00Z",
            );
            let packet = GitConflictHandoffService::default().preview_git_conflict(&request);
            assert_packet_matches(&packet, &fixture);
        }
        "vfs_external_change" => {
            let input = fixture.vfs.as_ref().expect("vfs input required");
            let mut root = build_vfs_root(input);
            let presentation_uri =
                VfsUri::parse(input.presentation_uri.clone()).expect("presentation uri parses");
            let mut counters = HookCounters::default();
            let token = open_save_target(
                &root,
                &presentation_uri,
                input.observed_at.clone(),
                &mut counters,
            )
            .expect("open save target");
            if let Some(external_content) = input.external_content_after_open.clone() {
                root.apply_commit(&input.canonical_uri, external_content.into_bytes())
                    .expect("external change target exists");
            }
            let compare = compare_external_change(
                &root,
                &token,
                input.local_content.as_bytes(),
                &mut counters,
            );
            let packet =
                GitConflictHandoffService::<SystemGitStatusBackend>::from_external_change_compare(
                    aureline_git::GitExternalChangeHandoffInput {
                        workspace_ref: &input.workspace_id,
                        repo_root: Path::new("/alpha/project"),
                        repo_relative_path: &fixture.path,
                        generated_at: "2026-05-13T00:00:00Z",
                        compare_record: &compare,
                        git_truth_source_ref: Some("git.status.snapshot.external.compare"),
                        rollback_checkpoint_ref: None,
                    },
                );
            assert_packet_matches(&packet, &fixture);
        }
        other => panic!("unsupported setup_mode: {other}"),
    }
}

#[test]
fn protected_conflict_handoff_fixtures_match_contract() {
    for entry in fs::read_dir(fixtures_dir()).expect("fixture dir") {
        let path = entry.expect("fixture entry").path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("yaml") {
            run_fixture(&path);
        }
    }
}
