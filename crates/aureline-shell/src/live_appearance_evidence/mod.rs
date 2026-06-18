//! Live OS appearance-change handling and exact-build evidence linkage for the
//! claimed M5 appearance rows.
//!
//! The technical-design and UI/UX specs treat a live OS theme, contrast, accent,
//! or text-scale change as governed runtime behavior, not a happy-path static
//! screenshot. A claimed appearance row is only truly qualified if it survives a
//! live OS change without corrupting layout, hiding state meaning, or producing
//! evidence nobody can attribute back to the build, theme package, and
//! appearance session that produced it.
//!
//! This module projects every live OS appearance change as one
//! [`LiveAppearanceEvidenceRow`] that binds a single platform-lab capture to the
//! exact build, theme package, appearance session, and OS signal that produced
//! it, and proves that the trust, severity, lifecycle, and focus cues survived
//! the transition:
//!
//! - the **platform** ([`DesktopPlatform`]) the lab ran on and the **OS signal**
//!   ([`OsAppearanceSignal`]) that drove the change, so a claim that only works
//!   on one platform cannot pass as cross-platform parity;
//! - the **changed appearance axis** ([`AppearanceAxis`]) and the **claimed M5
//!   appearance rows** ([`M5AppearanceRow`]) the change exercises, reusing the
//!   frozen appearance vocabulary instead of minting a parallel one;
//! - the **apply posture** ([`LiveApplyCapability`]): a change either applies
//!   live through the appearance-session model or carries an *explicit*
//!   restart-or-reload posture — the posture is disclosed up front, never
//!   discovered after a broken live update;
//! - the **exact-build evidence capture** ([`EvidenceCapture`]): a screenshot
//!   ref and golden-baseline ref whose [`CaptureAttribution`] names the build
//!   identity, theme package, appearance session, checkpoint, platform, and OS
//!   signal that produced them, so a release reviewer can always prove which
//!   build, package, and session generated a capture; and
//! - the **cue-preservation result** ([`CuePreservation`]): trust, severity,
//!   lifecycle, focus, state-semantics, and layout outcomes captured across the
//!   live change, so a live transition can never silently hide a high-salience
//!   cue.
//!
//! The records are inspectable, serde-serializable truth packets that carry no
//! raw screenshots, raw pixel data, raw paths, or raw user content — only opaque
//! capture refs, closed vocabulary, counts, and short labels. They are consumed
//! by the live release/evidence center, the headless inspector
//! (`aureline_shell_m5_live_appearance_evidence`), the support-export wrapper,
//! the docs page under `docs/m5/live-appearance-and-evidence-linkage.md`, the
//! published report under `artifacts/ux/m5/live-appearance-platform-labs/`, and
//! the boundary schema `schemas/ux/m5-live-appearance-evidence.schema.json`.
//!
//! The closed appearance vocabulary ([`AppearanceAxis`], [`LiveApplyCapability`],
//! [`AtomicityClass`], [`RollbackPathClass`], [`TransitionTrigger`],
//! [`M5AppearanceRow`], [`M5AppearanceSurfaceFamily`], [`M5SemanticSalience`],
//! [`M5StateSemantics`], [`M5FocusVisibility`], [`M5LayoutIntegrity`],
//! [`M5BoundaryCue`], [`M5QualificationStatus`], [`M5EvidenceFreshness`]) is
//! re-exported by reference from the already-frozen appearance-session and
//! appearance-parity contracts; this lane mints no parallel appearance values.
//! Only the live-change-specific vocabulary ([`OsAppearanceSignal`],
//! [`EvidenceCaptureKind`], [`GoldenMatchState`]) is new.
//!
//! The seeded projection is deterministic so the checked-in fixtures under
//! `fixtures/ux/m5/os-appearance-contrast-accent/` are bit-for-bit equal to the
//! output of [`seeded_live_appearance_evidence_report`]. The exact build a real
//! runtime would stamp into [`CaptureAttribution::build_identity_ref`] comes from
//! [`aureline_build_info::exact_build_identity_ref`]; the seed uses a frozen
//! representative ref so the fixtures stay reproducible, and
//! [`build_live_appearance_evidence_report`] lets a runtime mint the same report
//! against the live build identity.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::appearance_session::{
    AppearanceAxis, AtomicityClass, LiveApplyCapability, RollbackPathClass, TransitionTrigger,
};
use crate::m5_appearance_parity::{
    M5AppearanceRow, M5AppearanceSurfaceFamily, M5BoundaryCue, M5EvidenceFreshness,
    M5FocusVisibility, M5LayoutIntegrity, M5QualificationStatus, M5SemanticSalience,
    M5StateSemantics,
};
use crate::m5_native_desktop_qualification::DesktopPlatform;

#[cfg(test)]
mod tests;

/// Schema version exported with every record.
pub const M5_LIVE_APPEARANCE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface.
pub const M5_LIVE_APPEARANCE_SHARED_CONTRACT_REF: &str = "shell:m5_live_appearance_evidence:v1";

/// Stable record kind for [`LiveAppearanceEvidenceReport`] payloads.
pub const M5_LIVE_APPEARANCE_REPORT_RECORD_KIND: &str =
    "shell_m5_live_appearance_evidence_report_record";

/// Stable record kind for [`LiveAppearanceEvidenceRow`] payloads.
pub const M5_LIVE_APPEARANCE_ROW_RECORD_KIND: &str = "shell_m5_live_appearance_evidence_row_record";

/// Stable record kind for [`LiveAppearanceEvidenceSupportExport`] payloads.
pub const M5_LIVE_APPEARANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_live_appearance_evidence_support_export_record";

/// Stable report id used to pivot across surfaces.
pub const M5_LIVE_APPEARANCE_REPORT_ID: &str = "shell:m5_live_appearance_evidence:audit:v1";

/// Stable support-export id.
pub const M5_LIVE_APPEARANCE_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-live-appearance-evidence:001";

/// Repo-relative ref to the boundary schema this report conforms to.
pub const M5_LIVE_APPEARANCE_SOURCE_SCHEMA_REF: &str =
    "schemas/ux/m5-live-appearance-evidence.schema.json";

/// Published markdown artifact ref reviewers reopen the report from.
pub const M5_LIVE_APPEARANCE_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m5/live-appearance-platform-labs/m5_live_appearance_evidence.md";

/// Published companion doc ref.
pub const M5_LIVE_APPEARANCE_PUBLISHED_DOC_REF: &str =
    "docs/m5/live-appearance-and-evidence-linkage.md";

/// Deterministic generated-at value carried by the seeded report.
const GENERATED_AT: &str = "2026-06-17T00:00:00Z";

/// Frozen, representative exact-build identity ref used by the seed.
///
/// A live runtime stamps [`aureline_build_info::exact_build_identity_ref`] here;
/// the seed uses a fixed value so the checked-in fixtures stay reproducible. The
/// format mirrors the `build-id:aureline:<channel>:<version>:<target>:<profile>:<commit>`
/// shape produced by that function.
pub const SEED_BUILD_IDENTITY_REF: &str =
    "build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2";

/// Frozen, representative release-channel class used by the seed.
pub const SEED_RELEASE_CHANNEL_CLASS: &str = "stable";

/// The OS appearance signal that drove a live change.
///
/// A live appearance change is keyed by the platform-specific OS event, not by
/// the internal axis alone, so a Windows-only forced-colors signal and a macOS
/// contrast-increase signal both map onto the shared [`AppearanceAxis::Contrast`]
/// without pretending the platforms emit the same event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OsAppearanceSignal {
    /// The OS flipped the followed light/dark system theme.
    SystemThemeFlip,
    /// The OS raised contrast (e.g. macOS Increase Contrast).
    ContrastIncreased,
    /// The OS enabled forced-colors / high-contrast mode (e.g. Windows).
    ForcedColorsEnabled,
    /// The OS changed the accent color.
    AccentColorChanged,
    /// The OS increased the text scale / display scale.
    TextScaleIncreased,
    /// The OS enabled the reduce-motion preference.
    ReducedMotionEnabled,
}

impl OsAppearanceSignal {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemThemeFlip => "system_theme_flip",
            Self::ContrastIncreased => "contrast_increased",
            Self::ForcedColorsEnabled => "forced_colors_enabled",
            Self::AccentColorChanged => "accent_color_changed",
            Self::TextScaleIncreased => "text_scale_increased",
            Self::ReducedMotionEnabled => "reduced_motion_enabled",
        }
    }

    /// The appearance axis this OS signal changes.
    pub const fn canonical_axis(self) -> AppearanceAxis {
        match self {
            Self::SystemThemeFlip => AppearanceAxis::FollowSystem,
            Self::ContrastIncreased | Self::ForcedColorsEnabled => AppearanceAxis::Contrast,
            Self::AccentColorChanged => AppearanceAxis::Accent,
            Self::TextScaleIncreased => AppearanceAxis::TextScale,
            Self::ReducedMotionEnabled => AppearanceAxis::ReducedMotion,
        }
    }
}

