//! Seeded teaching/classroom conformance corpus, support export, and validation.
//!
//! This is the mint-from-truth corpus for the beta teaching/classroom sessions.
//! Each case builds one [`TeachingSession`] and projects its role-aware
//! [`TeachingAffordanceProjection`]; the checked-in fixtures under
//! `fixtures/help/m3/teaching_classroom/` are a literal projection of
//! [`seeded_teaching_classroom_corpus`], so the JSON cannot drift from the Rust
//! types.
//!
//! The corpus exercises every teaching role, every client class, every
//! docs-pack state (installed, cached, mirrored, offline, not-installed), every
//! segment kind, every demonstration kind, every replay policy, every retention
//! class, both session kinds, and all three restore triggers so the exit-gate
//! contract — role-aware, cited, non-mutating-by-default, offline-honest, and
//! restore-on-exit — is proven rather than asserted. Every segment cites the
//! same learning-mode tours, exercise packs, docs nodes, and graph nodes that
//! learning mode itself ships, so teaching content can never fork from learning
//! mode.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::affordances::{project_affordances, TeachingAffordanceProjection};
use super::session::{
    restore_from_checkpoint, ClientClass, DemonstratedAction, DemonstrationKind, DocsPackState,
    ReplayPolicy, RestoreCheckpoint, RestoreTrigger, RetentionClass, SegmentKind, SessionKind,
    SessionLifecycleState, TeachingParticipant, TeachingRestoreOutcome, TeachingRole,
    TeachingSegment, TeachingSession, TeachingSessionBuilder, TEACHING_SESSION_BETA_SCHEMA_VERSION,
    TEACHING_SESSION_BETA_SHARED_CONTRACT_REF, TEACHING_SESSION_RECORD_KIND,
};

/// Stable record kind for [`TeachingClassroomSessionCase`] payloads.
pub const TEACHING_CLASSROOM_SESSION_CASE_RECORD_KIND: &str =
    "shell_teaching_classroom_session_case_record";

/// Stable record kind for [`TeachingClassroomCorpus`] payloads.
pub const TEACHING_CLASSROOM_CORPUS_RECORD_KIND: &str = "shell_teaching_classroom_corpus_record";

/// Stable record kind for [`TeachingClassroomSupportExport`] payloads.
pub const TEACHING_CLASSROOM_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_teaching_classroom_support_export_record";

/// Stable record kind for [`TeachingClassroomSupportExportRow`] payloads.
pub const TEACHING_CLASSROOM_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "shell_teaching_classroom_support_export_row_record";

const GENERATED_AT: &str = "2026-05-20T00:00:00Z";

/// One seeded case: a scenario, the governed session, and its role-aware
/// affordance projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingClassroomSessionCase {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub scenario_label: String,
    pub session: TeachingSession,
    pub affordances: TeachingAffordanceProjection,
}

/// Aggregate coverage summary for the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingClassroomCorpusSummary {
    pub session_count: u32,
    pub session_kinds_covered: Vec<SessionKind>,
    pub roles_covered: Vec<TeachingRole>,
    pub client_classes_covered: Vec<ClientClass>,
    pub docs_pack_states_covered: Vec<DocsPackState>,
    pub segment_kinds_covered: Vec<SegmentKind>,
    pub demonstration_kinds_covered: Vec<DemonstrationKind>,
    pub replay_policies_covered: Vec<ReplayPolicy>,
    pub retention_classes_covered: Vec<RetentionClass>,
    pub restore_triggers_covered: Vec<RestoreTrigger>,
    pub all_segments_cite_learning_mode: bool,
    pub all_segments_resumable: bool,
    pub all_docs_pack_states_disclosed: bool,
    pub all_demonstrations_non_mutating_or_fenced: bool,
    pub all_opt_in_markers_consistent: bool,
    pub no_role_grants_terminal_or_debug_control: bool,
    pub no_authority_widening: bool,
    pub all_constrained_clients_join_safe: bool,
    pub all_restores_match_checkpoint: bool,
}

/// The full seeded teaching/classroom corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingClassroomCorpus {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub generated_at: String,
    pub summary: TeachingClassroomCorpusSummary,
    pub sessions: Vec<TeachingClassroomSessionCase>,
    pub restore_outcomes: Vec<TeachingRestoreOutcome>,
}

/// One support-safe row. Carries enums, counts, refs, and guardrail booleans —
/// never segment titles, scenario copy, or raw file paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingClassroomSupportExportRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub session_id: String,
    pub session_kind: SessionKind,
    pub lifecycle_state: SessionLifecycleState,
    pub local_role: TeachingRole,
    pub replay_policy: ReplayPolicy,
    pub retention_class: RetentionClass,
    pub segment_count: u32,
    pub segment_kinds: Vec<SegmentKind>,
    pub docs_pack_states: Vec<DocsPackState>,
    pub demonstration_kinds: Vec<DemonstrationKind>,
    pub mutating_demonstration_count: u32,
    pub participant_count: u32,
    pub roles: Vec<TeachingRole>,
    pub client_classes: Vec<ClientClass>,
    pub constrained_client_count: u32,
    pub external_guest_count: u32,
    pub replay_archive_opt_in_explicit: bool,
    pub shared_retention_opt_in_explicit: bool,
    pub grants_mutation_authority: bool,
    pub grants_terminal_or_debug_control: bool,
    pub grants_broader_authority_than_workspace: bool,
    pub establishes_private_data_ownership: bool,
    pub creates_hidden_progress_model: bool,
    pub creates_cohort_or_grading_flow: bool,
    pub demonstrations_non_mutating_by_default: bool,
    pub preserves_source_citations: bool,
    pub reuses_learning_mode_objects_only: bool,
    pub restore_on_exit_guaranteed: bool,
    pub all_constrained_clients_join_safe: bool,
    pub keyboard_complete: bool,
    pub pointer_only: bool,
    pub screen_reader_reachable: bool,
}

