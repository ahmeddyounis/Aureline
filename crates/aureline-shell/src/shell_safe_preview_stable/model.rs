//! Canonical stable truth model for **suspicious-content, safe-preview,
//! copy/export, and representation cues on shell-adjacent surfaces**.
//!
//! ## Why one governed record per shell-adjacent surface posture
//!
//! Shell-adjacent surfaces — notifications, the activity center, browser /
//! open-external handoff, support export, screenshot / evidence capture, and
//! trust-sensitive review actions like install, attach, approve, publish, and
//! delete — all converge on the same risk: a surface that *renders or hands off*
//! ambiguous content quietly flattens it into a generic "preview" string. When
//! that happens the bidi-control, invisible-formatting, or mixed-script warning
//! disappears, the trust class is lost, and the raw-vs-rendered representation
//! choice (Copy raw / Copy rendered / Copy escaped) never reaches the user. A
//! switching user then commits a trust-sensitive action against bytes that do
//! not mean what they appear to mean.
//!
//! This module mints one governed [`SafePreviewRecord`] per shell-adjacent
//! surface posture. The record binds, for a single shell-adjacent surface
//! identity:
//!
//! - **The consumed trust-class ladder** — the [`TrustClass`] (`RawText`,
//!   `SanitizedRich`, `TrustedLocalActive`, `IsolatedRemoteActive`) and the
//!   [`DetectorOutcomeClass`] come straight from
//!   [`aureline_content_safety`]; this lane does not invent a parallel
//!   evidence vocabulary.
//! - **Explicit representation cues** — a raw/reveal affordance, a representation
//!   label, and `Copy raw` / `Copy rendered` / `Copy escaped` choices that stay
//!   explicit whenever rendered meaning can differ from source bytes.
//! - **Surfaced suspicious-content findings** — every finding keeps its reveal
//!   affordances and a reachable escaped-copy path rather than collapsing to a
//!   single warning glyph.
//! - **Cue survival across carriers** — the trust class, the representation
//!   label, and the suspicious-content warning survive the notification,
//!   activity-center, browser-handoff, support-export, and screenshot/evidence
//!   carriers without flattening to a generic preview.
//! - **A stricter-boundary disclosure before commit** — trust-sensitive actions
//!   (install, attach, approve, publish, delete, open-external) may enforce a
//!   stricter preview class than ordinary browsing, but they must show that
//!   stricter boundary before the user commits.
//! - **Complete accessibility cues** — the warning is announced (never
//!   color-only), the representation label and trust class are announced, and the
//!   reveal affordance is keyboard reachable across normal / high-contrast /
//!   zoomed layouts.
//! - **Per-OS conformance** — macOS, Windows, and Linux each carry current proof.
//! - **A public claim ceiling** and **automatic narrowing** below Stable with a
//!   named reason.
//! - **Recovery, route, and accessibility parity** and **no-account /
//!   no-managed-services availability**.
//!
//! The shell surface, the activity center, the CLI inspector, Help/About, and the
//! diagnostics support export read this record verbatim instead of cloning status
//! text. The trust-class ladder, the suspicious-content detector, and the
//! representation-transfer grammar are **not** reinvented here: the record is a
//! genuine projection of the live detector and representation builders in
//! [`aureline_content_safety`].
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `certify-suspicious-content-safe-preview-copy-export-and`) are:
//!
//! - [`model`](self) — the governed record, its closed vocabularies, the builder,
//!   and the honesty invariants. The boundary schema is
//!   `schemas/ux/certify-suspicious-content-safe-preview-copy-export-and.schema.json`.
//! - [`corpus`](super::corpus) — the deterministic claimed-stable matrix,
//!   projected through the live content-safety detector, and pinned on disk under
//!   `fixtures/ux/m4/certify-suspicious-content-safe-preview-copy-export-and/`.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use aureline_content_safety::{
    BodyPosture, DetectorOutcomeClass, RepresentationActionId, RepresentationClass,
    SuspiciousContentClass, TrustClass,
};

use crate::notification_attention_stable::model::{
    is_canonical_object_ref, AccessibilityDisclosure, AttentionRouteSurface, EntryRouteRecord,
    LayoutMode, LifecycleMarker, RecoveryActionRole, RecoveryRouteRecord, StableClaimClass,
};

/// Stable record-kind tag carried in serialized safe-preview records.
pub const SAFE_PREVIEW_RECORD_KIND: &str = "shell_safe_preview_record";

/// Schema version for the [`SafePreviewRecord`] payload shape.
pub const SAFE_PREVIEW_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const SAFE_PREVIEW_SHARED_CONTRACT_REF: &str = "shell:safe_preview_stable:v1";

/// Content-safety contract this record is a genuine projection of.
pub const CONTENT_SAFETY_CONTRACT_REF: &str = "content-safety:trust_class_and_representation:v1";

/// Reviewer-facing notice rendered on every safe-preview surface.
pub const SAFE_PREVIEW_NOTICE: &str =
    "Safe-preview truth: each claimed-stable shell-adjacent surface that can render or hand off \
     ambiguous content consumes the content-safety trust-class ladder (RawText, SanitizedRich, \
     TrustedLocalActive, IsolatedRemoteActive) and the suspicious-content detector verbatim rather \
     than minting a parallel vocabulary; a raw/reveal affordance, a representation label, and Copy \
     raw / Copy rendered / Copy escaped choices stay explicit whenever rendered meaning can differ \
     from source bytes; suspicious-content findings keep their reveal affordances and a reachable \
     escaped-copy path instead of collapsing to a single glyph; the trust class, the representation \
     label, and the suspicious-content warning survive the notification, activity-center, \
     browser-handoff, support-export, and screenshot/evidence carriers without flattening to a \
     generic preview; trust-sensitive actions such as install, attach, approve, publish, delete, \
     and open-external may enforce a stricter preview class than ordinary browsing but must show \
     that stricter boundary before the user commits; the warning is announced rather than \
     color-only and the reveal affordance is keyboard reachable across normal, high-contrast, and \
     zoomed layouts; per-OS conformance covers macOS, Windows, and Linux; a posture that cannot \
     prove a pillar, or that sits on a binding surface whose own marker is below Stable, is narrowed \
     below Stable with a named reason rather than inheriting an adjacent green row; the same posture \
     opens from the activity center, command palette, status bar, and a menu command, keyboard-first; \
     and every posture stays available without an account or managed services.";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on a present (non-canonical) ref.
const MAX_REF_CHARS: usize = 200;

fn is_reviewable_sentence(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_SENTENCE_CHARS
}

fn is_present_ref(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_REF_CHARS
}

fn require_canonical_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_string(),
        })
    }
}

fn require_present_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_present_ref(value) {
        Ok(())
    } else {
        Err(BuildError::MissingUpstreamRef { field })
    }
}

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// Shell-adjacent surface family a posture covers. Carriers render or hand off
/// content; trust-sensitive actions commit a side effect against it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellAdjacentSurface {
    /// An OS / in-product notification surface.
    Notification,
    /// The durable activity-center row.
    ActivityCenter,
    /// A browser / open-external preview handed off from the shell.
    BrowserHandoff,
    /// A redacted support / diagnostics export surface.
    SupportExport,
    /// A screenshot / evidence-capture path.
    ScreenshotEvidence,
    /// Extension / package install review (trust-sensitive action).
    InstallReview,
    /// Remote attach / connect review (trust-sensitive action).
    AttachReview,
    /// An approval surface (trust-sensitive action).
    ApproveSurface,
    /// A publish review (trust-sensitive action).
    PublishReview,
    /// A delete review (trust-sensitive action).
    DeleteReview,
    /// Commit to open content in the system browser / external app
    /// (trust-sensitive action).
    OpenExternalHandoff,
}

