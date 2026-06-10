//! REST and GraphQL response viewers, assertions, timing tabs, and
//! browser-runtime trust class qualification records.
//!
//! This module owns the typed records that keep response viewers, assertion
//! panels, timing breakdowns, and browser-runtime trust classes inspectable
//! and attributable without depending on hidden shell shortcuts or ad hoc
//! scripts. The boundary schema is
//! [`/schemas/data/ship-rest-and-graphql-response-viewers-assertions-timing-tabs-and-browser-runtime-trust-classes.schema.json`](../../../schemas/data/ship-rest-and-graphql-response-viewers-assertions-timing-tabs-and-browser-runtime-trust-classes.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/ship-rest-and-graphql-response-viewers-assertions-timing-tabs-and-browser-runtime-trust-classes.json`](../../../artifacts/data/m5/ship-rest-and-graphql-response-viewers-assertions-timing-tabs-and-browser-runtime-trust-classes.json).
//!
//! Raw endpoint URLs, raw secrets, raw credential bodies, raw response bodies,
//! and raw cookie or token values do not belong in these records. They carry
//! stable IDs, closed posture vocabularies, and reviewable summaries that UI,
//! CLI, export, support, and public-proof surfaces can ingest safely.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for response-viewer qualification packets.
pub const RESPONSE_VIEWER_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ResponseViewerQualificationPacket`].
pub const RESPONSE_VIEWER_QUALIFICATION_RECORD_KIND: &str =
    "ship_rest_and_graphql_response_viewers_assertions_timing_tabs_and_browser_runtime_trust_classes";

/// Repo-relative path to the checked-in response-viewer qualification packet.
pub const RESPONSE_VIEWER_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/ship-rest-and-graphql-response-viewers-assertions-timing-tabs-and-browser-runtime-trust-classes.json";

/// Embedded checked-in packet JSON.
pub const RESPONSE_VIEWER_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/ship-rest-and-graphql-response-viewers-assertions-timing-tabs-and-browser-runtime-trust-classes.json"
));

/// Qualification label shown on promoted response-viewer surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseViewerQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl ResponseViewerQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Response-viewer surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseViewerSurfaceKind {
    /// REST response viewer.
    RestResponseViewer,
    /// GraphQL response viewer.
    GraphqlResponseViewer,
    /// Assertion result panel.
    AssertionPanel,
    /// Request timing breakdown tab.
    TimingTab,
    /// Browser-runtime trust class inspector.
    BrowserRuntimeTrustPanel,
}

/// Response viewer document kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseViewerKind {
    /// REST HTTP response.
    RestResponse,
    /// GraphQL operation response.
    GraphqlResponse,
}

/// Assertion outcome classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssertionOutcome {
    /// Assertion evaluated to true.
    Pass,
    /// Assertion evaluated to false.
    Fail,
    /// Assertion could not be evaluated.
    Error,
    /// Assertion was skipped.
    Skipped,
    /// Assertion blocked by policy.
    Blocked,
    /// Assertion timed out.
    Timeout,
}

/// Timing phase kind for request waterfall breakdowns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimingPhaseKind {
    /// DNS resolution time.
    Dns,
    /// TCP connection time.
    Connect,
    /// TLS handshake time.
    Tls,
    /// Time to first byte.
    Ttfb,
    /// Response body download time.
    Download,
    /// Total request duration.
    Total,
}

/// Browser-runtime trust class for rendered and inspected content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserRuntimeTrustClass {
    /// Sanitized rich rendering with no ambient execution rights.
    SanitizedRich,
    /// Raw text with no execution rights.
    RawText,
    /// Metadata only, no body values.
    MetadataOnly,
    /// Sandboxed preview with constrained execution.
    SandboxPreview,
    /// Trusted local content with full execution rights.
    TrustedLocal,
}

/// Browser-runtime inspector surface kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserRuntimeSurfaceKind {
    /// DOM element inspector.
    DomInspector,
    /// Web storage inspector.
    StorageInspector,
    /// Network request inspector.
    NetworkInspector,
    /// Console message inspector.
    ConsoleInspector,
}

/// Response safe-preview class for body rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponsePreviewClass {
    /// Rendered as a navigable JSON tree.
    JsonTree,
    /// Raw text with no execution rights.
    RawText,
    /// Sanitized rich rendering.
    SanitizedRich,
    /// Metadata only, no body values.
    MetadataOnly,
    /// Digest or hash only.
    DigestOnly,
    /// Summary for large payloads.
    LargePayloadSummary,
    /// Redacted, no values shown.
    Redacted,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseViewerQualificationProof {
    /// Stable proof packet id.
    pub packet_id: String,
    /// Repo-relative proof artifact reference.
    pub packet_ref: String,
    /// Proof-index reference.
    pub proof_index_ref: String,
    /// UTC capture date.
    pub captured_at: String,
    /// Evidence artifact references.
    pub evidence_refs: Vec<String>,
}

/// Boolean guard set that keeps stable surfaces from inheriting generic table truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseViewerSurfaceGuardSet {
    /// Response preview class is visible.
    pub response_preview_visible: bool,
    /// Raw response view is available.
    pub raw_view_visible: bool,
    /// Structured response view is available.
    pub structured_view_visible: bool,
    /// Assertion results are visible.
    pub assertion_results_visible: bool,
    /// Timing breakdown is visible.
    pub timing_breakdown_visible: bool,
    /// Export redaction posture is visible.
    pub export_redaction_visible: bool,
    /// Browser-runtime trust class is visible.
    pub browser_runtime_trust_visible: bool,
    /// Rendered view safety is visible.
    pub rendered_view_safety_visible: bool,
}

impl ResponseViewerSurfaceGuardSet {
    /// Returns true when every required visible guard is present.
    pub const fn all_visible(&self) -> bool {
        self.response_preview_visible
            && self.raw_view_visible
            && self.structured_view_visible
            && self.assertion_results_visible
            && self.timing_breakdown_visible
            && self.export_redaction_visible
            && self.browser_runtime_trust_visible
            && self.rendered_view_safety_visible
    }
}

/// One governed surface row in the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseViewerSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: ResponseViewerSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: ResponseViewerQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: ResponseViewerQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<ResponseViewerQualificationProof>,
    /// Visible guard set.
    pub guards: ResponseViewerSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One response viewer row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseViewerRow {
    /// Stable viewer id.
    pub viewer_id: String,
    /// Response viewer kind.
    pub viewer_kind: ResponseViewerKind,
    /// Supported preview classes.
    pub preview_classes: Vec<ResponsePreviewClass>,
    /// Whether raw view is supported.
    pub raw_view_supported: bool,
    /// Whether structured view is supported.
    pub structured_view_supported: bool,
    /// Whether rendered view is supported.
    pub rendered_view_supported: bool,
    /// Whether assertion results are shown.
    pub assertion_results_visible: bool,
    /// Whether timing breakdown is shown.
    pub timing_visible: bool,
    /// Whether export redaction posture is shown.
    pub export_redaction_visible: bool,
    /// Response body size limit in bytes.
    pub body_size_limit_bytes: u64,
    /// Whether HTML/JS responses receive ambient execution rights.
    pub grants_ambient_execution: bool,
}

/// One assertion row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssertionRow {
    /// Stable assertion id.
    pub assertion_id: String,
    /// Assertion outcome.
    pub outcome: AssertionOutcome,
    /// Assertion kind label (status, header, body, latency, etc.).
    pub assertion_kind: String,
    /// Request identity ref.
    pub request_identity_ref: String,
    /// Response identity ref.
    pub response_identity_ref: String,
    /// Whether the assertion result is visible before export.
    pub visible_before_export: bool,
    /// Whether the assertion is included in support bundles.
    pub support_bundle_safe: bool,
    /// Redaction class for failed assertion details.
    pub failure_redaction_class: String,
}

/// One timing tab row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimingTabRow {
    /// Stable timing tab id.
    pub timing_tab_id: String,
    /// Timing phases covered.
    pub phases: Vec<TimingPhaseKind>,
    /// Request identity ref.
    pub request_identity_ref: String,
    /// Whether timing is visible in the response viewer.
    pub visible_in_response_viewer: bool,
    /// Whether timing is included in history and replay.
    pub preserved_in_history: bool,
    /// Whether timing is export-safe.
    pub export_safe: bool,
}

/// One browser-runtime trust class row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BrowserRuntimeTrustRow {
    /// Stable trust row id.
    pub trust_row_id: String,
    /// Browser-runtime surface kind.
    pub surface_kind: BrowserRuntimeSurfaceKind,
    /// Trust class applied to this surface.
    pub trust_class: BrowserRuntimeTrustClass,
    /// Whether the trust class is visible in UI.
    pub visible_in_ui: bool,
    /// Whether the trust class is visible in exports.
    pub visible_in_export: bool,
    /// Whether mutating actions require explicit review.
    pub mutation_requires_review: bool,
    /// Whether cross-origin limitations are disclosed.
    pub cross_origin_disclosed: bool,
    /// Session identity ref.
    pub session_identity_ref: String,
}

/// Summary counts for a response-viewer qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseViewerQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of response viewer rows.
    pub response_viewer_count: usize,
    /// Number of assertion rows.
    pub assertion_count: usize,
    /// Number of timing tab rows.
    pub timing_tab_count: usize,
    /// Number of browser-runtime trust rows.
    pub browser_runtime_trust_count: usize,
}

/// Canonical response-viewer qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseViewerQualificationPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Release document reference.
    pub release_doc_ref: String,
    /// Help document reference.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<ResponseViewerSurfaceQualificationRow>,
    /// Response viewer rows.
    pub response_viewers: Vec<ResponseViewerRow>,
    /// Assertion rows.
    pub assertions: Vec<AssertionRow>,
    /// Timing tab rows.
    pub timing_tabs: Vec<TimingTabRow>,
    /// Browser-runtime trust rows.
    pub browser_runtime_trusts: Vec<BrowserRuntimeTrustRow>,
    /// Summary counts.
    pub summary: ResponseViewerQualificationSummary,
}

impl ResponseViewerQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> ResponseViewerQualificationSummary {
        let promoted_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .count();
        let stable_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.displayed_label.is_stable())
            .count();
        ResponseViewerQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            response_viewer_count: self.response_viewers.len(),
            assertion_count: self.assertions.len(),
            timing_tab_count: self.timing_tabs.len(),
            browser_runtime_trust_count: self.browser_runtime_trusts.len(),
        }
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<ResponseViewerQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != RESPONSE_VIEWER_QUALIFICATION_SCHEMA_VERSION {
            violations.push(ResponseViewerQualificationViolation::SchemaVersion {
                expected: RESPONSE_VIEWER_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != RESPONSE_VIEWER_QUALIFICATION_RECORD_KIND {
            violations.push(ResponseViewerQualificationViolation::RecordKind {
                expected: RESPONSE_VIEWER_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        collect_ids(
            self.surfaces
                .iter()
                .map(|surface| surface.surface_id.as_str()),
            &mut violations,
            ResponseViewerQualificationViolationKind::Surface,
        );
        collect_ids(
            self.response_viewers
                .iter()
                .map(|row| row.viewer_id.as_str()),
            &mut violations,
            ResponseViewerQualificationViolationKind::ResponseViewer,
        );
        collect_ids(
            self.assertions.iter().map(|row| row.assertion_id.as_str()),
            &mut violations,
            ResponseViewerQualificationViolationKind::Assertion,
        );
        collect_ids(
            self.timing_tabs
                .iter()
                .map(|row| row.timing_tab_id.as_str()),
            &mut violations,
            ResponseViewerQualificationViolationKind::TimingTab,
        );
        collect_ids(
            self.browser_runtime_trusts
                .iter()
                .map(|row| row.trust_row_id.as_str()),
            &mut violations,
            ResponseViewerQualificationViolationKind::BrowserRuntimeTrust,
        );

        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        ResponseViewerQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        ResponseViewerQualificationViolation::StableSurfaceMissingGuard {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    ResponseViewerQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let viewer_kinds: BTreeSet<_> = self
            .response_viewers
            .iter()
            .map(|row| row.viewer_kind)
            .collect();
        for required_kind in [
            ResponseViewerKind::RestResponse,
            ResponseViewerKind::GraphqlResponse,
        ] {
            if !viewer_kinds.contains(&required_kind) {
                violations.push(
                    ResponseViewerQualificationViolation::MissingResponseViewerKind {
                        viewer_kind: required_kind,
                    },
                );
            }
        }

        let preview_classes: BTreeSet<_> = self
            .response_viewers
            .iter()
            .flat_map(|row| row.preview_classes.iter().copied())
            .collect();
        for required_class in [
            ResponsePreviewClass::JsonTree,
            ResponsePreviewClass::RawText,
            ResponsePreviewClass::Redacted,
        ] {
            if !preview_classes.contains(&required_class) {
                violations.push(
                    ResponseViewerQualificationViolation::MissingResponsePreviewClass {
                        preview_class: required_class,
                    },
                );
            }
        }

        for row in &self.response_viewers {
            if row.grants_ambient_execution {
                violations.push(
                    ResponseViewerQualificationViolation::ResponseViewerGrantsAmbientExecution {
                        viewer_id: row.viewer_id.clone(),
                    },
                );
            }
            if row.body_size_limit_bytes == 0 {
                violations.push(
                    ResponseViewerQualificationViolation::ResponseViewerHasNoSizeLimit {
                        viewer_id: row.viewer_id.clone(),
                    },
                );
            }
        }

        let assertion_outcomes: BTreeSet<_> =
            self.assertions.iter().map(|row| row.outcome).collect();
        for required_outcome in [
            AssertionOutcome::Pass,
            AssertionOutcome::Fail,
            AssertionOutcome::Error,
            AssertionOutcome::Skipped,
        ] {
            if !assertion_outcomes.contains(&required_outcome) {
                violations.push(
                    ResponseViewerQualificationViolation::MissingAssertionOutcome {
                        outcome: required_outcome,
                    },
                );
            }
        }

        for row in &self.assertions {
            if row.assertion_kind.is_empty()
                || row.request_identity_ref.is_empty()
                || row.response_identity_ref.is_empty()
            {
                violations.push(
                    ResponseViewerQualificationViolation::IncompleteAssertionProjection {
                        assertion_id: row.assertion_id.clone(),
                    },
                );
            }
        }

        let timing_phases: BTreeSet<_> = self
            .timing_tabs
            .iter()
            .flat_map(|row| row.phases.iter().copied())
            .collect();
        for required_phase in [
            TimingPhaseKind::Dns,
            TimingPhaseKind::Connect,
            TimingPhaseKind::Ttfb,
            TimingPhaseKind::Total,
        ] {
            if !timing_phases.contains(&required_phase) {
                violations.push(ResponseViewerQualificationViolation::MissingTimingPhase {
                    phase: required_phase,
                });
            }
        }

        for row in &self.timing_tabs {
            if row.request_identity_ref.is_empty() {
                violations.push(
                    ResponseViewerQualificationViolation::IncompleteTimingTabProjection {
                        timing_tab_id: row.timing_tab_id.clone(),
                    },
                );
            }
        }

        let trust_classes: BTreeSet<_> = self
            .browser_runtime_trusts
            .iter()
            .map(|row| row.trust_class)
            .collect();
        for required_class in [
            BrowserRuntimeTrustClass::SanitizedRich,
            BrowserRuntimeTrustClass::RawText,
            BrowserRuntimeTrustClass::MetadataOnly,
            BrowserRuntimeTrustClass::SandboxPreview,
        ] {
            if !trust_classes.contains(&required_class) {
                violations.push(
                    ResponseViewerQualificationViolation::MissingBrowserRuntimeTrustClass {
                        trust_class: required_class,
                    },
                );
            }
        }

        let browser_surface_kinds: BTreeSet<_> = self
            .browser_runtime_trusts
            .iter()
            .map(|row| row.surface_kind)
            .collect();
        for required_kind in [
            BrowserRuntimeSurfaceKind::DomInspector,
            BrowserRuntimeSurfaceKind::NetworkInspector,
        ] {
            if !browser_surface_kinds.contains(&required_kind) {
                violations.push(
                    ResponseViewerQualificationViolation::MissingBrowserRuntimeSurfaceKind {
                        surface_kind: required_kind,
                    },
                );
            }
        }

        for row in &self.browser_runtime_trusts {
            if row.session_identity_ref.is_empty() {
                violations.push(
                    ResponseViewerQualificationViolation::IncompleteBrowserRuntimeTrustProjection {
                        trust_row_id: row.trust_row_id.clone(),
                    },
                );
            }
            if !row.visible_in_ui {
                violations.push(
                    ResponseViewerQualificationViolation::BrowserRuntimeTrustNotVisibleInUi {
                        trust_row_id: row.trust_row_id.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ResponseViewerQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in response-viewer qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_response_viewer_qualification(
) -> Result<ResponseViewerQualificationPacket, serde_json::Error> {
    serde_json::from_str(RESPONSE_VIEWER_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseViewerQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Response viewer rows.
    ResponseViewer,
    /// Assertion rows.
    Assertion,
    /// Timing tab rows.
    TimingTab,
    /// Browser-runtime trust rows.
    BrowserRuntimeTrust,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<ResponseViewerQualificationViolation>,
    kind: ResponseViewerQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(ResponseViewerQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for response-viewer qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseViewerQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: ResponseViewerQualificationViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required response viewer kind is missing.
    MissingResponseViewerKind { viewer_kind: ResponseViewerKind },
    /// Required response preview class is missing.
    MissingResponsePreviewClass { preview_class: ResponsePreviewClass },
    /// Response viewer grants ambient execution rights.
    ResponseViewerGrantsAmbientExecution { viewer_id: String },
    /// Response viewer has no body size limit.
    ResponseViewerHasNoSizeLimit { viewer_id: String },
    /// Required assertion outcome is missing.
    MissingAssertionOutcome { outcome: AssertionOutcome },
    /// Assertion row does not project truth everywhere.
    IncompleteAssertionProjection { assertion_id: String },
    /// Required timing phase is missing.
    MissingTimingPhase { phase: TimingPhaseKind },
    /// Timing tab row does not project truth everywhere.
    IncompleteTimingTabProjection { timing_tab_id: String },
    /// Required browser-runtime trust class is missing.
    MissingBrowserRuntimeTrustClass {
        trust_class: BrowserRuntimeTrustClass,
    },
    /// Required browser-runtime surface kind is missing.
    MissingBrowserRuntimeSurfaceKind {
        surface_kind: BrowserRuntimeSurfaceKind,
    },
    /// Browser-runtime trust row does not project truth everywhere.
    IncompleteBrowserRuntimeTrustProjection { trust_row_id: String },
    /// Browser-runtime trust class is not visible in UI.
    BrowserRuntimeTrustNotVisibleInUi { trust_row_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for ResponseViewerQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "{kind:?} id {id} is duplicated"),
            Self::StableSurfaceMissingProof { surface_id } => {
                write!(f, "{surface_id} is stable without a proof packet")
            }
            Self::StableSurfaceMissingGuard { surface_id } => {
                write!(f, "{surface_id} is stable without complete guard truth")
            }
            Self::NarrowedSurfaceLacksDowngradeRule { surface_id } => {
                write!(f, "{surface_id} is narrowed without a downgrade rule")
            }
            Self::MissingResponseViewerKind { viewer_kind } => {
                write!(f, "response viewer kind {viewer_kind:?} is not covered")
            }
            Self::MissingResponsePreviewClass { preview_class } => {
                write!(f, "response preview class {preview_class:?} is not covered")
            }
            Self::ResponseViewerGrantsAmbientExecution { viewer_id } => {
                write!(f, "{viewer_id} grants ambient execution rights")
            }
            Self::ResponseViewerHasNoSizeLimit { viewer_id } => {
                write!(f, "{viewer_id} has no body size limit")
            }
            Self::MissingAssertionOutcome { outcome } => {
                write!(f, "assertion outcome {outcome:?} is not covered")
            }
            Self::IncompleteAssertionProjection { assertion_id } => {
                write!(
                    f,
                    "{assertion_id} does not project assertion truth everywhere"
                )
            }
            Self::MissingTimingPhase { phase } => {
                write!(f, "timing phase {phase:?} is not covered")
            }
            Self::IncompleteTimingTabProjection { timing_tab_id } => {
                write!(
                    f,
                    "{timing_tab_id} does not project timing tab truth everywhere"
                )
            }
            Self::MissingBrowserRuntimeTrustClass { trust_class } => {
                write!(
                    f,
                    "browser runtime trust class {trust_class:?} is not covered"
                )
            }
            Self::MissingBrowserRuntimeSurfaceKind { surface_kind } => {
                write!(
                    f,
                    "browser runtime surface kind {surface_kind:?} is not covered"
                )
            }
            Self::IncompleteBrowserRuntimeTrustProjection { trust_row_id } => {
                write!(
                    f,
                    "{trust_row_id} does not project browser-runtime trust truth everywhere"
                )
            }
            Self::BrowserRuntimeTrustNotVisibleInUi { trust_row_id } => {
                write!(f, "{trust_row_id} is not visible in UI")
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for ResponseViewerQualificationViolation {}
