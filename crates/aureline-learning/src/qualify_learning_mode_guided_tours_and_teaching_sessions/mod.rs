//! Qualification layer for learning-mode surfaces, guided tours, exercise
//! rails, glossary packs, progress snapshots, and teaching-session flows.
//!
//! This module turns learnability surfaces into explicit product truth by
//! attaching citation proof, privacy posture, offline/cached degradation
//! state, explain-vs-apply separation, scope posture, restore proof, and
//! role/authority separation to each surface kind, then deriving a typed
//! [`QualificationVerdict`] from those fields rather than trusting input
//! claims.
//!
//! ## Surface kinds qualified
//!
//! | Surface kind | Qualified by |
//! |---|---|
//! | Glossary pack | [`GlossaryPackQualificationRecord`] |
//! | Tour package | [`TourPackageQualificationRecord`] |
//! | Guided exercise rail | [`ExerciseRailQualificationRecord`] |
//! | Learning-mode profile | [`LearningModeProfileQualificationRecord`] |
//! | Progress snapshot | [`ProgressSnapshotQualificationRecord`] |
//! | Teaching / presentation session | [`TeachingSessionQualificationRecord`] |
//!
//! A [`GuidedLearningQualificationManifest`] aggregates all surface records
//! and carries the derived overall verdict. The manifest is the canonical
//! truth source; docs/help, Start Center, support export, and release packets
//! ingest it rather than cloning status text.
//!
//! ## Narrowing invariant
//!
//! Any surface that cannot prove citation, privacy, offline, and authority
//! truth is automatically narrowed below Stable:
//! [`derive_surface_verdict`] returns [`QualificationVerdict::NarrowedPreview`]
//! or [`QualificationVerdict::NarrowedBeta`] with a named reason, and the
//! manifest overall verdict reflects the narrowest member.
//!
//! ## Canonical references
//!
//! - Schema: `schemas/learning/guided-learning-contracts.schema.json`
//! - Fixture dir: `fixtures/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions/`
//! - Artifact: `artifacts/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions.md`
//! - Doc: `docs/m4/qualify-learning-mode-guided-tours-and-teaching-sessions.md`

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// ── Schema-version and record-kind constants ─────────────────────────────────

/// Integer schema version for qualification records. Bumped only on breaking
/// payload changes; additive-optional fields do not bump this version.
pub const GUIDED_LEARNING_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`GlossaryPackQualificationRecord`].
pub const GLOSSARY_PACK_QUALIFICATION_RECORD_KIND: &str = "glossary_pack_qualification_record";

/// Record kind for [`TourPackageQualificationRecord`].
pub const TOUR_PACKAGE_QUALIFICATION_RECORD_KIND: &str = "tour_package_qualification_record";

/// Record kind for [`ExerciseRailQualificationRecord`].
pub const EXERCISE_RAIL_QUALIFICATION_RECORD_KIND: &str = "exercise_rail_qualification_record";

/// Record kind for [`LearningModeProfileQualificationRecord`].
pub const LEARNING_MODE_PROFILE_QUALIFICATION_RECORD_KIND: &str =
    "learning_mode_profile_qualification_record";

/// Record kind for [`ProgressSnapshotQualificationRecord`].
pub const PROGRESS_SNAPSHOT_QUALIFICATION_RECORD_KIND: &str =
    "progress_snapshot_qualification_record";

/// Record kind for [`TeachingSessionQualificationRecord`].
pub const TEACHING_SESSION_QUALIFICATION_RECORD_KIND: &str =
    "teaching_session_qualification_record";

/// Record kind for [`GuidedLearningQualificationManifest`].
pub const GUIDED_LEARNING_QUALIFICATION_MANIFEST_RECORD_KIND: &str =
    "guided_learning_qualification_manifest_record";

// ── Canonical path constants ──────────────────────────────────────────────────

/// Repository-relative path to the qualification schema.
pub const GUIDED_LEARNING_CONTRACTS_SCHEMA_REF: &str =
    "schemas/learning/guided-learning-contracts.schema.json";

/// Repository-relative path to the fixture directory.
pub const GUIDED_LEARNING_QUALIFICATION_FIXTURE_DIR: &str =
    "fixtures/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions";

/// Repository-relative path to the artifact doc.
pub const GUIDED_LEARNING_QUALIFICATION_ARTIFACT_REF: &str =
    "artifacts/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions.md";

/// Repository-relative path to the public doc.
pub const GUIDED_LEARNING_QUALIFICATION_DOC_REF: &str =
    "docs/m4/qualify-learning-mode-guided-tours-and-teaching-sessions.md";

/// Repository-relative path to the presentation-packet schema the manifests
/// cross-reference for evidence review.
pub const LEARNING_PRESENTATION_PACKET_SCHEMA_REF: &str =
    "schemas/learning/learning_presentation_packet.schema.json";

// ── Top-level lifecycle/verdict vocabulary ────────────────────────────────────

/// Derived lifecycle verdict for one qualified learnability surface.
///
/// The verdict is produced by [`derive_surface_verdict`] from the evidence
/// fields of a qualification record rather than from an input claim. Any
/// surface missing citation, privacy, offline, or authority proof is narrowed
/// rather than inheriting an adjacent green row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationVerdict {
    /// All required proofs present; the surface qualifies Stable.
    QualifiedStable,
    /// One or more proofs missing; narrowed to Preview with a named reason.
    NarrowedPreview,
    /// One or more proofs missing; narrowed to Beta with a named reason.
    NarrowedBeta,
    /// The surface is absent or explicitly disabled.
    Absent,
}

impl QualificationVerdict {
    /// Stable string token suitable for records, support exports, and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualifiedStable => "qualified_stable",
            Self::NarrowedPreview => "narrowed_preview",
            Self::NarrowedBeta => "narrowed_beta",
            Self::Absent => "absent",
        }
    }

    /// Returns the strictest (narrowest) verdict between `self` and `other`.
    pub fn meet(self, other: Self) -> Self {
        // Ordering: Absent > NarrowedPreview > NarrowedBeta > QualifiedStable
        use QualificationVerdict::*;
        match (self, other) {
            (Absent, _) | (_, Absent) => Absent,
            (NarrowedPreview, _) | (_, NarrowedPreview) => NarrowedPreview,
            (NarrowedBeta, _) | (_, NarrowedBeta) => NarrowedBeta,
            (QualifiedStable, QualifiedStable) => QualifiedStable,
        }
    }
}

// ── Citation proof ────────────────────────────────────────────────────────────

/// Proof that a surface cites at least one canonical anchor (command id, docs
/// node, symbol, or file object).
///
/// A surface with `has_citation: false` automatically narrows below Stable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationProof {
    /// Whether the surface cites at least one stable anchor.
    pub has_citation: bool,
    /// Opaque refs to the canonical command ids this surface cites.
    #[serde(default)]
    pub command_id_refs: Vec<String>,
    /// Opaque refs to the docs/help citation anchors this surface cites.
    #[serde(default)]
    pub docs_citation_anchor_refs: Vec<String>,
    /// Opaque refs to symbol-linked references this surface cites.
    #[serde(default)]
    pub symbol_linked_refs: Vec<String>,
    /// Whether every cited anchor resolves to the installed/live authoritative
    /// pack revision (as opposed to cached or stale).
    pub all_anchors_live_authoritative: bool,
    /// Named reason when `has_citation` is false or anchors are not live.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
}

