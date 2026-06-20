//! Generated-artifact local-history, timeline, reversible-checkpoint, and
//! export semantics that explain captured, omitted, regenerated, and redacted
//! bytes.
//!
//! Ordinary local history can imply full-source byte continuity: every entry
//! looks like a complete snapshot the user can restore exactly. Generated
//! artifacts break that assumption. A scaffolded file may have a full
//! snapshot; a notebook output or preview derivative may store only metadata
//! and a reference to its canonical source; a regenerated candidate may be a
//! fresh re-run rather than the original bytes; an oversized or
//! policy-withheld artifact may be omitted entirely. If the timeline presents
//! all of these as ordinary full-source history, restore and compare quietly
//! lie about what they can reproduce.
//!
//! This module freezes one typed model for a generated artifact's history.
//! Each [`GeneratedTimelineEntry`] records, explicitly:
//!
//! - its [`CaptureMode`] — full snapshot, metadata-plus-reference,
//!   regenerated candidate, or omitted bytes,
//! - its [`RedactionClass`] — whether the captured content was reduced for
//!   secrets, size, or policy,
//! - and its lineage links — [`GeneratorIdentity`], [`CanonicalSourceRef`],
//!   divergence ([`DriftState`]), and a reversible-checkpoint lineage
//!   reference.
//!
//! One [`classify_generated_history`] engine folds the capture mode, the
//! redaction class, and the divergence state into a single
//! [`GeneratedHistoryOutcome`]: the [`RestoreFidelity`] a restore may claim
//! (exact snapshot, compatible regeneration, or evidence only), whether
//! **exact generated-byte continuity** may be claimed at all, a
//! [`ByteProvenance`] explanation of what was captured, reconstructed, or
//! omitted, the [`CompareBasis`] available, the [`RestoreAvailability`] of the
//! restore action, and stable block-reason tokens. The marquee guardrail is
//! frozen here: **exact generated-byte continuity is claimed only when the
//! timeline captured a full, unredacted snapshot.** A metadata-plus-reference,
//! regenerated-candidate, omitted, or redacted capture never lets restore or
//! compare claim exact byte continuity.
//!
//! Every entry carries a metadata-safe, lineage-preserving
//! [`TimelineExportProjection`], so compare, restore, support, and export
//! flows consume one object model that names the generator, the canonical
//! source, the checkpoint lineage, the capture mode, and the restore fidelity
//! without ever crossing a raw body, secret, or live-authority boundary.
//!
//! The packet is mirrored, byte-for-byte, by the checked-in schema, reviewer
//! doc, proof packet, certification report, and fixture corpus named on the
//! module constants:
//!
//! - [`/schemas/generated/generated-timeline-entry.schema.json`](../../../../schemas/generated/generated-timeline-entry.schema.json)
//! - [`/docs/generated/generated-history.md`](../../../../docs/generated/generated-history.md)
//! - [`/artifacts/generated/generated-timeline-packet.json`](../../../../artifacts/generated/generated-timeline-packet.json)
//! - [`/artifacts/generated/generated-timeline.md`](../../../../artifacts/generated/generated-timeline.md)
//! - [`/fixtures/generated/timeline/`](../../../../fixtures/generated/timeline/)

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::descriptor::{
    CanonicalSourceRef, CanonicalSourceState, GeneratorIdentity, GeneratorKind,
};
use crate::m5_generated_governance::ArtifactClass;

pub use crate::descriptor::DriftState;

/// Schema version stamped onto the timeline packet and fixtures.
pub const GENERATED_TIMELINE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the timeline packet.
pub const GENERATED_TIMELINE_PACKET_RECORD_KIND: &str = "generated_timeline_packet_record";

/// Stable record-kind tag carried by timeline fixtures.
pub const GENERATED_TIMELINE_FIXTURE_RECORD_KIND: &str = "generated_timeline_fixture_record";

/// Stable packet id every surface binding ingests.
pub const GENERATED_TIMELINE_PACKET_ID: &str = "generated.generated_timeline.v1";

/// Repo-relative schema ref.
pub const GENERATED_TIMELINE_SCHEMA_REF: &str =
    "schemas/generated/generated-timeline-entry.schema.json";

/// Repo-relative reviewer doc ref.
pub const GENERATED_TIMELINE_DOC_REF: &str = "docs/generated/generated-history.md";

/// Repo-relative machine-readable proof packet.
pub const GENERATED_TIMELINE_PACKET_REF: &str =
    "artifacts/generated/generated-timeline-packet.json";

/// Repo-relative reviewer certification summary.
pub const GENERATED_TIMELINE_REPORT_REF: &str = "artifacts/generated/generated-timeline.md";

/// Repo-relative fixture directory.
pub const GENERATED_TIMELINE_FIXTURE_DIR: &str = "fixtures/generated/timeline";

/// Repo-relative fixture manifest.
pub const GENERATED_TIMELINE_FIXTURE_MANIFEST_REF: &str =
    "fixtures/generated/timeline/manifest.yaml";

// ---------------------------------------------------------------------------
// Capture vocabulary.
// ---------------------------------------------------------------------------

/// How a generated artifact's bytes were captured into local history for one
/// timeline entry. This is the structural form of what history actually
/// stored, named explicitly so a derived file is never presented as ordinary
/// full-source history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMode {
    /// The full artifact bytes were captured directly into local history.
    FullSnapshot,
    /// Only metadata and a reference to the canonical source were captured;
    /// the bytes are reconstructed by regenerating from that source.
    MetadataPlusReference,
    /// A regenerated candidate was captured — bytes produced by re-running the
    /// generator rather than the original captured bytes.
    RegeneratedCandidate,
    /// The bytes were intentionally omitted; only evidence and metadata were
    /// captured.
    OmittedBytes,
}

impl CaptureMode {
    /// Every capture mode in canonical order.
    pub const ALL: [Self; 4] = [
        Self::FullSnapshot,
        Self::MetadataPlusReference,
        Self::RegeneratedCandidate,
        Self::OmittedBytes,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSnapshot => "full_snapshot",
            Self::MetadataPlusReference => "metadata_plus_reference",
            Self::RegeneratedCandidate => "regenerated_candidate",
            Self::OmittedBytes => "omitted_bytes",
        }
    }

    /// Whether this capture mode stored artifact bytes locally for the entry.
    /// A full snapshot stores the original bytes; a regenerated candidate
    /// stores candidate bytes; a metadata-plus-reference and an omitted
    /// capture store no bytes.
    pub const fn stores_local_bytes(self) -> bool {
        matches!(self, Self::FullSnapshot | Self::RegeneratedCandidate)
    }

    /// Whether this capture mode captured the artifact's *original* bytes in
    /// full. Only a full snapshot can ever back an exact-byte-continuity
    /// claim.
    pub const fn captures_original_bytes(self) -> bool {
        matches!(self, Self::FullSnapshot)
    }

    /// Whether restoring from this capture mode requires regenerating from the
    /// canonical source rather than writing the original captured bytes.
    pub const fn requires_regeneration(self) -> bool {
        !self.captures_original_bytes()
    }

    /// The stable block-reason token this capture mode contributes, if any.
    /// A full snapshot contributes none; every other mode names why exact
    /// byte continuity is unavailable.
    pub const fn block_token(self) -> Option<&'static str> {
        match self {
            Self::FullSnapshot => None,
            Self::MetadataPlusReference => Some("capture_metadata_plus_reference"),
            Self::RegeneratedCandidate => Some("capture_regenerated_candidate"),
            Self::OmittedBytes => Some("capture_bytes_omitted"),
        }
    }
}

