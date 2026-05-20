//! Crash-loop and restore-fidelity corpus: the evidence-first conformance lane
//! that keeps repeated-startup recovery and restore honesty from regressing.
//!
//! The crash-loop recovery center (`crate::crash_loop_center`) and the restore
//! hydrator (`aureline_workspace::restore_hydrator`) already own the *runtime*
//! truth for repeated-startup failure and skeleton-first restore. This module
//! owns the *corpus* that proves those two surfaces keep their launch-bearing
//! continuity promises under real fault conditions: extension-host crash loops,
//! bad layout restore, missing extension panes, unavailable remote sessions,
//! off-screen window remap, checkpoint-only recovery, and explicit
//! no-silent-rerun across the privileged session lanes.
//!
//! Unlike a declaration-only scenario list, every drill in this corpus binds an
//! *input fixture* that the protected drill harness replays through the real
//! evaluator. The harness asserts the observed recovery outcome, no-silent-rerun
//! posture, truthful-placeholder honesty, and accessibility, then folds the
//! validated corpus into a [`RecoveryChoiceMatrix`] — the publishable evidence
//! packet that shows exact vs compatible vs layout-only vs evidence-only
//! recovery outcomes on every covered row.
//!
//! ## What this corpus owns
//!
//! - The closed [`DrillConditionClass`] set every beta continuity claim must
//!   keep covered with a drill and a current evidence packet
//!   ([`required_condition_classes`]).
//! - The shared severity/lifecycle vocabularies — [`RecoveryOutcomeClass`],
//!   [`ProtectedLaneClass`], [`AccessibilityCheckClass`], and
//!   [`ClaimDowngradeTriggerClass`] — that the in-product cards, the support
//!   export, and this corpus all read.
//! - The [`CrashLoopRestoreFidelityCorpus::validate`] contract that refuses a
//!   corpus that drops a required condition, registers duplicates, weakens the
//!   no-silent-rerun requirement on a privileged lane, or fails the
//!   metadata-safe baseline; and the [`RecoveryChoiceMatrix`] projection that
//!   stays metadata-only.
//!
//! ## What this corpus does NOT own
//!
//! - The crash-loop center or restore hydrator runtime, or the input fixtures
//!   themselves. Each drill references the already-owning crate/test/fixture
//!   triad and asks the harness to re-prove the recovery path, not to
//!   re-implement it.
//! - Generic shell-chaos fuzzing. Every drill is linked to a named beta
//!   continuity claim; a drill with no claim linkage is rejected.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for a seeded crash-loop / restore-fidelity drill.
pub const DRILL_RECORD_KIND: &str = "crash_loop_restore_fidelity_drill_record";

/// Stable record-kind tag for the recovery-choice matrix projection.
pub const RECOVERY_CHOICE_MATRIX_RECORD_KIND: &str = "crash_loop_restore_recovery_matrix_record";

/// Stable record-kind tag for one row in the recovery-choice matrix.
pub const RECOVERY_CHOICE_MATRIX_ROW_RECORD_KIND: &str =
    "crash_loop_restore_recovery_matrix_row_record";

/// Frozen schema version for the corpus and its matrix projection.
pub const CRASH_LOOP_RESTORE_FIDELITY_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path of the protected corpus directory.
pub const CORPUS_DIR: &str = "fixtures/recovery/m3/crash_loop_and_restore_fidelity";

/// Repository-relative path of the protected corpus manifest.
pub const CORPUS_MANIFEST_REF: &str =
    "fixtures/recovery/m3/crash_loop_and_restore_fidelity/manifest.yaml";

/// Repository-relative path of the reviewer doc.
pub const CORPUS_DOC_REF: &str = "docs/support/m3/crash_loop_and_restore_corpus.md";

/// Repository-relative path of the reviewer-facing report artifact.
pub const CORPUS_REPORT_REF: &str =
    "artifacts/support/m3/crash_loop_and_restore_fidelity_report.md";

/// Repository-relative path of the recovery-choice matrix artifact.
pub const RECOVERY_CHOICE_MATRIX_REF: &str = "artifacts/support/m3/recovery_choice_matrix.json";

const DRILL_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/recovery/m3/crash_loop_and_restore_fidelity/extension_host_crash_loop.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/crash_loop_and_restore_fidelity/extension_host_crash_loop.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/crash_loop_and_restore_fidelity/checkpoint_only_recovery.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/crash_loop_and_restore_fidelity/checkpoint_only_recovery.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/crash_loop_and_restore_fidelity/bad_layout_restore.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/crash_loop_and_restore_fidelity/bad_layout_restore.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/crash_loop_and_restore_fidelity/missing_extension_panes.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/crash_loop_and_restore_fidelity/missing_extension_panes.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/crash_loop_and_restore_fidelity/unavailable_remote_sessions.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/crash_loop_and_restore_fidelity/unavailable_remote_sessions.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/crash_loop_and_restore_fidelity/offscreen_window_remap.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/crash_loop_and_restore_fidelity/offscreen_window_remap.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/crash_loop_and_restore_fidelity/no_silent_rerun_protected_lanes.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/crash_loop_and_restore_fidelity/no_silent_rerun_protected_lanes.yaml"
        )),
    ),
];