/// Whether a capture was taken across the live change or as a steady baseline.
///
/// A qualified live-change claim needs evidence captured *through* the OS
/// transition. A steady-state baseline alone is accepted only for an explicitly
/// narrowed row that does not claim the live change worked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceCaptureKind {
    /// Captured across the live OS appearance transition (or its reload/restart).
    LiveTransition,
    /// A steady-state baseline capture, not taken across a live change.
    SteadyState,
}

impl EvidenceCaptureKind {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTransition => "live_transition",
            Self::SteadyState => "steady_state",
        }
    }

    /// `true` when the capture was taken across the live transition.
    pub const fn is_live_transition(self) -> bool {
        matches!(self, Self::LiveTransition)
    }
}

/// Match state of a golden capture against its baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoldenMatchState {
    /// The capture matches its golden baseline exactly.
    Matched,
    /// The capture differs within the accepted perceptual tolerance.
    DiffWithinTolerance,
    /// The capture does not match its golden baseline. Always a blocker on a
    /// qualified row.
    Mismatch,
    /// No golden baseline exists to attribute the capture against. Always a
    /// blocker on a qualified row.
    NoBaseline,
}

impl GoldenMatchState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Matched => "matched",
            Self::DiffWithinTolerance => "diff_within_tolerance",
            Self::Mismatch => "mismatch",
            Self::NoBaseline => "no_baseline",
        }
    }

    /// `true` when the golden capture is attributable to a matched baseline.
    pub const fn is_attributable(self) -> bool {
        matches!(self, Self::Matched | Self::DiffWithinTolerance)
    }
}

/// The build / package / session linkage that makes a capture attributable.
///
/// Every field is required so a release reviewer can always prove which build,
/// theme package, and appearance session produced a screenshot or golden
/// capture, on which platform, in response to which OS signal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureAttribution {
    /// Exact-build identity ref the capture was produced on.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// Active theme-package ref at capture time.
    pub theme_package_ref: String,
    /// Active theme-package revision ref at capture time.
    pub theme_revision_ref: String,
    /// Appearance-session ref that drove the change.
    pub appearance_session_ref: String,
    /// Checkpoint ref the change was applied from.
    pub checkpoint_ref: String,
    /// Platform the lab ran on.
    pub platform: DesktopPlatform,
    /// OS signal that drove the change.
    pub os_signal: OsAppearanceSignal,
    /// Deterministic capture timestamp.
    pub captured_at: String,
}

/// An exact-build screenshot / golden capture for one live appearance change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceCapture {
    /// Stable opaque capture id.
    pub capture_id: String,
    /// Whether the capture was taken across the live transition.
    pub capture_kind: EvidenceCaptureKind,
    /// Opaque screenshot ref (no raw pixels, no path).
    pub screenshot_ref: String,
    /// Opaque content digest of the screenshot.
    pub screenshot_digest: String,
    /// Opaque golden-baseline ref the capture is attributed against.
    pub golden_baseline_ref: String,
    /// Golden match state.
    pub golden_match: GoldenMatchState,
    /// Freshness of the capture.
    pub freshness: M5EvidenceFreshness,
    /// Build / package / session linkage that makes the capture attributable.
    pub attribution: CaptureAttribution,
}

impl EvidenceCapture {
    /// `true` when the capture carries non-empty screenshot and golden refs.
    pub fn has_refs(&self) -> bool {
        !self.screenshot_ref.trim().is_empty() && !self.golden_baseline_ref.trim().is_empty()
    }
}

/// Trust, severity, lifecycle, focus, state-semantics, and layout outcomes
/// captured across a live appearance change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CuePreservation {
    /// Trust / identity cue outcome.
    pub trust_cue: M5BoundaryCue,
    /// Severity / risk cue outcome.
    pub severity_cue: M5BoundaryCue,
    /// Lifecycle (preview / stale / pending) cue outcome.
    pub lifecycle_cue: M5BoundaryCue,
    /// Focus-visibility outcome.
    pub focus_cue: M5FocusVisibility,
    /// State-semantics outcome.
    pub state_semantics: M5StateSemantics,
    /// Layout-integrity outcome.
    pub layout_integrity: M5LayoutIntegrity,
}

impl CuePreservation {
    /// `true` when the focus ring, state semantics, and layout all survive.
    pub const fn structurally_intact(self) -> bool {
        matches!(self.focus_cue, M5FocusVisibility::VisibleFocusRing)
            && matches!(self.state_semantics, M5StateSemantics::Preserved)
            && matches!(self.layout_integrity, M5LayoutIntegrity::Intact)
    }
}

/// One live OS appearance change, exercised on one platform, with one
/// attributable exact-build evidence capture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveAppearanceEvidenceRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable row id quoted across surfaces.
    pub row_id: String,
    /// Reviewer-facing title for the row.
    pub title: String,
    /// Platform the lab ran on.
    pub platform: DesktopPlatform,
    /// OS signal that drove the change.
    pub os_signal: OsAppearanceSignal,
    /// Appearance axis the change touched. MUST equal the OS signal's axis.
    pub changed_axis: AppearanceAxis,
    /// Claimed M5 appearance rows this change exercises (empty for accent and
    /// text-scale, which qualify no dedicated claimed row).
    pub qualifies_rows: Vec<M5AppearanceRow>,
    /// Surface family the change was exercised on.
    pub surface_family: M5AppearanceSurfaceFamily,
    /// Semantic salience of the exercised surface.
    pub semantic_salience: M5SemanticSalience,
    /// Trigger that drove the transition. MUST be an OS signal.
    pub transition_trigger: TransitionTrigger,
    /// Short before-value label (e.g. `light`).
    pub from_value: String,
    /// Short after-value label (e.g. `dark`).
    pub to_value: String,
    /// How the change applies: live, or with an explicit reload/restart, or with
    /// an unavailable platform signal.
    pub apply_posture: LiveApplyCapability,
    /// `true` when a reload / restart requirement is disclosed up front. MUST be
    /// `true` whenever the posture needs a reload or restart.
    pub restart_or_reload_disclosed: bool,
    /// Atomicity class of the applied change.
    pub atomicity_class: AtomicityClass,
    /// Rollback path class that reverses the change.
    pub rollback_path_class: RollbackPathClass,
    /// Active theme-package ref. MUST match the capture attribution.
    pub theme_package_ref: String,
    /// Active theme-package revision ref. MUST match the capture attribution.
    pub theme_revision_ref: String,
    /// Appearance-session ref. MUST match the capture attribution.
    pub appearance_session_ref: String,
    /// Checkpoint ref. MUST match the capture attribution.
    pub checkpoint_ref: String,
    /// Qualification status the lab reports for this change.
    pub qualification_status: M5QualificationStatus,
    /// Required whenever the status is narrowed / omitted / a declared gap.
    pub narrowing_reason: Option<String>,
    /// Cue-preservation result. Required for a qualified row.
    pub cue_preservation: Option<CuePreservation>,
    /// Exact-build evidence capture. Required for a qualified row.
    pub evidence: Option<EvidenceCapture>,
    /// Docs/help refs that publish the row.
    pub docs_help_refs: Vec<String>,
    /// Reviewer-facing narrative summary.
    pub narrative: String,
}

impl LiveAppearanceEvidenceRow {
    /// `true` when the row claims the live change qualified (projects evidence).
    pub fn is_qualified(&self) -> bool {
        self.qualification_status.projects_evidence()
    }

    /// `true` when this row is marketed: a qualified change counted toward
    /// cross-platform coverage.
    pub fn is_marketed(&self) -> bool {
        self.is_qualified()
    }

    /// `true` when the apply posture needs an explicit reload / restart.
    pub fn posture_needs_reload_or_restart(&self) -> bool {
        matches!(
            self.apply_posture,
            LiveApplyCapability::RequiresSurfaceReload | LiveApplyCapability::RequiresAppRestart
        )
    }

