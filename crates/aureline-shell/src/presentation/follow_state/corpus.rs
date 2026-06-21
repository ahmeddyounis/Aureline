//! Seeded cross-client follow-state corpus, support export, and validation.
//!
//! Each case builds one [`PresentationSession`] plus a set of
//! [`ClientFollowInput`]s and projects the [`FollowStateTruth`] packet that
//! proves desktop, browser, and companion clients share one follow vocabulary
//! and the same recovery actions. The checked-in fixtures under
//! `fixtures/presentation/browser-and-companion-follow/` are a literal
//! projection of [`seeded_follow_state_corpus`], so the JSON cannot drift from
//! the Rust types.
//!
//! The corpus deliberately covers a fully live session (desktop presenting,
//! browser and companion following live), a mixed independent session (browser
//! broken away with a durable banner, companion requesting follow), a
//! cached-snapshot session (the companion is offline and shows a self-labeled
//! snapshot while the desktop keeps presenting), and a take-over session (a
//! browser co-presenter requests take-over while still seeing the live route) —
//! so live / independent / cached-snapshot honesty and take-over are proven
//! across clients rather than asserted.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::presentation_mode::{
    AudienceScope, BoundaryLabel, FollowWaypoint, LeaderFollowState, PresentationSession,
    PresentationSessionBuilder, RestoreCheckpoint, WalkthroughSurfaceKind, WaypointCompletionState,
    PRESENTATION_MODE_BETA_SCHEMA_VERSION, PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF,
};

use super::state::{
    project_follow_state_truth, ClientFollowInput, ClientSurface, FollowMode,
    FollowStateSupportExport, FollowStateTruth, FollowStateViolation, LivenessClass,
    SnapshotIdentity, SnapshotStalenessReason,
};

/// Stable record kind for [`FollowStateCase`] payloads.
pub const FOLLOW_STATE_CASE_RECORD_KIND: &str = "presentation_follow_state_case_record";

/// Stable record kind for [`FollowStateCorpus`] payloads.
pub const FOLLOW_STATE_CORPUS_RECORD_KIND: &str = "presentation_follow_state_corpus_record";

/// One seeded case: a scenario plus the projected cross-client follow packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowStateCase {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable case id.
    pub case_id: String,
    /// Human-readable scenario label.
    pub scenario_label: String,
    /// The projected cross-client follow-state packet.
    pub truth: FollowStateTruth,
}

/// Aggregate coverage summary for the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowStateSummary {
    /// Number of cases.
    pub case_count: u32,
    /// Distinct client surfaces covered across the corpus.
    pub client_surfaces_covered: Vec<ClientSurface>,
    /// Distinct follow modes covered across the corpus.
    pub follow_modes_covered: Vec<FollowMode>,
    /// Distinct liveness classes covered across the corpus.
    pub liveness_classes_covered: Vec<LivenessClass>,
    /// True when every view in every case is internally consistent.
    pub all_views_consistent: bool,
    /// True when every case keeps the recovery vocabulary at parity.
    pub all_recovery_vocabulary_parity: bool,
    /// True when every breakaway banner across the corpus is durable.
    pub all_breakaway_banners_durable: bool,
    /// True when no cached snapshot anywhere claims to be a live route.
    pub no_snapshot_implies_live: bool,
    /// True when no view anywhere infers its state from drift / timing / toast.
    pub no_inferred_state: bool,
    /// True when no case widens mutation or control authority.
    pub no_authority_widening: bool,
    /// True when at least one case demonstrates a self-labeled cached snapshot.
    pub cached_snapshot_demonstrated: bool,
    /// True when at least one case demonstrates a durable breakaway banner.
    pub breakaway_demonstrated: bool,
    /// True when at least one case demonstrates an explicit take-over request.
    pub take_over_demonstrated: bool,
}

/// The full seeded cross-client follow-state corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowStateCorpus {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Mint timestamp.
    pub generated_at: String,
    /// Coverage summary.
    pub summary: FollowStateSummary,
    /// Per-scenario cases.
    pub cases: Vec<FollowStateCase>,
}

impl FollowStateCorpus {
    /// Every projected packet across the corpus, in case order.
    pub fn all_packets(&self) -> impl Iterator<Item = &FollowStateTruth> {
        self.cases.iter().map(|case| &case.truth)
    }
}

/// Errors emitted by [`validate_follow_state_corpus`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FollowStateCorpusError {
    /// The corpus carried the wrong record kind or schema version.
    MalformedCorpus,
    /// A case carried the wrong record kind or schema version.
    MalformedCase {
        /// The offending case id.
        case_id: String,
    },
    /// A case's packet failed validation.
    CaseInvalid {
        /// The offending case id.
        case_id: String,
        /// The violations the packet emitted.
        violations: Vec<FollowStateViolation>,
    },
    /// The summary did not match the cases it claims to summarize.
    SummaryMismatch,
    /// No case demonstrated a self-labeled cached snapshot.
    CachedSnapshotNotDemonstrated,
    /// No case demonstrated an explicit take-over request.
    TakeOverNotDemonstrated,
}