/// Required claim-downgrade triggers every drill must declare.
const REQUIRED_DOWNGRADE_TRIGGERS: &[ClaimDowngradeTriggerClass] = &[
    ClaimDowngradeTriggerClass::InputFixtureMissing,
    ClaimDowngradeTriggerClass::RecoveryClassRegressed,
    ClaimDowngradeTriggerClass::SilentReplayDetected,
];

/// Closed drill-condition vocabulary the corpus must keep covered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillConditionClass {
    /// An extension host repeatedly crashes the launch.
    ExtensionHostCrashLoop,
    /// A saved layout restores its shell but a surface degrades to a placeholder.
    BadLayoutRestore,
    /// Required extension panes are missing at restore time.
    MissingExtensionPanes,
    /// Required remote sessions are unavailable at restore time.
    UnavailableRemoteSessions,
    /// Saved bounds land off-screen and must be remapped onto a safe display.
    #[serde(rename = "offscreen_window_remap")]
    OffScreenWindowRemap,
    /// Restore replay is unsafe and recovery falls back to checkpoint/evidence.
    CheckpointOnlyRecovery,
    /// Restored privileged session lanes must never re-run silently.
    NoSilentRerunProtectedLanes,
}

impl DrillConditionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExtensionHostCrashLoop => "extension_host_crash_loop",
            Self::BadLayoutRestore => "bad_layout_restore",
            Self::MissingExtensionPanes => "missing_extension_panes",
            Self::UnavailableRemoteSessions => "unavailable_remote_sessions",
            Self::OffScreenWindowRemap => "offscreen_window_remap",
            Self::CheckpointOnlyRecovery => "checkpoint_only_recovery",
            Self::NoSilentRerunProtectedLanes => "no_silent_rerun_protected_lanes",
        }
    }

    /// The drill kind admitted for this condition.
    pub const fn admitted_drill_kind(self) -> DrillKind {
        match self {
            Self::ExtensionHostCrashLoop | Self::CheckpointOnlyRecovery => {
                DrillKind::CrashLoopCenter
            }
            Self::BadLayoutRestore
            | Self::MissingExtensionPanes
            | Self::UnavailableRemoteSessions
            | Self::OffScreenWindowRemap
            | Self::NoSilentRerunProtectedLanes => DrillKind::RestoreFidelity,
        }
    }
}

/// The closed set of conditions the corpus must keep covered.
pub const REQUIRED_CONDITION_CLASSES: [DrillConditionClass; 7] = [
    DrillConditionClass::ExtensionHostCrashLoop,
    DrillConditionClass::BadLayoutRestore,
    DrillConditionClass::MissingExtensionPanes,
    DrillConditionClass::UnavailableRemoteSessions,
    DrillConditionClass::OffScreenWindowRemap,
    DrillConditionClass::CheckpointOnlyRecovery,
    DrillConditionClass::NoSilentRerunProtectedLanes,
];

/// Returns the closed list of conditions the corpus must cover.
pub fn required_condition_classes() -> &'static [DrillConditionClass] {
    &REQUIRED_CONDITION_CLASSES
}

/// Closed drill-kind vocabulary: which evaluator the drill replays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillKind {
    /// Replays a crash-loop signal through the crash-loop recovery center.
    CrashLoopCenter,
    /// Replays a restore-hydration request through the restore hydrator.
    RestoreFidelity,
}

impl DrillKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrashLoopCenter => "crash_loop_center",
            Self::RestoreFidelity => "restore_fidelity",
        }
    }

    /// Expected repo-relative extension for this drill kind's input fixture.
    const fn input_fixture_suffix(self) -> &'static str {
        match self {
            Self::CrashLoopCenter => ".yaml",
            Self::RestoreFidelity => ".json",
        }
    }
}

/// Shared restore-fidelity / severity vocabulary, aligned with the crash-loop
/// center `RestoreClass` and the restore hydrator `RestoreLevel`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryOutcomeClass {
    /// The prior session is restored exactly.
    ExactRestore,
    /// The prior session is restored with compatible substitutions or remaps.
    CompatibleRestore,
    /// Only the window/pane layout is restored; heavy surfaces degrade.
    LayoutOnly,
    /// Only retained evidence (drafts, transcripts, checkpoints) is surfaced.
    EvidenceOnly,
    /// No restore is performed.
    NoRestore,
}

impl RecoveryOutcomeClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRestore => "exact_restore",
            Self::CompatibleRestore => "compatible_restore",
            Self::LayoutOnly => "layout_only",
            Self::EvidenceOnly => "evidence_only",
            Self::NoRestore => "no_restore",
        }
    }

    /// Controlled reviewer label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::ExactRestore => "Exact restore",
            Self::CompatibleRestore => "Compatible restore",
            Self::LayoutOnly => "Layout only",
            Self::EvidenceOnly => "Evidence only",
            Self::NoRestore => "No restore",
        }
    }

    /// Parses a stable snake-case token back into a class.
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "exact_restore" => Some(Self::ExactRestore),
            "compatible_restore" => Some(Self::CompatibleRestore),
            "layout_only" => Some(Self::LayoutOnly),
            "evidence_only" => Some(Self::EvidenceOnly),
            "no_restore" => Some(Self::NoRestore),
            _ => None,
        }
    }
}