/// Support-export wrapper over the corpus. Privacy-safe by construction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingClassroomSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub export_id: String,
    pub generated_at: String,
    pub rows: Vec<TeachingClassroomSupportExportRow>,
    pub restore_outcomes: Vec<TeachingRestoreOutcome>,
    pub raw_private_material_excluded: bool,
}

impl TeachingClassroomSupportExport {
    /// Project a corpus into a support-safe export.
    pub fn from_corpus(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        corpus: &TeachingClassroomCorpus,
    ) -> Self {
        let rows = corpus
            .sessions
            .iter()
            .map(|case| {
                let s = &case.session;
                let segment_kinds: Vec<SegmentKind> = s
                    .segments
                    .iter()
                    .map(|seg| seg.segment_kind)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect();
                let docs_pack_states: Vec<DocsPackState> = s
                    .segments
                    .iter()
                    .map(|seg| seg.docs_pack_state)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect();
                let demonstration_kinds: Vec<DemonstrationKind> = s
                    .segments
                    .iter()
                    .filter_map(|seg| seg.demonstrated_action.as_ref().map(|a| a.kind))
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect();
                let mutating_demonstration_count = s
                    .segments
                    .iter()
                    .filter_map(|seg| seg.demonstrated_action.as_ref())
                    .filter(|a| a.mutates_workspace)
                    .count() as u32;
                let roles: Vec<TeachingRole> = s
                    .participants
                    .iter()
                    .map(|p| p.role)
                    .chain(std::iter::once(s.local_role))
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect();
                let client_classes: Vec<ClientClass> = s
                    .participants
                    .iter()
                    .map(|p| p.client_class)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect();
                let constrained_client_count = s
                    .participants
                    .iter()
                    .filter(|p| p.client_class.is_constrained())
                    .count() as u32;
                let external_guest_count = s
                    .participants
                    .iter()
                    .filter(|p| p.is_external_guest)
                    .count() as u32;

                TeachingClassroomSupportExportRow {
                    record_kind: TEACHING_CLASSROOM_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
                    schema_version: TEACHING_SESSION_BETA_SCHEMA_VERSION,
                    shared_contract_ref: TEACHING_SESSION_BETA_SHARED_CONTRACT_REF.to_owned(),
                    case_id: case.case_id.clone(),
                    session_id: s.session_id.clone(),
                    session_kind: s.session_kind,
                    lifecycle_state: s.lifecycle_state,
                    local_role: s.local_role,
                    replay_policy: s.replay_policy,
                    retention_class: s.retention_class,
                    segment_count: s.segments.len() as u32,
                    segment_kinds,
                    docs_pack_states,
                    demonstration_kinds,
                    mutating_demonstration_count,
                    participant_count: s.participants.len() as u32,
                    roles,
                    client_classes,
                    constrained_client_count,
                    external_guest_count,
                    replay_archive_opt_in_explicit: s.replay_archive_opt_in_explicit,
                    shared_retention_opt_in_explicit: s.shared_retention_opt_in_explicit,
                    grants_mutation_authority: s.grants_mutation_authority,
                    grants_terminal_or_debug_control: s.grants_terminal_or_debug_control,
                    grants_broader_authority_than_workspace: s
                        .grants_broader_authority_than_workspace,
                    establishes_private_data_ownership: s.establishes_private_data_ownership,
                    creates_hidden_progress_model: s.creates_hidden_progress_model,
                    creates_cohort_or_grading_flow: s.creates_cohort_or_grading_flow,
                    demonstrations_non_mutating_by_default: s
                        .demonstrations_non_mutating_by_default,
                    preserves_source_citations: s.preserves_source_citations,
                    reuses_learning_mode_objects_only: s.reuses_learning_mode_objects_only,
                    restore_on_exit_guaranteed: s.restore_on_exit_guaranteed,
                    all_constrained_clients_join_safe: case
                        .affordances
                        .all_constrained_clients_join_safe,
                    keyboard_complete: case.affordances.keyboard_complete,
                    pointer_only: case.affordances.pointer_only,
                    screen_reader_reachable: case.affordances.screen_reader_reachable,
                }
            })
            .collect();

        Self {
            record_kind: TEACHING_CLASSROOM_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TEACHING_SESSION_BETA_SCHEMA_VERSION,
            shared_contract_ref: TEACHING_SESSION_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            rows,
            restore_outcomes: corpus.restore_outcomes.clone(),
            raw_private_material_excluded: true,
        }
    }
}

// ---- builders -------------------------------------------------------------

fn checkpoint(id: &str, at: &str) -> RestoreCheckpoint {
    RestoreCheckpoint {
        checkpoint_id: format!("teaching:checkpoint:{id}"),
        prior_layout_ref: format!("window-topology:{id}:prior"),
        prior_focus_ref: format!("focus-chain:{id}:prior"),
        prior_panel_visibility_ref: format!("panel-visibility:{id}:prior"),
        accessibility_posture_ref: format!("a11y-posture:{id}:prior"),
        captured_at: at.to_owned(),
    }
}