impl CitationProof {
    /// Returns true when the proof satisfies Stable citation requirements.
    pub fn qualifies_stable(&self) -> bool {
        self.has_citation && self.all_anchors_live_authoritative
    }
}

// ── Privacy posture ───────────────────────────────────────────────────────────

/// Privacy posture for a learning surface.
///
/// Progress, dismissals, bookmarks, speaker notes, and teaching-session
/// artifacts MUST remain local/private unless explicitly promoted. A surface
/// with `progress_local_by_default: false` or `repo_visible: true` or
/// `telemetry_grade_read_access: true` automatically narrows below Stable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivacyPosture {
    /// Whether progress and dismissal state is stored locally by default.
    pub progress_local_by_default: bool,
    /// Whether the user must explicitly promote data to shared/retained status.
    pub explicit_promotion_required_for_sharing: bool,
    /// Whether the repository can observe this surface's progress state.
    pub repo_visible: bool,
    /// Whether any background service gains telemetry-grade read access.
    pub telemetry_grade_read_access: bool,
    /// Named reason when privacy invariants are not met.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
}

impl PrivacyPosture {
    /// Returns true when the posture satisfies Stable privacy requirements.
    pub fn qualifies_stable(&self) -> bool {
        self.progress_local_by_default
            && self.explicit_promotion_required_for_sharing
            && !self.repo_visible
            && !self.telemetry_grade_read_access
    }
}

// ── Offline/cached degradation posture ───────────────────────────────────────

/// How the surface degrades when docs packs, the graph, or network are
/// unavailable.
///
/// Surfaces MUST degrade to an explicit named state rather than silently
/// disappearing or misrepresenting availability. A surface with
/// `silent_disappearance_on_offline: true` automatically narrows below Stable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflinePosture {
    /// Explicit degradation state label shown when offline or pack-unavailable.
    ///
    /// One of: `no_degradation`, `cached_disclosed`, `local_only_disclosed`,
    /// `not_installed_disclosed`, `preview_degraded`, `unsupported_disclosed`.
    pub degradation_state: String,
    /// Whether the surface silently disappears instead of showing a named state.
    pub silent_disappearance_on_offline: bool,
    /// Whether a cached docs/glossary pack is accepted with freshness disclosed.
    pub cached_pack_accepted_with_disclosure: bool,
    /// Whether the surface is available in air-gapped / local-only profiles.
    pub available_in_local_only_profile: bool,
    /// Named reason when the offline posture is inadequate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
}

impl OfflinePosture {
    /// Returns true when the posture satisfies Stable offline requirements.
    pub fn qualifies_stable(&self) -> bool {
        !self.silent_disappearance_on_offline
    }
}

// ── Explain-vs-apply separation ───────────────────────────────────────────────

/// Separation class for explain vs apply verbs on a learning surface.
///
/// Educational AI, guided exercises, and tour flows MUST keep `Explain` and
/// `Apply` as separate verbs. Any teaching mutation MUST use the same command
/// id, preview sheet, approval path, rollback/checkpoint posture, and
/// sandbox/reversible labeling as ordinary product interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainApplyClass {
    /// Explain and Apply are fully separated; all mutations use the standard
    /// command/preview/approval/rollback path.
    FullySeparated,
    /// Apply actions require explicit approval through the standard preview
    /// fence before any mutation occurs.
    ApplyRequiresApproval,
    /// Surface is read-only; no Apply verb is exposed.
    ReadOnly,
    /// Explain and Apply are conflated; the surface narrows below Stable.
    Conflated,
}

impl ExplainApplyClass {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullySeparated => "fully_separated",
            Self::ApplyRequiresApproval => "apply_requires_approval",
            Self::ReadOnly => "read_only",
            Self::Conflated => "conflated",
        }
    }

    /// Returns true when the class satisfies Stable separation requirements.
    pub const fn qualifies_stable(self) -> bool {
        matches!(
            self,
            Self::FullySeparated | Self::ApplyRequiresApproval | Self::ReadOnly
        )
    }
}

// ── Scope posture ─────────────────────────────────────────────────────────────

/// Named scope for a learning surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeClass {
    /// Scoped to the local user only; no sharing by default.
    UserLocal,
    /// Opt-in per workspace in addition to per-user.
    UserOrWorkspaceOptIn,
    /// Shared to a named audience through an explicit grant.
    SharedNamedAudienceGranted,
    /// Classroom managed pool; classroom authority governs sharing.
    ClassroomManagedPool,
    /// Policy disabled; the surface is suppressed by policy.
    PolicyDisabled,
}

impl ScopeClass {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserLocal => "user_local",
            Self::UserOrWorkspaceOptIn => "user_or_workspace_opt_in",
            Self::SharedNamedAudienceGranted => "shared_named_audience_granted",
            Self::ClassroomManagedPool => "classroom_managed_pool",
            Self::PolicyDisabled => "policy_disabled",
        }
    }
}

/// Share/scope posture with follow and grant fields for teaching/presentation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopePosture {
    /// Named scope class.
    pub scope_class: ScopeClass,
    /// Whether follow mode requires an explicit grant before taking effect.
    pub follow_mode_requires_explicit_grant: bool,
    /// Whether share-to-audience requires an explicit user action.
    pub share_requires_explicit_user_action: bool,
    /// Opaque ref to the follow/presenter grant record when granted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follow_grant_ref: Option<String>,
}

// ── Restore proof ─────────────────────────────────────────────────────────────

/// Proof that exiting a teaching/presentation session restores the prior
/// workspace state (layout, panel visibility, focus, selection, accessibility
/// posture).
///
/// A teaching/presentation surface that cannot prove restore automatically
/// narrows below Stable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreProof {
    /// Whether the layout is restored on clean exit.
    pub layout_restored_on_exit: bool,
    /// Whether focus and selection state is restored.
    pub focus_selection_restored: bool,
    /// Whether panel visibility is restored.
    pub panel_visibility_restored: bool,
    /// Whether accessibility posture is restored.
    pub accessibility_posture_restored: bool,
    /// Whether the prior state is restored on crash recovery.
    pub crash_recovery_restores_prior_state: bool,
    /// Opaque ref to the restore checkpoint record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_checkpoint_ref: Option<String>,
    /// Named reason when restore invariants are not met.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
}

impl RestoreProof {
    /// Returns true when the proof satisfies Stable restore requirements.
    pub fn qualifies_stable(&self) -> bool {
        self.layout_restored_on_exit
            && self.focus_selection_restored
            && self.panel_visibility_restored
            && self.accessibility_posture_restored
            && self.crash_recovery_restores_prior_state
    }
}

// ── Role / authority separation ───────────────────────────────────────────────

/// Named role authority class for a teaching/presentation surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoleAuthorityClass {
    /// Teaching roles describe participation only; no broader authority grant.
    ParticipationOnlyNoAuthorityGrant,
    /// No teaching role in scope; authority is governed by workspace only.
    WorkspaceAuthorityOnly,
    /// Authority is overstated or wider than the underlying workspace allows.
    AuthorityOverstated,
}