/// Closed privileged-lane vocabulary the no-silent-rerun guarantee protects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedLaneClass {
    /// Terminal sessions.
    Terminal,
    /// Task / build runners.
    TaskRunner,
    /// Preview runtimes / dev servers.
    PreviewRuntime,
    /// Notebook kernels.
    Notebook,
    /// Collaboration / pipeline authority lanes.
    CollaborationAuthority,
    /// Debug sessions.
    DebugSession,
    /// Remote attach sessions.
    RemoteSession,
}

impl ProtectedLaneClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Terminal => "terminal",
            Self::TaskRunner => "task_runner",
            Self::PreviewRuntime => "preview_runtime",
            Self::Notebook => "notebook",
            Self::CollaborationAuthority => "collaboration_authority",
            Self::DebugSession => "debug_session",
            Self::RemoteSession => "remote_session",
        }
    }

    /// The restore hydrator `LiveSurfaceClass` token that exercises this lane,
    /// used by the harness to confirm the input fixture actually drives it.
    pub const fn live_surface_class_token(self) -> &'static str {
        match self {
            Self::Terminal => "terminal",
            Self::TaskRunner => "task_runner",
            Self::PreviewRuntime => "preview_runtime",
            Self::Notebook => "notebook",
            Self::CollaborationAuthority => "pipeline_view",
            Self::DebugSession => "debug_session",
            Self::RemoteSession => "remote_shell",
        }
    }
}

/// Closed accessibility-check vocabulary for the recovery cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessibilityCheckClass {
    /// Fully reachable and operable by keyboard.
    KeyboardComplete,
    /// Every action carries a screen-reader label.
    ScreenReaderLabeled,
    /// The surface respects reduced-motion preferences.
    ReducedMotionSafe,
}

impl AccessibilityCheckClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardComplete => "keyboard_complete",
            Self::ScreenReaderLabeled => "screen_reader_labeled",
            Self::ReducedMotionSafe => "reduced_motion_safe",
        }
    }
}

/// The closed accessibility-check set every recovery card must satisfy.
pub const REQUIRED_ACCESSIBILITY_CHECKS: [AccessibilityCheckClass; 3] = [
    AccessibilityCheckClass::KeyboardComplete,
    AccessibilityCheckClass::ScreenReaderLabeled,
    AccessibilityCheckClass::ReducedMotionSafe,
];

/// Closed claim-downgrade-trigger vocabulary for stale or regressed evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimDowngradeTriggerClass {
    /// The drill-spec fixture is missing from the corpus.
    DrillFixtureMissing,
    /// The replayed input fixture is missing.
    InputFixtureMissing,
    /// The observed recovery class no longer matches the claimed class.
    RecoveryClassRegressed,
    /// A protected session replayed silently during restore or crash recovery.
    SilentReplayDetected,
    /// A missing-dependency surface masqueraded as a ready, live surface.
    PlaceholderMasqueradedAsReady,
    /// A recovery card lost keyboard, screen-reader, or reduced-motion support.
    AccessibilityRegressed,
    /// The publishable evidence packet could not be produced.
    EvidencePacketMissing,
}

impl ClaimDowngradeTriggerClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DrillFixtureMissing => "drill_fixture_missing",
            Self::InputFixtureMissing => "input_fixture_missing",
            Self::RecoveryClassRegressed => "recovery_class_regressed",
            Self::SilentReplayDetected => "silent_replay_detected",
            Self::PlaceholderMasqueradedAsReady => "placeholder_masqueraded_as_ready",
            Self::AccessibilityRegressed => "accessibility_regressed",
            Self::EvidencePacketMissing => "evidence_packet_missing",
        }
    }
}

/// Refs to the owning crate/test/doc/schema and the replayed input fixture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillSourceRefs {
    /// Reviewer doc ref (repo-relative `docs/*`).
    pub doc_ref: String,
    /// Boundary schema ref (repo-relative `schemas/*`).
    pub schema_ref: String,
    /// First-consumer crate module ref (repo-relative `crates/*`).
    pub crate_consumer: String,
    /// Protected integration test ref (repo-relative `crates/*`).
    pub integration_test: String,
    /// Repo-relative input fixture the harness replays.
    pub input_fixture_ref: String,
}

/// Accessibility expectation the recovery card must satisfy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityExpectation {
    /// Fully keyboard reachable and operable.
    pub keyboard_complete: bool,
    /// Every action carries a screen-reader label.
    pub screen_reader_labeled: bool,
    /// The surface respects reduced-motion preferences.
    pub reduced_motion_safe: bool,
}

impl AccessibilityExpectation {
    /// Returns true when all three accessibility checks are satisfied.
    pub const fn all_satisfied(&self) -> bool {
        self.keyboard_complete && self.screen_reader_labeled && self.reduced_motion_safe
    }