impl ShellAdjacentSurface {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notification => "notification",
            Self::ActivityCenter => "activity_center",
            Self::BrowserHandoff => "browser_handoff",
            Self::SupportExport => "support_export",
            Self::ScreenshotEvidence => "screenshot_evidence",
            Self::InstallReview => "install_review",
            Self::AttachReview => "attach_review",
            Self::ApproveSurface => "approve_surface",
            Self::PublishReview => "publish_review",
            Self::DeleteReview => "delete_review",
            Self::OpenExternalHandoff => "open_external_handoff",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Notification => "notification",
            Self::ActivityCenter => "activity center",
            Self::BrowserHandoff => "browser handoff",
            Self::SupportExport => "support export",
            Self::ScreenshotEvidence => "screenshot/evidence capture",
            Self::InstallReview => "install review",
            Self::AttachReview => "attach review",
            Self::ApproveSurface => "approval surface",
            Self::PublishReview => "publish review",
            Self::DeleteReview => "delete review",
            Self::OpenExternalHandoff => "open-external handoff",
        }
    }

    /// True when the surface commits a trust-sensitive side effect and may
    /// therefore enforce a stricter preview class than ordinary browsing.
    pub const fn is_trust_sensitive_action(self) -> bool {
        matches!(
            self,
            Self::InstallReview
                | Self::AttachReview
                | Self::ApproveSurface
                | Self::PublishReview
                | Self::DeleteReview
                | Self::OpenExternalHandoff
        )
    }
}

/// Carrier path a shell-adjacent cue must survive without flattening. Every
/// record must prove all five so trust-sensitive shell flows never lose
/// representation or trust-class truth on the way to a downstream surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CueCarrier {
    /// OS / in-product notification.
    Notification,
    /// Durable activity-center row.
    ActivityCenter,
    /// Browser / open-external handoff.
    BrowserHandoff,
    /// Redacted support / diagnostics export.
    SupportExport,
    /// Screenshot / evidence capture.
    ScreenshotEvidence,
}

impl CueCarrier {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notification => "notification",
            Self::ActivityCenter => "activity_center",
            Self::BrowserHandoff => "browser_handoff",
            Self::SupportExport => "support_export",
            Self::ScreenshotEvidence => "screenshot_evidence",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Notification => "Notification",
            Self::ActivityCenter => "Activity center",
            Self::BrowserHandoff => "Browser handoff",
            Self::SupportExport => "Support export",
            Self::ScreenshotEvidence => "Screenshot/evidence",
        }
    }

    /// Every carrier a Stable posture must prove cues survive.
    pub const REQUIRED: [Self; 5] = [
        Self::Notification,
        Self::ActivityCenter,
        Self::BrowserHandoff,
        Self::SupportExport,
        Self::ScreenshotEvidence,
    ];
}

/// Surface that ingests the shared safe-preview record. The same record drives
/// the shell surface, the activity center, the CLI inspector, Help/About, and the
/// support export rather than each cloning prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafePreviewTruthSurface {
    /// The live shell surface that renders the preview.
    ShellSurface,
    /// The durable activity-center projection.
    ActivityCenter,
    /// The CLI / headless inspector.
    CliInspect,
    /// The Help/About safe-preview posture.
    HelpAbout,
    /// The diagnostics support export.
    SupportExport,
}

impl SafePreviewTruthSurface {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellSurface => "shell_surface",
            Self::ActivityCenter => "activity_center",
            Self::CliInspect => "cli_inspect",
            Self::HelpAbout => "help_about",
            Self::SupportExport => "support_export",
        }
    }

    /// The five surfaces that must all bind the shared record.
    pub const REQUIRED: [Self; 5] = [
        Self::ShellSurface,
        Self::ActivityCenter,
        Self::CliInspect,
        Self::HelpAbout,
        Self::SupportExport,
    ];
}

/// Closed recovery-action vocabulary exposed on a safe-preview posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafePreviewRecoveryAction {
    /// Reveal the exact source bytes (raw view).
    RevealRawSource,
    /// Copy the safe (escaped / sanitized) representation.
    CopySafeRepresentation,
    /// Open the codepoint inspector for the suspicious finding.
    InspectCodepoints,
    /// Open the content-safety help / glossary.
    OpenContentSafetyHelp,
    /// Export a redacted safe-preview support packet.
    ExportSafePreviewSupport,
}

impl SafePreviewRecoveryAction {
    /// Stable action id quoted across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealRawSource => "reveal_raw_source",
            Self::CopySafeRepresentation => "copy_safe_representation",
            Self::InspectCodepoints => "inspect_codepoints",
            Self::OpenContentSafetyHelp => "open_content_safety_help",
            Self::ExportSafePreviewSupport => "export_safe_preview_support",
        }
    }

    /// Reviewer-facing label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::RevealRawSource => "Reveal raw source",
            Self::CopySafeRepresentation => "Copy safe representation",
            Self::InspectCodepoints => "Inspect codepoints",
            Self::OpenContentSafetyHelp => "Open content-safety help",
            Self::ExportSafePreviewSupport => "Export safe-preview support",
        }
    }

    /// Placement / confirmation role for this action.
    pub const fn role(self) -> RecoveryActionRole {
        match self {
            Self::RevealRawSource | Self::CopySafeRepresentation => RecoveryActionRole::Primary,
            Self::InspectCodepoints => RecoveryActionRole::Recovery,
            Self::OpenContentSafetyHelp | Self::ExportSafePreviewSupport => {
                RecoveryActionRole::Secondary
            }
        }
    }

    /// Builds a route record for this action.
    pub fn route(self) -> RecoveryRouteRecord {
        RecoveryRouteRecord {
            action_id: self.as_str().to_string(),
            action_label: self.surface_label().to_string(),
            action_role: self.role(),
            keyboard_reachable: true,
        }
    }

    /// The recovery actions every posture must expose regardless of surface.
    pub const REQUIRED: [Self; 3] = [
        Self::RevealRawSource,
        Self::CopySafeRepresentation,
        Self::ExportSafePreviewSupport,
    ];
}

/// Returns the recovery routes a posture must expose, in rendered order, given
/// whether the posture carries an inspectable suspicious finding.
pub fn required_recovery_routes(has_finding: bool) -> Vec<RecoveryRouteRecord> {
    let mut actions = vec![
        SafePreviewRecoveryAction::RevealRawSource,
        SafePreviewRecoveryAction::CopySafeRepresentation,
    ];
    if has_finding {
        actions.push(SafePreviewRecoveryAction::InspectCodepoints);
    }
    actions.push(SafePreviewRecoveryAction::OpenContentSafetyHelp);
    actions.push(SafePreviewRecoveryAction::ExportSafePreviewSupport);
    actions
        .into_iter()
        .map(SafePreviewRecoveryAction::route)
        .collect()
}

/// Per-OS desktop profile a conformance row covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformProfileClass {
    /// macOS (universal).
    #[serde(rename = "macos")]
    MacOs,
    /// Windows (x86_64).
    Windows,
    /// Linux (GNOME/Wayland, x86_64).
    Linux,
}

impl PlatformProfileClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MacOs => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
        }
    }

    /// Every per-OS profile a Stable conformance posture must cover.
    pub const REQUIRED: [Self; 3] = [Self::MacOs, Self::Windows, Self::Linux];
}

/// Closed reason a posture is narrowed below Stable. Required whenever the claim
/// class is below the cutline; forbidden when it is Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafePreviewNarrowingReason {
    /// The raw/reveal affordance, the representation label, or the Copy raw /
    /// Copy rendered choice is missing where meaning can differ from source.
    RepresentationCuesNotExplicit,
    /// A suspicious-content finding is flattened to a single glyph without a
    /// reveal affordance or a reachable escaped-copy path.
    SuspiciousFindingsFlattened,
    /// A copy/export choice lacks an explicit representation label or class, or
    /// the posture offers no raw and no safe-inspection path.
    CopyExportNotLabeled,
    /// A carrier flattens the trust class, the representation label, or the
    /// suspicious-content warning into a generic preview.
    CuesFlattenedOnCarrier,
    /// A trust-sensitive action does not show its stricter preview boundary
    /// before the user commits.
    StricterBoundaryNotShownBeforeCommit,
    /// The warning is color-only, or the representation label / reveal affordance
    /// is not announced or keyboard reachable across layouts.
    AccessibilityCuesIncomplete,
    /// Per-OS conformance is incomplete.
    PlatformConformanceIncomplete,
    /// The binding surface's own lifecycle marker is below Stable, so it must not
    /// inherit Stable by adjacency.
    SurfaceNotYetStable,
}

