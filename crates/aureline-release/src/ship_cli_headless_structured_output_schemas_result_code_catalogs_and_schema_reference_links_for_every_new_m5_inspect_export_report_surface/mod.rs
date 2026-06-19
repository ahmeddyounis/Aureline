//! Typed M5 CLI/headless structured-output and result-code catalog: the
//! canonical index of every new M5 CLI/headless inspect, export, report, and
//! health surface that emits structured output.
//!
//! Where the M5 JSON Schema catalog publishes the durable artifact *families*,
//! this catalog speaks for the *CLI/headless surfaces* that emit them. Each
//! [`CliOutputSurface`] binds one surface to:
//!
//! - its surface kind ([`SurfaceKind`]), the command id it reuses, and the
//!   lifecycle label it publishes ([`LifecycleLabel`], equal to the publication
//!   matrix's effective published label for the family),
//! - its structured-output schema reference
//!   ([`CliOutputSurface::structured_output_schema_ref`]) — a checked-in JSON
//!   Schema package under `schemas/public/m5-json/` resolved from the JSON Schema
//!   catalog — and the [`OutputEnvelopeClass`] a machine consumer binds against,
//! - its result-code catalog ([`ResultCodeRow`]): stable [`ResultCode`] enums
//!   drawn verbatim from the CLI/headless machine-output stability contract, each
//!   with a pinned numeric code and a partial-result flag,
//! - the [`PartialResultState`] and [`FreshnessState`] vocabularies the surface
//!   can emit, so machine output is safe for automation, and
//! - its UI/CLI parity declaration: the UI inspect surface, the
//!   [`ParityMatchMode`], and a CLI fixture and a UI fixture proving the
//!   lifecycle/degraded-state vocabulary is identical on both surfaces.
//!
//! Downstream surfaces (CLI help, docs, sample payloads, support bundles) resolve
//! a surface's schema reference and result-code catalog from this catalog instead
//! of restating field semantics. A surface missing its schema reference,
//! result-code catalog, lifecycle label, or parity fixture narrows below the
//! launch cutline ([`DowngradeBehavior::NarrowBelowCutline`]).
//!
//! The catalog is checked in at `artifacts/contracts/m5-cli-output-catalog.json`
//! and embedded here, so this typed consumer and the CI validator agree on every
//! surface without a cargo build in CI. The model is metadata-only: every field
//! is a typed state or an opaque repo-relative ref or URI. It carries no surface
//! payloads, rendered bodies, signatures, or credential material.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Supported catalog schema version.
pub const M5_CLI_OUTPUT_CATALOG_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the catalog.
pub const M5_CLI_OUTPUT_CATALOG_RECORD_KIND: &str = "m5_cli_output_catalog";

/// Stable catalog identifier.
pub const M5_CLI_OUTPUT_CATALOG_ID: &str = "m5_cli_output_catalog:v1";

/// Repo-relative path to the checked-in catalog.
pub const M5_CLI_OUTPUT_CATALOG_PATH: &str = "artifacts/contracts/m5-cli-output-catalog.json";

/// Embedded checked-in catalog JSON.
pub const M5_CLI_OUTPUT_CATALOG_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/contracts/m5-cli-output-catalog.json"
));

/// The kind of CLI/headless surface a row describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKind {
    /// An inspect surface.
    Inspect,
    /// An export surface.
    Export,
    /// A report surface.
    Report,
    /// A health surface.
    Health,
}

impl SurfaceKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 4] = [Self::Inspect, Self::Export, Self::Report, Self::Health];
}

/// The lifecycle/stability label a surface publishes.
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

/// The machine-output stability class a surface commits to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MachineOutputStabilityClass {
    /// Schema is frozen; additive-minor bumps only without a decision row.
    StableSchemaGoverned,
    /// Schema is published but subject to additive-minor churn.
    PreviewSchemaGovernedAdditiveMinorOnly,
    /// Schema is published but explicitly permitted to break.
    ExperimentalSchemaGovernedMayBreak,
    /// No external stability promise.
    InternalNoStabilityPromise,
}