    /// Closed accessibility-check tokens the expectation satisfies, in order.
    pub fn satisfied_checks(&self) -> Vec<AccessibilityCheckClass> {
        let mut out = Vec::new();
        if self.keyboard_complete {
            out.push(AccessibilityCheckClass::KeyboardComplete);
        }
        if self.screen_reader_labeled {
            out.push(AccessibilityCheckClass::ScreenReaderLabeled);
        }
        if self.reduced_motion_safe {
            out.push(AccessibilityCheckClass::ReducedMotionSafe);
        }
        out
    }
}

/// Linkage from a drill to the launch-bearing beta continuity claim it backs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimLinkage {
    /// Stable beta continuity row id this drill keeps provable.
    pub beta_continuity_row: String,
    /// Reviewer-safe statement of the daily-driver recovery claim.
    pub claim_text: String,
}

/// Safety baseline for one drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillSafety {
    /// Whether the drill is read-only diagnosis.
    pub read_only_diagnosis: bool,
    /// Whether raw private material is excluded by default.
    pub raw_private_material_excluded: bool,
    /// Whether the drill contains destructive resets (must be false).
    pub destructive_resets_present: bool,
    /// Whether user-authored files are preserved.
    pub preserves_user_authored_files: bool,
}

impl DrillSafety {
    /// Returns true when the metadata-safe baseline is met.
    pub const fn baseline_met(&self) -> bool {
        self.read_only_diagnosis
            && self.raw_private_material_excluded
            && !self.destructive_resets_present
            && self.preserves_user_authored_files
    }
}

/// One seeded crash-loop / restore-fidelity drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopRestoreDrill {
    /// Frozen schema version (1).
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable drill id.
    pub drill_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Closed condition class the drill covers.
    pub condition_class: DrillConditionClass,
    /// Closed drill kind (which evaluator the harness replays).
    pub drill_kind: DrillKind,
    /// Owning support / release lane for the drill.
    pub drill_owner_lane: String,
    /// Source refs (doc, schema, crate consumer, integration test, input fixture).
    pub source_refs: DrillSourceRefs,
    /// Claimed (headline) recovery outcome class the harness must observe.
    pub expected_recovery_outcome: RecoveryOutcomeClass,
    /// Additional recovery outcome classes the drill demonstrates beyond the
    /// headline aggregate (for example, the exactly-restored auxiliary window in
    /// an off-screen remap). Kept so the matrix shows the full fidelity
    /// spectrum on the covered rows.
    #[serde(default)]
    pub secondary_recovery_outcomes: Vec<RecoveryOutcomeClass>,
    /// Closed recovery-path tokens the card offers (a subset of what the live
    /// evaluator emits): crash-loop choice classes or restore safe-action
    /// classes.
    pub recovery_paths: Vec<String>,
    /// Privileged lanes this drill exercises and must keep from re-running.
    #[serde(default)]
    pub protected_lanes: Vec<ProtectedLaneClass>,
    /// Whether the drill requires the no-silent-rerun guarantee.
    pub no_silent_rerun_required: bool,
    /// Whether missing-dependency surfaces must reopen as truthful placeholders.
    pub truthful_placeholders_required: bool,
    /// Accessibility expectation for the recovery card.
    pub accessibility: AccessibilityExpectation,
    /// Linkage to the beta continuity claim the drill backs.
    pub claim_linkage: ClaimLinkage,
    /// Opaque ref to the publishable, metadata-safe evidence packet.
    pub evidence_packet_ref: String,
    /// Claim-downgrade triggers the drill declares.
    pub claim_downgrade_triggers: Vec<ClaimDowngradeTriggerClass>,
    /// Safety baseline.
    pub safety: DrillSafety,
    /// UTC timestamp when the drill fixture was emitted.
    pub emitted_at: String,
}

/// One fixture-bound entry in the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopRestoreCorpusEntry {
    /// Repository-relative drill-spec fixture path.
    pub fixture_ref: String,
    /// Parsed drill record.
    pub drill: CrashLoopRestoreDrill,
}

/// Validation violation emitted by the corpus validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusViolation {
    /// Stable check id.
    pub check_id: String,
    /// Drill id, fixture ref, or corpus id that failed.
    pub target_ref: String,
    /// Reviewer-facing message.
    pub message: String,
}

/// The crash-loop and restore-fidelity corpus loaded from checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopRestoreFidelityCorpus {
    /// Fixture-bound entries.
    pub entries: Vec<CrashLoopRestoreCorpusEntry>,
}

impl CrashLoopRestoreFidelityCorpus {
    /// Returns the parsed drills without their fixture-path wrappers.
    pub fn drills(&self) -> impl Iterator<Item = &CrashLoopRestoreDrill> {
        self.entries.iter().map(|entry| &entry.drill)
    }

    /// Returns the drill covering the given condition, if present.
    pub fn drill_for(&self, condition: DrillConditionClass) -> Option<&CrashLoopRestoreDrill> {
        self.drills()
            .find(|drill| drill.condition_class == condition)
    }

