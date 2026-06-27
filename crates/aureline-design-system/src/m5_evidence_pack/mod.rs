//! Component-gallery demo scenes and visual / accessibility evidence packs for the launch-critical
//! M5 component families.
//!
//! Where [`crate::m5_host_primitive`] ships the host-rendered *implementations* every M5 surface
//! routes through, and [`crate::m5_component_manifest`] freezes their *contracts*, this module
//! generates the **evidence** that proves those surfaces still render correctly — a reproducible
//! gallery the shell-quality gate reads instead of a folder of hand-captured screenshots.
//!
//! A versioned [`M5EvidencePack`] carries one [`M5ComponentEvidence`] per
//! [component kind](crate::m5_component_manifest::M5ComponentKind). Each component evidence record:
//!
//! - **renders one [`M5GalleryScene`] per controlled state**, derived directly from the
//!   [host primitive's render plan](crate::m5_host_primitive::M5StateRenderPlan): the parts it
//!   renders, the [non-color cues](crate::NonColorCueClass) it carries (always including label text),
//!   the status message id it announces, and the keyboard / assistive-technology evidence refs. The
//!   scenes are minted from the checked-in primitive library, so the proof is reproducible from the
//!   same contract Aureline ships rather than from a manual capture.
//! - **captures one [`M5AppearanceVariantEvidence`] per appearance variant** — normal dark and light
//!   themes, both high-contrast variants, the reduced-motion posture, and two zoom levels — in the
//!   *same* pack. Each variant carries a deterministic [`baseline_digest`](M5AppearanceVariantEvidence::baseline_digest)
//!   computed over its canonical descriptor, so a visual regression changes the digest and the
//!   checked-in baseline diff fails without anyone eyeballing a screenshot.
//! - **attaches proof freshness and the owning component identity** — the
//!   [owner role](M5ComponentEvidence::owner_role), [component id](M5ComponentEvidence::component_id),
//!   the [captured / evaluated dates](M5ComponentEvidence::captured_at) and freshness window, and a
//!   derived [claim gate](M5EvidenceClaimGate). Freshness is *computed* from the dates and window
//!   ([`evidence_freshness`]); when a component's evidence falls outside its window the gate
//!   auto-narrows that component's shell-quality claim, and missing coverage blocks it outright.
//!
//! The records are metadata-only truth packets: they carry semantic token *references*, message
//! *ids*, and content *digests*, never raw color values, raw screenshots, credential bodies, or
//! provider payloads.
//!
//! - Schema:
//!   [`schemas/design-system/m5-evidence-pack.schema.json`](../../../../../schemas/design-system/m5-evidence-pack.schema.json)
//! - Doc:
//!   [`docs/design-system/m5-evidence-pack.md`](../../../../../docs/design-system/m5-evidence-pack.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_evidence_pack, seeded_m5_evidence_pack_stale_narrowed, M5_EVIDENCE_PACK_ID,
    M5_EVIDENCE_PACK_STALE_EVALUATED_AT, M5_EVIDENCE_PACK_VERSION,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_ui::themes::AccessibilityPostureClass;
use aureline_ui::tokens::ThemeClass;

use crate::m5_component_manifest::M5ComponentKind;
use crate::{CanonicalStateClass, NonColorCueClass};

/// Record-kind tag carried by [`M5EvidencePack`].
pub const M5_EVIDENCE_PACK_RECORD_KIND: &str = "m5_design_system_evidence_pack";

/// Record-kind tag carried by [`M5EvidencePackReleasePacket`].
pub const M5_EVIDENCE_PACK_RELEASE_RECORD_KIND: &str = "m5_design_system_evidence_pack_release";

/// Schema version shared by the evidence-pack records.
pub const M5_EVIDENCE_PACK_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the evidence-pack boundary schema.
pub const M5_EVIDENCE_PACK_SCHEMA_REF: &str = "schemas/design-system/m5-evidence-pack.schema.json";

/// Repo-relative path of the evidence-pack contract doc.
pub const M5_EVIDENCE_PACK_DOC_REF: &str = "docs/design-system/m5-evidence-pack.md";

/// Repo-relative path of the release-grade evidence-pack proof packet — the proof lane that blocks
/// drift for the pack.
pub const M5_EVIDENCE_PACK_PROOF_REF: &str =
    "artifacts/release/m5-design-system-proof/evidence-pack-release.json";

/// Release packet that keeps the evidence pack current (shared with the foundation package,
/// component manifests, host primitives, and contract matrix).
pub const M5_EVIDENCE_PACK_RELEASE_PACKET_REF: &str = "evidence:m5-design-system-release-packet";

/// Repo-relative directory of the checked-in evidence-pack gallery fixtures.
pub const M5_EVIDENCE_PACK_DIR: &str = "fixtures/ui/m5-component-gallery/";

/// Prefix every governed message id in this lane carries so consumers can route them.
pub const M5_EVIDENCE_MESSAGE_ID_PREFIX: &str = "design_system_evidence.";