impl MachineOutputStabilityClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StableSchemaGoverned,
        Self::PreviewSchemaGovernedAdditiveMinorOnly,
        Self::ExperimentalSchemaGovernedMayBreak,
        Self::InternalNoStabilityPromise,
    ];
}

/// The single primary envelope a machine consumer binds against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputEnvelopeClass {
    /// One JSON document.
    JsonDocumentSingle,
    /// One JSON value per line.
    JsonlLineStream,
    /// An NDJSON event stream.
    NdjsonEventStream,
    /// A SARIF 2.1.0 document.
    #[serde(rename = "sarif_2_1_0_document")]
    Sarif210Document,
    /// A JUnit XML document.
    JunitXmlDocument,
}

impl OutputEnvelopeClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::JsonDocumentSingle,
        Self::JsonlLineStream,
        Self::NdjsonEventStream,
        Self::Sarif210Document,
        Self::JunitXmlDocument,
    ];
}

/// A stable result code, drawn verbatim from the CLI/headless machine-output
/// stability contract's `exit_code_class` vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultCode {
    /// Completed with a full result.
    Success,
    /// Completed; nothing matched.
    SuccessNoActionTaken,
    /// Some rows resolved; partial-result block follows.
    PartialSuccessWithWarnings,
    /// The invocation was malformed.
    UsageError,
    /// An argument failed validation.
    InputValidationError,
    /// Admin policy or workspace trust denied the surface.
    PolicyOrTrustDenied,
    /// A credential handle was denied.
    CredentialBrokerDenied,
    /// A required preview was not shown.
    PreviewRequiredNotShown,
    /// A required approval was not granted.
    ApprovalRequiredNotGranted,
    /// A dry run reported what it would have applied.
    DryRunWouldHaveApplied,
    /// A deadline was exceeded.
    TimeoutOrDeadlineExceeded,
    /// A remote dependency was unavailable.
    NetworkOrRemoteUnavailable,
    /// A kill switch is active.
    KillSwitchActive,
    /// A required input was missing or stale.
    DependencyMissingOrStale,
    /// No machine projection exists in this headless context.
    UnsupportedOnHeadless,
    /// The invocation was cancelled.
    CancelledByUser,
    /// An internal error prevented a result.
    UnrecoverableInternalError,
}

impl ResultCode {
    /// Every code, in declaration order.
    pub const ALL: [Self; 17] = [
        Self::Success,
        Self::SuccessNoActionTaken,
        Self::PartialSuccessWithWarnings,
        Self::UsageError,
        Self::InputValidationError,
        Self::PolicyOrTrustDenied,
        Self::CredentialBrokerDenied,
        Self::PreviewRequiredNotShown,
        Self::ApprovalRequiredNotGranted,
        Self::DryRunWouldHaveApplied,
        Self::TimeoutOrDeadlineExceeded,
        Self::NetworkOrRemoteUnavailable,
        Self::KillSwitchActive,
        Self::DependencyMissingOrStale,
        Self::UnsupportedOnHeadless,
        Self::CancelledByUser,
        Self::UnrecoverableInternalError,
    ];

    /// True for the success and no-action-taken codes, which pin numeric code 0.
    pub fn is_success(self) -> bool {
        matches!(self, Self::Success | Self::SuccessNoActionTaken)
    }
}

/// A partial-result state a surface can emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartialResultState {
    /// Fully resolved.
    Complete,
    /// Some rows resolved.
    Partial,
    /// Resolved with reduced fidelity.
    Degraded,
    /// Could not resolve at all.
    Unavailable,
    /// Inputs are stale; a retest is needed.
    StaleRetestNeeded,
}

impl PartialResultState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Complete,
        Self::Partial,
        Self::Degraded,
        Self::Unavailable,
        Self::StaleRetestNeeded,
    ];

    /// True for the partial and degraded states the partial-result carrier covers.
    pub fn is_partial_or_degraded(self) -> bool {
        matches!(self, Self::Partial | Self::Degraded)
    }
}