    /// Validates the corpus against the continuity-coverage contract.
    pub fn validate(&self) -> Vec<CorpusViolation> {
        let mut violations = Vec::new();

        if self.entries.is_empty() {
            push_violation(
                &mut violations,
                "corpus.empty",
                CORPUS_DIR,
                "corpus must contain at least one drill",
            );
            return violations;
        }

        let mut drill_ids = BTreeSet::new();
        let mut fixture_refs = BTreeSet::new();
        let mut evidence_refs = BTreeSet::new();
        let mut condition_seen: BTreeSet<DrillConditionClass> = BTreeSet::new();

        for entry in &self.entries {
            if !fixture_refs.insert(entry.fixture_ref.clone()) {
                push_violation(
                    &mut violations,
                    "corpus.duplicate_fixture_ref",
                    &entry.fixture_ref,
                    "fixture_ref must be unique within the corpus",
                );
            }
            let drill = &entry.drill;
            if !drill_ids.insert(drill.drill_id.clone()) {
                push_violation(
                    &mut violations,
                    "corpus.duplicate_drill_id",
                    &drill.drill_id,
                    "drill_id must be unique within the corpus",
                );
            }
            if !evidence_refs.insert(drill.evidence_packet_ref.clone()) {
                push_violation(
                    &mut violations,
                    "corpus.duplicate_evidence_packet_ref",
                    &drill.evidence_packet_ref,
                    "evidence_packet_ref must be unique within the corpus",
                );
            }
            if !condition_seen.insert(drill.condition_class) {
                push_violation(
                    &mut violations,
                    "corpus.duplicate_condition_class",
                    drill.condition_class.as_str(),
                    "each condition class must appear at most once",
                );
            }
            validate_drill(&mut violations, drill);
        }

        for required in required_condition_classes() {
            if !condition_seen.contains(required) {
                push_violation(
                    &mut violations,
                    "corpus.required_condition_missing",
                    required.as_str(),
                    format!(
                        "required condition {} has no seeded drill",
                        required.as_str()
                    ),
                );
            }
        }

        violations
    }

