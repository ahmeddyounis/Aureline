//! Canonical cross-surface parity and promotion truth for the imported
//! code-quality (SARIF/scanner) lane.
//!
//! The stable scanner-import lane in [`crate::scanner_import`] already normalizes
//! SARIF and structured scanner payloads into read-only imported findings, delta
//! packets, CI/local parity views, review packets, pipeline-viewer projections,
//! support exports, and release evidence. This module does not re-derive any of
//! that. Instead it publishes one inspectable, serde truth packet that states,
//! for the whole M5 imported-quality lane:
//!
//! - **Which product surfaces ingest imported scanner truth, and what each one
//!   guarantees?** A [`SurfaceParity`] row per surface (Problems, review,
//!   pipeline viewer, support bundle, release packet, CI/local parity, and CLI),
//!   each bound to the stable record-kind it consumes and asserting that imported
//!   results stay source-labeled, freshness-labeled, baseline-compatible, and
//!   read-only with an explicit downgrade behavior.
//! - **Which parity states are first-class rather than hidden caveats?** A
//!   [`ParityStateRow`] per state in the closed [`ParityStateClass`] vocabulary,
//!   so parity gaps, stale imports, unmapped rules, and unsupported scanner
//!   families are nameable states with defined delta and promotion semantics.
//! - **What must hold before any M5 scanner row promotes?** A [`PromotionGate`]
//!   that cannot waive delta compatibility, export-safe artifacts, or downgrade
//!   behavior, and whose `promotable` flag is recomputed from the surface and
//!   parity-state guarantees so weakening any of them flips the gate closed.
//!
//! The packet is checked in at
//! `artifacts/quality/m5/scanner-import-quality-parity.json` and embedded here so
//! this typed consumer and any CI gate agree on every row without a cargo build.
//! The model is metadata-only: it carries no raw scanner bodies, raw source, raw
//! paths, provider payloads, or secrets.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported scanner-import quality-parity packet schema version.
pub const SCANNER_IMPORT_QUALITY_PARITY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const SCANNER_IMPORT_QUALITY_PARITY_RECORD_KIND: &str = "scanner_import_quality_parity_record";

/// Repo-relative path to the checked-in packet.
pub const SCANNER_IMPORT_QUALITY_PARITY_PATH: &str =
    "artifacts/quality/m5/scanner-import-quality-parity.json";

/// Embedded checked-in packet JSON.
pub const SCANNER_IMPORT_QUALITY_PARITY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/quality/m5/scanner-import-quality-parity.json"
));

/// Scanner source format claimed by the imported-quality lane.
///
/// This is the closed set of scanner shapes the lane normalizes; broadening it is
/// explicitly out of scope for this row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParitySourceFormatClass {
    /// OASIS SARIF 2.1.0 JSON.
    #[serde(rename = "sarif_2_1_0")]
    Sarif21,
    /// Structured scanner JSON normalized through the same import model.
    StructuredScannerJson,
}

impl ParitySourceFormatClass {
    /// Every supported source format, in declaration order.
    pub const ALL: [Self; 2] = [Self::Sarif21, Self::StructuredScannerJson];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sarif21 => "sarif_2_1_0",
            Self::StructuredScannerJson => "structured_scanner_json",
        }
    }
}

/// A product surface that ingests imported scanner truth in the M5 lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// The Problems panel projection.
    Problems,
    /// The review workspace diagnostic review packet.
    ReviewWorkspace,
    /// The pipeline/run viewer projection.
    PipelineViewer,
    /// The support-export bundle.
    SupportBundle,
    /// The release evidence packet.
    ReleasePacket,
    /// The CI/local parity view.
    CiLocalParity,
    /// The CLI/headless projection.
    Cli,
}

impl SurfaceClass {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Problems,
        Self::ReviewWorkspace,
        Self::PipelineViewer,
        Self::SupportBundle,
        Self::ReleasePacket,
        Self::CiLocalParity,
        Self::Cli,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Problems => "problems",
            Self::ReviewWorkspace => "review_workspace",
            Self::PipelineViewer => "pipeline_viewer",
            Self::SupportBundle => "support_bundle",
            Self::ReleasePacket => "release_packet",
            Self::CiLocalParity => "ci_local_parity",
            Self::Cli => "cli",
        }
    }
}

