//! Unit and fixture-driven tests for the bounded M1 AI evidence-packet
//! seed and route/spend truth-strip wedge.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_ai::{
    AttachmentKind, AttachmentStatusClass, ComposerAttachment, ComposerDraft, ComposerMention,
    MentionKind, MentionResolutionState, RoutePlaceholder, SelectionReasonClass, SourceClass,
    TrustPosture,
};
use serde::Deserialize;

use super::*;

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ai/m1_evidence_and_spend_seed_cases"
);

fn baseline_draft() -> ComposerDraft {
    let mut draft = ComposerDraft::new(
        "draft.test",
        "session.test",
        "request_workspace.test",
        "Explain editor.find",
    );
    draft.add_mention(ComposerMention {
        mention_id: "mention.editor_find_symbol".to_owned(),
        kind: MentionKind::SymbolMention,
        target_stable_id: Some("cmd:editor.find".to_owned()),
        display_label: "@editor.find".to_owned(),
        resolution_state: MentionResolutionState::Resolved,
    });
    draft.add_attachment(ComposerAttachment {
        attachment_id: "att.workspace_slice".to_owned(),
        kind: AttachmentKind::WorkspaceSliceBundle,
        source_class: SourceClass::WorkspaceFileSlice,
        trust_posture: TrustPosture::TrustedFirstParty,
        selection_reason: SelectionReasonClass::UserPinned,
        status: AttachmentStatusClass::Live,
        estimated_byte_size: 1024,
        display_label: "src/editor/find.rs slice".to_owned(),
        scope_truth: None,
        placed_under_fenced_role: false,
    });
    draft
}

fn baseline_inputs() -> (String, String, String) {
    (
        "evidence_packet.test.local_walk".to_owned(),
        "build-id:aureline:dev:0.0.0:aarch64-apple-darwin:debug:0000abcd".to_owned(),
        "mono:0".to_owned(),
    )
}

fn project_with_default(draft: &ComposerDraft) -> AiTruthStripSnapshot {
    let (packet_id, build_id, minted_at) = baseline_inputs();
    AiTruthStripSnapshot::project(
        draft,
        &AiRouteSpendPosture::m1_seed_default(),
        AiTruthStripInputs {
            evidence_packet_id: &packet_id,
            exact_build_identity_ref: &build_id,
            minted_at: &minted_at,
        },
    )
}

#[test]
fn protected_walk_local_no_dispatch_renders_clean_packet() {
    let draft = baseline_draft();
    let snapshot = project_with_default(&draft);

    assert_eq!(snapshot.record_kind, AI_TRUTH_STRIP_SNAPSHOT_RECORD_KIND);
    assert_eq!(snapshot.schema_version, AI_EVIDENCE_SEED_SCHEMA_VERSION);
    assert_eq!(
        snapshot.prototype_label_token,
        PrototypeLabel::M1PrototypeEvidenceAndSpendSeed.as_str()
    );

    let packet = &snapshot.evidence_packet;
    assert_eq!(packet.record_kind, AI_EVIDENCE_SEED_RECORD_KIND);
    assert_eq!(packet.evidence_packet_id, "evidence_packet.test.local_walk");
    assert_eq!(packet.composer_draft_ref, "draft.test");
    assert_eq!(packet.composer_session_ref, "session.test");
    assert_eq!(packet.request_workspace_ref, "request_workspace.test");
    assert_eq!(
        packet.run_state_class,
        AiRunStateClass::DispatchDisabledInM1Seed
    );
    assert_eq!(packet.provider_class, "disabled_no_provider_in_m1_seed");
    assert_eq!(packet.route_path_class, "denied_by_policy_in_m1_seed");
    assert_eq!(
        packet.dispatch_target_class,
        "disabled_no_dispatch_in_m1_seed"
    );
    assert_eq!(
        packet.local_or_remote_path_class,
        LocalOrRemotePathClass::LocalNoDispatch
    );
    assert_eq!(
        packet.spend_posture_class,
        SpendPostureClass::NoSpendInM1Seed
    );

    assert!(!snapshot.has_invariant_violations);
    assert!(snapshot.invariant_violations.is_empty());
}