impl SafePreviewNarrowingReason {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepresentationCuesNotExplicit => "representation_cues_not_explicit",
            Self::SuspiciousFindingsFlattened => "suspicious_findings_flattened",
            Self::CopyExportNotLabeled => "copy_export_not_labeled",
            Self::CuesFlattenedOnCarrier => "cues_flattened_on_carrier",
            Self::StricterBoundaryNotShownBeforeCommit => {
                "stricter_boundary_not_shown_before_commit"
            }
            Self::AccessibilityCuesIncomplete => "accessibility_cues_incomplete",
            Self::PlatformConformanceIncomplete => "platform_conformance_incomplete",
            Self::SurfaceNotYetStable => "surface_not_yet_stable",
        }
    }
}

// ---------------------------------------------------------------------------
// Per-pillar evidence blocks
// ---------------------------------------------------------------------------

/// Input form of the representation-cue block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepresentationCuesInput {
    /// Whether a raw/reveal affordance is present on the surface.
    pub raw_reveal_available: bool,
    /// Whether a representation label is present on the surface.
    pub representation_label_present: bool,
    /// The reviewable representation label rendered on the surface.
    pub representation_label: String,
}

/// Output form of the representation-cue block with the derived explicitness
/// verdict. `copy_raw_present` and `copy_rendered_present` are derived from the
/// posture's representation choices so the cue block can never disagree with the
/// actual copy/export affordances.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationCues {
    /// Whether a raw/reveal affordance is present on the surface.
    pub raw_reveal_available: bool,
    /// Whether a representation label is present on the surface.
    pub representation_label_present: bool,
    /// The reviewable representation label rendered on the surface.
    pub representation_label: String,
    /// Whether a Copy raw choice is present (derived from the choices).
    pub copy_raw_present: bool,
    /// Whether a Copy rendered choice is present (derived from the choices).
    pub copy_rendered_present: bool,
    /// Whether the raw-vs-rendered choice stays explicit whenever rendered
    /// meaning can differ from source bytes (derived).
    pub explicit_when_meaning_differs: bool,
}

/// One surfaced suspicious-content finding. The class is consumed from the
/// content-safety vocabulary so this lane never re-spells the detector taxonomy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuspiciousFindingRow {
    /// Stable finding id (matches the detector finding id where projected).
    pub finding_id: String,
    /// Suspicious-content class, consumed from the content-safety detector.
    pub content_class: SuspiciousContentClass,
    /// Reviewable visibility-impact token (e.g. `reorders_text`).
    pub visibility_impact: String,
    /// Reviewable per-finding representation label.
    pub representation_label: String,
    /// Reveal affordances surfaced for this finding, in rendered order.
    pub reveal_affordances: Vec<String>,
    /// Whether a raw toggle is reachable for this finding.
    pub raw_toggle_available: bool,
    /// Whether an escaped-copy path is reachable for this finding.
    pub escaped_copy_available: bool,
}

impl SuspiciousFindingRow {
    /// True when the finding keeps its reveal affordances and a reachable
    /// escaped-copy path rather than collapsing to a single glyph.
    pub fn holds(&self) -> bool {
        !self.reveal_affordances.is_empty()
            && self.raw_toggle_available
            && self.escaped_copy_available
    }
}

/// One representation-labeled copy/export choice. The action, representation
/// class, and body posture are consumed from the content-safety vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationChoiceRow {
    /// Transfer-action id, consumed from the content-safety vocabulary.
    pub action_id: RepresentationActionId,
    /// Representation class, consumed from the content-safety vocabulary.
    pub representation_class: RepresentationClass,
    /// Body posture, consumed from the content-safety vocabulary.
    pub body_posture: BodyPosture,
    /// Reviewable choice label (e.g. "Copy raw").
    pub label: String,
    /// Whether the raw source is required to fulfil this choice.
    pub raw_source_required: bool,
    /// Whether active content is removed before transfer.
    pub active_content_removed: bool,
}

impl RepresentationChoiceRow {
    /// True when this choice transfers the exact source bytes.
    pub fn is_exact_raw(&self) -> bool {
        matches!(self.representation_class, RepresentationClass::Raw)
    }

    /// True when this choice transfers a rendered view.
    pub fn is_rendered(&self) -> bool {
        matches!(self.representation_class, RepresentationClass::Rendered)
    }

    /// True when this choice transfers a safe-inspection representation.
    pub fn is_safe_inspection(&self) -> bool {
        matches!(
            self.representation_class,
            RepresentationClass::Escaped
                | RepresentationClass::Sanitized
                | RepresentationClass::BlockedMetadataOnly
        )
    }
}

/// One carrier's cue-survival evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CueCarrierRow {
    /// The carrier path.
    pub carrier: CueCarrier,
    /// Whether the trust class survives the carrier.
    pub preserves_trust_class: bool,
    /// Whether the representation label survives the carrier.
    pub preserves_representation_label: bool,
    /// Whether the suspicious-content warning survives the carrier.
    pub preserves_suspicious_warning: bool,
    /// Whether the carrier avoids flattening the content into a generic preview.
    pub does_not_flatten_to_generic_preview: bool,
    /// Reviewable sentence describing what the carrier renders.
    pub carried_summary: String,
}

impl CueCarrierRow {
    /// True when the carrier preserves the trust class, the representation label,
    /// and the suspicious-content warning without flattening to a generic
    /// preview.
    pub fn holds(&self) -> bool {
        self.preserves_trust_class
            && self.preserves_representation_label
            && self.preserves_suspicious_warning
            && self.does_not_flatten_to_generic_preview
    }
}

/// The stricter-preview boundary a trust-sensitive action enforces before
/// commit. The ordinary and enforced classes are consumed from the trust-class
/// ladder so the boundary is expressed in the shared vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StricterBoundary {
    /// The trust-sensitive action this boundary guards.
    pub action: ShellAdjacentSurface,
    /// Whether the action enforces a stricter preview class than ordinary
    /// browsing.
    pub enforces_stricter_preview_class: bool,
    /// The trust class ordinary browsing would use.
    pub ordinary_browsing_class: TrustClass,
    /// The stricter trust class the action enforces.
    pub enforced_preview_class: TrustClass,
    /// Whether the stricter boundary is shown before the user commits.
    pub shows_boundary_before_commit: bool,
    /// Whether commit is blocked until the boundary is acknowledged.
    pub commit_blocked_until_acknowledged: bool,
    /// Reviewable disclosure sentence shown before commit.
    pub boundary_disclosure: String,
}

impl StricterBoundary {
    /// True when the action enforces a stricter class, shows the boundary before
    /// commit, and blocks commit until the boundary is acknowledged.
    pub fn holds(&self) -> bool {
        self.enforces_stricter_preview_class
            && self.shows_boundary_before_commit
            && self.commit_blocked_until_acknowledged
    }
}

/// Accessibility cue evidence for one safe-preview posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafePreviewA11yCues {
    /// Whether the warning is announced rather than conveyed by color alone.
    pub warning_announced_not_color_only: bool,
    /// Whether the representation label is announced.
    pub representation_label_announced: bool,
    /// Whether the trust class is announced.
    pub trust_class_announced: bool,
    /// Whether the reveal affordance is keyboard reachable.
    pub reveal_affordance_keyboard_reachable: bool,
    /// Reviewable warning-summary label read by assistive technology.
    pub warning_summary_label: String,
}