/// A segment whose docs pack is installed and current (no disclosure needed).
#[allow(clippy::too_many_arguments)]
fn segment(
    id: &str,
    ordinal: u32,
    kind: SegmentKind,
    title: &str,
    learning_object_ref: &str,
    docs_node_refs: &[&str],
    graph_node_refs: &[&str],
    citation_refs: &[&str],
    docs_pack_ref: &str,
    docs_pack_state: DocsPackState,
    demonstrated_action: Option<DemonstratedAction>,
) -> TeachingSegment {
    let disclosure = if docs_pack_state.requires_disclosure() {
        Some(format!(
            "disclosure:docs-pack:{id}:{}",
            docs_pack_state.as_str()
        ))
    } else {
        None
    };
    TeachingSegment {
        segment_id: id.to_owned(),
        ordinal,
        segment_kind: kind,
        title: title.to_owned(),
        learning_object_ref: learning_object_ref.to_owned(),
        docs_node_refs: docs_node_refs.iter().map(|s| s.to_string()).collect(),
        graph_node_refs: graph_node_refs.iter().map(|s| s.to_string()).collect(),
        citation_refs: citation_refs.iter().map(|s| s.to_string()).collect(),
        docs_pack_ref: docs_pack_ref.to_owned(),
        docs_pack_state,
        docs_pack_disclosure_ref: disclosure,
        resume_ref: format!("resume:segment:{id}"),
        resumable_across_restart: true,
        resumable_across_reconnect: true,
        demonstrated_action,
        cites_learning_mode_object: true,
    }
}

fn participant(
    id: &str,
    role: TeachingRole,
    client_class: ClientClass,
    is_external_guest: bool,
) -> TeachingParticipant {
    TeachingParticipant {
        participant_id: id.to_owned(),
        role,
        client_class,
        is_external_guest,
    }
}

// ---- seed sessions --------------------------------------------------------

struct SeedCase {
    case_id: &'static str,
    scenario_label: &'static str,
    session: TeachingSession,
}