#[test]
fn truth_strip_renders_canonical_row_order() {
    let draft = baseline_draft();
    let snapshot = project_with_default(&draft);
    let row_ids: Vec<_> = snapshot
        .truth_strip_rows
        .iter()
        .map(|row| row.row_id.as_str())
        .collect();
    assert_eq!(
        row_ids,
        vec![
            "provider",
            "route",
            "dispatch_target",
            "local_or_remote_path",
            "spend_posture",
            "run_state",
            "context_summary",
            "build_identity",
        ]
    );
}

#[test]
fn claim_limits_render_in_canonical_order() {
    let draft = baseline_draft();
    let snapshot = project_with_default(&draft);
    assert_eq!(
        snapshot.evidence_packet.claim_limits,
        vec![
            AiTruthStripClaimLimit::SingleBoundedWedgeOnly,
            AiTruthStripClaimLimit::NoLiveModelDispatch,
            AiTruthStripClaimLimit::NoBillingOrQuotaTracking,
            AiTruthStripClaimLimit::NoRawSecretsOrProviderUrls,
        ]
    );
}

#[test]
fn result_lineage_carries_one_row_per_block_reason() {
    let draft = baseline_draft();
    let snapshot = project_with_default(&draft);
    // The protected walk only emits the always-on policy_blocked_route
    // honesty marker.
    assert_eq!(snapshot.evidence_packet.result_lineage.len(), 1);
    let row = &snapshot.evidence_packet.result_lineage[0];
    assert_eq!(row.block_reason_token, "policy_blocked_route");
    assert_eq!(row.addressable_target_token, "route:placeholder");
}

#[test]
fn unresolved_mention_routes_into_packet_lineage() {
    let mut draft = baseline_draft();
    draft.add_mention(ComposerMention {
        mention_id: "mention.missing".to_owned(),
        kind: MentionKind::FileMention,
        target_stable_id: None,
        display_label: "@missing.rs".to_owned(),
        resolution_state: MentionResolutionState::UnresolvedNotFound,
    });
    let snapshot = project_with_default(&draft);
    let lineage = &snapshot.evidence_packet.result_lineage;
    assert!(lineage
        .iter()
        .any(|row| row.block_reason_token == "unresolved_mention"
            && row.addressable_target_token == "mention:mention.missing"));
    // A draft with an actionable block must surface as
    // BlockedPendingResolution on the run-state row.
    assert_eq!(
        snapshot.evidence_packet.run_state_class,
        AiRunStateClass::BlockedPendingResolution
    );
}

#[test]
fn tainted_attachment_routes_into_packet_lineage_and_counts() {
    let mut draft = baseline_draft();
    draft.add_attachment(ComposerAttachment {
        attachment_id: "att.tainted_user_paste".to_owned(),
        kind: AttachmentKind::UserSuppliedText,
        source_class: SourceClass::UserSuppliedText,
        trust_posture: TrustPosture::UntrustedUserSupplied,
        selection_reason: SelectionReasonClass::UserPastedFreeformText,
        status: AttachmentStatusClass::TaintedOutsideFencedSection,
        estimated_byte_size: 512,
        display_label: "Pasted instructions from external chat".to_owned(),
        scope_truth: None,
        placed_under_fenced_role: false,
    });
    let snapshot = project_with_default(&draft);
    let lineage = &snapshot.evidence_packet.result_lineage;
    assert!(lineage.iter().any(|row| row.block_reason_token
        == "tainted_attachment_outside_fenced_section"
        && row.addressable_target_token == "attachment:att.tainted_user_paste"));
    let summary = &snapshot.evidence_packet.context_summary;
    assert_eq!(summary.attachment_count, 2);
    assert_eq!(summary.trusted_attachment_count, 1);
    assert_eq!(summary.tainted_attachment_count, 1);
    assert_eq!(summary.fenced_attachment_count, 0);
}

