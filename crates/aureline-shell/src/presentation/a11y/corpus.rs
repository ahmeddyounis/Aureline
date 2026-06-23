//! Seeded presentation-accessibility corpus, support export, and validation.
//!
//! Each case builds one [`PresentationSession`] and projects the
//! [`PresentationAccessibilityReport`] that proves the overlay surfaces meet the
//! shell's accessibility and boundary-truth bar: a stable keyboard focus ring,
//! visible focus, reduced-motion safety, screen-reader reachability, high-zoom
//! support, accessible labels on every surface, and explicit local / remote /
//! shared boundary labels that survive into diagnostics. The checked-in fixtures
//! under `fixtures/presentation/a11y-and-motion/` are a literal projection of
//! [`seeded_presentation_a11y_corpus`], so the JSON cannot drift from the Rust
//! types.
//!
//! The corpus deliberately covers both zoom tiers, all three boundary labels,
//! both conformance classes, and the presenting and broken-away states: a solo
//! rehearsal at standard zoom that is fully accessible, a shared session at high
//! zoom whose dense rail and audience strip degrade to honest summarized-reachable
//! forms, a broken-away follower whose breakaway banner stays operable, an
//! invited-guests session on a remote boundary, and a mixed local/remote/shared
//! walkthrough whose boundary labels are kept distinct rather than flattened — so
//! the named accessibility dimensions and boundary truth are proven rather than
//! asserted.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::presentation_mode::{
    AudienceParticipant, AudienceScope, BoundaryLabel, FollowWaypoint, LeaderFollowState,
    ParticipantFollowState, ParticipantRole, PresentationSessionBuilder, RestoreCheckpoint,
    WalkthroughSurfaceKind, WaypointCompletionState, PRESENTATION_MODE_BETA_SCHEMA_VERSION,
    PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF,
};

use super::conformance::{
    project_accessibility_report, AccessibilityProjectionInputs, PresentationA11yClass,
    PresentationA11ySupportExport, PresentationA11yViolation, PresentationAccessibilityReport,
    PresentationSurfaceTag, ZoomTier,
};

/// Stable record kind for [`A11yCase`] payloads.
pub const PRESENTATION_A11Y_CASE_RECORD_KIND: &str = "presentation_accessibility_case_record";

/// Stable record kind for [`PresentationA11yCorpus`] payloads.
pub const PRESENTATION_A11Y_CORPUS_RECORD_KIND: &str = "presentation_accessibility_corpus_record";

/// One seeded case: a scenario plus the projected accessibility report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct A11yCase {
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
    /// The projected accessibility report.
    pub report: PresentationAccessibilityReport,
}

/// Aggregate coverage summary for the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct A11yCorpusSummary {
    /// Number of cases.
    pub case_count: u32,
    /// Distinct conformance classes covered.
    pub classes_covered: Vec<PresentationA11yClass>,
    /// Distinct zoom tiers covered.
    pub zoom_tiers_covered: Vec<ZoomTier>,
    /// Distinct boundary labels covered across every report.
    pub boundary_labels_covered: Vec<BoundaryLabel>,
    /// Distinct audience scopes covered.
    pub audience_scopes_covered: Vec<AudienceScope>,
    /// Distinct overlay surfaces exercised across every report.
    pub surfaces_covered: Vec<PresentationSurfaceTag>,
    /// True when every report validates.
    pub all_reports_valid: bool,
    /// True when every actionable surface in every report is keyboard reachable.
    pub all_keyboard_complete: bool,
    /// True when no report is pointer-only.
    pub none_pointer_only: bool,
    /// True when every surface in every report is screen-reader reachable.
    pub all_screen_reader_reachable: bool,
    /// True when every surface in every report respects reduced motion.
    pub all_reduced_motion_respected: bool,
    /// True when every report stays operable at high zoom.
    pub all_high_zoom_supported: bool,
    /// True when every report's focus ring is contiguous.
    pub all_focus_order_contiguous: bool,
    /// True when no report traps focus.
    pub none_traps_focus: bool,
    /// True when every report preserves its boundary labels.
    pub all_boundary_labels_preserved: bool,
    /// True when every surface in every report carries an accessible label.
    pub all_accessible_labels_complete: bool,
    /// True when at least one case is fully accessible.
    pub fully_accessible_demonstrated: bool,
    /// True when at least one case degrades to a summarized-but-reachable form.
    pub degraded_announced_demonstrated: bool,
    /// True when at least one case shows the breakaway banner.
    pub breakaway_banner_demonstrated: bool,
    /// True when at least one case shows the spotlight frame.
    pub spotlight_demonstrated: bool,
}

