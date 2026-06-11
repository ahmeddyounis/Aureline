//! Guided tours, glossary packs, contextual help cards, and command-backed
//! learning rails for the M5 depth feature families, with offline and
//! mirror-aware parity.
//!
//! This module extends the stable guided-learning objects
//! ([`GlossaryPackQualificationRecord`], [`TourPackageQualificationRecord`],
//! [`ExerciseRailQualificationRecord`], [`ProgressSnapshotQualificationRecord`])
//! into the new M5 feature families — notebooks, request and database
//! workspaces, profiler/trace flows, docs/browser depth, preview surfaces,
//! template/scaffold planners, companion/incident flows, and sync/offboarding —
//! rather than inventing hidden feature-local coachmarks. Each family that
//! claims a depth surface binds one [`M5FamilyLearningBundle`] that carries its
//! glossary pack, guided tour, contextual help cards, exercise rail, progress
//! snapshot, and an explicit [`MirrorParityPosture`].
//!
//! ## Invariants enforced
//!
//! - **Command-backed, not tutorial-only.** Every learning step reuses the same
//!   command ids, preview sheets, and approval paths as ordinary work. A bundle
//!   whose learning path is not command-backed — or whose only path is AI chat
//!   or browser handoff — narrows below Stable.
//! - **Offline and mirror parity.** Glossary and guided-help state must surface
//!   on local-only, air-gapped, and mirrored profiles with an explicit freshness
//!   label instead of silently degrading to dead links.
//! - **User-owned, private progress.** Progress, dismissal, and resume state is
//!   local-by-default, exportable, and privacy-safe on the same terms as the
//!   existing learning-mode assets.
//! - **No bypass of preview/approval.** Learning rails teach real product
//!   behavior; any Apply step rides the standard command/preview/approval path.
//!
//! ## Canonical truth source
//!
//! [`seeded_m5_feature_family_learning_manifest`] produces the canonical
//! manifest. Docs/help, Start Center, support export, and release packets ingest
//! it rather than cloning status text.
//!
//! - Schema: `schemas/learning/m5-feature-family-learning-rails.schema.json`
//! - Fixture: `fixtures/ux/m5/guided-tours/m5_feature_family_learning_manifest.json`
//! - Artifact: `artifacts/ux/m5/learning-packets/implement-m5-feature-family-guided-tours-and-learning-rails.md`
//! - Doc: `docs/help/m5/m5-feature-family-learning-rails.md`

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::qualify_learning_mode_guided_tours_and_teaching_sessions::{
    derive_surface_verdict, AccessibilityPosture, CitationProof, ExerciseRailQualificationRecord,
    ExplainApplyClass, GlossaryPackQualificationRecord, OfflinePosture, PrivacyPosture,
    ProgressSnapshotQualificationRecord, QualificationVerdict, RoleAuthorityClass, ScopeClass,
    ScopePosture, SpeakerNoteLocality, TourPackageQualificationRecord, VerdictInputs,
    EXERCISE_RAIL_QUALIFICATION_RECORD_KIND, GLOSSARY_PACK_QUALIFICATION_RECORD_KIND,
    GUIDED_LEARNING_CONTRACTS_SCHEMA_REF, GUIDED_LEARNING_QUALIFICATION_SCHEMA_VERSION,
    PROGRESS_SNAPSHOT_QUALIFICATION_RECORD_KIND, TOUR_PACKAGE_QUALIFICATION_RECORD_KIND,
};

// ── Schema-version and record-kind constants ─────────────────────────────────

/// Integer schema version for the M5 feature-family learning records. Bumped
/// only on breaking payload changes.
pub const M5_FEATURE_FAMILY_LEARNING_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`ContextualHelpCardRecord`].
pub const CONTEXTUAL_HELP_CARD_RECORD_KIND: &str = "contextual_help_card_record";

/// Record kind for [`M5FamilyLearningBundle`].
pub const M5_FAMILY_LEARNING_BUNDLE_RECORD_KIND: &str = "m5_family_learning_bundle_record";

/// Record kind for [`M5FeatureFamilyLearningManifest`].
pub const M5_FEATURE_FAMILY_LEARNING_MANIFEST_RECORD_KIND: &str =
    "m5_feature_family_learning_manifest_record";

// ── Canonical path constants ──────────────────────────────────────────────────

/// Repository-relative path to the M5 learning-rails schema.
pub const M5_FEATURE_FAMILY_LEARNING_SCHEMA_REF: &str =
    "schemas/learning/m5-feature-family-learning-rails.schema.json";

/// Repository-relative path to the canonical manifest fixture.
pub const M5_FEATURE_FAMILY_LEARNING_FIXTURE_REF: &str =
    "fixtures/ux/m5/guided-tours/m5_feature_family_learning_manifest.json";

/// Repository-relative path to the artifact doc.
pub const M5_FEATURE_FAMILY_LEARNING_ARTIFACT_REF: &str =
    "artifacts/ux/m5/learning-packets/implement-m5-feature-family-guided-tours-and-learning-rails.md";

/// Repository-relative path to the public doc.
pub const M5_FEATURE_FAMILY_LEARNING_DOC_REF: &str =
    "docs/help/m5/m5-feature-family-learning-rails.md";

// ── M5 learning surface families ──────────────────────────────────────────────