impl RoleAuthorityClass {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParticipationOnlyNoAuthorityGrant => {
                "participation_only_no_authority_grant"
            }
            Self::WorkspaceAuthorityOnly => "workspace_authority_only",
            Self::AuthorityOverstated => "authority_overstated",
        }
    }

    /// Returns true when the class satisfies Stable authority requirements.
    pub const fn qualifies_stable(self) -> bool {
        matches!(
            self,
            Self::ParticipationOnlyNoAuthorityGrant | Self::WorkspaceAuthorityOnly
        )
    }
}

// ── Speaker-note locality ─────────────────────────────────────────────────────

/// Where speaker notes reside by default.
///
/// Speaker notes MUST default to local/private and MUST NOT be visible to the
/// audience under any class. A surface with
/// `default_locality != SpeakerNoteLocality::FacilitatorOnlyLocal`
/// automatically narrows below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeakerNoteLocality {
    /// Notes are facilitator-only and machine-local by default.
    FacilitatorOnlyLocal,
    /// Notes are visible to co-presenters after explicit promotion.
    CoPresentersAfterExplicitPromotion,
    /// No speaker-note adjunct; not applicable for this surface.
    NotApplicable,
    /// Notes are visible to the audience; automatically narrows below Stable.
    AudienceVisible,
}

impl SpeakerNoteLocality {
    /// Stable string token for records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacilitatorOnlyLocal => "facilitator_only_local",
            Self::CoPresentersAfterExplicitPromotion => {
                "co_presenters_after_explicit_promotion"
            }
            Self::NotApplicable => "not_applicable",
            Self::AudienceVisible => "audience_visible",
        }
    }

    /// Returns true when the locality satisfies Stable privacy requirements.
    pub const fn qualifies_stable(self) -> bool {
        matches!(
            self,
            Self::FacilitatorOnlyLocal
                | Self::CoPresentersAfterExplicitPromotion
                | Self::NotApplicable
        )
    }
}

// ── Accessibility posture ─────────────────────────────────────────────────────

/// Accessibility coverage summary for a qualified surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityPosture {
    /// Whether keyboard reachability is proved for all surface actions.
    pub keyboard_reachable: bool,
    /// Whether screen-reader narration is provided.
    pub screen_reader_narration: bool,
    /// Whether reset/skip actions are keyboard-accessible.
    pub reset_skip_keyboard_accessible: bool,
    /// Whether offline/mirrored-docs degradation is disclosed accessibly.
    pub offline_degradation_accessible: bool,
    /// Whether reduced-motion preference is honored.
    pub reduced_motion_honored: bool,
}

impl AccessibilityPosture {
    /// Returns true when every accessibility axis is covered.
    pub fn fully_covered(&self) -> bool {
        self.keyboard_reachable
            && self.screen_reader_narration
            && self.reset_skip_keyboard_accessible
            && self.offline_degradation_accessible
            && self.reduced_motion_honored
    }
}

// ── Verdict derivation ────────────────────────────────────────────────────────

/// Inputs to [`derive_surface_verdict`] gathered from a qualification record.
pub struct VerdictInputs<'a> {
    pub citation: &'a CitationProof,
    pub privacy: &'a PrivacyPosture,
    pub offline: &'a OfflinePosture,
    pub explain_apply: ExplainApplyClass,
    pub role_authority: RoleAuthorityClass,
    pub speaker_note_locality: SpeakerNoteLocality,
    /// Whether this surface requires a restore proof (teaching/presentation
    /// sessions do; glossary cards and progress snapshots do not).
    pub restore_proof: Option<&'a RestoreProof>,
}

/// Derives the [`QualificationVerdict`] from evidence fields.
///
/// The verdict is `QualifiedStable` only when every proof passes. Any failing
/// proof yields `NarrowedBeta`. The caller may promote specific narrowed cases
/// to `NarrowedPreview` (e.g., surfaces that have never shipped at all).
pub fn derive_surface_verdict(inputs: &VerdictInputs<'_>) -> (QualificationVerdict, Vec<String>) {
    let mut reasons: Vec<String> = Vec::new();

    if !inputs.citation.qualifies_stable() {
        if let Some(r) = &inputs.citation.narrowing_reason {
            reasons.push(format!("citation: {r}"));
        } else {
            reasons.push("citation_proof_incomplete".to_string());
        }
    }

    if !inputs.privacy.qualifies_stable() {
        if let Some(r) = &inputs.privacy.narrowing_reason {
            reasons.push(format!("privacy: {r}"));
        } else {
            reasons.push("privacy_posture_incomplete".to_string());
        }
    }

    if !inputs.offline.qualifies_stable() {
        if let Some(r) = &inputs.offline.narrowing_reason {
            reasons.push(format!("offline: {r}"));
        } else {
            reasons.push("offline_posture_silent_disappearance".to_string());
        }
    }

    if !inputs.explain_apply.qualifies_stable() {
        reasons.push(format!(
            "explain_apply_conflated: {}",
            inputs.explain_apply.as_str()
        ));
    }

    if !inputs.role_authority.qualifies_stable() {
        reasons.push(format!(
            "role_authority_overstated: {}",
            inputs.role_authority.as_str()
        ));
    }

    if !inputs.speaker_note_locality.qualifies_stable() {
        reasons.push(format!(
            "speaker_note_audience_visible: {}",
            inputs.speaker_note_locality.as_str()
        ));
    }

    if let Some(restore) = inputs.restore_proof {
        if !restore.qualifies_stable() {
            if let Some(r) = &restore.narrowing_reason {
                reasons.push(format!("restore_proof: {r}"));
            } else {
                reasons.push("restore_proof_incomplete".to_string());
            }
        }
    }

    let verdict = if reasons.is_empty() {
        QualificationVerdict::QualifiedStable
    } else {
        QualificationVerdict::NarrowedBeta
    };

    (verdict, reasons)
}

// ── Per-surface qualification records ────────────────────────────────────────

/// Qualification record for one glossary pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlossaryPackQualificationRecord {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this record.
    pub record_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Opaque ref to the glossary pack being qualified.
    pub pack_ref: String,
    /// Lifecycle label visible in product, docs/help, and support export.
    pub lifecycle_label: String,
    /// Citation proof.
    pub citation: CitationProof,
    /// Privacy posture.
    pub privacy: PrivacyPosture,
    /// Offline/cached degradation posture.
    pub offline: OfflinePosture,
    /// Explain-vs-apply separation class.
    pub explain_apply_class: ExplainApplyClass,
    /// Scope posture.
    pub scope: ScopePosture,
    /// Accessibility posture.
    pub accessibility: AccessibilityPosture,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
    /// Opaque fixture refs backing this record.
    #[serde(default)]
    pub evidence_fixture_refs: Vec<String>,
}

/// Qualification record for one tour package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TourPackageQualificationRecord {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this record.
    pub record_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Opaque ref to the tour package being qualified.
    pub package_ref: String,
    /// Lifecycle label visible in product, docs/help, and support export.
    pub lifecycle_label: String,
    /// Citation proof.
    pub citation: CitationProof,
    /// Privacy posture.
    pub privacy: PrivacyPosture,
    /// Offline/cached degradation posture.
    pub offline: OfflinePosture,
    /// Explain-vs-apply separation class.
    pub explain_apply_class: ExplainApplyClass,
    /// Scope posture.
    pub scope: ScopePosture,
    /// Accessibility posture.
    pub accessibility: AccessibilityPosture,
    /// Opaque refs to waypoints this package walks.
    #[serde(default)]
    pub waypoint_refs: Vec<String>,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
    /// Opaque fixture refs backing this record.
    #[serde(default)]
    pub evidence_fixture_refs: Vec<String>,
}