/// A freshness/staleness state a surface can emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessState {
    /// Inputs are fresh.
    Fresh,
    /// Inputs are stale.
    Stale,
    /// A retest is needed.
    RetestNeeded,
    /// Freshness could not be determined.
    Unknown,
}

impl FreshnessState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Fresh, Self::Stale, Self::RetestNeeded, Self::Unknown];
}

/// How a UI inspect surface must match the CLI surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityMatchMode {
    /// The lifecycle/degraded-state vocabulary must match field-for-field.
    ExactMatchRequired,
    /// The surface projects the row via a registry-owned projection.
    ProjectionMatchRequired,
    /// The surface cites the CLI surface in text but owns no parity cell.
    InformationalOnly,
}

impl ParityMatchMode {
    /// Every mode, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ExactMatchRequired,
        Self::ProjectionMatchRequired,
        Self::InformationalOnly,
    ];
}

/// What happens to a surface that loses required publication evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeBehavior {
    /// The surface narrows below the launch cutline.
    NarrowBelowCutline,
    /// The artifact is rejected.
    Reject,
}

impl DowngradeBehavior {
    /// Every behavior, in declaration order.
    pub const ALL: [Self; 2] = [Self::NarrowBelowCutline, Self::Reject];
}

/// One result code in a surface's result-code catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultCodeRow {
    /// The stable result code.
    pub result_code: ResultCode,
    /// The pinned POSIX-compatible numeric code.
    pub numeric_code: u16,
    /// A short reviewable meaning.
    pub meaning: String,
    /// True when this code carries a partial result.
    pub partial_result: bool,
}

/// One new M5 CLI/headless inspect/export/report/health surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CliOutputSurface {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Human-readable title.
    pub title: String,
    /// One-line summary.
    pub summary: String,
    /// The surface kind.
    pub surface_kind: SurfaceKind,
    /// The durable family whose schema package this surface's output validates against.
    pub family_id: String,
    /// The command id this surface reuses.
    pub command_id: String,
    /// The lifecycle label this surface publishes.
    pub lifecycle_label: LifecycleLabel,
    /// The machine-output stability class.
    pub machine_output_stability_class: MachineOutputStabilityClass,
    /// The primary machine-output envelope class.
    pub output_envelope_class: OutputEnvelopeClass,
    /// Repo-relative path to the resolved JSON Schema package.
    pub structured_output_schema_ref: String,
    /// The resolved JSON Schema package `$id`.
    pub structured_output_schema_id: String,
    /// The result-code catalog (non-empty; success + at least one error).
    pub result_code_catalog: Vec<ResultCodeRow>,
    /// The partial-result states this surface can emit.
    pub partial_result_states: Vec<PartialResultState>,
    /// The freshness states this surface can emit.
    pub freshness_states: Vec<FreshnessState>,
    /// What happens when required publication evidence is lost.
    pub downgrade_behavior: DowngradeBehavior,
    /// Human-readable compatibility note.
    pub compatibility_note: String,
    /// Ref to the doc that carries the compatibility note.
    pub compatibility_note_ref: String,
    /// The UI inspect surface this CLI surface keeps parity with.
    pub ui_inspect_surface: String,
    /// How the UI inspect surface must match.
    pub parity_match_mode: ParityMatchMode,
    /// Ref to the checked-in CLI structured-output parity fixture.
    pub cli_parity_fixture_ref: String,
    /// Ref to the checked-in UI inspect parity fixture.
    pub ui_parity_fixture_ref: String,
    /// Ref to the JSON Schema catalog package row.
    pub json_schema_catalog_ref: String,
    /// Ref to the publication-matrix row.
    pub matrix_row_ref: String,
    /// Refs to the validators that gate this surface.
    pub validator_suite_refs: Vec<String>,
}

impl CliOutputSurface {
    /// True when this surface publishes at or above the stable cutline.
    pub fn publishes_stable(&self) -> bool {
        matches!(
            self.lifecycle_label,
            LifecycleLabel::Lts | LifecycleLabel::Stable
        )
    }

