//! Seeded presentation-mode corpus, support export, and validation.
//!
//! This is the mint-from-truth corpus for the beta presentation overlays. Each
//! case builds one [`PresentationSession`] and projects its reversible
//! [`PresentationOverlay`]; the checked-in fixtures under
//! `fixtures/help/m3/presentation_mode/` are a literal projection of
//! [`seeded_presentation_mode_corpus`], so the JSON cannot drift from the Rust
//! types.
//!
//! The corpus exercises every walkthrough surface kind, every leader/follow
//! state, every audience scope, every boundary label, both note scopes, and all
//! three restore triggers so the exit-gate contract — a thin, reversible layer
//! over existing surfaces with explicit follow/breakaway truth and local-only
//! notes — is proven rather than asserted.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::overlay::{project_overlay, PresentationOverlay};
use super::session::{
    restore_from_checkpoint, AudienceParticipant, AudienceScope, BoundaryLabel, FollowWaypoint,
    LayoutPreset, LeaderFollowState, ParticipantFollowState, ParticipantRole, PresentationSession,
    PresentationSessionBuilder, RestoreCheckpoint, RestoreOutcome, RestoreTrigger, SpeakerNote,
    SpeakerNoteScope, WalkthroughSurfaceKind, WaypointCompletionState,
    PRESENTATION_MODE_BETA_SCHEMA_VERSION, PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF,
    PRESENTATION_SESSION_RECORD_KIND,
};

/// Stable record kind for [`PresentationSessionCase`] payloads.
pub const PRESENTATION_SESSION_CASE_RECORD_KIND: &str = "shell_presentation_session_case_record";

/// Stable record kind for [`PresentationModeCorpus`] payloads.
pub const PRESENTATION_MODE_CORPUS_RECORD_KIND: &str = "shell_presentation_mode_corpus_record";

/// Stable record kind for [`PresentationModeSupportExport`] payloads.
pub const PRESENTATION_MODE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_presentation_mode_support_export_record";

/// Stable record kind for [`PresentationModeSupportExportRow`] payloads.
pub const PRESENTATION_MODE_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "shell_presentation_mode_support_export_row_record";

/// One seeded case: a scenario, the governed session, and its reversible
/// overlay projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationSessionCase {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub scenario_label: String,
    pub session: PresentationSession,
    pub overlay: PresentationOverlay,
}

/// Aggregate coverage summary for the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationModeCorpusSummary {
    pub session_count: u32,
    pub surface_kinds_covered: Vec<WalkthroughSurfaceKind>,
    pub leader_follow_states_covered: Vec<LeaderFollowState>,
    pub audience_scopes_covered: Vec<AudienceScope>,
    pub boundary_labels_covered: Vec<BoundaryLabel>,
    pub note_scopes_covered: Vec<SpeakerNoteScope>,
    pub restore_triggers_covered: Vec<RestoreTrigger>,
    pub all_sessions_notes_local_by_default: bool,
    pub all_shared_notes_explicit: bool,
    pub all_waypoints_reuse_existing_surfaces: bool,
    pub all_waypoints_preserve_provenance: bool,
    pub all_overlays_keyboard_complete: bool,
    pub no_authority_widening: bool,
    pub all_restores_match_checkpoint: bool,
}

/// The full seeded presentation-mode corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationModeCorpus {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub generated_at: String,
    pub summary: PresentationModeCorpusSummary,
    pub sessions: Vec<PresentationSessionCase>,
    pub restore_outcomes: Vec<RestoreOutcome>,
}