/// Whether and why a captured generated artifact's content was reduced for
/// safety or size. Redaction is orthogonal to [`CaptureMode`]: a full snapshot
/// can still be redacted, in which case the stored bytes are no longer a
/// faithful copy of the original.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// No redaction; the captured content is faithful to what was captured.
    #[serde(rename = "none")]
    Unredacted,
    /// Secret material was stripped from the captured content.
    SecretsRedacted,
    /// Content was truncated or capped because of a size limit.
    SizeCapped,
    /// Content was withheld by policy.
    PolicyWithheld,
}

impl RedactionClass {
    /// Every redaction class in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Unredacted,
        Self::SecretsRedacted,
        Self::SizeCapped,
        Self::PolicyWithheld,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unredacted => "none",
            Self::SecretsRedacted => "secrets_redacted",
            Self::SizeCapped => "size_capped",
            Self::PolicyWithheld => "policy_withheld",
        }
    }

    /// Whether any content was redacted.
    pub const fn is_redacted(self) -> bool {
        !matches!(self, Self::Unredacted)
    }

    /// The restore-fidelity floor this redaction class forces, if any. Any
    /// redaction means the stored bytes are not a faithful copy, so they can
    /// no longer back an exact-snapshot restore; a policy withholding leaves
    /// only evidence.
    pub const fn restore_fidelity_floor(self) -> Option<RestoreFidelity> {
        match self {
            Self::Unredacted => None,
            Self::SecretsRedacted | Self::SizeCapped => {
                Some(RestoreFidelity::CompatibleRegeneration)
            }
            Self::PolicyWithheld => Some(RestoreFidelity::EvidenceOnly),
        }
    }

    /// The stable block-reason token this redaction class contributes, if any.
    pub const fn block_token(self) -> Option<&'static str> {
        match self {
            Self::Unredacted => None,
            Self::SecretsRedacted => Some("redaction_secrets"),
            Self::SizeCapped => Some("redaction_size_capped"),
            Self::PolicyWithheld => Some("redaction_policy_withheld"),
        }
    }
}

/// The fidelity a restore may claim for a generated-artifact timeline entry.
/// Declaration order is the narrowing order: [`RestoreFidelity::ExactSnapshot`]
/// is the strongest claim and [`RestoreFidelity::EvidenceOnly`] the weakest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreFidelity {
    /// The full original bytes were captured; restore writes them exactly and
    /// may claim exact generated-byte continuity.
    ExactSnapshot,
    /// The bytes must be regenerated from the canonical source; restore is
    /// compatible but cannot claim exact byte continuity.
    CompatibleRegeneration,
    /// Only evidence and metadata exist; no byte-restore is possible.
    EvidenceOnly,
}

impl RestoreFidelity {
    /// Every restore fidelity in canonical order.
    pub const ALL: [Self; 3] = [
        Self::ExactSnapshot,
        Self::CompatibleRegeneration,
        Self::EvidenceOnly,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactSnapshot => "exact_snapshot",
            Self::CompatibleRegeneration => "compatible_regeneration",
            Self::EvidenceOnly => "evidence_only",
        }
    }

    /// User-facing label paired with the token.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::ExactSnapshot => "Exact snapshot",
            Self::CompatibleRegeneration => "Compatible regeneration",
            Self::EvidenceOnly => "Evidence only",
        }
    }

    /// Narrowing severity. Higher is a weaker, more honest claim; the engine
    /// always takes the highest severity among the base fidelity and every
    /// triggered floor.
    pub const fn severity(self) -> u8 {
        match self {
            Self::ExactSnapshot => 0,
            Self::CompatibleRegeneration => 1,
            Self::EvidenceOnly => 2,
        }
    }

    /// The compare basis this fidelity exposes.
    pub const fn compare_basis(self) -> CompareBasis {
        match self {
            Self::ExactSnapshot => CompareBasis::ByteSnapshot,
            Self::CompatibleRegeneration => CompareBasis::RegeneratedCandidate,
            Self::EvidenceOnly => CompareBasis::EvidenceManifest,
        }
    }

    /// The restore action availability this fidelity exposes.
    pub const fn restore_availability(self) -> RestoreAvailability {
        match self {
            Self::ExactSnapshot => RestoreAvailability::Available,
            Self::CompatibleRegeneration => RestoreAvailability::ReviewRequired,
            Self::EvidenceOnly => RestoreAvailability::DisabledExportOnly,
        }
    }
}

/// The comparison basis a compare view may offer for a timeline entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareBasis {
    /// A byte-for-byte comparison against the captured snapshot.
    ByteSnapshot,
    /// A comparison against a regenerated candidate rebuilt from the canonical
    /// source.
    RegeneratedCandidate,
    /// A comparison limited to the evidence manifest; bytes are unavailable.
    EvidenceManifest,
}

impl CompareBasis {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ByteSnapshot => "byte_snapshot",
            Self::RegeneratedCandidate => "regenerated_candidate",
            Self::EvidenceManifest => "evidence_manifest",
        }
    }
}

/// The availability of the restore action on a timeline entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreAvailability {
    /// Restore can write the captured bytes without an extra review step.
    Available,
    /// Restore is available only after a regeneration review or confirmation.
    ReviewRequired,
    /// Restore is disabled; only evidence export remains available.
    DisabledExportOnly,
}

impl RestoreAvailability {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::ReviewRequired => "review_required",
            Self::DisabledExportOnly => "disabled_export_only",
        }
    }

    /// Whether this state can write a new restore checkpoint.
    pub const fn can_restore(self) -> bool {
        matches!(self, Self::Available | Self::ReviewRequired)
    }
}

/// A surface that renders or consumes generated-artifact timeline entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimelineSurface {
    /// The in-product local-history timeline list.
    HistoryTimeline,
    /// A compare/diff view opened from a timeline entry.
    CompareView,
    /// A restore preview or recovery card.
    RestorePreview,
    /// A metadata-safe support export.
    SupportExport,
}

impl TimelineSurface {
    /// Every consuming surface in canonical order.
    pub const ALL: [Self; 4] = [
        Self::HistoryTimeline,
        Self::CompareView,
        Self::RestorePreview,
        Self::SupportExport,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HistoryTimeline => "history_timeline",
            Self::CompareView => "compare_view",
            Self::RestorePreview => "restore_preview",
            Self::SupportExport => "support_export",
        }
    }
}

// ---------------------------------------------------------------------------
// Engine outcome.
// ---------------------------------------------------------------------------

/// A review-safe explanation of what a timeline entry captured, reconstructed,
/// or omitted. This is the field that keeps the timeline from implying ordinary
/// full-source history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ByteProvenance {
    /// True when the full original bytes were captured directly.
    pub captured_directly: bool,
    /// True when the bytes are reconstructed from the canonical source plus
    /// metadata rather than read from a captured snapshot.
    pub reconstructed_from_source: bool,
    /// True when the bytes were intentionally omitted.
    pub bytes_omitted: bool,
    /// True when the captured content was redacted.
    pub redacted: bool,
    /// Review-safe sentence explaining the byte provenance.
    pub summary: String,
}