#[test]
fn failure_drill_alternate_route_surfaces_distinct_tokens() {
    // The drill matches the spec's "Route an AI run through a different
    // provider or spend class and confirm the strip/evidence packet
    // exposes the exact route and cost posture" requirement.
    let mut draft = baseline_draft();
    // Use the mocked-for-fixtures route placeholder so the upstream
    // provider / route tokens flip away from the live defaults.
    draft.route_placeholder = RoutePlaceholder::mocked_for_fixtures();
    let (packet_id, build_id, minted_at) = baseline_inputs();
    let snapshot = AiTruthStripSnapshot::project(
        &draft,
        &AiRouteSpendPosture::mocked_alternative_for_failure_drill(),
        AiTruthStripInputs {
            evidence_packet_id: &packet_id,
            exact_build_identity_ref: &build_id,
            minted_at: &minted_at,
        },
    );
    let packet = &snapshot.evidence_packet;
    assert_eq!(packet.provider_class, "mocked_test_provider");
    assert_eq!(packet.route_path_class, "offline_cached_only");
    assert_eq!(
        packet.local_or_remote_path_class,
        LocalOrRemotePathClass::RemoteMockedForFixtures
    );
    assert_eq!(
        packet.spend_posture_class,
        SpendPostureClass::MockedSpendForFixtures
    );
    assert_eq!(
        packet.run_state_class,
        AiRunStateClass::PreviewPreDispatchMocked
    );
    // The strip MUST quote the alternative tokens verbatim — that's how
    // the user sees "this run went somewhere different".
    let provider_row = snapshot.strip_row("provider").expect("provider row");
    assert_eq!(provider_row.value_token, "mocked_test_provider");
    let path_row = snapshot
        .strip_row("local_or_remote_path")
        .expect("path row");
    assert_eq!(path_row.value_token, "remote_mocked_for_fixtures");
    let spend_row = snapshot.strip_row("spend_posture").expect("spend row");
    assert_eq!(spend_row.value_token, "mocked_spend_for_fixtures");

    assert!(!snapshot.has_invariant_violations);
}

#[test]
fn mocked_spend_against_live_run_state_surfaces_typed_invariant() {
    // A buggy caller swaps in the mocked spend posture but leaves the
    // run-state class at the live default. The wedge MUST surface
    // SpendPostureContradictsRoute and (because the mocked path is also
    // set) RouteAndPathClassDisagree rather than minting a clean packet.
    let draft = baseline_draft();
    let (packet_id, build_id, minted_at) = baseline_inputs();
    let posture = AiRouteSpendPosture {
        local_or_remote_path_class: LocalOrRemotePathClass::RemoteMockedForFixtures,
        spend_posture_class: SpendPostureClass::MockedSpendForFixtures,
        run_state_class: AiRunStateClass::DispatchDisabledInM1Seed,
    };
    let snapshot = AiTruthStripSnapshot::project(
        &draft,
        &posture,
        AiTruthStripInputs {
            evidence_packet_id: &packet_id,
            exact_build_identity_ref: &build_id,
            minted_at: &minted_at,
        },
    );
    assert!(snapshot.has_invariant_violations);
    assert!(snapshot
        .invariant_violations
        .contains(&AiTruthStripInvariantViolation::RouteAndPathClassDisagree));
    assert!(snapshot
        .invariant_violations
        .contains(&AiTruthStripInvariantViolation::SpendPostureContradictsRoute));
}

#[test]
fn missing_evidence_packet_id_surfaces_typed_invariant() {
    let draft = baseline_draft();
    let snapshot = AiTruthStripSnapshot::project(
        &draft,
        &AiRouteSpendPosture::m1_seed_default(),
        AiTruthStripInputs {
            evidence_packet_id: "",
            exact_build_identity_ref: "build-id:aureline:dev:0.0.0:host:debug:abcd",
            minted_at: "mono:0",
        },
    );
    assert!(snapshot
        .invariant_violations
        .contains(&AiTruthStripInvariantViolation::EvidencePacketIdMissing));
}

#[test]
fn missing_exact_build_identity_ref_surfaces_typed_invariant() {
    let draft = baseline_draft();
    let snapshot = AiTruthStripSnapshot::project(
        &draft,
        &AiRouteSpendPosture::m1_seed_default(),
        AiTruthStripInputs {
            evidence_packet_id: "evidence_packet.x",
            exact_build_identity_ref: "",
            minted_at: "mono:0",
        },
    );
    assert!(snapshot
        .invariant_violations
        .contains(&AiTruthStripInvariantViolation::ExactBuildIdentityRefMissing));
}

