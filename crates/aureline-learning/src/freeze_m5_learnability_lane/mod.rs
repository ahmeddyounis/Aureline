//! Frozen M5 learnability lane: one controlled vocabulary and one
//! family-by-term matrix that every claimed M5 feature family routes its
//! onboarding, guided-practice, educational-AI, and progress surfaces through.
//!
//! Where [`crate::m5_feature_family_learning_rails`] qualifies the *bundles* a
//! family ships, this module sits one level above: it **freezes** the
//! controlled vocabulary for learning mode, tour package, guided exercise,
//! glossary pack, contextual why-now card, educational AI, practice/sandbox
//! indicator, learning digest, and progress snapshot, and maps every claimed M5
//! feature-family learnability surface onto one canonical lane. The goal is that
//! later implementation rows reuse these frozen terms and lane refs instead of
//! inventing feature-local coachmarks or parallel onboarding state.
//!
//! ## What is frozen
//!
//! - **Controlled vocabulary.** [`LearnabilityTerm`] enumerates the nine frozen
//!   terms. [`VocabularyEntry`] pins each term's definition, explain-vs-do
//!   posture, mutation-path class, data-ownership class, and authority-change
//!   posture so no surface can quietly redefine a term.
//! - **Lane matrix.** One [`LearnabilityLaneRow`] per claimed family per term
//!   binds the surface to a single [`canonical_lane_ref`](LearnabilityLaneRow::canonical_lane_ref).
//!   Cross-cutting terms (learning mode, educational AI, learning digest) share
//!   one canonical lane across all families, proving no family forks its own.
//! - **Educational-AI boundary.** [`EducationalAiBoundary`] freezes the
//!   explain-versus-do separation: educational AI explains freely but any "do"
//!   rides the same preview/approval/rollback path as ordinary work and never
//!   mutates live state directly.
//!
//! ## Invariants enforced
//!
//! - **No hidden coachmarks, no private mutation paths.** Every row must be
//!   command-backed and inspectable; rows that hide behind feature-local
//!   coachmarks or private mutation paths narrow below Stable and fail
//!   validation.
//! - **Explain stays separate from do.** Any row that conflates explain/apply or
//!   exposes a hidden direct-mutation path narrows below Stable.
//! - **User-owned, local-first progress.** Progress and learning state is
//!   user-owned and local-first; repo-visible or telemetry-grade ownership
//!   narrows below Stable.
//! - **Offline/mirror parity, support/export parity.** Learning state surfaces
//!   on local-only and mirrored profiles with an explicit freshness label, and
//!   the same state is inspectable in support export without credential bodies.
//!
//! ## Canonical truth source
//!
//! [`seeded_m5_learnability_lane_freeze`] produces the canonical freeze packet.
//! Help/About, release-center, support-export, and docs/migration surfaces
//! ingest it rather than rephrasing learnability state by hand.
//!
//! - Schema: [`M5_LEARNABILITY_LANE_SCHEMA_REF`]
//! - Fixture: [`M5_LEARNABILITY_LANE_FIXTURE_REF`]
//! - Artifact: [`M5_LEARNABILITY_LANE_ARTIFACT_REF`]
//! - Doc: [`M5_LEARNABILITY_LANE_DOC_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::m5_feature_family_learning_rails::{
    M5LearningSurfaceFamily, MirrorParityPosture, M5_FEATURE_FAMILY_LEARNING_SCHEMA_REF,
};
use crate::qualify_learning_mode_guided_tours_and_teaching_sessions::{
    ExplainApplyClass, QualificationVerdict, GUIDED_LEARNING_CONTRACTS_SCHEMA_REF,
};

// ── Schema-version and record-kind constants ─────────────────────────────────

/// Integer schema version for the frozen M5 learnability-lane records. Bumped
/// only on breaking payload changes.
pub const M5_LEARNABILITY_LANE_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`VocabularyEntry`].
pub const VOCABULARY_ENTRY_RECORD_KIND: &str = "learnability_vocabulary_entry";

/// Record kind for [`LearnabilityLaneRow`].
pub const LEARNABILITY_LANE_ROW_RECORD_KIND: &str = "learnability_lane_row";

/// Record kind for [`M5LearnabilityLaneFreeze`].
pub const M5_LEARNABILITY_LANE_FREEZE_RECORD_KIND: &str = "m5_learnability_lane_freeze";

// ── Canonical path constants ──────────────────────────────────────────────────