    /// Returns deterministic compact lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let golden = self
            .evidence
            .as_ref()
            .map(|e| e.golden_match.as_str())
            .unwrap_or("none");
        let capture = self
            .evidence
            .as_ref()
            .map(|e| e.capture_kind.as_str())
            .unwrap_or("none");
        let mut lines = vec![
            format!(
                "{} [{}/{}]",
                self.title,
                self.platform.as_str(),
                self.os_signal.as_str()
            ),
            format!(
                "  axis={} status={} posture={} restart_or_reload_disclosed={}",
                self.changed_axis.as_str(),
                self.qualification_status.as_str(),
                self.apply_posture.as_str(),
                self.restart_or_reload_disclosed
            ),
            format!(
                "  surface={} salience={} {}->{}",
                self.surface_family.as_str(),
                self.semantic_salience.as_str(),
                self.from_value,
                self.to_value
            ),
            format!("  evidence capture={capture} golden={golden}"),
            format!(
                "  session={} checkpoint={} package={}",
                self.appearance_session_ref, self.checkpoint_ref, self.theme_package_ref
            ),
        ];
        if let Some(reason) = &self.narrowing_reason {
            lines.push(format!("  narrowing_reason: {reason}"));
        }
        lines
    }

    /// Computes the blocking findings for this row against the report build ref.
    fn compute_findings(&self, report_build_ref: &str) -> Vec<LiveAppearanceBlockingFinding> {
        let mut findings = Vec::new();
        let row_id = self.row_id.clone();

        // The OS signal and the declared axis must agree.
        if self.changed_axis != self.os_signal.canonical_axis() {
            findings.push(LiveAppearanceBlockingFinding::AxisSignalMismatch {
                row_id: row_id.clone(),
            });
        }

        // A reload / restart posture must be disclosed up front.
        if self.posture_needs_reload_or_restart() && !self.restart_or_reload_disclosed {
            findings.push(
                LiveAppearanceBlockingFinding::RestartReloadPostureUndisclosed {
                    row_id: row_id.clone(),
                },
            );
        }

        // Ad-hoc local styling outside the appearance-session model is never
        // acceptable.
        if matches!(
            self.qualification_status,
            M5QualificationStatus::UnqualifiedLocalAppearance
        ) {
            findings.push(LiveAppearanceBlockingFinding::UnqualifiedLocalAppearance {
                row_id: row_id.clone(),
            });
        }

        // Narrowed statuses must carry a reason.
        if self.qualification_status.requires_narrowing_reason()
            && self
                .narrowing_reason
                .as_deref()
                .map(str::trim)
                .unwrap_or("")
                .is_empty()
        {
            findings.push(LiveAppearanceBlockingFinding::MissingNarrowingReason {
                row_id: row_id.clone(),
            });
        }

        if self.is_qualified() {
            self.compute_qualified_findings(report_build_ref, &row_id, &mut findings);
        }

        findings
    }

    fn compute_qualified_findings(
        &self,
        report_build_ref: &str,
        row_id: &str,
        findings: &mut Vec<LiveAppearanceBlockingFinding>,
    ) {
        // A qualified row must carry an evidence capture with refs.
        let Some(evidence) = self.evidence.as_ref() else {
            findings.push(LiveAppearanceBlockingFinding::MissingEvidence {
                row_id: row_id.to_owned(),
            });
            self.compute_cue_findings(row_id, findings);
            return;
        };
        if !evidence.has_refs() {
            findings.push(LiveAppearanceBlockingFinding::MissingEvidence {
                row_id: row_id.to_owned(),
            });
        }

        // The capture must be attributable to this build.
        if evidence.attribution.build_identity_ref.trim().is_empty()
            || evidence.attribution.release_channel_class.trim().is_empty()
            || evidence.attribution.build_identity_ref != report_build_ref
        {
            findings.push(LiveAppearanceBlockingFinding::BuildAttributionMismatch {
                row_id: row_id.to_owned(),
            });
        }

        // The capture must be attributable to this row's package / session /
        // checkpoint / platform / signal.
        let attribution_field = self.attribution_mismatch_field(&evidence.attribution);
        if let Some(field) = attribution_field {
            findings.push(LiveAppearanceBlockingFinding::EvidenceAttributionMismatch {
                row_id: row_id.to_owned(),
                field: field.to_owned(),
            });
        }

        // The golden capture must be attributable to a matched baseline.
        if !evidence.golden_match.is_attributable() {
            findings.push(LiveAppearanceBlockingFinding::GoldenNotAttributable {
                row_id: row_id.to_owned(),
            });
        }

        // Marketed evidence must be fresh.
        if matches!(evidence.freshness, M5EvidenceFreshness::Stale) {
            findings.push(LiveAppearanceBlockingFinding::StaleEvidence {
                row_id: row_id.to_owned(),
            });
        }

        // A change that applies live must be proven with a live-transition
        // capture, not a happy-path static screenshot.
        if self.apply_posture.applies_live() && !evidence.capture_kind.is_live_transition() {
            findings.push(LiveAppearanceBlockingFinding::StaticEvidenceOnly {
                row_id: row_id.to_owned(),
            });
        }

        self.compute_cue_findings(row_id, findings);
    }

    fn compute_cue_findings(
        &self,
        row_id: &str,
        findings: &mut Vec<LiveAppearanceBlockingFinding>,
    ) {
        let Some(cues) = self.cue_preservation else {
            findings.push(LiveAppearanceBlockingFinding::CueHidden {
                row_id: row_id.to_owned(),
                cue: "all".to_owned(),
            });
            return;
        };

        for (cue_value, salience, token) in [
            (cues.trust_cue, M5SemanticSalience::TrustBearing, "trust"),
            (
                cues.severity_cue,
                M5SemanticSalience::SeverityBearing,
                "severity",
            ),
            (
                cues.lifecycle_cue,
                M5SemanticSalience::LifecycleBearing,
                "lifecycle",
            ),
        ] {
            let hidden = matches!(cue_value, M5BoundaryCue::Hidden);
            // A surface whose salience carries this meaning must present the cue;
            // it may not fall back to "not applicable" under a live change.
            let salience_demands_cue =
                self.semantic_salience == salience && !matches!(cue_value, M5BoundaryCue::Present);
            if hidden || salience_demands_cue {
                findings.push(LiveAppearanceBlockingFinding::CueHidden {
                    row_id: row_id.to_owned(),
                    cue: token.to_owned(),
                });
            }
        }

        if matches!(cues.focus_cue, M5FocusVisibility::NotVisible) {
            findings.push(LiveAppearanceBlockingFinding::FocusNotVisible {
                row_id: row_id.to_owned(),
            });
        }
        if matches!(cues.state_semantics, M5StateSemantics::Lost) {
            findings.push(LiveAppearanceBlockingFinding::StateSemanticsLost {
                row_id: row_id.to_owned(),
            });
        }
        if matches!(cues.layout_integrity, M5LayoutIntegrity::Corrupted) {
            findings.push(LiveAppearanceBlockingFinding::LayoutCorrupted {
                row_id: row_id.to_owned(),
            });
        }
    }

    fn attribution_mismatch_field(&self, attribution: &CaptureAttribution) -> Option<&'static str> {
        if attribution.theme_package_ref != self.theme_package_ref {
            Some("theme_package_ref")
        } else if attribution.theme_revision_ref != self.theme_revision_ref {
            Some("theme_revision_ref")
        } else if attribution.appearance_session_ref != self.appearance_session_ref {
            Some("appearance_session_ref")
        } else if attribution.checkpoint_ref != self.checkpoint_ref {
            Some("checkpoint_ref")
        } else if attribution.platform != self.platform {
            Some("platform")
        } else if attribution.os_signal != self.os_signal {
            Some("os_signal")
        } else {
            None
        }
    }
}

/// Per-scope blocking-finding summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LiveAppearanceFindingSummary {
    /// Blocking findings owned by a row.
    pub row_findings: usize,
    /// Blocking findings owned by report-level coverage.
    pub coverage_findings: usize,
    /// Total blocking findings.
    pub total_blocking_findings: usize,
}

impl LiveAppearanceFindingSummary {
    fn record(&mut self, finding: &LiveAppearanceBlockingFinding) {
        match finding {
            LiveAppearanceBlockingFinding::SinglePlatformClaim { .. }
            | LiveAppearanceBlockingFinding::SurfaceFamilyUncovered { .. } => {
                self.coverage_findings += 1;
            }
            _ => self.row_findings += 1,
        }
        self.total_blocking_findings += 1;
    }
}