/// One M5 depth feature family that exposes learnability assets.
///
/// Each variant names a marketed M5 depth surface. The exit gate requires that
/// users can learn and revisit every claimed surface inside Aureline without
/// vendor-tab archaeology or mandatory setup videos.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningSurfaceFamily {
    /// Notebook authoring, execution, and review surface.
    Notebook,
    /// HTTP/API request workspace.
    RequestWorkspace,
    /// Database/SQL statement-safety and result-grid workspace.
    DatabaseWorkspace,
    /// Profiler and trace inspection flows.
    ProfilerTrace,
    /// In-product docs/knowledge and embedded browser depth.
    DocsBrowser,
    /// Preview/runtime surface for generated and live outputs.
    Preview,
    /// Template and scaffold planner flows.
    TemplateScaffold,
    /// Companion and incident response surfaces.
    Companion,
    /// Sync, retention, and offboarding flows.
    SyncOffboarding,
}

impl M5LearningSurfaceFamily {
    /// Stable string token for records, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::RequestWorkspace => "request_workspace",
            Self::DatabaseWorkspace => "database_workspace",
            Self::ProfilerTrace => "profiler_trace",
            Self::DocsBrowser => "docs_browser",
            Self::Preview => "preview",
            Self::TemplateScaffold => "template_scaffold",
            Self::Companion => "companion",
            Self::SyncOffboarding => "sync_offboarding",
        }
    }

    /// The full set of M5 learning surface families, in canonical order.
    pub const ALL: [M5LearningSurfaceFamily; 9] = [
        Self::Notebook,
        Self::RequestWorkspace,
        Self::DatabaseWorkspace,
        Self::ProfilerTrace,
        Self::DocsBrowser,
        Self::Preview,
        Self::TemplateScaffold,
        Self::Companion,
        Self::SyncOffboarding,
    ];
}

// ── Mirror / offline parity posture ───────────────────────────────────────────

/// Offline and mirror-aware availability posture for a learning bundle.
///
/// Local-only, air-gapped, and mirrored profiles must still surface glossary and
/// guided-help state with an explicit freshness label rather than silently
/// degrading to dead links. A posture that drops offline, drops on a mirror, or
/// shows a dead link on stale content automatically narrows below Stable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorParityPosture {
    /// Whether the bundle's glossary and help state is available offline.
    pub available_offline: bool,
    /// Whether the bundle is available on a mirrored (non-origin) profile.
    pub available_on_mirror: bool,
    /// Explicit freshness label surfaced to the user.
    ///
    /// One of: `live_authoritative`, `cached_disclosed`, `mirror_synced_disclosed`,
    /// `local_only_disclosed`, `stale_disclosed`.
    pub freshness_label: String,
    /// Whether freshness is disclosed in-product (never silently implied).
    pub explicit_freshness_disclosed: bool,
    /// Whether the bundle shows a silent dead link when content is stale or the
    /// origin is unreachable. MUST be false.
    pub silent_dead_link_on_stale: bool,
    /// Named reason when the parity posture is inadequate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
}

impl MirrorParityPosture {
    /// Returns true when the posture satisfies Stable offline/mirror parity.
    pub fn qualifies_stable(&self) -> bool {
        self.available_offline
            && self.available_on_mirror
            && self.explicit_freshness_disclosed
            && !self.silent_dead_link_on_stale
    }
}

// ── Contextual help card ──────────────────────────────────────────────────────

/// Qualification record for one contextual help card bound to an M5 surface.
///
/// Help cards explain a surface in place and link back to authoritative commands
/// and docs. They are read-only or explain-first; any Apply affordance reuses the
/// standard command/preview/approval path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextualHelpCardRecord {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Opaque stable id for this record.
    pub record_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Feature family this card serves.
    pub family: M5LearningSurfaceFamily,
    /// Opaque ref to the help card being qualified.
    pub card_ref: String,
    /// Lifecycle label visible in product, docs/help, and support export.
    pub lifecycle_label: String,
    /// Citation proof — the card cites at least one command or docs anchor.
    pub citation: CitationProof,
    /// Offline/cached degradation posture.
    pub offline: OfflinePosture,
    /// Explain-vs-apply separation class.
    pub explain_apply_class: ExplainApplyClass,
    /// Accessibility posture.
    pub accessibility: AccessibilityPosture,
    /// Whether every actionable affordance on the card is command-backed (reuses
    /// a real command id rather than a tutorial-only shortcut).
    pub command_backed: bool,
    /// Derived verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
    /// Opaque fixture refs backing this record.
    #[serde(default)]
    pub evidence_fixture_refs: Vec<String>,
}

// ── Per-family learning bundle ────────────────────────────────────────────────