/// Degraded behavior a surface guarantees when imported parity is not exact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeBehaviorClass {
    /// Imported rows stay read-only and labeled as imported evidence.
    KeepImportedReadOnlyLabeled,
    /// Exact-delta or live-parity claims are blocked.
    BlockExactDeltaClaim,
    /// Mutation or confirmation claims require a compatible local run first.
    RequireLocalConfirmation,
    /// Raw-payload backlinks are redacted or omitted by policy.
    RedactRawPayload,
    /// Promotion of the imported run as live truth is blocked.
    BlockPromotion,
}

impl DowngradeBehaviorClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeepImportedReadOnlyLabeled => "keep_imported_read_only_labeled",
            Self::BlockExactDeltaClaim => "block_exact_delta_claim",
            Self::RequireLocalConfirmation => "require_local_confirmation",
            Self::RedactRawPayload => "redact_raw_payload",
            Self::BlockPromotion => "block_promotion",
        }
    }
}

/// First-class parity state for the imported-quality lane.
///
/// Every gap is a nameable state with defined delta and promotion semantics so a
/// parity gap is surfaced rather than buried as a hidden caveat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityStateClass {
    /// Imported and local evidence are comparable.
    Comparable,
    /// Comparable only once a compatible local run confirms the family.
    ComparableRequiresLocalConfirmation,
    /// Imported evidence is stale relative to the current target.
    ParityGapStaleImported,
    /// An imported rule has no admitted local mapping or anchor.
    ParityGapUnmappedRule,
    /// Rule-pack drift blocks comparison.
    ParityGapRulePackMismatch,
    /// Profile or tool drift blocks comparison.
    ParityGapProfileMismatch,
    /// The scanner family is outside the supported import lanes.
    UnsupportedScannerFamily,
    /// The evidence source stays distinct and is not comparable.
    DistinctSourceNotComparable,
}

impl ParityStateClass {
    /// Every parity state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Comparable,
        Self::ComparableRequiresLocalConfirmation,
        Self::ParityGapStaleImported,
        Self::ParityGapUnmappedRule,
        Self::ParityGapRulePackMismatch,
        Self::ParityGapProfileMismatch,
        Self::UnsupportedScannerFamily,
        Self::DistinctSourceNotComparable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Comparable => "comparable",
            Self::ComparableRequiresLocalConfirmation => "comparable_requires_local_confirmation",
            Self::ParityGapStaleImported => "parity_gap_stale_imported",
            Self::ParityGapUnmappedRule => "parity_gap_unmapped_rule",
            Self::ParityGapRulePackMismatch => "parity_gap_rule_pack_mismatch",
            Self::ParityGapProfileMismatch => "parity_gap_profile_mismatch",
            Self::UnsupportedScannerFamily => "unsupported_scanner_family",
            Self::DistinctSourceNotComparable => "distinct_source_not_comparable",
        }
    }

    /// Canonical truth: whether this state is a parity gap.
    pub const fn is_gap(self) -> bool {
        !matches!(
            self,
            Self::Comparable | Self::ComparableRequiresLocalConfirmation
        )
    }

    /// Canonical truth: whether this state blocks an exact-delta claim.
    pub const fn blocks_exact_delta(self) -> bool {
        !matches!(self, Self::Comparable)
    }

    /// Canonical truth: whether this state blocks promoting the import as live.
    ///
    /// Stale and unmapped gaps are recoverable by a local rerun or confirmation,
    /// so they downgrade rather than hard-block; rule-pack, profile, unsupported,
    /// and distinct-source gaps cannot be confirmed away and block promotion.
    pub const fn blocks_promotion(self) -> bool {
        matches!(
            self,
            Self::ParityGapRulePackMismatch
                | Self::ParityGapProfileMismatch
                | Self::UnsupportedScannerFamily
                | Self::DistinctSourceNotComparable
        )
    }
}

