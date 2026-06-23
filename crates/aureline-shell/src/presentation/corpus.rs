//! Seeded overlay/navigation binding corpus, support export, and validation.
//!
//! Each case builds one [`PresentationSession`] and a live
//! [`ZoneRegistryLayout`], then projects the
//! [`PresentationOverlayNavigationBinding`] that proves the overlay sits on the
//! existing pane-and-navigation system. The checked-in fixtures under
//! `fixtures/presentation/overlay-and-waypoint/` are a literal projection of
//! [`seeded_overlay_navigation_corpus`], so the JSON cannot drift from the Rust
//! types.
//!
//! The corpus deliberately covers an expanded-desktop presenting rehearsal, a
//! shared-workspace breakaway (banner present), and a compact-desktop session
//! where the collapsed sidebar forces the waypoint rail to float into the
//! transient overlay zone — so the thin-overlay, provenance-preserved, and
//! reversible-restore contract is proven across layouts rather than asserted.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::layout::zone_registry::{
    ZoneDefaults, ZoneRegistry, ZoneRegistryInput, ZoneRegistryLayout,
};
use crate::presentation_mode::{
    AudienceParticipant, AudienceScope, BoundaryLabel, FollowWaypoint, LeaderFollowState,
    ParticipantFollowState, ParticipantRole, PresentationSession, PresentationSessionBuilder,
    RestoreCheckpoint, SpeakerNote, WalkthroughSurfaceKind, WaypointCompletionState,
    PRESENTATION_MODE_BETA_SCHEMA_VERSION, PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF,
};

use super::binding::{
    project_overlay_navigation_binding, OverlaySurfaceTag, PresentationBindingViolation,
    PresentationOverlayNavigationBinding, ShellZoneTag,
};

/// Stable record kind for [`PresentationOverlayBindingCase`] payloads.
pub const PRESENTATION_OVERLAY_BINDING_CASE_RECORD_KIND: &str =
    "shell_presentation_overlay_binding_case_record";

/// Stable record kind for [`PresentationOverlayBindingCorpus`] payloads.
pub const PRESENTATION_OVERLAY_BINDING_CORPUS_RECORD_KIND: &str =
    "shell_presentation_overlay_binding_corpus_record";

/// Stable record kind for [`PresentationOverlayBindingSupportExport`] payloads.
pub const PRESENTATION_OVERLAY_BINDING_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_presentation_overlay_binding_support_export_record";

/// Stable record kind for [`PresentationOverlayBindingSupportExportRow`] payloads.
pub const PRESENTATION_OVERLAY_BINDING_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "shell_presentation_overlay_binding_support_export_row_record";

/// One seeded case: a scenario plus the projected overlay/navigation binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationOverlayBindingCase {
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
    /// The projected overlay/navigation binding.
    pub binding: PresentationOverlayNavigationBinding,
}

/// Aggregate coverage summary for the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationOverlayBindingSummary {
    /// Number of cases.
    pub case_count: u32,
    /// Distinct overlay surfaces bound across the corpus.
    pub surfaces_covered: Vec<OverlaySurfaceTag>,
    /// Distinct host zones used across the corpus.
    pub host_zones_covered: Vec<ShellZoneTag>,
    /// True when every case keeps the panes (no replacement).
    pub all_panes_preserved: bool,
    /// True when every case keeps source provenance visible.
    pub all_provenance_preserved: bool,
    /// True when every overlay is keyboard complete.
    pub all_keyboard_complete: bool,
    /// True when every case restores the checkpoint under all triggers.
    pub all_checkpoints_restore: bool,
    /// True when no case widens mutation or control authority.
    pub no_authority_widening: bool,
    /// True when at least one case demonstrates a floated fallback placement.
    pub fallback_placement_demonstrated: bool,
}

/// The full seeded overlay/navigation binding corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationOverlayBindingCorpus {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Mint timestamp.
    pub generated_at: String,
    /// Coverage summary.
    pub summary: PresentationOverlayBindingSummary,
    /// Per-scenario cases.
    pub cases: Vec<PresentationOverlayBindingCase>,
}