/// All learning assets bound to one M5 feature family.
///
/// A family that claims a depth surface (`claimed: true`) must expose
/// command-backed glossary, guided-tour, and contextual-help assets that remain
/// usable offline and on mirrored profiles. The bundle verdict is the strictest
/// (narrowest) verdict across its member assets, folded with the mirror-parity
/// and command-backed guardrails.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FamilyLearningBundle {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Feature family this bundle serves.
    pub family: M5LearningSurfaceFamily,
    /// Whether the feature family is claimed (a marketed depth surface). An
    /// unclaimed family is `Absent` rather than narrowed.
    pub claimed: bool,
    /// Whether the family exposes an in-product, command-backed learning path
    /// (not AI-chat- or browser-handoff-only).
    pub in_product_command_backed_path: bool,
    /// Glossary pack qualified for this family.
    pub glossary_pack: GlossaryPackQualificationRecord,
    /// Guided tour package qualified for this family.
    pub tour_package: TourPackageQualificationRecord,
    /// Contextual help cards qualified for this family.
    #[serde(default)]
    pub contextual_help_cards: Vec<ContextualHelpCardRecord>,
    /// Optional guided exercise rail (present when the family teaches an Apply
    /// flow).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exercise_rail: Option<ExerciseRailQualificationRecord>,
    /// Progress snapshot for this family's learning state.
    pub progress_snapshot: ProgressSnapshotQualificationRecord,
    /// Offline/mirror parity posture for the bundle.
    pub mirror_parity: MirrorParityPosture,
    /// Derived bundle verdict.
    pub verdict: QualificationVerdict,
    /// Named narrowing reasons (empty when verdict is QualifiedStable).
    #[serde(default)]
    pub narrowing_reasons: Vec<String>,
}

impl M5FamilyLearningBundle {
    /// Recomputes [`verdict`](Self::verdict) and
    /// [`narrowing_reasons`](Self::narrowing_reasons) from the member assets and
    /// guardrails, writing them back.
    pub fn sync_verdict(&mut self) {
        let (verdict, reasons) = derive_bundle_verdict(self);
        self.verdict = verdict;
        self.narrowing_reasons = reasons;
    }
}

/// Derives the bundle verdict and narrowing reasons from member assets and the
/// mirror-parity / command-backed guardrails.
///
/// An unclaimed family is [`QualificationVerdict::Absent`]. A claimed family
/// meets the verdicts of its glossary pack, tour package, help cards, exercise
/// rail, and progress snapshot, then narrows further if the mirror-parity posture
/// fails or no in-product command-backed path exists.
pub fn derive_bundle_verdict(
    bundle: &M5FamilyLearningBundle,
) -> (QualificationVerdict, Vec<String>) {
    if !bundle.claimed {
        return (QualificationVerdict::Absent, Vec::new());
    }

    let family = bundle.family.as_str();
    let mut verdict = QualificationVerdict::QualifiedStable;
    let mut reasons: Vec<String> = Vec::new();

    verdict = verdict.meet(bundle.glossary_pack.verdict);
    reasons.extend(bundle.glossary_pack.narrowing_reasons.iter().cloned());

    verdict = verdict.meet(bundle.tour_package.verdict);
    reasons.extend(bundle.tour_package.narrowing_reasons.iter().cloned());

    for card in &bundle.contextual_help_cards {
        verdict = verdict.meet(card.verdict);
        reasons.extend(card.narrowing_reasons.iter().cloned());
    }

    if let Some(rail) = &bundle.exercise_rail {
        verdict = verdict.meet(rail.verdict);
        reasons.extend(rail.narrowing_reasons.iter().cloned());
    }

    verdict = verdict.meet(bundle.progress_snapshot.verdict);
    reasons.extend(bundle.progress_snapshot.narrowing_reasons.iter().cloned());

    if !bundle.mirror_parity.qualifies_stable() {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        if let Some(r) = &bundle.mirror_parity.narrowing_reason {
            reasons.push(format!("{family}: mirror_parity: {r}"));
        } else {
            reasons.push(format!("{family}: mirror_parity_inadequate"));
        }
    }

    if !bundle.in_product_command_backed_path {
        verdict = verdict.meet(QualificationVerdict::NarrowedBeta);
        reasons.push(format!(
            "{family}: no_in_product_command_backed_learning_path"
        ));
    }

    reasons.sort();
    reasons.dedup();
    (verdict, reasons)
}

// ── Manifest ──────────────────────────────────────────────────────────────────

/// Aggregated learning manifest for all M5 feature families.
///
/// This manifest is the canonical truth source for M5 learnability claims. It
/// records one bundle per claimed depth family and carries the strictest
/// (narrowest) overall verdict, so a single under-qualified family narrows the
/// published M5 learnability posture rather than inheriting an adjacent green
/// row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FeatureFamilyLearningManifest {
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
    /// Per-family learning bundles.
    pub family_bundles: Vec<M5FamilyLearningBundle>,
    /// Overall derived verdict — the strictest verdict across claimed families.
    pub overall_verdict: QualificationVerdict,
    /// Named narrowing reasons aggregated from claimed families (empty when
    /// overall_verdict is QualifiedStable).
    #[serde(default)]
    pub overall_narrowing_reasons: Vec<String>,
}

impl M5FeatureFamilyLearningManifest {
    /// Recomputes every bundle verdict and the overall verdict from the current
    /// bundle evidence, writing them back.
    pub fn sync_verdicts(&mut self) {
        let mut overall = QualificationVerdict::QualifiedStable;
        let mut reasons: Vec<String> = Vec::new();
        for bundle in &mut self.family_bundles {
            bundle.sync_verdict();
            if bundle.claimed {
                overall = overall.meet(bundle.verdict);
                reasons.extend(bundle.narrowing_reasons.iter().cloned());
            }
        }
        reasons.sort();
        reasons.dedup();
        self.overall_verdict = overall;
        self.overall_narrowing_reasons = reasons;
    }

    /// Returns the bundle for `family`, if present.
    pub fn bundle(&self, family: M5LearningSurfaceFamily) -> Option<&M5FamilyLearningBundle> {
        self.family_bundles.iter().find(|b| b.family == family)
    }
}