fn seed_sessions() -> Vec<SeedCase> {
    vec![
        // 1. Solo teaching walkthrough over an installed docs pack. One segment
        //    of each kind (tour, exercise pack, glossary card, speaker note),
        //    and a fenced mutation demonstration that reuses the ordinary
        //    import-profile command path. Ephemeral, discard-on-exit, exit.
        SeedCase {
            case_id: "case:teaching-installed-full-coverage",
            scenario_label:
                "A presenter teaches the safe-start path alone over an installed docs pack. The agenda walks the safe-start tour, the import-profile exercise pack, a glossary card, and a speaker note. The one demonstrated mutation reuses the ordinary import-profile command id, preview sheet, approval fence, and rollback semantics; nothing is retained.",
            session: TeachingSessionBuilder::new(
                "teaching:session:installed-01",
                SessionKind::Teaching,
                TeachingRole::Moderator,
                ReplayPolicy::Ephemeral,
                RetentionClass::DiscardOnExit,
                checkpoint("installed-01", "2026-05-20T09:00:00Z"),
            )
            .focus("segment:installed-01:1")
            .exercise_pack("tour-pack:aureline.safe-start.beta")
            .segment(segment(
                "segment:installed-01:1",
                1,
                SegmentKind::Tour,
                "Walk the safe-start command-backed tour",
                "tour:aureline.safe-start.command-backed",
                &["docs-node:help.guided-tours.safe-start", "docs-node:workspace.open-folder"],
                &[],
                &["citation:command-registry:workspace.open-folder", "citation:docs:workspace.open-folder"],
                "docs-pack:aureline-help:guided-tours",
                DocsPackState::Installed,
                Some(DemonstratedAction::open_docs(
                    "action:installed-01:open-docs",
                    "cmd:docs.open_in_browser",
                )),
            ))
            .segment(segment(
                "segment:installed-01:2",
                2,
                SegmentKind::ExercisePack,
                "Prepare an import preview before any profile mutation",
                "step:safe-start.import-profile-preview",
                &["docs-node:onboarding.import-profile.preview"],
                &["graph-node:command.workspace.import_profile"],
                &["citation:command-registry:workspace.import-profile", "citation:docs:onboarding.import-profile.preview"],
                "docs-pack:aureline-help:guided-tours",
                DocsPackState::Installed,
                Some(DemonstratedAction::mutation_through_fences(
                    "action:installed-01:import-profile",
                    "cmd:workspace.import_profile",
                    "preview:workspace.import_profile",
                    "approval:path:workspace.import_profile",
                    "rollback:workspace.import_profile.checkpoint-or-undo",
                    "evidence-rule:command-preview-approval",
                )),
            ))
            .segment(segment(
                "segment:installed-01:3",
                3,
                SegmentKind::GlossaryCard,
                "Define the cited docs source vocabulary",
                "tour:aureline.safe-start.command-backed",
                &["docs-node:help.guided-tours.safe-start"],
                &[],
                &["citation:docs-help:guided-tours-beta"],
                "docs-pack:aureline-help:guided-tours",
                DocsPackState::Installed,
                Some(DemonstratedAction::explain("action:installed-01:glossary")),
            ))
            .segment(segment(
                "segment:installed-01:4",
                4,
                SegmentKind::SpeakerNote,
                "Presenter cue: open the cited source before trusting a claim",
                "step:safe-start.read-docs-source",
                &["docs-node:help.guided-tours.safe-start"],
                &[],
                &["citation:architecture:BR.2:derived-tour-artifacts"],
                "docs-pack:aureline-help:guided-tours",
                DocsPackState::Installed,
                Some(DemonstratedAction::explain("action:installed-01:speaker-note")),
            ))
            .build(),
        },
        // 2. Full classroom over a cached docs pack: every role present, with a
        //    limited observer and a low-bandwidth scribe. Local replay,
        //    local retention, leave restore. The approver sees an approve
        //    affordance that reuses the ordinary fence.
        SeedCase {
            case_id: "case:classroom-cached-every-role",
            scenario_label:
                "A classroom runs over a cached docs pack with the cached label visible. A moderator drives; a full-client participant works the exercise; a limited-client observer watches; a low-bandwidth scribe takes notes; an approver can approve the demonstrated mutation through the ordinary fence. Replay stays local and user-owned; nothing leaves the machine without opt-in.",
            session: TeachingSessionBuilder::new(
                "teaching:session:cached-02",
                SessionKind::Classroom,
                TeachingRole::Moderator,
                ReplayPolicy::LocalReplayUserOwned,
                RetentionClass::LocalUserOwned,
                checkpoint("cached-02", "2026-05-20T09:10:00Z"),
            )
            .focus("segment:cached-02:1")
            .exercise_pack("tour-pack:aureline.cached-docs.preview")
            .participant(participant("teaching:cached-02:participant", TeachingRole::Participant, ClientClass::Full, false))
            .participant(participant("teaching:cached-02:observer", TeachingRole::Observer, ClientClass::Limited, false))
            .participant(participant("teaching:cached-02:scribe", TeachingRole::Scribe, ClientClass::LowBandwidth, false))
            .participant(participant("teaching:cached-02:approver", TeachingRole::Approver, ClientClass::Full, false))
            .segment(segment(
                "segment:cached-02:1",
                1,
                SegmentKind::Tour,
                "Open cached docs with the cached label visible",
                "tour:aureline.cached-docs.fallback",
                &["docs-node:help.cached-docs.learning"],
                &[],
                &["citation:mirror:guided-learning.cached-docs"],
                "docs-pack:mirror:aureline-guided-learning",
                DocsPackState::Cached,
                Some(DemonstratedAction::open_docs(
                    "action:cached-02:open-docs",
                    "cmd:docs.open_in_browser",
                )),
            ))
            .segment(segment(
                "segment:cached-02:2",
                2,
                SegmentKind::ExercisePack,
                "Demonstrate the fenced import preview for the class",
                "step:safe-start.import-profile-preview",
                &["docs-node:onboarding.import-profile.preview"],
                &["graph-node:command.workspace.import_profile"],
                &["citation:command-registry:workspace.import-profile"],
                "docs-pack:mirror:aureline-guided-learning",
                DocsPackState::Cached,
                Some(DemonstratedAction::mutation_through_fences(
                    "action:cached-02:import-profile",
                    "cmd:workspace.import_profile",
                    "preview:workspace.import_profile",
                    "approval:path:workspace.import_profile",
                    "rollback:workspace.import_profile.checkpoint-or-undo",
                    "evidence-rule:command-preview-approval",
                )),
            ))
            .build(),
        },
        // 3. Classroom over an offline docs pack: remote enrichment is
        //    unavailable, so local content shows with an explicit reconnect
        //    disclosure. Shared archive + shared retention, both opted into.
        //    A low-bandwidth participant joins as a note-taker. Crash recovery.
        SeedCase {
            case_id: "case:classroom-offline-shared-archive",
            scenario_label:
                "A classroom runs while remote enrichment is offline. Local content shows with an explicit reconnect-required disclosure rather than pretending the remote graph is live. The replay is archived to the shared workspace and retained there — both explicit opt-ins. A low-bandwidth participant joins as a note-taker. The session is crash-recovered and the prior workspace is restored.",
            session: TeachingSessionBuilder::new(
                "teaching:session:offline-03",
                SessionKind::Classroom,
                TeachingRole::Moderator,
                ReplayPolicy::SharedArchiveOptIn,
                RetentionClass::SharedWorkspaceRetained,
                checkpoint("offline-03", "2026-05-20T09:20:00Z"),
            )
            .lifecycle(SessionLifecycleState::Active)
            .focus("segment:offline-03:1")
            .exercise_pack("tour-pack:aureline.graph-map.placeholder")
            .participant(participant("teaching:offline-03:participant", TeachingRole::Participant, ClientClass::LowBandwidth, false))
            .segment(segment(
                "segment:offline-03:1",
                1,
                SegmentKind::Tour,
                "Show the dependency topology with the graph offline",
                "tour:aureline.graph-map.placeholder",
                &["docs-node:help.graph-map.unavailable"],
                &["graph-node:unavailable"],
                &["citation:docs-help:guided-tours-beta"],
                "docs-pack:aureline-help:graph-map-placeholder",
                DocsPackState::Offline,
                Some(DemonstratedAction::explain("action:offline-03:explain")),
            ))
            .segment(segment(
                "segment:offline-03:2",
                2,
                SegmentKind::GlossaryCard,
                "Define topology terms from the offline placeholder",
                "tour:aureline.graph-map.placeholder",
                &["docs-node:help.graph-map.unavailable"],
                &[],
                &["citation:docs-help:guided-tours-beta"],
                "docs-pack:aureline-help:graph-map-placeholder",
                DocsPackState::Offline,
                None,
            ))
            .build(),
        },
        // 4. Solo teaching over a mirrored docs pack: an offline mirror serves
        //    stale-but-disclosed content. A preview-only demonstration prepares
        //    a preview without applying it. Local replay, local retention, exit.
        SeedCase {
            case_id: "case:teaching-mirrored-preview-only",
            scenario_label:
                "A presenter teaches from an offline mirror of the guided-learning docs. The mirror label is visible so the room knows the content is stale-but-disclosed. The demonstrated action prepares an import preview only — it is never applied — proving demonstrations are non-mutating by default. Replay stays local; the session exits and restores cleanly.",
            session: TeachingSessionBuilder::new(
                "teaching:session:mirrored-04",
                SessionKind::Teaching,
                TeachingRole::Moderator,
                ReplayPolicy::LocalReplayUserOwned,
                RetentionClass::LocalUserOwned,
                checkpoint("mirrored-04", "2026-05-20T09:30:00Z"),
            )
            .focus("segment:mirrored-04:1")
            .exercise_pack("tour-pack:aureline.cached-docs.preview")
            .participant(participant("teaching:mirrored-04:participant", TeachingRole::Participant, ClientClass::Full, false))
            .segment(segment(
                "segment:mirrored-04:1",
                1,
                SegmentKind::Tour,
                "Open mirrored docs with the mirror label visible",
                "tour:aureline.cached-docs.fallback",
                &["docs-node:help.cached-docs.learning"],
                &[],
                &["citation:mirror:guided-learning.cached-docs"],
                "docs-pack:mirror:aureline-guided-learning",
                DocsPackState::Mirrored,
                Some(DemonstratedAction::preview_only(
                    "action:mirrored-04:preview",
                    "cmd:workspace.import_profile",
                    "preview:workspace.import_profile",
                )),
            ))
            .segment(segment(
                "segment:mirrored-04:2",
                2,
                SegmentKind::SpeakerNote,
                "Presenter cue: call out the mirror snapshot date",
                "tour:aureline.cached-docs.fallback",
                &["docs-node:help.cached-docs.learning"],
                &[],
                &["citation:mirror:guided-learning.cached-docs"],
                "docs-pack:mirror:aureline-guided-learning",
                DocsPackState::Mirrored,
                Some(DemonstratedAction::explain("action:mirrored-04:speaker-note")),
            ))
            .build(),
        },
        // 5. Teaching with a not-installed docs pack: the pack's content is
        //    blocked behind an explicit install, disclosed rather than faked. An
        //    invited external guest joins as an observer. Ephemeral,
        //    discard-on-exit, leave.
        SeedCase {
            case_id: "case:teaching-not-installed-blocked",
            scenario_label:
                "A presenter reaches a segment whose docs pack is not installed. The content is blocked behind an explicit install disclosure instead of pretending the enrichment is available. An invited external guest joins as an observer on a limited client and sees no broken controls. Nothing is retained; the guest leaves and the prior workspace is restored.",
            session: TeachingSessionBuilder::new(
                "teaching:session:not-installed-05",
                SessionKind::Teaching,
                TeachingRole::Moderator,
                ReplayPolicy::Ephemeral,
                RetentionClass::DiscardOnExit,
                checkpoint("not-installed-05", "2026-05-20T09:40:00Z"),
            )
            .focus("segment:not-installed-05:1")
            .exercise_pack("tour-pack:aureline.safe-start.beta")
            .participant(participant("teaching:not-installed-05:guest", TeachingRole::Observer, ClientClass::Limited, true))
            .segment(segment(
                "segment:not-installed-05:1",
                1,
                SegmentKind::Tour,
                "Walk the installed safe-start tour first",
                "tour:aureline.safe-start.command-backed",
                &["docs-node:help.guided-tours.safe-start"],
                &[],
                &["citation:docs-help:guided-tours-beta"],
                "docs-pack:aureline-help:guided-tours",
                DocsPackState::Installed,
                Some(DemonstratedAction::open_docs(
                    "action:not-installed-05:open-docs",
                    "cmd:docs.open_in_browser",
                )),
            ))
            .segment(segment(
                "segment:not-installed-05:2",
                2,
                SegmentKind::GlossaryCard,
                "Glossary card whose pack must be installed first",
                "tour:aureline.cached-docs.fallback",
                &["docs-node:help.cached-docs.learning"],
                &[],
                &["citation:mirror:guided-learning.cached-docs"],
                "docs-pack:mirror:aureline-guided-learning",
                DocsPackState::NotInstalled,
                Some(DemonstratedAction::explain("action:not-installed-05:explain")),
            ))
            .build(),
        },
    ]
}

