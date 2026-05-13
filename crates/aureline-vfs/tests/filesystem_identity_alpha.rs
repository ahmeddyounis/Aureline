use std::path::Path;

use aureline_vfs::save::open_save_target;
use aureline_vfs::{
    compare_external_change, derive_path_truth_chip, filesystem_identity_reference_set,
    inspect_aliases, review_save_target, Alias, AliasKind, CapabilityFlags, CaseSensitivity,
    HookCounters, NormalizationForm, PermissionSnapshot, RootClass, SaveOutcome,
    StrongestIdentityTokenKind, SymlinkEscapePolicy, SyntheticRootBuilder, TrustState, VfsUri,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AlphaFixture {
    schema_version: u32,
    case_id: String,
    input: AlphaInput,
    expected: AlphaExpected,
}

#[derive(Debug, Deserialize)]
struct AlphaInput {
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
    aliases: Vec<AlphaAlias>,
    presentation_alias_kind: Option<String>,
    presentation_resolution_chain: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AlphaAlias {
    alias_uri: String,
    alias_kind: String,
    resolution_chain: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AlphaExpected {
    filesystem_identity_ref: String,
    shared_flow_refs: SharedFlowRefs,
    path_truth_class: String,
    alias_count: usize,
    distinct_alias_kinds: Vec<String>,
    writes_to_canonical_uri: String,
    save_token_kind: String,
    save_token_value: String,
    compare_outcome: String,
    blocking_save_outcome: Option<String>,
    observed_generation_token_value: Option<String>,
    review_required: bool,
    silent_overwrite_forbidden: bool,
    diff_availability: String,
    diff_hunks: u32,
    external_line_changes: u32,
    local_line_changes: u32,
    resolution_actions: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SharedFlowRefs {
    editor_file_identity_ref: String,
    git_file_identity_ref: String,
    restore_file_identity_ref: String,
    mutation_file_identity_ref: String,
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

fn capability_flags(input: &AlphaInput) -> CapabilityFlags {
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

fn build_root(input: &AlphaInput) -> aureline_vfs::SyntheticRoot {
    let aliases: Vec<Alias> = input
        .aliases
        .iter()
        .map(|alias| Alias {
            alias_uri: VfsUri::parse(alias.alias_uri.clone()).unwrap(),
            alias_kind: parse_alias_kind(&alias.alias_kind),
            resolution_chain: alias.resolution_chain.clone(),
        })
        .collect();

    let mut builder = SyntheticRootBuilder::new(
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
    );

    if input.presentation_uri != input.canonical_uri {
        builder = builder.add_presentation(
            input.canonical_uri.clone(),
            input.display_label.clone(),
            input.canonical_uri.clone(),
            None,
            vec!["-> canonical".to_owned()],
        );
    }

    builder.build()
}

#[test]
fn filesystem_identity_alpha_fixtures_project_same_truth() {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/filesystem_identity_alpha");
    let mut count = 0usize;

    for entry in std::fs::read_dir(&root_dir).expect("fixture directory must exist") {
        let entry = entry.expect("fixture directory entry must read");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }

        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let fixture: AlphaFixture = serde_json::from_str(&payload).expect("fixture must parse");
        assert_eq!(fixture.schema_version, 1, "{}", fixture.case_id);

        let mut root = build_root(&fixture.input);
        let presentation_uri = VfsUri::parse(fixture.input.presentation_uri.clone())
            .expect("presentation uri must parse");
        let mut counters = HookCounters::default();
        let token = open_save_target(
            &root,
            &presentation_uri,
            fixture.input.observed_at.clone(),
            &mut counters,
        )
        .expect("open_save_target must succeed");

        if let Some(external_content) = fixture.input.external_content_after_open.clone() {
            root.apply_commit(&fixture.input.canonical_uri, external_content.into_bytes())
                .expect("external change target must exist");
        }

        let chip = derive_path_truth_chip(&token.identity);
        let aliases = inspect_aliases(&token.identity);
        let save_review = review_save_target(&token);
        let refs = filesystem_identity_reference_set(&token.identity);
        let compare = compare_external_change(
            &root,
            &token,
            fixture.input.local_content.as_bytes(),
            &mut counters,
        );

        assert_eq!(
            refs.filesystem_identity_ref, fixture.expected.filesystem_identity_ref,
            "{}",
            fixture.case_id
        );
        assert_eq!(
            refs.editor_file_identity_ref,
            fixture.expected.shared_flow_refs.editor_file_identity_ref
        );
        assert_eq!(
            refs.git_file_identity_ref,
            fixture.expected.shared_flow_refs.git_file_identity_ref
        );
        assert_eq!(
            refs.restore_file_identity_ref,
            fixture.expected.shared_flow_refs.restore_file_identity_ref
        );
        assert_eq!(
            refs.mutation_file_identity_ref,
            fixture.expected.shared_flow_refs.mutation_file_identity_ref
        );
        assert!(refs.all_flows_share_identity(), "{}", fixture.case_id);

        assert_eq!(chip.class.as_str(), fixture.expected.path_truth_class);
        assert_eq!(chip.alias_count, fixture.expected.alias_count);
        assert_eq!(
            aliases
                .distinct_alias_kinds
                .iter()
                .map(|kind| kind.as_str().to_owned())
                .collect::<Vec<_>>(),
            fixture.expected.distinct_alias_kinds
        );
        assert_eq!(
            save_review.writes_to_canonical_uri.as_str(),
            fixture.expected.writes_to_canonical_uri
        );
        assert_eq!(
            save_review.pinned_generation_token_kind.as_str(),
            fixture.expected.save_token_kind
        );
        assert_eq!(
            save_review.pinned_generation_token_value,
            fixture.expected.save_token_value
        );

        assert_eq!(compare.outcome.as_str(), fixture.expected.compare_outcome);
        assert_eq!(
            compare
                .blocking_save_outcome
                .map(SaveOutcome::as_str)
                .map(str::to_owned),
            fixture.expected.blocking_save_outcome
        );
        assert_eq!(
            compare.observed_generation_token_value,
            fixture.expected.observed_generation_token_value
        );
        assert_eq!(compare.review_required, fixture.expected.review_required);
        assert_eq!(
            compare.silent_overwrite_forbidden,
            fixture.expected.silent_overwrite_forbidden
        );
        assert_eq!(
            compare.diff.availability.as_str(),
            fixture.expected.diff_availability
        );
        assert_eq!(compare.diff.changed_hunk_count, fixture.expected.diff_hunks);
        assert_eq!(
            compare.diff.external_line_change_count,
            fixture.expected.external_line_changes
        );
        assert_eq!(
            compare.diff.local_line_change_count,
            fixture.expected.local_line_changes
        );
        assert_eq!(
            compare
                .resolution_actions
                .iter()
                .map(|action| action.as_str().to_owned())
                .collect::<Vec<_>>(),
            fixture.expected.resolution_actions
        );

        assert_eq!(
            token
                .identity
                .canonical_filesystem_object
                .strongest_identity_token
                .kind,
            StrongestIdentityTokenKind::DeviceInodeGeneration,
            "{} must use exact identity token for the alpha local fixtures",
            fixture.case_id
        );

        count += 1;
    }

    assert!(count >= 3, "expected at least three alpha fixtures");
}