// ── Shared proof builders ─────────────────────────────────────────────────────

const GENERATED_AT: &str = "2026-06-11T13:00:00Z";

fn stable_citation(command_ids: &[&str], anchors: &[&str]) -> CitationProof {
    CitationProof {
        has_citation: true,
        command_id_refs: command_ids.iter().map(|s| s.to_string()).collect(),
        docs_citation_anchor_refs: anchors.iter().map(|s| s.to_string()).collect(),
        symbol_linked_refs: vec![],
        all_anchors_live_authoritative: true,
        narrowing_reason: None,
    }
}

fn cached_citation(command_ids: &[&str], anchors: &[&str], reason: &str) -> CitationProof {
    CitationProof {
        has_citation: true,
        command_id_refs: command_ids.iter().map(|s| s.to_string()).collect(),
        docs_citation_anchor_refs: anchors.iter().map(|s| s.to_string()).collect(),
        symbol_linked_refs: vec![],
        all_anchors_live_authoritative: false,
        narrowing_reason: Some(reason.to_string()),
    }
}

fn local_private_privacy() -> PrivacyPosture {
    PrivacyPosture {
        progress_local_by_default: true,
        explicit_promotion_required_for_sharing: true,
        repo_visible: false,
        telemetry_grade_read_access: false,
        narrowing_reason: None,
    }
}

fn disclosed_offline(state: &str) -> OfflinePosture {
    OfflinePosture {
        degradation_state: state.to_string(),
        silent_disappearance_on_offline: false,
        cached_pack_accepted_with_disclosure: true,
        available_in_local_only_profile: true,
        narrowing_reason: None,
    }
}

fn user_local_scope() -> ScopePosture {
    ScopePosture {
        scope_class: ScopeClass::UserLocal,
        follow_mode_requires_explicit_grant: true,
        share_requires_explicit_user_action: true,
        follow_grant_ref: None,
    }
}

fn full_accessibility() -> AccessibilityPosture {
    AccessibilityPosture {
        keyboard_reachable: true,
        screen_reader_narration: true,
        reset_skip_keyboard_accessible: true,
        offline_degradation_accessible: true,
        reduced_motion_honored: true,
    }
}

fn stable_mirror_parity(freshness: &str) -> MirrorParityPosture {
    MirrorParityPosture {
        available_offline: true,
        available_on_mirror: true,
        freshness_label: freshness.to_string(),
        explicit_freshness_disclosed: true,
        silent_dead_link_on_stale: false,
        narrowing_reason: None,
    }
}

/// Builds the verdict for an explain-first, read-only or approval-gated learning
/// asset from its shared proofs.
fn surface_verdict(
    citation: &CitationProof,
    privacy: &PrivacyPosture,
    offline: &OfflinePosture,
    explain_apply: ExplainApplyClass,
) -> (QualificationVerdict, Vec<String>) {
    derive_surface_verdict(&VerdictInputs {
        citation,
        privacy,
        offline,
        explain_apply,
        role_authority: RoleAuthorityClass::WorkspaceAuthorityOnly,
        speaker_note_locality: SpeakerNoteLocality::NotApplicable,
        restore_proof: None,
    })
}

// ── Per-asset builders ────────────────────────────────────────────────────────

fn glossary_pack_for(
    family: M5LearningSurfaceFamily,
    commands: &[&str],
    anchors: &[&str],
) -> GlossaryPackQualificationRecord {
    let citation = stable_citation(commands, anchors);
    let privacy = local_private_privacy();
    let offline = disclosed_offline("cached_disclosed");
    let (verdict, narrowing_reasons) =
        surface_verdict(&citation, &privacy, &offline, ExplainApplyClass::ReadOnly);
    let fam = family.as_str();
    GlossaryPackQualificationRecord {
        record_kind: GLOSSARY_PACK_QUALIFICATION_RECORD_KIND.to_string(),
        schema_version: GUIDED_LEARNING_QUALIFICATION_SCHEMA_VERSION,
        record_id: format!("qualify:m5:glossary_pack:{fam}:v1"),
        generated_at: GENERATED_AT.to_string(),
        pack_ref: format!("learning:m5:glossary_pack:{fam}:v1"),
        lifecycle_label: "beta".to_string(),
        citation,
        privacy,
        offline,
        explain_apply_class: ExplainApplyClass::ReadOnly,
        scope: user_local_scope(),
        accessibility: full_accessibility(),
        verdict,
        narrowing_reasons,
        evidence_fixture_refs: vec![M5_FEATURE_FAMILY_LEARNING_FIXTURE_REF.to_string()],
    }
}

#[allow(clippy::too_many_arguments)]
fn tour_package_for(
    family: M5LearningSurfaceFamily,
    citation: CitationProof,
    waypoints: &[&str],
) -> TourPackageQualificationRecord {
    let privacy = local_private_privacy();
    let offline = disclosed_offline("cached_disclosed");
    let (verdict, narrowing_reasons) = surface_verdict(
        &citation,
        &privacy,
        &offline,
        ExplainApplyClass::ApplyRequiresApproval,
    );
    let fam = family.as_str();
    TourPackageQualificationRecord {
        record_kind: TOUR_PACKAGE_QUALIFICATION_RECORD_KIND.to_string(),
        schema_version: GUIDED_LEARNING_QUALIFICATION_SCHEMA_VERSION,
        record_id: format!("qualify:m5:tour_package:{fam}:v1"),
        generated_at: GENERATED_AT.to_string(),
        package_ref: format!("learning:m5:tour_package:{fam}:v1"),
        lifecycle_label: "beta".to_string(),
        citation,
        privacy,
        offline,
        explain_apply_class: ExplainApplyClass::ApplyRequiresApproval,
        scope: user_local_scope(),
        accessibility: full_accessibility(),
        waypoint_refs: waypoints.iter().map(|s| s.to_string()).collect(),
        verdict,
        narrowing_reasons,
        evidence_fixture_refs: vec![M5_FEATURE_FAMILY_LEARNING_FIXTURE_REF.to_string()],
    }
}