/// One support-safe row. Carries enums, counts, refs, and guardrail booleans —
/// never note bodies, step titles, scenario copy, or raw file paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationModeSupportExportRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub session_id: String,
    pub lifecycle_state: super::session::SessionLifecycleState,
    pub leader_follow_state: LeaderFollowState,
    pub audience_scope: AudienceScope,
    pub layout_preset: LayoutPreset,
    pub waypoint_count: u32,
    pub surface_kinds: Vec<WalkthroughSurfaceKind>,
    pub boundary_labels: Vec<BoundaryLabel>,
    pub local_note_count: u32,
    pub shared_note_count: u32,
    pub participant_count: u32,
    pub following_count: u32,
    pub broken_away_count: u32,
    pub external_guest_count: u32,
    pub grants_mutation_authority: bool,
    pub grants_control_authority: bool,
    pub establishes_private_data_ownership: bool,
    pub speaker_notes_default_local_only: bool,
    pub preserves_source_provenance: bool,
    pub reuses_existing_surfaces_only: bool,
    pub keyboard_complete: bool,
    pub pointer_only: bool,
    pub screen_reader_reachable: bool,
}

/// Support-export wrapper over the corpus. Privacy-safe by construction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationModeSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub export_id: String,
    pub generated_at: String,
    pub rows: Vec<PresentationModeSupportExportRow>,
    pub restore_outcomes: Vec<RestoreOutcome>,
    pub raw_private_material_excluded: bool,
}