/// One captured appearance variant: the axis of appearance a gallery scene is captured under. Every
/// scene captures all of them in the same pack, so high-contrast, reduced-motion, and zoom evidence
/// never lives apart from the normal-theme baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceVariantKind {
    /// Normal dark reference theme at 100% zoom.
    NormalDark,
    /// Normal light parity theme at 100% zoom.
    NormalLight,
    /// High-contrast dark theme at 100% zoom.
    HighContrastDark,
    /// High-contrast light theme at 100% zoom.
    HighContrastLight,
    /// Dark reference theme captured under the reduced-motion posture.
    ReducedMotion,
    /// Dark reference theme captured at 150% zoom.
    #[serde(rename = "zoom_150")]
    Zoom150,
    /// Dark reference theme captured at 200% zoom.
    #[serde(rename = "zoom_200")]
    Zoom200,
}

impl M5EvidenceVariantKind {
    /// Every required appearance variant, in capture order. Every scene MUST carry all of them.
    pub const ALL: [Self; 7] = [
        Self::NormalDark,
        Self::NormalLight,
        Self::HighContrastDark,
        Self::HighContrastLight,
        Self::ReducedMotion,
        Self::Zoom150,
        Self::Zoom200,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NormalDark => "normal_dark",
            Self::NormalLight => "normal_light",
            Self::HighContrastDark => "high_contrast_dark",
            Self::HighContrastLight => "high_contrast_light",
            Self::ReducedMotion => "reduced_motion",
            Self::Zoom150 => "zoom_150",
            Self::Zoom200 => "zoom_200",
        }
    }

    /// The theme class this variant is captured under.
    pub const fn theme_class(self) -> ThemeClass {
        match self {
            Self::NormalDark | Self::ReducedMotion | Self::Zoom150 | Self::Zoom200 => {
                ThemeClass::DarkReference
            }
            Self::NormalLight => ThemeClass::LightParity,
            Self::HighContrastDark => ThemeClass::HighContrastDark,
            Self::HighContrastLight => ThemeClass::HighContrastLight,
        }
    }

    /// The motion posture this variant is captured under.
    pub const fn motion_posture(self) -> AccessibilityPostureClass {
        match self {
            Self::ReducedMotion => AccessibilityPostureClass::MotionReduced,
            _ => AccessibilityPostureClass::MotionStandard,
        }
    }

    /// The zoom level (percent) this variant is captured at.
    pub const fn zoom_percent(self) -> u32 {
        match self {
            Self::Zoom150 => 150,
            Self::Zoom200 => 200,
            _ => 100,
        }
    }

    /// True for the two high-contrast variants.
    pub const fn is_high_contrast(self) -> bool {
        matches!(self, Self::HighContrastDark | Self::HighContrastLight)
    }

    /// True for the reduced-motion variant.
    pub const fn is_reduced_motion(self) -> bool {
        matches!(self, Self::ReducedMotion)
    }

    /// True for a zoom variant above 100%.
    pub const fn is_zoom(self) -> bool {
        matches!(self, Self::Zoom150 | Self::Zoom200)
    }
}

/// Freshness of a component's evidence, computed from its capture date, evaluation date, and
/// freshness window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceFreshness {
    /// The evidence is within its freshness window.
    Current,
    /// The evidence has fallen outside its freshness window and must be recaptured.
    Stale,
}

impl M5EvidenceFreshness {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
        }
    }

    /// True when the evidence is within its freshness window.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// The shell-quality claim a component's evidence supports, derived from its freshness and coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceClaimGate {
    /// Current evidence with full coverage certifies the component's shell-quality claim.
    Certified,
    /// Stale evidence auto-narrows the component's claim to a disclosed reduced posture.
    Narrowed,
    /// Missing scene or variant coverage blocks the claim outright until evidence is captured.
    Blocked,
}

impl M5EvidenceClaimGate {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Narrowed => "narrowed",
            Self::Blocked => "blocked",
        }
    }

    /// True when the gate prevents the component from promoting at full shell-quality.
    pub const fn blocks(self) -> bool {
        matches!(self, Self::Blocked)
    }

    /// True when the gate narrows the component's claim rather than certifying or blocking it.
    pub const fn narrows(self) -> bool {
        matches!(self, Self::Narrowed)
    }
}

/// One captured appearance variant of a gallery scene.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AppearanceVariantEvidence {
    /// The appearance variant this evidence captures.
    pub variant_kind: M5EvidenceVariantKind,
    /// The theme class the variant is captured under (mirrors the variant kind).
    pub theme_class: ThemeClass,
    /// The motion posture the variant is captured under (mirrors the variant kind).
    pub motion_posture: AccessibilityPostureClass,
    /// The zoom level (percent) the variant is captured at (mirrors the variant kind).
    pub zoom_percent: u32,
    /// Deterministic content digest of the captured variant descriptor; the visual-diff baseline.
    pub baseline_digest: String,
    /// Repo-relative reference into the proof packet for this variant's baseline capture.
    pub baseline_capture_ref: String,
    /// Repo-relative reference into the proof packet for this variant's diff artifact.
    pub diff_artifact_ref: String,
    /// True when the scene's meaning is carried by non-color cues in this variant (never color only).
    pub non_color_meaning_present: bool,
    /// True when a focus indicator is visible in this variant where the scene can receive focus.
    pub focus_visible: bool,
}