/// Repository-relative path to the frozen learnability-lane schema.
pub const M5_LEARNABILITY_LANE_SCHEMA_REF: &str = "schemas/help/m5-learnability-lane.schema.json";

/// Repository-relative path to the canonical freeze-packet fixture.
pub const M5_LEARNABILITY_LANE_FIXTURE_REF: &str =
    "fixtures/help/m5/learnability-regression/m5_learnability_lane_freeze.json";

/// Repository-relative path to the proof artifact.
pub const M5_LEARNABILITY_LANE_ARTIFACT_REF: &str =
    "artifacts/ux/m5/learnability-freeze-packet/freeze-the-m5-learning-mode-tour-package-guided-exercise-and-progress-snapshot-matrix.md";

/// Repository-relative path to the public doc.
pub const M5_LEARNABILITY_LANE_DOC_REF: &str = "docs/m5/learning-mode-and-guided-exercises.md";

// ── Controlled vocabulary ─────────────────────────────────────────────────────

/// One frozen term in the M5 learnability controlled vocabulary.
///
/// These nine terms are the only vocabulary M5 feature families may use to name
/// their learnability surfaces. Adding a term is additive-minor; repurposing an
/// existing term is breaking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearnabilityTerm {
    /// Opt-in, user-owned learning-mode profile that tunes tip intensity,
    /// jargon, and explanation posture without changing authority or trust.
    LearningMode,
    /// Versioned, command-backed guided-tour package.
    TourPackage,
    /// Hands-on guided exercise whose Apply steps ride the standard
    /// preview/approval/rollback path.
    GuidedExercise,
    /// Citation-backed glossary of the terms a flow uses.
    GlossaryPack,
    /// In-place contextual card explaining why a surface matters right now.
    ContextualWhyNowCard,
    /// Educational AI that explains freely and keeps "do" behind the standard
    /// preview/approval fence.
    EducationalAi,
    /// Indicator that distinguishes practice/sandbox surfaces from live work.
    PracticeSandboxIndicator,
    /// User-owned digest summarizing learning progress and resume points.
    LearningDigest,
    /// User-owned, local-first snapshot of learning progress and dismissals.
    ProgressSnapshot,
}

impl LearnabilityTerm {
    /// Stable string token for records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LearningMode => "learning_mode",
            Self::TourPackage => "tour_package",
            Self::GuidedExercise => "guided_exercise",
            Self::GlossaryPack => "glossary_pack",
            Self::ContextualWhyNowCard => "contextual_why_now_card",
            Self::EducationalAi => "educational_ai",
            Self::PracticeSandboxIndicator => "practice_sandbox_indicator",
            Self::LearningDigest => "learning_digest",
            Self::ProgressSnapshot => "progress_snapshot",
        }
    }

    /// The full frozen vocabulary, in canonical order.
    pub const ALL: [LearnabilityTerm; 9] = [
        Self::LearningMode,
        Self::TourPackage,
        Self::GuidedExercise,
        Self::GlossaryPack,
        Self::ContextualWhyNowCard,
        Self::EducationalAi,
        Self::PracticeSandboxIndicator,
        Self::LearningDigest,
        Self::ProgressSnapshot,
    ];

    /// One-line frozen definition for this term.
    pub const fn frozen_definition(self) -> &'static str {
        match self {
            Self::LearningMode => {
                "Opt-in, user-owned profile that tunes tip intensity, jargon, and explanation posture; it never changes authority, trust, ownership, or mutation-approval semantics."
            }
            Self::TourPackage => {
                "Versioned guided-tour package whose every step reuses the same command ids, preview sheets, and approval paths as ordinary work."
            }
            Self::GuidedExercise => {
                "Hands-on guided practice whose Apply steps are reversible and ride the standard preview/approval/rollback fence."
            }
            Self::GlossaryPack => {
                "Citation-backed glossary of the terms a flow uses, each citing an authoritative command or docs anchor."
            }
            Self::ContextualWhyNowCard => {
                "In-place, read-only card that explains why a surface matters now and links back to the authoritative command and docs."
            }
            Self::EducationalAi => {
                "AI that explains freely but keeps 'do' separate: any mutation rides the same preview/approval/rollback path as ordinary work and never mutates live state directly."
            }
            Self::PracticeSandboxIndicator => {
                "Explicit indicator that distinguishes a practice/sandbox surface from live work so a learner never confuses the two."
            }
            Self::LearningDigest => {
                "User-owned digest summarizing learning progress, dismissals, and resume points; local-first and support-export safe."
            }
            Self::ProgressSnapshot => {
                "User-owned, local-first snapshot of learning progress and dismissals that survives restart and is safe for support export."
            }
        }
    }

    /// Frozen explain-vs-do posture for this term.
    ///
    /// Terms that can run a command (tours, exercises, educational AI) sit at
    /// [`ExplainApplyClass::ApplyRequiresApproval`]; everything else is
    /// [`ExplainApplyClass::ReadOnly`].
    pub const fn frozen_explain_apply(self) -> ExplainApplyClass {
        match self {
            Self::TourPackage | Self::GuidedExercise | Self::EducationalAi => {
                ExplainApplyClass::ApplyRequiresApproval
            }
            Self::LearningMode
            | Self::GlossaryPack
            | Self::ContextualWhyNowCard
            | Self::PracticeSandboxIndicator
            | Self::LearningDigest
            | Self::ProgressSnapshot => ExplainApplyClass::ReadOnly,
        }
    }

    /// Frozen mutation-path class for this term.
    pub const fn frozen_mutation_path(self) -> MutationPathClass {
        match self {
            Self::TourPackage | Self::GuidedExercise | Self::EducationalAi => {
                MutationPathClass::PreviewApprovalRequired
            }
            Self::LearningMode
            | Self::GlossaryPack
            | Self::ContextualWhyNowCard
            | Self::PracticeSandboxIndicator
            | Self::LearningDigest
            | Self::ProgressSnapshot => MutationPathClass::ReadOnlyNoMutation,
        }
    }

    /// Whether this term is cross-cutting — governed by one shared canonical lane
    /// across every family rather than per-family.
    pub const fn is_cross_cutting(self) -> bool {
        matches!(
            self,
            Self::LearningMode | Self::EducationalAi | Self::LearningDigest
        )
    }
}