/// The full seeded presentation-accessibility corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationA11yCorpus {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Mint timestamp.
    pub generated_at: String,
    /// Coverage summary.
    pub summary: A11yCorpusSummary,
    /// Per-scenario cases.
    pub cases: Vec<A11yCase>,
}

impl PresentationA11yCorpus {
    /// Every projected report across the corpus, in case order.
    pub fn all_reports(&self) -> impl Iterator<Item = &PresentationAccessibilityReport> {
        self.cases.iter().map(|case| &case.report)
    }
}

/// Errors emitted by [`validate_presentation_a11y_corpus`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum A11yCorpusError {
    /// The corpus carried the wrong record kind or schema version.
    MalformedCorpus,
    /// A case carried the wrong record kind or schema version.
    MalformedCase {
        /// The offending case id.
        case_id: String,
    },
    /// A case's report failed validation.
    CaseInvalid {
        /// The offending case id.
        case_id: String,
        /// The violations the report emitted.
        violations: Vec<PresentationA11yViolation>,
    },
    /// The summary did not match the cases it claims to summarize.
    SummaryMismatch,
    /// A required conformance class was not demonstrated.
    ClassNotDemonstrated {
        /// The class missing from the corpus.
        class: PresentationA11yClass,
    },
    /// A required boundary label was not demonstrated.
    BoundaryLabelNotDemonstrated {
        /// The label missing from the corpus.
        label: BoundaryLabel,
    },
}

/// Validate the seeded presentation-accessibility corpus.
pub fn validate_presentation_a11y_corpus(
    corpus: &PresentationA11yCorpus,
) -> Result<(), A11yCorpusError> {
    if corpus.record_kind != PRESENTATION_A11Y_CORPUS_RECORD_KIND
        || corpus.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
    {
        return Err(A11yCorpusError::MalformedCorpus);
    }

    for case in &corpus.cases {
        if case.record_kind != PRESENTATION_A11Y_CASE_RECORD_KIND
            || case.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
        {
            return Err(A11yCorpusError::MalformedCase {
                case_id: case.case_id.clone(),
            });
        }
        let violations = case.report.validate();
        if !violations.is_empty() {
            return Err(A11yCorpusError::CaseInvalid {
                case_id: case.case_id.clone(),
                violations,
            });
        }
    }

    let expected = summarize(&corpus.cases);
    if expected != corpus.summary {
        return Err(A11yCorpusError::SummaryMismatch);
    }

    for class in [
        PresentationA11yClass::FullyAccessible,
        PresentationA11yClass::DegradedAnnounced,
    ] {
        if !corpus.summary.classes_covered.contains(&class) {
            return Err(A11yCorpusError::ClassNotDemonstrated { class });
        }
    }
    for label in [
        BoundaryLabel::Local,
        BoundaryLabel::Remote,
        BoundaryLabel::Shared,
    ] {
        if !corpus.summary.boundary_labels_covered.contains(&label) {
            return Err(A11yCorpusError::BoundaryLabelNotDemonstrated { label });
        }
    }

    Ok(())
}

/// Project a corpus into a support-safe export over every accessibility report.
pub fn presentation_a11y_support_export(
    export_id: impl Into<String>,
    generated_at: impl Into<String>,
    corpus: &PresentationA11yCorpus,
) -> PresentationA11ySupportExport {
    PresentationA11ySupportExport::from_reports(export_id, generated_at, corpus.all_reports())
}