/// Parity guarantees for one M5 surface that ingests imported scanner truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceParity {
    /// Surface this row speaks for.
    pub surface_class: SurfaceClass,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Stable record-kind this surface consumes from the scanner-import lane.
    pub record_kind_ref: String,
    /// True when imported rows stay source-labeled on this surface.
    pub source_labeled: bool,
    /// True when imported rows stay freshness-labeled on this surface.
    pub freshness_labeled: bool,
    /// True when this surface preserves baseline/delta compatibility state.
    pub baseline_compatible: bool,
    /// True when imported rows remain read-only inspect-only evidence here.
    pub imported_read_only: bool,
    /// True when this surface threads diff-scoped imported findings.
    pub threads_diff_scoped_findings: bool,
    /// True when this surface threads suppression and waiver state.
    pub threads_suppressions: bool,
    /// True when this surface threads baseline shifts (new/resolved/persisting).
    pub threads_baseline_shifts: bool,
    /// Downgrade behaviors this surface guarantees when parity is not exact.
    #[serde(default)]
    pub downgrade_behaviors: Vec<DowngradeBehaviorClass>,
    /// Export-safe surface summary.
    pub summary: String,
}

impl SurfaceParity {
    /// True when imported evidence is fully labeled and inspect-only here.
    pub const fn keeps_imported_truth(&self) -> bool {
        self.source_labeled && self.freshness_labeled && self.imported_read_only
    }
}

/// First-class parity-state row with defined delta and promotion semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParityStateRow {
    /// Parity state this row defines.
    pub parity_state_class: ParityStateClass,
    /// True when this state is a parity gap.
    pub is_gap: bool,
    /// True when this state blocks an exact-delta claim.
    pub blocks_exact_delta: bool,
    /// True when this state blocks promoting the import as live truth.
    pub blocks_promotion: bool,
    /// Remediation action ref offered for a recoverable gap, when any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remediation_action_ref: Option<String>,
    /// Export-safe state summary.
    pub summary: String,
}

/// Promotion gate that guards M5 imported-quality rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionGate {
    /// Stable gate id.
    pub gate_id: String,
    /// True because delta compatibility is a non-waivable promotion requirement.
    pub requires_delta_compatibility: bool,
    /// True because export-safe artifacts are a non-waivable promotion requirement.
    pub requires_export_safe_artifacts: bool,
    /// True because defined downgrade behavior is a non-waivable promotion requirement.
    pub requires_downgrade_behavior: bool,
    /// True because mirror-only/air-gapped imports require a signed import/export.
    pub requires_signed_import_on_restricted_profiles: bool,
    /// True when the lane satisfies every promotion requirement.
    pub promotable: bool,
    /// Reasons promotion is blocked; empty exactly when `promotable` is true.
    #[serde(default)]
    pub blocking_reasons: Vec<String>,
    /// Export-safe gate summary.
    pub summary: String,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScannerImportQualityParitySummary {
    /// Total surfaces materialized.
    pub total_surfaces: usize,
    /// Surfaces that keep imported rows source-labeled.
    pub source_labeled_surfaces: usize,
    /// Surfaces that keep imported rows freshness-labeled.
    pub freshness_labeled_surfaces: usize,
    /// Surfaces that preserve baseline/delta compatibility state.
    pub baseline_compatible_surfaces: usize,
    /// Surfaces that keep imported rows read-only.
    pub read_only_surfaces: usize,
    /// Total parity states materialized.
    pub total_parity_states: usize,
    /// Parity states that are gaps.
    pub gap_states: usize,
    /// Parity states that block promotion.
    pub promotion_blocking_states: usize,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerImportQualityParityExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Surface tokens that keep imported truth labeled and read-only.
    pub labeled_read_only_surfaces: Vec<String>,
    /// Parity-gap state tokens that are first-class in the lane.
    pub gap_state_tokens: Vec<String>,
    /// True when the lane is promotable under the gate.
    pub promotable: bool,
    /// True because no raw scanner bodies, source, paths, or secrets are projected.
    pub redaction_safe: bool,
}

/// The typed scanner-import quality-parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScannerImportQualityParity {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed scanner-source-format vocabulary.
    pub source_format_classes: Vec<ParitySourceFormatClass>,
    /// Per-surface parity guarantees.
    #[serde(default)]
    pub surfaces: Vec<SurfaceParity>,
    /// First-class parity-state rows.
    #[serde(default)]
    pub parity_states: Vec<ParityStateRow>,
    /// Promotion gate for M5 imported-quality rows.
    pub promotion_gate: PromotionGate,
    /// Summary counts.
    pub summary: ScannerImportQualityParitySummary,
}