fn help_card_for(
    family: M5LearningSurfaceFamily,
    slug: &str,
    commands: &[&str],
    anchors: &[&str],
) -> ContextualHelpCardRecord {
    let citation = stable_citation(commands, anchors);
    let privacy = local_private_privacy();
    let offline = disclosed_offline("cached_disclosed");
    let (verdict, narrowing_reasons) =
        surface_verdict(&citation, &privacy, &offline, ExplainApplyClass::ReadOnly);
    let fam = family.as_str();
    ContextualHelpCardRecord {
        record_kind: CONTEXTUAL_HELP_CARD_RECORD_KIND.to_string(),
        schema_version: M5_FEATURE_FAMILY_LEARNING_SCHEMA_VERSION,
        record_id: format!("qualify:m5:help_card:{fam}:{slug}:v1"),
        generated_at: GENERATED_AT.to_string(),
        family,
        card_ref: format!("learning:m5:help_card:{fam}:{slug}:v1"),
        lifecycle_label: "beta".to_string(),
        citation,
        offline,
        explain_apply_class: ExplainApplyClass::ReadOnly,
        accessibility: full_accessibility(),
        command_backed: true,
        verdict,
        narrowing_reasons,
        evidence_fixture_refs: vec![M5_FEATURE_FAMILY_LEARNING_FIXTURE_REF.to_string()],
    }
}

fn exercise_rail_for(
    family: M5LearningSurfaceFamily,
    commands: &[&str],
    anchors: &[&str],
) -> ExerciseRailQualificationRecord {
    let citation = stable_citation(commands, anchors);
    let privacy = local_private_privacy();
    let offline = disclosed_offline("cached_disclosed");
    let (verdict, narrowing_reasons) = surface_verdict(
        &citation,
        &privacy,
        &offline,
        ExplainApplyClass::ApplyRequiresApproval,
    );
    let fam = family.as_str();
    ExerciseRailQualificationRecord {
        record_kind: EXERCISE_RAIL_QUALIFICATION_RECORD_KIND.to_string(),
        schema_version: GUIDED_LEARNING_QUALIFICATION_SCHEMA_VERSION,
        record_id: format!("qualify:m5:exercise_rail:{fam}:v1"),
        generated_at: GENERATED_AT.to_string(),
        rail_ref: format!("learning:m5:exercise_rail:{fam}:v1"),
        lifecycle_label: "beta".to_string(),
        citation,
        privacy,
        offline,
        explain_apply_class: ExplainApplyClass::ApplyRequiresApproval,
        scope: ScopePosture {
            scope_class: ScopeClass::UserOrWorkspaceOptIn,
            follow_mode_requires_explicit_grant: true,
            share_requires_explicit_user_action: true,
            follow_grant_ref: None,
        },
        accessibility: full_accessibility(),
        apply_steps_reversible: true,
        verdict,
        narrowing_reasons,
        evidence_fixture_refs: vec![M5_FEATURE_FAMILY_LEARNING_FIXTURE_REF.to_string()],
    }
}

fn progress_snapshot_for(family: M5LearningSurfaceFamily) -> ProgressSnapshotQualificationRecord {
    let privacy = local_private_privacy();
    let offline = disclosed_offline("local_only_disclosed");
    let citation = stable_citation(&[], &[]);
    let (verdict, narrowing_reasons) =
        surface_verdict(&citation, &privacy, &offline, ExplainApplyClass::ReadOnly);
    let fam = family.as_str();
    ProgressSnapshotQualificationRecord {
        record_kind: PROGRESS_SNAPSHOT_QUALIFICATION_RECORD_KIND.to_string(),
        schema_version: GUIDED_LEARNING_QUALIFICATION_SCHEMA_VERSION,
        record_id: format!("qualify:m5:progress_snapshot:{fam}:v1"),
        generated_at: GENERATED_AT.to_string(),
        snapshot_ref: format!("learning:m5:progress:{fam}:v1"),
        lifecycle_label: "beta".to_string(),
        privacy,
        offline,
        scope: user_local_scope(),
        accessibility: full_accessibility(),
        survives_restart: true,
        safe_for_support_export: true,
        verdict,
        narrowing_reasons,
        evidence_fixture_refs: vec![M5_FEATURE_FAMILY_LEARNING_FIXTURE_REF.to_string()],
    }
}

// ── Bundle builder ────────────────────────────────────────────────────────────

struct BundleSpec<'a> {
    family: M5LearningSurfaceFamily,
    commands: &'a [&'a str],
    anchors: &'a [&'a str],
    help_card_slugs: &'a [&'a str],
    waypoints: &'a [&'a str],
    tour_citation: CitationProof,
    mirror_parity: MirrorParityPosture,
    in_product_command_backed_path: bool,
}

