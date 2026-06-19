//! Typed M5 interchange-conformance register: the conformance layer that proves every
//! named M5 import/export family survives real product use across the desktop, CLI/headless,
//! and support/export surfaces.
//!
//! M5 ships many versioned import/export families. This register binds each high-value
//! family — request/API collections, notebook paired/parity exports, docs packets,
//! trace/profile/replay captures, support bundles, and portable-state packages — to:
//!
//! - the import/export [`Validator`] that guards it and the stable, copy-safe [`ReasonCode`]s
//!   an interchange failure reports instead of a raw parser exception,
//! - the cross-surface conformance [`Runner`] that exercises a real emitted artifact,
//! - the [`ConsumerAgreement`] that records the contract version, lifecycle label, and
//!   degraded-state vocabulary the [`ConsumerSurface`]s must agree on, and
//! - the per-dimension [`Dimension`] cells (one per [`DimensionKind`]) and the
//!   [`ConformanceState`] and [`DecisionState`] those cells produce.
//!
//! A catalog-linked family's [`ConformanceRow::lifecycle_label`] equals the published
//! contract family's label, so an interchange claim can never run ahead of the contract.
//! `compare_only` and `import_validation_only` are first-class [`ConformanceClass`]es: a
//! family the source docs scope to compare/inspect behavior is not forced to support
//! write-back. A release-blocking family with a failing required dimension sets the
//! register's promotion decision to [`DecisionState::Hold`].
//!
//! The register is checked in at `artifacts/contracts/m5-interchange-conformance.json` and
//! embedded here, so this typed consumer and the CI validator agree on every family and
//! dimension without a cargo build in CI. The model is metadata-plus-state only: every field
//! is a typed state, an opaque repo-relative ref or URI, or a copy/export-safe summary. It
//! carries no credential bodies or raw provider payloads.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Supported register schema version.
pub const M5_INTERCHANGE_CONFORMANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_INTERCHANGE_CONFORMANCE_RECORD_KIND: &str = "m5_interchange_conformance_register";

/// Stable register identifier.
pub const M5_INTERCHANGE_CONFORMANCE_REGISTER_ID: &str = "m5_interchange_conformance:v1";

/// Repo-relative path to the checked-in register.
pub const M5_INTERCHANGE_CONFORMANCE_PATH: &str =
    "artifacts/contracts/m5-interchange-conformance.json";

/// Embedded checked-in register JSON.
pub const M5_INTERCHANGE_CONFORMANCE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/contracts/m5-interchange-conformance.json"
));

/// The lifecycle/stability label a family publishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleLabel {
    /// Long-term-stable.
    Lts,
    /// Stable.
    Stable,
    /// Beta.
    Beta,
    /// Preview.
    Preview,
    /// Withdrawn.
    Withdrawn,
}

impl LifecycleLabel {
    /// Every label, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Lts,
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Withdrawn,
    ];
}

/// The interchange direction a family proves end-to-end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterchangeDirection {
    /// Export only (no import path proven).
    ExportOnly,
    /// Export plus import-validation (no write-back).
    ImportValidation,
    /// Full export/import round-trip.
    RoundTrip,
}

impl InterchangeDirection {
    /// Every direction, in declaration order.
    pub const ALL: [Self; 3] = [Self::ExportOnly, Self::ImportValidation, Self::RoundTrip];
}

/// The conformance class. `compare_only` and `import_validation_only` are first-class valid
/// classes, not downgrades.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConformanceClass {
    /// Full round-trip with write-back.
    RoundTripWriteBack,
    /// Import-validation only; no write-back.
    ImportValidationOnly,
    /// Compare-only / inspect-only by design.
    CompareOnly,
}

impl ConformanceClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::RoundTripWriteBack,
        Self::ImportValidationOnly,
        Self::CompareOnly,
    ];

    /// True when this class requires a lossless round-trip with write-back.
    pub fn requires_write_back(self) -> bool {
        matches!(self, Self::RoundTripWriteBack)
    }
}