/// A blocking finding the live-appearance evidence audit refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum LiveAppearanceBlockingFinding {
    /// The declared axis does not match the OS signal's axis.
    AxisSignalMismatch {
        /// Owning row id.
        row_id: String,
    },
    /// A reload / restart posture is not disclosed up front.
    RestartReloadPostureUndisclosed {
        /// Owning row id.
        row_id: String,
    },
    /// The change is rendered through ad-hoc styling outside the
    /// appearance-session model.
    UnqualifiedLocalAppearance {
        /// Owning row id.
        row_id: String,
    },
    /// A narrowed / omitted / declared-gap row carries no reason.
    MissingNarrowingReason {
        /// Owning row id.
        row_id: String,
    },
    /// A qualified row carries no attributable evidence capture.
    MissingEvidence {
        /// Owning row id.
        row_id: String,
    },
    /// A capture's build identity does not match the report's exact build.
    BuildAttributionMismatch {
        /// Owning row id.
        row_id: String,
    },
    /// A capture's attribution does not match the row's package / session /
    /// checkpoint / platform / signal.
    EvidenceAttributionMismatch {
        /// Owning row id.
        row_id: String,
        /// The first attribution field that diverged.
        field: String,
    },
    /// A qualified row's golden capture is a mismatch or has no baseline.
    GoldenNotAttributable {
        /// Owning row id.
        row_id: String,
    },
    /// A marketed qualified row carries stale evidence.
    StaleEvidence {
        /// Owning row id.
        row_id: String,
    },
    /// A live-applying change is proven only by a steady-state static capture.
    StaticEvidenceOnly {
        /// Owning row id.
        row_id: String,
    },
    /// A trust / severity / lifecycle cue is hidden under the live change.
    CueHidden {
        /// Owning row id.
        row_id: String,
        /// The hidden cue token.
        cue: String,
    },
    /// The focus ring is not visible under the live change.
    FocusNotVisible {
        /// Owning row id.
        row_id: String,
    },
    /// State meaning is lost under the live change.
    StateSemanticsLost {
        /// Owning row id.
        row_id: String,
    },
    /// Layout is corrupted by the live change.
    LayoutCorrupted {
        /// Owning row id.
        row_id: String,
    },
    /// A marketed appearance axis is proven on only one platform.
    SinglePlatformClaim {
        /// The axis proven on a single platform.
        axis: String,
    },
    /// A required surface family is exercised by no qualified row.
    SurfaceFamilyUncovered {
        /// The uncovered surface family token.
        surface_family: String,
    },
}

impl LiveAppearanceBlockingFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::AxisSignalMismatch { .. } => "axis_signal_mismatch",
            Self::RestartReloadPostureUndisclosed { .. } => "restart_reload_posture_undisclosed",
            Self::UnqualifiedLocalAppearance { .. } => "unqualified_local_appearance",
            Self::MissingNarrowingReason { .. } => "missing_narrowing_reason",
            Self::MissingEvidence { .. } => "missing_evidence",
            Self::BuildAttributionMismatch { .. } => "build_attribution_mismatch",
            Self::EvidenceAttributionMismatch { .. } => "evidence_attribution_mismatch",
            Self::GoldenNotAttributable { .. } => "golden_not_attributable",
            Self::StaleEvidence { .. } => "stale_evidence",
            Self::StaticEvidenceOnly { .. } => "static_evidence_only",
            Self::CueHidden { .. } => "cue_hidden",
            Self::FocusNotVisible { .. } => "focus_not_visible",
            Self::StateSemanticsLost { .. } => "state_semantics_lost",
            Self::LayoutCorrupted { .. } => "layout_corrupted",
            Self::SinglePlatformClaim { .. } => "single_platform_claim",
            Self::SurfaceFamilyUncovered { .. } => "surface_family_uncovered",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::AxisSignalMismatch { row_id }
            | Self::RestartReloadPostureUndisclosed { row_id }
            | Self::UnqualifiedLocalAppearance { row_id }
            | Self::MissingNarrowingReason { row_id }
            | Self::MissingEvidence { row_id }
            | Self::BuildAttributionMismatch { row_id }
            | Self::EvidenceAttributionMismatch { row_id, .. }
            | Self::GoldenNotAttributable { row_id }
            | Self::StaleEvidence { row_id }
            | Self::StaticEvidenceOnly { row_id }
            | Self::CueHidden { row_id, .. }
            | Self::FocusNotVisible { row_id }
            | Self::StateSemanticsLost { row_id }
            | Self::LayoutCorrupted { row_id } => row_id,
            Self::SinglePlatformClaim { axis } => axis,
            Self::SurfaceFamilyUncovered { surface_family } => surface_family,
        }
    }
}

/// The surface families that a live OS appearance change must be proven not to
/// corrupt: notebooks, data grids, profiler / trace views, preview panes,
/// docs/help, and companion-adjacent surfaces.
pub const REQUIRED_SURFACE_FAMILIES: [M5AppearanceSurfaceFamily; 7] = [
    M5AppearanceSurfaceFamily::NotebookCellChrome,
    M5AppearanceSurfaceFamily::ResultGridRow,
    M5AppearanceSurfaceFamily::ProfilerPanel,
    M5AppearanceSurfaceFamily::TracePanel,
    M5AppearanceSurfaceFamily::PreviewRouteBadge,
    M5AppearanceSurfaceFamily::DocsBrowserPane,
    M5AppearanceSurfaceFamily::CompanionSurface,
];

/// Per-axis platform coverage for a marketed appearance axis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AxisPlatformCoverage {
    /// The covered appearance axis.
    pub axis: AppearanceAxis,
    /// Platforms the axis is proven on, in canonical order.
    pub platforms: Vec<DesktopPlatform>,
}

fn axis_platform_coverage(rows: &[LiveAppearanceEvidenceRow]) -> Vec<AxisPlatformCoverage> {
    let mut order: Vec<AppearanceAxis> = Vec::new();
    for row in rows {
        if row.is_marketed() && !order.contains(&row.changed_axis) {
            order.push(row.changed_axis);
        }
    }
    order
        .into_iter()
        .map(|axis| {
            let mut platforms: Vec<DesktopPlatform> = Vec::new();
            for row in rows {
                if row.is_marketed()
                    && row.changed_axis == axis
                    && !platforms.contains(&row.platform)
                {
                    platforms.push(row.platform);
                }
            }
            platforms.sort_by_key(|p| p.as_str());
            AxisPlatformCoverage { axis, platforms }
        })
        .collect()
}

fn covered_surface_families(rows: &[LiveAppearanceEvidenceRow]) -> BTreeSet<&'static str> {
    rows.iter()
        .filter(|row| row.is_qualified())
        .map(|row| row.surface_family.as_str())
        .collect()
}

fn compute_coverage_findings(
    rows: &[LiveAppearanceEvidenceRow],
) -> Vec<LiveAppearanceBlockingFinding> {
    let mut findings = Vec::new();

    // No marketed appearance axis may be proven on only one platform.
    for coverage in axis_platform_coverage(rows) {
        if coverage.platforms.len() < 2 {
            findings.push(LiveAppearanceBlockingFinding::SinglePlatformClaim {
                axis: coverage.axis.as_str().to_owned(),
            });
        }
    }

    // Every required surface family must be exercised by some qualified row.
    let covered = covered_surface_families(rows);
    for family in REQUIRED_SURFACE_FAMILIES {
        if !covered.contains(family.as_str()) {
            findings.push(LiveAppearanceBlockingFinding::SurfaceFamilyUncovered {
                surface_family: family.as_str().to_owned(),
            });
        }
    }

    findings
}