fn build_bundle(spec: BundleSpec<'_>) -> M5FamilyLearningBundle {
    let glossary_pack = glossary_pack_for(spec.family, spec.commands, spec.anchors);
    let tour_package = tour_package_for(spec.family, spec.tour_citation, spec.waypoints);
    let contextual_help_cards = spec
        .help_card_slugs
        .iter()
        .map(|slug| help_card_for(spec.family, slug, spec.commands, spec.anchors))
        .collect();
    let exercise_rail = Some(exercise_rail_for(spec.family, spec.commands, spec.anchors));
    let progress_snapshot = progress_snapshot_for(spec.family);

    let mut bundle = M5FamilyLearningBundle {
        record_kind: M5_FAMILY_LEARNING_BUNDLE_RECORD_KIND.to_string(),
        schema_version: M5_FEATURE_FAMILY_LEARNING_SCHEMA_VERSION,
        family: spec.family,
        claimed: true,
        in_product_command_backed_path: spec.in_product_command_backed_path,
        glossary_pack,
        tour_package,
        contextual_help_cards,
        exercise_rail,
        progress_snapshot,
        mirror_parity: spec.mirror_parity,
        verdict: QualificationVerdict::QualifiedStable,
        narrowing_reasons: vec![],
    };
    bundle.sync_verdict();
    bundle
}

// ── Seeded corpus ─────────────────────────────────────────────────────────────