impl ScannerImportQualityParity {
    /// Returns the parity row for `surface`, when present.
    pub fn surface(&self, surface: SurfaceClass) -> Option<&SurfaceParity> {
        self.surfaces.iter().find(|s| s.surface_class == surface)
    }

    /// Returns the parity-state row for `state`, when present.
    pub fn parity_state(&self, state: ParityStateClass) -> Option<&ParityStateRow> {
        self.parity_states
            .iter()
            .find(|row| row.parity_state_class == state)
    }

    /// Recomputes whether the lane is promotable from the structural guarantees.
    ///
    /// The lane is promotable only when every surface keeps imported truth
    /// labeled, read-only, baseline-compatible, and backed by a downgrade
    /// behavior, the full parity-state vocabulary is materialized, and the gate
    /// keeps every non-waivable requirement set.
    pub fn computed_promotable(&self) -> bool {
        let gate = &self.promotion_gate;
        let requirements_held = gate.requires_delta_compatibility
            && gate.requires_export_safe_artifacts
            && gate.requires_downgrade_behavior
            && gate.requires_signed_import_on_restricted_profiles;
        let surfaces_complete = SurfaceClass::ALL
            .iter()
            .all(|surface| self.surface(*surface).is_some());
        let surfaces_hold = self.surfaces.iter().all(|surface| {
            surface.keeps_imported_truth()
                && surface.baseline_compatible
                && !surface.downgrade_behaviors.is_empty()
        });
        let states_complete = ParityStateClass::ALL
            .iter()
            .all(|state| self.parity_state(*state).is_some());
        requirements_held && surfaces_complete && surfaces_hold && states_complete
    }

    /// Recomputes the summary block from the surfaces and parity states.
    pub fn computed_summary(&self) -> ScannerImportQualityParitySummary {
        ScannerImportQualityParitySummary {
            total_surfaces: self.surfaces.len(),
            source_labeled_surfaces: self.surfaces.iter().filter(|s| s.source_labeled).count(),
            freshness_labeled_surfaces: self
                .surfaces
                .iter()
                .filter(|s| s.freshness_labeled)
                .count(),
            baseline_compatible_surfaces: self
                .surfaces
                .iter()
                .filter(|s| s.baseline_compatible)
                .count(),
            read_only_surfaces: self
                .surfaces
                .iter()
                .filter(|s| s.imported_read_only)
                .count(),
            total_parity_states: self.parity_states.len(),
            gap_states: self.parity_states.iter().filter(|row| row.is_gap).count(),
            promotion_blocking_states: self
                .parity_states
                .iter()
                .filter(|row| row.blocks_promotion)
                .count(),
        }
    }