/// The live-appearance evidence-linkage report, shared by the release/evidence
/// center, the support-export wrapper, and the docs/help surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveAppearanceEvidenceReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the report.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable report id used to pivot across surfaces.
    pub report_id: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// Exact-build identity ref the report was generated against. Every capture
    /// attribution must match this.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// Live OS appearance-change rows in canonical order.
    pub rows: Vec<LiveAppearanceEvidenceRow>,
    /// Per-axis platform coverage for marketed axes.
    pub axis_platform_coverage: Vec<AxisPlatformCoverage>,
    /// Surface families exercised by qualified rows, in canonical order.
    pub covered_surface_families: Vec<String>,
    /// OS signals exercised, in first-seen order.
    pub os_signal_coverage: Vec<OsAppearanceSignal>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of marketed (qualified) rows.
    pub marketed_row_count: usize,
    /// Number of rows that need a reload or restart for the change.
    pub restart_or_reload_row_count: usize,
    /// `true` when at least one row proves a live transition.
    pub live_change_demonstrated: bool,
    /// `true` when every capture is attributable to the report's build.
    pub all_captures_build_attributed: bool,
    /// Per-scope blocking-finding summary.
    pub findings_summary: LiveAppearanceFindingSummary,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<LiveAppearanceBlockingFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Release / evidence-center refs that route the report.
    pub release_evidence_refs: Vec<String>,
    /// Extension-inspection refs that consume the report.
    pub extension_inspection_refs: Vec<String>,
    /// Sync / import refs that preserve attribution.
    pub sync_refs: Vec<String>,
    /// Docs/help refs the report reopens from.
    pub docs_help_refs: Vec<String>,
    /// Support / export refs that preserve the report.
    pub support_export_refs: Vec<String>,
    /// Published markdown artifact ref.
    pub published_report_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl LiveAppearanceEvidenceReport {
    /// Returns the row registered under `row_id`, if any.
    pub fn row(&self, row_id: &str) -> Option<&LiveAppearanceEvidenceRow> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "report: id={}, rows={}, marketed={}, restart_or_reload={}, clean={}",
            self.report_id,
            self.row_count,
            self.marketed_row_count,
            self.restart_or_reload_row_count,
            self.report_clean,
        ));
        lines.push(format!(
            "build={} channel={} live_change_demonstrated={} all_captures_build_attributed={}",
            self.build_identity_ref,
            self.release_channel_class,
            self.live_change_demonstrated,
            self.all_captures_build_attributed,
        ));
        for coverage in &self.axis_platform_coverage {
            lines.push(format!(
                "axis: {} platforms=[{}]",
                coverage.axis.as_str(),
                coverage
                    .platforms
                    .iter()
                    .map(|p| p.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }
        for row in &self.rows {
            lines.extend(row.compact_lines());
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref()
            ));
        }
        lines
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 live-appearance change & evidence-linkage report\n\n");
        out.push_str(
            "Generated from the seeded report in\n\
             [`crate::live_appearance_evidence`](../../../../crates/aureline-shell/src/live_appearance_evidence/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- markdown > \\\n  artifacts/ux/m5/live-appearance-platform-labs/m5_live_appearance_evidence.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!(
            "- Release channel: `{}`\n",
            self.release_channel_class
        ));
        out.push_str(&format!("- Rows: {}\n", self.row_count));
        out.push_str(&format!("- Marketed rows: {}\n", self.marketed_row_count));
        out.push_str(&format!(
            "- Rows needing reload/restart: {}\n",
            self.restart_or_reload_row_count
        ));
        out.push_str(&format!(
            "- Live change demonstrated: `{}`\n",
            self.live_change_demonstrated
        ));
        out.push_str(&format!(
            "- All captures build-attributed: `{}`\n",
            self.all_captures_build_attributed
        ));
        out.push_str(&format!(
            "- Blocking findings: {}\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Live OS appearance changes\n\n");
        out.push_str(
            "| Platform | OS signal | Axis | Qualifies | Posture | Capture | Golden | Status |\n\
             | -------- | --------- | ---- | --------- | ------- | ------- | ------ | ------ |\n",
        );
        for row in &self.rows {
            let qualifies = if row.qualifies_rows.is_empty() {
                "—".to_owned()
            } else {
                row.qualifies_rows
                    .iter()
                    .map(|r| r.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let capture = row
                .evidence
                .as_ref()
                .map(|e| e.capture_kind.as_str())
                .unwrap_or("—");
            let golden = row
                .evidence
                .as_ref()
                .map(|e| e.golden_match.as_str())
                .unwrap_or("—");
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} | `{}` | `{}` | `{}` | `{}` |\n",
                row.platform.as_str(),
                row.os_signal.as_str(),
                row.changed_axis.as_str(),
                qualifies,
                row.apply_posture.as_str(),
                capture,
                golden,
                row.qualification_status.as_str(),
            ));
        }
        out.push('\n');

        out.push_str("## Evidence attribution\n\n");
        out.push_str(
            "| Row | Build | Theme package | Session | Checkpoint |\n\
             | --- | ----- | ------------- | ------- | ---------- |\n",
        );
        for row in &self.rows {
            let build = row
                .evidence
                .as_ref()
                .map(|e| e.attribution.build_identity_ref.as_str())
                .unwrap_or("—");
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                row.row_id,
                build,
                row.theme_package_ref,
                row.appearance_session_ref,
                row.checkpoint_ref,
            ));
        }
        out.push('\n');

        out.push_str("## Cross-platform axis coverage\n\n");
        out.push_str("| Axis | Platforms |\n| ---- | --------- |\n");
        for coverage in &self.axis_platform_coverage {
            out.push_str(&format!(
                "| `{}` | {} |\n",
                coverage.axis.as_str(),
                coverage
                    .platforms
                    .iter()
                    .map(|p| format!("`{}`", p.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out.push('\n');

        out.push_str("## Surface coverage\n\n");
        out.push_str(&format!(
            "Qualified rows exercise: {}.\n\n",
            self.covered_surface_families
                .iter()
                .map(|s| format!("`{s}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));

        out.push_str("## Findings\n\n");
        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_live_appearance_evidence_fixtures\n");
        out.push_str("python3 tools/ci/m5/live_appearance_evidence_check.py --repo-root .\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the live-appearance evidence report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveAppearanceEvidenceSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Report quoted in full.
    pub report: LiveAppearanceEvidenceReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl LiveAppearanceEvidenceSupportExport {
    /// Builds the support-export wrapper for a report.
    ///
    /// Every report id, the exact-build ref, each row id, appearance-session
    /// ref, checkpoint ref, theme-package ref, screenshot ref, and golden ref is
    /// quoted as a case id so a support reviewer — or a release-evidence pack —
    /// can name the same capture, build, package, and session the runtime used.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: LiveAppearanceEvidenceReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone(), report.build_identity_ref.clone()];
        for row in &report.rows {
            case_ids.push(row.row_id.clone());
            case_ids.push(row.appearance_session_ref.clone());
            case_ids.push(row.checkpoint_ref.clone());
            case_ids.push(row.theme_package_ref.clone());
            if let Some(evidence) = &row.evidence {
                case_ids.push(evidence.capture_id.clone());
                case_ids.push(evidence.screenshot_ref.clone());
                case_ids.push(evidence.golden_baseline_ref.clone());
            }
        }
        Self {
            record_kind: M5_LIVE_APPEARANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_LIVE_APPEARANCE_SCHEMA_VERSION,
            shared_contract_ref: M5_LIVE_APPEARANCE_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Builds a [`LiveAppearanceEvidenceReport`] from the exact build identity and
/// the live OS appearance-change rows.
///
/// A live runtime passes [`aureline_build_info::exact_build_identity_ref`] and
/// [`aureline_build_info::release_channel_class`] so the report and every
/// capture attribution are stamped with the build that produced them. Coverage
/// summaries and blocking findings are recomputed here so the report is the
/// single source of truth.
pub fn build_live_appearance_evidence_report(
    build_identity_ref: impl Into<String>,
    release_channel_class: impl Into<String>,
    rows: Vec<LiveAppearanceEvidenceRow>,
) -> LiveAppearanceEvidenceReport {
    let build_identity_ref = build_identity_ref.into();
    let release_channel_class = release_channel_class.into();

    let mut findings_summary = LiveAppearanceFindingSummary::default();
    let mut blocking_findings: Vec<LiveAppearanceBlockingFinding> = Vec::new();
    for row in &rows {
        for finding in row.compute_findings(&build_identity_ref) {
            findings_summary.record(&finding);
            blocking_findings.push(finding);
        }
    }
    for finding in compute_coverage_findings(&rows) {
        findings_summary.record(&finding);
        blocking_findings.push(finding);
    }
    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });

    let axis_platform_coverage = axis_platform_coverage(&rows);
    let covered_surface_families: Vec<String> = covered_surface_families(&rows)
        .into_iter()
        .map(str::to_owned)
        .collect();

    let mut os_signal_coverage: Vec<OsAppearanceSignal> = Vec::new();
    for row in &rows {
        if !os_signal_coverage.contains(&row.os_signal) {
            os_signal_coverage.push(row.os_signal);
        }
    }

    let row_count = rows.len();
    let marketed_row_count = rows.iter().filter(|row| row.is_marketed()).count();
    let restart_or_reload_row_count = rows
        .iter()
        .filter(|row| row.posture_needs_reload_or_restart())
        .count();
    let live_change_demonstrated = rows.iter().any(|row| {
        row.is_qualified()
            && row
                .evidence
                .as_ref()
                .is_some_and(|e| e.capture_kind.is_live_transition())
    });
    let all_captures_build_attributed = rows.iter().filter(|row| row.is_qualified()).all(|row| {
        row.evidence
            .as_ref()
            .is_some_and(|e| e.attribution.build_identity_ref == build_identity_ref)
    });
    let report_clean = findings_summary.total_blocking_findings == 0;

    LiveAppearanceEvidenceReport {
        record_kind: M5_LIVE_APPEARANCE_REPORT_RECORD_KIND.to_owned(),
        schema_version: M5_LIVE_APPEARANCE_SCHEMA_VERSION,
        shared_contract_ref: M5_LIVE_APPEARANCE_SHARED_CONTRACT_REF.to_owned(),
        report_id: M5_LIVE_APPEARANCE_REPORT_ID.to_owned(),
        source_schema_ref: M5_LIVE_APPEARANCE_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Live OS theme, contrast, accent, and text-scale changes for the claimed M5 \
                   appearance rows, each bound to the exact build, theme package, and appearance \
                   session that produced its screenshot and golden evidence."
            .to_owned(),
        build_identity_ref,
        release_channel_class,
        rows,
        axis_platform_coverage,
        covered_surface_families,
        os_signal_coverage,
        row_count,
        marketed_row_count,
        restart_or_reload_row_count,
        live_change_demonstrated,
        all_captures_build_attributed,
        findings_summary,
        blocking_findings,
        report_clean,
        release_evidence_refs: vec![
            "release_center.live_appearance_evidence".to_owned(),
            "docs/release/release_evidence_object_model.md#live-appearance-evidence".to_owned(),
        ],
        extension_inspection_refs: vec![
            "extensions.appearance_inspection.live_change_evidence".to_owned()
        ],
        sync_refs: vec!["sync.appearance_evidence.attribution".to_owned()],
        docs_help_refs: vec![
            M5_LIVE_APPEARANCE_PUBLISHED_DOC_REF.to_owned(),
            "docs/m5/appearance-session-runtime.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-live-appearance-evidence".to_owned()],
        published_report_ref: M5_LIVE_APPEARANCE_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: M5_LIVE_APPEARANCE_PUBLISHED_DOC_REF.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_live_appearance_evidence_report`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum LiveAppearanceValidationError {
    /// The report has no registered rows.
    NoRegisteredRows,
    /// The report's exact-build identity ref is empty.
    BuildIdentityRefMissing,
    /// The declared axis-platform coverage does not match the rows.
    AxisPlatformCoverageStale,
    /// The declared surface coverage does not match the rows.
    SurfaceCoverageStale,
    /// The declared OS-signal coverage does not match the rows.
    OsSignalCoverageStale,
    /// The declared blocking findings do not match the recomputed findings.
    BlockingFindingsStale,
    /// A blocking finding remains in the report.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning subject ref.
        subject_ref: String,
    },
    /// No row demonstrates a live appearance change.
    NoLiveChangeDemonstrated,
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a report against the live-appearance evidence-linkage invariants.
///
/// The checks encode the track invariant and acceptance criteria: live OS
/// changes either apply through the appearance-session model or carry an
/// explicit restart/reload posture; evidence is always attributable to the exact
/// build, theme package, and appearance session that produced it; claimed rows
/// survive without hiding trust, severity, lifecycle, or focus cues; and no
/// marketed axis passes on a single platform or with only static screenshots.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_live_appearance_evidence_report(
    report: &LiveAppearanceEvidenceReport,
) -> Result<(), Vec<LiveAppearanceValidationError>> {
    let mut errors = Vec::new();

    if report.rows.is_empty() {
        errors.push(LiveAppearanceValidationError::NoRegisteredRows);
    }
    if report.build_identity_ref.trim().is_empty() {
        errors.push(LiveAppearanceValidationError::BuildIdentityRefMissing);
    }

    if axis_platform_coverage(&report.rows) != report.axis_platform_coverage {
        errors.push(LiveAppearanceValidationError::AxisPlatformCoverageStale);
    }
    let covered: Vec<String> = covered_surface_families(&report.rows)
        .into_iter()
        .map(str::to_owned)
        .collect();
    if covered != report.covered_surface_families {
        errors.push(LiveAppearanceValidationError::SurfaceCoverageStale);
    }
    let mut os_signals: Vec<OsAppearanceSignal> = Vec::new();
    for row in &report.rows {
        if !os_signals.contains(&row.os_signal) {
            os_signals.push(row.os_signal);
        }
    }
    if os_signals != report.os_signal_coverage {
        errors.push(LiveAppearanceValidationError::OsSignalCoverageStale);
    }

    // Recompute findings and assert the declared set matches.
    let mut recomputed: Vec<LiveAppearanceBlockingFinding> = Vec::new();
    for row in &report.rows {
        recomputed.extend(row.compute_findings(&report.build_identity_ref));
    }
    recomputed.extend(compute_coverage_findings(&report.rows));
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != report.blocking_findings {
        errors.push(LiveAppearanceValidationError::BlockingFindingsStale);
    }
    for finding in &report.blocking_findings {
        errors.push(LiveAppearanceValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if !report.live_change_demonstrated {
        errors.push(LiveAppearanceValidationError::NoLiveChangeDemonstrated);
    }
    if report.published_report_ref.trim().is_empty() {
        errors.push(LiveAppearanceValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(LiveAppearanceValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Compact description of one seeded live OS appearance change before its
/// derived refs, evidence, and cues are filled in.
struct RowSeed {
    slug: &'static str,
    title: &'static str,
    platform: DesktopPlatform,
    os_signal: OsAppearanceSignal,
    from_value: &'static str,
    to_value: &'static str,
    qualifies_rows: &'static [M5AppearanceRow],
    surface_family: M5AppearanceSurfaceFamily,
    semantic_salience: M5SemanticSalience,
    apply_posture: LiveApplyCapability,
    atomicity_class: AtomicityClass,
    rollback_path_class: RollbackPathClass,
    qualification_status: M5QualificationStatus,
    capture_kind: EvidenceCaptureKind,
    golden_match: GoldenMatchState,
    freshness: M5EvidenceFreshness,
    narrowing_reason: Option<&'static str>,
    narrative: &'static str,
}

impl RowSeed {
    fn expand(&self) -> LiveAppearanceEvidenceRow {
        let platform = self.platform.as_str();
        let row_id = format!("live-appearance:{platform}:{}", self.slug);
        let theme_package_ref = format!("theme-package:aureline.default@{platform}");
        let theme_revision_ref = "theme-revision:rev-3".to_owned();
        let appearance_session_ref = format!("appearance-session:{platform}:{}", self.slug);
        let checkpoint_ref = format!("appearance-checkpoint:{platform}:{}", self.slug);

        let projects_evidence = self.qualification_status.projects_evidence();
        let evidence = projects_evidence.then(|| EvidenceCapture {
            capture_id: format!("capture:{platform}:{}", self.slug),
            capture_kind: self.capture_kind,
            screenshot_ref: format!("capture://m5-live-appearance/{platform}/{}", self.slug),
            screenshot_digest: format!("sha256:seed-{platform}-{}", self.slug),
            golden_baseline_ref: format!(
                "golden://m5-live-appearance/{platform}/{}@rev-3",
                self.slug
            ),
            golden_match: self.golden_match,
            freshness: self.freshness,
            attribution: CaptureAttribution {
                build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
                release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
                theme_package_ref: theme_package_ref.clone(),
                theme_revision_ref: theme_revision_ref.clone(),
                appearance_session_ref: appearance_session_ref.clone(),
                checkpoint_ref: checkpoint_ref.clone(),
                platform: self.platform,
                os_signal: self.os_signal,
                captured_at: GENERATED_AT.to_owned(),
            },
        });

        let cue_preservation = projects_evidence.then(|| CuePreservation {
            trust_cue: cue_for(self.semantic_salience, M5SemanticSalience::TrustBearing),
            severity_cue: cue_for(self.semantic_salience, M5SemanticSalience::SeverityBearing),
            lifecycle_cue: cue_for(self.semantic_salience, M5SemanticSalience::LifecycleBearing),
            focus_cue: M5FocusVisibility::VisibleFocusRing,
            state_semantics: M5StateSemantics::Preserved,
            layout_integrity: M5LayoutIntegrity::Intact,
        });

        LiveAppearanceEvidenceRow {
            record_kind: M5_LIVE_APPEARANCE_ROW_RECORD_KIND.to_owned(),
            schema_version: M5_LIVE_APPEARANCE_SCHEMA_VERSION,
            shared_contract_ref: M5_LIVE_APPEARANCE_SHARED_CONTRACT_REF.to_owned(),
            row_id,
            title: self.title.to_owned(),
            platform: self.platform,
            os_signal: self.os_signal,
            changed_axis: self.os_signal.canonical_axis(),
            qualifies_rows: self.qualifies_rows.to_vec(),
            surface_family: self.surface_family,
            semantic_salience: self.semantic_salience,
            transition_trigger: TransitionTrigger::OsSignal,
            from_value: self.from_value.to_owned(),
            to_value: self.to_value.to_owned(),
            apply_posture: self.apply_posture,
            restart_or_reload_disclosed: matches!(
                self.apply_posture,
                LiveApplyCapability::RequiresSurfaceReload
                    | LiveApplyCapability::RequiresAppRestart
            ),
            atomicity_class: self.atomicity_class,
            rollback_path_class: self.rollback_path_class,
            theme_package_ref,
            theme_revision_ref,
            appearance_session_ref,
            checkpoint_ref,
            qualification_status: self.qualification_status,
            narrowing_reason: self.narrowing_reason.map(str::to_owned),
            cue_preservation,
            evidence,
            docs_help_refs: vec![format!(
                "{M5_LIVE_APPEARANCE_PUBLISHED_DOC_REF}#{}",
                self.slug
            )],
            narrative: self.narrative.to_owned(),
        }
    }
}

/// Returns the cue outcome a surface of `actual` salience reports for a cue
/// owned by `cue_salience`: present when the surface carries that meaning, not
/// applicable otherwise.
fn cue_for(actual: M5SemanticSalience, cue_salience: M5SemanticSalience) -> M5BoundaryCue {
    if actual == cue_salience {
        M5BoundaryCue::Present
    } else {
        M5BoundaryCue::NotApplicable
    }
}

/// Builds the seeded live-appearance evidence rows.
fn seeded_rows() -> Vec<LiveAppearanceEvidenceRow> {
    const SEEDS: &[RowSeed] = &[
        RowSeed {
            slug: "system-theme-flip",
            title: "macOS system theme flip on notebook cell chrome",
            platform: DesktopPlatform::Macos,
            os_signal: OsAppearanceSignal::SystemThemeFlip,
            from_value: "light",
            to_value: "dark",
            qualifies_rows: &[M5AppearanceRow::ThemeDark],
            surface_family: M5AppearanceSurfaceFamily::NotebookCellChrome,
            semantic_salience: M5SemanticSalience::LifecycleBearing,
            apply_posture: LiveApplyCapability::AppliesLive,
            atomicity_class: AtomicityClass::SingleCheckpointAtomic,
            rollback_path_class: RollbackPathClass::SingleCheckpointRevert,
            qualification_status: M5QualificationStatus::Qualified,
            capture_kind: EvidenceCaptureKind::LiveTransition,
            golden_match: GoldenMatchState::Matched,
            freshness: M5EvidenceFreshness::Fresh,
            narrowing_reason: None,
            narrative: "Following the OS dark signal flips the notebook cell chrome live from one \
                        checkpoint; the run-state lifecycle cue stays legible.",
        },
        RowSeed {
            slug: "system-theme-flip",
            title: "Windows system theme flip on result-grid rows",
            platform: DesktopPlatform::Windows,
            os_signal: OsAppearanceSignal::SystemThemeFlip,
            from_value: "dark",
            to_value: "light",
            qualifies_rows: &[M5AppearanceRow::ThemeLight],
            surface_family: M5AppearanceSurfaceFamily::ResultGridRow,
            semantic_salience: M5SemanticSalience::SeverityBearing,
            apply_posture: LiveApplyCapability::AppliesLive,
            atomicity_class: AtomicityClass::SingleCheckpointAtomic,
            rollback_path_class: RollbackPathClass::SingleCheckpointRevert,
            qualification_status: M5QualificationStatus::Qualified,
            capture_kind: EvidenceCaptureKind::LiveTransition,
            golden_match: GoldenMatchState::Matched,
            freshness: M5EvidenceFreshness::Fresh,
            narrowing_reason: None,
            narrative: "Result-grid severity badges keep their meaning when the OS flips to the \
                        light system theme live.",
        },
        RowSeed {
            slug: "system-theme-flip",
            title: "Linux system theme flip on the profiler panel",
            platform: DesktopPlatform::Linux,
            os_signal: OsAppearanceSignal::SystemThemeFlip,
            from_value: "light",
            to_value: "dark",
            qualifies_rows: &[M5AppearanceRow::ThemeDark],
            surface_family: M5AppearanceSurfaceFamily::ProfilerPanel,
            semantic_salience: M5SemanticSalience::Informational,
            apply_posture: LiveApplyCapability::AppliesLive,
            atomicity_class: AtomicityClass::SingleCheckpointAtomic,
            rollback_path_class: RollbackPathClass::SingleCheckpointRevert,
            qualification_status: M5QualificationStatus::Qualified,
            capture_kind: EvidenceCaptureKind::LiveTransition,
            golden_match: GoldenMatchState::Matched,
            freshness: M5EvidenceFreshness::Fresh,
            narrowing_reason: None,
            narrative: "The profiler flame panel re-themes live on the OS dark signal without \
                        losing its capture-axis labels.",
        },
        RowSeed {
            slug: "contrast-increased",
            title: "macOS increase-contrast on the trace panel",
            platform: DesktopPlatform::Macos,
            os_signal: OsAppearanceSignal::ContrastIncreased,
            from_value: "contrast_standard",
            to_value: "contrast_high",
            qualifies_rows: &[M5AppearanceRow::ThemeHighContrast],
            surface_family: M5AppearanceSurfaceFamily::TracePanel,
            semantic_salience: M5SemanticSalience::SeverityBearing,
            apply_posture: LiveApplyCapability::AppliesLive,
            atomicity_class: AtomicityClass::SingleCheckpointAtomic,
            rollback_path_class: RollbackPathClass::SingleCheckpointRevert,
            qualification_status: M5QualificationStatus::Qualified,
            capture_kind: EvidenceCaptureKind::LiveTransition,
            golden_match: GoldenMatchState::Matched,
            freshness: M5EvidenceFreshness::Fresh,
            narrowing_reason: None,
            narrative: "Raising OS contrast live keeps trace-span severity colors above the \
                        contrast threshold.",
        },
        RowSeed {
            slug: "forced-colors-enabled",
            title: "Windows forced-colors on the embedded preview-route badge",
            platform: DesktopPlatform::Windows,
            os_signal: OsAppearanceSignal::ForcedColorsEnabled,
            from_value: "contrast_standard",
            to_value: "contrast_forced_colors",
            qualifies_rows: &[M5AppearanceRow::ThemeHighContrast],
            surface_family: M5AppearanceSurfaceFamily::PreviewRouteBadge,
            semantic_salience: M5SemanticSalience::TrustBearing,
            apply_posture: LiveApplyCapability::RequiresSurfaceReload,
            atomicity_class: AtomicityClass::SurfaceReloadFromSingleCheckpoint,
            rollback_path_class: RollbackPathClass::SurfaceReloadThenRevert,
            qualification_status: M5QualificationStatus::Qualified,
            capture_kind: EvidenceCaptureKind::LiveTransition,
            golden_match: GoldenMatchState::Matched,
            freshness: M5EvidenceFreshness::Fresh,
            narrowing_reason: None,
            narrative: "Forced-colors needs an embedded preview reload, disclosed up front; the \
                        host-boundary trust cue survives the reload.",
        },
        RowSeed {
            slug: "contrast-increased",
            title: "Linux high-contrast on the docs/browser pane",
            platform: DesktopPlatform::Linux,
            os_signal: OsAppearanceSignal::ContrastIncreased,
            from_value: "contrast_standard",
            to_value: "contrast_high",
            qualifies_rows: &[M5AppearanceRow::ThemeHighContrast],
            surface_family: M5AppearanceSurfaceFamily::DocsBrowserPane,
            semantic_salience: M5SemanticSalience::Informational,
            apply_posture: LiveApplyCapability::AppliesLive,
            atomicity_class: AtomicityClass::SingleCheckpointAtomic,
            rollback_path_class: RollbackPathClass::SingleCheckpointRevert,
            qualification_status: M5QualificationStatus::Qualified,
            capture_kind: EvidenceCaptureKind::LiveTransition,
            golden_match: GoldenMatchState::DiffWithinTolerance,
            freshness: M5EvidenceFreshness::Fresh,
            narrowing_reason: None,
            narrative: "The docs pane re-renders at high contrast live; the golden capture is \
                        within perceptual tolerance.",
        },
        RowSeed {
            slug: "reduced-motion-enabled",
            title: "macOS reduce-motion on the companion surface",
            platform: DesktopPlatform::Macos,
            os_signal: OsAppearanceSignal::ReducedMotionEnabled,
            from_value: "motion_full",
            to_value: "motion_reduced",
            qualifies_rows: &[M5AppearanceRow::ReducedMotion],
            surface_family: M5AppearanceSurfaceFamily::CompanionSurface,
            semantic_salience: M5SemanticSalience::TrustBearing,
            apply_posture: LiveApplyCapability::AppliesLive,
            atomicity_class: AtomicityClass::SingleCheckpointAtomic,
            rollback_path_class: RollbackPathClass::SingleCheckpointRevert,
            qualification_status: M5QualificationStatus::Qualified,
            capture_kind: EvidenceCaptureKind::LiveTransition,
            golden_match: GoldenMatchState::Matched,
            freshness: M5EvidenceFreshness::Fresh,
            narrowing_reason: None,
            narrative: "Reduce-motion downgrades companion presence transitions live; the device \
                        trust cue stays present.",
        },
        RowSeed {
            slug: "reduced-motion-enabled",
            title: "Windows reduce-motion on notebook cell chrome",
            platform: DesktopPlatform::Windows,
            os_signal: OsAppearanceSignal::ReducedMotionEnabled,
            from_value: "motion_full",
            to_value: "motion_reduced",
            qualifies_rows: &[M5AppearanceRow::ReducedMotion],
            surface_family: M5AppearanceSurfaceFamily::NotebookCellChrome,
            semantic_salience: M5SemanticSalience::LifecycleBearing,
            apply_posture: LiveApplyCapability::AppliesLive,
            atomicity_class: AtomicityClass::SingleCheckpointAtomic,
            rollback_path_class: RollbackPathClass::SingleCheckpointRevert,
            qualification_status: M5QualificationStatus::Qualified,
            capture_kind: EvidenceCaptureKind::LiveTransition,
            golden_match: GoldenMatchState::Matched,
            freshness: M5EvidenceFreshness::Fresh,
            narrowing_reason: None,
            narrative:
                "Notebook run-state animation reduces to a static lifecycle cue live on the \
                        OS reduce-motion signal.",
        },
        RowSeed {
            slug: "accent-color-changed",
            title: "macOS accent change on the pipeline card",
            platform: DesktopPlatform::Macos,
            os_signal: OsAppearanceSignal::AccentColorChanged,
            from_value: "accent_blue",
            to_value: "accent_graphite",
            qualifies_rows: &[],
            surface_family: M5AppearanceSurfaceFamily::PipelineCard,
            semantic_salience: M5SemanticSalience::LifecycleBearing,
            apply_posture: LiveApplyCapability::AppliesLive,
            atomicity_class: AtomicityClass::SingleCheckpointAtomic,
            rollback_path_class: RollbackPathClass::SingleCheckpointRevert,
            qualification_status: M5QualificationStatus::Qualified,
            capture_kind: EvidenceCaptureKind::LiveTransition,
            golden_match: GoldenMatchState::Matched,
            freshness: M5EvidenceFreshness::Fresh,
            narrowing_reason: None,
            narrative: "Pipeline-card status does not depend on the OS accent for meaning; the \
                        lifecycle cue survives the live accent change.",
        },
        RowSeed {
            slug: "accent-color-changed",
            title: "Windows accent change on result-grid rows",
            platform: DesktopPlatform::Windows,
            os_signal: OsAppearanceSignal::AccentColorChanged,
            from_value: "accent_blue",
            to_value: "accent_orange",
            qualifies_rows: &[],
            surface_family: M5AppearanceSurfaceFamily::ResultGridRow,
            semantic_salience: M5SemanticSalience::Informational,
            apply_posture: LiveApplyCapability::AppliesLive,
            atomicity_class: AtomicityClass::SingleCheckpointAtomic,
            rollback_path_class: RollbackPathClass::SingleCheckpointRevert,
            qualification_status: M5QualificationStatus::Qualified,
            capture_kind: EvidenceCaptureKind::LiveTransition,
            golden_match: GoldenMatchState::Matched,
            freshness: M5EvidenceFreshness::Fresh,
            narrowing_reason: None,
            narrative: "Result-grid selection follows the OS accent live without overriding \
                        severity coloring.",
        },
        RowSeed {
            slug: "text-scale-increased",
            title: "macOS text-scale increase on result-grid rows",
            platform: DesktopPlatform::Macos,
            os_signal: OsAppearanceSignal::TextScaleIncreased,
            from_value: "scale_100",
            to_value: "scale_125",
            qualifies_rows: &[],
            surface_family: M5AppearanceSurfaceFamily::ResultGridRow,
            semantic_salience: M5SemanticSalience::Informational,
            apply_posture: LiveApplyCapability::AppliesLive,
            atomicity_class: AtomicityClass::SingleCheckpointAtomic,
            rollback_path_class: RollbackPathClass::SingleCheckpointRevert,
            qualification_status: M5QualificationStatus::Qualified,
            capture_kind: EvidenceCaptureKind::LiveTransition,
            golden_match: GoldenMatchState::Matched,
            freshness: M5EvidenceFreshness::Fresh,
            narrowing_reason: None,
            narrative: "Result-grid rows reflow at 125% text scale live; no row truncates its \
                        state column.",
        },
        RowSeed {
            slug: "text-scale-increased",
            title: "Linux display-scale change on the docs/browser pane",
            platform: DesktopPlatform::Linux,
            os_signal: OsAppearanceSignal::TextScaleIncreased,
            from_value: "scale_100",
            to_value: "scale_150",
            qualifies_rows: &[],
            surface_family: M5AppearanceSurfaceFamily::DocsBrowserPane,
            semantic_salience: M5SemanticSalience::Informational,
            apply_posture: LiveApplyCapability::RequiresAppRestart,
            atomicity_class: AtomicityClass::FullRestartFromSingleCheckpoint,
            rollback_path_class: RollbackPathClass::FullRestartThenRevert,
            qualification_status: M5QualificationStatus::Qualified,
            capture_kind: EvidenceCaptureKind::LiveTransition,
            golden_match: GoldenMatchState::Matched,
            freshness: M5EvidenceFreshness::Fresh,
            narrowing_reason: None,
            narrative: "The Linux display-scale change needs a disclosed app restart; the \
                        post-restart capture is attributed to the same session checkpoint.",
        },
        RowSeed {
            slug: "forced-colors-portable-omitted",
            title: "Portable build omits forced-colors on the preview-route badge",
            platform: DesktopPlatform::Windows,
            os_signal: OsAppearanceSignal::ForcedColorsEnabled,
            from_value: "contrast_standard",
            to_value: "contrast_forced_colors",
            qualifies_rows: &[M5AppearanceRow::ThemeHighContrast],
            surface_family: M5AppearanceSurfaceFamily::PreviewRouteBadge,
            semantic_salience: M5SemanticSalience::TrustBearing,
            apply_posture: LiveApplyCapability::PlatformSignalUnavailable,
            atomicity_class: AtomicityClass::SurfaceReloadFromSingleCheckpoint,
            rollback_path_class: RollbackPathClass::SurfaceReloadThenRevert,
            qualification_status: M5QualificationStatus::PlatformOmitted,
            capture_kind: EvidenceCaptureKind::SteadyState,
            golden_match: GoldenMatchState::NoBaseline,
            freshness: M5EvidenceFreshness::Fresh,
            narrowing_reason: Some(
                "The portable build does not register the OS forced-colors handler, so this live \
                 change is omitted and disclosed rather than claimed.",
            ),
            narrative:
                "An honest omission: with no platform signal the portable build narrows the \
                        row instead of faking a live forced-colors capture.",
        },
    ];

    SEEDS.iter().map(RowSeed::expand).collect()
}

/// Builds the seeded live-appearance evidence report.
///
/// Uses the frozen [`SEED_BUILD_IDENTITY_REF`] so the checked-in fixtures stay
/// reproducible. A live runtime would call [`build_live_appearance_evidence_report`]
/// with [`aureline_build_info::exact_build_identity_ref`] instead.
pub fn seeded_live_appearance_evidence_report() -> LiveAppearanceEvidenceReport {
    build_live_appearance_evidence_report(
        SEED_BUILD_IDENTITY_REF,
        SEED_RELEASE_CHANNEL_CLASS,
        seeded_rows(),
    )
}
