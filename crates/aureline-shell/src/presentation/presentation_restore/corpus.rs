//! Seeded presentation-restore corpus, support export, and validation.
//!
//! Each case builds one [`PresentationSession`] and projects the
//! [`PresentationRestoreReport`] that proves a restore brought the prior layout
//! back at a classified, support-safe fidelity — and that any target it could
//! not bring back was surfaced as an honest placeholder / disconnected state
//! rather than silently re-run, re-authorized, or hidden behind a generic
//! success. The checked-in fixtures under `fixtures/presentation/restore-no-rerun/`
//! are a literal projection of [`seeded_presentation_restore_corpus`], so the
//! JSON cannot drift from the Rust types.
//!
//! The corpus deliberately covers every restore class and trigger: a clean
//! exit that restores exactly, a crash recovery that needs a compatible layout
//! translation, an interrupted resume that degrades two waypoints to honest
//! placeholder / disconnected cards, a cancel whose remote targets are
//! unavailable and whose authority expired, a crash recovery whose live
//! walkthrough could not be rehydrated (evidence-only), and an interrupted
//! resume with no checkpoint at all (no-restore) — so exact / compatible /
//! layout-only / evidence-only / no-restore fidelity and every degrade cause are
//! proven across triggers rather than asserted.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::presentation_mode::{
    AudienceScope, BoundaryLabel, FollowWaypoint, LeaderFollowState, PresentationSession,
    PresentationSessionBuilder, RestoreCheckpoint, WalkthroughSurfaceKind, WaypointCompletionState,
    PRESENTATION_MODE_BETA_SCHEMA_VERSION, PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF,
};

use super::restore::{
    project_evidence_only_report, project_no_restore_report, project_restore_report,
    PresentationRestoreClass, PresentationRestoreReport, PresentationRestoreSupportExport,
    PresentationRestoreTrigger, PresentationRestoreViolation, RestoreDegradeTrigger,
    RestoreProjectionInputs, WaypointAvailability, WaypointDegrade,
};

/// Stable record kind for [`RestoreCase`] payloads.
pub const PRESENTATION_RESTORE_CASE_RECORD_KIND: &str = "presentation_restore_case_record";

/// Stable record kind for [`PresentationRestoreCorpus`] payloads.
pub const PRESENTATION_RESTORE_CORPUS_RECORD_KIND: &str = "presentation_restore_corpus_record";

/// One seeded case: a scenario plus the projected restore report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreCase {
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
    /// The projected restore report.
    pub report: PresentationRestoreReport,
}

/// Aggregate coverage summary for the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreCorpusSummary {
    /// Number of cases.
    pub case_count: u32,
    /// Distinct restore triggers covered.
    pub triggers_covered: Vec<PresentationRestoreTrigger>,
    /// Distinct restore classes covered.
    pub classes_covered: Vec<PresentationRestoreClass>,
    /// Distinct degrade triggers covered.
    pub degrade_triggers_covered: Vec<RestoreDegradeTrigger>,
    /// Distinct waypoint availabilities covered.
    pub availabilities_covered: Vec<WaypointAvailability>,
    /// True when every report in the corpus validates.
    pub all_reports_valid: bool,
    /// True when no report replays a mutating action.
    pub no_mutating_replay: bool,
    /// True when no report re-acquires authority.
    pub no_authority_reacquired: bool,
    /// True when no report leaves the user in an improvised shell.
    pub no_improvised_shell: bool,
    /// True when no degraded report hides its cause behind a generic success.
    pub no_hidden_degrade: bool,
    /// True when at least one case restores exactly.
    pub exact_demonstrated: bool,
    /// True when at least one case restores via a compatible translation.
    pub compatible_demonstrated: bool,
    /// True when at least one case degrades to layout-only.
    pub layout_only_demonstrated: bool,
    /// True when at least one case degrades to evidence-only.
    pub evidence_only_demonstrated: bool,
    /// True when at least one case ends in no-restore.
    pub no_restore_demonstrated: bool,
    /// True when at least one waypoint degrades to a placeholder.
    pub placeholder_demonstrated: bool,
    /// True when at least one waypoint degrades to a disconnected state.
    pub disconnected_demonstrated: bool,
}