    /// Produces a redaction-safe export projection for support/release ingest.
    pub fn export_projection(&self) -> ScannerImportQualityParityExportProjection {
        ScannerImportQualityParityExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            labeled_read_only_surfaces: self
                .surfaces
                .iter()
                .filter(|s| s.keeps_imported_truth())
                .map(|s| s.surface_class.as_str().to_owned())
                .collect(),
            gap_state_tokens: self
                .parity_states
                .iter()
                .filter(|row| row.is_gap)
                .map(|row| row.parity_state_class.as_str().to_owned())
                .collect(),
            promotable: self.promotion_gate.promotable,
            redaction_safe: true,
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<ScannerImportQualityParityViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_surfaces(&mut violations);
        self.validate_parity_states(&mut violations);
        self.validate_gate(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(ScannerImportQualityParityViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<ScannerImportQualityParityViolation>) {
        if self.schema_version != SCANNER_IMPORT_QUALITY_PARITY_SCHEMA_VERSION {
            violations.push(
                ScannerImportQualityParityViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != SCANNER_IMPORT_QUALITY_PARITY_RECORD_KIND {
            violations.push(ScannerImportQualityParityViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(ScannerImportQualityParityViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.source_format_classes != ParitySourceFormatClass::ALL.to_vec() {
            violations.push(
                ScannerImportQualityParityViolation::ClosedVocabularyMismatch {
                    field: "source_format_classes",
                },
            );
        }
    }

    fn validate_surfaces(&self, violations: &mut Vec<ScannerImportQualityParityViolation>) {
        let mut seen = BTreeSet::new();
        for surface in &self.surfaces {
            if !seen.insert(surface.surface_class) {
                violations.push(ScannerImportQualityParityViolation::DuplicateSurface {
                    surface: surface.surface_class.as_str(),
                });
            }
            let id = surface.surface_class.as_str();
            for (field, value) in [
                ("surface_label", &surface.surface_label),
                ("record_kind_ref", &surface.record_kind_ref),
                ("summary", &surface.summary),
            ] {
                if value.trim().is_empty() {
                    violations.push(ScannerImportQualityParityViolation::EmptyField {
                        id: id.to_owned(),
                        field_name: field,
                    });
                }
            }
            // Acceptance: imported results stay source-labeled, freshness-labeled,
            // and read-only on every surface; they never read as live truth.
            if !surface.keeps_imported_truth() {
                violations.push(
                    ScannerImportQualityParityViolation::UnlabeledImportedSurface { surface: id },
                );
            }
            if !surface.baseline_compatible {
                violations.push(
                    ScannerImportQualityParityViolation::SurfaceDropsBaselineState { surface: id },
                );
            }
            // Acceptance: downgrade behavior is first-class, never an implicit gap.
            if surface.downgrade_behaviors.is_empty() {
                violations.push(
                    ScannerImportQualityParityViolation::SurfaceMissingDowngrade { surface: id },
                );
            }
        }
        for surface in SurfaceClass::ALL {
            if !seen.contains(&surface) {
                violations.push(ScannerImportQualityParityViolation::MissingSurface {
                    surface: surface.as_str(),
                });
            }
        }
    }

    fn validate_parity_states(&self, violations: &mut Vec<ScannerImportQualityParityViolation>) {
        let mut seen = BTreeSet::new();
        for row in &self.parity_states {
            if !seen.insert(row.parity_state_class) {
                violations.push(ScannerImportQualityParityViolation::DuplicateParityState {
                    state: row.parity_state_class.as_str(),
                });
            }
            if row.summary.trim().is_empty() {
                violations.push(ScannerImportQualityParityViolation::EmptyField {
                    id: row.parity_state_class.as_str().to_owned(),
                    field_name: "summary",
                });
            }
            // Row flags must match the canonical parity semantics so a gap can
            // never be quietly recorded as comparable, or vice versa.
            let class = row.parity_state_class;
            if row.is_gap != class.is_gap()
                || row.blocks_exact_delta != class.blocks_exact_delta()
                || row.blocks_promotion != class.blocks_promotion()
            {
                violations.push(
                    ScannerImportQualityParityViolation::ParityStateFlagMismatch {
                        state: class.as_str(),
                    },
                );
            }
        }
        for state in ParityStateClass::ALL {
            if !seen.contains(&state) {
                violations.push(ScannerImportQualityParityViolation::MissingParityState {
                    state: state.as_str(),
                });
            }
        }
    }

    fn validate_gate(&self, violations: &mut Vec<ScannerImportQualityParityViolation>) {
        let gate = &self.promotion_gate;
        if gate.gate_id.trim().is_empty() || gate.summary.trim().is_empty() {
            violations.push(ScannerImportQualityParityViolation::EmptyField {
                id: "<promotion_gate>".to_owned(),
                field_name: "gate_id_or_summary",
            });
        }
        // Acceptance: no M5 scanner row promotes while waiving delta compatibility,
        // export-safe artifacts, or downgrade behavior. The gate cannot turn these
        // off; doing so is a hard violation rather than a softer "not promotable".
        for (held, requirement) in [
            (
                gate.requires_delta_compatibility,
                "requires_delta_compatibility",
            ),
            (
                gate.requires_export_safe_artifacts,
                "requires_export_safe_artifacts",
            ),
            (
                gate.requires_downgrade_behavior,
                "requires_downgrade_behavior",
            ),
            (
                gate.requires_signed_import_on_restricted_profiles,
                "requires_signed_import_on_restricted_profiles",
            ),
        ] {
            if !held {
                violations.push(
                    ScannerImportQualityParityViolation::PromotionRequirementWaived { requirement },
                );
            }
        }
        // The recorded `promotable` flag must agree with the recomputed guarantees,
        // and blocking reasons must be present exactly when it is not promotable.
        if gate.promotable != self.computed_promotable() {
            violations.push(ScannerImportQualityParityViolation::PromotionFlagMismatch);
        }
        if gate.promotable != gate.blocking_reasons.is_empty() {
            violations.push(ScannerImportQualityParityViolation::PromotionReasonsInconsistent);
        }
    }
}

/// A validation violation for the scanner-import quality-parity packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScannerImportQualityParityViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary is not the canonical value.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row, surface, or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A surface appears more than once.
    DuplicateSurface {
        /// Duplicate surface token.
        surface: &'static str,
    },
    /// A surface is missing from the packet.
    MissingSurface {
        /// Missing surface token.
        surface: &'static str,
    },
    /// A surface does not keep imported rows source/freshness-labeled and read-only.
    UnlabeledImportedSurface {
        /// Offending surface token.
        surface: &'static str,
    },
    /// A surface drops baseline/delta compatibility state.
    SurfaceDropsBaselineState {
        /// Offending surface token.
        surface: &'static str,
    },
    /// A surface declares no downgrade behavior.
    SurfaceMissingDowngrade {
        /// Offending surface token.
        surface: &'static str,
    },
    /// A parity state appears more than once.
    DuplicateParityState {
        /// Duplicate state token.
        state: &'static str,
    },
    /// A parity state is missing from the packet.
    MissingParityState {
        /// Missing state token.
        state: &'static str,
    },
    /// A parity-state row's flags disagree with its canonical semantics.
    ParityStateFlagMismatch {
        /// Offending state token.
        state: &'static str,
    },
    /// The promotion gate waives a non-waivable requirement.
    PromotionRequirementWaived {
        /// Waived requirement name.
        requirement: &'static str,
    },
    /// The recorded `promotable` flag disagrees with the recomputed guarantees.
    PromotionFlagMismatch,
    /// Blocking reasons disagree with the `promotable` flag.
    PromotionReasonsInconsistent,
    /// The summary counts disagree with the surfaces and parity states.
    SummaryMismatch,
}

impl fmt::Display for ScannerImportQualityParityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateSurface { surface } => {
                write!(f, "duplicate surface {surface}")
            }
            Self::MissingSurface { surface } => {
                write!(f, "missing surface {surface}")
            }
            Self::UnlabeledImportedSurface { surface } => {
                write!(
                    f,
                    "surface {surface} does not keep imported rows source/freshness-labeled and read-only"
                )
            }
            Self::SurfaceDropsBaselineState { surface } => {
                write!(
                    f,
                    "surface {surface} drops baseline/delta compatibility state"
                )
            }
            Self::SurfaceMissingDowngrade { surface } => {
                write!(f, "surface {surface} declares no downgrade behavior")
            }
            Self::DuplicateParityState { state } => {
                write!(f, "duplicate parity state {state}")
            }
            Self::MissingParityState { state } => {
                write!(f, "missing parity state {state}")
            }
            Self::ParityStateFlagMismatch { state } => {
                write!(
                    f,
                    "parity state {state} flags disagree with its canonical semantics"
                )
            }
            Self::PromotionRequirementWaived { requirement } => {
                write!(
                    f,
                    "promotion gate waives the non-waivable requirement {requirement}"
                )
            }
            Self::PromotionFlagMismatch => {
                write!(
                    f,
                    "promotion gate promotable flag disagrees with the recomputed guarantees"
                )
            }
            Self::PromotionReasonsInconsistent => {
                write!(
                    f,
                    "promotion gate blocking reasons disagree with the promotable flag"
                )
            }
            Self::SummaryMismatch => {
                write!(
                    f,
                    "packet summary counts disagree with the materializations"
                )
            }
        }
    }
}

impl Error for ScannerImportQualityParityViolation {}

/// Loads the embedded scanner-import quality-parity packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`ScannerImportQualityParity`].
pub fn current_scanner_import_quality_parity(
) -> Result<ScannerImportQualityParity, serde_json::Error> {
    serde_json::from_str(SCANNER_IMPORT_QUALITY_PARITY_JSON)
}

#[cfg(test)]
mod tests;