/// One support-safe row. Carries enums, counts, refs, and booleans — never file
/// paths, symbol anchors, accessible labels, or other raw provenance bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationOverlayBindingSupportExportRow {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Case id.
    pub case_id: String,
    /// Session id.
    pub session_id: String,
    /// Leader / follow state.
    pub leader_follow_state: LeaderFollowState,
    /// Audience scope.
    pub audience_scope: AudienceScope,
    /// Responsive adaptive class.
    pub adaptive_class: String,
    /// Number of overlay placements.
    pub placement_count: u32,
    /// Surfaces bound.
    pub surfaces: Vec<OverlaySurfaceTag>,
    /// Host zones used.
    pub host_zones: Vec<ShellZoneTag>,
    /// Number of floated fallback placements.
    pub fallback_placement_count: u32,
    /// Boundary label of the current anchor.
    pub boundary_label: BoundaryLabel,
    /// Whether provenance stays visible under the overlay.
    pub provenance_visible: bool,
    /// Whether the overlay preserves the pane-and-navigation system.
    pub preserves_pane_and_navigation_system: bool,
    /// Whether every control is keyboard reachable.
    pub keyboard_complete: bool,
    /// Whether anything is pointer-only.
    pub pointer_only: bool,
    /// Whether every surface is screen-reader reachable.
    pub screen_reader_reachable: bool,
    /// Whether the checkpoint restores under all triggers.
    pub restores_under_all_triggers: bool,
    /// Whether the binding widens mutation authority.
    pub grants_mutation_authority: bool,
    /// Whether the binding widens control authority.
    pub grants_control_authority: bool,
}

/// Support-export wrapper over the corpus. Privacy-safe by construction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationOverlayBindingSupportExport {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Export id.
    pub export_id: String,
    /// Mint timestamp.
    pub generated_at: String,
    /// Support-safe rows.
    pub rows: Vec<PresentationOverlayBindingSupportExportRow>,
    /// Always `true`: raw provenance bodies are excluded.
    pub raw_private_material_excluded: bool,
}

impl PresentationOverlayBindingSupportExport {
    /// Project a corpus into a support-safe export.
    pub fn from_corpus(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        corpus: &PresentationOverlayBindingCorpus,
    ) -> Self {
        let rows = corpus
            .cases
            .iter()
            .map(|case| {
                let b = &case.binding;
                let surfaces: Vec<OverlaySurfaceTag> = b
                    .placements
                    .iter()
                    .map(|p| p.surface)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect();
                let host_zones: Vec<ShellZoneTag> = b
                    .placements
                    .iter()
                    .map(|p| p.host_zone)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect();
                let fallback_placement_count = b
                    .placements
                    .iter()
                    .filter(|p| p.is_fallback_placement)
                    .count() as u32;
                PresentationOverlayBindingSupportExportRow {
                    record_kind: PRESENTATION_OVERLAY_BINDING_SUPPORT_EXPORT_ROW_RECORD_KIND
                        .to_owned(),
                    schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
                    shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
                    case_id: case.case_id.clone(),
                    session_id: b.session_id.clone(),
                    leader_follow_state: b.leader_follow_state,
                    audience_scope: b.audience_scope,
                    adaptive_class: b.adaptive_class.clone(),
                    placement_count: b.placements.len() as u32,
                    surfaces,
                    host_zones,
                    fallback_placement_count,
                    boundary_label: b.provenance.boundary_label,
                    provenance_visible: b.provenance.provenance_visible_under_overlay,
                    preserves_pane_and_navigation_system: b.preserves_pane_and_navigation_system,
                    keyboard_complete: b.keyboard_complete,
                    pointer_only: b.pointer_only,
                    screen_reader_reachable: b.screen_reader_reachable,
                    restores_under_all_triggers: b.checkpoint.restores_under_all_triggers,
                    grants_mutation_authority: b.grants_mutation_authority,
                    grants_control_authority: b.grants_control_authority,
                }
            })
            .collect();
        Self {
            record_kind: PRESENTATION_OVERLAY_BINDING_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            rows,
            raw_private_material_excluded: true,
        }
    }
}

/// Errors emitted by [`validate_overlay_navigation_corpus`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverlayBindingCorpusError {
    /// The corpus carried the wrong record kind or schema version.
    MalformedCorpus,
    /// A case's binding failed validation.
    CaseInvalid {
        /// The offending case id.
        case_id: String,
        /// The violations the binding emitted.
        violations: Vec<PresentationBindingViolation>,
    },
    /// The summary did not match the cases it claims to summarize.
    SummaryMismatch,
    /// No case demonstrated a floated fallback placement.
    FallbackNotDemonstrated,
}