// ── Mutation-path class ───────────────────────────────────────────────────────

/// How a learnability surface may mutate state.
///
/// Only read-only and preview/approval-gated paths qualify. A hidden
/// direct-mutation path is the explicit anti-pattern the freeze forbids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationPathClass {
    /// Surface does not mutate workspace state.
    ReadOnlyNoMutation,
    /// Any mutation rides the standard preview/approval/rollback fence.
    PreviewApprovalRequired,
    /// Surface mutates directly without the standard fence; narrows below Stable.
    HiddenDirectMutation,
}

impl MutationPathClass {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyNoMutation => "read_only_no_mutation",
            Self::PreviewApprovalRequired => "preview_approval_required",
            Self::HiddenDirectMutation => "hidden_direct_mutation",
        }
    }

    /// Returns true when the class satisfies Stable mutation-path requirements.
    pub const fn qualifies_stable(self) -> bool {
        matches!(
            self,
            Self::ReadOnlyNoMutation | Self::PreviewApprovalRequired
        )
    }
}

// ── Data-ownership class ──────────────────────────────────────────────────────

/// Who owns a learnability surface's state.
///
/// Learning progress is user-owned and local-first by default. Repo-visible or
/// telemetry-grade ownership narrows below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataOwnershipClass {
    /// State is user-owned, local-first, and shared only by explicit promotion.
    UserOwnedLocalFirst,
    /// State is visible to the repository; narrows below Stable.
    RepoVisibleShared,
    /// State is read at telemetry grade by a background service; narrows below
    /// Stable.
    TelemetryGradeShared,
}

impl DataOwnershipClass {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserOwnedLocalFirst => "user_owned_local_first",
            Self::RepoVisibleShared => "repo_visible_shared",
            Self::TelemetryGradeShared => "telemetry_grade_shared",
        }
    }

    /// Returns true when the class satisfies Stable ownership requirements.
    pub const fn qualifies_stable(self) -> bool {
        matches!(self, Self::UserOwnedLocalFirst)
    }
}

// ── Vocabulary entry ──────────────────────────────────────────────────────────

/// A frozen controlled-vocabulary entry for one [`LearnabilityTerm`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VocabularyEntry {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// The frozen term.
    pub term: LearnabilityTerm,
    /// Stable string token for the term.
    pub token: String,
    /// One-line frozen definition.
    pub definition: String,
    /// Frozen explain-vs-do posture.
    pub explain_apply_class: ExplainApplyClass,
    /// Frozen mutation-path class.
    pub mutation_path_class: MutationPathClass,
    /// Frozen data-ownership class.
    pub data_ownership_class: DataOwnershipClass,
    /// Whether the term must be command-backed (never tutorial-only).
    pub command_backed_required: bool,
    /// Whether the term may change authority/trust boundaries. MUST be false.
    pub authority_boundary_change_allowed: bool,
}