    /// The result codes this surface publishes, in catalog order.
    pub fn result_codes(&self) -> Vec<ResultCode> {
        self.result_code_catalog
            .iter()
            .map(|r| r.result_code)
            .collect()
    }

    /// True when this surface can emit a partial or degraded result.
    pub fn declares_partial_or_degraded(&self) -> bool {
        self.partial_result_states
            .iter()
            .any(|s| s.is_partial_or_degraded())
    }

    /// True when the surface publishes the partial-result carrier code.
    pub fn has_partial_result_carrier(&self) -> bool {
        self.result_code_catalog
            .iter()
            .any(|r| r.result_code == ResultCode::PartialSuccessWithWarnings && r.partial_result)
    }
}

/// The offline/mirror bundling declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OfflineBundle {
    /// True when the surface set bundles into mirror artifact sets.
    pub mirrorable: bool,
    /// True when validation requires runtime service access.
    pub requires_runtime_service: bool,
    /// Bundle members (catalog, schema, fixtures, validator).
    pub bundle_members: Vec<String>,
    /// Human-readable note.
    pub note: String,
}

/// Summary counts over the surface set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5CliOutputCatalogSummary {
    /// Total surfaces.
    pub total_surfaces: usize,
    /// Inspect surfaces.
    pub inspect_surfaces: usize,
    /// Export surfaces.
    pub export_surfaces: usize,
    /// Report surfaces.
    pub report_surfaces: usize,
    /// Health surfaces.
    pub health_surfaces: usize,
    /// Surfaces published at the stable label.
    pub stable_label_surfaces: usize,
    /// Surfaces published at the beta label.
    pub beta_label_surfaces: usize,
    /// Surfaces with a partial-result carrier code.
    pub surfaces_with_partial_result_carrier: usize,
    /// Surfaces with both parity fixtures.
    pub surfaces_with_parity_fixtures: usize,
}

/// A structural validation violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CliOutputCatalogViolation {
    /// Stable check id.
    pub check_id: String,
    /// Human-readable detail.
    pub detail: String,
}

/// The typed M5 CLI/headless structured-output and result-code catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5CliOutputCatalog {
    /// Catalog schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable catalog identifier.
    pub catalog_id: String,
    /// Lifecycle status of this catalog artifact.
    pub status: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// CLI reference doc.
    pub cli_doc_page: String,
    /// Ref to the JSON Schema catalog.
    pub json_schema_catalog_ref: String,
    /// Ref to the public-contract publication matrix.
    pub publication_matrix_ref: String,
    /// Ref to the CLI/headless surface contract.
    pub cli_surface_contract_ref: String,
    /// Ref to the CLI output registry boundary schema.
    pub cli_output_registry_schema_ref: String,
    /// Ref to the canonical M5 evidence index.
    pub evidence_index_ref: String,
    /// Schema home for the resolved structured-output packages.
    pub schema_home: String,
    /// Closed surface-kind vocabulary.
    pub surface_kinds: Vec<SurfaceKind>,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<LifecycleLabel>,
    /// Closed machine-output-stability vocabulary.
    pub machine_output_stability_classes: Vec<MachineOutputStabilityClass>,
    /// Closed output-envelope vocabulary.
    pub output_envelope_classes: Vec<OutputEnvelopeClass>,
    /// Closed result-code vocabulary.
    pub result_codes: Vec<ResultCode>,
    /// Closed partial-result-state vocabulary.
    pub partial_result_states: Vec<PartialResultState>,
    /// Closed freshness-state vocabulary.
    pub freshness_states: Vec<FreshnessState>,
    /// Closed parity-match-mode vocabulary.
    pub parity_match_modes: Vec<ParityMatchMode>,
    /// Closed downgrade-behavior vocabulary.
    pub downgrade_behaviors: Vec<DowngradeBehavior>,
    /// The offline/mirror bundling declaration.
    pub offline_bundle: OfflineBundle,
    /// The published surfaces.
    pub surfaces: Vec<CliOutputSurface>,
    /// Summary counts.
    pub summary: M5CliOutputCatalogSummary,
}