/// The conclusion the engine reaches for one timeline entry's capture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedHistoryOutcome {
    /// The fidelity a restore may claim.
    pub restore_fidelity: RestoreFidelity,
    /// Whether exact generated-byte continuity may be claimed. True only when
    /// the timeline captured a full, unredacted snapshot.
    pub exact_byte_continuity_claimed: bool,
    /// What was captured, reconstructed, or omitted.
    pub byte_provenance: ByteProvenance,
    /// The comparison basis available.
    pub compare_basis: CompareBasis,
    /// The restore action availability.
    pub restore_availability: RestoreAvailability,
    /// Whether a restore writes a new local-history checkpoint.
    pub writes_new_checkpoint_on_restore: bool,
    /// Stable tokens naming every input that narrowed the restore fidelity or
    /// blocked exact byte continuity, sorted and deduplicated.
    pub block_reason_tokens: Vec<String>,
}

/// Folds a timeline entry's capture mode, redaction class, and divergence
/// state into the single restore/compare/export outcome every surface shares.
///
/// Two guardrails are frozen here:
///
/// - **Exact byte continuity requires a full, unredacted snapshot.** The
///   outcome claims exact generated-byte continuity only when the capture mode
///   is [`CaptureMode::FullSnapshot`] and the redaction class is
///   [`RedactionClass::Unredacted`]; every other capture or any redaction
///   narrows the fidelity below [`RestoreFidelity::ExactSnapshot`].
/// - **Fidelity only narrows.** The fidelity starts at the capture mode's base
///   and is floored by redaction and — when restore must regenerate — by a
///   missing canonical source; the strictest result wins.
pub fn classify_generated_history(
    capture_mode: CaptureMode,
    redaction_class: RedactionClass,
    divergence_state: DriftState,
) -> GeneratedHistoryOutcome {
    let mut restore_fidelity = base_fidelity(capture_mode);

    if let Some(floor) = redaction_class.restore_fidelity_floor() {
        if floor.severity() > restore_fidelity.severity() {
            restore_fidelity = floor;
        }
    }

    // Divergence only narrows the regeneration path. A full snapshot holds the
    // original bytes locally, so a drifting or missing canonical source cannot
    // weaken its restore. When restore must regenerate, a missing source means
    // the bytes can no longer be rebuilt at all.
    if capture_mode.requires_regeneration()
        && divergence_state == DriftState::SourceMissing
        && RestoreFidelity::EvidenceOnly.severity() > restore_fidelity.severity()
    {
        restore_fidelity = RestoreFidelity::EvidenceOnly;
    }

    let exact_byte_continuity_claimed = restore_fidelity == RestoreFidelity::ExactSnapshot;

    let mut block_reason_tokens = Vec::new();
    if let Some(token) = capture_mode.block_token() {
        block_reason_tokens.push(token.to_owned());
    }
    if let Some(token) = redaction_class.block_token() {
        block_reason_tokens.push(token.to_owned());
    }
    if capture_mode.requires_regeneration() {
        if let Some(token) = divergence_block_token(divergence_state) {
            block_reason_tokens.push(token.to_owned());
        }
    }
    block_reason_tokens.sort();
    block_reason_tokens.dedup();

    let byte_provenance = byte_provenance(capture_mode, redaction_class, divergence_state);

    GeneratedHistoryOutcome {
        restore_fidelity,
        exact_byte_continuity_claimed,
        byte_provenance,
        compare_basis: restore_fidelity.compare_basis(),
        restore_availability: restore_fidelity.restore_availability(),
        writes_new_checkpoint_on_restore: restore_fidelity.restore_availability().can_restore(),
        block_reason_tokens,
    }
}

const fn base_fidelity(capture_mode: CaptureMode) -> RestoreFidelity {
    match capture_mode {
        CaptureMode::FullSnapshot => RestoreFidelity::ExactSnapshot,
        CaptureMode::MetadataPlusReference | CaptureMode::RegeneratedCandidate => {
            RestoreFidelity::CompatibleRegeneration
        }
        CaptureMode::OmittedBytes => RestoreFidelity::EvidenceOnly,
    }
}

const fn divergence_block_token(divergence_state: DriftState) -> Option<&'static str> {
    match divergence_state {
        DriftState::InSync => None,
        DriftState::Drifting => Some("regeneration_source_drifting"),
        DriftState::SourceMissing => Some("regeneration_source_missing"),
        DriftState::Unknown => Some("regeneration_source_unknown"),
    }
}