impl SafePreviewA11yCues {
    /// True when the warning, representation label, and trust class are announced
    /// and the reveal affordance is keyboard reachable.
    pub fn holds(&self) -> bool {
        self.warning_announced_not_color_only
            && self.representation_label_announced
            && self.trust_class_announced
            && self.reveal_affordance_keyboard_reachable
    }
}

/// Per-OS conformance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformConformanceRow {
    /// The per-OS profile.
    pub profile: PlatformProfileClass,
    /// Stable profile id (e.g. `macos_15_plus_universal`).
    pub profile_id: String,
    /// Whether the profile is covered with current proof.
    pub covered: bool,
    /// Source proof ref.
    pub proof_ref: String,
    /// Named safe-preview behaviors exercised on this profile.
    pub named_behaviors: Vec<String>,
}

/// Input form of one binding-surface projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafePreviewSurfaceProjectionInput {
    /// The binding surface.
    pub surface: SafePreviewTruthSurface,
    /// The surface's own lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Whether the surface reads the shared record rather than cloning prose.
    pub reads_shared_record: bool,
}

/// Output form of one binding-surface projection, with a derived summary line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafePreviewSurfaceProjection {
    /// The binding surface.
    pub surface: SafePreviewTruthSurface,
    /// The surface's own lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Whether the surface reads the shared record rather than cloning prose.
    pub reads_shared_record: bool,
    /// Derived, deterministic summary line the surface renders.
    pub summary_line: String,
}

/// The proven pillars of one safe-preview posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafePreviewPillars {
    /// Whether the raw/reveal affordance, the representation label, and the Copy
    /// raw / Copy rendered choice stay explicit where meaning can differ.
    pub representation_cues_explicit: bool,
    /// Whether every suspicious-content finding keeps its reveal affordances and
    /// a reachable escaped-copy path.
    pub suspicious_findings_surfaced: bool,
    /// Whether every copy/export choice carries an explicit representation label
    /// and class with a raw and a safe-inspection path.
    pub copy_export_labeled: bool,
    /// Whether the cues survive all five carriers without flattening.
    pub cues_survive_all_carriers: bool,
    /// Whether a trust-sensitive action shows its stricter boundary before
    /// commit (vacuously true for non-action carriers).
    pub stricter_boundary_shown_before_commit: bool,
    /// Whether the accessibility cues are complete.
    pub accessibility_cues_complete: bool,
    /// Whether per-OS conformance is complete.
    pub platform_conformance_complete: bool,
}

/// The public claim ceiling: what a posture is allowed to assert. Each field
/// must be provable from the posture's real evidence; the builder enforces it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SafePreviewClaimCeiling {
    /// Whether the posture may claim explicit representation cues.
    pub asserts_representation_cues_explicit: bool,
    /// Whether the posture may claim surfaced suspicious findings.
    pub asserts_suspicious_findings_surfaced: bool,
    /// Whether the posture may claim labeled copy/export.
    pub asserts_copy_export_labeled: bool,
    /// Whether the posture may claim cue survival across carriers.
    pub asserts_cues_survive_all_carriers: bool,
    /// Whether the posture may claim a stricter boundary shown before commit.
    pub asserts_stricter_boundary_shown_before_commit: bool,
    /// Whether the posture may claim complete accessibility cues.
    pub asserts_accessibility_cues_complete: bool,
    /// Whether the posture may claim complete per-OS conformance.
    pub asserts_platform_conformance_complete: bool,
}

/// The derived stable-claim verdict for a posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafePreviewQualification {
    /// The derived claim class (Stable when fully qualified, else narrowed).
    pub claim_class: StableClaimClass,
    /// Whether the posture qualifies at or above the launch cutline.
    pub qualifies_stable: bool,
    /// The reasons the posture is narrowed below Stable, in canonical order.
    pub narrowing_reasons: Vec<SafePreviewNarrowingReason>,
}

/// Upstream ids the record is a genuine projection of, kept for support
/// traceability. These are upstream source refs, not canonical durable objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafePreviewUpstream {
    /// Content-safety contract ref this record projects from.
    pub content_safety_contract_ref: String,
    /// Trust-class schema version consumed from the content-safety crate.
    pub trust_class_schema_version: u32,
    /// Representation-policy schema version consumed from the content-safety
    /// crate.
    pub representation_policy_schema_version: u32,
    /// Contributing content-safety case ids this record projects from, in order.
    pub contributing_case_refs: Vec<String>,
}