/// Builds the frozen [`VocabularyEntry`] for `term`.
pub fn vocabulary_entry(term: LearnabilityTerm) -> VocabularyEntry {
    VocabularyEntry {
        record_kind: VOCABULARY_ENTRY_RECORD_KIND.to_string(),
        schema_version: M5_LEARNABILITY_LANE_SCHEMA_VERSION,
        term,
        token: term.as_str().to_string(),
        definition: term.frozen_definition().to_string(),
        explain_apply_class: term.frozen_explain_apply(),
        mutation_path_class: term.frozen_mutation_path(),
        data_ownership_class: DataOwnershipClass::UserOwnedLocalFirst,
        command_backed_required: true,
        authority_boundary_change_allowed: false,
    }
}

// ── Support / export parity ───────────────────────────────────────────────────

/// Support-export parity posture for one lane row.
///
/// The same learning state shown in-product must be inspectable in support
/// export, must match the in-product state, and must never carry credential
/// bodies or raw provider payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportParity {
    /// Whether the row's state is inspectable in support export.
    pub inspectable_in_support_export: bool,
    /// Whether the exported state matches the in-product state.
    pub matches_in_product_state: bool,
    /// Whether the export carries no credential bodies / raw payloads.
    pub carries_no_credential_bodies: bool,
    /// Named reason when parity is inadequate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
}

impl SupportExportParity {
    /// Returns true when the posture satisfies Stable support-export parity.
    pub fn qualifies_stable(&self) -> bool {
        self.inspectable_in_support_export
            && self.matches_in_product_state
            && self.carries_no_credential_bodies
    }
}

// ── Educational-AI boundary ───────────────────────────────────────────────────

/// Frozen explain-versus-do boundary for educational AI across the lane.
///
/// Educational AI may explain freely, but "do" stays separate: any mutation
/// rides the same preview/approval/rollback path as ordinary work, and the AI
/// never mutates live state directly. A practice/sandbox indicator distinguishes
/// rehearsal from live work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EducationalAiBoundary {
    /// Whether explain and do are separate verbs. MUST be true.
    pub explain_and_do_separate: bool,
    /// Whether "do" requires the same preview/approval path as ordinary work.
    /// MUST be true.
    pub do_requires_same_preview_approval: bool,
    /// Whether the AI may mutate live state directly. MUST be false.
    pub can_mutate_live_state_directly: bool,
    /// Whether a practice/sandbox indicator is present for rehearsal surfaces.
    pub practice_sandbox_indicator_present: bool,
    /// Named reason when the boundary is inadequate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
}

impl EducationalAiBoundary {
    /// Returns true when the boundary satisfies the frozen explain-vs-do
    /// separation.
    pub fn qualifies_stable(&self) -> bool {
        self.explain_and_do_separate
            && self.do_requires_same_preview_approval
            && !self.can_mutate_live_state_directly
            && self.practice_sandbox_indicator_present
    }
}

// ── Lane row ──────────────────────────────────────────────────────────────────

/// One cell of the frozen lane matrix: how a family routes one learnability
/// term through the canonical lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearnabilityLaneRow {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// The feature family this row covers.
    pub family: M5LearningSurfaceFamily,
    /// The learnability term this row covers.
    pub term: LearnabilityTerm,
    /// Opaque surface token (`<family>:<term>`).
    pub surface_token: String,
    /// The single canonical lane this surface routes through. Cross-cutting
    /// terms share one ref across every family.
    pub canonical_lane_ref: String,
    /// Whether the surface is command-backed (never a tutorial-only shortcut).
    pub command_backed: bool,
    /// Whether the surface hides behind a feature-local coachmark. MUST be false.
    pub hidden_feature_local_coachmark: bool,
    /// Whether the surface mutates through a private path. MUST be false.
    pub private_mutation_path: bool,
    /// Explain-vs-do posture for this row.
    pub explain_apply_class: ExplainApplyClass,
    /// Mutation-path class for this row.
    pub mutation_path_class: MutationPathClass,
    /// Data-ownership class for this row.
    pub data_ownership_class: DataOwnershipClass,
    /// Offline/mirror parity posture for this row.
    pub mirror_parity: MirrorParityPosture,
    /// Support-export parity posture for this row.
    pub support_export_parity: SupportExportParity,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
}