/// One controlled-state gallery scene: the deterministic capture of a host primitive in a single
/// controlled state across every appearance variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GalleryScene {
    /// Stable scene id, unique within the pack.
    pub scene_id: String,
    /// The controlled state this scene renders.
    pub state: CanonicalStateClass,
    /// True when the host primitive marks this state mandatory for the family.
    pub mandatory: bool,
    /// Human-readable scene name.
    pub display_name: String,
    /// Anatomy part ids rendered in this scene (inherited from the primitive render plan).
    pub rendered_parts: Vec<String>,
    /// Non-color cues carried in this scene; always includes label text.
    pub non_color_cues: Vec<NonColorCueClass>,
    /// Governed status message id announced in this scene; prefixed [`M5_EVIDENCE_MESSAGE_ID_PREFIX`].
    pub status_message_id: String,
    /// True when the scene offers a focusable action.
    pub interactive: bool,
    /// Repo-relative reference to the keyboard-journey evidence for this scene.
    pub keyboard_journey_ref: String,
    /// Repo-relative reference to the assistive-technology evidence for this scene.
    pub assistive_technology_ref: String,
    /// One appearance variant per [`M5EvidenceVariantKind`], in capture order.
    pub variants: Vec<M5AppearanceVariantEvidence>,
}

impl M5GalleryScene {
    /// The variant captured for a kind, if present.
    pub fn variant(&self, kind: M5EvidenceVariantKind) -> Option<&M5AppearanceVariantEvidence> {
        self.variants.iter().find(|v| v.variant_kind == kind)
    }

    /// The high-contrast variants captured for this scene.
    pub fn high_contrast_variants(&self) -> Vec<&M5AppearanceVariantEvidence> {
        self.variants
            .iter()
            .filter(|v| v.variant_kind.is_high_contrast())
            .collect()
    }
}

/// One launch-critical family's evidence: its gallery scenes, owning identity, and freshness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComponentEvidence {
    /// The governed component kind, shared with the host primitive and component manifest.
    pub component_kind: M5ComponentKind,
    /// The component id this evidence is owned by (the manifest / primitive component id).
    pub component_id: String,
    /// The host primitive id the scenes are rendered from.
    pub primitive_id: String,
    /// Owner role accountable for this component's evidence.
    pub owner_role: String,
    /// Human-readable component name.
    pub display_name: String,
    /// Date the evidence was captured (ISO-8601).
    pub captured_at: String,
    /// Date the freshness was evaluated as-of (ISO-8601).
    pub evaluated_at: String,
    /// Freshness window in days; evidence older than this at evaluation time is stale.
    pub freshness_window_days: u32,
    /// Freshness computed from the dates and window (see [`evidence_freshness`]).
    pub freshness: M5EvidenceFreshness,
    /// The shell-quality claim gate derived from freshness and coverage.
    pub claim_gate: M5EvidenceClaimGate,
    /// One scene per controlled state, covering the full canonical set.
    pub scenes: Vec<M5GalleryScene>,
    /// Stable summary message id; prefixed [`M5_EVIDENCE_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
}

impl M5ComponentEvidence {
    /// Total appearance variants captured across all scenes.
    pub fn total_variants(&self) -> usize {
        self.scenes.iter().map(|s| s.variants.len()).sum()
    }

    /// The scene for a controlled state, if present.
    pub fn scene(&self, state: CanonicalStateClass) -> Option<&M5GalleryScene> {
        self.scenes.iter().find(|s| s.state == state)
    }

    /// Count of captured variants of a given kind across all scenes.
    pub fn variant_kind_count(&self, kind: M5EvidenceVariantKind) -> usize {
        self.scenes
            .iter()
            .flat_map(|s| s.variants.iter())
            .filter(|v| v.variant_kind == kind)
            .count()
    }

    /// True when scene and variant coverage is complete (every canonical state, every variant kind).
    pub fn coverage_complete(&self) -> bool {
        let states: BTreeSet<CanonicalStateClass> = self.scenes.iter().map(|s| s.state).collect();
        let canonical: BTreeSet<CanonicalStateClass> =
            CanonicalStateClass::required().iter().copied().collect();
        if states != canonical || states.len() != self.scenes.len() {
            return false;
        }
        self.scenes.iter().all(|scene| {
            let kinds: BTreeSet<M5EvidenceVariantKind> =
                scene.variants.iter().map(|v| v.variant_kind).collect();
            kinds.len() == scene.variants.len()
                && M5EvidenceVariantKind::ALL.iter().all(|k| kinds.contains(k))
        })
    }
}