/// Qualification record for one guided exercise rail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseRailQualificationRecord {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this record.
    pub record_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Opaque ref to the exercise rail being qualified.
    pub rail_ref: String,
    /// Lifecycle label visible in product, docs/help, and support export.
    pub lifecycle_label: String,
    /// Citation proof.
    pub citation: CitationProof,
    /// Privacy posture.
    pub privacy: PrivacyPosture,
    /// Offline/cached degradation posture.
    pub offline: OfflinePosture,
    /// Explain-vs-apply separation class. Exercise rails MUST prove that any
    /// Apply step uses the same command/preview/approval/rollback path as
    /// ordinary product interaction.
    pub explain_apply_class: ExplainApplyClass,
    /// Scope posture.
    pub scope: ScopePosture,
    /// Accessibility posture.
    pub accessibility: AccessibilityPosture,
    /// Whether every apply step is reversible/sandboxed.
    pub apply_steps_reversible: bool,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
    /// Opaque fixture refs backing this record.
    #[serde(default)]
    pub evidence_fixture_refs: Vec<String>,
}

/// Qualification record for one learning-mode profile.
///
/// Learning mode is opt-in per user and optionally per workspace. The profile
/// MUST NOT block first edit or replacement-grade onboarding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningModeProfileQualificationRecord {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this record.
    pub record_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Opaque ref to the learning-mode profile being qualified.
    pub profile_ref: String,
    /// Lifecycle label visible in product, docs/help, and support export.
    pub lifecycle_label: String,
    /// Citation proof.
    pub citation: CitationProof,
    /// Privacy posture.
    pub privacy: PrivacyPosture,
    /// Offline/cached degradation posture.
    pub offline: OfflinePosture,
    /// Explain-vs-apply separation class.
    pub explain_apply_class: ExplainApplyClass,
    /// Scope posture.
    pub scope: ScopePosture,
    /// Accessibility posture.
    pub accessibility: AccessibilityPosture,
    /// Whether the profile is strictly opt-in (never auto-enrolled).
    pub opt_in_only: bool,
    /// Whether enabling the profile blocks first useful work.
    pub blocks_first_useful_work: bool,
    /// Whether the user can pause, snooze, reset, skip, and resume at any time.
    pub pause_snooze_reset_skip_resume_available: bool,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
    /// Opaque fixture refs backing this record.
    #[serde(default)]
    pub evidence_fixture_refs: Vec<String>,
}

/// Qualification record for one progress snapshot.
///
/// Progress snapshots are user-owned and MUST survive restart and export with
/// private-by-default state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressSnapshotQualificationRecord {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this record.
    pub record_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Opaque ref to the progress snapshot being qualified.
    pub snapshot_ref: String,
    /// Lifecycle label visible in product, docs/help, and support export.
    pub lifecycle_label: String,
    /// Privacy posture.
    pub privacy: PrivacyPosture,
    /// Offline/cached degradation posture.
    pub offline: OfflinePosture,
    /// Scope posture.
    pub scope: ScopePosture,
    /// Accessibility posture.
    pub accessibility: AccessibilityPosture,
    /// Whether the snapshot survives restart.
    pub survives_restart: bool,
    /// Whether the snapshot is safe to export in support bundles.
    pub safe_for_support_export: bool,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
    /// Opaque fixture refs backing this record.
    #[serde(default)]
    pub evidence_fixture_refs: Vec<String>,
}

/// Qualification record for one teaching or presentation session packet.
///
/// Teaching sessions are thin, reversible layers over existing learnability
/// surfaces. They MUST restore the prior layout, panel visibility, focus, and
/// selection on exit. Speaker notes MUST default to local/private.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingSessionQualificationRecord {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this record.
    pub record_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Opaque ref to the teaching/presentation session being qualified.
    pub session_ref: String,
    /// Lifecycle label visible in product, docs/help, and support export.
    pub lifecycle_label: String,
    /// Citation proof.
    pub citation: CitationProof,
    /// Privacy posture.
    pub privacy: PrivacyPosture,
    /// Offline/cached degradation posture.
    pub offline: OfflinePosture,
    /// Explain-vs-apply separation class.
    pub explain_apply_class: ExplainApplyClass,
    /// Scope posture.
    pub scope: ScopePosture,
    /// Accessibility posture.
    pub accessibility: AccessibilityPosture,
    /// Role authority class.
    pub role_authority_class: RoleAuthorityClass,
    /// Speaker-note locality class.
    pub speaker_note_locality: SpeakerNoteLocality,
    /// Restore proof.
    pub restore_proof: RestoreProof,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
    /// Opaque fixture refs backing this record.
    #[serde(default)]
    pub evidence_fixture_refs: Vec<String>,
}

// ── Manifest ──────────────────────────────────────────────────────────────────

/// Aggregated qualification manifest for all guided learning surfaces.
///
/// The manifest is the canonical truth source for learnability claims on the
/// Stable line. Docs/help, Start Center, support export, and release packets
/// ingest this record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidedLearningQualificationManifest {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this manifest.
    pub manifest_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Schema, docs, and contract refs this manifest consumes.
    pub contract_refs: BTreeMap<String, String>,
    /// Qualified glossary packs.
    pub glossary_packs: Vec<GlossaryPackQualificationRecord>,
    /// Qualified tour packages.
    pub tour_packages: Vec<TourPackageQualificationRecord>,
    /// Qualified exercise rails.
    pub exercise_rails: Vec<ExerciseRailQualificationRecord>,
    /// Qualified learning-mode profiles.
    pub learning_mode_profiles: Vec<LearningModeProfileQualificationRecord>,
    /// Qualified progress snapshots.
    pub progress_snapshots: Vec<ProgressSnapshotQualificationRecord>,
    /// Qualified teaching/presentation sessions.
    pub teaching_sessions: Vec<TeachingSessionQualificationRecord>,
    /// Overall derived verdict — the strictest (narrowest) verdict across all
    /// surface records.
    pub overall_verdict: QualificationVerdict,
    /// Named narrowing reasons aggregated from all surface records (empty when
    /// overall_verdict is QualifiedStable).
    #[serde(default)]
    pub overall_narrowing_reasons: Vec<String>,
}

impl GuidedLearningQualificationManifest {
    /// Recomputes [`overall_verdict`] and [`overall_narrowing_reasons`] from
    /// the current surface records and writes them back.
    pub fn sync_overall_verdict(&mut self) {
        let mut verdict = QualificationVerdict::QualifiedStable;
        let mut reasons: Vec<String> = Vec::new();

        for r in &self.glossary_packs {
            verdict = verdict.meet(r.verdict);
            reasons.extend(r.narrowing_reasons.iter().cloned());
        }
        for r in &self.tour_packages {
            verdict = verdict.meet(r.verdict);
            reasons.extend(r.narrowing_reasons.iter().cloned());
        }
        for r in &self.exercise_rails {
            verdict = verdict.meet(r.verdict);
            reasons.extend(r.narrowing_reasons.iter().cloned());
        }
        for r in &self.learning_mode_profiles {
            verdict = verdict.meet(r.verdict);
            reasons.extend(r.narrowing_reasons.iter().cloned());
        }
        for r in &self.progress_snapshots {
            verdict = verdict.meet(r.verdict);
            reasons.extend(r.narrowing_reasons.iter().cloned());
        }
        for r in &self.teaching_sessions {
            verdict = verdict.meet(r.verdict);
            reasons.extend(r.narrowing_reasons.iter().cloned());
        }

        reasons.sort();
        reasons.dedup();
        self.overall_verdict = verdict;
        self.overall_narrowing_reasons = reasons;
    }
}