/// Validated input used to mint a [`SafePreviewRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafePreviewInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The shell-adjacent surface family this posture covers.
    pub surface_class: ShellAdjacentSurface,
    /// Present id of the live shell-adjacent surface.
    pub surface_id_ref: String,
    /// Whether the surface renders rich content (so a rendered view exists and
    /// can differ from source bytes).
    pub renders_rich_content: bool,
    /// The trust class, consumed from the content-safety ladder.
    pub trust_class: TrustClass,
    /// The detector outcome, consumed from the content-safety detector.
    pub detector_outcome: DetectorOutcomeClass,
    /// The representation-cue block.
    pub representation: RepresentationCuesInput,
    /// The surfaced suspicious-content findings, in canonical order.
    pub suspicious_findings: Vec<SuspiciousFindingRow>,
    /// The representation-labeled copy/export choices, in canonical order.
    pub representation_choices: Vec<RepresentationChoiceRow>,
    /// The carrier cue-survival rows, in canonical order.
    pub cue_survival: Vec<CueCarrierRow>,
    /// The stricter-preview boundary for trust-sensitive actions, if any.
    pub stricter_boundary: Option<StricterBoundary>,
    /// The accessibility cue evidence.
    pub a11y_cues: SafePreviewA11yCues,
    /// The per-OS conformance rows.
    pub platform_conformance: Vec<PlatformConformanceRow>,
    /// The binding-surface projections.
    pub surface_projections: Vec<SafePreviewSurfaceProjectionInput>,
    /// Public claim ceiling.
    pub claim_ceiling: SafePreviewClaimCeiling,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the same posture.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the posture stays available without an account.
    pub available_without_account: bool,
    /// Whether the posture stays available without managed services.
    pub available_without_managed_services: bool,
    /// Upstream ids the record projects from.
    pub upstream: SafePreviewUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// The canonical, governed safe-preview record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafePreviewRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The shell-adjacent surface family this posture covers.
    pub surface_class: ShellAdjacentSurface,
    /// Present id of the live shell-adjacent surface.
    pub surface_id_ref: String,
    /// Whether the surface is a trust-sensitive action (derived).
    pub is_trust_sensitive_action: bool,
    /// Whether the surface renders rich content.
    pub renders_rich_content: bool,
    /// The lowest binding-surface lifecycle marker.
    pub surface_lifecycle_marker: LifecycleMarker,
    /// The trust class, consumed from the content-safety ladder.
    pub trust_class: TrustClass,
    /// The detector outcome, consumed from the content-safety detector.
    pub detector_outcome: DetectorOutcomeClass,
    /// Whether suspicious content is present (derived).
    pub suspicious_content_present: bool,
    /// The representation-cue block with the derived explicitness verdict.
    pub representation: RepresentationCues,
    /// The surfaced suspicious-content findings, in canonical order.
    pub suspicious_findings: Vec<SuspiciousFindingRow>,
    /// The representation-labeled copy/export choices, in canonical order.
    pub representation_choices: Vec<RepresentationChoiceRow>,
    /// The carrier cue-survival rows, in canonical order.
    pub cue_survival: Vec<CueCarrierRow>,
    /// The stricter-preview boundary for trust-sensitive actions, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stricter_boundary: Option<StricterBoundary>,
    /// The accessibility cue evidence.
    pub a11y_cues: SafePreviewA11yCues,
    /// The per-OS conformance rows.
    pub platform_conformance: Vec<PlatformConformanceRow>,
    /// The binding-surface projections.
    pub surface_projections: Vec<SafePreviewSurfaceProjection>,
    /// The proven pillars.
    pub pillars: SafePreviewPillars,
    /// The public claim ceiling.
    pub claim_ceiling: SafePreviewClaimCeiling,
    /// The derived stable-claim verdict.
    pub stable_qualification: SafePreviewQualification,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the same posture.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the posture stays available without an account.
    pub available_without_account: bool,
    /// Whether the posture stays available without managed services.
    pub available_without_managed_services: bool,
    /// Whether the honesty marker is rendered (narrowed or below-Stable surface).
    pub honesty_marker_present: bool,
    /// Upstream ids the record projects from.
    pub upstream: SafePreviewUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// Reasons a [`SafePreviewRecord`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// An upstream projection ref was missing.
    MissingUpstreamRef { field: &'static str },
    /// A required cue carrier was missing.
    MissingCueCarrier { carrier: CueCarrier },
    /// A required per-OS profile was missing.
    MissingPlatformProfile { profile: PlatformProfileClass },
    /// A per-OS profile lacked current proof.
    PlatformProofMissing { profile: PlatformProfileClass },
    /// A trust-sensitive action was missing its stricter-boundary block.
    TrustSensitiveActionMissingBoundary { surface: ShellAdjacentSurface },
    /// A non-action carrier carried a stricter-boundary block it must not.
    NonActionCarriesBoundary { surface: ShellAdjacentSurface },
    /// A stricter boundary named a different action than the surface.
    StricterBoundaryActionMismatch,
    /// The claim ceiling asserted explicit representation cues it cannot prove.
    OverclaimsRepresentationCues,
    /// The claim ceiling asserted surfaced findings it cannot prove.
    OverclaimsSuspiciousFindings,
    /// The claim ceiling asserted labeled copy/export it cannot prove.
    OverclaimsCopyExport,
    /// The claim ceiling asserted carrier survival it cannot prove.
    OverclaimsCueSurvival,
    /// The claim ceiling asserted a stricter boundary it cannot prove.
    OverclaimsStricterBoundary,
    /// The claim ceiling asserted complete accessibility cues it cannot prove.
    OverclaimsAccessibilityCues,
    /// The claim ceiling asserted complete per-OS conformance it cannot prove.
    OverclaimsPlatformConformance,
    /// A required recovery route was missing.
    MissingRecoveryRoute { action: SafePreviewRecoveryAction },
    /// A recovery route was not keyboard reachable.
    RecoveryRouteNotKeyboardReachable { action_id: String },
    /// A binding-surface projection was duplicated.
    DuplicateSurfaceProjection { surface: SafePreviewTruthSurface },
    /// A binding surface cloned prose rather than reading the shared record.
    SurfaceClonesProse { surface: SafePreviewTruthSurface },
    /// A required binding surface was missing.
    SurfaceProjectionMissing { surface: SafePreviewTruthSurface },
    /// A required entry-route surface was missing.
    RouteSurfaceMissing { surface: AttentionRouteSurface },
    /// An entry route was not keyboard reachable.
    RouteNotKeyboardReachable { surface: AttentionRouteSurface },
    /// An entry route did not activate the same posture.
    RouteTargetsDifferentItem { surface: AttentionRouteSurface },
    /// An entry-route surface was duplicated.
    DuplicateRouteSurface { surface: AttentionRouteSurface },
    /// A required accessibility layout mode was missing.
    AccessibilityLayoutModeMissing { mode: LayoutMode },
    /// An accessibility layout mode was unreachable or lost narration.
    AccessibilityLayoutModeUnreachable { mode: LayoutMode },
    /// The accessibility action labels did not match the recovery routes.
    AccessibilityActionLabelsMismatch,
    /// A row was hidden when no account was present.
    HiddenWithoutAccount,
    /// A row was hidden when managed services were absent.
    HiddenWithoutManagedServices,
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSentence { field } => {
                write!(f, "field {field} is not a reviewable sentence")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(f, "field {field} value {value:?} is not a canonical object ref")
            }
            Self::MissingUpstreamRef { field } => write!(f, "missing upstream ref for {field}"),
            Self::MissingCueCarrier { carrier } => {
                write!(f, "missing cue carrier {}", carrier.as_str())
            }
            Self::MissingPlatformProfile { profile } => {
                write!(f, "missing per-OS profile {}", profile.as_str())
            }
            Self::PlatformProofMissing { profile } => {
                write!(f, "per-OS profile {} lacks current proof", profile.as_str())
            }
            Self::TrustSensitiveActionMissingBoundary { surface } => write!(
                f,
                "trust-sensitive action {} lacks a stricter-boundary block",
                surface.as_str()
            ),
            Self::NonActionCarriesBoundary { surface } => write!(
                f,
                "non-action surface {} carries a stricter-boundary block",
                surface.as_str()
            ),
            Self::StricterBoundaryActionMismatch => {
                write!(f, "stricter boundary names a different action than the surface")
            }
            Self::OverclaimsRepresentationCues => {
                write!(f, "claims explicit representation cues not proven")
            }
            Self::OverclaimsSuspiciousFindings => {
                write!(f, "claims surfaced suspicious findings not proven")
            }
            Self::OverclaimsCopyExport => write!(f, "claims labeled copy/export not proven"),
            Self::OverclaimsCueSurvival => write!(f, "claims carrier cue survival not proven"),
            Self::OverclaimsStricterBoundary => {
                write!(f, "claims a stricter boundary before commit not proven")
            }
            Self::OverclaimsAccessibilityCues => {
                write!(f, "claims complete accessibility cues not proven")
            }
            Self::OverclaimsPlatformConformance => {
                write!(f, "claims complete per-OS conformance not proven")
            }
            Self::MissingRecoveryRoute { action } => {
                write!(f, "missing recovery route {}", action.as_str())
            }
            Self::RecoveryRouteNotKeyboardReachable { action_id } => {
                write!(f, "recovery route {action_id} is not keyboard reachable")
            }
            Self::DuplicateSurfaceProjection { surface } => {
                write!(f, "duplicate surface projection {}", surface.as_str())
            }
            Self::SurfaceClonesProse { surface } => {
                write!(f, "surface {} clones prose", surface.as_str())
            }
            Self::SurfaceProjectionMissing { surface } => {
                write!(f, "missing surface projection {}", surface.as_str())
            }
            Self::RouteSurfaceMissing { surface } => {
                write!(f, "missing entry-route surface {}", surface.as_str())
            }
            Self::RouteNotKeyboardReachable { surface } => {
                write!(f, "entry route {} is not keyboard reachable", surface.as_str())
            }
            Self::RouteTargetsDifferentItem { surface } => {
                write!(f, "entry route {} targets a different item", surface.as_str())
            }
            Self::DuplicateRouteSurface { surface } => {
                write!(f, "duplicate entry-route surface {}", surface.as_str())
            }
            Self::AccessibilityLayoutModeMissing { mode } => {
                write!(f, "missing accessibility layout mode {}", mode.as_str())
            }
            Self::AccessibilityLayoutModeUnreachable { mode } => {
                write!(f, "accessibility layout mode {} unreachable", mode.as_str())
            }
            Self::AccessibilityActionLabelsMismatch => {
                write!(f, "accessibility action labels do not match recovery routes")
            }
            Self::HiddenWithoutAccount => write!(f, "row hidden without an account"),
            Self::HiddenWithoutManagedServices => {
                write!(f, "row hidden without managed services")
            }
        }
    }
}

impl std::error::Error for BuildError {}