/// Validate the seeded overlay/navigation binding corpus.
pub fn validate_overlay_navigation_corpus(
    corpus: &PresentationOverlayBindingCorpus,
) -> Result<(), OverlayBindingCorpusError> {
    if corpus.record_kind != PRESENTATION_OVERLAY_BINDING_CORPUS_RECORD_KIND
        || corpus.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
    {
        return Err(OverlayBindingCorpusError::MalformedCorpus);
    }

    for case in &corpus.cases {
        let violations = case.binding.validate();
        if !violations.is_empty() {
            return Err(OverlayBindingCorpusError::CaseInvalid {
                case_id: case.case_id.clone(),
                violations,
            });
        }
    }

    let expected = summarize(&corpus.cases);
    if expected != corpus.summary {
        return Err(OverlayBindingCorpusError::SummaryMismatch);
    }
    if !corpus.summary.fallback_placement_demonstrated {
        return Err(OverlayBindingCorpusError::FallbackNotDemonstrated);
    }
    Ok(())
}

fn summarize(cases: &[PresentationOverlayBindingCase]) -> PresentationOverlayBindingSummary {
    let mut surfaces: BTreeSet<OverlaySurfaceTag> = BTreeSet::new();
    let mut host_zones: BTreeSet<ShellZoneTag> = BTreeSet::new();
    let mut all_panes_preserved = true;
    let mut all_provenance_preserved = true;
    let mut all_keyboard_complete = true;
    let mut all_checkpoints_restore = true;
    let mut no_authority_widening = true;
    let mut fallback_placement_demonstrated = false;

    for case in cases {
        let b = &case.binding;
        for placement in &b.placements {
            surfaces.insert(placement.surface);
            host_zones.insert(placement.host_zone);
            if !placement.pane_preserved() {
                all_panes_preserved = false;
            }
            if placement.is_fallback_placement {
                fallback_placement_demonstrated = true;
            }
        }
        all_provenance_preserved &= b.provenance.is_preserved();
        all_keyboard_complete &= b.keyboard_complete && !b.pointer_only;
        all_checkpoints_restore &= b.checkpoint.restores_under_all_triggers;
        if b.grants_mutation_authority || b.grants_control_authority {
            no_authority_widening = false;
        }
    }

    PresentationOverlayBindingSummary {
        case_count: cases.len() as u32,
        surfaces_covered: surfaces.into_iter().collect(),
        host_zones_covered: host_zones.into_iter().collect(),
        all_panes_preserved,
        all_provenance_preserved,
        all_keyboard_complete,
        all_checkpoints_restore,
        no_authority_widening,
        fallback_placement_demonstrated,
    }
}

// ---- builders -------------------------------------------------------------

fn expanded_layout() -> ZoneRegistryLayout {
    ZoneRegistry::new(ZoneDefaults::standard()).layout(ZoneRegistryInput {
        window_width: 1920,
        window_height: 1080,
        split_heavy: false,
        main_workspace_min_width_override: None,
    })
}

fn compact_layout() -> ZoneRegistryLayout {
    ZoneRegistry::new(ZoneDefaults::standard()).layout(ZoneRegistryInput {
        window_width: 1024,
        window_height: 720,
        split_heavy: false,
        main_workspace_min_width_override: None,
    })
}

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

#[allow(clippy::too_many_arguments)]
fn waypoint(
    id: &str,
    ordinal: u32,
    title: &str,
    kind: WalkthroughSurfaceKind,
    file: Option<&str>,
    symbol: Option<&str>,
    boundary: BoundaryLabel,
    note: Option<SpeakerNote>,
) -> FollowWaypoint {
    FollowWaypoint {
        waypoint_id: id.to_owned(),
        ordinal,
        step_title: title.to_owned(),
        surface_kind: kind,
        target_object_ref: format!("obj:{id}"),
        file_path_ref: file.map(str::to_owned),
        symbol_anchor_ref: symbol.map(str::to_owned),
        branch_workspace_ref: "branch:main@workspace:local".to_owned(),
        boundary_label: boundary,
        zoom_layout_hint_ref: Some(format!("zoom-hint:{id}")),
        reveal_action_ref: Some(format!("reveal:{id}")),
        completion_state: if ordinal == 1 {
            WaypointCompletionState::Current
        } else {
            WaypointCompletionState::Pending
        },
        speaker_note: note,
        reuses_existing_surface: true,
        creates_parallel_artifact: false,
    }
}