/// The full seeded teaching/classroom corpus.
pub fn seeded_teaching_classroom_corpus() -> TeachingClassroomCorpus {
    let seeds = seed_sessions();
    let sessions: Vec<TeachingClassroomSessionCase> = seeds
        .iter()
        .map(|seed| TeachingClassroomSessionCase {
            record_kind: TEACHING_CLASSROOM_SESSION_CASE_RECORD_KIND.to_owned(),
            schema_version: TEACHING_SESSION_BETA_SCHEMA_VERSION,
            shared_contract_ref: TEACHING_SESSION_BETA_SHARED_CONTRACT_REF.to_owned(),
            case_id: seed.case_id.to_owned(),
            scenario_label: seed.scenario_label.to_owned(),
            session: seed.session.clone(),
            affordances: project_affordances(&seed.session),
        })
        .collect();

    // Restore outcomes prove the prior environment returns under every trigger.
    let by_id = |id: &str| {
        sessions
            .iter()
            .find(|c| c.case_id == id)
            .map(|c| &c.session)
            .expect("seed session present")
    };
    let restore_outcomes = vec![
        restore_from_checkpoint(
            by_id("case:teaching-installed-full-coverage"),
            RestoreTrigger::Exit,
        ),
        restore_from_checkpoint(
            by_id("case:classroom-cached-every-role"),
            RestoreTrigger::Leave,
        ),
        restore_from_checkpoint(
            by_id("case:classroom-offline-shared-archive"),
            RestoreTrigger::CrashRecovery,
        ),
    ];

    let summary = summarize(&sessions, &restore_outcomes);

    TeachingClassroomCorpus {
        record_kind: TEACHING_CLASSROOM_CORPUS_RECORD_KIND.to_owned(),
        schema_version: TEACHING_SESSION_BETA_SCHEMA_VERSION,
        shared_contract_ref: TEACHING_SESSION_BETA_SHARED_CONTRACT_REF.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        summary,
        sessions,
        restore_outcomes,
    }
}