#[test]
fn export_safe_run_metadata_omits_raw_intent_and_carries_typed_tokens() {
    let draft = baseline_draft();
    let snapshot = project_with_default(&draft);
    let exported = snapshot.export_safe_run_metadata();
    // The exported payload is the packet only. It MUST carry typed
    // tokens and MUST NOT carry the raw intent text or any raw
    // mention / attachment labels — those live on the upstream draft,
    // not on the packet.
    assert!(exported.contains("\"provider_class\": \"disabled_no_provider_in_m1_seed\""));
    assert!(exported.contains("\"local_or_remote_path_class\": \"local_no_dispatch\""));
    assert!(exported.contains("\"spend_posture_class\": \"no_spend_in_m1_seed\""));
    assert!(
        exported.contains("\"prototype_label_token\": \"m1_prototype_evidence_and_spend_seed\"")
    );
    assert!(!exported.contains("Explain editor.find"));
    assert!(!exported.contains("@editor.find"));
    assert!(!exported.contains("src/editor/find.rs slice"));
}

#[test]
fn render_plaintext_quotes_every_section_in_stable_order() {
    let draft = baseline_draft();
    let snapshot = project_with_default(&draft);
    let plaintext = snapshot.render_plaintext();
    let strip_idx = plaintext
        .find("[Route / spend truth strip]")
        .expect("strip heading");
    let lineage_idx = plaintext.find("[Result lineage]").expect("lineage heading");
    let limits_idx = plaintext.find("[Claim limits]").expect("limits heading");
    let invariants_idx = plaintext.find("[Invariants]").expect("invariants heading");
    assert!(strip_idx < lineage_idx);
    assert!(lineage_idx < limits_idx);
    assert!(limits_idx < invariants_idx);
    assert!(plaintext.contains("disabled_no_provider_in_m1_seed"));
    assert!(plaintext.contains("policy_blocked_route -> route:placeholder"));
    assert!(plaintext.contains("(all clear)"));
}

#[test]
fn snapshot_round_trips_through_serde_json() {
    let draft = baseline_draft();
    let snapshot = project_with_default(&draft);
    let json = serde_json::to_string(&snapshot).expect("serialise");
    let parsed: AiTruthStripSnapshot = serde_json::from_str(&json).expect("round trip parses");
    assert_eq!(parsed, snapshot);
}

#[test]
fn project_launch_wedge_helper_matches_explicit_call() {
    let draft = baseline_draft();
    let (packet_id, build_id, minted_at) = baseline_inputs();
    let helper = project_launch_wedge(
        &draft,
        AiTruthStripInputs {
            evidence_packet_id: &packet_id,
            exact_build_identity_ref: &build_id,
            minted_at: &minted_at,
        },
    );
    let explicit = AiTruthStripSnapshot::project(
        &draft,
        &AiRouteSpendPosture::m1_seed_default(),
        AiTruthStripInputs {
            evidence_packet_id: &packet_id,
            exact_build_identity_ref: &build_id,
            minted_at: &minted_at,
        },
    );
    assert_eq!(helper, explicit);
}

// ---------------------------------------------------------------------------
// Fixture replays
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct FixtureCase {
    inputs: FixtureInputs,
    draft: FixtureDraft,
    expect: FixtureExpectation,
}

#[derive(Debug, Deserialize)]
struct FixtureInputs {
    evidence_packet_id: String,
    exact_build_identity_ref: String,
    minted_at: String,
    posture: String,
    #[serde(default)]
    use_mocked_route_placeholder: bool,
}

#[derive(Debug, Deserialize)]
struct FixtureDraft {
    composer_draft_id: String,
    composer_session_id: String,
    request_workspace_id: String,
    intent_text: String,
    #[serde(default)]
    mentions: Vec<FixtureMention>,
    #[serde(default)]
    attachments: Vec<FixtureAttachment>,
}

#[derive(Debug, Deserialize)]
struct FixtureMention {
    mention_id: String,
    kind: String,
    #[serde(default)]
    target_stable_id: Option<String>,
    display_label: String,
    resolution_state: String,
}

#[derive(Debug, Deserialize)]
struct FixtureAttachment {
    attachment_id: String,
    kind: String,
    source_class: String,
    trust_posture: String,
    selection_reason: String,
    status: String,
    estimated_byte_size: u64,
    display_label: String,
    #[serde(default)]
    scope_truth: Option<aureline_search::ScopeCandidateTruthRecord>,
    #[serde(default)]
    placed_under_fenced_role: bool,
}