impl LearnabilityLaneRow {
    /// Recomputes [`verdict`](Self::verdict) and
    /// [`narrowing_reasons`](Self::narrowing_reasons) from the row's evidence.
    pub fn sync_verdict(&mut self) {
        let (verdict, reasons) = derive_lane_row_verdict(self);
        self.verdict = verdict;
        self.narrowing_reasons = reasons;
    }
}

/// Derives a lane row's verdict and narrowing reasons from its evidence.
///
/// A row qualifies Stable only when it is command-backed, free of hidden
/// coachmarks and private mutation paths, explain/apply-separated, user-owned
/// and local-first, and proves offline/mirror and support-export parity.
pub fn derive_lane_row_verdict(row: &LearnabilityLaneRow) -> (QualificationVerdict, Vec<String>) {
    let mut verdict = QualificationVerdict::QualifiedStable;
    let mut reasons: Vec<String> = Vec::new();
    let label = &row.surface_token;

    if row.hidden_feature_local_coachmark {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!("{label}: hidden_feature_local_coachmark"));
    }
    if row.private_mutation_path {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!("{label}: private_mutation_path"));
    }
    if !row.command_backed {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!("{label}: not_command_backed"));
    }
    if !row.explain_apply_class.qualifies_stable() {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!(
            "{label}: explain_apply_conflated: {}",
            row.explain_apply_class.as_str()
        ));
    }
    if !row.mutation_path_class.qualifies_stable() {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!(
            "{label}: mutation_path: {}",
            row.mutation_path_class.as_str()
        ));
    }
    if !row.data_ownership_class.qualifies_stable() {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!(
            "{label}: data_ownership: {}",
            row.data_ownership_class.as_str()
        ));
    }
    if !row.mirror_parity.qualifies_stable() {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        if let Some(r) = &row.mirror_parity.narrowing_reason {
            reasons.push(format!("{label}: mirror_parity: {r}"));
        } else {
            reasons.push(format!("{label}: mirror_parity_inadequate"));
        }
    }
    if !row.support_export_parity.qualifies_stable() {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        if let Some(r) = &row.support_export_parity.narrowing_reason {
            reasons.push(format!("{label}: support_export: {r}"));
        } else {
            reasons.push(format!("{label}: support_export_parity_inadequate"));
        }
    }

    reasons.sort();
    reasons.dedup();
    (verdict, reasons)
}

// ── Freeze packet ─────────────────────────────────────────────────────────────

/// The frozen M5 learnability lane: controlled vocabulary, lane matrix, and
/// educational-AI boundary in one canonical packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearnabilityLaneFreeze {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this freeze.
    pub freeze_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Schema, docs, and contract refs this freeze consumes.
    pub contract_refs: BTreeMap<String, String>,
    /// The frozen controlled vocabulary — one entry per term.
    pub vocabulary: Vec<VocabularyEntry>,
    /// The frozen explain-versus-do boundary for educational AI.
    pub educational_ai_boundary: EducationalAiBoundary,
    /// The lane matrix — one row per claimed family per term.
    pub lane_rows: Vec<LearnabilityLaneRow>,
    /// Overall derived verdict — the strictest verdict across all rows folded
    /// with the educational-AI boundary.
    pub overall_verdict: QualificationVerdict,
    /// Named narrowing reasons aggregated across rows (empty when
    /// overall_verdict is QualifiedStable).
    #[serde(default)]
    pub overall_narrowing_reasons: Vec<String>,
}

impl M5LearnabilityLaneFreeze {
    /// Recomputes every row verdict and the overall verdict from current
    /// evidence, writing them back.
    pub fn sync_verdicts(&mut self) {
        let mut overall = QualificationVerdict::QualifiedStable;
        let mut reasons: Vec<String> = Vec::new();
        for row in &mut self.lane_rows {
            row.sync_verdict();
            overall = overall.meet(row.verdict);
            reasons.extend(row.narrowing_reasons.iter().cloned());
        }
        if !self.educational_ai_boundary.qualifies_stable() {
            overall = overall.meet(QualificationVerdict::NarrowedBeta);
            if let Some(r) = &self.educational_ai_boundary.narrowing_reason {
                reasons.push(format!("educational_ai_boundary: {r}"));
            } else {
                reasons.push("educational_ai_boundary_inadequate".to_string());
            }
        }
        reasons.sort();
        reasons.dedup();
        self.overall_verdict = overall;
        self.overall_narrowing_reasons = reasons;
    }