// ── Seeded qualification corpus ───────────────────────────────────────────────

const GENERATED_AT: &str = "2026-06-03T13:00:00Z";

/// Returns the seeded qualification corpus for the four M4 learning surfaces.
///
/// The corpus covers:
/// - One glossary pack qualified Stable (full citation + live-authoritative pack).
/// - One tour package narrowed to Beta (no live-authoritative citation anchor).
/// - One exercise rail qualified Stable with explain/apply separation proved.
/// - One learning-mode profile qualified Stable (opt-in, non-blocking).
/// - One progress snapshot qualified Stable (local-only, restart-safe).
/// - One teaching session qualified Stable (speaker notes local, restore proved).
pub fn seeded_guided_learning_qualification_corpus() -> GuidedLearningQualificationManifest {
    use ExplainApplyClass::*;
    use RoleAuthorityClass::*;
    use ScopeClass::*;
    use SpeakerNoteLocality::*;

    let glossary_pack = {
        let citation = CitationProof {
            has_citation: true,
            command_id_refs: vec![
                "cmd:docs.open_in_browser".to_string(),
                "cmd:workspace.open_folder".to_string(),
            ],
            docs_citation_anchor_refs: vec![
                "docs:anchor:glossary:command_palette".to_string(),
                "docs:anchor:glossary:workspace_settings".to_string(),
            ],
            symbol_linked_refs: vec![],
            all_anchors_live_authoritative: true,
            narrowing_reason: None,
        };
        let privacy = PrivacyPosture {
            progress_local_by_default: true,
            explicit_promotion_required_for_sharing: true,
            repo_visible: false,
            telemetry_grade_read_access: false,
            narrowing_reason: None,
        };
        let offline = OfflinePosture {
            degradation_state: "cached_disclosed".to_string(),
            silent_disappearance_on_offline: false,
            cached_pack_accepted_with_disclosure: true,
            available_in_local_only_profile: true,
            narrowing_reason: None,
        };
        let scope = ScopePosture {
            scope_class: UserLocal,
            follow_mode_requires_explicit_grant: true,
            share_requires_explicit_user_action: true,
            follow_grant_ref: None,
        };
        let accessibility = AccessibilityPosture {
            keyboard_reachable: true,
            screen_reader_narration: true,
            reset_skip_keyboard_accessible: true,
            offline_degradation_accessible: true,
            reduced_motion_honored: true,
        };
        let (verdict, narrowing_reasons) = derive_surface_verdict(&VerdictInputs {
            citation: &citation,
            privacy: &privacy,
            offline: &offline,
            explain_apply: ReadOnly,
            role_authority: WorkspaceAuthorityOnly,
            speaker_note_locality: NotApplicable,
            restore_proof: None,
        });
        GlossaryPackQualificationRecord {
            record_kind: GLOSSARY_PACK_QUALIFICATION_RECORD_KIND.to_string(),
            schema_version: GUIDED_LEARNING_QUALIFICATION_SCHEMA_VERSION,
            record_id: "qualify:glossary_pack:core_terms:v1".to_string(),
            generated_at: GENERATED_AT.to_string(),
            pack_ref: "learning:glossary_pack:core_terms:beta:v1".to_string(),
            lifecycle_label: "beta".to_string(),
            citation,
            privacy,
            offline,
            explain_apply_class: ReadOnly,
            scope,
            accessibility,
            verdict,
            narrowing_reasons,
            evidence_fixture_refs: vec![
                "fixtures/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions/glossary_pack_qualified_stable.yaml".to_string(),
            ],
        }
    };

    let tour_package = {
        let citation = CitationProof {
            has_citation: true,
            command_id_refs: vec!["cmd:docs.open_in_browser".to_string()],
            docs_citation_anchor_refs: vec![
                "docs:anchor:tour:getting_started_step_1".to_string(),
            ],
            symbol_linked_refs: vec![],
            // Not live-authoritative — using cached pack revision.
            all_anchors_live_authoritative: false,
            narrowing_reason: Some(
                "tour_citation_anchors_cached_not_live_authoritative".to_string(),
            ),
        };
        let privacy = PrivacyPosture {
            progress_local_by_default: true,
            explicit_promotion_required_for_sharing: true,
            repo_visible: false,
            telemetry_grade_read_access: false,
            narrowing_reason: None,
        };
        let offline = OfflinePosture {
            degradation_state: "cached_disclosed".to_string(),
            silent_disappearance_on_offline: false,
            cached_pack_accepted_with_disclosure: true,
            available_in_local_only_profile: false,
            narrowing_reason: None,
        };
        let scope = ScopePosture {
            scope_class: UserLocal,
            follow_mode_requires_explicit_grant: true,
            share_requires_explicit_user_action: true,
            follow_grant_ref: None,
        };
        let accessibility = AccessibilityPosture {
            keyboard_reachable: true,
            screen_reader_narration: true,
            reset_skip_keyboard_accessible: true,
            offline_degradation_accessible: true,
            reduced_motion_honored: true,
        };
        let (verdict, narrowing_reasons) = derive_surface_verdict(&VerdictInputs {
            citation: &citation,
            privacy: &privacy,
            offline: &offline,
            explain_apply: ReadOnly,
            role_authority: WorkspaceAuthorityOnly,
            speaker_note_locality: NotApplicable,
            restore_proof: None,
        });
        TourPackageQualificationRecord {
            record_kind: TOUR_PACKAGE_QUALIFICATION_RECORD_KIND.to_string(),
            schema_version: GUIDED_LEARNING_QUALIFICATION_SCHEMA_VERSION,
            record_id: "qualify:tour_package:getting_started:v1".to_string(),
            generated_at: GENERATED_AT.to_string(),
            package_ref: "learning:tour_package:getting_started:beta:v1".to_string(),
            lifecycle_label: "beta".to_string(),
            citation,
            privacy,
            offline,
            explain_apply_class: ReadOnly,
            scope,
            accessibility,
            waypoint_refs: vec![
                "learning:waypoint:getting_started:step_1".to_string(),
                "learning:waypoint:getting_started:step_2".to_string(),
            ],
            verdict,
            narrowing_reasons,
            evidence_fixture_refs: vec![
                "fixtures/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions/tour_package_narrowed_beta_no_citation.yaml".to_string(),
            ],
        }
    };

    let exercise_rail = {
        let citation = CitationProof {
            has_citation: true,
            command_id_refs: vec![
                "cmd:workspace.open_folder".to_string(),
                "cmd:editor.save_file".to_string(),
            ],
            docs_citation_anchor_refs: vec![
                "docs:anchor:exercise:open_and_save_file".to_string(),
            ],
            symbol_linked_refs: vec![],
            all_anchors_live_authoritative: true,
            narrowing_reason: None,
        };
        let privacy = PrivacyPosture {
            progress_local_by_default: true,
            explicit_promotion_required_for_sharing: true,
            repo_visible: false,
            telemetry_grade_read_access: false,
            narrowing_reason: None,
        };
        let offline = OfflinePosture {
            degradation_state: "cached_disclosed".to_string(),
            silent_disappearance_on_offline: false,
            cached_pack_accepted_with_disclosure: true,
            available_in_local_only_profile: true,
            narrowing_reason: None,
        };
        let scope = ScopePosture {
            scope_class: UserOrWorkspaceOptIn,
            follow_mode_requires_explicit_grant: true,
            share_requires_explicit_user_action: true,
            follow_grant_ref: None,
        };
        let accessibility = AccessibilityPosture {
            keyboard_reachable: true,
            screen_reader_narration: true,
            reset_skip_keyboard_accessible: true,
            offline_degradation_accessible: true,
            reduced_motion_honored: true,
        };
        let (verdict, narrowing_reasons) = derive_surface_verdict(&VerdictInputs {
            citation: &citation,
            privacy: &privacy,
            offline: &offline,
            explain_apply: ApplyRequiresApproval,
            role_authority: WorkspaceAuthorityOnly,
            speaker_note_locality: NotApplicable,
            restore_proof: None,
        });
        ExerciseRailQualificationRecord {
            record_kind: EXERCISE_RAIL_QUALIFICATION_RECORD_KIND.to_string(),
            schema_version: GUIDED_LEARNING_QUALIFICATION_SCHEMA_VERSION,
            record_id: "qualify:exercise_rail:open_and_save:v1".to_string(),
            generated_at: GENERATED_AT.to_string(),
            rail_ref: "learning:exercise_rail:open_and_save:beta:v1".to_string(),
            lifecycle_label: "beta".to_string(),
            citation,
            privacy,
            offline,
            explain_apply_class: ApplyRequiresApproval,
            scope,
            accessibility,
            apply_steps_reversible: true,
            verdict,
            narrowing_reasons,
            evidence_fixture_refs: vec![
                "fixtures/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions/exercise_rail_explain_apply_separation.yaml".to_string(),
            ],
        }
    };

    let learning_mode_profile = {
        let citation = CitationProof {
            has_citation: true,
            command_id_refs: vec!["cmd:learning.enable_mode".to_string()],
            docs_citation_anchor_refs: vec!["docs:anchor:help:learning_mode".to_string()],
            symbol_linked_refs: vec![],
            all_anchors_live_authoritative: true,
            narrowing_reason: None,
        };
        let privacy = PrivacyPosture {
            progress_local_by_default: true,
            explicit_promotion_required_for_sharing: true,
            repo_visible: false,
            telemetry_grade_read_access: false,
            narrowing_reason: None,
        };
        let offline = OfflinePosture {
            degradation_state: "cached_disclosed".to_string(),
            silent_disappearance_on_offline: false,
            cached_pack_accepted_with_disclosure: true,
            available_in_local_only_profile: true,
            narrowing_reason: None,
        };
        let scope = ScopePosture {
            scope_class: UserOrWorkspaceOptIn,
            follow_mode_requires_explicit_grant: true,
            share_requires_explicit_user_action: true,
            follow_grant_ref: None,
        };
        let accessibility = AccessibilityPosture {
            keyboard_reachable: true,
            screen_reader_narration: true,
            reset_skip_keyboard_accessible: true,
            offline_degradation_accessible: true,
            reduced_motion_honored: true,
        };
        let (verdict, narrowing_reasons) = derive_surface_verdict(&VerdictInputs {
            citation: &citation,
            privacy: &privacy,
            offline: &offline,
            explain_apply: ReadOnly,
            role_authority: WorkspaceAuthorityOnly,
            speaker_note_locality: NotApplicable,
            restore_proof: None,
        });
        LearningModeProfileQualificationRecord {
            record_kind: LEARNING_MODE_PROFILE_QUALIFICATION_RECORD_KIND.to_string(),
            schema_version: GUIDED_LEARNING_QUALIFICATION_SCHEMA_VERSION,
            record_id: "qualify:learning_mode_profile:default_individual:v1".to_string(),
            generated_at: GENERATED_AT.to_string(),
            profile_ref: "learning:profile:default_individual_learner:v1".to_string(),
            lifecycle_label: "beta".to_string(),
            citation,
            privacy,
            offline,
            explain_apply_class: ReadOnly,
            scope,
            accessibility,
            opt_in_only: true,
            blocks_first_useful_work: false,
            pause_snooze_reset_skip_resume_available: true,
            verdict,
            narrowing_reasons,
            evidence_fixture_refs: vec![
                "fixtures/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions/learning_mode_profile_opt_in.yaml".to_string(),
            ],
        }
    };

    let progress_snapshot = {
        let privacy = PrivacyPosture {
            progress_local_by_default: true,
            explicit_promotion_required_for_sharing: true,
            repo_visible: false,
            telemetry_grade_read_access: false,
            narrowing_reason: None,
        };
        let offline = OfflinePosture {
            degradation_state: "local_only_disclosed".to_string(),
            silent_disappearance_on_offline: false,
            cached_pack_accepted_with_disclosure: false,
            available_in_local_only_profile: true,
            narrowing_reason: None,
        };
        let scope = ScopePosture {
            scope_class: UserLocal,
            follow_mode_requires_explicit_grant: true,
            share_requires_explicit_user_action: true,
            follow_grant_ref: None,
        };
        let accessibility = AccessibilityPosture {
            keyboard_reachable: true,
            screen_reader_narration: false,
            reset_skip_keyboard_accessible: true,
            offline_degradation_accessible: true,
            reduced_motion_honored: true,
        };
        let (verdict, narrowing_reasons) = derive_surface_verdict(&VerdictInputs {
            citation: &CitationProof {
                has_citation: true,
                command_id_refs: vec![],
                docs_citation_anchor_refs: vec![],
                symbol_linked_refs: vec![],
                all_anchors_live_authoritative: true,
                narrowing_reason: None,
            },
            privacy: &privacy,
            offline: &offline,
            explain_apply: ReadOnly,
            role_authority: WorkspaceAuthorityOnly,
            speaker_note_locality: NotApplicable,
            restore_proof: None,
        });
        ProgressSnapshotQualificationRecord {
            record_kind: PROGRESS_SNAPSHOT_QUALIFICATION_RECORD_KIND.to_string(),
            schema_version: GUIDED_LEARNING_QUALIFICATION_SCHEMA_VERSION,
            record_id: "qualify:progress_snapshot:individual_learner:v1".to_string(),
            generated_at: GENERATED_AT.to_string(),
            snapshot_ref: "learning:progress:individual:beta:v1".to_string(),
            lifecycle_label: "beta".to_string(),
            privacy,
            offline,
            scope,
            accessibility,
            survives_restart: true,
            safe_for_support_export: true,
            verdict,
            narrowing_reasons,
            evidence_fixture_refs: vec![
                "fixtures/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions/progress_snapshot_local_by_default.yaml".to_string(),
            ],
        }
    };

    let teaching_session = {
        let citation = CitationProof {
            has_citation: true,
            command_id_refs: vec![
                "cmd:docs.open_in_browser".to_string(),
                "cmd:workspace.open_folder".to_string(),
            ],
            docs_citation_anchor_refs: vec![
                "docs:anchor:teaching:session_waypoints".to_string(),
            ],
            symbol_linked_refs: vec![],
            all_anchors_live_authoritative: true,
            narrowing_reason: None,
        };
        let privacy = PrivacyPosture {
            progress_local_by_default: true,
            explicit_promotion_required_for_sharing: true,
            repo_visible: false,
            telemetry_grade_read_access: false,
            narrowing_reason: None,
        };
        let offline = OfflinePosture {
            degradation_state: "cached_disclosed".to_string(),
            silent_disappearance_on_offline: false,
            cached_pack_accepted_with_disclosure: true,
            available_in_local_only_profile: true,
            narrowing_reason: None,
        };
        let scope = ScopePosture {
            scope_class: UserLocal,
            follow_mode_requires_explicit_grant: true,
            share_requires_explicit_user_action: true,
            follow_grant_ref: None,
        };
        let accessibility = AccessibilityPosture {
            keyboard_reachable: true,
            screen_reader_narration: true,
            reset_skip_keyboard_accessible: true,
            offline_degradation_accessible: true,
            reduced_motion_honored: true,
        };
        let restore = RestoreProof {
            layout_restored_on_exit: true,
            focus_selection_restored: true,
            panel_visibility_restored: true,
            accessibility_posture_restored: true,
            crash_recovery_restores_prior_state: true,
            restore_checkpoint_ref: Some(
                "teaching:restore_checkpoint:session_entry:v1".to_string(),
            ),
            narrowing_reason: None,
        };
        let (verdict, narrowing_reasons) = derive_surface_verdict(&VerdictInputs {
            citation: &citation,
            privacy: &privacy,
            offline: &offline,
            explain_apply: ApplyRequiresApproval,
            role_authority: ParticipationOnlyNoAuthorityGrant,
            speaker_note_locality: FacilitatorOnlyLocal,
            restore_proof: Some(&restore),
        });
        TeachingSessionQualificationRecord {
            record_kind: TEACHING_SESSION_QUALIFICATION_RECORD_KIND.to_string(),
            schema_version: GUIDED_LEARNING_QUALIFICATION_SCHEMA_VERSION,
            record_id: "qualify:teaching_session:standard:v1".to_string(),
            generated_at: GENERATED_AT.to_string(),
            session_ref: "teaching:session:standard:beta:v1".to_string(),
            lifecycle_label: "beta".to_string(),
            citation,
            privacy,
            offline,
            explain_apply_class: ApplyRequiresApproval,
            scope,
            accessibility,
            role_authority_class: ParticipationOnlyNoAuthorityGrant,
            speaker_note_locality: FacilitatorOnlyLocal,
            restore_proof: restore,
            verdict,
            narrowing_reasons,
            evidence_fixture_refs: vec![
                "fixtures/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions/teaching_session_speaker_note_local.yaml".to_string(),
                "fixtures/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions/teaching_session_restore_proof.yaml".to_string(),
            ],
        }
    };

    let mut contract_refs = BTreeMap::new();
    contract_refs.insert(
        "guided_learning_contracts_schema".to_string(),
        GUIDED_LEARNING_CONTRACTS_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "learning_presentation_packet_schema".to_string(),
        LEARNING_PRESENTATION_PACKET_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "guided_surface_state_schema".to_string(),
        "schemas/ux/guided_surface_state.schema.json".to_string(),
    );
    contract_refs.insert(
        "guided_tour_schema".to_string(),
        "schemas/ux/guided_tour.schema.json".to_string(),
    );
    contract_refs.insert(
        "learning_mode_profile_schema".to_string(),
        "schemas/ux/learning_mode_profile.schema.json".to_string(),
    );
    contract_refs.insert(
        "presentation_mode_state_schema".to_string(),
        "schemas/ux/presentation_mode_state.schema.json".to_string(),
    );
    contract_refs.insert(
        "artifact_doc".to_string(),
        GUIDED_LEARNING_QUALIFICATION_ARTIFACT_REF.to_string(),
    );
    contract_refs.insert(
        "public_doc".to_string(),
        GUIDED_LEARNING_QUALIFICATION_DOC_REF.to_string(),
    );

    let glossary_packs = vec![glossary_pack];
    let tour_packages = vec![tour_package];
    let exercise_rails = vec![exercise_rail];
    let learning_mode_profiles = vec![learning_mode_profile];
    let progress_snapshots = vec![progress_snapshot];
    let teaching_sessions = vec![teaching_session];

    let mut manifest = GuidedLearningQualificationManifest {
        record_kind: GUIDED_LEARNING_QUALIFICATION_MANIFEST_RECORD_KIND.to_string(),
        schema_version: GUIDED_LEARNING_QUALIFICATION_SCHEMA_VERSION,
        manifest_id: "guided-learning:qualification:manifest:2026.06.03-01".to_string(),
        generated_at: GENERATED_AT.to_string(),
        contract_refs,
        glossary_packs,
        tour_packages,
        exercise_rails,
        learning_mode_profiles,
        progress_snapshots,
        teaching_sessions,
        overall_verdict: QualificationVerdict::QualifiedStable,
        overall_narrowing_reasons: vec![],
    };
    manifest.sync_overall_verdict();
    manifest
}