#[derive(Debug, Deserialize)]
struct FixtureExpectation {
    run_state_class: String,
    provider_class: String,
    route_path_class: String,
    dispatch_target_class: String,
    local_or_remote_path_class: String,
    spend_posture_class: String,
    draft_state_token: String,
    has_invariant_violations: bool,
    result_lineage_tokens: Vec<FixtureLineageRow>,
    context_summary: FixtureContextSummary,
}

#[derive(Debug, Deserialize)]
struct FixtureLineageRow {
    block_reason_token: String,
    addressable_target_token: String,
}

#[derive(Debug, Deserialize)]
struct FixtureContextSummary {
    mention_count: u32,
    resolved_mention_count: u32,
    attachment_count: u32,
    trusted_attachment_count: u32,
    tainted_attachment_count: u32,
    fenced_attachment_count: u32,
    slash_command_count: u32,
    resolved_slash_command_count: u32,
    aggregate_byte_estimate: u64,
}

fn load_fixture(name: &str) -> FixtureCase {
    let path: PathBuf = Path::new(FIXTURE_DIR).join(name);
    let raw = fs::read_to_string(&path).expect("fixture file present");
    serde_json::from_str(&raw).expect("fixture parses")
}

fn parse_mention_kind(token: &str) -> MentionKind {
    match token {
        "symbol_mention" => MentionKind::SymbolMention,
        "file_mention" => MentionKind::FileMention,
        "workset_mention" => MentionKind::WorksetMention,
        "search_result_mention" => MentionKind::SearchResultMention,
        "docs_anchor_mention" => MentionKind::DocsAnchorMention,
        "diagnostic_mention" => MentionKind::DiagnosticMention,
        "execution_context_mention" => MentionKind::ExecutionContextMention,
        other => panic!("unknown mention kind in fixture: {other}"),
    }
}

fn parse_mention_resolution(token: &str) -> MentionResolutionState {
    match token {
        "resolved" => MentionResolutionState::Resolved,
        "unresolved_not_found" => MentionResolutionState::UnresolvedNotFound,
        "unresolved_scope_excluded" => MentionResolutionState::UnresolvedScopeExcluded,
        "unresolved_stale" => MentionResolutionState::UnresolvedStale,
        other => panic!("unknown mention resolution in fixture: {other}"),
    }
}

fn parse_attachment_kind(token: &str) -> AttachmentKind {
    match token {
        "user_supplied_text" => AttachmentKind::UserSuppliedText,
        "user_supplied_file" => AttachmentKind::UserSuppliedFile,
        "workspace_slice_bundle" => AttachmentKind::WorkspaceSliceBundle,
        "retrieved_document" => AttachmentKind::RetrievedDocument,
        "terminal_log_capture" => AttachmentKind::TerminalLogCapture,
        "citation_anchor_bundle" => AttachmentKind::CitationAnchorBundle,
        "diagnostic_bundle" => AttachmentKind::DiagnosticBundle,
        other => panic!("unknown attachment kind in fixture: {other}"),
    }
}

fn parse_source_class(token: &str) -> SourceClass {
    match token {
        "workspace_file_slice" => SourceClass::WorkspaceFileSlice,
        "workspace_symbol" => SourceClass::WorkspaceSymbol,
        "workspace_buffer_slice" => SourceClass::WorkspaceBufferSlice,
        "workspace_search_result" => SourceClass::WorkspaceSearchResult,
        "workspace_diagnostics" => SourceClass::WorkspaceDiagnostics,
        "docs_pack_excerpt" => SourceClass::DocsPackExcerpt,
        "terminal_transcript_excerpt" => SourceClass::TerminalTranscriptExcerpt,
        "user_supplied_text" => SourceClass::UserSuppliedText,
        "user_supplied_file" => SourceClass::UserSuppliedFile,
        "citation_anchor_quote" => SourceClass::CitationAnchorQuote,
        other => panic!("unknown source class in fixture: {other}"),
    }
}