fn byte_provenance(
    capture_mode: CaptureMode,
    redaction_class: RedactionClass,
    divergence_state: DriftState,
) -> ByteProvenance {
    let captured_directly = matches!(capture_mode, CaptureMode::FullSnapshot);
    let reconstructed_from_source = matches!(
        capture_mode,
        CaptureMode::MetadataPlusReference | CaptureMode::RegeneratedCandidate
    );
    let bytes_omitted = matches!(capture_mode, CaptureMode::OmittedBytes);
    let redacted = redaction_class.is_redacted();

    let mut summary = match capture_mode {
        CaptureMode::FullSnapshot => {
            "Full bytes were captured directly into local history.".to_owned()
        }
        CaptureMode::MetadataPlusReference => {
            "Only metadata and a canonical-source reference were captured; bytes are reconstructed by regenerating from the canonical source.".to_owned()
        }
        CaptureMode::RegeneratedCandidate => {
            "A regenerated candidate was captured; restoring re-runs the generator and may differ from the original bytes.".to_owned()
        }
        CaptureMode::OmittedBytes => {
            "Bytes were intentionally omitted; only evidence and metadata were captured.".to_owned()
        }
    };

    if redacted {
        summary.push(' ');
        summary.push_str(match redaction_class {
            RedactionClass::Unredacted => "",
            RedactionClass::SecretsRedacted => {
                "Secret material was redacted, so the stored content is not a faithful copy."
            }
            RedactionClass::SizeCapped => {
                "Content was capped for size, so the stored content is incomplete."
            }
            RedactionClass::PolicyWithheld => {
                "Content was withheld by policy, leaving only evidence."
            }
        });
    }

    if capture_mode.requires_regeneration() {
        match divergence_state {
            DriftState::Drifting => {
                summary.push_str(" The canonical source has drifted, so a regenerated candidate may not match the captured state.");
            }
            DriftState::SourceMissing => {
                summary.push_str(
                    " The canonical source is missing, so the bytes can no longer be regenerated.",
                );
            }
            DriftState::Unknown => {
                summary.push_str(" Drift against the canonical source has not been computed.");
            }
            DriftState::InSync => {}
        }
    }

    ByteProvenance {
        captured_directly,
        reconstructed_from_source,
        bytes_omitted,
        redacted,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Export projection.
// ---------------------------------------------------------------------------

/// The metadata-safe, lineage-preserving projection a timeline entry exposes
/// to compare, restore, support, and export flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineExportProjection {
    /// True when the export includes the entry id.
    pub includes_entry_id: bool,
    /// True when the export includes the generator identity.
    pub includes_generator_identity: bool,
    /// True when the export includes the canonical-source reference.
    pub includes_canonical_source_ref: bool,
    /// True when the export includes the checkpoint lineage reference.
    pub includes_checkpoint_lineage_ref: bool,
    /// True when the export includes the capture mode.
    pub includes_capture_mode: bool,
    /// True when the export includes the restore fidelity.
    pub includes_restore_fidelity: bool,
    /// True when raw captured bodies are excluded.
    pub raw_body_excluded: bool,
    /// True when secret material is excluded.
    pub raw_secret_material_excluded: bool,
    /// True when live authority or privilege handles are excluded.
    pub live_authority_excluded: bool,
}

impl TimelineExportProjection {
    /// The metadata-safe, lineage-preserving baseline every entry carries.
    pub const fn metadata_safe_baseline() -> Self {
        Self {
            includes_entry_id: true,
            includes_generator_identity: true,
            includes_canonical_source_ref: true,
            includes_checkpoint_lineage_ref: true,
            includes_capture_mode: true,
            includes_restore_fidelity: true,
            raw_body_excluded: true,
            raw_secret_material_excluded: true,
            live_authority_excluded: true,
        }
    }

    /// True when the projection both preserves lineage and excludes raw
    /// material, so it can cross a support-export boundary.
    pub const fn is_export_safe(&self) -> bool {
        self.includes_entry_id
            && self.includes_generator_identity
            && self.includes_canonical_source_ref
            && self.includes_checkpoint_lineage_ref
            && self.includes_capture_mode
            && self.includes_restore_fidelity
            && self.raw_body_excluded
            && self.raw_secret_material_excluded
            && self.live_authority_excluded
    }
}

// ---------------------------------------------------------------------------
// Timeline entry.
// ---------------------------------------------------------------------------

/// One generated-artifact local-history timeline entry: the per-checkpoint
/// object compare, restore, support, and export surfaces render.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedTimelineEntry {
    /// Stable entry id for timeline and support joins.
    pub entry_id: String,
    /// Compact, review-safe label.
    pub display_label: String,
    /// Generated-artifact class.
    pub artifact_class: ArtifactClass,
    /// Review-safe display label for the artifact path.
    pub artifact_path_label: String,
    /// Generator that produced the artifact, with version (lineage link).
    pub generator: GeneratorIdentity,
    /// Canonical source the artifact derives from (lineage link).
    pub canonical_source: CanonicalSourceRef,
    /// Divergence between the captured/derived bytes and the canonical source
    /// (lineage link).
    pub divergence_state: DriftState,
    /// Reference to the reversible-checkpoint lineage that captured the change
    /// (lineage link).
    pub checkpoint_lineage_ref: String,
    /// How the bytes were captured into local history.
    pub capture_mode: CaptureMode,
    /// Whether and why the captured content was reduced.
    pub redaction_class: RedactionClass,
    /// Review-safe reference to locally-stored bytes for this entry. Non-empty
    /// only when the capture mode stores local bytes.
    pub captured_body_ref: String,
    /// The engine-computed outcome stamped onto the entry.
    pub outcome: GeneratedHistoryOutcome,
    /// The metadata-safe, lineage-preserving export projection.
    pub export_projection: TimelineExportProjection,
    /// The one stable copy/export form for the entry.
    pub copy_line: String,
    /// Producer timestamp for the captured checkpoint.
    pub captured_at: String,
    /// Short reviewer note.
    pub notes: String,
}

impl GeneratedTimelineEntry {
    /// The one stable copy/export form for the entry.
    pub fn copy_line(&self) -> String {
        timeline_copy_line(self)
    }
}

/// Computes the stable copy/export form for a timeline entry.
pub fn timeline_copy_line(entry: &GeneratedTimelineEntry) -> String {
    format!(
        "generated-timeline class={} capture={} redaction={} divergence={} fidelity={} exact_byte_continuity={} compare={} restore={} generator={} source={} checkpoint_lineage_present={}",
        entry.artifact_class.as_str(),
        entry.capture_mode.as_str(),
        entry.redaction_class.as_str(),
        entry.divergence_state.as_str(),
        entry.outcome.restore_fidelity.as_str(),
        entry.outcome.exact_byte_continuity_claimed,
        entry.outcome.compare_basis.as_str(),
        entry.outcome.restore_availability.as_str(),
        entry.generator.copy_form(),
        entry.canonical_source.state.as_str(),
        !entry.checkpoint_lineage_ref.is_empty(),
    )
}

// ---------------------------------------------------------------------------
// Packet structures.
// ---------------------------------------------------------------------------

/// One binding proving a surface ingests this packet rather than re-deriving
/// generated-artifact history semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineSurfaceBinding {
    /// Surface that ingests the packet.
    pub surface: TimelineSurface,
    /// Checked consumer ref that renders the timeline entry.
    pub consumer_ref: String,
    /// Packet id the surface ingests.
    pub ingested_packet_id: String,
    /// Review-safe summary of the binding.
    pub summary: String,
}

/// Shared source references for the timeline packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineSourceContractRefs {
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Proof packet ref.
    pub packet_ref: String,
    /// Certification summary ref.
    pub report_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

/// Top-level packet modeling generated-artifact timeline entries and the
/// surfaces that render them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedTimelinePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared refs.
    pub source_contract_refs: TimelineSourceContractRefs,
    /// Surfaces that consume the timeline.
    pub consumer_surfaces: Vec<TimelineSurface>,
    /// Upstream generated-artifact and history packets this lane composes.
    pub evidence_packet_refs: Vec<String>,
    /// Timeline entries covering the capture modes and restore fidelities.
    pub entries: Vec<GeneratedTimelineEntry>,
    /// Surface bindings, one per consuming surface.
    pub surface_bindings: Vec<TimelineSurfaceBinding>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// One fixture binding a timeline entry to its expected outcome, proving the
/// canonical restore/compare behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedTimelineEntryFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Reviewer scenario label.
    pub scenario: String,
    /// The entry under test.
    pub entry: GeneratedTimelineEntry,
    /// Expected restore fidelity.
    pub expected_restore_fidelity: RestoreFidelity,
    /// Expected exact-byte-continuity claim.
    pub expected_exact_byte_continuity_claimed: bool,
    /// Expected block-reason tokens.
    pub expected_block_reason_tokens: Vec<String>,
    /// One consumer that renders this entry.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

/// One validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationViolation {
    /// Stable check id.
    pub check_id: &'static str,
    /// Human-readable explanation.
    pub message: String,
}

/// Validation report for the timeline packet or fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    /// All detected violations.
    pub violations: Vec<ValidationViolation>,
}

impl ValidationReport {
    fn push(&mut self, check_id: &'static str, message: impl Into<String>) {
        self.violations.push(ValidationViolation {
            check_id,
            message: message.into(),
        });
    }

    fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "generated-artifact timeline validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

fn is_forbidden_ref(value: &str) -> bool {
    value.starts_with("obj:")
        || value.starts_with("raw:")
        || value.starts_with("secret:")
        || value.starts_with("token:")
}

fn validate_entry(report: &mut ValidationReport, entry: &GeneratedTimelineEntry) {
    let owner = format!("entry {}", entry.entry_id);

    if entry.entry_id.trim().is_empty() {
        report.push("entry.id", "entry must carry a stable id");
    }
    if entry.display_label.trim().is_empty() {
        report.push(
            "entry.display_label",
            format!("{owner} must carry a display label"),
        );
    }
    if entry.artifact_path_label.trim().is_empty() {
        report.push(
            "entry.path_label",
            format!("{owner} must carry an artifact path label"),
        );
    }
    if entry.generator.name.trim().is_empty() || entry.generator.version.trim().is_empty() {
        report.push(
            "entry.generator_identity",
            format!("{owner} must carry a generator name and version"),
        );
    }
    if entry.checkpoint_lineage_ref.trim().is_empty() {
        report.push(
            "entry.checkpoint_lineage_ref",
            format!("{owner} must preserve a checkpoint lineage ref"),
        );
    }
    if entry.captured_at.trim().is_empty() {
        report.push(
            "entry.captured_at",
            format!("{owner} must carry a captured-at timestamp"),
        );
    }
    if entry.notes.trim().is_empty() {
        report.push("entry.notes", format!("{owner} must carry a note"));
    }

    // Lineage refs must never expose raw bodies, secrets, or live handles.
    for reference in [
        entry.checkpoint_lineage_ref.as_str(),
        entry.canonical_source.source_ref.as_str(),
        entry.captured_body_ref.as_str(),
    ] {
        if is_forbidden_ref(reference) {
            report.push(
                "entry.ref_forbidden",
                format!("{owner} refs must not expose raw body, secret, or token handles"),
            );
        }
    }

    // Canonical-source consistency mirrors the descriptor lane.
    match entry.canonical_source.state {
        CanonicalSourceState::Linked => {
            if entry.canonical_source.source_ref.trim().is_empty() {
                report.push(
                    "entry.source_ref",
                    format!("{owner} linked canonical source must carry a source ref"),
                );
            }
        }
        CanonicalSourceState::Hidden | CanonicalSourceState::Missing => {
            if !entry.canonical_source.source_ref.trim().is_empty() {
                report.push(
                    "entry.source_ref",
                    format!("{owner} hidden/missing canonical source must not carry a source ref"),
                );
            }
        }
    }
    if entry.canonical_source.state == CanonicalSourceState::Missing
        && entry.divergence_state != DriftState::SourceMissing
    {
        report.push(
            "entry.divergence_consistency",
            format!("{owner} missing canonical source must report source_missing divergence"),
        );
    }
    if entry.divergence_state == DriftState::SourceMissing
        && entry.canonical_source.state != CanonicalSourceState::Missing
    {
        report.push(
            "entry.divergence_consistency",
            format!("{owner} source_missing divergence requires a missing canonical source"),
        );
    }

    // The captured-body ref is present exactly when the capture mode stores
    // local bytes.
    if entry.capture_mode.stores_local_bytes() {
        if entry.captured_body_ref.trim().is_empty() {
            report.push(
                "entry.captured_body_ref",
                format!("{owner} byte-storing capture must carry a captured body ref"),
            );
        }
    } else if !entry.captured_body_ref.trim().is_empty() {
        report.push(
            "entry.captured_body_ref",
            format!("{owner} non-byte-storing capture must not carry a captured body ref"),
        );
    }

    // The stamped outcome must equal what the engine computes.
    let expected = classify_generated_history(
        entry.capture_mode,
        entry.redaction_class,
        entry.divergence_state,
    );
    if entry.outcome != expected {
        report.push(
            "entry.outcome",
            format!("{owner} stamped outcome disagrees with the engine"),
        );
    }

    // The marquee guardrail: exact byte continuity requires a full, unredacted
    // snapshot. Restore and compare may never claim exact continuity for a
    // metadata-plus-reference, regenerated-candidate, omitted, or redacted
    // capture.
    let exact_allowed = entry.capture_mode.captures_original_bytes()
        && entry.redaction_class == RedactionClass::Unredacted;
    if entry.outcome.exact_byte_continuity_claimed && !exact_allowed {
        report.push(
            "entry.exact_byte_continuity_guardrail",
            format!(
                "{owner} must not claim exact byte continuity without a full unredacted snapshot"
            ),
        );
    }
    if entry.outcome.exact_byte_continuity_claimed
        && entry.outcome.restore_fidelity != RestoreFidelity::ExactSnapshot
    {
        report.push(
            "entry.exact_byte_continuity_guardrail",
            format!("{owner} exact byte continuity requires exact-snapshot fidelity"),
        );
    }
    if entry.outcome.compare_basis == CompareBasis::ByteSnapshot
        && !entry.outcome.exact_byte_continuity_claimed
    {
        report.push(
            "entry.compare_guardrail",
            format!("{owner} byte-snapshot compare requires an exact-byte-continuity claim"),
        );
    }

    // The copy line and export projection must be stable and safe.
    if entry.copy_line != timeline_copy_line(entry) {
        report.push(
            "entry.copy_line",
            format!("{owner} stamped copy line disagrees with the engine"),
        );
    }
    if !entry.export_projection.is_export_safe() {
        report.push(
            "entry.export_projection",
            format!("{owner} export projection must preserve lineage and exclude raw material"),
        );
    }
}

/// Validates the checked-in timeline packet contract.
pub fn validate_generated_timeline_packet(
    packet: &GeneratedTimelinePacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != GENERATED_TIMELINE_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            "packet record_kind does not match the frozen token",
        );
    }
    if packet.schema_version != GENERATED_TIMELINE_SCHEMA_VERSION {
        report.push("packet.schema_version", "packet schema_version must be 1");
    }
    if packet.packet_id != GENERATED_TIMELINE_PACKET_ID {
        report.push("packet.packet_id", "packet_id drifted from the frozen id");
    }
    if packet.title.trim().is_empty() {
        report.push("packet.title", "packet must carry a title");
    }
    if packet.source_contract_refs.doc_ref != GENERATED_TIMELINE_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted from the frozen doc");
    }
    if packet.source_contract_refs.schema_ref != GENERATED_TIMELINE_SCHEMA_REF {
        report.push(
            "packet.schema_ref",
            "schema_ref drifted from the frozen schema",
        );
    }
    if packet.source_contract_refs.packet_ref != GENERATED_TIMELINE_PACKET_REF {
        report.push(
            "packet.packet_ref",
            "packet_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.report_ref != GENERATED_TIMELINE_REPORT_REF {
        report.push(
            "packet.report_ref",
            "report_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.fixture_manifest_ref != GENERATED_TIMELINE_FIXTURE_MANIFEST_REF {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted from the frozen manifest",
        );
    }
    if packet.consumer_surfaces != TimelineSurface::ALL.to_vec() {
        report.push(
            "packet.consumer_surfaces",
            "packet must list every consumer surface in canonical order",
        );
    }
    if packet.evidence_packet_refs.is_empty() {
        report.push(
            "packet.evidence_packet_refs",
            "packet must cite the upstream generated-artifact and history evidence",
        );
    }
    if packet.invariants.is_empty() {
        report.push("packet.invariants", "packet must declare invariants");
    }

    if packet.entries.is_empty() {
        report.push("packet.entries", "packet must carry at least one entry");
    }

    let mut entry_ids = BTreeSet::new();
    let mut covered_captures = BTreeSet::new();
    let mut covered_fidelities = BTreeSet::new();
    for entry in &packet.entries {
        if !entry_ids.insert(entry.entry_id.clone()) {
            report.push(
                "packet.entry_unique",
                format!("duplicate entry id {}", entry.entry_id),
            );
        }
        covered_captures.insert(entry.capture_mode);
        covered_fidelities.insert(entry.outcome.restore_fidelity);
        validate_entry(&mut report, entry);
    }
    for required in CaptureMode::ALL {
        if !covered_captures.contains(&required) {
            report.push(
                "packet.capture_coverage",
                format!("packet must cover capture mode {}", required.as_str()),
            );
        }
    }
    for required in RestoreFidelity::ALL {
        if !covered_fidelities.contains(&required) {
            report.push(
                "packet.fidelity_coverage",
                format!("packet must cover restore fidelity {}", required.as_str()),
            );
        }
    }

    validate_surface_bindings(&mut report, packet);

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

fn validate_surface_bindings(report: &mut ValidationReport, packet: &GeneratedTimelinePacket) {
    let mut surfaces = BTreeSet::new();
    for binding in &packet.surface_bindings {
        surfaces.insert(binding.surface);
        if binding.ingested_packet_id != packet.packet_id {
            report.push(
                "binding.packet_id",
                format!(
                    "binding for {} must ingest the packet id",
                    binding.surface.as_str()
                ),
            );
        }
        if binding.consumer_ref.trim().is_empty() || binding.summary.trim().is_empty() {
            report.push(
                "binding.prose",
                format!(
                    "binding for {} must carry a consumer ref and summary",
                    binding.surface.as_str()
                ),
            );
        }
    }
    for required in TimelineSurface::ALL {
        if !surfaces.contains(&required) {
            report.push(
                "packet.binding_coverage",
                format!("packet must bind surface {}", required.as_str()),
            );
        }
    }
}

/// Validates one checked-in timeline fixture against the frozen contract.
pub fn validate_generated_timeline_entry_fixture(
    fixture: &GeneratedTimelineEntryFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != GENERATED_TIMELINE_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != GENERATED_TIMELINE_SCHEMA_VERSION {
        report.push("fixture.schema_version", "fixture schema_version must be 1");
    }
    if fixture.fixture_id.trim().is_empty() {
        report.push("fixture.id", "fixture must carry a stable id");
    }
    if fixture.scenario.trim().is_empty() {
        report.push(
            "fixture.scenario",
            format!("fixture {} must carry a scenario label", fixture.fixture_id),
        );
    }
    if fixture.consumer_ref.trim().is_empty() {
        report.push(
            "fixture.consumer_ref",
            format!("fixture {} must cite a consumer ref", fixture.fixture_id),
        );
    }
    if fixture.notes.trim().is_empty() {
        report.push(
            "fixture.notes",
            format!("fixture {} must carry a reviewer note", fixture.fixture_id),
        );
    }

    validate_entry(&mut report, &fixture.entry);

    let outcome = &fixture.entry.outcome;
    if fixture.expected_restore_fidelity != outcome.restore_fidelity {
        report.push(
            "fixture.expected_restore_fidelity",
            format!(
                "fixture {} expected restore fidelity disagrees with the entry",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_exact_byte_continuity_claimed != outcome.exact_byte_continuity_claimed {
        report.push(
            "fixture.expected_exact_byte_continuity_claimed",
            format!(
                "fixture {} expected exact-byte-continuity claim disagrees with the entry",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_block_reason_tokens != outcome.block_reason_tokens {
        report.push(
            "fixture.expected_block_reason_tokens",
            format!(
                "fixture {} expected block-reason tokens disagree with the entry",
                fixture.fixture_id
            ),
        );
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

// ---------------------------------------------------------------------------
// Evidence and class helpers used by the seed.
// ---------------------------------------------------------------------------

const GOVERNANCE_PACKET_REF: &str = "artifacts/generated/m5-generated-proof-packet.json";
const DESCRIPTOR_PACKET_REF: &str = "artifacts/generated/generated-artifact-descriptor-packet.json";
const ROLLBACK_CHECKPOINT_REF: &str =
    "artifacts/migration/rollback_checkpoint_examples/checkpoint_created_pre_apply.yaml";
const RESTORE_PROVENANCE_REF: &str = "artifacts/migration/m3/restore_provenance_packet.md";

fn evidence_packet_refs() -> Vec<String> {
    [
        GOVERNANCE_PACKET_REF,
        DESCRIPTOR_PACKET_REF,
        ROLLBACK_CHECKPOINT_REF,
        RESTORE_PROVENANCE_REF,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

const SEED_CAPTURED_AT: &str = "2026-06-20T00:00:00Z";

fn class_generator(artifact_class: ArtifactClass) -> GeneratorIdentity {
    let (kind, name, version) = match artifact_class {
        ArtifactClass::ScaffoldedProject => (GeneratorKind::Template, "rust-cli-starter", "1.4.0"),
        ArtifactClass::NotebookOutput => (GeneratorKind::Kernel, "python-kernel", "3.11.6"),
        ArtifactClass::PreviewDerivative => (GeneratorKind::Builder, "preview-bundler", "0.9.2"),
        ArtifactClass::RequestArtifact => (GeneratorKind::Runner, "request-runner", "2.3.1"),
        ArtifactClass::FrameworkCodegen => (GeneratorKind::Framework, "openapi-codegen", "5.0.0"),
        ArtifactClass::AiAssistedEdit => (GeneratorKind::Composer, "scoped-composer", "1.0.0"),
        ArtifactClass::SupportPacket => (GeneratorKind::Exporter, "support-exporter", "4.2.0"),
    };
    GeneratorIdentity {
        kind,
        name: name.to_owned(),
        version: version.to_owned(),
    }
}

fn class_path_label(artifact_class: ArtifactClass) -> &'static str {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => "src/main.rs",
        ArtifactClass::NotebookOutput => "analysis.ipynb#cell-7-output",
        ArtifactClass::PreviewDerivative => ".preview/bundle.js",
        ArtifactClass::RequestArtifact => "requests/users.list.response.json",
        ArtifactClass::FrameworkCodegen => "generated/api_client.rs",
        ArtifactClass::AiAssistedEdit => "src/parser.rs",
        ArtifactClass::SupportPacket => "support/diagnostic-bundle.json",
    }
}

fn class_source_ref(artifact_class: ArtifactClass) -> &'static str {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => "templates/rust-cli-starter",
        ArtifactClass::NotebookOutput => "analysis.ipynb#cell-7",
        ArtifactClass::PreviewDerivative => "src/index.ts",
        ArtifactClass::RequestArtifact => "requests/users.list.request.json",
        ArtifactClass::FrameworkCodegen => "openapi/users.yaml",
        ArtifactClass::AiAssistedEdit => "src/parser.rs@checkpoint",
        ArtifactClass::SupportPacket => "workspace diagnostics snapshot",
    }
}

/// Builds a timeline entry, stamping the engine-computed outcome and copy line.
#[allow(clippy::too_many_arguments)]
fn build_entry(
    entry_id: &str,
    display_label: &str,
    artifact_class: ArtifactClass,
    capture_mode: CaptureMode,
    redaction_class: RedactionClass,
    canonical_source_state: CanonicalSourceState,
    divergence_state: DriftState,
    notes: &str,
) -> GeneratedTimelineEntry {
    let generator = class_generator(artifact_class);
    let source_ref = match canonical_source_state {
        CanonicalSourceState::Linked => class_source_ref(artifact_class).to_owned(),
        CanonicalSourceState::Hidden | CanonicalSourceState::Missing => String::new(),
    };
    let captured_body_ref = if capture_mode.stores_local_bytes() {
        match capture_mode {
            CaptureMode::FullSnapshot => {
                "local-history snapshot body (captured in this checkpoint)".to_owned()
            }
            CaptureMode::RegeneratedCandidate => {
                "local-history regenerated candidate body".to_owned()
            }
            _ => String::new(),
        }
    } else {
        String::new()
    };
    let outcome = classify_generated_history(capture_mode, redaction_class, divergence_state);
    let mut entry = GeneratedTimelineEntry {
        entry_id: entry_id.to_owned(),
        display_label: display_label.to_owned(),
        artifact_class,
        artifact_path_label: class_path_label(artifact_class).to_owned(),
        generator,
        canonical_source: CanonicalSourceRef {
            state: canonical_source_state,
            source_ref,
        },
        divergence_state,
        checkpoint_lineage_ref: ROLLBACK_CHECKPOINT_REF.to_owned(),
        capture_mode,
        redaction_class,
        captured_body_ref,
        outcome,
        export_projection: TimelineExportProjection::metadata_safe_baseline(),
        copy_line: String::new(),
        captured_at: SEED_CAPTURED_AT.to_owned(),
        notes: notes.to_owned(),
    };
    entry.copy_line = timeline_copy_line(&entry);
    entry
}

fn binding(surface: TimelineSurface, consumer_ref: &str, summary: &str) -> TimelineSurfaceBinding {
    TimelineSurfaceBinding {
        surface,
        consumer_ref: consumer_ref.to_owned(),
        ingested_packet_id: GENERATED_TIMELINE_PACKET_ID.to_owned(),
        summary: summary.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Seeded packet.
// ---------------------------------------------------------------------------

/// Returns the checked-in generated-artifact timeline packet this lane freezes.
pub fn seeded_generated_timeline_packet() -> GeneratedTimelinePacket {
    let entries = vec![
        build_entry(
            "generated.timeline.scaffolded_project_full_snapshot",
            "Scaffolded file — full snapshot",
            ArtifactClass::ScaffoldedProject,
            CaptureMode::FullSnapshot,
            RedactionClass::Unredacted,
            CanonicalSourceState::Linked,
            DriftState::InSync,
            "A full, unredacted snapshot of the scaffolded file restores its exact bytes.",
        ),
        build_entry(
            "generated.timeline.notebook_output_metadata_plus_reference",
            "Notebook output — metadata plus reference",
            ArtifactClass::NotebookOutput,
            CaptureMode::MetadataPlusReference,
            RedactionClass::Unredacted,
            CanonicalSourceState::Linked,
            DriftState::InSync,
            "History stored metadata and a canonical-source reference; the output is rebuilt by re-running the cell rather than restored byte-for-byte.",
        ),
        build_entry(
            "generated.timeline.framework_codegen_regenerated_candidate",
            "Framework codegen — regenerated candidate",
            ArtifactClass::FrameworkCodegen,
            CaptureMode::RegeneratedCandidate,
            RedactionClass::Unredacted,
            CanonicalSourceState::Linked,
            DriftState::Drifting,
            "A regenerated candidate was captured against a drifting source, so restore is compatible but not exact.",
        ),
        build_entry(
            "generated.timeline.preview_derivative_omitted_bytes",
            "Preview derivative — omitted bytes",
            ArtifactClass::PreviewDerivative,
            CaptureMode::OmittedBytes,
            RedactionClass::SizeCapped,
            CanonicalSourceState::Linked,
            DriftState::InSync,
            "The oversized preview bundle was omitted from local history; only evidence remains.",
        ),
        build_entry(
            "generated.timeline.request_artifact_redacted_snapshot",
            "Request response — redacted snapshot",
            ArtifactClass::RequestArtifact,
            CaptureMode::FullSnapshot,
            RedactionClass::SecretsRedacted,
            CanonicalSourceState::Linked,
            DriftState::InSync,
            "A full snapshot was captured but secrets were redacted, so restore cannot claim exact byte continuity.",
        ),
        build_entry(
            "generated.timeline.support_packet_reference_source_missing",
            "Support packet — reference with missing source",
            ArtifactClass::SupportPacket,
            CaptureMode::MetadataPlusReference,
            RedactionClass::Unredacted,
            CanonicalSourceState::Missing,
            DriftState::SourceMissing,
            "History stored only a reference, but the canonical source is gone, so the bytes can no longer be regenerated.",
        ),
        build_entry(
            "generated.timeline.ai_assisted_edit_full_snapshot",
            "AI-assisted edit — full snapshot",
            ArtifactClass::AiAssistedEdit,
            CaptureMode::FullSnapshot,
            RedactionClass::Unredacted,
            CanonicalSourceState::Linked,
            DriftState::InSync,
            "The accepted AI edit was captured as a full snapshot and restores its exact bytes from the apply checkpoint.",
        ),
    ];

    let surface_bindings = vec![
        binding(
            TimelineSurface::HistoryTimeline,
            "crates/aureline-history/src/local_history/mod.rs",
            "The local-history timeline reads each entry's capture mode and restore fidelity so a metadata-plus-reference entry is never rendered as ordinary full-source history.",
        ),
        binding(
            TimelineSurface::CompareView,
            "crates/aureline-review/src/change_inspector/mod.rs",
            "The compare view picks the entry's compare basis, so a byte-snapshot diff is offered only when exact byte continuity holds and a regenerated-candidate compare otherwise.",
        ),
        binding(
            TimelineSurface::RestorePreview,
            "crates/aureline-recovery/src/lib.rs",
            "The restore preview honors the restore fidelity: an exact snapshot writes captured bytes, a compatible regeneration requires a review, and an evidence-only entry disables restore.",
        ),
        binding(
            TimelineSurface::SupportExport,
            "crates/aureline-support/src/generated_lineage/mod.rs",
            "The support export re-emits the entry's lineage links and copy line with no raw bodies, secret material, or live authority, so diagnostics cite one history object model.",
        ),
    ];

    GeneratedTimelinePacket {
        record_kind: GENERATED_TIMELINE_PACKET_RECORD_KIND.to_owned(),
        schema_version: GENERATED_TIMELINE_SCHEMA_VERSION,
        packet_id: GENERATED_TIMELINE_PACKET_ID.to_owned(),
        title: "Generated-artifact local-history, timeline, reversible-checkpoint, and export semantics".to_owned(),
        source_contract_refs: TimelineSourceContractRefs {
            doc_ref: GENERATED_TIMELINE_DOC_REF.to_owned(),
            schema_ref: GENERATED_TIMELINE_SCHEMA_REF.to_owned(),
            packet_ref: GENERATED_TIMELINE_PACKET_REF.to_owned(),
            report_ref: GENERATED_TIMELINE_REPORT_REF.to_owned(),
            fixture_manifest_ref: GENERATED_TIMELINE_FIXTURE_MANIFEST_REF.to_owned(),
        },
        consumer_surfaces: TimelineSurface::ALL.to_vec(),
        evidence_packet_refs: evidence_packet_refs(),
        entries,
        surface_bindings,
        invariants: vec![
            "Each generated-artifact timeline entry records its capture mode — full snapshot, metadata-plus-reference, regenerated candidate, or omitted bytes — and its redaction class explicitly, instead of implying ordinary full-source history.".to_owned(),
            "One engine folds capture mode, redaction class, and divergence state into a restore fidelity (exact snapshot, compatible regeneration, or evidence only), a byte-provenance explanation, a compare basis, a restore availability, and stable block-reason tokens.".to_owned(),
            "Exact generated-byte continuity is claimed only when the timeline captured a full, unredacted snapshot; a metadata-plus-reference, regenerated-candidate, omitted, or redacted capture never claims exact byte continuity on restore or compare.".to_owned(),
            "Every entry preserves its lineage links — generator identity, canonical source, divergence state, and reversible-checkpoint lineage — so compare, restore, export, and support flows cite one object model.".to_owned(),
            "The support/export projection stays metadata-safe and lineage-preserving: it keeps ids, generator identity, canonical-source and checkpoint refs, capture mode, and restore fidelity, and excludes raw bodies, secret material, and live authority.".to_owned(),
        ],
    }
}

/// Returns the checked-in timeline fixture corpus this lane freezes.
pub fn seeded_generated_timeline_fixtures() -> Vec<GeneratedTimelineEntryFixture> {
    vec![
        fixture(
            "fixture.generated_timeline.full_snapshot_exact",
            "Full snapshot restores exactly",
            build_entry(
                "generated.timeline.fixture.full_snapshot_exact",
                "Scaffolded file — full snapshot",
                ArtifactClass::ScaffoldedProject,
                CaptureMode::FullSnapshot,
                RedactionClass::Unredacted,
                CanonicalSourceState::Linked,
                DriftState::InSync,
                "A full, unredacted snapshot claims exact byte continuity and offers a byte-snapshot compare.",
            ),
            "crates/aureline-recovery/src/lib.rs",
            "A full unredacted snapshot is the only capture that claims exact byte continuity.",
        ),
        fixture(
            "fixture.generated_timeline.metadata_plus_reference_compatible",
            "Metadata-plus-reference never claims exact continuity",
            build_entry(
                "generated.timeline.fixture.metadata_plus_reference_compatible",
                "Notebook output — metadata plus reference",
                ArtifactClass::NotebookOutput,
                CaptureMode::MetadataPlusReference,
                RedactionClass::Unredacted,
                CanonicalSourceState::Linked,
                DriftState::InSync,
                "Stored metadata plus a reference yields a compatible regeneration; exact byte continuity is withheld.",
            ),
            "crates/aureline-history/src/local_history/mod.rs",
            "The guardrail: a metadata-plus-reference capture restores by regeneration and never claims exact byte continuity.",
        ),
        fixture(
            "fixture.generated_timeline.regenerated_candidate_drifting",
            "Regenerated candidate against a drifting source",
            build_entry(
                "generated.timeline.fixture.regenerated_candidate_drifting",
                "Framework codegen — regenerated candidate",
                ArtifactClass::FrameworkCodegen,
                CaptureMode::RegeneratedCandidate,
                RedactionClass::Unredacted,
                CanonicalSourceState::Linked,
                DriftState::Drifting,
                "A regenerated candidate is compatible only; drift against the source adds a regeneration block token.",
            ),
            "crates/aureline-review/src/change_inspector/mod.rs",
            "A regenerated candidate is compatible regeneration, and a drifting source is flagged.",
        ),
        fixture(
            "fixture.generated_timeline.omitted_bytes_evidence_only",
            "Omitted bytes leave evidence only",
            build_entry(
                "generated.timeline.fixture.omitted_bytes_evidence_only",
                "Preview derivative — omitted bytes",
                ArtifactClass::PreviewDerivative,
                CaptureMode::OmittedBytes,
                RedactionClass::SizeCapped,
                CanonicalSourceState::Linked,
                DriftState::InSync,
                "Omitted bytes leave an evidence-only entry with restore disabled.",
            ),
            "crates/aureline-support/src/generated_lineage/mod.rs",
            "Omitted bytes leave only evidence; restore is disabled and export-only.",
        ),
        fixture(
            "fixture.generated_timeline.redacted_full_snapshot_not_exact",
            "Redacted full snapshot drops exact continuity",
            build_entry(
                "generated.timeline.fixture.redacted_full_snapshot_not_exact",
                "Request response — redacted snapshot",
                ArtifactClass::RequestArtifact,
                CaptureMode::FullSnapshot,
                RedactionClass::SecretsRedacted,
                CanonicalSourceState::Linked,
                DriftState::InSync,
                "A redacted full snapshot is no longer faithful, so exact byte continuity is withheld.",
            ),
            "crates/aureline-recovery/src/lib.rs",
            "Redaction floors a full snapshot below exact: the stored bytes are not a faithful copy.",
        ),
        fixture(
            "fixture.generated_timeline.reference_source_missing_evidence_only",
            "Reference with a missing source falls to evidence only",
            build_entry(
                "generated.timeline.fixture.reference_source_missing_evidence_only",
                "Support packet — reference with missing source",
                ArtifactClass::SupportPacket,
                CaptureMode::MetadataPlusReference,
                RedactionClass::Unredacted,
                CanonicalSourceState::Missing,
                DriftState::SourceMissing,
                "A reference with no canonical source can no longer be regenerated, so the entry falls to evidence only.",
            ),
            "crates/aureline-support/src/generated_lineage/mod.rs",
            "A missing canonical source removes the regeneration path, narrowing a reference capture to evidence only.",
        ),
        fixture(
            "fixture.generated_timeline.policy_withheld_evidence_only",
            "Policy-withheld content leaves evidence only",
            build_entry(
                "generated.timeline.fixture.policy_withheld_evidence_only",
                "Request response — policy withheld",
                ArtifactClass::RequestArtifact,
                CaptureMode::MetadataPlusReference,
                RedactionClass::PolicyWithheld,
                CanonicalSourceState::Linked,
                DriftState::InSync,
                "Policy-withheld content leaves an evidence-only entry even though a reference exists.",
            ),
            "crates/aureline-support/src/generated_lineage/mod.rs",
            "A policy withholding floors the fidelity to evidence only regardless of capture mode.",
        ),
    ]
}

fn fixture(
    fixture_id: &str,
    scenario: &str,
    entry: GeneratedTimelineEntry,
    consumer_ref: &str,
    notes: &str,
) -> GeneratedTimelineEntryFixture {
    GeneratedTimelineEntryFixture {
        record_kind: GENERATED_TIMELINE_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: GENERATED_TIMELINE_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        scenario: scenario.to_owned(),
        expected_restore_fidelity: entry.outcome.restore_fidelity,
        expected_exact_byte_continuity_claimed: entry.outcome.exact_byte_continuity_claimed,
        expected_block_reason_tokens: entry.outcome.block_reason_tokens.clone(),
        entry,
        consumer_ref: consumer_ref.to_owned(),
        notes: notes.to_owned(),
    }
}

#[cfg(test)]
mod tests;