impl SafePreviewRecord {
    /// Builds a governed safe-preview record from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a record that lies
    /// about representation-cue explicitness, suspicious-finding survival,
    /// copy/export labeling, carrier survival, the stricter-boundary disclosure,
    /// accessibility cues, per-OS coverage, recovery routes, binding surfaces,
    /// route reachability, or accessibility. The stable claim class is *derived*
    /// from the evidence, so a posture can never publish a claim wider than its
    /// proof.
    pub fn build(input: SafePreviewInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        if !is_reviewable_sentence(&input.title) {
            return Err(BuildError::InvalidSentence { field: "title" });
        }
        if !is_reviewable_sentence(&input.summary) {
            return Err(BuildError::InvalidSentence { field: "summary" });
        }
        if !is_reviewable_sentence(&input.posture_label) {
            return Err(BuildError::InvalidSentence {
                field: "posture_label",
            });
        }
        require_canonical_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_canonical_ref("support_export_ref", &input.support_export_ref)?;
        for evidence in &input.evidence_refs {
            require_canonical_ref("evidence_refs", evidence)?;
        }
        for narrative in &input.narrative_refs {
            require_canonical_ref("narrative_refs", narrative)?;
        }
        require_present_ref(
            "upstream.content_safety_contract_ref",
            &input.upstream.content_safety_contract_ref,
        )?;

        // --- coverage: required cue carriers ---------------------------------
        let present_carriers: BTreeSet<CueCarrier> =
            input.cue_survival.iter().map(|row| row.carrier).collect();
        for required in CueCarrier::REQUIRED {
            if !present_carriers.contains(&required) {
                return Err(BuildError::MissingCueCarrier { carrier: required });
            }
        }

        // --- per-OS conformance: every profile present with current proof ----
        for required in PlatformProfileClass::REQUIRED {
            let row = input
                .platform_conformance
                .iter()
                .find(|row| row.profile == required)
                .ok_or(BuildError::MissingPlatformProfile { profile: required })?;
            if !row.covered || row.proof_ref.trim().is_empty() {
                return Err(BuildError::PlatformProofMissing { profile: required });
            }
        }
        let platform_conformance_complete = PlatformProfileClass::REQUIRED.iter().all(|profile| {
            input.platform_conformance.iter().any(|row| {
                row.profile == *profile && row.covered && !row.proof_ref.trim().is_empty()
            })
        });

        // --- stricter boundary: required iff trust-sensitive action ----------
        let is_trust_sensitive_action = input.surface_class.is_trust_sensitive_action();
        match (&input.stricter_boundary, is_trust_sensitive_action) {
            (None, true) => {
                return Err(BuildError::TrustSensitiveActionMissingBoundary {
                    surface: input.surface_class,
                });
            }
            (Some(_), false) => {
                return Err(BuildError::NonActionCarriesBoundary {
                    surface: input.surface_class,
                });
            }
            (Some(boundary), true) => {
                if boundary.action != input.surface_class {
                    return Err(BuildError::StricterBoundaryActionMismatch);
                }
            }
            (None, false) => {}
        }

        // --- derive the representation-cue verdict ---------------------------
        let copy_raw_present = input
            .representation_choices
            .iter()
            .any(|c| matches!(c.action_id, RepresentationActionId::CopyRaw));
        let copy_rendered_present = input
            .representation_choices
            .iter()
            .any(|c| matches!(c.action_id, RepresentationActionId::CopyRendered));
        let explicit_when_meaning_differs = input.representation.raw_reveal_available
            && input.representation.representation_label_present
            && copy_raw_present
            && (!input.renders_rich_content || copy_rendered_present);

        // --- derive the pillars from the evidence ----------------------------
        let representation_cues_explicit = explicit_when_meaning_differs;
        let suspicious_findings_surfaced =
            input.suspicious_findings.iter().all(SuspiciousFindingRow::holds);
        let has_exact_raw = input
            .representation_choices
            .iter()
            .any(RepresentationChoiceRow::is_exact_raw);
        let has_safe_inspection = input
            .representation_choices
            .iter()
            .any(RepresentationChoiceRow::is_safe_inspection);
        let all_choices_labeled = input
            .representation_choices
            .iter()
            .all(|c| is_reviewable_sentence(&c.label));
        let copy_export_labeled = !input.representation_choices.is_empty()
            && all_choices_labeled
            && has_exact_raw
            && has_safe_inspection;
        let cues_survive_all_carriers = CueCarrier::REQUIRED
            .iter()
            .all(|carrier| present_carriers.contains(carrier))
            && input.cue_survival.iter().all(CueCarrierRow::holds);
        let stricter_boundary_shown_before_commit = match &input.stricter_boundary {
            Some(boundary) => boundary.holds(),
            None => true,
        };
        let accessibility_cues_complete = input.a11y_cues.holds();
        let suspicious_content_present = !input.suspicious_findings.is_empty();

        // --- claim ceiling: never claim what the product cannot prove --------
        if input.claim_ceiling.asserts_representation_cues_explicit
            && !representation_cues_explicit
        {
            return Err(BuildError::OverclaimsRepresentationCues);
        }
        if input.claim_ceiling.asserts_suspicious_findings_surfaced
            && !suspicious_findings_surfaced
        {
            return Err(BuildError::OverclaimsSuspiciousFindings);
        }
        if input.claim_ceiling.asserts_copy_export_labeled && !copy_export_labeled {
            return Err(BuildError::OverclaimsCopyExport);
        }
        if input.claim_ceiling.asserts_cues_survive_all_carriers && !cues_survive_all_carriers {
            return Err(BuildError::OverclaimsCueSurvival);
        }
        if input.claim_ceiling.asserts_stricter_boundary_shown_before_commit
            && !stricter_boundary_shown_before_commit
        {
            return Err(BuildError::OverclaimsStricterBoundary);
        }
        if input.claim_ceiling.asserts_accessibility_cues_complete && !accessibility_cues_complete {
            return Err(BuildError::OverclaimsAccessibilityCues);
        }
        if input.claim_ceiling.asserts_platform_conformance_complete
            && !platform_conformance_complete
        {
            return Err(BuildError::OverclaimsPlatformConformance);
        }

        // --- recovery routes -------------------------------------------------
        let route_ids: Vec<&str> = input
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in SafePreviewRecoveryAction::REQUIRED {
            if !route_ids.iter().any(|id| *id == required.as_str()) {
                return Err(BuildError::MissingRecoveryRoute { action: required });
            }
        }
        for route in &input.recovery_routes {
            if !route.keyboard_reachable {
                return Err(BuildError::RecoveryRouteNotKeyboardReachable {
                    action_id: route.action_id.clone(),
                });
            }
        }

        // --- surface projections ---------------------------------------------
        let mut seen_surfaces: BTreeSet<SafePreviewTruthSurface> = BTreeSet::new();
        for projection in &input.surface_projections {
            if !seen_surfaces.insert(projection.surface) {
                return Err(BuildError::DuplicateSurfaceProjection {
                    surface: projection.surface,
                });
            }
            if !projection.reads_shared_record {
                return Err(BuildError::SurfaceClonesProse {
                    surface: projection.surface,
                });
            }
        }
        for required in SafePreviewTruthSurface::REQUIRED {
            if !seen_surfaces.contains(&required) {
                return Err(BuildError::SurfaceProjectionMissing { surface: required });
            }
        }
        let mut surface_projections: Vec<SafePreviewSurfaceProjection> = Vec::new();
        for required in SafePreviewTruthSurface::REQUIRED {
            let projection = input
                .surface_projections
                .iter()
                .find(|p| p.surface == required)
                .expect("surface presence checked above");
            surface_projections.push(SafePreviewSurfaceProjection {
                surface: required,
                surface_marker: projection.surface_marker,
                reads_shared_record: projection.reads_shared_record,
                summary_line: surface_summary_line(required, &input),
            });
        }
        let surface_lifecycle_marker = surface_projections
            .iter()
            .map(|projection| projection.surface_marker)
            .min()
            .unwrap_or(LifecycleMarker::Stable);

        // --- entry routes ----------------------------------------------------
        let mut seen_route_surfaces: Vec<AttentionRouteSurface> = Vec::new();
        for route in &input.routes {
            if seen_route_surfaces.contains(&route.surface) {
                return Err(BuildError::DuplicateRouteSurface {
                    surface: route.surface,
                });
            }
            seen_route_surfaces.push(route.surface);
            require_canonical_ref("routes.route_ref", &route.route_ref)?;
            if !route.keyboard_reachable {
                return Err(BuildError::RouteNotKeyboardReachable {
                    surface: route.surface,
                });
            }
            if !route.activates_same_item {
                return Err(BuildError::RouteTargetsDifferentItem {
                    surface: route.surface,
                });
            }
        }
        for required in AttentionRouteSurface::REQUIRED {
            if !seen_route_surfaces.contains(&required) {
                return Err(BuildError::RouteSurfaceMissing { surface: required });
            }
        }

        // --- accessibility ---------------------------------------------------
        if input.accessibility.action_labels.len() != input.recovery_routes.len() {
            return Err(BuildError::AccessibilityActionLabelsMismatch);
        }
        for (label, route) in input
            .accessibility
            .action_labels
            .iter()
            .zip(input.recovery_routes.iter())
        {
            if label != &route.action_label {
                return Err(BuildError::AccessibilityActionLabelsMismatch);
            }
        }
        for required in LayoutMode::REQUIRED {
            let Some(disclosure) = input
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
            else {
                return Err(BuildError::AccessibilityLayoutModeMissing { mode: required });
            };
            if !disclosure.row_narration_available || !disclosure.recovery_affordances_reachable {
                return Err(BuildError::AccessibilityLayoutModeUnreachable { mode: required });
            }
        }

        // --- availability ----------------------------------------------------
        if !input.available_without_account {
            return Err(BuildError::HiddenWithoutAccount);
        }
        if !input.available_without_managed_services {
            return Err(BuildError::HiddenWithoutManagedServices);
        }

        // --- pillars ---------------------------------------------------------
        let pillars = SafePreviewPillars {
            representation_cues_explicit,
            suspicious_findings_surfaced,
            copy_export_labeled,
            cues_survive_all_carriers,
            stricter_boundary_shown_before_commit,
            accessibility_cues_complete,
            platform_conformance_complete,
        };

        // --- normalise per-OS conformance + upstream refs --------------------
        let mut platform_conformance = input.platform_conformance;
        platform_conformance.sort_by_key(|row| row.profile);
        let mut contributing_case_refs = input.upstream.contributing_case_refs.clone();
        contributing_case_refs.sort();
        contributing_case_refs.dedup();

        // --- derive the stable-claim verdict ---------------------------------
        let mut narrowing_reasons = Vec::new();
        if !representation_cues_explicit {
            narrowing_reasons.push(SafePreviewNarrowingReason::RepresentationCuesNotExplicit);
        }
        if !suspicious_findings_surfaced {
            narrowing_reasons.push(SafePreviewNarrowingReason::SuspiciousFindingsFlattened);
        }
        if !copy_export_labeled {
            narrowing_reasons.push(SafePreviewNarrowingReason::CopyExportNotLabeled);
        }
        if !cues_survive_all_carriers {
            narrowing_reasons.push(SafePreviewNarrowingReason::CuesFlattenedOnCarrier);
        }
        if !stricter_boundary_shown_before_commit {
            narrowing_reasons.push(SafePreviewNarrowingReason::StricterBoundaryNotShownBeforeCommit);
        }
        if !accessibility_cues_complete {
            narrowing_reasons.push(SafePreviewNarrowingReason::AccessibilityCuesIncomplete);
        }
        if !platform_conformance_complete {
            narrowing_reasons.push(SafePreviewNarrowingReason::PlatformConformanceIncomplete);
        }
        if surface_lifecycle_marker.is_below_stable() {
            narrowing_reasons.push(SafePreviewNarrowingReason::SurfaceNotYetStable);
        }
        let qualifies_stable = narrowing_reasons.is_empty();
        let claim_class = if qualifies_stable {
            StableClaimClass::Stable
        } else if narrowing_reasons.len() == 1
            && narrowing_reasons[0] == SafePreviewNarrowingReason::SurfaceNotYetStable
        {
            match surface_lifecycle_marker {
                LifecycleMarker::Preview => StableClaimClass::Preview,
                _ => StableClaimClass::Beta,
            }
        } else {
            StableClaimClass::Beta
        };
        let stable_qualification = SafePreviewQualification {
            claim_class,
            qualifies_stable,
            narrowing_reasons,
        };
        let honesty_marker_present =
            !qualifies_stable || surface_lifecycle_marker.is_below_stable();

        Ok(Self {
            record_kind: SAFE_PREVIEW_RECORD_KIND.to_string(),
            schema_version: SAFE_PREVIEW_SCHEMA_VERSION,
            notice: SAFE_PREVIEW_NOTICE.to_string(),
            shared_contract_ref: SAFE_PREVIEW_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            posture_id: input.posture_id,
            posture_label: input.posture_label,
            title: input.title,
            summary: input.summary,
            surface_class: input.surface_class,
            surface_id_ref: input.surface_id_ref,
            is_trust_sensitive_action,
            renders_rich_content: input.renders_rich_content,
            surface_lifecycle_marker,
            trust_class: input.trust_class,
            detector_outcome: input.detector_outcome,
            suspicious_content_present,
            representation: RepresentationCues {
                raw_reveal_available: input.representation.raw_reveal_available,
                representation_label_present: input.representation.representation_label_present,
                representation_label: input.representation.representation_label,
                copy_raw_present,
                copy_rendered_present,
                explicit_when_meaning_differs,
            },
            suspicious_findings: input.suspicious_findings,
            representation_choices: input.representation_choices,
            cue_survival: input.cue_survival,
            stricter_boundary: input.stricter_boundary,
            a11y_cues: input.a11y_cues,
            platform_conformance,
            surface_projections,
            pillars,
            claim_ceiling: input.claim_ceiling,
            stable_qualification,
            recovery_routes: input.recovery_routes,
            routes: input.routes,
            accessibility: input.accessibility,
            available_without_account: input.available_without_account,
            available_without_managed_services: input.available_without_managed_services,
            honesty_marker_present,
            upstream: SafePreviewUpstream {
                content_safety_contract_ref: input.upstream.content_safety_contract_ref,
                trust_class_schema_version: input.upstream.trust_class_schema_version,
                representation_policy_schema_version: input
                    .upstream
                    .representation_policy_schema_version,
                contributing_case_refs,
            },
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("safe_preview: {}", self.record_id),
            format!("as_of: {}", self.as_of),
            format!("posture: {} ({})", self.posture_id, self.posture_label),
            format!(
                "surface: {} class={} trust_sensitive_action={} renders_rich={}",
                self.surface_id_ref,
                self.surface_class.as_str(),
                self.is_trust_sensitive_action,
                self.renders_rich_content
            ),
            format!(
                "surface_lifecycle_marker: {}",
                self.surface_lifecycle_marker.as_str()
            ),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            format!(
                "trust_class: {} detector_outcome: {} suspicious_content_present={}",
                self.trust_class.as_str(),
                self.detector_outcome.as_str(),
                self.suspicious_content_present
            ),
            format!(
                "stable_qualification: class={} qualifies_stable={} narrowing=[{}]",
                self.stable_qualification.claim_class.as_str(),
                self.stable_qualification.qualifies_stable,
                self.stable_qualification
                    .narrowing_reasons
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            format!(
                "pillars: representation_cues_explicit={} suspicious_findings_surfaced={} copy_export_labeled={} cues_survive_all_carriers={} stricter_boundary_before_commit={} a11y_cues={} platform_conformance={}",
                self.pillars.representation_cues_explicit,
                self.pillars.suspicious_findings_surfaced,
                self.pillars.copy_export_labeled,
                self.pillars.cues_survive_all_carriers,
                self.pillars.stricter_boundary_shown_before_commit,
                self.pillars.accessibility_cues_complete,
                self.pillars.platform_conformance_complete
            ),
            format!(
                "representation: raw_reveal_available={} label_present={} copy_raw={} copy_rendered={} explicit_when_meaning_differs={} :: {}",
                self.representation.raw_reveal_available,
                self.representation.representation_label_present,
                self.representation.copy_raw_present,
                self.representation.copy_rendered_present,
                self.representation.explicit_when_meaning_differs,
                self.representation.representation_label
            ),
            format!(
                "a11y_cues: warning_announced={} representation_label_announced={} trust_class_announced={} reveal_keyboard_reachable={} :: {}",
                self.a11y_cues.warning_announced_not_color_only,
                self.a11y_cues.representation_label_announced,
                self.a11y_cues.trust_class_announced,
                self.a11y_cues.reveal_affordance_keyboard_reachable,
                self.a11y_cues.warning_summary_label
            ),
        ];
        lines.push("suspicious_findings:".to_string());
        for finding in &self.suspicious_findings {
            lines.push(format!(
                "  - {} class={} impact={} raw_toggle={} escaped_copy={} reveal=[{}] :: {}",
                finding.finding_id,
                finding.content_class.as_str(),
                finding.visibility_impact,
                finding.raw_toggle_available,
                finding.escaped_copy_available,
                finding.reveal_affordances.join(", "),
                finding.representation_label
            ));
        }
        lines.push("representation_choices:".to_string());
        for choice in &self.representation_choices {
            lines.push(format!(
                "  - {} representation={} body={} raw_required={} active_removed={} :: {}",
                choice.action_id.as_str(),
                choice.representation_class.as_str(),
                choice.body_posture.as_str(),
                choice.raw_source_required,
                choice.active_content_removed,
                choice.label
            ));
        }
        lines.push("cue_survival:".to_string());
        for row in &self.cue_survival {
            lines.push(format!(
                "  - {} trust_class={} label={} warning={} no_flatten={} :: {}",
                row.carrier.as_str(),
                row.preserves_trust_class,
                row.preserves_representation_label,
                row.preserves_suspicious_warning,
                row.does_not_flatten_to_generic_preview,
                row.carried_summary
            ));
        }
        if let Some(boundary) = &self.stricter_boundary {
            lines.push(format!(
                "stricter_boundary: action={} enforces={} ordinary={} enforced={} before_commit={} commit_blocked={} :: {}",
                boundary.action.as_str(),
                boundary.enforces_stricter_preview_class,
                boundary.ordinary_browsing_class.as_str(),
                boundary.enforced_preview_class.as_str(),
                boundary.shows_boundary_before_commit,
                boundary.commit_blocked_until_acknowledged,
                boundary.boundary_disclosure
            ));
        }
        lines.push("platform_conformance:".to_string());
        for row in &self.platform_conformance {
            lines.push(format!(
                "  - {} profile_id={} covered={} behaviors=[{}] :: {}",
                row.profile.as_str(),
                row.profile_id,
                row.covered,
                row.named_behaviors.join(", "),
                row.proof_ref
            ));
        }
        lines.push("surface_projections:".to_string());
        for projection in &self.surface_projections {
            lines.push(format!(
                "  - {} marker={} reads_shared_record={} :: {}",
                projection.surface.as_str(),
                projection.surface_marker.as_str(),
                projection.reads_shared_record,
                projection.summary_line
            ));
        }
        lines.push(format!(
            "availability: without_account={} without_managed_services={}",
            self.available_without_account, self.available_without_managed_services
        ));
        lines.push(format!(
            "honesty_marker_present: {}",
            self.honesty_marker_present
        ));
        lines.push(format!(
            "diagnostics_export_ref: {}",
            self.diagnostics_export_ref
        ));
        lines.push(format!("support_export_ref: {}", self.support_export_ref));
        lines
    }
}

fn surface_summary_line(
    surface: SafePreviewTruthSurface,
    input: &SafePreviewInput,
) -> String {
    let prefix = match surface {
        SafePreviewTruthSurface::ShellSurface => "Shell surface",
        SafePreviewTruthSurface::ActivityCenter => "Activity center",
        SafePreviewTruthSurface::CliInspect => "CLI inspect",
        SafePreviewTruthSurface::HelpAbout => "Help/About",
        SafePreviewTruthSurface::SupportExport => "Support export",
    };
    let findings = input.suspicious_findings.len();
    format!(
        "{prefix}: {} — trust class {}, {} suspicious finding(s), representation label and Copy raw/rendered/escaped choices explicit.",
        input.surface_class.label(),
        input.trust_class.as_str(),
        findings,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trust_sensitive_actions_classified() {
        assert!(ShellAdjacentSurface::InstallReview.is_trust_sensitive_action());
        assert!(ShellAdjacentSurface::DeleteReview.is_trust_sensitive_action());
        assert!(ShellAdjacentSurface::OpenExternalHandoff.is_trust_sensitive_action());
        assert!(!ShellAdjacentSurface::Notification.is_trust_sensitive_action());
        assert!(!ShellAdjacentSurface::SupportExport.is_trust_sensitive_action());
    }

    #[test]
    fn finding_row_holds_when_revealable() {
        let row = SuspiciousFindingRow {
            finding_id: "finding:bidi:0".to_string(),
            content_class: SuspiciousContentClass::BidiControl,
            visibility_impact: "reorders_text".to_string(),
            representation_label: "Bidi control".to_string(),
            reveal_affordances: vec!["inline_marker".to_string()],
            raw_toggle_available: true,
            escaped_copy_available: true,
        };
        assert!(row.holds());
        let flattened = SuspiciousFindingRow {
            reveal_affordances: Vec::new(),
            ..row.clone()
        };
        assert!(!flattened.holds());
        let no_escape = SuspiciousFindingRow {
            escaped_copy_available: false,
            ..row
        };
        assert!(!no_escape.holds());
    }

    #[test]
    fn carrier_row_holds_when_nothing_flattened() {
        let row = CueCarrierRow {
            carrier: CueCarrier::Notification,
            preserves_trust_class: true,
            preserves_representation_label: true,
            preserves_suspicious_warning: true,
            does_not_flatten_to_generic_preview: true,
            carried_summary: "label".to_string(),
        };
        assert!(row.holds());
        let flattened = CueCarrierRow {
            does_not_flatten_to_generic_preview: false,
            ..row
        };
        assert!(!flattened.holds());
    }

    #[test]
    fn stricter_boundary_holds_when_shown_before_commit() {
        let boundary = StricterBoundary {
            action: ShellAdjacentSurface::InstallReview,
            enforces_stricter_preview_class: true,
            ordinary_browsing_class: TrustClass::SanitizedRich,
            enforced_preview_class: TrustClass::IsolatedRemoteActive,
            shows_boundary_before_commit: true,
            commit_blocked_until_acknowledged: true,
            boundary_disclosure: "stricter".to_string(),
        };
        assert!(boundary.holds());
        let not_shown = StricterBoundary {
            shows_boundary_before_commit: false,
            ..boundary
        };
        assert!(!not_shown.holds());
    }

    #[test]
    fn required_recovery_routes_expand_with_finding() {
        let base = required_recovery_routes(false);
        let ids: Vec<&str> = base.iter().map(|r| r.action_id.as_str()).collect();
        for required in SafePreviewRecoveryAction::REQUIRED {
            assert!(ids.contains(&required.as_str()));
        }
        assert!(!ids.contains(&"inspect_codepoints"));
        let full = required_recovery_routes(true);
        let ids: Vec<String> = full.iter().map(|r| r.action_id.clone()).collect();
        assert!(ids.iter().any(|id| id == "inspect_codepoints"));
    }
}