    /// Projects the validated corpus into a metadata-safe recovery-choice matrix.
    pub fn recovery_choice_matrix(
        &self,
        matrix_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> RecoveryChoiceMatrix {
        let rows = self
            .entries
            .iter()
            .map(RecoveryChoiceMatrixRow::from_entry)
            .collect::<Vec<_>>();
        let mut observed_set = BTreeSet::new();
        for row in &rows {
            observed_set.insert(row.recovery_outcome_class);
            for secondary in &row.secondary_recovery_outcomes {
                if let Some(class) = RecoveryOutcomeClass::from_token(secondary) {
                    observed_set.insert(class);
                }
            }
        }
        let observed = observed_set.into_iter().collect::<Vec<_>>();
        RecoveryChoiceMatrix {
            record_kind: RECOVERY_CHOICE_MATRIX_RECORD_KIND.to_owned(),
            schema_version: CRASH_LOOP_RESTORE_FIDELITY_SCHEMA_VERSION,
            matrix_id: matrix_id.into(),
            generated_at: generated_at.into(),
            corpus_manifest_ref: CORPUS_MANIFEST_REF.to_owned(),
            corpus_doc_ref: CORPUS_DOC_REF.to_owned(),
            report_ref: CORPUS_REPORT_REF.to_owned(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            required_condition_classes: required_condition_classes()
                .iter()
                .map(|condition| condition.as_str().to_owned())
                .collect(),
            observed_recovery_outcome_classes: observed
                .into_iter()
                .map(|outcome| outcome.as_str().to_owned())
                .collect(),
            rows,
        }
    }
}

/// Loads the checked-in crash-loop / restore-fidelity corpus.
///
/// # Errors
///
/// Returns a YAML parse error when a drill fixture does not match
/// [`CrashLoopRestoreDrill`].
pub fn current_corpus() -> Result<CrashLoopRestoreFidelityCorpus, serde_yaml::Error> {
    let entries = DRILL_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<CrashLoopRestoreDrill>(yaml).map(|drill| {
                CrashLoopRestoreCorpusEntry {
                    fixture_ref: (*fixture_ref).to_owned(),
                    drill,
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(CrashLoopRestoreFidelityCorpus { entries })
}

/// One projected row in the recovery-choice matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryChoiceMatrixRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Closed condition class.
    pub condition_class: DrillConditionClass,
    /// Stable drill id.
    pub drill_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Closed drill kind.
    pub drill_kind: DrillKind,
    /// Owning lane.
    pub drill_owner_lane: String,
    /// Repo-relative input fixture the harness replays.
    pub input_fixture_ref: String,
    /// Claimed (headline) recovery outcome class on this row.
    pub recovery_outcome_class: RecoveryOutcomeClass,
    /// Additional recovery outcome classes this row demonstrates.
    pub secondary_recovery_outcomes: Vec<String>,
    /// Closed recovery-path tokens the card offers.
    pub recovery_paths: Vec<String>,
    /// Protected-lane tokens this row exercises.
    pub protected_lanes: Vec<String>,
    /// Whether the no-silent-rerun guarantee is required on this row.
    pub no_silent_rerun_required: bool,
    /// Whether truthful placeholders are required on this row.
    pub truthful_placeholders_required: bool,
    /// Accessibility-check tokens the card satisfies.
    pub accessibility_checks: Vec<String>,
    /// Beta continuity row this drill keeps provable.
    pub beta_continuity_row: String,
    /// Reviewer-safe daily-driver recovery claim.
    pub claim_text: String,
    /// Opaque ref to the publishable evidence packet.
    pub evidence_packet_ref: String,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// First-consumer crate module ref.
    pub crate_consumer_ref: String,
    /// Protected integration-test ref.
    pub integration_test_ref: String,
    /// Closed claim-downgrade-trigger tokens the drill declares.
    pub claim_downgrade_trigger_classes: Vec<String>,
    /// Whether the row passes the metadata-safe baseline.
    pub metadata_safe_baseline_met: bool,
}

impl RecoveryChoiceMatrixRow {
    fn from_entry(entry: &CrashLoopRestoreCorpusEntry) -> Self {
        let drill = &entry.drill;
        let claim_downgrade_trigger_classes = {
            let mut tokens = drill
                .claim_downgrade_triggers
                .iter()
                .map(|trigger| trigger.as_str().to_owned())
                .collect::<Vec<_>>();
            tokens.sort();
            tokens.dedup();
            tokens
        };
        Self {
            record_kind: RECOVERY_CHOICE_MATRIX_ROW_RECORD_KIND.to_owned(),
            condition_class: drill.condition_class,
            drill_id: drill.drill_id.clone(),
            title: drill.title.clone(),
            drill_kind: drill.drill_kind,
            drill_owner_lane: drill.drill_owner_lane.clone(),
            input_fixture_ref: drill.source_refs.input_fixture_ref.clone(),
            recovery_outcome_class: drill.expected_recovery_outcome,
            secondary_recovery_outcomes: drill
                .secondary_recovery_outcomes
                .iter()
                .map(|outcome| outcome.as_str().to_owned())
                .collect(),
            recovery_paths: drill.recovery_paths.clone(),
            protected_lanes: drill
                .protected_lanes
                .iter()
                .map(|lane| lane.as_str().to_owned())
                .collect(),
            no_silent_rerun_required: drill.no_silent_rerun_required,
            truthful_placeholders_required: drill.truthful_placeholders_required,
            accessibility_checks: drill
                .accessibility
                .satisfied_checks()
                .iter()
                .map(|check| check.as_str().to_owned())
                .collect(),
            beta_continuity_row: drill.claim_linkage.beta_continuity_row.clone(),
            claim_text: drill.claim_linkage.claim_text.clone(),
            evidence_packet_ref: drill.evidence_packet_ref.clone(),
            doc_ref: drill.source_refs.doc_ref.clone(),
            schema_ref: drill.source_refs.schema_ref.clone(),
            crate_consumer_ref: drill.source_refs.crate_consumer.clone(),
            integration_test_ref: drill.source_refs.integration_test.clone(),
            claim_downgrade_trigger_classes,
            metadata_safe_baseline_met: drill.safety.baseline_met()
                && drill.accessibility.all_satisfied(),
        }
    }
}

/// Recovery-choice matrix projected from the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryChoiceMatrix {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Projection schema version.
    pub schema_version: u32,
    /// Stable matrix id.
    pub matrix_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Corpus manifest ref.
    pub corpus_manifest_ref: String,
    /// Reviewer doc ref.
    pub corpus_doc_ref: String,
    /// Reviewer-facing report ref.
    pub report_ref: String,
    /// Whether raw private material is excluded by default.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded by default.
    pub ambient_authority_excluded: bool,
    /// Required condition-class tokens covered by the matrix.
    pub required_condition_classes: Vec<String>,
    /// Recovery-outcome-class tokens observed across the covered rows.
    pub observed_recovery_outcome_classes: Vec<String>,
    /// Per-condition rows.
    pub rows: Vec<RecoveryChoiceMatrixRow>,
}

impl RecoveryChoiceMatrix {
    /// Returns true when every required condition has a row, every row meets the
    /// metadata-safe baseline, and the matrix shows the exact / compatible /
    /// layout-only / evidence-only recovery spectrum.
    pub fn is_export_safe(&self) -> bool {
        if !self.raw_private_material_excluded || !self.ambient_authority_excluded {
            return false;
        }
        let condition_set = self
            .rows
            .iter()
            .map(|row| row.condition_class.as_str().to_owned())
            .collect::<BTreeSet<_>>();
        for required in &self.required_condition_classes {
            if !condition_set.contains(required) {
                return false;
            }
        }
        if !self.rows.iter().all(|row| row.metadata_safe_baseline_met) {
            return false;
        }
        let observed = self
            .observed_recovery_outcome_classes
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        for required in [
            RecoveryOutcomeClass::ExactRestore,
            RecoveryOutcomeClass::CompatibleRestore,
            RecoveryOutcomeClass::LayoutOnly,
            RecoveryOutcomeClass::EvidenceOnly,
        ] {
            if !observed.contains(required.as_str()) {
                return false;
            }
        }
        true
    }

    /// Renders a support-safe, screen-reader-legible plaintext view.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Crash-loop and restore-fidelity recovery-choice matrix\n");
        out.push_str(&format!("  matrix id: {}\n", self.matrix_id));
        out.push_str(&format!(
            "  observed recovery outcomes: {}\n",
            self.observed_recovery_outcome_classes.join(", ")
        ));
        for row in &self.rows {
            out.push_str(&format!(
                "  - {} [{}] outcome={} no_silent_rerun={} placeholders={} a11y={}\n",
                row.condition_class.as_str(),
                row.drill_kind.as_str(),
                row.recovery_outcome_class.as_str(),
                row.no_silent_rerun_required,
                row.truthful_placeholders_required,
                row.accessibility_checks.join("+"),
            ));
        }
        out
    }
}

/// Strongly typed error returned by [`load_drill`].
#[derive(Debug)]
pub enum DrillLoadError {
    /// YAML parse error.
    Yaml(serde_yaml::Error),
}

impl fmt::Display for DrillLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yaml(err) => write!(f, "drill yaml parse error: {err}"),
        }
    }
}