fn summarize(cases: &[A11yCase]) -> A11yCorpusSummary {
    let mut classes: BTreeSet<PresentationA11yClass> = BTreeSet::new();
    let mut zoom_tiers: BTreeSet<ZoomTier> = BTreeSet::new();
    let mut boundary_labels: BTreeSet<BoundaryLabel> = BTreeSet::new();
    let mut audience_scopes: BTreeSet<AudienceScope> = BTreeSet::new();
    let mut surfaces: BTreeSet<PresentationSurfaceTag> = BTreeSet::new();
    let mut all_reports_valid = true;
    let mut all_keyboard_complete = true;
    let mut none_pointer_only = true;
    let mut all_screen_reader_reachable = true;
    let mut all_reduced_motion_respected = true;
    let mut all_high_zoom_supported = true;
    let mut all_focus_order_contiguous = true;
    let mut none_traps_focus = true;
    let mut all_boundary_labels_preserved = true;
    let mut all_accessible_labels_complete = true;
    let mut breakaway = false;
    let mut spotlight = false;

    for case in cases {
        let report = &case.report;
        classes.insert(report.conformance_class);
        zoom_tiers.insert(report.zoom_tier);
        audience_scopes.insert(report.boundary_posture.audience_scope);
        boundary_labels.extend(
            report
                .boundary_posture
                .distinct_boundary_labels
                .iter()
                .copied(),
        );
        all_reports_valid &= report.validate().is_empty();
        all_keyboard_complete &= report.keyboard_complete;
        none_pointer_only &= !report.pointer_only;
        all_screen_reader_reachable &= report.screen_reader_reachable;
        all_reduced_motion_respected &= report.reduced_motion_respected;
        all_high_zoom_supported &= report.high_zoom_supported;
        all_focus_order_contiguous &= report.focus_order_contiguous;
        none_traps_focus &= report.no_focus_trap;
        all_boundary_labels_preserved &= report.boundary_labels_preserved;
        all_accessible_labels_complete &= report.accessible_labels_complete;
        for surface in &report.surfaces {
            surfaces.insert(surface.surface);
        }
        if report
            .surface(PresentationSurfaceTag::BreakawayBanner)
            .is_some()
        {
            breakaway = true;
        }
        if report
            .surface(PresentationSurfaceTag::SpotlightFrame)
            .is_some()
        {
            spotlight = true;
        }
    }

    A11yCorpusSummary {
        case_count: cases.len() as u32,
        classes_covered: classes.iter().copied().collect(),
        zoom_tiers_covered: zoom_tiers.into_iter().collect(),
        boundary_labels_covered: boundary_labels.into_iter().collect(),
        audience_scopes_covered: audience_scopes.into_iter().collect(),
        surfaces_covered: surfaces.into_iter().collect(),
        all_reports_valid,
        all_keyboard_complete,
        none_pointer_only,
        all_screen_reader_reachable,
        all_reduced_motion_respected,
        all_high_zoom_supported,
        all_focus_order_contiguous,
        none_traps_focus,
        all_boundary_labels_preserved,
        all_accessible_labels_complete,
        fully_accessible_demonstrated: classes.contains(&PresentationA11yClass::FullyAccessible),
        degraded_announced_demonstrated: classes
            .contains(&PresentationA11yClass::DegradedAnnounced),
        breakaway_banner_demonstrated: breakaway,
        spotlight_demonstrated: spotlight,
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

fn waypoint(id: &str, ordinal: u32, boundary: BoundaryLabel) -> FollowWaypoint {
    FollowWaypoint {
        waypoint_id: id.to_owned(),
        ordinal,
        step_title: format!("Step {ordinal}"),
        surface_kind: WalkthroughSurfaceKind::Editor,
        target_object_ref: format!("obj:{id}"),
        file_path_ref: Some(
            "crates/aureline-shell/src/presentation/a11y/conformance.rs".to_owned(),
        ),
        symbol_anchor_ref: Some("fn project_accessibility_report".to_owned()),
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

fn participant(id: &str, follow: ParticipantFollowState, guest: bool) -> AudienceParticipant {
    AudienceParticipant {
        participant_id: id.to_owned(),
        role_badge: ParticipantRole::Viewer,
        follow_state: follow,
        is_external_guest: guest,
    }
}

fn case(case_id: &str, scenario: &str, report: PresentationAccessibilityReport) -> A11yCase {
    A11yCase {
        record_kind: PRESENTATION_A11Y_CASE_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        case_id: case_id.to_owned(),
        scenario_label: scenario.to_owned(),
        report,
    }
}

fn presenter_standard_local_case() -> A11yCase {
    let id = "presenter_standard_local";
    let wp = format!("wp:{id}:1");
    let session = PresentationSessionBuilder::new(
        format!("presentation:session:a11y:{id}"),
        LeaderFollowState::Presenting,
        AudienceScope::SoloRehearsal,
        checkpoint(id),
    )
    .focus(wp.clone())
    .waypoint(waypoint(&wp, 1, BoundaryLabel::Local))
    .build();
    let report = project_accessibility_report(&session, &AccessibilityProjectionInputs::standard());
    case(
        "a11y-case:presenter-standard-local",
        "A solo rehearsal at standard zoom on a local target. Every overlay \
         surface — presenter bar, agenda rail, spotlight inset, notes tray, \
         audience strip, provenance strip — is keyboard reachable in a single \
         contiguous focus ring, has a visible focus indicator, respects reduced \
         motion, is screen-reader reachable, and carries an accessible label. The \
         local boundary label stays visible: fully accessible.",
        report,
    )
}

fn presenter_high_zoom_case() -> A11yCase {
    let id = "presenter_high_zoom";
    let wp1 = format!("wp:{id}:1");
    let wp2 = format!("wp:{id}:2");
    let session = PresentationSessionBuilder::new(
        format!("presentation:session:a11y:{id}"),
        LeaderFollowState::Presenting,
        AudienceScope::SharedWorkspace,
        checkpoint(id),
    )
    .focus(wp1.clone())
    .waypoint(waypoint(&wp1, 1, BoundaryLabel::Shared))
    .waypoint(waypoint(&wp2, 2, BoundaryLabel::Shared))
    .participant(participant("p:1", ParticipantFollowState::Following, false))
    .participant(participant(
        "p:2",
        ParticipantFollowState::BrokenAway,
        false,
    ))
    .build();
    let report =
        project_accessibility_report(&session, &AccessibilityProjectionInputs::high_zoom());
    case(
        "a11y-case:presenter-high-zoom-summarized",
        "A shared session driven at high zoom / large text. The dense agenda rail \
         and audience strip reflow to a labeled, keyboard-reachable summary that \
         expands on demand; every other surface reflows in place. Nothing is \
         truncated silently and nothing becomes pointer-only — the degrade is \
         announced, so the report is degraded-announced rather than non-conformant.",
        report,
    )
}

fn broken_away_shared_banner_case() -> A11yCase {
    let id = "broken_away_shared";
    let wp = format!("wp:{id}:1");
    let session = PresentationSessionBuilder::new(
        format!("presentation:session:a11y:{id}"),
        LeaderFollowState::BrokenAway,
        AudienceScope::SharedWorkspace,
        checkpoint(id),
    )
    .focus(wp.clone())
    .waypoint(waypoint(&wp, 1, BoundaryLabel::Shared))
    .participant(participant("p:1", ParticipantFollowState::Following, false))
    .build();
    let report = project_accessibility_report(&session, &AccessibilityProjectionInputs::standard());
    case(
        "a11y-case:broken-away-shared-banner",
        "A follower who has broken away to browse independently. The durable \
         breakaway banner joins the focus ring as a keyboard-reachable, announced \
         control with a return-to-presenter action; it never traps focus and the \
         shared boundary label stays visible. Fully accessible.",
        report,
    )
}

fn invited_guests_remote_case() -> A11yCase {
    let id = "invited_guests_remote";
    let wp = format!("wp:{id}:1");
    let session = PresentationSessionBuilder::new(
        format!("presentation:session:a11y:{id}"),
        LeaderFollowState::Presenting,
        AudienceScope::InvitedGuests,
        checkpoint(id),
    )
    .focus(wp.clone())
    .waypoint(waypoint(&wp, 1, BoundaryLabel::Remote))
    .participant(participant(
        "guest:1",
        ParticipantFollowState::Following,
        true,
    ))
    .build();
    let report = project_accessibility_report(&session, &AccessibilityProjectionInputs::standard());
    case(
        "a11y-case:invited-guests-remote",
        "An invited-guests session anchored on a remote target. The remote \
         boundary label is carried explicitly on the provenance strip and the \
         spotlight inset and exported as remote — never flattened to a generic \
         shared badge — so diagnostics can explain exactly where the audience is \
         looking. Fully accessible.",
        report,
    )
}

fn mixed_boundary_case() -> A11yCase {
    let id = "mixed_boundary";
    let wp1 = format!("wp:{id}:1");
    let wp2 = format!("wp:{id}:2");
    let wp3 = format!("wp:{id}:3");
    let session = PresentationSessionBuilder::new(
        format!("presentation:session:a11y:{id}"),
        LeaderFollowState::Presenting,
        AudienceScope::SharedWorkspace,
        checkpoint(id),
    )
    .focus(wp2.clone())
    .waypoint(waypoint(&wp1, 1, BoundaryLabel::Local))
    .waypoint(waypoint(&wp2, 2, BoundaryLabel::Shared))
    .waypoint(waypoint(&wp3, 3, BoundaryLabel::Remote))
    .build();
    let report =
        project_accessibility_report(&session, &AccessibilityProjectionInputs::high_zoom());
    case(
        "a11y-case:mixed-boundary-rail",
        "A walkthrough whose steps span local, shared, and remote targets, driven \
         at high zoom. The current step's shared boundary is shown, and the \
         distinct local / shared / remote labels are all kept in the boundary \
         posture rather than collapsed to one badge. The summarized rail and \
         audience strip stay reachable: degraded-announced, boundary-honest.",
        report,
    )
}

/// Build the full seeded presentation-accessibility corpus.
pub fn seeded_presentation_a11y_corpus() -> PresentationA11yCorpus {
    let cases = vec![
        presenter_standard_local_case(),
        presenter_high_zoom_case(),
        broken_away_shared_banner_case(),
        invited_guests_remote_case(),
        mixed_boundary_case(),
    ];
    let summary = summarize(&cases);
    PresentationA11yCorpus {
        record_kind: PRESENTATION_A11Y_CORPUS_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        generated_at: "2026-06-20T00:00:00Z".to_owned(),
        summary,
        cases,
    }
}