/// Validate the seeded cross-client follow-state corpus.
pub fn validate_follow_state_corpus(
    corpus: &FollowStateCorpus,
) -> Result<(), FollowStateCorpusError> {
    if corpus.record_kind != FOLLOW_STATE_CORPUS_RECORD_KIND
        || corpus.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
    {
        return Err(FollowStateCorpusError::MalformedCorpus);
    }

    for case in &corpus.cases {
        if case.record_kind != FOLLOW_STATE_CASE_RECORD_KIND
            || case.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
        {
            return Err(FollowStateCorpusError::MalformedCase {
                case_id: case.case_id.clone(),
            });
        }
        let violations = case.truth.validate();
        if !violations.is_empty() {
            return Err(FollowStateCorpusError::CaseInvalid {
                case_id: case.case_id.clone(),
                violations,
            });
        }
    }

    let expected = summarize(&corpus.cases);
    if expected != corpus.summary {
        return Err(FollowStateCorpusError::SummaryMismatch);
    }
    if !corpus.summary.cached_snapshot_demonstrated {
        return Err(FollowStateCorpusError::CachedSnapshotNotDemonstrated);
    }
    if !corpus.summary.take_over_demonstrated {
        return Err(FollowStateCorpusError::TakeOverNotDemonstrated);
    }
    Ok(())
}

/// Project a corpus into a support-safe export over every client view.
pub fn follow_state_support_export(
    export_id: impl Into<String>,
    generated_at: impl Into<String>,
    corpus: &FollowStateCorpus,
) -> FollowStateSupportExport {
    FollowStateSupportExport::from_packets(export_id, generated_at, corpus.all_packets())
}

fn summarize(cases: &[FollowStateCase]) -> FollowStateSummary {
    let mut surfaces: BTreeSet<ClientSurface> = BTreeSet::new();
    let mut modes: BTreeSet<FollowMode> = BTreeSet::new();
    let mut liveness: BTreeSet<LivenessClass> = BTreeSet::new();
    let mut all_views_consistent = true;
    let mut all_recovery_vocabulary_parity = true;
    let mut all_breakaway_banners_durable = true;
    let mut no_snapshot_implies_live = true;
    let mut no_inferred_state = true;
    let mut no_authority_widening = true;
    let mut cached_snapshot_demonstrated = false;
    let mut breakaway_demonstrated = false;
    let mut take_over_demonstrated = false;

    for case in cases {
        let truth = &case.truth;
        all_recovery_vocabulary_parity &=
            truth.recovery_actions_parity_across_clients && truth.vocabulary_parity_across_clients;
        all_breakaway_banners_durable &= truth.breakaway_banner_durable;
        no_snapshot_implies_live &= truth.no_snapshot_implies_live;
        no_inferred_state &= truth.no_state_from_viewport_drift
            && truth.no_state_from_connection_timing
            && truth.no_transient_toast_only_state;
        if truth.grants_mutation_authority || truth.grants_control_authority {
            no_authority_widening = false;
        }
        for view in &truth.client_views {
            surfaces.insert(view.client_surface);
            modes.insert(view.follow_mode);
            liveness.insert(view.liveness);
            all_views_consistent &= view.is_consistent();
            if view.breakaway_banner.is_some() {
                breakaway_demonstrated = true;
            }
            if view.snapshot_identity.is_some() {
                cached_snapshot_demonstrated = true;
            }
            if view.follow_mode == FollowMode::RequestingTakeOver {
                take_over_demonstrated = true;
            }
        }
    }

    FollowStateSummary {
        case_count: cases.len() as u32,
        client_surfaces_covered: surfaces.into_iter().collect(),
        follow_modes_covered: modes.into_iter().collect(),
        liveness_classes_covered: liveness.into_iter().collect(),
        all_views_consistent,
        all_recovery_vocabulary_parity,
        all_breakaway_banners_durable,
        no_snapshot_implies_live,
        no_inferred_state,
        no_authority_widening,
        cached_snapshot_demonstrated,
        breakaway_demonstrated,
        take_over_demonstrated,
    }
}

// ---- builders -------------------------------------------------------------

fn checkpoint(id: &str) -> RestoreCheckpoint {
    RestoreCheckpoint {
        checkpoint_id: format!("presentation:checkpoint:{id}"),
        prior_layout_ref: format!("window-topology:{id}:prior"),
        prior_focus_ref: format!("focus-chain:{id}:prior"),
        prior_panel_visibility_ref: format!("panel-visibility:{id}:prior"),
        accessibility_posture_ref: format!("a11y-posture:{id}:prior"),
        captured_at: "2026-06-20T09:00:00Z".to_owned(),
    }
}