impl PresentationModeSupportExport {
    /// Project a corpus into a support-safe export.
    pub fn from_corpus(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        corpus: &PresentationModeCorpus,
    ) -> Self {
        let rows = corpus
            .sessions
            .iter()
            .map(|case| {
                let s = &case.session;
                let surface_kinds: Vec<WalkthroughSurfaceKind> = s
                    .waypoints
                    .iter()
                    .map(|w| w.surface_kind)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect();
                let boundary_labels: Vec<BoundaryLabel> = s
                    .waypoints
                    .iter()
                    .map(|w| w.boundary_label)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect();
                let notes: Vec<&SpeakerNote> = s
                    .waypoints
                    .iter()
                    .filter_map(|w| w.speaker_note.as_ref())
                    .collect();
                let local_note_count =
                    notes.iter().filter(|n| n.scope.is_local_only()).count() as u32;
                let shared_note_count =
                    notes.iter().filter(|n| !n.scope.is_local_only()).count() as u32;
                let following_count = s
                    .audience_participants
                    .iter()
                    .filter(|p| p.follow_state == ParticipantFollowState::Following)
                    .count() as u32;
                let broken_away_count = s
                    .audience_participants
                    .iter()
                    .filter(|p| p.follow_state == ParticipantFollowState::BrokenAway)
                    .count() as u32;
                let external_guest_count = s
                    .audience_participants
                    .iter()
                    .filter(|p| p.is_external_guest)
                    .count() as u32;

                PresentationModeSupportExportRow {
                    record_kind: PRESENTATION_MODE_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
                    schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
                    shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
                    case_id: case.case_id.clone(),
                    session_id: s.session_id.clone(),
                    lifecycle_state: s.lifecycle_state,
                    leader_follow_state: s.leader_follow_state,
                    audience_scope: s.audience_scope,
                    layout_preset: s.layout_preset,
                    waypoint_count: s.waypoints.len() as u32,
                    surface_kinds,
                    boundary_labels,
                    local_note_count,
                    shared_note_count,
                    participant_count: s.audience_participants.len() as u32,
                    following_count,
                    broken_away_count,
                    external_guest_count,
                    grants_mutation_authority: s.grants_mutation_authority,
                    grants_control_authority: s.grants_control_authority,
                    establishes_private_data_ownership: s.establishes_private_data_ownership,
                    speaker_notes_default_local_only: s.speaker_notes_default_local_only,
                    preserves_source_provenance: s.preserves_source_provenance,
                    reuses_existing_surfaces_only: s.reuses_existing_surfaces_only,
                    keyboard_complete: case.overlay.keyboard_complete,
                    pointer_only: case.overlay.pointer_only,
                    screen_reader_reachable: case.overlay.screen_reader_reachable,
                }
            })
            .collect();

        Self {
            record_kind: PRESENTATION_MODE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
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
        checkpoint_id: format!("presentation:checkpoint:{id}"),
        prior_layout_ref: format!("window-topology:{id}:prior"),
        prior_focus_ref: format!("focus-chain:{id}:prior"),
        prior_panel_visibility_ref: format!("panel-visibility:{id}:prior"),
        accessibility_posture_ref: format!("a11y-posture:{id}:prior"),
        captured_at: at.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn waypoint(
    id: &str,
    ordinal: u32,
    step_title: &str,
    surface_kind: WalkthroughSurfaceKind,
    target_object_ref: &str,
    file_path_ref: Option<&str>,
    symbol_anchor_ref: Option<&str>,
    branch_workspace_ref: &str,
    boundary_label: BoundaryLabel,
    completion_state: WaypointCompletionState,
    speaker_note: Option<SpeakerNote>,
) -> FollowWaypoint {
    FollowWaypoint {
        waypoint_id: id.to_owned(),
        ordinal,
        step_title: step_title.to_owned(),
        surface_kind,
        target_object_ref: target_object_ref.to_owned(),
        file_path_ref: file_path_ref.map(str::to_owned),
        symbol_anchor_ref: symbol_anchor_ref.map(str::to_owned),
        branch_workspace_ref: branch_workspace_ref.to_owned(),
        boundary_label,
        zoom_layout_hint_ref: Some(format!("zoom-hint:{id}")),
        reveal_action_ref: Some(format!("reveal:{id}")),
        completion_state,
        speaker_note,
        reuses_existing_surface: true,
        creates_parallel_artifact: false,
    }
}

fn viewer(id: &str, follow: ParticipantFollowState, external: bool) -> AudienceParticipant {
    AudienceParticipant {
        participant_id: id.to_owned(),
        role_badge: ParticipantRole::Viewer,
        follow_state: follow,
        is_external_guest: external,
    }
}

fn co_presenter(id: &str, follow: ParticipantFollowState) -> AudienceParticipant {
    AudienceParticipant {
        participant_id: id.to_owned(),
        role_badge: ParticipantRole::CoPresenter,
        follow_state: follow,
        is_external_guest: false,
    }
}

fn presenter(id: &str) -> AudienceParticipant {
    AudienceParticipant {
        participant_id: id.to_owned(),
        role_badge: ParticipantRole::Presenter,
        follow_state: ParticipantFollowState::Following,
        is_external_guest: false,
    }
}

// ---- seed sessions --------------------------------------------------------

struct SeedCase {
    case_id: &'static str,
    scenario_label: &'static str,
    session: PresentationSession,
}

fn seed_sessions() -> Vec<SeedCase> {
    vec![
        // 1. Solo rehearsal: a presenter dry-runs a code-then-diff walkthrough.
        //    No audience, so every note stays local and cannot leak.
        SeedCase {
            case_id: "case:solo-rehearsal-local-notes",
            scenario_label:
                "A presenter rehearses a code-then-diff walkthrough alone. No audience is present, the focused-single layout is used, and both speaker notes stay local-only.",
            session: PresentationSessionBuilder::new(
                "presentation:session:solo-01",
                LeaderFollowState::Presenting,
                AudienceScope::SoloRehearsal,
                checkpoint("solo-01", "2026-05-20T09:00:00Z"),
            )
            .layout(LayoutPreset::FocusedSingle)
            .focus("wp:solo-01:1")
            .waypoint(waypoint(
                "wp:solo-01:1",
                1,
                "Walk the login entry point",
                WalkthroughSurfaceKind::Editor,
                "editor:buffer:auth-login",
                Some("src/auth/login.rs"),
                Some("fn authenticate"),
                "branch:main@workspace:local",
                BoundaryLabel::Local,
                WaypointCompletionState::Current,
                Some(SpeakerNote::local(
                    "note:solo-01:1",
                    "wp:solo-01:1",
                    "Remind the room this is the only entry point.",
                )),
            ))
            .waypoint(waypoint(
                "wp:solo-01:2",
                2,
                "Show the validation diff",
                WalkthroughSurfaceKind::Diff,
                "diff:review:auth-login-validation",
                Some("src/auth/login.rs"),
                None,
                "branch:main@workspace:local",
                BoundaryLabel::Local,
                WaypointCompletionState::Pending,
                Some(
                    SpeakerNote::local(
                        "note:solo-01:2",
                        "wp:solo-01:2",
                        "Call out the new bounds check before moving on.",
                    )
                    .with_next_step("Then jump to the topology map.")
                    .with_citations(vec!["docs:auth/validation".to_owned()]),
                ),
            ))
            .build(),
        },
        // 2. Shared workspace: a presenter leads a docs-then-graph review with an
        //    audience that is mostly following while one viewer browses alone.
        SeedCase {
            case_id: "case:shared-workspace-following-audience",
            scenario_label:
                "A presenter leads a docs-then-graph review inside the shared workspace. A co-presenter and a viewer follow; one viewer has broken away to browse independently. The split-compare layout keeps the docs and the remote topology side by side.",
            session: PresentationSessionBuilder::new(
                "presentation:session:shared-02",
                LeaderFollowState::Presenting,
                AudienceScope::SharedWorkspace,
                checkpoint("shared-02", "2026-05-20T09:10:00Z"),
            )
            .layout(LayoutPreset::SplitCompare)
            .focus("wp:shared-02:1")
            .participant(co_presenter("party:shared-02:co", ParticipantFollowState::Following))
            .participant(viewer("party:shared-02:v1", ParticipantFollowState::Following, false))
            .participant(viewer("party:shared-02:v2", ParticipantFollowState::BrokenAway, false))
            .waypoint(waypoint(
                "wp:shared-02:1",
                1,
                "Read the architecture overview",
                WalkthroughSurfaceKind::Docs,
                "docs:node:architecture-overview",
                None,
                None,
                "branch:main@workspace:shared",
                BoundaryLabel::Shared,
                WaypointCompletionState::Current,
                Some(SpeakerNote::local(
                    "note:shared-02:1",
                    "wp:shared-02:1",
                    "Skip the appendix; we cover it in the graph step.",
                )),
            ))
            .waypoint(waypoint(
                "wp:shared-02:2",
                2,
                "Trace the dependency topology",
                WalkthroughSurfaceKind::Graph,
                "graph:node:service-topology",
                None,
                None,
                "branch:main@workspace:remote-mirror",
                BoundaryLabel::Remote,
                WaypointCompletionState::Pending,
                None,
            ))
            .build(),
        },
        // 3. Invited guests: one note is intentionally shared, which sets its
        //    explicit promotion marker so the share decision stays auditable.
        SeedCase {
            case_id: "case:invited-guests-shared-note",
            scenario_label:
                "A presenter walks invited external guests through an editor-then-notebook narrative. One note is deliberately promoted to shared so guests can read it; the share is explicit and auditable. A second note stays local. One guest is following, one has requested to follow.",
            session: PresentationSessionBuilder::new(
                "presentation:session:guests-03",
                LeaderFollowState::Presenting,
                AudienceScope::InvitedGuests,
                checkpoint("guests-03", "2026-05-20T09:20:00Z"),
            )
            .layout(LayoutPreset::NarrativeWide)
            .focus("wp:guests-03:1")
            .participant(viewer("party:guests-03:g1", ParticipantFollowState::Following, true))
            .participant(viewer(
                "party:guests-03:g2",
                ParticipantFollowState::RequestingFollow,
                true,
            ))
            .waypoint(waypoint(
                "wp:guests-03:1",
                1,
                "Introduce the public API surface",
                WalkthroughSurfaceKind::Editor,
                "editor:buffer:public-api",
                Some("src/api/mod.rs"),
                Some("pub mod public"),
                "branch:main@workspace:local",
                BoundaryLabel::Local,
                WaypointCompletionState::Current,
                Some(SpeakerNote::shared(
                    "note:guests-03:1",
                    "wp:guests-03:1",
                    "Shared with guests: the API is stable for the beta.",
                )),
            ))
            .waypoint(waypoint(
                "wp:guests-03:2",
                2,
                "Run the worked example notebook",
                WalkthroughSurfaceKind::Notebook,
                "notebook:doc:worked-example",
                Some("examples/worked_example.ipynb"),
                None,
                "branch:main@workspace:local",
                BoundaryLabel::Local,
                WaypointCompletionState::Pending,
                Some(SpeakerNote::local(
                    "note:guests-03:2",
                    "wp:guests-03:2",
                    "Do not run the cell live if the kernel is cold.",
                )),
            ))
            .build(),
        },
        // 4. The local user has broken away from a presenter to inspect a diff
        //    independently. The breakaway banner must appear.
        SeedCase {
            case_id: "case:local-user-broken-away",
            scenario_label:
                "While another teammate presents, the local user breaks away to inspect a shared diff independently. The breakaway banner shows the independent-browsing state and offers a keyboard return to the presenter's anchor.",
            session: PresentationSessionBuilder::new(
                "presentation:session:breakaway-04",
                LeaderFollowState::BrokenAway,
                AudienceScope::SharedWorkspace,
                checkpoint("breakaway-04", "2026-05-20T09:30:00Z"),
            )
            .layout(LayoutPreset::InheritCurrent)
            .focus("wp:breakaway-04:1")
            .participant(presenter("party:breakaway-04:presenter"))
            .participant(viewer("party:breakaway-04:v1", ParticipantFollowState::Following, false))
            .waypoint(waypoint(
                "wp:breakaway-04:1",
                1,
                "Inspect the migration diff",
                WalkthroughSurfaceKind::Diff,
                "diff:review:migration-step",
                Some("src/migration/step.rs"),
                None,
                "branch:release@workspace:shared",
                BoundaryLabel::Shared,
                WaypointCompletionState::Current,
                None,
            ))
            .build(),
        },
        // 5. The local user is following another presenter through a remote graph.
        SeedCase {
            case_id: "case:following-presenter-graph",
            scenario_label:
                "The local user follows another presenter through a remote dependency graph. Navigation tracks the leader's anchor, and a local note keeps a private reminder.",
            session: PresentationSessionBuilder::new(
                "presentation:session:following-05",
                LeaderFollowState::FollowingPresenter,
                AudienceScope::SharedWorkspace,
                checkpoint("following-05", "2026-05-20T09:40:00Z"),
            )
            .layout(LayoutPreset::InheritCurrent)
            .focus("wp:following-05:1")
            .participant(presenter("party:following-05:presenter"))
            .participant(viewer("party:following-05:v1", ParticipantFollowState::Following, false))
            .waypoint(waypoint(
                "wp:following-05:1",
                1,
                "Follow the remote topology tour",
                WalkthroughSurfaceKind::Graph,
                "graph:node:remote-topology",
                None,
                None,
                "branch:main@workspace:remote-mirror",
                BoundaryLabel::Remote,
                WaypointCompletionState::Current,
                Some(SpeakerNote::local(
                    "note:following-05:1",
                    "wp:following-05:1",
                    "Ask about the cache edge after this node.",
                )),
            ))
            .build(),
        },
        // 6. The local user has requested to (re)join the presenter and is
        //    waiting to re-sync while keeping a remote notebook in view.
        SeedCase {
            case_id: "case:requesting-follow-resync",
            scenario_label:
                "After a brief detour, the local user requests to rejoin the presenter and waits to re-sync. The follow chip shows the requesting-follow state while a remote notebook stays in view.",
            session: PresentationSessionBuilder::new(
                "presentation:session:requesting-06",
                LeaderFollowState::RequestingFollow,
                AudienceScope::SharedWorkspace,
                checkpoint("requesting-06", "2026-05-20T09:50:00Z"),
            )
            .layout(LayoutPreset::InheritCurrent)
            .focus("wp:requesting-06:1")
            .participant(presenter("party:requesting-06:presenter"))
            .participant(viewer(
                "party:requesting-06:v1",
                ParticipantFollowState::Following,
                false,
            ))
            .waypoint(waypoint(
                "wp:requesting-06:1",
                1,
                "Catch up on the remote notebook",
                WalkthroughSurfaceKind::Notebook,
                "notebook:doc:remote-analysis",
                Some("analysis/remote_analysis.ipynb"),
                None,
                "branch:main@workspace:remote-mirror",
                BoundaryLabel::Remote,
                WaypointCompletionState::Current,
                Some(SpeakerNote::local(
                    "note:requesting-06:1",
                    "wp:requesting-06:1",
                    "Resume from the presenter's cell when sync lands.",
                )),
            ))
            .build(),
        },
    ]
}

/// The full seeded presentation-mode corpus.
pub fn seeded_presentation_mode_corpus() -> PresentationModeCorpus {
    let seeds = seed_sessions();
    let sessions: Vec<PresentationSessionCase> = seeds
        .iter()
        .map(|seed| PresentationSessionCase {
            record_kind: PRESENTATION_SESSION_CASE_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            case_id: seed.case_id.to_owned(),
            scenario_label: seed.scenario_label.to_owned(),
            session: seed.session.clone(),
            overlay: project_overlay(&seed.session),
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
            by_id("case:solo-rehearsal-local-notes"),
            RestoreTrigger::Exit,
        ),
        restore_from_checkpoint(
            by_id("case:shared-workspace-following-audience"),
            RestoreTrigger::CrashRecovery,
        ),
        restore_from_checkpoint(by_id("case:local-user-broken-away"), RestoreTrigger::Cancel),
    ];

    let summary = summarize(&sessions, &restore_outcomes);

    PresentationModeCorpus {
        record_kind: PRESENTATION_MODE_CORPUS_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        generated_at: "2026-05-20T00:00:00Z".to_owned(),
        summary,
        sessions,
        restore_outcomes,
    }
}

fn summarize(
    sessions: &[PresentationSessionCase],
    restore_outcomes: &[RestoreOutcome],
) -> PresentationModeCorpusSummary {
    let mut surface_kinds: BTreeSet<WalkthroughSurfaceKind> = BTreeSet::new();
    let mut leader_states: BTreeSet<LeaderFollowState> = BTreeSet::new();
    let mut audience_scopes: BTreeSet<AudienceScope> = BTreeSet::new();
    let mut boundary_labels: BTreeSet<BoundaryLabel> = BTreeSet::new();
    let mut note_scopes: BTreeSet<SpeakerNoteScope> = BTreeSet::new();
    let mut restore_triggers: BTreeSet<RestoreTrigger> = BTreeSet::new();

    let mut all_local_default = true;
    let mut all_shared_explicit = true;
    let mut all_reuse = true;
    let mut all_provenance = true;
    let mut all_keyboard = true;
    let mut no_widening = true;

    for case in sessions {
        let s = &case.session;
        leader_states.insert(s.leader_follow_state);
        audience_scopes.insert(s.audience_scope);
        all_local_default &= s.speaker_notes_default_local_only;
        all_shared_explicit &= s.shared_notes_are_explicit();
        all_reuse &= s.waypoints_reuse_existing_surfaces() && s.reuses_existing_surfaces_only;
        all_provenance &= s.waypoints_preserve_provenance() && s.preserves_source_provenance;
        no_widening &= !s.grants_mutation_authority
            && !s.grants_control_authority
            && !s.establishes_private_data_ownership;
        for w in &s.waypoints {
            surface_kinds.insert(w.surface_kind);
            boundary_labels.insert(w.boundary_label);
            if let Some(note) = &w.speaker_note {
                note_scopes.insert(note.scope);
            }
        }
        let overlay = &case.overlay;
        all_keyboard &=
            overlay.keyboard_complete && !overlay.pointer_only && overlay.screen_reader_reachable;
    }

    let mut all_restores_match = true;
    for outcome in restore_outcomes {
        restore_triggers.insert(outcome.trigger);
        all_restores_match &= outcome.matches_checkpoint && !outcome.left_in_improvised_shell;
    }

    PresentationModeCorpusSummary {
        session_count: sessions.len() as u32,
        surface_kinds_covered: surface_kinds.into_iter().collect(),
        leader_follow_states_covered: leader_states.into_iter().collect(),
        audience_scopes_covered: audience_scopes.into_iter().collect(),
        boundary_labels_covered: boundary_labels.into_iter().collect(),
        note_scopes_covered: note_scopes.into_iter().collect(),
        restore_triggers_covered: restore_triggers.into_iter().collect(),
        all_sessions_notes_local_by_default: all_local_default,
        all_shared_notes_explicit: all_shared_explicit,
        all_waypoints_reuse_existing_surfaces: all_reuse,
        all_waypoints_preserve_provenance: all_provenance,
        all_overlays_keyboard_complete: all_keyboard,
        no_authority_widening: no_widening,
        all_restores_match_checkpoint: all_restores_match,
    }
}

// ---- validation -----------------------------------------------------------

/// Validate the corpus invariants. Returns the list of violations; an empty
/// list means the corpus conforms.
pub fn validate_presentation_mode_corpus(
    corpus: &PresentationModeCorpus,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if corpus.record_kind != PRESENTATION_MODE_CORPUS_RECORD_KIND {
        errors.push(format!("corpus record_kind is {}", corpus.record_kind));
    }
    if corpus.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION {
        errors.push(format!(
            "corpus schema_version is {}",
            corpus.schema_version
        ));
    }
    if corpus.shared_contract_ref != PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF {
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

    // Coverage: every surface kind, leader/follow state, audience scope,
    // boundary label, both note scopes, and all three restore triggers.
    for kind in [
        WalkthroughSurfaceKind::Editor,
        WalkthroughSurfaceKind::Diff,
        WalkthroughSurfaceKind::Docs,
        WalkthroughSurfaceKind::Graph,
        WalkthroughSurfaceKind::Notebook,
    ] {
        if !corpus.summary.surface_kinds_covered.contains(&kind) {
            errors.push(format!(
                "corpus does not cover surface kind {}",
                kind.as_str()
            ));
        }
    }
    for state in [
        LeaderFollowState::Presenting,
        LeaderFollowState::FollowingPresenter,
        LeaderFollowState::BrokenAway,
        LeaderFollowState::RequestingFollow,
    ] {
        if !corpus.summary.leader_follow_states_covered.contains(&state) {
            errors.push(format!(
                "corpus does not cover leader/follow state {}",
                state.as_str()
            ));
        }
    }
    for scope in [
        AudienceScope::SoloRehearsal,
        AudienceScope::SharedWorkspace,
        AudienceScope::InvitedGuests,
    ] {
        if !corpus.summary.audience_scopes_covered.contains(&scope) {
            errors.push(format!(
                "corpus does not cover audience scope {}",
                scope.as_str()
            ));
        }
    }
    for label in [
        BoundaryLabel::Local,
        BoundaryLabel::Remote,
        BoundaryLabel::Shared,
    ] {
        if !corpus.summary.boundary_labels_covered.contains(&label) {
            errors.push(format!(
                "corpus does not cover boundary label {}",
                label.as_str()
            ));
        }
    }
    for scope in [SpeakerNoteScope::Local, SpeakerNoteScope::Shared] {
        if !corpus.summary.note_scopes_covered.contains(&scope) {
            errors.push(format!(
                "corpus does not cover note scope {}",
                scope.as_str()
            ));
        }
    }
    for trigger in [
        RestoreTrigger::Exit,
        RestoreTrigger::Cancel,
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

fn validate_case(case: &PresentationSessionCase, errors: &mut Vec<String>) {
    let where_ = &case.case_id;
    let s = &case.session;

    if case.record_kind != PRESENTATION_SESSION_CASE_RECORD_KIND {
        errors.push(format!(
            "{where_}: case record_kind is {}",
            case.record_kind
        ));
    }
    if s.record_kind != PRESENTATION_SESSION_RECORD_KIND {
        errors.push(format!(
            "{where_}: session record_kind is {}",
            s.record_kind
        ));
    }

    // Guardrails: presentation mode never widens authority or claims ownership.
    if s.grants_mutation_authority {
        errors.push(format!("{where_}: session grants mutation authority"));
    }
    if s.grants_control_authority {
        errors.push(format!("{where_}: session grants control authority"));
    }
    if s.establishes_private_data_ownership {
        errors.push(format!("{where_}: session claims private data ownership"));
    }

    // Notes default local-only; shared notes must be explicit.
    if !s.speaker_notes_default_local_only {
        errors.push(format!(
            "{where_}: session does not default notes local-only"
        ));
    }
    if !s.shared_notes_are_explicit() {
        errors.push(format!(
            "{where_}: a shared note is missing an explicit promotion"
        ));
    }

    // Surfaces are reused, never duplicated, and provenance survives.
    if !s.waypoints_reuse_existing_surfaces() {
        errors.push(format!(
            "{where_}: a waypoint does not reuse an existing surface"
        ));
    }
    if !s.waypoints_preserve_provenance() {
        errors.push(format!("{where_}: a waypoint drops source provenance"));
    }
    for w in &s.waypoints {
        if !w.reuses_existing_surface || w.creates_parallel_artifact {
            errors.push(format!(
                "{where_}: waypoint {} duplicates a surface instead of reusing one",
                w.waypoint_id
            ));
        }
        if let Some(note) = &w.speaker_note {
            if note.linked_waypoint_ref != w.waypoint_id {
                errors.push(format!(
                    "{where_}: note {} links to the wrong waypoint",
                    note.note_id
                ));
            }
            if note.scope == SpeakerNoteScope::Shared && !note.shared_promotion_explicit {
                errors.push(format!(
                    "{where_}: shared note {} is not explicitly promoted",
                    note.note_id
                ));
            }
            if note.scope == SpeakerNoteScope::Local && note.shared_promotion_explicit {
                errors.push(format!(
                    "{where_}: local note {} is incorrectly marked promoted",
                    note.note_id
                ));
            }
        }
    }

    // The overlay must be the reversible, keyboard-complete projection.
    let overlay = &case.overlay;
    if overlay.session_id != s.session_id {
        errors.push(format!(
            "{where_}: overlay session id diverges from the session"
        ));
    }
    if !overlay.keyboard_complete {
        errors.push(format!("{where_}: overlay is not keyboard-complete"));
    }
    if overlay.pointer_only {
        errors.push(format!("{where_}: overlay declares a pointer-only control"));
    }
    if !overlay.screen_reader_reachable {
        errors.push(format!("{where_}: overlay is not screen-reader reachable"));
    }
    if !overlay.provenance_strip.source_identity_preserved {
        errors.push(format!("{where_}: overlay drops the provenance strip"));
    }
    if !(overlay.restore_affordance.restores_prior_layout
        && overlay.restore_affordance.restores_prior_focus
        && overlay.restore_affordance.restores_on_crash_recovery)
    {
        errors.push(format!(
            "{where_}: overlay restore affordance is incomplete"
        ));
    }

    // Every keyboard action is reachable and attention-only (never mutating).
    for action in overlay.all_actions() {
        if action.command_id.is_empty()
            || action.key_binding_ref.is_empty()
            || action.accessible_label.is_empty()
        {
            errors.push(format!(
                "{where_}: action {} is missing command/key/accessible metadata",
                action.action_id
            ));
        }
        if action.is_destructive {
            errors.push(format!(
                "{where_}: action {} is destructive — presentation actions must not be",
                action.action_id
            ));
        }
        if action.mutates_workspace {
            errors.push(format!(
                "{where_}: action {} mutates the workspace — presentation actions must not",
                action.action_id
            ));
        }
    }

    // A broken-away local user must see the breakaway banner; a non-broken-away
    // user must not.
    let broken_away = s.leader_follow_state.is_broken_away();
    if broken_away && overlay.breakaway_banner.is_none() {
        errors.push(format!(
            "{where_}: broken-away session is missing the breakaway banner"
        ));
    }
    if !broken_away && overlay.breakaway_banner.is_some() {
        errors.push(format!(
            "{where_}: non-broken-away session shows a breakaway banner"
        ));
    }
}