/// The full seeded presentation-restore corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationRestoreCorpus {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Mint timestamp.
    pub generated_at: String,
    /// Coverage summary.
    pub summary: RestoreCorpusSummary,
    /// Per-scenario cases.
    pub cases: Vec<RestoreCase>,
}

impl PresentationRestoreCorpus {
    /// Every projected report across the corpus, in case order.
    pub fn all_reports(&self) -> impl Iterator<Item = &PresentationRestoreReport> {
        self.cases.iter().map(|case| &case.report)
    }
}

/// Errors emitted by [`validate_presentation_restore_corpus`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreCorpusError {
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
        violations: Vec<PresentationRestoreViolation>,
    },
    /// The summary did not match the cases it claims to summarize.
    SummaryMismatch,
    /// A required restore class was not demonstrated.
    ClassNotDemonstrated {
        /// The class missing from the corpus.
        class: PresentationRestoreClass,
    },
}

/// Validate the seeded presentation-restore corpus.
pub fn validate_presentation_restore_corpus(
    corpus: &PresentationRestoreCorpus,
) -> Result<(), RestoreCorpusError> {
    if corpus.record_kind != PRESENTATION_RESTORE_CORPUS_RECORD_KIND
        || corpus.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
    {
        return Err(RestoreCorpusError::MalformedCorpus);
    }

    for case in &corpus.cases {
        if case.record_kind != PRESENTATION_RESTORE_CASE_RECORD_KIND
            || case.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
        {
            return Err(RestoreCorpusError::MalformedCase {
                case_id: case.case_id.clone(),
            });
        }
        let violations = case.report.validate();
        if !violations.is_empty() {
            return Err(RestoreCorpusError::CaseInvalid {
                case_id: case.case_id.clone(),
                violations,
            });
        }
    }

    let expected = summarize(&corpus.cases);
    if expected != corpus.summary {
        return Err(RestoreCorpusError::SummaryMismatch);
    }

    for class in [
        PresentationRestoreClass::ExactRestore,
        PresentationRestoreClass::CompatibleRestore,
        PresentationRestoreClass::LayoutOnly,
        PresentationRestoreClass::EvidenceOnly,
        PresentationRestoreClass::NoRestore,
    ] {
        if !corpus.summary.classes_covered.contains(&class) {
            return Err(RestoreCorpusError::ClassNotDemonstrated { class });
        }
    }

    Ok(())
}

/// Project a corpus into a support-safe export over every restore report.
pub fn presentation_restore_support_export(
    export_id: impl Into<String>,
    generated_at: impl Into<String>,
    corpus: &PresentationRestoreCorpus,
) -> PresentationRestoreSupportExport {
    PresentationRestoreSupportExport::from_reports(export_id, generated_at, corpus.all_reports())
}