/// A consumer surface that must agree on contract version, lifecycle label, and degraded-state
/// vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// The desktop application.
    Desktop,
    /// The CLI/headless surface.
    CliHeadless,
    /// The support/export surface.
    SupportExport,
}

impl ConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 3] = [Self::Desktop, Self::CliHeadless, Self::SupportExport];
}

/// A conformance dimension a family is scored on (one cell per kind).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DimensionKind {
    /// A real emitted artifact is present and exercised by the runner.
    EmittedArtifactPresent,
    /// An import/export validator descriptor is wired.
    ImportExportValidator,
    /// The round-trip or declared compare-only behavior is proven.
    RoundTripOrCompare,
    /// Required provenance is preserved.
    ProvenancePreserved,
    /// Trust is not silently widened on import.
    TrustNotWidened,
    /// Desktop, CLI/headless, and support/export consumers agree.
    CrossSurfaceAgreement,
    /// Failures map to stable, copy-safe reason codes.
    StableReasonCodes,
}

impl DimensionKind {
    /// Every dimension kind, in evaluation order.
    pub const ALL: [Self; 7] = [
        Self::EmittedArtifactPresent,
        Self::ImportExportValidator,
        Self::RoundTripOrCompare,
        Self::ProvenancePreserved,
        Self::TrustNotWidened,
        Self::CrossSurfaceAgreement,
        Self::StableReasonCodes,
    ];
}

/// The outcome of evaluating one dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DimensionOutcome {
    /// The dimension is proven.
    Pass,
    /// The dimension is partially proven; the family narrows.
    Downgrade,
    /// The dimension is not proven; the family is held.
    Fail,
}

impl DimensionOutcome {
    /// Every outcome, in declaration order.
    pub const ALL: [Self; 3] = [Self::Pass, Self::Downgrade, Self::Fail];
}

/// A family's overall interchange-conformance state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConformanceState {
    /// Every required dimension passes.
    Conformant,
    /// A required dimension downgraded, or the family was narrowed.
    Narrowed,
    /// A release-blocking family has a failing required dimension.
    Failed,
}

impl ConformanceState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Conformant, Self::Narrowed, Self::Failed];
}

/// The promotion decision for a family or the whole register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionState {
    /// Release-clear.
    Clear,
    /// Promotion held.
    Hold,
}

impl DecisionState {
    /// Every decision, in declaration order.
    pub const ALL: [Self; 2] = [Self::Clear, Self::Hold];
}

/// The shared degraded-state vocabulary the consumers agree on. A degraded outcome is a
/// stable user-facing state, never an error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedState {
    /// A partial outcome.
    Partial,
    /// A field or section is not provided.
    NotProvided,
    /// Compare-only behavior.
    CompareOnly,
    /// Degraded behavior.
    Degraded,
    /// Temporarily unavailable.
    Unavailable,
}

impl DegradedState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Partial,
        Self::NotProvided,
        Self::CompareOnly,
        Self::Degraded,
        Self::Unavailable,
    ];
}

/// A stable, copy-safe import/export reason code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasonCode {
    /// The artifact declares an unsupported contract version.
    UnsupportedContractVersion,
    /// The artifact is missing required provenance.
    MissingRequiredProvenance,
    /// The artifact does not match the contract schema.
    SchemaValidationFailed,
    /// Importing the artifact would widen trust; the import is blocked.
    TrustWideningBlocked,
    /// Re-export after import did not reproduce the artifact.
    RoundTripMismatch,
    /// The artifact is truncated or corrupt.
    CorruptOrTruncatedPayload,
    /// The artifact carries fields a round-trip would drop.
    UnknownFieldUnpreserved,
    /// The artifact's redaction class conflicts with the destination policy.
    RedactionClassConflict,
}