impl M5CliOutputCatalog {
    /// Returns the surface registered for `surface_id`.
    pub fn surface(&self, surface_id: &str) -> Option<&CliOutputSurface> {
        self.surfaces.iter().find(|s| s.surface_id == surface_id)
    }

    /// Resolves the structured-output schema reference and lifecycle label for a surface.
    ///
    /// This is the lookup CLI help, docs, and sample payloads share so they quote
    /// one schema reference and one lifecycle label per surface.
    pub fn resolve_surface_schema(&self, surface_id: &str) -> Option<(&str, LifecycleLabel)> {
        self.surface(surface_id)
            .map(|s| (s.structured_output_schema_ref.as_str(), s.lifecycle_label))
    }

    /// Surfaces of a given kind.
    pub fn surfaces_for_kind(&self, kind: SurfaceKind) -> Vec<&CliOutputSurface> {
        self.surfaces
            .iter()
            .filter(|s| s.surface_kind == kind)
            .collect()
    }

    /// Surfaces published at or above the stable cutline.
    pub fn stable_surfaces(&self) -> Vec<&CliOutputSurface> {
        self.surfaces
            .iter()
            .filter(|s| s.publishes_stable())
            .collect()
    }

    /// Recomputes the summary block from the surfaces.
    pub fn computed_summary(&self) -> M5CliOutputCatalogSummary {
        let count =
            |f: &dyn Fn(&CliOutputSurface) -> bool| self.surfaces.iter().filter(|s| f(s)).count();
        M5CliOutputCatalogSummary {
            total_surfaces: self.surfaces.len(),
            inspect_surfaces: count(&|s| s.surface_kind == SurfaceKind::Inspect),
            export_surfaces: count(&|s| s.surface_kind == SurfaceKind::Export),
            report_surfaces: count(&|s| s.surface_kind == SurfaceKind::Report),
            health_surfaces: count(&|s| s.surface_kind == SurfaceKind::Health),
            stable_label_surfaces: count(&|s| s.lifecycle_label == LifecycleLabel::Stable),
            beta_label_surfaces: count(&|s| s.lifecycle_label == LifecycleLabel::Beta),
            surfaces_with_partial_result_carrier: count(&|s| s.has_partial_result_carrier()),
            surfaces_with_parity_fixtures: count(&|s| {
                !s.cli_parity_fixture_ref.is_empty() && !s.ui_parity_fixture_ref.is_empty()
            }),
        }
    }