fn summarize(
    sessions: &[TeachingClassroomSessionCase],
    restore_outcomes: &[TeachingRestoreOutcome],
) -> TeachingClassroomCorpusSummary {
    let mut session_kinds: BTreeSet<SessionKind> = BTreeSet::new();
    let mut roles: BTreeSet<TeachingRole> = BTreeSet::new();
    let mut client_classes: BTreeSet<ClientClass> = BTreeSet::new();
    let mut docs_pack_states: BTreeSet<DocsPackState> = BTreeSet::new();
    let mut segment_kinds: BTreeSet<SegmentKind> = BTreeSet::new();
    let mut demonstration_kinds: BTreeSet<DemonstrationKind> = BTreeSet::new();
    let mut replay_policies: BTreeSet<ReplayPolicy> = BTreeSet::new();
    let mut retention_classes: BTreeSet<RetentionClass> = BTreeSet::new();
    let mut restore_triggers: BTreeSet<RestoreTrigger> = BTreeSet::new();

    let mut all_cite = true;
    let mut all_resumable = true;
    let mut all_disclosed = true;
    let mut all_fenced = true;
    let mut all_opt_in = true;
    let mut no_terminal = true;
    let mut no_widening = true;
    let mut all_constrained_safe = true;

    for case in sessions {
        let s = &case.session;
        session_kinds.insert(s.session_kind);
        roles.insert(s.local_role);
        replay_policies.insert(s.replay_policy);
        retention_classes.insert(s.retention_class);

        all_cite &= s.segments_cite_learning_mode() && s.preserves_source_citations;
        all_resumable &= s.segments_are_resumable();
        all_disclosed &= s.docs_pack_states_disclosed();
        all_fenced &= s.demonstrations_are_fenced() && s.demonstrations_non_mutating_by_default;
        all_opt_in &= s.opt_in_markers_consistent();
        no_terminal &= !s.grants_terminal_or_debug_control;
        no_widening &= !s.grants_mutation_authority
            && !s.grants_broader_authority_than_workspace
            && !s.establishes_private_data_ownership
            && !s.creates_hidden_progress_model
            && !s.creates_cohort_or_grading_flow;
        all_constrained_safe &= case.affordances.all_constrained_clients_join_safe;

        for seg in &s.segments {
            segment_kinds.insert(seg.segment_kind);
            docs_pack_states.insert(seg.docs_pack_state);
            if let Some(action) = &seg.demonstrated_action {
                demonstration_kinds.insert(action.kind);
            }
        }
        for p in &s.participants {
            roles.insert(p.role);
            client_classes.insert(p.client_class);
        }
        // The local user is always projected as a full client.
        client_classes.insert(ClientClass::Full);
    }

    let mut all_restores_match = true;
    for outcome in restore_outcomes {
        restore_triggers.insert(outcome.trigger);
        all_restores_match &= outcome.matches_checkpoint && !outcome.left_in_improvised_shell;
    }

    TeachingClassroomCorpusSummary {
        session_count: sessions.len() as u32,
        session_kinds_covered: session_kinds.into_iter().collect(),
        roles_covered: roles.into_iter().collect(),
        client_classes_covered: client_classes.into_iter().collect(),
        docs_pack_states_covered: docs_pack_states.into_iter().collect(),
        segment_kinds_covered: segment_kinds.into_iter().collect(),
        demonstration_kinds_covered: demonstration_kinds.into_iter().collect(),
        replay_policies_covered: replay_policies.into_iter().collect(),
        retention_classes_covered: retention_classes.into_iter().collect(),
        restore_triggers_covered: restore_triggers.into_iter().collect(),
        all_segments_cite_learning_mode: all_cite,
        all_segments_resumable: all_resumable,
        all_docs_pack_states_disclosed: all_disclosed,
        all_demonstrations_non_mutating_or_fenced: all_fenced,
        all_opt_in_markers_consistent: all_opt_in,
        no_role_grants_terminal_or_debug_control: no_terminal,
        no_authority_widening: no_widening,
        all_constrained_clients_join_safe: all_constrained_safe,
        all_restores_match_checkpoint: all_restores_match,
    }
}

// ---- validation -----------------------------------------------------------