impl ReasonCode {
    /// Every reason code, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::UnsupportedContractVersion,
        Self::MissingRequiredProvenance,
        Self::SchemaValidationFailed,
        Self::TrustWideningBlocked,
        Self::RoundTripMismatch,
        Self::CorruptOrTruncatedPayload,
        Self::UnknownFieldUnpreserved,
        Self::RedactionClassConflict,
    ];
}

/// The import/export validator descriptor reference carried on a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Validator {
    /// Stable validator id.
    pub validator_id: String,
    /// Repo-relative ref to the per-family validator descriptor.
    pub descriptor_ref: String,
    /// The validator kind.
    pub kind: String,
    /// The reason codes this validator can report.
    pub reason_codes_emitted: Vec<ReasonCode>,
}

/// The cross-surface conformance runner that exercises a real emitted artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Runner {
    /// Stable runner id.
    pub runner_id: String,
    /// Repo-relative ref to the real emitted artifact.
    pub artifact_ref: String,
    /// The emitted-artifact envelope record kind.
    pub artifact_record_kind: String,
    /// The family-specific emitted record kind.
    pub emitted_record_kind: String,
    /// The consumer surfaces this runner exercises.
    pub surfaces_exercised: Vec<ConsumerSurface>,
    /// The runner result.
    pub result: DimensionOutcome,
}

/// The cross-surface consumer agreement for a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConsumerAgreement {
    /// The consumer surfaces that agree.
    pub surfaces: Vec<ConsumerSurface>,
    /// The agreed contract version.
    pub agreed_contract_version: u32,
    /// The agreed lifecycle label.
    pub agreed_lifecycle_label: LifecycleLabel,
    /// The agreed degraded-state vocabulary.
    pub agreed_degraded_states: Vec<DegradedState>,
    /// Whether the surfaces agree.
    pub agrees: bool,
}

/// One evaluated conformance dimension for a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dimension {
    /// The dimension kind.
    pub dimension_kind: DimensionKind,
    /// Whether the dimension is required for this family.
    pub required: bool,
    /// The evaluated outcome.
    pub outcome: DimensionOutcome,
    /// Evidence refs the dimension read.
    pub evidence_refs: Vec<String>,
    /// Human-readable detail.
    pub detail: String,
}

impl Dimension {
    /// True when this dimension fails and is required.
    pub fn is_required_failure(&self) -> bool {
        self.required && self.outcome == DimensionOutcome::Fail
    }

    /// True when this dimension downgrades and is required.
    pub fn is_required_downgrade(&self) -> bool {
        self.required && self.outcome == DimensionOutcome::Downgrade
    }
}

/// One interchange-conformance row: a family and its validator, runner, consumer agreement,
/// dimensions, and conformance decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConformanceRow {
    /// Stable family id.
    pub family_id: String,
    /// Human-readable title.
    pub title: String,
    /// Human-readable summary.
    pub summary: String,
    /// The owning crate or lane.
    pub owning_package: String,
    /// The contract form (catalog lexicon).
    pub contract_form: String,
    /// The interchange direction proven.
    pub interchange_direction: InterchangeDirection,
    /// The conformance class.
    pub conformance_class: ConformanceClass,
    /// The lifecycle label the family is put forward at.
    pub claim_label: LifecycleLabel,
    /// The lifecycle label this register publishes (equals the catalog's for linked families).
    pub lifecycle_label: LifecycleLabel,
    /// Whether the family narrows below its claim label.
    pub narrowed: bool,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The resolvable contract version.
    pub contract_version: u32,
    /// The in-band contract-version field the emitted artifact carries.
    pub contract_version_field: String,
    /// The import/export validator descriptor reference.
    pub validator: Validator,
    /// The cross-surface conformance runner.
    pub runner: Runner,
    /// The cross-surface consumer agreement.
    pub consumer_agreement: ConsumerAgreement,
    /// The evaluated dimensions (one per dimension kind, in order).
    pub dimensions: Vec<Dimension>,
    /// The degraded states this family supports.
    pub degraded_states_supported: Vec<DegradedState>,
    /// The reason codes this family currently raises (empty when conformant).
    pub active_reason_codes: Vec<ReasonCode>,
    /// The linked contract-catalog family id, or empty when not linked.
    pub catalog_family_id: String,
    /// Ref to the contract-catalog entry (present only for linked families).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog_entry_ref: Option<String>,
    /// Ref to the publication-matrix row (present only for linked families).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix_row_ref: Option<String>,
    /// The overall conformance state.
    pub conformance_state: ConformanceState,
    /// The promotion decision.
    pub decision: DecisionState,
}