fn parse_trust_posture(token: &str) -> TrustPosture {
    match token {
        "trusted_first_party" => TrustPosture::TrustedFirstParty,
        "reviewed_derived" => TrustPosture::ReviewedDerived,
        "unreviewed_derived" => TrustPosture::UnreviewedDerived,
        "untrusted_user_supplied" => TrustPosture::UntrustedUserSupplied,
        "untrusted_external" => TrustPosture::UntrustedExternal,
        "untrusted_extension_proposed" => TrustPosture::UntrustedExtensionProposed,
        "policy_quarantined" => TrustPosture::PolicyQuarantined,
        other => panic!("unknown trust posture in fixture: {other}"),
    }
}

fn parse_selection_reason(token: &str) -> SelectionReasonClass {
    match token {
        "user_pinned" => SelectionReasonClass::UserPinned,
        "mention_trail" => SelectionReasonClass::MentionTrail,
        "slash_command_requested" => SelectionReasonClass::SlashCommandRequested,
        "search_result_packet" => SelectionReasonClass::SearchResultPacket,
        "citation_anchor_excerpt" => SelectionReasonClass::CitationAnchorExcerpt,
        "diagnostic_context" => SelectionReasonClass::DiagnosticContext,
        "terminal_capture_attached" => SelectionReasonClass::TerminalCaptureAttached,
        "user_pasted_freeform_text" => SelectionReasonClass::UserPastedFreeformText,
        other => panic!("unknown selection reason in fixture: {other}"),
    }
}

fn parse_attachment_status(token: &str) -> AttachmentStatusClass {
    match token {
        "live" => AttachmentStatusClass::Live,
        "stale" => AttachmentStatusClass::Stale,
        "tainted_outside_fenced_section" => AttachmentStatusClass::TaintedOutsideFencedSection,
        "over_budget" => AttachmentStatusClass::OverBudget,
        "policy_blocked" => AttachmentStatusClass::PolicyBlocked,
        "out_of_scope" => AttachmentStatusClass::OutOfScope,
        other => panic!("unknown attachment status in fixture: {other}"),
    }
}

fn build_draft_from_fixture(fixture: &FixtureDraft, use_mocked_route: bool) -> ComposerDraft {
    let mut draft = ComposerDraft::new(
        fixture.composer_draft_id.clone(),
        fixture.composer_session_id.clone(),
        fixture.request_workspace_id.clone(),
        fixture.intent_text.clone(),
    );
    for mention in &fixture.mentions {
        draft.add_mention(ComposerMention {
            mention_id: mention.mention_id.clone(),
            kind: parse_mention_kind(&mention.kind),
            target_stable_id: mention.target_stable_id.clone(),
            display_label: mention.display_label.clone(),
            resolution_state: parse_mention_resolution(&mention.resolution_state),
        });
    }
    for attachment in &fixture.attachments {
        draft.add_attachment(ComposerAttachment {
            attachment_id: attachment.attachment_id.clone(),
            kind: parse_attachment_kind(&attachment.kind),
            source_class: parse_source_class(&attachment.source_class),
            trust_posture: parse_trust_posture(&attachment.trust_posture),
            selection_reason: parse_selection_reason(&attachment.selection_reason),
            status: parse_attachment_status(&attachment.status),
            estimated_byte_size: attachment.estimated_byte_size,
            display_label: attachment.display_label.clone(),
            scope_truth: attachment.scope_truth.clone(),
            placed_under_fenced_role: attachment.placed_under_fenced_role,
        });
    }
    if use_mocked_route {
        draft.route_placeholder = RoutePlaceholder::mocked_for_fixtures();
    }
    draft
}

fn parse_posture(token: &str) -> AiRouteSpendPosture {
    match token {
        "m1_seed_default" => AiRouteSpendPosture::m1_seed_default(),
        "mocked_alternative_for_failure_drill" => {
            AiRouteSpendPosture::mocked_alternative_for_failure_drill()
        }
        other => panic!("unknown posture token in fixture: {other}"),
    }
}