    /// Returns the lane row for `(family, term)`, if present.
    pub fn row(
        &self,
        family: M5LearningSurfaceFamily,
        term: LearnabilityTerm,
    ) -> Option<&LearnabilityLaneRow> {
        self.lane_rows
            .iter()
            .find(|r| r.family == family && r.term == term)
    }
}

// ── Lane builders ─────────────────────────────────────────────────────────────

const GENERATED_AT: &str = "2026-06-19T13:00:00Z";

/// Stable canonical-lane ref for a cross-cutting term (shared across families).
fn cross_cutting_lane_ref(term: LearnabilityTerm) -> String {
    format!("learning:m5:canonical_lane:{}", term.as_str())
}

/// Per-family canonical-lane ref — the family's learning bundle in the rails
/// manifest.
fn family_lane_ref(family: M5LearningSurfaceFamily) -> String {
    format!("learning:m5:family_bundle:{}", family.as_str())
}

fn stable_parity(freshness: &str) -> MirrorParityPosture {
    MirrorParityPosture {
        available_offline: true,
        available_on_mirror: true,
        freshness_label: freshness.to_string(),
        explicit_freshness_disclosed: true,
        silent_dead_link_on_stale: false,
        narrowing_reason: None,
    }
}

fn not_yet_mirror_synced_parity() -> MirrorParityPosture {
    MirrorParityPosture {
        available_offline: true,
        available_on_mirror: false,
        freshness_label: "local_only_disclosed".to_string(),
        explicit_freshness_disclosed: true,
        silent_dead_link_on_stale: false,
        narrowing_reason: Some("learning_pack_not_yet_mirror_synced".to_string()),
    }
}

fn stable_support_export() -> SupportExportParity {
    SupportExportParity {
        inspectable_in_support_export: true,
        matches_in_product_state: true,
        carries_no_credential_bodies: true,
        narrowing_reason: None,
    }
}

/// Builds one lane row for `(family, term)` from the term's frozen posture and
/// the supplied parity postures.
fn lane_row(
    family: M5LearningSurfaceFamily,
    term: LearnabilityTerm,
    mirror_parity: MirrorParityPosture,
) -> LearnabilityLaneRow {
    let canonical_lane_ref = if term.is_cross_cutting() {
        cross_cutting_lane_ref(term)
    } else {
        family_lane_ref(family)
    };
    let mut row = LearnabilityLaneRow {
        record_kind: LEARNABILITY_LANE_ROW_RECORD_KIND.to_string(),
        schema_version: M5_LEARNABILITY_LANE_SCHEMA_VERSION,
        family,
        term,
        surface_token: format!("{}:{}", family.as_str(), term.as_str()),
        canonical_lane_ref,
        command_backed: true,
        hidden_feature_local_coachmark: false,
        private_mutation_path: false,
        explain_apply_class: term.frozen_explain_apply(),
        mutation_path_class: term.frozen_mutation_path(),
        data_ownership_class: DataOwnershipClass::UserOwnedLocalFirst,
        mirror_parity,
        support_export_parity: stable_support_export(),
        verdict: QualificationVerdict::QualifiedStable,
        narrowing_reasons: vec![],
    };
    row.sync_verdict();
    row
}

// ── Seeded freeze packet ──────────────────────────────────────────────────────