/// A versioned, machine-readable pack of M5 component evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EvidencePack {
    /// Record kind; must equal [`M5_EVIDENCE_PACK_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EVIDENCE_PACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable pack id.
    pub pack_id: String,
    /// Pack version (semver `MAJOR.MINOR.PATCH`).
    pub pack_version: String,
    /// Owner role accountable for the pack.
    pub owner_role: String,
    /// The host-primitive library the scenes are rendered from.
    pub source_primitive_library_ref: String,
    /// The component-manifest package the owning identities are taken from.
    pub source_manifest_package_ref: String,
    /// The foundation package the appearance vocabulary is taken from.
    pub source_foundation_package_ref: String,
    /// Per-component evidence (one per [`M5ComponentKind`]).
    pub components: Vec<M5ComponentEvidence>,
    /// Repo-relative proof lane that blocks drift.
    pub proof_lane_ref: String,
    /// Repo-relative release packet that keeps the pack current.
    pub release_packet_ref: String,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Stable summary message id; prefixed [`M5_EVIDENCE_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5EvidencePack {
    /// Finds the evidence for a component kind.
    pub fn component(&self, kind: M5ComponentKind) -> Option<&M5ComponentEvidence> {
        self.components.iter().find(|c| c.component_kind == kind)
    }

    /// Total components in the pack.
    pub fn total_components(&self) -> usize {
        self.components.len()
    }

    /// Total scenes across the pack.
    pub fn total_scenes(&self) -> usize {
        self.components.iter().map(|c| c.scenes.len()).sum()
    }

    /// Total appearance variants across the pack.
    pub fn total_variants(&self) -> usize {
        self.components.iter().map(|c| c.total_variants()).sum()
    }

    /// The components whose evidence is stale at the recorded evaluation time.
    pub fn stale_components(&self) -> Vec<&M5ComponentEvidence> {
        self.components
            .iter()
            .filter(|c| c.freshness == M5EvidenceFreshness::Stale)
            .collect()
    }

    /// The components whose claim is narrowed by stale evidence.
    pub fn narrowed_components(&self) -> Vec<&M5ComponentEvidence> {
        self.components
            .iter()
            .filter(|c| c.claim_gate.narrows())
            .collect()
    }

    /// The components whose claim is blocked by missing evidence coverage.
    pub fn blocked_components(&self) -> Vec<&M5ComponentEvidence> {
        self.components
            .iter()
            .filter(|c| c.claim_gate.blocks())
            .collect()
    }

    /// The pack-level claim gate: the worst (most restrictive) gate across components.
    pub fn pack_claim_gate(&self) -> M5EvidenceClaimGate {
        if self.components.iter().any(|c| c.claim_gate.blocks()) {
            M5EvidenceClaimGate::Blocked
        } else if self.components.iter().any(|c| c.claim_gate.narrows()) {
            M5EvidenceClaimGate::Narrowed
        } else {
            M5EvidenceClaimGate::Certified
        }
    }

    /// Re-evaluates every component's freshness and claim gate as-of a new evaluation date, returning
    /// the re-evaluated pack. This is how a shell-quality gate inspects whether the checked-in
    /// evidence is still current for a given release date and narrows claims when it is not. Capture
    /// dates, scenes, and digests are unchanged — only freshness and the derived gate move.
    pub fn reevaluate(&self, evaluated_at: &str) -> M5EvidencePack {
        let mut next = self.clone();
        for component in &mut next.components {
            component.evaluated_at = evaluated_at.to_owned();
            component.freshness = evidence_freshness(
                &component.captured_at,
                evaluated_at,
                component.freshness_window_days,
            );
            component.claim_gate =
                derive_claim_gate(component.freshness, component.coverage_complete());
        }
        next
    }

    /// Validates the pack invariants, returning the violations (empty when valid).
    pub fn validate(&self) -> Vec<M5EvidencePackViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_EVIDENCE_PACK_RECORD_KIND {
            violations.push(M5EvidencePackViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EVIDENCE_PACK_SCHEMA_VERSION {
            violations.push(M5EvidencePackViolation::WrongSchemaVersion);
        }
        if self.pack_id.trim().is_empty()
            || self.owner_role.trim().is_empty()
            || self.source_primitive_library_ref.trim().is_empty()
            || self.source_manifest_package_ref.trim().is_empty()
            || self.source_foundation_package_ref.trim().is_empty()
            || self.proof_lane_ref.trim().is_empty()
            || self.release_packet_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5EvidencePackViolation::MissingIdentity);
        }
        if !is_semver(&self.pack_version) {
            violations.push(M5EvidencePackViolation::BadPackVersion);
        }
        if !self
            .summary_message_id
            .starts_with(M5_EVIDENCE_MESSAGE_ID_PREFIX)
        {
            violations.push(M5EvidencePackViolation::MessageIdPrefixMissing);
        }

        for required in [
            M5_EVIDENCE_PACK_SCHEMA_REF,
            M5_EVIDENCE_PACK_DOC_REF,
            M5_EVIDENCE_PACK_PROOF_REF,
        ] {
            if !self.source_contract_refs.iter().any(|r| r == required) {
                violations.push(M5EvidencePackViolation::MissingSourceContracts);
                break;
            }
        }

        validate_component_set(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 evidence pack serializes"),
        ) {
            violations.push(M5EvidencePackViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// True when the pack validates with no violations.
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }

    /// Deterministic export-safe JSON for the pack.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 evidence pack serializes")
    }

    /// Imports a pack from JSON. The caller validates the returned pack with [`Self::validate`].
    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }

    /// Projects the release-packet inclusion: per-component freshness, gate, and shape summaries, so
    /// a release record names the evidence QA and support exports cite and can narrow claims when a
    /// component's proof goes stale.
    pub fn release_packet(&self) -> M5EvidencePackReleasePacket {
        let component_summaries: Vec<M5EvidenceComponentSummary> = self
            .components
            .iter()
            .map(|c| M5EvidenceComponentSummary {
                component_kind: c.component_kind,
                component_id: c.component_id.clone(),
                owner_role: c.owner_role.clone(),
                scene_count: c.scenes.len() as u32,
                variant_count: c.total_variants() as u32,
                high_contrast_variant_count: c
                    .variant_kind_count(M5EvidenceVariantKind::HighContrastDark)
                    as u32
                    + c.variant_kind_count(M5EvidenceVariantKind::HighContrastLight) as u32,
                reduced_motion_variant_count: c
                    .variant_kind_count(M5EvidenceVariantKind::ReducedMotion)
                    as u32,
                zoom_variant_count: c.variant_kind_count(M5EvidenceVariantKind::Zoom150) as u32
                    + c.variant_kind_count(M5EvidenceVariantKind::Zoom200) as u32,
                captured_at: c.captured_at.clone(),
                freshness: c.freshness,
                claim_gate: c.claim_gate,
            })
            .collect();

        M5EvidencePackReleasePacket {
            record_kind: M5_EVIDENCE_PACK_RELEASE_RECORD_KIND.to_owned(),
            schema_version: M5_EVIDENCE_PACK_SCHEMA_VERSION,
            pack_id: self.pack_id.clone(),
            pack_version: self.pack_version.clone(),
            total_components: self.total_components() as u32,
            total_scenes: self.total_scenes() as u32,
            total_variants: self.total_variants() as u32,
            certified_component_count: self
                .components
                .iter()
                .filter(|c| c.claim_gate == M5EvidenceClaimGate::Certified)
                .count() as u32,
            narrowed_component_count: self.narrowed_components().len() as u32,
            blocked_component_count: self.blocked_components().len() as u32,
            pack_claim_gate: self.pack_claim_gate(),
            component_summaries,
            proof_lane_ref: self.proof_lane_ref.clone(),
            release_packet_ref: self.release_packet_ref.clone(),
            source_contract_refs: self.source_contract_refs.clone(),
            redaction_class_token: self.redaction_class_token.clone(),
            summary_message_id: format!(
                "{}{}.release",
                M5_EVIDENCE_MESSAGE_ID_PREFIX, self.pack_id
            ),
            minted_at: self.minted_at.clone(),
        }
    }
}

/// Reads and validates the checked-in canonical evidence-pack fixture.
pub fn current_stable_m5_evidence_pack() -> Result<M5EvidencePack, M5EvidencePackArtifactError> {
    let pack: M5EvidencePack = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-component-gallery/evidence-pack.json"
    )))
    .map_err(M5EvidencePackArtifactError::Parse)?;
    let violations = pack.validate();
    if violations.is_empty() {
        Ok(pack)
    } else {
        Err(M5EvidencePackArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading a checked-in evidence-pack export.
#[derive(Debug)]
pub enum M5EvidencePackArtifactError {
    /// The export failed to parse.
    Parse(serde_json::Error),
    /// The export failed validation.
    Validation(Vec<M5EvidencePackViolation>),
}

impl fmt::Display for M5EvidencePackArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => {
                write!(formatter, "m5 evidence pack parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
                write!(
                    formatter,
                    "m5 evidence pack failed validation: {}",
                    tokens.join(",")
                )
            }
        }
    }
}

impl Error for M5EvidencePackArtifactError {}

/// Validation failures emitted by [`M5EvidencePack::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5EvidencePackViolation {
    /// Pack record kind is wrong.
    WrongRecordKind,
    /// Pack schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is missing.
    MissingIdentity,
    /// The pack version is not `MAJOR.MINOR.PATCH`.
    BadPackVersion,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A governed component kind has no evidence.
    RequiredComponentKindMissing,
    /// Two components share a kind.
    DuplicateComponentKind,
    /// Two components share a component id.
    DuplicateComponentId,
    /// A component is missing an identity field (component id, primitive id, owner, or summary id).
    ComponentIncomplete,
    /// A component's freshness window is zero.
    FreshnessWindowZero,
    /// A component's recorded freshness does not match the value computed from its dates and window.
    FreshnessMismatch,
    /// A component's recorded claim gate does not match the value derived from freshness and coverage.
    ClaimGateMismatch,
    /// A component's scenes do not cover exactly the canonical state set, repeat a state, or declare
    /// no mandatory scene.
    SceneCoverageIncomplete,
    /// A scene is incomplete (missing id, no rendered parts, missing label-text cue, or unprefixed
    /// status id).
    SceneIncomplete,
    /// A scene's variants do not cover every required appearance variant kind, or repeat a kind.
    VariantCoverageIncomplete,
    /// A captured variant is incomplete (digest mismatch, missing refs, color-only meaning, or
    /// missing focus on an interactive scene).
    VariantIncomplete,
    /// A message id is missing the governed prefix.
    MessageIdPrefixMissing,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5EvidencePackViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::BadPackVersion => "bad_pack_version",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredComponentKindMissing => "required_component_kind_missing",
            Self::DuplicateComponentKind => "duplicate_component_kind",
            Self::DuplicateComponentId => "duplicate_component_id",
            Self::ComponentIncomplete => "component_incomplete",
            Self::FreshnessWindowZero => "freshness_window_zero",
            Self::FreshnessMismatch => "freshness_mismatch",
            Self::ClaimGateMismatch => "claim_gate_mismatch",
            Self::SceneCoverageIncomplete => "scene_coverage_incomplete",
            Self::SceneIncomplete => "scene_incomplete",
            Self::VariantCoverageIncomplete => "variant_coverage_incomplete",
            Self::VariantIncomplete => "variant_incomplete",
            Self::MessageIdPrefixMissing => "message_id_prefix_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

// ---------------------------------------------------------------------------
// Release-packet records.
// ---------------------------------------------------------------------------

/// Release-packet projection of an evidence pack: one freshness / gate / shape summary per component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EvidencePackReleasePacket {
    /// Record kind; must equal [`M5_EVIDENCE_PACK_RELEASE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// The pack id this release record projects.
    pub pack_id: String,
    /// The pack version.
    pub pack_version: String,
    /// Total components across the pack.
    pub total_components: u32,
    /// Total scenes across the pack.
    pub total_scenes: u32,
    /// Total appearance variants across the pack.
    pub total_variants: u32,
    /// Count of components whose claim is certified.
    pub certified_component_count: u32,
    /// Count of components whose claim is narrowed by stale evidence.
    pub narrowed_component_count: u32,
    /// Count of components whose claim is blocked by missing evidence.
    pub blocked_component_count: u32,
    /// The pack-level claim gate (the most restrictive across components).
    pub pack_claim_gate: M5EvidenceClaimGate,
    /// Per-component freshness / gate / shape summaries, in pack order.
    pub component_summaries: Vec<M5EvidenceComponentSummary>,
    /// Repo-relative proof lane.
    pub proof_lane_ref: String,
    /// Repo-relative release packet.
    pub release_packet_ref: String,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Stable message id; prefixed [`M5_EVIDENCE_MESSAGE_ID_PREFIX`].
    pub summary_message_id: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl M5EvidencePackReleasePacket {
    /// Deterministic export-safe JSON for the release packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 evidence pack release packet serializes")
    }
}

/// One component's freshness, gate, and shape summary inside a release packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EvidenceComponentSummary {
    /// The governed component kind.
    pub component_kind: M5ComponentKind,
    /// The owning component id.
    pub component_id: String,
    /// The owner role accountable for the evidence.
    pub owner_role: String,
    /// Scene count (the full canonical set).
    pub scene_count: u32,
    /// Total captured variant count.
    pub variant_count: u32,
    /// High-contrast variant count.
    pub high_contrast_variant_count: u32,
    /// Reduced-motion variant count.
    pub reduced_motion_variant_count: u32,
    /// Zoom variant count.
    pub zoom_variant_count: u32,
    /// Date the evidence was captured.
    pub captured_at: String,
    /// Freshness at evaluation time.
    pub freshness: M5EvidenceFreshness,
    /// The component's claim gate.
    pub claim_gate: M5EvidenceClaimGate,
}