impl Error for DrillLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Yaml(err) => Some(err),
        }
    }
}

impl From<serde_yaml::Error> for DrillLoadError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::Yaml(value)
    }
}

/// Parses one crash-loop / restore-fidelity drill YAML record.
///
/// # Errors
///
/// Returns [`DrillLoadError::Yaml`] when the YAML does not match
/// [`CrashLoopRestoreDrill`].
pub fn load_drill(yaml: &str) -> Result<CrashLoopRestoreDrill, DrillLoadError> {
    serde_yaml::from_str::<CrashLoopRestoreDrill>(yaml).map_err(DrillLoadError::from)
}

fn validate_drill(violations: &mut Vec<CorpusViolation>, drill: &CrashLoopRestoreDrill) {
    let target = drill.drill_id.as_str();

    if drill.schema_version != CRASH_LOOP_RESTORE_FIDELITY_SCHEMA_VERSION {
        push_violation(
            violations,
            "drill.schema_version",
            target,
            "schema_version must be 1",
        );
    }
    if drill.record_kind != DRILL_RECORD_KIND {
        push_violation(
            violations,
            "drill.record_kind",
            target,
            "record_kind must be crash_loop_restore_fidelity_drill_record",
        );
    }
    if drill.drill_id.trim().is_empty() {
        push_violation(
            violations,
            "drill.drill_id",
            target,
            "drill_id must be non-empty",
        );
    }
    if drill.title.trim().is_empty() {
        push_violation(violations, "drill.title", target, "title must be non-empty");
    }
    if drill.drill_owner_lane.trim().is_empty() {
        push_violation(
            violations,
            "drill.drill_owner_lane",
            target,
            "drill_owner_lane must be non-empty",
        );
    }

    // The drill kind must be the one admitted for the condition.
    if drill.drill_kind != drill.condition_class.admitted_drill_kind() {
        push_violation(
            violations,
            "drill.kind_mismatch",
            target,
            format!(
                "drill_kind {} is not admitted for condition {}",
                drill.drill_kind.as_str(),
                drill.condition_class.as_str()
            ),
        );
    }

    validate_source_refs(violations, target, &drill.source_refs, drill.drill_kind);

    if drill.recovery_paths.is_empty() {
        push_violation(
            violations,
            "drill.recovery_paths.empty",
            target,
            "recovery_paths must name at least one bounded recovery path",
        );
    }
    let mut seen_paths = BTreeSet::new();
    for path in &drill.recovery_paths {
        if path.trim().is_empty() {
            push_violation(
                violations,
                "drill.recovery_paths.empty_token",
                target,
                "recovery_paths token must be non-empty",
            );
        }
        if !seen_paths.insert(path.as_str()) {
            push_violation(
                violations,
                "drill.recovery_paths.duplicate",
                target,
                format!("duplicate recovery_path token {path}"),
            );
        }
    }

    // No-silent-rerun is mandatory wherever a privileged lane is exercised.
    if !drill.protected_lanes.is_empty() && !drill.no_silent_rerun_required {
        push_violation(
            violations,
            "drill.no_silent_rerun_required",
            target,
            "drills exercising a protected lane must require no_silent_rerun",
        );
    }
    let mut seen_lanes = BTreeSet::new();
    for lane in &drill.protected_lanes {
        if !seen_lanes.insert(*lane) {
            push_violation(
                violations,
                "drill.protected_lanes.duplicate",
                target,
                format!("duplicate protected lane {}", lane.as_str()),
            );
        }
    }
    if drill.condition_class == DrillConditionClass::NoSilentRerunProtectedLanes
        && drill.protected_lanes.is_empty()
    {
        push_violation(
            violations,
            "drill.protected_lanes.missing",
            target,
            "the no-silent-rerun condition must enumerate the protected lanes it exercises",
        );
    }

    if !drill.accessibility.all_satisfied() {
        push_violation(
            violations,
            "drill.accessibility",
            target,
            "recovery cards must be keyboard-complete, screen-reader-labeled, and reduced-motion-safe",
        );
    }

    if drill.claim_linkage.beta_continuity_row.trim().is_empty()
        || drill.claim_linkage.claim_text.trim().is_empty()
    {
        push_violation(
            violations,
            "drill.claim_linkage",
            target,
            "claim_linkage must name a beta_continuity_row and a claim_text",
        );
    }

    if drill.evidence_packet_ref.trim().is_empty() {
        push_violation(
            violations,
            "drill.evidence_packet_ref",
            target,
            "evidence_packet_ref must be non-empty",
        );
    }

    validate_downgrade_triggers(violations, target, &drill.claim_downgrade_triggers);
    validate_safety(violations, target, &drill.safety);

    if drill.emitted_at.trim().is_empty() {
        push_violation(
            violations,
            "drill.emitted_at",
            target,
            "emitted_at must be non-empty",
        );
    }
}