/// Returns the seeded, canonical M5 learnability-lane freeze packet.
///
/// The packet freezes all nine vocabulary terms and maps the full matrix of nine
/// claimed M5 feature families against them — eighty-one lane rows in total. Most
/// rows qualify Stable. The `preview` family's `tour_package` and
/// `guided_exercise` rows narrow to Beta because their learning pack is not yet
/// mirror-synced, demonstrating the narrowing invariant in line with the
/// per-family learning-rails manifest.
pub fn seeded_m5_learnability_lane_freeze() -> M5LearnabilityLaneFreeze {
    let vocabulary: Vec<VocabularyEntry> = LearnabilityTerm::ALL
        .iter()
        .copied()
        .map(vocabulary_entry)
        .collect();

    let mut lane_rows: Vec<LearnabilityLaneRow> = Vec::new();
    for family in M5LearningSurfaceFamily::ALL {
        for term in LearnabilityTerm::ALL {
            // The preview family's pack-backed walkthroughs are not yet
            // mirror-synced; everything else proves full offline/mirror parity.
            let preview_pack_backed = family == M5LearningSurfaceFamily::Preview
                && matches!(
                    term,
                    LearnabilityTerm::TourPackage | LearnabilityTerm::GuidedExercise
                );
            let mirror_parity = if preview_pack_backed {
                not_yet_mirror_synced_parity()
            } else {
                stable_parity("live_authoritative")
            };
            lane_rows.push(lane_row(family, term, mirror_parity));
        }
    }

    let educational_ai_boundary = EducationalAiBoundary {
        explain_and_do_separate: true,
        do_requires_same_preview_approval: true,
        can_mutate_live_state_directly: false,
        practice_sandbox_indicator_present: true,
        narrowing_reason: None,
    };

    let mut contract_refs = BTreeMap::new();
    contract_refs.insert(
        "m5_learnability_lane_schema".to_string(),
        M5_LEARNABILITY_LANE_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "guided_learning_contracts_schema".to_string(),
        GUIDED_LEARNING_CONTRACTS_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "m5_feature_family_learning_rails_schema".to_string(),
        M5_FEATURE_FAMILY_LEARNING_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "artifact_doc".to_string(),
        M5_LEARNABILITY_LANE_ARTIFACT_REF.to_string(),
    );
    contract_refs.insert(
        "public_doc".to_string(),
        M5_LEARNABILITY_LANE_DOC_REF.to_string(),
    );
    contract_refs.insert(
        "canonical_fixture".to_string(),
        M5_LEARNABILITY_LANE_FIXTURE_REF.to_string(),
    );

    let mut freeze = M5LearnabilityLaneFreeze {
        record_kind: M5_LEARNABILITY_LANE_FREEZE_RECORD_KIND.to_string(),
        schema_version: M5_LEARNABILITY_LANE_SCHEMA_VERSION,
        freeze_id: "m5-learnability-lane:freeze:2026.06.19-01".to_string(),
        generated_at: GENERATED_AT.to_string(),
        contract_refs,
        vocabulary,
        educational_ai_boundary,
        lane_rows,
        overall_verdict: QualificationVerdict::QualifiedStable,
        overall_narrowing_reasons: vec![],
    };
    freeze.sync_verdicts();
    freeze
}

// ── Validation ────────────────────────────────────────────────────────────────

/// A typed validation error from [`validate_m5_learnability_lane`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LearnabilityLaneValidationError {
    /// Opaque id of the subject that failed.
    pub subject_id: String,
    /// Human-readable description of the failure.
    pub message: String,
}

impl std::fmt::Display for M5LearnabilityLaneValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.subject_id, self.message)
    }
}