// ---------------------------------------------------------------------------
// Freshness, gate, and digest helpers (the deterministic, clock-free core).
// ---------------------------------------------------------------------------

/// Computes a component's freshness from its capture date, evaluation date, and freshness window.
///
/// Both dates are read as a leading `YYYY-MM-DD` (any time suffix is ignored). The evidence is
/// [`M5EvidenceFreshness::Current`] when its age (evaluation minus capture, in days) is within the
/// window, and [`M5EvidenceFreshness::Stale`] once it exceeds the window. Evidence captured at or
/// after the evaluation date is current; an unparseable date is treated conservatively as stale.
pub fn evidence_freshness(
    captured_at: &str,
    evaluated_at: &str,
    window_days: u32,
) -> M5EvidenceFreshness {
    match (parse_date_days(captured_at), parse_date_days(evaluated_at)) {
        (Some(captured), Some(evaluated)) => {
            let age_days = evaluated - captured;
            if age_days <= i64::from(window_days) {
                M5EvidenceFreshness::Current
            } else {
                M5EvidenceFreshness::Stale
            }
        }
        _ => M5EvidenceFreshness::Stale,
    }
}

/// Derives a component's claim gate from its freshness and coverage. Missing coverage blocks the
/// claim, stale evidence narrows it, and current evidence with full coverage certifies it.
pub(crate) fn derive_claim_gate(
    freshness: M5EvidenceFreshness,
    coverage_complete: bool,
) -> M5EvidenceClaimGate {
    if !coverage_complete {
        M5EvidenceClaimGate::Blocked
    } else {
        match freshness {
            M5EvidenceFreshness::Current => M5EvidenceClaimGate::Certified,
            M5EvidenceFreshness::Stale => M5EvidenceClaimGate::Narrowed,
        }
    }
}