impl ConformanceRow {
    /// Recomputes the conformance state from the dimensions, narrowing, and blocking flags.
    pub fn computed_conformance_state(&self) -> ConformanceState {
        let any_required_fail = self.dimensions.iter().any(Dimension::is_required_failure);
        let any_required_downgrade = self.dimensions.iter().any(Dimension::is_required_downgrade);
        if self.release_blocking && any_required_fail {
            ConformanceState::Failed
        } else if self.narrowed || any_required_downgrade || any_required_fail {
            ConformanceState::Narrowed
        } else {
            ConformanceState::Conformant
        }
    }

    /// The decision implied by the computed conformance state.
    pub fn computed_decision(&self) -> DecisionState {
        if self.computed_conformance_state() == ConformanceState::Failed {
            DecisionState::Hold
        } else {
            DecisionState::Clear
        }
    }
}

/// Top-level promotion blocker summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Blockers {
    /// The promotion decision.
    pub decision: DecisionState,
    /// Families holding promotion.
    pub blocking_family_ids: Vec<String>,
    /// Dimension kinds failing on blocking families.
    pub blocking_dimension_kinds: Vec<DimensionKind>,
    /// Families that are narrowed.
    pub narrowed_family_ids: Vec<String>,
    /// Human-readable rationale.
    pub rationale: String,
}

/// Summary counts over the family set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5InterchangeConformanceSummary {
    /// Total families.
    pub total_families: usize,
    /// Release-blocking families.
    pub release_blocking_families: usize,
    /// Conformant families.
    pub conformant_families: usize,
    /// Narrowed families.
    pub narrowed_families: usize,
    /// Failed families.
    pub failed_families: usize,
    /// Families whose decision is hold.
    pub families_held: usize,
    /// Families linked to a published contract family.
    pub catalog_linked_families: usize,
    /// Round-trip-write-back families.
    pub round_trip_families: usize,
    /// Compare-only / import-validation-only families.
    pub compare_or_validate_only_families: usize,
    /// Total dimension evaluations.
    pub total_dimensions_evaluated: usize,
    /// Passing dimension evaluations.
    pub dimensions_passing: usize,
    /// Downgrading dimension evaluations.
    pub dimensions_downgrading: usize,
    /// Failing dimension evaluations.
    pub dimensions_failing: usize,
}

/// A structural validation violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5InterchangeConformanceViolation {
    /// Stable check id.
    pub check_id: String,
    /// Human-readable detail.
    pub detail: String,
}

impl std::fmt::Display for M5InterchangeConformanceViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.check_id, self.detail)
    }
}

/// One support/shiproom export row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InterchangeConformanceExportRow {
    /// Stable family id.
    pub family_id: String,
    /// The lifecycle label the family publishes.
    pub lifecycle_label: LifecycleLabel,
    /// The conformance class.
    pub conformance_class: ConformanceClass,
    /// The overall conformance state.
    pub conformance_state: ConformanceState,
    /// The decision.
    pub decision: DecisionState,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The resolvable contract version.
    pub contract_version: u32,
    /// The reason codes the family currently raises.
    pub active_reason_codes: Vec<ReasonCode>,
}