/// Returns the seeded learning manifest covering every claimed M5 feature
/// family.
///
/// The corpus covers all nine M5 depth families. Most families qualify Stable
/// with command-backed, offline-and-mirror-parity learning assets. Two families
/// demonstrate the narrowing invariant:
///
/// - `companion` narrows to Beta because its tour cites a cached (not
///   live-authoritative) anchor revision.
/// - `preview` narrows to Beta because its learning bundle is not yet available
///   on a mirrored profile.
#[allow(clippy::vec_init_then_push)]
pub fn seeded_m5_feature_family_learning_manifest() -> M5FeatureFamilyLearningManifest {
    use M5LearningSurfaceFamily::*;

    // One bundle per claimed M5 depth family. `push` (rather than a `vec!`
    // literal) keeps each family's spec on its own clearly-commented block.
    let mut family_bundles = Vec::new();

    family_bundles.push(build_bundle(BundleSpec {
        family: Notebook,
        commands: &["cmd:notebook.run_cell", "cmd:notebook.export"],
        anchors: &[
            "docs:anchor:notebook:execution_model",
            "docs:anchor:notebook:kernel_trust",
        ],
        help_card_slugs: &["run_cell", "kernel_trust"],
        waypoints: &[
            "learning:waypoint:notebook:open",
            "learning:waypoint:notebook:run_and_review",
        ],
        tour_citation: stable_citation(
            &["cmd:notebook.run_cell"],
            &["docs:anchor:notebook:execution_model"],
        ),
        mirror_parity: stable_mirror_parity("live_authoritative"),
        in_product_command_backed_path: true,
    }));

    family_bundles.push(build_bundle(BundleSpec {
        family: RequestWorkspace,
        commands: &["cmd:request.send", "cmd:request.save_to_collection"],
        anchors: &[
            "docs:anchor:request:auth_profiles",
            "docs:anchor:request:environment_vars",
        ],
        help_card_slugs: &["send_request", "secret_redaction"],
        waypoints: &[
            "learning:waypoint:request:compose",
            "learning:waypoint:request:inspect_response",
        ],
        tour_citation: stable_citation(
            &["cmd:request.send"],
            &["docs:anchor:request:auth_profiles"],
        ),
        mirror_parity: stable_mirror_parity("live_authoritative"),
        in_product_command_backed_path: true,
    }));

    family_bundles.push(build_bundle(BundleSpec {
        family: DatabaseWorkspace,
        commands: &["cmd:database.run_statement", "cmd:database.explain"],
        anchors: &[
            "docs:anchor:database:statement_safety",
            "docs:anchor:database:result_grid",
        ],
        help_card_slugs: &["statement_safety", "result_grid"],
        waypoints: &[
            "learning:waypoint:database:connect",
            "learning:waypoint:database:safe_run",
        ],
        tour_citation: stable_citation(
            &["cmd:database.run_statement"],
            &["docs:anchor:database:statement_safety"],
        ),
        mirror_parity: stable_mirror_parity("live_authoritative"),
        in_product_command_backed_path: true,
    }));

    family_bundles.push(build_bundle(BundleSpec {
        family: ProfilerTrace,
        commands: &["cmd:profiler.start_capture", "cmd:trace.open_flame_graph"],
        anchors: &[
            "docs:anchor:profiler:capture_model",
            "docs:anchor:trace:flame_graph",
        ],
        help_card_slugs: &["start_capture", "read_flame_graph"],
        waypoints: &[
            "learning:waypoint:profiler:capture",
            "learning:waypoint:profiler:interpret",
        ],
        tour_citation: stable_citation(
            &["cmd:profiler.start_capture"],
            &["docs:anchor:profiler:capture_model"],
        ),
        mirror_parity: stable_mirror_parity("live_authoritative"),
        in_product_command_backed_path: true,
    }));

    family_bundles.push(build_bundle(BundleSpec {
        family: DocsBrowser,
        commands: &["cmd:docs.open_in_browser", "cmd:docs.search"],
        anchors: &[
            "docs:anchor:docs_browser:contract",
            "docs:anchor:docs_browser:offline_packs",
        ],
        help_card_slugs: &["open_docs", "offline_packs"],
        waypoints: &[
            "learning:waypoint:docs_browser:open",
            "learning:waypoint:docs_browser:cite_back",
        ],
        tour_citation: stable_citation(
            &["cmd:docs.open_in_browser"],
            &["docs:anchor:docs_browser:contract"],
        ),
        mirror_parity: stable_mirror_parity("mirror_synced_disclosed"),
        in_product_command_backed_path: true,
    }));

    // Preview narrows to Beta: not yet available on a mirrored profile.
    family_bundles.push(build_bundle(BundleSpec {
        family: Preview,
        commands: &["cmd:preview.open", "cmd:preview.refresh"],
        anchors: &[
            "docs:anchor:preview:origin_model",
            "docs:anchor:preview:lineage",
        ],
        help_card_slugs: &["open_preview", "lineage_trace"],
        waypoints: &[
            "learning:waypoint:preview:open",
            "learning:waypoint:preview:trace_lineage",
        ],
        tour_citation: stable_citation(
            &["cmd:preview.open"],
            &["docs:anchor:preview:origin_model"],
        ),
        mirror_parity: MirrorParityPosture {
            available_offline: true,
            available_on_mirror: false,
            freshness_label: "local_only_disclosed".to_string(),
            explicit_freshness_disclosed: true,
            silent_dead_link_on_stale: false,
            narrowing_reason: Some("preview_learning_pack_not_yet_mirror_synced".to_string()),
        },
        in_product_command_backed_path: true,
    }));

    family_bundles.push(build_bundle(BundleSpec {
        family: TemplateScaffold,
        commands: &["cmd:scaffold.plan", "cmd:scaffold.apply"],
        anchors: &[
            "docs:anchor:scaffold:planner_model",
            "docs:anchor:scaffold:lineage",
        ],
        help_card_slugs: &["plan_scaffold", "review_before_apply"],
        waypoints: &[
            "learning:waypoint:scaffold:plan",
            "learning:waypoint:scaffold:review_and_apply",
        ],
        tour_citation: stable_citation(
            &["cmd:scaffold.plan"],
            &["docs:anchor:scaffold:planner_model"],
        ),
        mirror_parity: stable_mirror_parity("live_authoritative"),
        in_product_command_backed_path: true,
    }));

    // Companion narrows to Beta: tour cites a cached (not live) anchor revision.
    family_bundles.push(build_bundle(BundleSpec {
        family: Companion,
        commands: &["cmd:companion.open", "cmd:incident.acknowledge"],
        anchors: &[
            "docs:anchor:companion:surface_contract",
            "docs:anchor:incident:response_model",
        ],
        help_card_slugs: &["open_companion", "incident_handoff"],
        waypoints: &[
            "learning:waypoint:companion:open",
            "learning:waypoint:companion:incident_flow",
        ],
        tour_citation: cached_citation(
            &["cmd:companion.open"],
            &["docs:anchor:companion:surface_contract"],
            "companion_tour_anchors_cached_not_live_authoritative",
        ),
        mirror_parity: stable_mirror_parity("cached_disclosed"),
        in_product_command_backed_path: true,
    }));

    family_bundles.push(build_bundle(BundleSpec {
        family: SyncOffboarding,
        commands: &["cmd:sync.status", "cmd:offboarding.export_bundle"],
        anchors: &[
            "docs:anchor:sync:retention_model",
            "docs:anchor:offboarding:export_and_destroy",
        ],
        help_card_slugs: &["sync_status", "export_and_offboard"],
        waypoints: &[
            "learning:waypoint:sync:review_state",
            "learning:waypoint:offboarding:export",
        ],
        tour_citation: stable_citation(
            &["cmd:offboarding.export_bundle"],
            &["docs:anchor:offboarding:export_and_destroy"],
        ),
        mirror_parity: stable_mirror_parity("live_authoritative"),
        in_product_command_backed_path: true,
    }));

    let mut contract_refs = BTreeMap::new();
    contract_refs.insert(
        "m5_feature_family_learning_schema".to_string(),
        M5_FEATURE_FAMILY_LEARNING_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "guided_learning_contracts_schema".to_string(),
        GUIDED_LEARNING_CONTRACTS_SCHEMA_REF.to_string(),
    );
    contract_refs.insert(
        "artifact_doc".to_string(),
        M5_FEATURE_FAMILY_LEARNING_ARTIFACT_REF.to_string(),
    );
    contract_refs.insert(
        "public_doc".to_string(),
        M5_FEATURE_FAMILY_LEARNING_DOC_REF.to_string(),
    );
    contract_refs.insert(
        "canonical_fixture".to_string(),
        M5_FEATURE_FAMILY_LEARNING_FIXTURE_REF.to_string(),
    );

    let mut manifest = M5FeatureFamilyLearningManifest {
        record_kind: M5_FEATURE_FAMILY_LEARNING_MANIFEST_RECORD_KIND.to_string(),
        schema_version: M5_FEATURE_FAMILY_LEARNING_SCHEMA_VERSION,
        manifest_id: "m5-feature-family-learning:manifest:2026.06.11-01".to_string(),
        generated_at: GENERATED_AT.to_string(),
        contract_refs,
        family_bundles,
        overall_verdict: QualificationVerdict::QualifiedStable,
        overall_narrowing_reasons: vec![],
    };
    manifest.sync_verdicts();
    manifest
}