/// The deterministic visual-diff baseline digest for one captured variant of a scene. Computing it
/// over the canonical descriptor (the scene's owning identity, state, rendered parts, cues, status
/// id, and the variant's appearance axes) means a content change moves the digest, so the
/// checked-in baseline diff fails without anyone comparing a screenshot. Both the seed builder and
/// [`M5EvidencePack::validate`] call this, so the digest cannot drift from what it certifies.
pub(crate) fn variant_baseline_digest(
    component_id: &str,
    scene: &SceneDigestInput<'_>,
    variant_kind: M5EvidenceVariantKind,
) -> String {
    let parts = scene.rendered_parts.join(",");
    let cues = scene
        .non_color_cues
        .iter()
        .map(|c| c.as_str())
        .collect::<Vec<_>>()
        .join(",");
    let descriptor = format!(
        "{component_id}|{state}|{status}|{parts}|{cues}|{interactive}|{variant}|{theme}|{motion}|{zoom}",
        state = scene.state.as_str(),
        status = scene.status_message_id,
        interactive = scene.interactive,
        variant = variant_kind.as_str(),
        theme = variant_kind.theme_class().token(),
        motion = variant_kind.motion_posture().token(),
        zoom = variant_kind.zoom_percent(),
    );
    format!("fnv1a64:{:016x}", fnv1a64(descriptor.as_bytes()))
}