    /// Validates the catalog's structural invariants.
    ///
    /// Mirrors the CI validator's semantic invariants. The checked-in catalog
    /// returns no violations; each negative fixture returns at least one.
    pub fn validate(&self) -> Vec<M5CliOutputCatalogViolation> {
        let mut out = Vec::new();
        let mut push = |check_id: &str, detail: String| {
            out.push(M5CliOutputCatalogViolation {
                check_id: check_id.to_string(),
                detail,
            })
        };

        if self.schema_version != M5_CLI_OUTPUT_CATALOG_SCHEMA_VERSION {
            push(
                "catalog.schema_version",
                format!("unexpected schema_version {}", self.schema_version),
            );
        }
        if self.record_kind != M5_CLI_OUTPUT_CATALOG_RECORD_KIND {
            push(
                "catalog.record_kind",
                format!("unexpected record_kind {}", self.record_kind),
            );
        }
        if self.catalog_id != M5_CLI_OUTPUT_CATALOG_ID {
            push(
                "catalog.catalog_id",
                format!("unexpected catalog_id {}", self.catalog_id),
            );
        }

        if self.surface_kinds != SurfaceKind::ALL {
            push(
                "vocab.surface_kinds",
                "surface_kinds off the canonical list".into(),
            );
        }
        if self.lifecycle_labels != LifecycleLabel::ALL {
            push(
                "vocab.lifecycle_labels",
                "lifecycle_labels off the canonical list".into(),
            );
        }
        if self.machine_output_stability_classes != MachineOutputStabilityClass::ALL {
            push(
                "vocab.machine_output_stability_classes",
                "machine_output_stability_classes off the canonical list".into(),
            );
        }
        if self.output_envelope_classes != OutputEnvelopeClass::ALL {
            push(
                "vocab.output_envelope_classes",
                "output_envelope_classes off the canonical list".into(),
            );
        }
        if self.result_codes != ResultCode::ALL {
            push(
                "vocab.result_codes",
                "result_codes off the canonical list".into(),
            );
        }
        if self.partial_result_states != PartialResultState::ALL {
            push(
                "vocab.partial_result_states",
                "partial_result_states off the canonical list".into(),
            );
        }
        if self.freshness_states != FreshnessState::ALL {
            push(
                "vocab.freshness_states",
                "freshness_states off the canonical list".into(),
            );
        }
        if self.parity_match_modes != ParityMatchMode::ALL {
            push(
                "vocab.parity_match_modes",
                "parity_match_modes off the canonical list".into(),
            );
        }
        if self.downgrade_behaviors != DowngradeBehavior::ALL {
            push(
                "vocab.downgrade_behaviors",
                "downgrade_behaviors off the canonical list".into(),
            );
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for surface in &self.surfaces {
            let sid = surface.surface_id.as_str();
            if !seen.insert(sid) {
                push(
                    "surfaces.duplicate_surface_id",
                    format!("duplicate surface_id {sid}"),
                );
            }
            if surface.result_code_catalog.is_empty() {
                push(
                    "surfaces.empty_result_code_catalog",
                    format!("{sid}: empty result_code_catalog"),
                );
            }
            let codes = surface.result_codes();
            if !codes.contains(&ResultCode::Success) {
                push(
                    "surfaces.missing_success_code",
                    format!("{sid}: result_code_catalog missing a success row"),
                );
            }
            if !codes.iter().any(|c| !c.is_success()) {
                push(
                    "surfaces.missing_error_code",
                    format!("{sid}: result_code_catalog missing an error row"),
                );
            }
            for row in &surface.result_code_catalog {
                if row.result_code.is_success() && row.numeric_code != 0 {
                    push(
                        "surfaces.success_code_numeric",
                        format!("{sid}: {:?} must map to numeric code 0", row.result_code),
                    );
                }
                if row.result_code == ResultCode::PartialSuccessWithWarnings && !row.partial_result
                {
                    push(
                        "surfaces.partial_carrier_flag",
                        format!(
                            "{sid}: partial_success_with_warnings must be a partial-result carrier"
                        ),
                    );
                }
            }
            if surface.declares_partial_or_degraded() && !surface.has_partial_result_carrier() {
                push(
                    "surfaces.partial_state_without_carrier",
                    format!("{sid}: declares a partial/degraded state without a partial-result carrier code"),
                );
            }
            if surface.has_partial_result_carrier() && !surface.declares_partial_or_degraded() {
                push(
                    "surfaces.carrier_without_partial_state",
                    format!("{sid}: carries the partial-result code but declares no partial/degraded state"),
                );
            }
            if !surface
                .partial_result_states
                .contains(&PartialResultState::StaleRetestNeeded)
            {
                push(
                    "surfaces.missing_stale_retest_state",
                    format!("{sid}: partial_result_states must include stale_retest_needed"),
                );
            }
            if surface.cli_parity_fixture_ref.is_empty() || surface.ui_parity_fixture_ref.is_empty()
            {
                push(
                    "surfaces.missing_parity_fixture",
                    format!("{sid}: missing a parity fixture ref"),
                );
            }
        }

        if self.summary != self.computed_summary() {
            push(
                "summary.count_mismatch",
                "summary counts disagree with the surfaces".into(),
            );
        }

        out
    }
}

/// Parses the embedded checked-in catalog into the typed model.
pub fn current_m5_cli_output_catalog() -> Result<M5CliOutputCatalog, serde_json::Error> {
    serde_json::from_str(M5_CLI_OUTPUT_CATALOG_JSON)
}

#[cfg(test)]
mod tests;