// ── Validation ────────────────────────────────────────────────────────────────

/// A typed validation error from [`validate_m5_feature_family_learning`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LearningValidationError {
    /// Opaque id of the bundle or record that failed.
    pub subject_id: String,
    /// Human-readable description of the failure.
    pub message: String,
}

impl std::fmt::Display for M5LearningValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.subject_id, self.message)
    }
}

/// Validates a [`M5FeatureFamilyLearningManifest`] against the M5 learnability
/// invariants and returns any violations as typed errors.
///
/// # Errors
///
/// Returns a non-empty `Vec` when any bundle's stored verdict diverges from the
/// verdict derived from its evidence, when a claimed family lacks a glossary,
/// tour, or contextual help asset, when a learning rail conflates explain/apply,
/// when a help card is not command-backed, when a bundle silently dead-links
/// offline or on a mirror, or when progress state is not local-by-default and
/// support-export safe.
pub fn validate_m5_feature_family_learning(
    manifest: &M5FeatureFamilyLearningManifest,
) -> Result<(), Vec<M5LearningValidationError>> {
    let mut errors: Vec<M5LearningValidationError> = Vec::new();

    for bundle in &manifest.family_bundles {
        let subject = format!("bundle:{}", bundle.family.as_str());

        // Stored verdict must match derived verdict.
        let (derived, _) = derive_bundle_verdict(bundle);
        if derived != bundle.verdict {
            errors.push(M5LearningValidationError {
                subject_id: subject.clone(),
                message: format!(
                    "stored verdict {:?} diverges from derived {:?}",
                    bundle.verdict, derived
                ),
            });
        }

        if !bundle.claimed {
            // Unclaimed families carry no further obligations.
            continue;
        }

        // Guardrail: every claimed family must expose in-product, command-backed
        // learnability assets — AI chat / browser handoff cannot be the only path.
        if !bundle.in_product_command_backed_path {
            errors.push(M5LearningValidationError {
                subject_id: subject.clone(),
                message: "claimed family has no in-product command-backed learning path"
                    .to_string(),
            });
        }

        // A claimed family must expose at least one contextual help card.
        if bundle.contextual_help_cards.is_empty() {
            errors.push(M5LearningValidationError {
                subject_id: subject.clone(),
                message: "claimed family has no contextual help cards".to_string(),
            });
        }

        // Mirror/offline parity: never a silent dead link.
        if bundle.mirror_parity.silent_dead_link_on_stale {
            errors.push(M5LearningValidationError {
                subject_id: subject.clone(),
                message: "bundle shows a silent dead link when stale/offline".to_string(),
            });
        }
        if !bundle.mirror_parity.explicit_freshness_disclosed {
            errors.push(M5LearningValidationError {
                subject_id: subject.clone(),
                message: "bundle does not disclose freshness explicitly".to_string(),
            });
        }

        // Glossary pack must be citation-backed.
        if !bundle.glossary_pack.citation.has_citation {
            errors.push(M5LearningValidationError {
                subject_id: subject.clone(),
                message: "glossary pack is not citation-backed".to_string(),
            });
        }

        // Tour package must not conflate explain/apply (no tutorial-only bypass).
        if bundle.tour_package.explain_apply_class == ExplainApplyClass::Conflated {
            errors.push(M5LearningValidationError {
                subject_id: subject.clone(),
                message: "tour package conflates explain/apply".to_string(),
            });
        }

        // Help cards must be command-backed and not conflate explain/apply.
        for card in &bundle.contextual_help_cards {
            if !card.command_backed {
                errors.push(M5LearningValidationError {
                    subject_id: card.record_id.clone(),
                    message: "contextual help card is not command-backed".to_string(),
                });
            }
            if card.explain_apply_class == ExplainApplyClass::Conflated {
                errors.push(M5LearningValidationError {
                    subject_id: card.record_id.clone(),
                    message: "contextual help card conflates explain/apply".to_string(),
                });
            }
        }

        // Exercise rail Apply steps must ride the standard reversible path.
        if let Some(rail) = &bundle.exercise_rail {
            if rail.explain_apply_class == ExplainApplyClass::Conflated {
                errors.push(M5LearningValidationError {
                    subject_id: rail.record_id.clone(),
                    message: "exercise rail conflates explain/apply".to_string(),
                });
            }
            if !rail.apply_steps_reversible {
                errors.push(M5LearningValidationError {
                    subject_id: rail.record_id.clone(),
                    message: "exercise rail apply steps are not reversible".to_string(),
                });
            }
        }

        // Progress must be user-owned, private, and support-export safe.
        let snap = &bundle.progress_snapshot;
        if !snap.privacy.progress_local_by_default || snap.privacy.repo_visible {
            errors.push(M5LearningValidationError {
                subject_id: snap.record_id.clone(),
                message: "progress snapshot is not local-by-default / private".to_string(),
            });
        }
        if snap.privacy.telemetry_grade_read_access {
            errors.push(M5LearningValidationError {
                subject_id: snap.record_id.clone(),
                message: "progress snapshot grants telemetry-grade read access".to_string(),
            });
        }
        if !snap.safe_for_support_export {
            errors.push(M5LearningValidationError {
                subject_id: snap.record_id.clone(),
                message: "progress snapshot is not safe for support export".to_string(),
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