/// Export projection for support, release-center, and partner-review surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InterchangeConformanceExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// The top-level promotion decision.
    pub decision: DecisionState,
    /// Export rows.
    pub rows: Vec<M5InterchangeConformanceExportRow>,
}

/// The typed M5 interchange-conformance register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5InterchangeConformanceRegister {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register identifier.
    pub register_id: String,
    /// Lifecycle status of this register artifact.
    pub status: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Narrative companion document.
    pub overview_page: String,
    /// Evidence/proof packet.
    pub evidence_page: String,
    /// Help-center page.
    pub help_page: String,
    /// The conformance report.
    pub conformance_report_ref: String,
    /// Ref to the validator manifest.
    pub validator_manifest_ref: String,
    /// The validators home directory.
    pub validators_home: String,
    /// Ref to the contract catalog.
    pub contract_catalog_ref: String,
    /// Ref to the publication matrix.
    pub publication_matrix_ref: String,
    /// Ref to the reader/writer compatibility suite.
    pub reader_writer_compat_ref: String,
    /// Ref to the canonical M5 evidence index.
    pub evidence_index_ref: String,
    /// Ref to the build-identity artifact.
    pub build_identity_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<LifecycleLabel>,
    /// Closed interchange-direction vocabulary.
    pub interchange_directions: Vec<InterchangeDirection>,
    /// Closed conformance-class vocabulary.
    pub conformance_classes: Vec<ConformanceClass>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<ConsumerSurface>,
    /// Closed dimension-kind vocabulary.
    pub dimension_kinds: Vec<DimensionKind>,
    /// Closed dimension-outcome vocabulary.
    pub dimension_outcomes: Vec<DimensionOutcome>,
    /// Closed conformance-state vocabulary.
    pub conformance_states: Vec<ConformanceState>,
    /// Closed decision-state vocabulary.
    pub decision_states: Vec<DecisionState>,
    /// Closed degraded-state vocabulary.
    pub degraded_states: Vec<DegradedState>,
    /// Closed reason-code vocabulary.
    pub reason_codes: Vec<ReasonCode>,
    /// The conformance rows.
    pub rows: Vec<ConformanceRow>,
    /// The top-level promotion blocker summary.
    pub blockers: Blockers,
    /// Summary counts.
    pub summary: M5InterchangeConformanceSummary,
}

impl M5InterchangeConformanceRegister {
    /// Returns the row registered for `family_id`.
    pub fn row(&self, family_id: &str) -> Option<&ConformanceRow> {
        self.rows.iter().find(|r| r.family_id == family_id)
    }

    /// Resolves the lifecycle label, conformance state, and decision for a family. This is
    /// the lookup support export, release-center, and the in-product inspect surface share.
    pub fn resolve_conformance(
        &self,
        family_id: &str,
    ) -> Option<(LifecycleLabel, ConformanceState, DecisionState)> {
        self.row(family_id)
            .map(|r| (r.lifecycle_label, r.conformance_state, r.decision))
    }

    /// Families holding promotion.
    pub fn failed_rows(&self) -> Vec<&ConformanceRow> {
        self.rows
            .iter()
            .filter(|r| r.conformance_state == ConformanceState::Failed)
            .collect()
    }