fn validate_source_refs(
    violations: &mut Vec<CorpusViolation>,
    target: &str,
    refs: &DrillSourceRefs,
    drill_kind: DrillKind,
) {
    for (field, value) in [
        ("doc_ref", refs.doc_ref.as_str()),
        ("schema_ref", refs.schema_ref.as_str()),
        ("crate_consumer", refs.crate_consumer.as_str()),
        ("integration_test", refs.integration_test.as_str()),
        ("input_fixture_ref", refs.input_fixture_ref.as_str()),
    ] {
        if value.trim().is_empty() {
            push_violation(
                violations,
                "drill.source_refs.empty",
                target,
                format!("source_refs.{field} must be non-empty"),
            );
        }
    }
    if !refs.doc_ref.starts_with("docs/") {
        push_violation(
            violations,
            "drill.source_refs.doc_ref",
            target,
            "source_refs.doc_ref must be a repo-relative docs/* path",
        );
    }
    if !refs.schema_ref.starts_with("schemas/") {
        push_violation(
            violations,
            "drill.source_refs.schema_ref",
            target,
            "source_refs.schema_ref must be a repo-relative schemas/* path",
        );
    }
    if !refs.crate_consumer.starts_with("crates/") {
        push_violation(
            violations,
            "drill.source_refs.crate_consumer",
            target,
            "source_refs.crate_consumer must be a repo-relative crates/* path",
        );
    }
    if !refs.integration_test.starts_with("crates/") {
        push_violation(
            violations,
            "drill.source_refs.integration_test",
            target,
            "source_refs.integration_test must be a repo-relative crates/* path",
        );
    }
    if !refs.input_fixture_ref.starts_with("fixtures/") {
        push_violation(
            violations,
            "drill.source_refs.input_fixture_ref",
            target,
            "source_refs.input_fixture_ref must be a repo-relative fixtures/* path",
        );
    }
    if !refs
        .input_fixture_ref
        .ends_with(drill_kind.input_fixture_suffix())
    {
        push_violation(
            violations,
            "drill.source_refs.input_fixture_suffix",
            target,
            format!(
                "input_fixture_ref must end with {} for a {} drill",
                drill_kind.input_fixture_suffix(),
                drill_kind.as_str()
            ),
        );
    }
}

fn validate_downgrade_triggers(
    violations: &mut Vec<CorpusViolation>,
    target: &str,
    triggers: &[ClaimDowngradeTriggerClass],
) {
    if triggers.is_empty() {
        push_violation(
            violations,
            "drill.claim_downgrade_triggers.empty",
            target,
            "claim_downgrade_triggers must declare at least one trigger",
        );
        return;
    }
    let seen = triggers.iter().copied().collect::<BTreeSet<_>>();
    if seen.len() != triggers.len() {
        push_violation(
            violations,
            "drill.claim_downgrade_triggers.duplicate",
            target,
            "claim_downgrade_triggers must not repeat a trigger",
        );
    }
    for required in REQUIRED_DOWNGRADE_TRIGGERS {
        if !seen.contains(required) {
            push_violation(
                violations,
                "drill.claim_downgrade_triggers.required_missing",
                target,
                format!(
                    "claim_downgrade_triggers must cover required trigger {}",
                    required.as_str()
                ),
            );
        }
    }
}

fn validate_safety(violations: &mut Vec<CorpusViolation>, target: &str, safety: &DrillSafety) {
    if !safety.read_only_diagnosis {
        push_violation(
            violations,
            "drill.safety.read_only_diagnosis",
            target,
            "diagnosis must be read-only",
        );
    }
    if !safety.raw_private_material_excluded {
        push_violation(
            violations,
            "drill.safety.raw_private_material_excluded",
            target,
            "raw private material must be excluded by default",
        );
    }
    if safety.destructive_resets_present {
        push_violation(
            violations,
            "drill.safety.destructive_resets_present",
            target,
            "destructive_resets_present must be false",
        );
    }
    if !safety.preserves_user_authored_files {
        push_violation(
            violations,
            "drill.safety.preserves_user_authored_files",
            target,
            "preserves_user_authored_files must be true",
        );
    }
}

fn push_violation(
    violations: &mut Vec<CorpusViolation>,
    check_id: impl Into<String>,
    target_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(CorpusViolation {
        check_id: check_id.into(),
        target_ref: target_ref.into(),
        message: message.into(),
    });
}