// ── Validation ────────────────────────────────────────────────────────────────

/// A typed validation error from [`validate_guided_learning_qualification`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualificationValidationError {
    /// Opaque id of the record that failed.
    pub record_id: String,
    /// Human-readable description of the failure.
    pub message: String,
}

impl std::fmt::Display for QualificationValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.record_id, self.message)
    }
}

/// Validates a [`GuidedLearningQualificationManifest`] and returns any
/// violations as typed errors.
///
/// # Errors
///
/// Returns a non-empty `Vec` when any surface record's stored verdict diverges
/// from the verdict derived from its evidence fields, or when any record
/// violates a hard invariant (e.g., a learning-mode profile that blocks first
/// useful work, a teaching session with audience-visible speaker notes, or a
/// surface that silently disappears offline).
pub fn validate_guided_learning_qualification(
    manifest: &GuidedLearningQualificationManifest,
) -> Result<(), Vec<QualificationValidationError>> {
    let mut errors: Vec<QualificationValidationError> = Vec::new();

    for r in &manifest.glossary_packs {
        let (derived, _) = derive_surface_verdict(&VerdictInputs {
            citation: &r.citation,
            privacy: &r.privacy,
            offline: &r.offline,
            explain_apply: r.explain_apply_class,
            role_authority: RoleAuthorityClass::WorkspaceAuthorityOnly,
            speaker_note_locality: SpeakerNoteLocality::NotApplicable,
            restore_proof: None,
        });
        if derived != r.verdict {
            errors.push(QualificationValidationError {
                record_id: r.record_id.clone(),
                message: format!(
                    "stored verdict {:?} diverges from derived {:?}",
                    r.verdict, derived
                ),
            });
        }
        if r.offline.silent_disappearance_on_offline {
            errors.push(QualificationValidationError {
                record_id: r.record_id.clone(),
                message: "glossary pack silently disappears offline".to_string(),
            });
        }
    }

    for r in &manifest.tour_packages {
        let (derived, _) = derive_surface_verdict(&VerdictInputs {
            citation: &r.citation,
            privacy: &r.privacy,
            offline: &r.offline,
            explain_apply: r.explain_apply_class,
            role_authority: RoleAuthorityClass::WorkspaceAuthorityOnly,
            speaker_note_locality: SpeakerNoteLocality::NotApplicable,
            restore_proof: None,
        });
        if derived != r.verdict {
            errors.push(QualificationValidationError {
                record_id: r.record_id.clone(),
                message: format!(
                    "stored verdict {:?} diverges from derived {:?}",
                    r.verdict, derived
                ),
            });
        }
    }

    for r in &manifest.exercise_rails {
        let (derived, _) = derive_surface_verdict(&VerdictInputs {
            citation: &r.citation,
            privacy: &r.privacy,
            offline: &r.offline,
            explain_apply: r.explain_apply_class,
            role_authority: RoleAuthorityClass::WorkspaceAuthorityOnly,
            speaker_note_locality: SpeakerNoteLocality::NotApplicable,
            restore_proof: None,
        });
        if derived != r.verdict {
            errors.push(QualificationValidationError {
                record_id: r.record_id.clone(),
                message: format!(
                    "stored verdict {:?} diverges from derived {:?}",
                    r.verdict, derived
                ),
            });
        }
        if r.explain_apply_class == ExplainApplyClass::Conflated {
            errors.push(QualificationValidationError {
                record_id: r.record_id.clone(),
                message: "exercise rail has conflated explain/apply".to_string(),
            });
        }
    }

    for r in &manifest.learning_mode_profiles {
        if r.blocks_first_useful_work {
            errors.push(QualificationValidationError {
                record_id: r.record_id.clone(),
                message: "learning-mode profile blocks first useful work".to_string(),
            });
        }
        if !r.opt_in_only {
            errors.push(QualificationValidationError {
                record_id: r.record_id.clone(),
                message: "learning-mode profile is not opt-in only".to_string(),
            });
        }
    }

    for r in &manifest.teaching_sessions {
        if r.speaker_note_locality == SpeakerNoteLocality::AudienceVisible {
            errors.push(QualificationValidationError {
                record_id: r.record_id.clone(),
                message: "teaching session speaker notes are audience-visible".to_string(),
            });
        }
        if r.role_authority_class == RoleAuthorityClass::AuthorityOverstated {
            errors.push(QualificationValidationError {
                record_id: r.record_id.clone(),
                message: "teaching session role authority is overstated".to_string(),
            });
        }
        let (derived, _) = derive_surface_verdict(&VerdictInputs {
            citation: &r.citation,
            privacy: &r.privacy,
            offline: &r.offline,
            explain_apply: r.explain_apply_class,
            role_authority: r.role_authority_class,
            speaker_note_locality: r.speaker_note_locality,
            restore_proof: Some(&r.restore_proof),
        });
        if derived != r.verdict {
            errors.push(QualificationValidationError {
                record_id: r.record_id.clone(),
                message: format!(
                    "stored verdict {:?} diverges from derived {:?}",
                    r.verdict, derived
                ),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_corpus_validates() {
        let corpus = seeded_guided_learning_qualification_corpus();
        validate_guided_learning_qualification(&corpus)
            .expect("seeded corpus must pass validation");
    }

    #[test]
    fn overall_verdict_reflects_narrowed_member() {
        let corpus = seeded_guided_learning_qualification_corpus();
        // The tour package is NarrowedBeta (cached citation anchor); the
        // overall verdict must be at least NarrowedBeta.
        assert_eq!(corpus.overall_verdict, QualificationVerdict::NarrowedBeta);
        assert!(!corpus.overall_narrowing_reasons.is_empty());
    }

    #[test]
    fn glossary_pack_qualifies_stable() {
        let corpus = seeded_guided_learning_qualification_corpus();
        let pack = &corpus.glossary_packs[0];
        assert_eq!(pack.verdict, QualificationVerdict::QualifiedStable);
        assert!(pack.narrowing_reasons.is_empty());
    }

    #[test]
    fn tour_package_narrowed_beta_for_cached_citation() {
        let corpus = seeded_guided_learning_qualification_corpus();
        let tour = &corpus.tour_packages[0];
        assert_eq!(tour.verdict, QualificationVerdict::NarrowedBeta);
        assert!(tour
            .narrowing_reasons
            .iter()
            .any(|r| r.contains("cached")));
    }

    #[test]
    fn exercise_rail_explain_apply_separated() {
        let corpus = seeded_guided_learning_qualification_corpus();
        let rail = &corpus.exercise_rails[0];
        assert_eq!(rail.explain_apply_class, ExplainApplyClass::ApplyRequiresApproval);
        assert!(rail.apply_steps_reversible);
        assert_eq!(rail.verdict, QualificationVerdict::QualifiedStable);
    }

    #[test]
    fn learning_mode_profile_opt_in_non_blocking() {
        let corpus = seeded_guided_learning_qualification_corpus();
        let profile = &corpus.learning_mode_profiles[0];
        assert!(profile.opt_in_only);
        assert!(!profile.blocks_first_useful_work);
        assert!(profile.pause_snooze_reset_skip_resume_available);
    }

    #[test]
    fn progress_snapshot_local_by_default() {
        let corpus = seeded_guided_learning_qualification_corpus();
        let snap = &corpus.progress_snapshots[0];
        assert!(snap.privacy.progress_local_by_default);
        assert!(!snap.privacy.repo_visible);
        assert!(!snap.privacy.telemetry_grade_read_access);
        assert!(snap.survives_restart);
    }

    #[test]
    fn teaching_session_speaker_note_local_and_restore_proved() {
        let corpus = seeded_guided_learning_qualification_corpus();
        let session = &corpus.teaching_sessions[0];
        assert_eq!(
            session.speaker_note_locality,
            SpeakerNoteLocality::FacilitatorOnlyLocal
        );
        assert!(session.restore_proof.layout_restored_on_exit);
        assert!(session.restore_proof.crash_recovery_restores_prior_state);
    }

    #[test]
    fn verdict_meet_is_commutative_and_narrowing() {
        use QualificationVerdict::*;
        assert_eq!(QualifiedStable.meet(NarrowedBeta), NarrowedBeta);
        assert_eq!(NarrowedBeta.meet(QualifiedStable), NarrowedBeta);
        assert_eq!(NarrowedPreview.meet(NarrowedBeta), NarrowedPreview);
        assert_eq!(Absent.meet(QualifiedStable), Absent);
    }

    #[test]
    fn validation_catches_audience_visible_speaker_notes() {
        let mut corpus = seeded_guided_learning_qualification_corpus();
        corpus.teaching_sessions[0].speaker_note_locality = SpeakerNoteLocality::AudienceVisible;
        let result = validate_guided_learning_qualification(&corpus);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.message.contains("speaker notes")));
    }

    #[test]
    fn validation_catches_profile_blocking_first_work() {
        let mut corpus = seeded_guided_learning_qualification_corpus();
        corpus.learning_mode_profiles[0].blocks_first_useful_work = true;
        let result = validate_guided_learning_qualification(&corpus);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.message.contains("blocks first")));
    }

    #[test]
    fn corpus_serializes_and_roundtrips() {
        let corpus = seeded_guided_learning_qualification_corpus();
        let json = serde_json::to_string_pretty(&corpus).expect("serialize");
        let back: GuidedLearningQualificationManifest =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(corpus, back);
    }
}