    /// True when the interchange set holds promotion.
    pub fn holds_promotion(&self) -> bool {
        self.blockers.decision == DecisionState::Hold
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5InterchangeConformanceSummary {
        let count = |f: &dyn Fn(&ConformanceRow) -> bool| self.rows.iter().filter(|r| f(r)).count();
        M5InterchangeConformanceSummary {
            total_families: self.rows.len(),
            release_blocking_families: count(&|r| r.release_blocking),
            conformant_families: count(&|r| r.conformance_state == ConformanceState::Conformant),
            narrowed_families: count(&|r| r.conformance_state == ConformanceState::Narrowed),
            failed_families: count(&|r| r.conformance_state == ConformanceState::Failed),
            families_held: count(&|r| r.decision == DecisionState::Hold),
            catalog_linked_families: count(&|r| !r.catalog_family_id.is_empty()),
            round_trip_families: count(&|r| {
                r.conformance_class == ConformanceClass::RoundTripWriteBack
            }),
            compare_or_validate_only_families: count(&|r| {
                r.conformance_class != ConformanceClass::RoundTripWriteBack
            }),
            total_dimensions_evaluated: self.rows.iter().map(|r| r.dimensions.len()).sum(),
            dimensions_passing: self
                .rows
                .iter()
                .flat_map(|r| &r.dimensions)
                .filter(|d| d.outcome == DimensionOutcome::Pass)
                .count(),
            dimensions_downgrading: self
                .rows
                .iter()
                .flat_map(|r| &r.dimensions)
                .filter(|d| d.outcome == DimensionOutcome::Downgrade)
                .count(),
            dimensions_failing: self
                .rows
                .iter()
                .flat_map(|r| &r.dimensions)
                .filter(|d| d.outcome == DimensionOutcome::Fail)
                .count(),
        }
    }

    /// Produces an export/inspect-safe projection downstream surfaces render instead of
    /// cloning register text.
    pub fn support_export_projection(&self) -> M5InterchangeConformanceExportProjection {
        M5InterchangeConformanceExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            decision: self.blockers.decision,
            rows: self
                .rows
                .iter()
                .map(|r| M5InterchangeConformanceExportRow {
                    family_id: r.family_id.clone(),
                    lifecycle_label: r.lifecycle_label,
                    conformance_class: r.conformance_class,
                    conformance_state: r.conformance_state,
                    decision: r.decision,
                    release_blocking: r.release_blocking,
                    contract_version: r.contract_version,
                    active_reason_codes: r.active_reason_codes.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register's structural invariants.
    ///
    /// Mirrors the CI validator's semantic invariants. The checked-in register returns no
    /// violations; each structurally-parseable negative fixture returns at least one.
    pub fn validate(&self) -> Vec<M5InterchangeConformanceViolation> {
        let mut out = Vec::new();
        let mut push = |check_id: &str, detail: String| {
            out.push(M5InterchangeConformanceViolation {
                check_id: check_id.to_string(),
                detail,
            })
        };

        if self.schema_version != M5_INTERCHANGE_CONFORMANCE_SCHEMA_VERSION {
            push(
                "register.schema_version",
                format!("unexpected schema_version {}", self.schema_version),
            );
        }
        if self.record_kind != M5_INTERCHANGE_CONFORMANCE_RECORD_KIND {
            push(
                "register.record_kind",
                format!("unexpected record_kind {}", self.record_kind),
            );
        }
        if self.register_id != M5_INTERCHANGE_CONFORMANCE_REGISTER_ID {
            push(
                "register.register_id",
                format!("unexpected register_id {}", self.register_id),
            );
        }

        if self.lifecycle_labels != LifecycleLabel::ALL {
            push("vocab.lifecycle_labels", "off the canonical list".into());
        }
        if self.interchange_directions != InterchangeDirection::ALL {
            push(
                "vocab.interchange_directions",
                "off the canonical list".into(),
            );
        }
        if self.conformance_classes != ConformanceClass::ALL {
            push("vocab.conformance_classes", "off the canonical list".into());
        }
        if self.consumer_surfaces != ConsumerSurface::ALL {
            push("vocab.consumer_surfaces", "off the canonical list".into());
        }
        if self.dimension_kinds != DimensionKind::ALL {
            push("vocab.dimension_kinds", "off the canonical list".into());
        }
        if self.dimension_outcomes != DimensionOutcome::ALL {
            push("vocab.dimension_outcomes", "off the canonical list".into());
        }
        if self.conformance_states != ConformanceState::ALL {
            push("vocab.conformance_states", "off the canonical list".into());
        }
        if self.decision_states != DecisionState::ALL {
            push("vocab.decision_states", "off the canonical list".into());
        }
        if self.degraded_states != DegradedState::ALL {
            push("vocab.degraded_states", "off the canonical list".into());
        }
        if self.reason_codes != ReasonCode::ALL {
            push("vocab.reason_codes", "off the canonical list".into());
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.family_id.as_str()) {
                push(
                    "rows.duplicate_family_id",
                    format!("duplicate family_id {}", row.family_id),
                );
            }

            let kinds: Vec<DimensionKind> =
                row.dimensions.iter().map(|d| d.dimension_kind).collect();
            if kinds != DimensionKind::ALL.to_vec() {
                push(
                    "rows.dimension_coverage",
                    format!(
                        "{}: dimensions must be exactly the dimension-kind set",
                        row.family_id
                    ),
                );
            }

            let expected_state = row.computed_conformance_state();
            if row.conformance_state != expected_state {
                push(
                    "rows.conformance_state",
                    format!(
                        "{}: conformance_state disagrees with the dimensions",
                        row.family_id
                    ),
                );
            }

            let expected_decision = row.computed_decision();
            if row.decision != expected_decision {
                push(
                    "rows.decision",
                    format!(
                        "{}: decision disagrees with the conformance state",
                        row.family_id
                    ),
                );
            }

            // Consumer-agreement block must cover every surface and agree with the row.
            if row.consumer_agreement.surfaces != ConsumerSurface::ALL.to_vec() {
                push(
                    "rows.consumer_agreement",
                    format!(
                        "{}: consumer surfaces are not the canonical set",
                        row.family_id
                    ),
                );
            }
            if row.consumer_agreement.agreed_contract_version != row.contract_version {
                push(
                    "rows.consumer_agreement",
                    format!(
                        "{}: agreed contract version disagrees with the row",
                        row.family_id
                    ),
                );
            }
            if row.consumer_agreement.agreed_lifecycle_label != row.lifecycle_label {
                push(
                    "rows.consumer_agreement",
                    format!(
                        "{}: agreed lifecycle label disagrees with the row",
                        row.family_id
                    ),
                );
            }

            // A round-trip-write-back family must prove the round-trip dimension.
            if row.conformance_class.requires_write_back() {
                let round_trip = row
                    .dimensions
                    .iter()
                    .find(|d| d.dimension_kind == DimensionKind::RoundTripOrCompare);
                if let Some(d) = round_trip {
                    if row.conformance_state == ConformanceState::Conformant
                        && d.outcome != DimensionOutcome::Pass
                    {
                        push(
                            "rows.round_trip",
                            format!(
                                "{}: a conformant round-trip family must pass round_trip_or_compare",
                                row.family_id
                            ),
                        );
                    }
                }
            }
        }

        // Top-level blocker decision recomputed from the rows.
        let blocked_ids: Vec<String> = self
            .rows
            .iter()
            .filter(|r| r.computed_conformance_state() == ConformanceState::Failed)
            .map(|r| r.family_id.clone())
            .collect();
        if self.blockers.blocking_family_ids != blocked_ids {
            push(
                "blockers.block",
                "blocking_family_ids disagree with the failed rows".into(),
            );
        }
        let expected_top = if blocked_ids.is_empty() {
            DecisionState::Clear
        } else {
            DecisionState::Hold
        };
        if self.blockers.decision != expected_top {
            push(
                "blockers.decision",
                "top-level decision disagrees with the failed rows".into(),
            );
        }

        if self.summary != self.computed_summary() {
            push(
                "summary.count_mismatch",
                "summary counts disagree with the rows".into(),
            );
        }

        out
    }
}

/// Parses the embedded checked-in register into the typed model.
pub fn current_m5_interchange_conformance_register(
) -> Result<M5InterchangeConformanceRegister, serde_json::Error> {
    serde_json::from_str(M5_INTERCHANGE_CONFORMANCE_JSON)
}

#[cfg(test)]
mod tests;