fn solo_rehearsal_session() -> PresentationSession {
    PresentationSessionBuilder::new(
        "presentation:session:overlay:solo_rehearsal",
        LeaderFollowState::Presenting,
        AudienceScope::SoloRehearsal,
        checkpoint("overlay-solo"),
    )
    .focus("wp:overlay:solo:1")
    .waypoint(waypoint(
        "wp:overlay:solo:1",
        1,
        "Open the editor anchor",
        WalkthroughSurfaceKind::Editor,
        Some("crates/aureline-shell/src/layout/presentation_overlays.rs"),
        Some("fn plan_presentation_overlays"),
        BoundaryLabel::Local,
        Some(SpeakerNote::local(
            "note:overlay:solo:1",
            "wp:overlay:solo:1",
            "Remind the room the spotlight is an inset, not a takeover.",
        )),
    ))
    .waypoint(waypoint(
        "wp:overlay:solo:2",
        2,
        "Show the provenance strip",
        WalkthroughSurfaceKind::Editor,
        Some("crates/aureline-shell/src/presentation/binding.rs"),
        Some("struct NavigationProvenanceBinding"),
        BoundaryLabel::Local,
        None,
    ))
    .build()
}

fn breakaway_session() -> PresentationSession {
    PresentationSessionBuilder::new(
        "presentation:session:overlay:shared_breakaway",
        LeaderFollowState::BrokenAway,
        AudienceScope::SharedWorkspace,
        checkpoint("overlay-breakaway"),
    )
    .focus("wp:overlay:break:1")
    .waypoint(waypoint(
        "wp:overlay:break:1",
        1,
        "Compare the diff",
        WalkthroughSurfaceKind::Diff,
        Some("crates/aureline-shell/src/layout/zone_registry.rs"),
        None,
        BoundaryLabel::Shared,
        None,
    ))
    .participant(AudienceParticipant {
        participant_id: "participant:overlay:break:1".to_owned(),
        role_badge: ParticipantRole::Viewer,
        follow_state: ParticipantFollowState::Following,
        is_external_guest: false,
    })
    .participant(AudienceParticipant {
        participant_id: "participant:overlay:break:2".to_owned(),
        role_badge: ParticipantRole::Viewer,
        follow_state: ParticipantFollowState::BrokenAway,
        is_external_guest: true,
    })
    .build()
}

fn compact_docs_session() -> PresentationSession {
    PresentationSessionBuilder::new(
        "presentation:session:overlay:compact_docs",
        LeaderFollowState::Presenting,
        AudienceScope::SharedWorkspace,
        checkpoint("overlay-compact"),
    )
    .focus("wp:overlay:compact:1")
    .waypoint(waypoint(
        "wp:overlay:compact:1",
        1,
        "Walk the docs narrative",
        WalkthroughSurfaceKind::Docs,
        None,
        None,
        BoundaryLabel::Local,
        Some(SpeakerNote::local(
            "note:overlay:compact:1",
            "wp:overlay:compact:1",
            "On a narrow window the rail floats; the editor never shrinks.",
        )),
    ))
    .build()
}

fn case(
    case_id: &str,
    scenario: &str,
    session: &PresentationSession,
    layout: &ZoneRegistryLayout,
) -> PresentationOverlayBindingCase {
    PresentationOverlayBindingCase {
        record_kind: PRESENTATION_OVERLAY_BINDING_CASE_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        case_id: case_id.to_owned(),
        scenario_label: scenario.to_owned(),
        binding: project_overlay_navigation_binding(session, layout),
    }
}

/// Build the full seeded overlay/navigation binding corpus.
pub fn seeded_overlay_navigation_corpus() -> PresentationOverlayBindingCorpus {
    let expanded = expanded_layout();
    let compact = compact_layout();
    let cases = vec![
        case(
            "overlay-case:solo-rehearsal",
            "Solo rehearsal on an expanded desktop: presenter bar, waypoint rail, \
             spotlight inset, speaker-notes tray, and provenance strip all ride \
             existing zones.",
            &solo_rehearsal_session(),
            &expanded,
        ),
        case(
            "overlay-case:shared-breakaway",
            "Shared-workspace breakaway on an expanded desktop: the durable \
             breakaway banner floats over the transient overlay while the \
             presenter anchor and provenance stay visible.",
            &breakaway_session(),
            &expanded,
        ),
        case(
            "overlay-case:compact-docs",
            "Compact desktop with a collapsed sidebar: the waypoint rail floats \
             into the transient overlay zone rather than reclaiming the editor's \
             space.",
            &compact_docs_session(),
            &compact,
        ),
    ];
    let summary = summarize(&cases);
    PresentationOverlayBindingCorpus {
        record_kind: PRESENTATION_OVERLAY_BINDING_CORPUS_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        generated_at: "2026-06-20T00:00:00Z".to_owned(),
        summary,
        cases,
    }
}