fn summarize(cases: &[RestoreCase]) -> RestoreCorpusSummary {
    let mut triggers: BTreeSet<PresentationRestoreTrigger> = BTreeSet::new();
    let mut classes: BTreeSet<PresentationRestoreClass> = BTreeSet::new();
    let mut degrade_triggers: BTreeSet<RestoreDegradeTrigger> = BTreeSet::new();
    let mut availabilities: BTreeSet<WaypointAvailability> = BTreeSet::new();
    let mut all_reports_valid = true;
    let mut no_mutating_replay = true;
    let mut no_authority_reacquired = true;
    let mut no_improvised_shell = true;
    let mut no_hidden_degrade = true;

    for case in cases {
        let report = &case.report;
        triggers.insert(report.trigger);
        classes.insert(report.restore_class);
        degrade_triggers.extend(report.degrade_triggers.iter().copied());
        all_reports_valid &= report.validate().is_empty();
        no_mutating_replay &= !report.replayed_any_mutating_action;
        no_authority_reacquired &= !report.reacquired_any_authority;
        no_improvised_shell &= !report.left_in_improvised_shell;
        no_hidden_degrade &= !report.hides_degrade_behind_generic_success;
        for waypoint in &report.waypoint_restores {
            availabilities.insert(waypoint.availability);
        }
    }

    RestoreCorpusSummary {
        case_count: cases.len() as u32,
        triggers_covered: triggers.into_iter().collect(),
        classes_covered: classes.iter().copied().collect(),
        degrade_triggers_covered: degrade_triggers.into_iter().collect(),
        availabilities_covered: availabilities.iter().copied().collect(),
        all_reports_valid,
        no_mutating_replay,
        no_authority_reacquired,
        no_improvised_shell,
        no_hidden_degrade,
        exact_demonstrated: classes.contains(&PresentationRestoreClass::ExactRestore),
        compatible_demonstrated: classes.contains(&PresentationRestoreClass::CompatibleRestore),
        layout_only_demonstrated: classes.contains(&PresentationRestoreClass::LayoutOnly),
        evidence_only_demonstrated: classes.contains(&PresentationRestoreClass::EvidenceOnly),
        no_restore_demonstrated: classes.contains(&PresentationRestoreClass::NoRestore),
        placeholder_demonstrated: availabilities.contains(&WaypointAvailability::Placeholder),
        disconnected_demonstrated: availabilities.contains(&WaypointAvailability::Disconnected),
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
            "crates/aureline-shell/src/presentation/presentation_restore/restore.rs".to_owned(),
        ),
        symbol_anchor_ref: Some("fn project_restore_report".to_owned()),
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
    let wp1 = format!("wp:{id}:1");
    let wp2 = format!("wp:{id}:2");
    PresentationSessionBuilder::new(
        format!("presentation:session:restore:{id}"),
        LeaderFollowState::Presenting,
        audience,
        checkpoint(id),
    )
    .focus(wp1.clone())
    .waypoint(waypoint(&wp1, 1, boundary))
    .waypoint(waypoint(&wp2, 2, boundary))
    .build()
}

fn case(case_id: &str, scenario: &str, report: PresentationRestoreReport) -> RestoreCase {
    RestoreCase {
        record_kind: PRESENTATION_RESTORE_CASE_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        case_id: case_id.to_owned(),
        scenario_label: scenario.to_owned(),
        report,
    }
}

fn exit_exact_case() -> RestoreCase {
    let session = session(
        "exit_exact",
        BoundaryLabel::Local,
        AudienceScope::SoloRehearsal,
    );
    let report = project_restore_report(
        &session,
        &RestoreProjectionInputs::exact(PresentationRestoreTrigger::Exit),
    );
    case(
        "restore-case:exit-exact",
        "A solo rehearsal exits cleanly: the prior layout, focus, panels, and \
         accessibility posture all come back exactly and every waypoint is restored \
         read-only. Nothing is re-run.",
        report,
    )
}

fn crash_compatible_case() -> RestoreCase {
    let session = session(
        "crash_compatible",
        BoundaryLabel::Local,
        AudienceScope::SharedWorkspace,
    );
    let report = project_restore_report(
        &session,
        &RestoreProjectionInputs::compatible(PresentationRestoreTrigger::CrashRecovery),
    );
    case(
        "restore-case:crash-compatible",
        "Crash recovery rehydrates the session, but the prior window topology no \
         longer maps one-to-one and is brought back through a compatible \
         translation. Every waypoint is still live; the fidelity is labeled \
         compatible rather than claimed exact.",
        report,
    )
}