/// Validate the corpus invariants. Returns the list of violations; an empty
/// list means the corpus conforms.
pub fn validate_teaching_classroom_corpus(
    corpus: &TeachingClassroomCorpus,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if corpus.record_kind != TEACHING_CLASSROOM_CORPUS_RECORD_KIND {
        errors.push(format!("corpus record_kind is {}", corpus.record_kind));
    }
    if corpus.schema_version != TEACHING_SESSION_BETA_SCHEMA_VERSION {
        errors.push(format!(
            "corpus schema_version is {}",
            corpus.schema_version
        ));
    }
    if corpus.shared_contract_ref != TEACHING_SESSION_BETA_SHARED_CONTRACT_REF {
        errors.push(format!(
            "corpus shared_contract_ref is {}",
            corpus.shared_contract_ref
        ));
    }
    if corpus.sessions.is_empty() {
        errors.push("corpus has no sessions".to_owned());
    }

    let mut seen_case_ids: BTreeSet<&str> = BTreeSet::new();
    let mut seen_session_ids: BTreeSet<&str> = BTreeSet::new();
    for case in &corpus.sessions {
        if !seen_case_ids.insert(case.case_id.as_str()) {
            errors.push(format!("duplicate case_id {}", case.case_id));
        }
        if !seen_session_ids.insert(case.session.session_id.as_str()) {
            errors.push(format!("duplicate session_id {}", case.session.session_id));
        }
        validate_case(case, &mut errors);
    }

    for outcome in &corpus.restore_outcomes {
        if !outcome.matches_checkpoint {
            errors.push(format!(
                "{}: restore outcome does not match the checkpoint",
                outcome.session_id
            ));
        }
        if outcome.left_in_improvised_shell {
            errors.push(format!(
                "{}: restore outcome left the user in an improvised shell",
                outcome.session_id
            ));
        }
    }

    // Summary must agree with the cases it claims to summarize.
    let recomputed = summarize(&corpus.sessions, &corpus.restore_outcomes);
    if recomputed != corpus.summary {
        errors.push("corpus summary does not match its cases".to_owned());
    }

    // Coverage: every role, client class, docs-pack state, segment kind,
    // demonstration kind, replay policy, retention class, session kind, and all
    // three restore triggers.
    for role in [
        TeachingRole::Moderator,
        TeachingRole::Participant,
        TeachingRole::Observer,
        TeachingRole::Approver,
        TeachingRole::Scribe,
    ] {
        if !corpus.summary.roles_covered.contains(&role) {
            errors.push(format!("corpus does not cover role {}", role.as_str()));
        }
    }
    for class in [
        ClientClass::Full,
        ClientClass::Limited,
        ClientClass::LowBandwidth,
    ] {
        if !corpus.summary.client_classes_covered.contains(&class) {
            errors.push(format!(
                "corpus does not cover client class {}",
                class.as_str()
            ));
        }
    }
    for state in [
        DocsPackState::Installed,
        DocsPackState::Cached,
        DocsPackState::Mirrored,
        DocsPackState::Offline,
        DocsPackState::NotInstalled,
    ] {
        if !corpus.summary.docs_pack_states_covered.contains(&state) {
            errors.push(format!(
                "corpus does not cover docs-pack state {}",
                state.as_str()
            ));
        }
    }
    for kind in [
        SegmentKind::Tour,
        SegmentKind::ExercisePack,
        SegmentKind::GlossaryCard,
        SegmentKind::SpeakerNote,
    ] {
        if !corpus.summary.segment_kinds_covered.contains(&kind) {
            errors.push(format!(
                "corpus does not cover segment kind {}",
                kind.as_str()
            ));
        }
    }
    for kind in [
        DemonstrationKind::Explain,
        DemonstrationKind::OpenDocs,
        DemonstrationKind::PreviewOnly,
        DemonstrationKind::MutationThroughFences,
    ] {
        if !corpus.summary.demonstration_kinds_covered.contains(&kind) {
            errors.push(format!(
                "corpus does not cover demonstration kind {}",
                kind.as_str()
            ));
        }
    }
    for policy in [
        ReplayPolicy::Ephemeral,
        ReplayPolicy::LocalReplayUserOwned,
        ReplayPolicy::SharedArchiveOptIn,
    ] {
        if !corpus.summary.replay_policies_covered.contains(&policy) {
            errors.push(format!(
                "corpus does not cover replay policy {}",
                policy.as_str()
            ));
        }
    }
    for class in [
        RetentionClass::DiscardOnExit,
        RetentionClass::LocalUserOwned,
        RetentionClass::SharedWorkspaceRetained,
    ] {
        if !corpus.summary.retention_classes_covered.contains(&class) {
            errors.push(format!(
                "corpus does not cover retention class {}",
                class.as_str()
            ));
        }
    }
    for kind in [SessionKind::Teaching, SessionKind::Classroom] {
        if !corpus.summary.session_kinds_covered.contains(&kind) {
            errors.push(format!(
                "corpus does not cover session kind {}",
                kind.as_str()
            ));
        }
    }
    for trigger in [
        RestoreTrigger::Exit,
        RestoreTrigger::Leave,
        RestoreTrigger::CrashRecovery,
    ] {
        if !corpus.summary.restore_triggers_covered.contains(&trigger) {
            errors.push(format!(
                "corpus does not cover restore trigger {}",
                trigger.as_str()
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_case(case: &TeachingClassroomSessionCase, errors: &mut Vec<String>) {
    let where_ = &case.case_id;
    let s = &case.session;

    if case.record_kind != TEACHING_CLASSROOM_SESSION_CASE_RECORD_KIND {
        errors.push(format!(
            "{where_}: case record_kind is {}",
            case.record_kind
        ));
    }
    if s.record_kind != TEACHING_SESSION_RECORD_KIND {
        errors.push(format!(
            "{where_}: session record_kind is {}",
            s.record_kind
        ));
    }
    if s.segments.is_empty() {
        errors.push(format!("{where_}: session has no segments"));
    }

    // Guardrails: teaching never widens authority or mints a hidden model.
    if s.grants_mutation_authority {
        errors.push(format!("{where_}: session grants mutation authority"));
    }
    if s.grants_terminal_or_debug_control {
        errors.push(format!("{where_}: session grants terminal/debug control"));
    }
    if s.grants_broader_authority_than_workspace {
        errors.push(format!(
            "{where_}: session widens authority beyond the workspace"
        ));
    }
    if s.establishes_private_data_ownership {
        errors.push(format!("{where_}: session claims private data ownership"));
    }
    if s.creates_hidden_progress_model {
        errors.push(format!("{where_}: session creates a hidden progress model"));
    }
    if s.creates_cohort_or_grading_flow {
        errors.push(format!("{where_}: session creates a cohort/grading flow"));
    }

    // Citations, resumability, disclosure, and fencing.
    if !s.segments_cite_learning_mode() {
        errors.push(format!(
            "{where_}: a segment does not cite a learning-mode object"
        ));
    }
    if !s.segments_are_resumable() {
        errors.push(format!(
            "{where_}: a segment is not resumable across restart/reconnect"
        ));
    }
    if !s.docs_pack_states_disclosed() {
        errors.push(format!(
            "{where_}: a degraded docs-pack state is missing its disclosure"
        ));
    }
    if !s.demonstrations_are_fenced() {
        errors.push(format!(
            "{where_}: a mutating demonstration is not properly fenced"
        ));
    }
    if !s.opt_in_markers_consistent() {
        errors.push(format!(
            "{where_}: replay/retention opt-in markers are inconsistent"
        ));
    }

    for seg in &s.segments {
        if seg.learning_object_ref.is_empty() || !seg.cites_learning_mode_object {
            errors.push(format!(
                "{where_}: segment {} does not reuse a learning-mode object",
                seg.segment_id
            ));
        }
        if !seg.has_citation() {
            errors.push(format!(
                "{where_}: segment {} carries no docs/graph/citation reference",
                seg.segment_id
            ));
        }
        if seg.docs_pack_state.requires_disclosure() && seg.docs_pack_disclosure_ref.is_none() {
            errors.push(format!(
                "{where_}: segment {} hides a degraded docs-pack state",
                seg.segment_id
            ));
        }
        if let Some(action) = &seg.demonstrated_action {
            if !action.reuses_ordinary_command_path {
                errors.push(format!(
                    "{where_}: demonstration {} bypasses the ordinary command path",
                    action.action_id
                ));
            }
            if action.mutates_workspace && !action.is_properly_fenced() {
                errors.push(format!(
                    "{where_}: mutating demonstration {} is missing an ordinary-work fence",
                    action.action_id
                ));
            }
            if !action.mutates_workspace && action.kind == DemonstrationKind::MutationThroughFences
            {
                errors.push(format!(
                    "{where_}: demonstration {} claims fences but does not mutate",
                    action.action_id
                ));
            }
        }
    }

    // The affordance projection must be the role-aware, keyboard-complete proof.
    let aff = &case.affordances;
    if aff.session_id != s.session_id {
        errors.push(format!(
            "{where_}: affordance projection session id diverges"
        ));
    }
    if !aff.keyboard_complete {
        errors.push(format!(
            "{where_}: affordance projection is not keyboard-complete"
        ));
    }
    if aff.pointer_only {
        errors.push(format!(
            "{where_}: affordance projection declares a pointer-only control"
        ));
    }
    if !aff.screen_reader_reachable {
        errors.push(format!(
            "{where_}: affordance projection is not screen-reader reachable"
        ));
    }
    if aff.exposes_terminal_or_debug_control {
        errors.push(format!(
            "{where_}: affordance projection exposes a terminal/debug control"
        ));
    }
    if aff.exposes_misleading_control {
        errors.push(format!(
            "{where_}: affordance projection exposes a misleading control"
        ));
    }
    if !aff.all_constrained_clients_join_safe {
        errors.push(format!("{where_}: a constrained client is not join-safe"));
    }

    for view in &aff.participant_views {
        // Only the moderator drives; observers and scribes never drive or mutate.
        if view.exposes_drive_control && !view.role.can_drive_session() {
            errors.push(format!(
                "{where_}: {} exposes a drive control without the moderator role",
                view.participant_id
            ));
        }
        if view.exposes_mutation_affordance && !view.role.may_expose_mutation_affordance() {
            errors.push(format!(
                "{where_}: {} exposes a mutation affordance its role may not",
                view.participant_id
            ));
        }
        if matches!(view.role, TeachingRole::Observer | TeachingRole::Scribe)
            && view.exposes_mutation_affordance
        {
            errors.push(format!(
                "{where_}: {} ({}) must not see a mutation affordance",
                view.participant_id,
                view.role.as_str()
            ));
        }
        if view.exposes_terminal_or_debug_control {
            errors.push(format!(
                "{where_}: {} exposes a terminal/debug control",
                view.participant_id
            ));
        }
        // A constrained client never sees drive or mutation controls.
        if view.client_class.is_constrained()
            && (view.exposes_drive_control || view.exposes_mutation_affordance)
        {
            errors.push(format!(
                "{where_}: constrained client {} sees a drive/mutation control",
                view.participant_id
            ));
        }
        for a in &view.affordances {
            if a.command_id.is_empty()
                || a.key_binding_ref.is_empty()
                || a.accessible_label_ref.is_empty()
            {
                errors.push(format!(
                    "{where_}: affordance {} is missing command/key/label metadata",
                    a.affordance_id
                ));
            }
            if a.is_terminal_or_debug_control {
                errors.push(format!(
                    "{where_}: affordance {} is a terminal/debug control",
                    a.affordance_id
                ));
            }
            if a.mutates_workspace && !a.routes_through_ordinary_command_path {
                errors.push(format!(
                    "{where_}: mutating affordance {} bypasses the ordinary command path",
                    a.affordance_id
                ));
            }
            if !a.actionable {
                errors.push(format!(
                    "{where_}: affordance {} is not actionable (broken control)",
                    a.affordance_id
                ));
            }
        }
    }
}