fn run_fixture(name: &str) -> (FixtureCase, AiTruthStripSnapshot) {
    let fixture = load_fixture(name);
    let draft =
        build_draft_from_fixture(&fixture.draft, fixture.inputs.use_mocked_route_placeholder);
    let posture = parse_posture(&fixture.inputs.posture);
    let snapshot = AiTruthStripSnapshot::project(
        &draft,
        &posture,
        AiTruthStripInputs {
            evidence_packet_id: &fixture.inputs.evidence_packet_id,
            exact_build_identity_ref: &fixture.inputs.exact_build_identity_ref,
            minted_at: &fixture.inputs.minted_at,
        },
    );
    (fixture, snapshot)
}

fn assert_fixture_matches(fixture: &FixtureCase, snapshot: &AiTruthStripSnapshot) {
    let packet = &snapshot.evidence_packet;
    assert_eq!(
        packet.run_state_class.as_str(),
        fixture.expect.run_state_class,
        "run_state_class"
    );
    assert_eq!(
        packet.provider_class, fixture.expect.provider_class,
        "provider_class"
    );
    assert_eq!(
        packet.route_path_class, fixture.expect.route_path_class,
        "route_path_class"
    );
    assert_eq!(
        packet.dispatch_target_class, fixture.expect.dispatch_target_class,
        "dispatch_target_class"
    );
    assert_eq!(
        packet.local_or_remote_path_class.as_str(),
        fixture.expect.local_or_remote_path_class,
        "local_or_remote_path_class"
    );
    assert_eq!(
        packet.spend_posture_class.as_str(),
        fixture.expect.spend_posture_class,
        "spend_posture_class"
    );
    assert_eq!(
        packet.draft_state_token, fixture.expect.draft_state_token,
        "draft_state_token"
    );
    assert_eq!(
        snapshot.has_invariant_violations, fixture.expect.has_invariant_violations,
        "has_invariant_violations"
    );

    assert_eq!(
        packet.result_lineage.len(),
        fixture.expect.result_lineage_tokens.len(),
        "result lineage row count"
    );
    for (row, expected) in packet
        .result_lineage
        .iter()
        .zip(fixture.expect.result_lineage_tokens.iter())
    {
        assert_eq!(row.block_reason_token, expected.block_reason_token);
        assert_eq!(
            row.addressable_target_token,
            expected.addressable_target_token
        );
    }

    let summary = &packet.context_summary;
    let expected_summary = &fixture.expect.context_summary;
    assert_eq!(summary.mention_count, expected_summary.mention_count);
    assert_eq!(
        summary.resolved_mention_count,
        expected_summary.resolved_mention_count
    );
    assert_eq!(summary.attachment_count, expected_summary.attachment_count);
    assert_eq!(
        summary.trusted_attachment_count,
        expected_summary.trusted_attachment_count
    );
    assert_eq!(
        summary.tainted_attachment_count,
        expected_summary.tainted_attachment_count
    );
    assert_eq!(
        summary.fenced_attachment_count,
        expected_summary.fenced_attachment_count
    );
    assert_eq!(
        summary.slash_command_count,
        expected_summary.slash_command_count
    );
    assert_eq!(
        summary.resolved_slash_command_count,
        expected_summary.resolved_slash_command_count
    );
    assert_eq!(
        summary.aggregate_byte_estimate,
        expected_summary.aggregate_byte_estimate
    );
}

#[test]
fn fixture_protected_walk_local_no_dispatch_replays_into_the_wedge() {
    let (fixture, snapshot) = run_fixture("protected_walk_local_no_dispatch.json");
    assert_fixture_matches(&fixture, &snapshot);
}

#[test]
fn fixture_failure_drill_alternate_route_and_spend_replays_into_the_wedge() {
    let (fixture, snapshot) = run_fixture("failure_drill_alternate_route_and_spend.json");
    assert_fixture_matches(&fixture, &snapshot);
    // Failure-drill specific: the strip row tokens MUST flip away from
    // the live defaults so the user sees the route/spend posture
    // surfaced verbatim.
    let provider_row = snapshot.strip_row("provider").expect("provider row");
    assert_eq!(provider_row.value_token, "mocked_test_provider");
    let path_row = snapshot
        .strip_row("local_or_remote_path")
        .expect("path row");
    assert_eq!(path_row.value_token, "remote_mocked_for_fixtures");
    let spend_row = snapshot.strip_row("spend_posture").expect("spend row");
    assert_eq!(spend_row.value_token, "mocked_spend_for_fixtures");
}