fn resume_layout_only_case() -> RestoreCase {
    let session = session(
        "resume_layout_only",
        BoundaryLabel::Shared,
        AudienceScope::SharedWorkspace,
    );
    let degrades = vec![
        WaypointDegrade::new(
            "wp:resume_layout_only:1",
            RestoreDegradeTrigger::MissingDependency,
            "Step 1 surface unavailable — the extension that rendered it is no \
             longer installed. Showing a placeholder.",
        ),
        WaypointDegrade::new(
            "wp:resume_layout_only:2",
            RestoreDegradeTrigger::RevokedSharingGrant,
            "Step 2 disconnected — the sharing grant for this shared target was \
             revoked. Reconnect requires a fresh grant.",
        ),
    ];
    let report = project_restore_report(
        &session,
        &RestoreProjectionInputs::with_degrades(
            PresentationRestoreTrigger::InterruptedResume,
            degrades,
        ),
    );
    case(
        "restore-case:resume-layout-only-degraded",
        "An interrupted session resumes: the prior layout comes back, but one \
         waypoint's surface dependency is gone (honest placeholder) and another's \
         sharing grant was revoked (honest disconnected). Neither is re-run or \
         re-authorized; the layout-only fidelity is surfaced, not hidden.",
        report,
    )
}

fn cancel_disconnected_case() -> RestoreCase {
    let session = session(
        "cancel_disconnected",
        BoundaryLabel::Remote,
        AudienceScope::InvitedGuests,
    );
    let degrades = vec![
        WaypointDegrade::new(
            "wp:cancel_disconnected:1",
            RestoreDegradeTrigger::UnavailableRemoteTarget,
            "Step 1 disconnected — the remote target is unreachable. Showing the \
             last-known anchor as disconnected.",
        ),
        WaypointDegrade::new(
            "wp:cancel_disconnected:2",
            RestoreDegradeTrigger::ExpiredAuthority,
            "Step 2 disconnected — the privileged grant this step relied on has \
             expired and is not silently re-acquired.",
        ),
    ];
    let report = project_restore_report(
        &session,
        &RestoreProjectionInputs::with_degrades(PresentationRestoreTrigger::Cancel, degrades),
    );
    case(
        "restore-case:cancel-disconnected-remote-and-expired",
        "A cancel restores the prior layout, but a remote target is unreachable \
         and a privileged grant has expired. Both waypoints degrade to honest \
         disconnected cards; the expired authority stays expired.",
        report,
    )
}

fn crash_evidence_only_case() -> RestoreCase {
    let session = session(
        "crash_evidence_only",
        BoundaryLabel::Shared,
        AudienceScope::SharedWorkspace,
    );
    let report = project_evidence_only_report(
        &session,
        PresentationRestoreTrigger::CrashRecovery,
        RestoreDegradeTrigger::LiveSessionUnavailable,
    );
    case(
        "restore-case:crash-evidence-only",
        "Crash recovery brings the prior layout back, but the live shared \
         walkthrough cannot be rehydrated, so only an evidence record of the \
         session remains. No waypoint is re-run; the evidence-only fidelity is \
         surfaced honestly.",
        report,
    )
}

fn resume_no_restore_case() -> RestoreCase {
    let report = project_no_restore_report(
        "presentation:session:restore:resume_no_restore",
        PresentationRestoreTrigger::InterruptedResume,
    );
    case(
        "restore-case:resume-no-restore",
        "An interrupted resume finds no checkpoint was ever captured — entry was \
         interrupted before the prior layout could be checkpointed. Nothing is \
         restored; the user keeps their current layout and is told the resume \
         could not proceed rather than shown a fake success.",
        report,
    )
}

/// Build the full seeded presentation-restore corpus.
pub fn seeded_presentation_restore_corpus() -> PresentationRestoreCorpus {
    let cases = vec![
        exit_exact_case(),
        crash_compatible_case(),
        resume_layout_only_case(),
        cancel_disconnected_case(),
        crash_evidence_only_case(),
        resume_no_restore_case(),
    ];
    let summary = summarize(&cases);
    PresentationRestoreCorpus {
        record_kind: PRESENTATION_RESTORE_CORPUS_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        generated_at: "2026-06-20T00:00:00Z".to_owned(),
        summary,
        cases,
    }
}