fn waypoint(id: &str, title: &str, boundary: BoundaryLabel) -> FollowWaypoint {
    FollowWaypoint {
        waypoint_id: id.to_owned(),
        ordinal: 1,
        step_title: title.to_owned(),
        surface_kind: WalkthroughSurfaceKind::Editor,
        target_object_ref: format!("obj:{id}"),
        file_path_ref: Some(
            "crates/aureline-shell/src/presentation/follow_state/state.rs".to_owned(),
        ),
        symbol_anchor_ref: Some("fn project_follow_state_truth".to_owned()),
        branch_workspace_ref: "branch:main@workspace:local".to_owned(),
        boundary_label: boundary,
        zoom_layout_hint_ref: None,
        reveal_action_ref: None,
        completion_state: WaypointCompletionState::Current,
        speaker_note: None,
        reuses_existing_surface: true,
        creates_parallel_artifact: false,
    }
}

fn session(id: &str, boundary: BoundaryLabel, audience: AudienceScope) -> PresentationSession {
    let wp_id = format!("wp:{id}:1");
    PresentationSessionBuilder::new(
        format!("presentation:session:follow:{id}"),
        LeaderFollowState::Presenting,
        audience,
        checkpoint(id),
    )
    .focus(wp_id.clone())
    .waypoint(waypoint(&wp_id, "Anchor the walkthrough", boundary))
    .build()
}

fn case(case_id: &str, scenario: &str, truth: FollowStateTruth) -> FollowStateCase {
    FollowStateCase {
        record_kind: FOLLOW_STATE_CASE_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        case_id: case_id.to_owned(),
        scenario_label: scenario.to_owned(),
        truth,
    }
}

fn all_live_case() -> FollowStateCase {
    let session = session(
        "all_live",
        BoundaryLabel::Shared,
        AudienceScope::SharedWorkspace,
    );
    let inputs = [
        ClientFollowInput::presenting(ClientSurface::Desktop),
        ClientFollowInput::following(ClientSurface::Browser),
        ClientFollowInput::following(ClientSurface::Companion),
    ];
    case(
        "follow-case:all-live-cross-client",
        "A live session observed from every client: the desktop presents while \
         the browser and companion follow the live route. All three share one \
         follow vocabulary and read as live.",
        project_follow_state_truth(&session, &inputs),
    )
}

fn mixed_independent_case() -> FollowStateCase {
    let session = session(
        "mixed_independent",
        BoundaryLabel::Shared,
        AudienceScope::SharedWorkspace,
    );
    let inputs = [
        ClientFollowInput::presenting(ClientSurface::Desktop),
        ClientFollowInput::broken_away(ClientSurface::Browser, "obj:browser-detour"),
        ClientFollowInput::requesting_follow(ClientSurface::Companion, "obj:companion-detour"),
    ];
    case(
        "follow-case:mixed-independent",
        "A mixed session: the browser has broken away to browse independently \
         behind a durable banner with a return-to-presenter path, while the \
         companion has requested follow and is waiting to resync. Neither reads \
         as live.",
        project_follow_state_truth(&session, &inputs),
    )
}

fn cached_snapshot_case() -> FollowStateCase {
    let session = session(
        "cached_snapshot",
        BoundaryLabel::Shared,
        AudienceScope::SharedWorkspace,
    );
    let identity = SnapshotIdentity::new(
        "snapshot:companion:captured:2026-06-20T09:05:00Z",
        SnapshotStalenessReason::ProviderOffline,
        true,
    );
    let inputs = [
        ClientFollowInput::presenting(ClientSurface::Desktop),
        ClientFollowInput::following(ClientSurface::Browser),
        ClientFollowInput::cached_snapshot(
            ClientSurface::Companion,
            "obj:companion-cached",
            identity,
        ),
    ];
    case(
        "follow-case:companion-cached-snapshot",
        "The provider went offline for the companion, which now shows a \
         self-labeled cached snapshot instead of pretending to be live. The \
         desktop keeps presenting and the browser keeps following the live route.",
        project_follow_state_truth(&session, &inputs),
    )
}

fn take_over_case() -> FollowStateCase {
    let session = session(
        "take_over",
        BoundaryLabel::Shared,
        AudienceScope::SharedWorkspace,
    );
    let inputs = [
        ClientFollowInput::presenting(ClientSurface::Desktop),
        ClientFollowInput::requesting_take_over(ClientSurface::Browser),
        ClientFollowInput::following(ClientSurface::Companion),
    ];
    case(
        "follow-case:browser-take-over-request",
        "A browser co-presenter explicitly requests to take over while still \
         seeing the presenter's live route; the request is a distinct, \
         attributable state, not an inferred control grab.",
        project_follow_state_truth(&session, &inputs),
    )
}

/// Build the full seeded cross-client follow-state corpus.
pub fn seeded_follow_state_corpus() -> FollowStateCorpus {
    let cases = vec![
        all_live_case(),
        mixed_independent_case(),
        cached_snapshot_case(),
        take_over_case(),
    ];
    let summary = summarize(&cases);
    FollowStateCorpus {
        record_kind: FOLLOW_STATE_CORPUS_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        generated_at: "2026-06-20T00:00:00Z".to_owned(),
        summary,
        cases,
    }
}