/// The fields of a scene that feed a variant's baseline digest. A small borrow-only view so the
/// digest can be computed both from a fully-built [`M5GalleryScene`] (in validation) and from the
/// pieces the seed builder holds before it constructs the scene.
pub(crate) struct SceneDigestInput<'a> {
    pub state: CanonicalStateClass,
    pub status_message_id: &'a str,
    pub rendered_parts: &'a [String],
    pub non_color_cues: &'a [NonColorCueClass],
    pub interactive: bool,
}

/// FNV-1a 64-bit hash. Stable run-to-run, so the digest it produces can be checked in and recomputed.
fn fnv1a64(input: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in input {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Parses the leading `YYYY-MM-DD` of a timestamp into a day number (days since the Unix epoch),
/// returning `None` when the leading date is not three numeric fields. Pure integer arithmetic, no
/// clock, so freshness is reproducible.
fn parse_date_days(timestamp: &str) -> Option<i64> {
    let date = timestamp.split(['T', ' ']).next().unwrap_or(timestamp);
    let mut fields = date.split('-');
    let year: i64 = fields.next()?.parse().ok()?;
    let month: i64 = fields.next()?.parse().ok()?;
    let day: i64 = fields.next()?.parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    Some(days_from_civil(year, month, day))
}

/// Days from the Unix epoch (1970-01-01) for a proleptic-Gregorian date (Howard Hinnant's algorithm).
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400; // [0, 399]
    let day_of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1; // [0, 365]
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year; // [0, 146096]
    era * 146097 + day_of_era - 719468
}

// ---------------------------------------------------------------------------
// Validation helpers.
// ---------------------------------------------------------------------------

fn validate_component_set(pack: &M5EvidencePack, violations: &mut Vec<M5EvidencePackViolation>) {
    let present: BTreeSet<M5ComponentKind> =
        pack.components.iter().map(|c| c.component_kind).collect();
    for required in M5ComponentKind::ALL {
        if !present.contains(&required) {
            violations.push(M5EvidencePackViolation::RequiredComponentKindMissing);
            break;
        }
    }
    if present.len() != pack.components.len() {
        violations.push(M5EvidencePackViolation::DuplicateComponentKind);
    }

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    let mut duplicate_id = false;
    for component in &pack.components {
        if !seen_ids.insert(component.component_id.as_str()) {
            duplicate_id = true;
        }
        validate_component(component, violations);
    }
    if duplicate_id {
        violations.push(M5EvidencePackViolation::DuplicateComponentId);
    }
}

fn validate_component(
    component: &M5ComponentEvidence,
    violations: &mut Vec<M5EvidencePackViolation>,
) {
    if component.component_id.trim().is_empty()
        || component.primitive_id.trim().is_empty()
        || component.owner_role.trim().is_empty()
        || component.display_name.trim().is_empty()
        || component.summary_message_id.trim().is_empty()
        || component.captured_at.trim().is_empty()
        || component.evaluated_at.trim().is_empty()
    {
        violations.push(M5EvidencePackViolation::ComponentIncomplete);
    }
    if !component
        .summary_message_id
        .starts_with(M5_EVIDENCE_MESSAGE_ID_PREFIX)
    {
        violations.push(M5EvidencePackViolation::MessageIdPrefixMissing);
    }
    if component.freshness_window_days == 0 {
        violations.push(M5EvidencePackViolation::FreshnessWindowZero);
    }

    let computed_freshness = evidence_freshness(
        &component.captured_at,
        &component.evaluated_at,
        component.freshness_window_days,
    );
    if component.freshness != computed_freshness {
        violations.push(M5EvidencePackViolation::FreshnessMismatch);
    }

    let coverage_complete = component.coverage_complete();
    if derive_claim_gate(component.freshness, coverage_complete) != component.claim_gate {
        violations.push(M5EvidencePackViolation::ClaimGateMismatch);
    }

    validate_scene_coverage(component, violations);
    for scene in &component.scenes {
        validate_scene(&component.component_id, scene, violations);
    }
}

fn validate_scene_coverage(
    component: &M5ComponentEvidence,
    violations: &mut Vec<M5EvidencePackViolation>,
) {
    let states: BTreeSet<CanonicalStateClass> = component.scenes.iter().map(|s| s.state).collect();
    let canonical: BTreeSet<CanonicalStateClass> =
        CanonicalStateClass::required().iter().copied().collect();
    let no_duplicates = states.len() == component.scenes.len();
    let any_mandatory = component.scenes.iter().any(|s| s.mandatory);
    if states != canonical || !no_duplicates || !any_mandatory {
        violations.push(M5EvidencePackViolation::SceneCoverageIncomplete);
    }
}

fn validate_scene(
    component_id: &str,
    scene: &M5GalleryScene,
    violations: &mut Vec<M5EvidencePackViolation>,
) {
    let labelled = scene.non_color_cues.contains(&NonColorCueClass::LabelText);
    if scene.scene_id.trim().is_empty()
        || scene.display_name.trim().is_empty()
        || scene.rendered_parts.is_empty()
        || scene.rendered_parts.iter().any(|p| p.trim().is_empty())
        || !labelled
        || scene.keyboard_journey_ref.trim().is_empty()
        || scene.assistive_technology_ref.trim().is_empty()
        || !scene
            .status_message_id
            .starts_with(M5_EVIDENCE_MESSAGE_ID_PREFIX)
    {
        violations.push(M5EvidencePackViolation::SceneIncomplete);
    }

    let kinds: BTreeSet<M5EvidenceVariantKind> =
        scene.variants.iter().map(|v| v.variant_kind).collect();
    let complete = kinds.len() == scene.variants.len()
        && M5EvidenceVariantKind::ALL.iter().all(|k| kinds.contains(k));
    if !complete {
        violations.push(M5EvidencePackViolation::VariantCoverageIncomplete);
    }

    let digest_input = SceneDigestInput {
        state: scene.state,
        status_message_id: &scene.status_message_id,
        rendered_parts: &scene.rendered_parts,
        non_color_cues: &scene.non_color_cues,
        interactive: scene.interactive,
    };
    for variant in &scene.variants {
        let kind = variant.variant_kind;
        let digest_ok =
            variant.baseline_digest == variant_baseline_digest(component_id, &digest_input, kind);
        let axes_ok = variant.theme_class == kind.theme_class()
            && variant.motion_posture == kind.motion_posture()
            && variant.zoom_percent == kind.zoom_percent();
        let focus_ok = !scene.interactive || variant.focus_visible;
        if !digest_ok
            || !axes_ok
            || !variant.non_color_meaning_present
            || !focus_ok
            || variant.baseline_capture_ref.trim().is_empty()
            || variant.diff_artifact_ref.trim().is_empty()
        {
            violations.push(M5EvidencePackViolation::VariantIncomplete);
        }
    }
}

/// True when `value` is a `MAJOR.MINOR.PATCH` numeric semver.
fn is_semver(value: &str) -> bool {
    let parts: Vec<&str> = value.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

/// Returns true when the JSON tree carries any forbidden raw-boundary material. Evidence packs are
/// metadata-only by construction; this is a defense-in-depth scan over the serialized export.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    const FORBIDDEN_KEYS: [&str; 6] = [
        "api_key",
        "authorization",
        "password",
        "secret",
        "access_token",
        "raw_payload",
    ];
    match value {
        serde_json::Value::Object(map) => {
            for (key, child) in map {
                if FORBIDDEN_KEYS.contains(&key.to_lowercase().as_str()) {
                    return true;
                }
                if json_contains_forbidden_boundary_material(child) {
                    return true;
                }
            }
            false
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