/// Validates a [`M5LearnabilityLaneFreeze`] against the frozen learnability
/// invariants and returns any violations as typed errors.
///
/// # Errors
///
/// Returns a non-empty `Vec` when the controlled vocabulary is incomplete or
/// duplicated, when a vocabulary entry allows an authority-boundary change or a
/// non-user-owned ownership class, when the educational-AI boundary conflates
/// explain/do, when the lane matrix does not cover every claimed family and
/// term, when a row hides behind a coachmark or a private mutation path, when a
/// cross-cutting term forks more than one canonical lane, or when any row's
/// stored verdict diverges from the verdict derived from its evidence.
pub fn validate_m5_learnability_lane(
    freeze: &M5LearnabilityLaneFreeze,
) -> Result<(), Vec<M5LearnabilityLaneValidationError>> {
    let mut errors: Vec<M5LearnabilityLaneValidationError> = Vec::new();

    // ── Controlled vocabulary: complete, unique, and frozen. ──
    let mut seen_terms: BTreeSet<LearnabilityTerm> = BTreeSet::new();
    for entry in &freeze.vocabulary {
        if !seen_terms.insert(entry.term) {
            errors.push(M5LearnabilityLaneValidationError {
                subject_id: format!("vocabulary:{}", entry.term.as_str()),
                message: "duplicate vocabulary term".to_string(),
            });
        }
        if entry.authority_boundary_change_allowed {
            errors.push(M5LearnabilityLaneValidationError {
                subject_id: format!("vocabulary:{}", entry.term.as_str()),
                message: "vocabulary term allows an authority-boundary change".to_string(),
            });
        }
        if !entry.data_ownership_class.qualifies_stable() {
            errors.push(M5LearnabilityLaneValidationError {
                subject_id: format!("vocabulary:{}", entry.term.as_str()),
                message: "vocabulary term is not user-owned / local-first".to_string(),
            });
        }
        if entry.token != entry.term.as_str() {
            errors.push(M5LearnabilityLaneValidationError {
                subject_id: format!("vocabulary:{}", entry.term.as_str()),
                message: "vocabulary token diverges from frozen term token".to_string(),
            });
        }
    }
    for term in LearnabilityTerm::ALL {
        if !seen_terms.contains(&term) {
            errors.push(M5LearnabilityLaneValidationError {
                subject_id: format!("vocabulary:{}", term.as_str()),
                message: "frozen vocabulary term is missing".to_string(),
            });
        }
    }

    // ── Educational-AI boundary: explain stays separate from do. ──
    let boundary = &freeze.educational_ai_boundary;
    if !boundary.explain_and_do_separate {
        errors.push(M5LearnabilityLaneValidationError {
            subject_id: "educational_ai_boundary".to_string(),
            message: "educational AI does not separate explain from do".to_string(),
        });
    }
    if !boundary.do_requires_same_preview_approval {
        errors.push(M5LearnabilityLaneValidationError {
            subject_id: "educational_ai_boundary".to_string(),
            message: "educational AI 'do' bypasses the standard preview/approval path".to_string(),
        });
    }
    if boundary.can_mutate_live_state_directly {
        errors.push(M5LearnabilityLaneValidationError {
            subject_id: "educational_ai_boundary".to_string(),
            message: "educational AI can mutate live state directly".to_string(),
        });
    }

    // ── Lane matrix: full coverage and per-row guardrails. ──
    for family in M5LearningSurfaceFamily::ALL {
        for term in LearnabilityTerm::ALL {
            if freeze.row(family, term).is_none() {
                errors.push(M5LearnabilityLaneValidationError {
                    subject_id: format!("lane:{}:{}", family.as_str(), term.as_str()),
                    message: "claimed family is missing a lane row for this term".to_string(),
                });
            }
        }
    }

    // Cross-cutting terms must route through exactly one canonical lane.
    for term in LearnabilityTerm::ALL
        .into_iter()
        .filter(|t| t.is_cross_cutting())
    {
        let refs: BTreeSet<&str> = freeze
            .lane_rows
            .iter()
            .filter(|r| r.term == term)
            .map(|r| r.canonical_lane_ref.as_str())
            .collect();
        if refs.len() > 1 {
            errors.push(M5LearnabilityLaneValidationError {
                subject_id: format!("lane:cross_cutting:{}", term.as_str()),
                message: format!(
                    "cross-cutting term forks {} canonical lanes instead of one",
                    refs.len()
                ),
            });
        }
    }

    for row in &freeze.lane_rows {
        let (derived, _) = derive_lane_row_verdict(row);
        if derived != row.verdict {
            errors.push(M5LearnabilityLaneValidationError {
                subject_id: row.surface_token.clone(),
                message: format!(
                    "stored verdict {:?} diverges from derived {:?}",
                    row.verdict, derived
                ),
            });
        }
        if row.hidden_feature_local_coachmark {
            errors.push(M5LearnabilityLaneValidationError {
                subject_id: row.surface_token.clone(),
                message: "row depends on a hidden feature-local coachmark".to_string(),
            });
        }
        if row.private_mutation_path
            || row.mutation_path_class == MutationPathClass::HiddenDirectMutation
        {
            errors.push(M5LearnabilityLaneValidationError {
                subject_id: row.surface_token.clone(),
                message: "row depends on a private/hidden mutation path".to_string(),
            });
        }
        if !row.command_backed {
            errors.push(M5LearnabilityLaneValidationError {
                subject_id: row.surface_token.clone(),
                message: "row is not command-backed".to_string(),
            });
        }
        if row.explain_apply_class == ExplainApplyClass::Conflated {
            errors.push(M5LearnabilityLaneValidationError {
                subject_id: row.surface_token.clone(),
                message: "row conflates explain/apply".to_string(),
            });
        }
        if !row.data_ownership_class.qualifies_stable() {
            errors.push(M5LearnabilityLaneValidationError {
                subject_id: row.surface_token.clone(),
                message: "row is not user-owned / local-first".to_string(),
            });
        }
        if row.mirror_parity.silent_dead_link_on_stale {
            errors.push(M5LearnabilityLaneValidationError {
                subject_id: row.surface_token.clone(),
                message: "row shows a silent dead link when stale/offline".to_string(),
            });
        }
        if !row.support_export_parity.carries_no_credential_bodies {
            errors.push(M5LearnabilityLaneValidationError {
                subject_id: row.surface_token.clone(),
                message: "row support export carries credential bodies".to_string(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests;
