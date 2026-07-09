//! Two reusable M5 experiment components — the artifact lineage panel and the result summary
//! card — so a user can tell which run and lineage produced an artifact and whether a shared
//! summary stays summary-first and export-scope-honest from the component alone, before any
//! open, compare, or share action: the lineage panel names its artifact identity, artifact
//! kind, producing run ID, generator step, environment / model fingerprint where relevant,
//! saved scope, and lineage state (`current`, `stale` / diverged, derived, regenerated, or
//! unknown), and offers first-class open-artifact / trace-to-run / export-lineage actions; the
//! result summary card names its headline metrics, artifact count, freshness, support / report
//! scope, include-raw toggle, provenance note, and explicit summary-versus-evidence-versus-raw
//! handoff choice, and offers review / share-summary-only paths.
//!
//! Aureline's frozen experiment-component matrix
//! ([`crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix`])
//! names the artifact lineage panel and the result summary card as two governed component
//! families and freezes their controlled vocabulary — the artifact kind classes
//! (`model_checkpoint`, `metrics_table`, `plot_figure`, `exported_report`, `log_bundle`,
//! `unknown_artifact`) and lineage states (`lineage_complete`, `lineage_partial`,
//! `lineage_broken`, `derived_upstream_known`, `derived_upstream_unknown`, `regenerated`) a
//! panel binds; the summary content classes (`headline_metric`, `metric_table`,
//! `narrative_summary`, `evidence_link`, `raw_payload_ref`, `no_result`) and export scopes
//! (`summary_scope`, `metadata_scope`, `evidence_scope`, `raw_scope`, `redacted_scope`,
//! `export_withheld`) a card binds; the one controlled disposition vocabulary; the surface
//! families; the deployment lines; the consumer surfaces; the accessibility routes; the
//! required labels; and the downgrade triggers. This module *implements* that contract as two
//! co-equal component vectors so a claimed M5 notebook, experiment-dashboard, comparison,
//! lineage, share-review, or CLI surface can project a lineage panel and a summary card that
//! keep the same truth.
//!
//! The module has two derived resolvers:
//!
//! 1. [`resolve_artifact_lineage`] — takes a lineage panel's lineage state and derives its
//!    traceability class (fully traced, regenerated, partially traced, or untraced), whether the
//!    artifact is reliably traced, and which notes the panel must carry — so a broken, diverged,
//!    or unknown-upstream artifact can never read as a fully-traced artifact and every artifact
//!    routes back to its producing run and lineage state rather than reading as an anonymous
//!    attachment.
//! 2. [`resolve_summary_export`] — takes a summary card's export scope and derives its export
//!    disposition (metadata-safe, evidence-scoped, raw-included, redacted, or withheld), whether
//!    the export includes a raw payload, whether it is metadata-only, whether it is withheld, and
//!    which notes the card must carry — so a raw payload is never included by default and the
//!    include-raw toggle stays an explicit, warned choice before a share or export.
//!
//! A single controls packet — [`ArtifactLineagePanelResultSummaryCardControlsPacket`] — binds
//! one vector of lineage panels and one vector of summary cards to the same artifact / lineage,
//! summary-content / export-scope, deep-link, and non-visual accessibility vocabulary, so
//! producing-run identity and export scope stay explicit across desktop, headless / export, and
//! support consumers.
//!
//! The artifact kind class ([`M5ArtifactKindClass`]), lineage state ([`M5LineageState`]),
//! summary content class ([`M5SummaryContentClass`]), summary export scope
//! ([`M5SummaryExportScope`]), disposition ([`M5ExperimentDisposition`]), surface family
//! ([`M5ExperimentSurfaceFamily`]), deployment line ([`M5ExperimentDeploymentLine`]), consumer
//! surface ([`M5ExperimentConsumerSurface`]), accessibility route
//! ([`M5ExperimentAccessibilityRoute`]), required label ([`M5ExperimentRequiredLabel`]), and
//! downgrade trigger ([`M5ExperimentDowngradeTrigger`]) are reused verbatim from the frozen
//! matrix. This module mints new vocabulary only for what that matrix left implicit about the
//! two components themselves: the derived traceability and export-disposition classes, the
//! bounded lineage-panel and summary-card actions, and the deep-link kinds. No M5 experiment
//! surface invents a second lineage-panel or summary-card grammar, and no panel or card invents
//! an artifact-specific provenance, redaction, retention, or export-scope exception.
//!
//! Raw artifact payloads, pasted paths, credentials, and private endpoints stay outside the
//! export boundary; every context line, deep-link reference, and component identity is carried
//! only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_artifact_lineage_panel_result_summary_card_controls,
    seeded_artifact_lineage_panel_result_summary_card_controls_lineage_panel_broken,
    seeded_artifact_lineage_panel_result_summary_card_controls_summary_card_raw_payload,
    ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_PACKET_ID,
};

// The artifact kind classes and lineage states, the summary content classes and export scopes,
// the disposition vocabulary, and the surface / deployment / consumer / accessibility / label /
// downgrade vocabularies are frozen once, in the experiment-component matrix. This lane reuses
// them verbatim so it never invents a parallel lineage-panel or summary-card vocabulary.
pub use crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix::{
    M5ArtifactKindClass, M5ExperimentAccessibilityRoute, M5ExperimentComponentFamily,
    M5ExperimentConsumerSurface, M5ExperimentDeploymentLine, M5ExperimentDisposition,
    M5ExperimentDowngradeTrigger, M5ExperimentRequiredLabel, M5ExperimentSurfaceFamily,
    M5LineageState, M5SummaryContentClass, M5SummaryExportScope,
    M5_ARTIFACT_LINEAGE_PANEL_SCHEMA_REF, M5_EXPERIMENT_COMPONENT_DOC_REF,
    M5_EXPERIMENT_COMPONENT_SCHEMA_REF, M5_RESULT_SUMMARY_CARD_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by
/// [`ArtifactLineagePanelResultSummaryCardControlsPacket`].
pub const ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_RECORD_KIND: &str =
    "implement_m5_artifact_lineage_panels_and_result_summary_cards_with_producing_run_identity_stale_diverged_notes_include_raw_toggles_and_export_boundary_truth_across_claimed_m5_experiment_surfaces";

/// Schema version for M5 artifact-lineage-panel / result-summary-card control records.
pub const ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-artifact-lineage-panel-result-summary-card-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_DOC_REF: &str =
    "docs/notebooks/m5_artifact_lineage_panel_result_summary_card_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_FIXTURE_DIR: &str =
    "fixtures/ui/m5-artifact-lineage-panel-result-summary-card-controls";

/// Repo-relative path of the checked support-export artifact.
pub const ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_ARTIFACT_REF: &str =
    "artifacts/release/m5-artifact-lineage-panel-result-summary-card-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_CSV_REF: &str =
    "artifacts/release/m5-artifact-lineage-panel-result-summary-card-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_REPORT_REF: &str =
    "artifacts/design/m5-artifact-lineage-panel-result-summary-card.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link an experiment component binds its next step against, so a
/// lineage panel or summary card never routes through an ephemeral overlay — every next step is
/// a stable run, notebook, dataset-catalog, or docs reference the user can reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkKind {
    /// A stable experiment-run object reference.
    RunObject,
    /// A stable notebook / cell location.
    NotebookLocation,
    /// A stable dataset-catalog anchor.
    DatasetCatalogAnchor,
    /// A stable docs anchor.
    DocsAnchor,
    /// No deep link is bound (the component names that it routes nowhere).
    NoDeepLink,
}

impl DeepLinkKind {
    /// Every deep-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RunObject,
        Self::NotebookLocation,
        Self::DatasetCatalogAnchor,
        Self::DocsAnchor,
        Self::NoDeepLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunObject => "run_object",
            Self::NotebookLocation => "notebook_location",
            Self::DatasetCatalogAnchor => "dataset_catalog_anchor",
            Self::DocsAnchor => "docs_anchor",
            Self::NoDeepLink => "no_deep_link",
        }
    }

    /// True when this kind names a resolvable deep-link target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDeepLink)
    }
}

// ---- artifact-lineage-panel vocabulary ----------------------------------

/// Derived traceability class an artifact lineage panel may present.
///
/// This is the lineage honesty axis: the class is derived from the frozen lineage state, never
/// asserted, so a broken, diverged, or unknown-upstream artifact can never present as a
/// fully-traced artifact and a user can always tell how completely the artifact routes back to
/// its producing run and upstream lineage before trusting a compare or share.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactTraceabilityClass {
    /// Lineage is complete end to end.
    FullyTraced,
    /// Artifact was regenerated from a known recipe.
    Regenerated,
    /// Lineage is only partial or derived from a known upstream.
    PartiallyTraced,
    /// Lineage is broken or derived from an unknown upstream (not reliably traced).
    Untraced,
}

impl ArtifactTraceabilityClass {
    /// Every traceability class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullyTraced,
        Self::Regenerated,
        Self::PartiallyTraced,
        Self::Untraced,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyTraced => "fully_traced",
            Self::Regenerated => "regenerated",
            Self::PartiallyTraced => "partially_traced",
            Self::Untraced => "untraced",
        }
    }

    /// True when the artifact is reliably traced (fully traced or regenerated from a recipe).
    pub const fn is_fully_traced(self) -> bool {
        matches!(self, Self::FullyTraced | Self::Regenerated)
    }
}

/// One keyboard-complete default action an artifact lineage panel offers, so a panel never hides
/// its open / trace / export path behind a pointer-only gesture. `OpenArtifact`, `TraceToRun`,
/// and `ExportLineage` are always offered so an artifact's producing run and lineage are
/// actionable — and never anonymous — before any trust decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactLineageAction {
    /// Open the artifact (always available).
    OpenArtifact,
    /// Trace the artifact back to its producing run (always available).
    TraceToRun,
    /// Export the artifact's lineage metadata only (always available).
    ExportLineage,
    /// Open the stable run / notebook / dataset / docs deep link.
    OpenDeepLink,
    /// Compare this artifact's lineage against another.
    CompareLineage,
    /// Copy the stable artifact id.
    CopyArtifactId,
}

impl ArtifactLineageAction {
    /// Every lineage-panel action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenArtifact,
        Self::TraceToRun,
        Self::ExportLineage,
        Self::OpenDeepLink,
        Self::CompareLineage,
        Self::CopyArtifactId,
    ];

    /// The default actions every keyboard-complete lineage panel must offer.
    pub const MANDATORY: [Self; 3] = [Self::OpenArtifact, Self::TraceToRun, Self::ExportLineage];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenArtifact => "open_artifact",
            Self::TraceToRun => "trace_to_run",
            Self::ExportLineage => "export_lineage",
            Self::OpenDeepLink => "open_deep_link",
            Self::CompareLineage => "compare_lineage",
            Self::CopyArtifactId => "copy_artifact_id",
        }
    }
}

/// Disclosures an artifact lineage panel must carry, derived from the lineage state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactLineageDisclosure {
    /// The derived traceability class this panel may present.
    pub traceability_class: ArtifactTraceabilityClass,
    /// Whether the artifact is reliably traced.
    pub is_fully_traced: bool,
    /// Whether the panel must carry an explicit partial-lineage note.
    pub needs_partial_lineage_note: bool,
    /// Whether the panel must carry an explicit stale / diverged (broken-lineage) note.
    pub needs_stale_or_diverged_note: bool,
    /// Whether the panel must carry an explicit unknown-upstream note.
    pub needs_unknown_upstream_note: bool,
    /// Whether the panel must carry an explicit regenerated note.
    pub needs_regenerated_note: bool,
}

/// Resolves the traceability truth an artifact lineage panel may present.
///
/// A `lineage_complete` artifact is fully traced. A `regenerated` artifact is regenerated (must
/// carry an explicit regenerated note). A `lineage_partial` or `derived_upstream_known` artifact
/// is partially traced (must carry an explicit partial note). A `lineage_broken` artifact is
/// untraced and diverged (must carry an explicit stale / diverged note) and a
/// `derived_upstream_unknown` artifact is untraced (must carry an explicit unknown-upstream
/// note), so an artifact that was not reliably traced can never read as fully traced.
pub fn resolve_artifact_lineage(state: M5LineageState) -> ArtifactLineageDisclosure {
    use ArtifactTraceabilityClass as Trace;
    use M5LineageState as State;

    let traceability_class = match state {
        State::LineageComplete => Trace::FullyTraced,
        State::Regenerated => Trace::Regenerated,
        State::LineagePartial | State::DerivedUpstreamKnown => Trace::PartiallyTraced,
        State::LineageBroken | State::DerivedUpstreamUnknown => Trace::Untraced,
    };

    ArtifactLineageDisclosure {
        traceability_class,
        is_fully_traced: traceability_class.is_fully_traced(),
        needs_partial_lineage_note: matches!(traceability_class, Trace::PartiallyTraced),
        needs_stale_or_diverged_note: matches!(state, State::LineageBroken),
        needs_unknown_upstream_note: matches!(state, State::DerivedUpstreamUnknown),
        needs_regenerated_note: matches!(state, State::Regenerated),
    }
}

/// An artifact lineage panel naming its artifact identity, artifact kind, producing run ID,
/// generator step, environment / model fingerprint where relevant, saved scope, derived
/// traceability class, lineage state, bounded open / trace / export actions, and a stable deep
/// link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactLineagePanel {
    /// Frozen component this control implements; must be `artifact_lineage_panel`.
    pub component: M5ExperimentComponentFamily,
    /// Stable lineage-panel id.
    pub panel_id: String,
    /// Human-readable artifact label; required and non-empty.
    pub artifact_label: String,
    /// Artifact kind class, reused from the frozen matrix.
    pub artifact_kind: M5ArtifactKindClass,
    /// Lineage state, reused from the frozen matrix.
    pub lineage_state: M5LineageState,
    /// Derived traceability class (must equal the resolved class).
    pub traceability_class: ArtifactTraceabilityClass,
    /// Whether the panel claims the artifact is fully traced (must equal the derived truth).
    pub claims_fully_traced: bool,
    /// Stable producing run id; always required so an artifact is never an anonymous attachment.
    pub producing_run_id: String,
    /// Human-readable producing run label; always required.
    pub producing_run_label: String,
    /// Generator step note; always required so how the artifact was produced stays explicit.
    pub generator_step_note: String,
    /// Environment / model fingerprint reference; always required (may name that none applies).
    pub environment_fingerprint_ref: String,
    /// Saved scope note; always required so where the artifact is saved stays explicit.
    pub saved_scope_note: String,
    /// Partial-lineage note; required when the lineage is partially traced.
    pub partial_lineage_note: String,
    /// Stale / diverged note; required when the lineage is broken.
    pub stale_or_diverged_note: String,
    /// Unknown-upstream note; required when the artifact is derived from an unknown upstream.
    pub unknown_upstream_note: String,
    /// Regenerated note; required when the artifact was regenerated.
    pub regenerated_note: String,
    /// Lineage / run note; always required so the producing run and lineage state stay explicit.
    pub lineage_and_run_note: String,
    /// Context note; always required so the panel names what to check before compare or share.
    pub context_note: String,
    /// Kind of stable deep link this panel binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include open / trace / export).
    pub panel_actions: Vec<ArtifactLineageAction>,
    /// Dispositions this panel binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5ExperimentDisposition>,
    /// Downgrade triggers this panel can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Mandatory labels this panel can show (must include the mandatory labels).
    pub required_labels: Vec<M5ExperimentRequiredLabel>,
    /// Claimed M5 surface families that render this panel.
    pub surface_families: Vec<M5ExperimentSurfaceFamily>,
    /// Deployment lines this panel keeps the same truth across.
    pub deployment_lines: Vec<M5ExperimentDeploymentLine>,
    /// Non-visual accessibility routes this panel offers.
    pub accessibility_routes: Vec<M5ExperimentAccessibilityRoute>,
    /// Experiment subsystems that consume this panel's projection.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this panel.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks provenance or sensitivity state. MUST be `false`.
    pub masks_provenance_or_sensitivity_state: bool,
    /// Hard invariant: never hides the producing run or lineage state. MUST be `false`.
    pub hides_producing_run_or_lineage_state: bool,
    /// Hard invariant: never exposes a raw payload by default. MUST be `false`.
    pub exposes_raw_payload_by_default: bool,
    /// Hard invariant: never implies apples-to-apples without parity. MUST be `false`.
    pub implies_apples_to_apples_without_parity: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl ArtifactLineagePanel {
    /// Lineage disclosures this panel must carry, derived from its state.
    pub fn lineage_disclosure(&self) -> ArtifactLineageDisclosure {
        resolve_artifact_lineage(self.lineage_state)
    }

    /// Whether the panel offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<ArtifactLineageAction> = self.panel_actions.iter().copied().collect();
        ArtifactLineageAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the panel declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ExperimentRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ExperimentRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the panel offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.panel_actions
            .contains(&ArtifactLineageAction::OpenDeepLink)
    }
}

// ---- result-summary-card vocabulary -------------------------------------

/// Derived export disposition a result summary card may present.
///
/// This is the export honesty axis: the disposition is derived from the frozen export scope,
/// never asserted, so a raw-scope export can never present as a metadata-only export and a user
/// can always tell whether a shared summary is metadata-safe, evidence-scoped, raw-included,
/// redacted, or withheld before sharing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SummaryExportDisposition {
    /// A summary- / metadata-only, metadata-safe export.
    MetadataSafe,
    /// An export that includes evidence links.
    EvidenceScoped,
    /// An export that includes a raw payload.
    RawIncluded,
    /// A redacted export.
    Redacted,
    /// A withheld export.
    Withheld,
}

impl SummaryExportDisposition {
    /// Every export disposition, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MetadataSafe,
        Self::EvidenceScoped,
        Self::RawIncluded,
        Self::Redacted,
        Self::Withheld,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafe => "metadata_safe",
            Self::EvidenceScoped => "evidence_scoped",
            Self::RawIncluded => "raw_included",
            Self::Redacted => "redacted",
            Self::Withheld => "withheld",
        }
    }

    /// True when the export is metadata-only (metadata-safe).
    pub const fn is_metadata_only(self) -> bool {
        matches!(self, Self::MetadataSafe)
    }
}

/// One keyboard-complete default action a result summary card offers, so a card never hides its
/// safe alternative behind a pointer-only gesture. `ReviewExportScope` and `ShareSummaryOnly`
/// are always offered so the summary-only, metadata-safe alternative stays visible before any
/// raw-inclusive export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SummaryCardAction {
    /// Review the export scope (always available).
    ReviewExportScope,
    /// Share the summary only — the metadata-safe alternative (always available).
    ShareSummaryOnly,
    /// Include the raw payload — the explicit, warned opt-in.
    IncludeRawPayload,
    /// Export the evidence links.
    ExportEvidence,
    /// Open the stable run / notebook / dataset / docs deep link.
    OpenDeepLink,
    /// Copy the stable summary id.
    CopySummaryId,
}

impl SummaryCardAction {
    /// Every summary-card action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewExportScope,
        Self::ShareSummaryOnly,
        Self::IncludeRawPayload,
        Self::ExportEvidence,
        Self::OpenDeepLink,
        Self::CopySummaryId,
    ];

    /// The default actions every keyboard-complete summary card must offer.
    pub const MANDATORY: [Self; 2] = [Self::ReviewExportScope, Self::ShareSummaryOnly];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewExportScope => "review_export_scope",
            Self::ShareSummaryOnly => "share_summary_only",
            Self::IncludeRawPayload => "include_raw_payload",
            Self::ExportEvidence => "export_evidence",
            Self::OpenDeepLink => "open_deep_link",
            Self::CopySummaryId => "copy_summary_id",
        }
    }
}

/// Disclosures a result summary card must carry, derived from the export scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SummaryExportDisclosure {
    /// The derived export disposition this card may present.
    pub export_disposition: SummaryExportDisposition,
    /// Whether the export includes a raw payload.
    pub includes_raw_payload: bool,
    /// Whether the export is metadata-only.
    pub is_metadata_only: bool,
    /// Whether the export is withheld.
    pub is_withheld: bool,
    /// Whether the card must carry an explicit raw-inclusion warning.
    pub needs_raw_inclusion_warning: bool,
    /// Whether the card must carry an explicit withheld note.
    pub needs_withheld_note: bool,
    /// Whether the card must carry an explicit redaction note.
    pub needs_redaction_note: bool,
}

/// Resolves the export truth a result summary card may present.
///
/// A `summary_scope` or `metadata_scope` export is metadata-safe. An `evidence_scope` export is
/// evidence-scoped. A `raw_scope` export is raw-included (must carry an explicit raw-inclusion
/// warning). A `redacted_scope` export is redacted (must carry an explicit redaction note). An
/// `export_withheld` export is withheld (must carry an explicit withheld note), so a raw payload
/// is never included by default and the include-raw toggle stays an explicit, warned choice.
pub fn resolve_summary_export(scope: M5SummaryExportScope) -> SummaryExportDisclosure {
    use M5SummaryExportScope as Scope;
    use SummaryExportDisposition as Disposition;

    let export_disposition = match scope {
        Scope::SummaryScope | Scope::MetadataScope => Disposition::MetadataSafe,
        Scope::EvidenceScope => Disposition::EvidenceScoped,
        Scope::RawScope => Disposition::RawIncluded,
        Scope::RedactedScope => Disposition::Redacted,
        Scope::ExportWithheld => Disposition::Withheld,
    };

    SummaryExportDisclosure {
        export_disposition,
        includes_raw_payload: matches!(export_disposition, Disposition::RawIncluded),
        is_metadata_only: export_disposition.is_metadata_only(),
        is_withheld: matches!(export_disposition, Disposition::Withheld),
        needs_raw_inclusion_warning: matches!(export_disposition, Disposition::RawIncluded),
        needs_withheld_note: matches!(export_disposition, Disposition::Withheld),
        needs_redaction_note: matches!(export_disposition, Disposition::Redacted),
    }
}

/// A result summary card naming its summary content class, export scope, headline metrics,
/// artifact count, freshness, support / report scope, include-raw toggle, provenance note,
/// derived export disposition, explicit summary-versus-evidence-versus-raw handoff choice,
/// bounded review / summary-only actions, and a stable deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultSummaryCard {
    /// Frozen component this control implements; must be `result_summary_card`.
    pub component: M5ExperimentComponentFamily,
    /// Stable summary-card id.
    pub card_id: String,
    /// Human-readable summary-card label; required and non-empty.
    pub card_label: String,
    /// Summary content class, reused from the frozen matrix.
    pub summary_content_class: M5SummaryContentClass,
    /// Export scope, reused from the frozen matrix.
    pub export_scope_state: M5SummaryExportScope,
    /// Derived export disposition (must equal the resolved disposition).
    pub export_disposition: SummaryExportDisposition,
    /// Whether the card claims the export includes a raw payload (must equal derived truth).
    pub claims_includes_raw_payload: bool,
    /// Whether the card claims the export is metadata-only (must equal derived truth).
    pub claims_metadata_only: bool,
    /// Whether the include-raw toggle is on (must equal the derived raw-inclusion truth).
    pub include_raw_toggle_on: bool,
    /// Headline metric note; always required so the headline result stays explicit.
    pub headline_metric_note: String,
    /// Artifact count label; always required so how many artifacts back the summary stays clear.
    pub artifact_count_label: String,
    /// Freshness note; always required so a stale summary stays visible.
    pub freshness_note: String,
    /// Support / report scope note; always required so the report scope stays explicit.
    pub support_report_scope_note: String,
    /// Provenance note; always required so the producing run and lineage behind the summary stay
    /// explicit.
    pub provenance_note: String,
    /// Raw-inclusion warning; required when the export includes a raw payload.
    pub raw_inclusion_warning: String,
    /// Withheld note; required when the export is withheld.
    pub withheld_note: String,
    /// Redaction note; required when the export is redacted.
    pub redaction_note: String,
    /// Summary-versus-evidence-versus-raw handoff note; always required so the handoff choice
    /// stays explicit.
    pub summary_evidence_raw_handoff_note: String,
    /// Context note; always required so the card names what to check before a share.
    pub context_note: String,
    /// Kind of stable deep link this card binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include review / share-summary-only).
    pub card_actions: Vec<SummaryCardAction>,
    /// Dispositions this card binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5ExperimentDisposition>,
    /// Downgrade triggers this card can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Mandatory labels this card can show (must include the mandatory labels).
    pub required_labels: Vec<M5ExperimentRequiredLabel>,
    /// Claimed M5 surface families that render this card.
    pub surface_families: Vec<M5ExperimentSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5ExperimentDeploymentLine>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5ExperimentAccessibilityRoute>,
    /// Experiment subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks provenance or sensitivity state. MUST be `false`.
    pub masks_provenance_or_sensitivity_state: bool,
    /// Hard invariant: never hides the producing run or lineage state. MUST be `false`.
    pub hides_producing_run_or_lineage_state: bool,
    /// Hard invariant: never exposes a raw payload by default. MUST be `false`.
    pub exposes_raw_payload_by_default: bool,
    /// Hard invariant: never implies apples-to-apples without parity. MUST be `false`.
    pub implies_apples_to_apples_without_parity: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl ResultSummaryCard {
    /// Export disclosures this card must carry, derived from its scope.
    pub fn export_disclosure(&self) -> SummaryExportDisclosure {
        resolve_summary_export(self.export_scope_state)
    }

    /// Whether the card offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<SummaryCardAction> = self.card_actions.iter().copied().collect();
        SummaryCardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the card declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ExperimentRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ExperimentRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the card offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.card_actions.contains(&SummaryCardAction::OpenDeepLink)
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance lineage / summary review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactSummaryReview {
    /// The lineage panel names its artifact and producing run.
    pub lineage_panel_shows_artifact_and_producing_run: bool,
    /// The lineage panel names its lineage state and generator step.
    pub lineage_panel_shows_lineage_state_and_generator_step: bool,
    /// The lineage panel offers open, trace-to-run, and lineage-export.
    pub lineage_panel_offers_open_trace_export: bool,
    /// The summary card names its headline metrics and export scope.
    pub summary_card_shows_headline_and_export_scope: bool,
    /// The summary card offers review and share-summary-only.
    pub summary_card_offers_review_and_summary_only: bool,
    /// Traceability and export scope are derived from state, never asserted.
    pub traceability_and_export_derived_never_asserted: bool,
    /// An untraced or broken artifact is never shown as fully traced.
    pub untraced_or_broken_never_shown_as_traced: bool,
    /// Every artifact routes back to a producing run, never an anonymous attachment.
    pub every_artifact_routes_to_producing_run: bool,
    /// A raw payload is never included by default.
    pub raw_payload_never_included_by_default: bool,
    /// The include-raw toggle is an explicit choice, never an accidental default.
    pub include_raw_is_explicit_choice_never_default: bool,
    /// Summary-versus-evidence-versus-raw export scope stays visible before a share.
    pub summary_evidence_raw_scope_visible_before_share: bool,
    /// Every next step names one stable run / notebook / dataset / docs deep link.
    pub every_next_step_names_stable_deep_link: bool,
    /// Panels and cards reuse Aureline's existing provenance / export vocabulary.
    pub reuses_existing_provenance_export_vocabulary: bool,
    /// Provenance and sensitivity state stays visible.
    pub provenance_and_sensitivity_state_visible: bool,
    /// Cached, offline, and local-only state stays visible.
    pub cached_offline_local_only_state_visible: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// The components stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl ArtifactSummaryReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.lineage_panel_shows_artifact_and_producing_run
            && self.lineage_panel_shows_lineage_state_and_generator_step
            && self.lineage_panel_offers_open_trace_export
            && self.summary_card_shows_headline_and_export_scope
            && self.summary_card_offers_review_and_summary_only
            && self.traceability_and_export_derived_never_asserted
            && self.untraced_or_broken_never_shown_as_traced
            && self.every_artifact_routes_to_producing_run
            && self.raw_payload_never_included_by_default
            && self.include_raw_is_explicit_choice_never_default
            && self.summary_evidence_raw_scope_visible_before_share
            && self.every_next_step_names_stable_deep_link
            && self.reuses_existing_provenance_export_vocabulary
            && self.provenance_and_sensitivity_state_visible
            && self.cached_offline_local_only_state_visible
            && self.no_surface_invents_alternate_state_label
            && self.components_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactSummaryConsumerProjection {
    /// The lineage surface reads a single canonical source.
    pub lineage_ui_reads_single_source: bool,
    /// The result-summary surface reads a single canonical source.
    pub result_summary_surface_reads_single_source: bool,
    /// The producing run and lineage are visible before a trust decision.
    pub producing_run_and_lineage_visible_before_trust: bool,
    /// The export scope is visible before a share.
    pub export_scope_visible_before_share: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
    /// Help / docs shows component truth.
    pub help_docs_shows_component_truth: bool,
}

impl ArtifactSummaryConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.lineage_ui_reads_single_source
            && self.result_summary_surface_reads_single_source
            && self.producing_run_and_lineage_visible_before_trust
            && self.export_scope_visible_before_share
            && self.support_export_shows_component_truth
            && self.help_docs_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactSummaryProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for
/// [`ArtifactLineagePanelResultSummaryCardControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactLineagePanelResultSummaryCardControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Artifact lineage panels.
    pub lineage_panels: Vec<ArtifactLineagePanel>,
    /// Result summary cards.
    pub summary_cards: Vec<ResultSummaryCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Lineage / summary review block.
    pub artifact_review: ArtifactSummaryReview,
    /// Consumer projection block.
    pub consumer_projection: ArtifactSummaryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ArtifactSummaryProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe artifact-lineage-panel / result-summary-card controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactLineagePanelResultSummaryCardControlsPacket {
    /// Record kind; must equal
    /// [`ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Artifact lineage panels.
    pub lineage_panels: Vec<ArtifactLineagePanel>,
    /// Result summary cards.
    pub summary_cards: Vec<ResultSummaryCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Lineage / summary review block.
    pub artifact_review: ArtifactSummaryReview,
    /// Consumer projection block.
    pub consumer_projection: ArtifactSummaryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ArtifactSummaryProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ArtifactLineagePanelResultSummaryCardControlsPacket {
    /// Builds an artifact-lineage-panel / result-summary-card controls packet from stable-lane
    /// input.
    pub fn new(input: ArtifactLineagePanelResultSummaryCardControlsPacketInput) -> Self {
        Self {
            record_kind: ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_RECORD_KIND.to_owned(),
            schema_version: ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            lineage_panels: input.lineage_panels,
            summary_cards: input.summary_cards,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            artifact_review: input.artifact_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the artifact-lineage-panel / result-summary-card control invariants.
    pub fn validate(&self) -> Vec<ArtifactLineagePanelResultSummaryCardViolation> {
        let mut violations = Vec::new();

        if self.record_kind != ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_RECORD_KIND {
            violations.push(ArtifactLineagePanelResultSummaryCardViolation::WrongRecordKind);
        }
        if self.schema_version != ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_SCHEMA_VERSION {
            violations.push(ArtifactLineagePanelResultSummaryCardViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ArtifactLineagePanelResultSummaryCardViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_lineage_panels(self, &mut violations);
        validate_summary_cards(self, &mut violations);

        if !self.artifact_review.all_hold() {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::ArtifactReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("artifact lineage summary card packet serializes"),
        ) {
            violations.push(ArtifactLineagePanelResultSummaryCardViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("artifact lineage summary card packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component,id,state_or_scope,kind_or_content,derived,safe_flag,deep_link_kind\n",
        );
        for panel in &self.lineage_panels {
            let disclosure = panel.lineage_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "artifact_lineage_panel",
                csv_field(&panel.panel_id),
                panel.lineage_state.as_str(),
                panel.artifact_kind.as_str(),
                disclosure.traceability_class.as_str(),
                disclosure.is_fully_traced,
                panel.deep_link_kind.as_str(),
            ));
        }
        for card in &self.summary_cards {
            let disclosure = card.export_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "result_summary_card",
                csv_field(&card.card_id),
                card.export_scope_state.as_str(),
                card.summary_content_class.as_str(),
                disclosure.export_disposition.as_str(),
                disclosure.is_metadata_only,
                card.deep_link_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let untraced = self
            .lineage_panels
            .iter()
            .filter(|panel| !panel.lineage_disclosure().is_fully_traced)
            .count();
        let raw_exports = self
            .summary_cards
            .iter()
            .filter(|card| card.export_disclosure().includes_raw_payload)
            .count();

        let mut out = String::new();
        out.push_str("# Artifact lineage panels and result summary cards\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Artifact lineage panels: {} ({} not fully traced)\n",
            self.lineage_panels.len(),
            untraced
        ));
        out.push_str(&format!(
            "- Result summary cards: {} ({} include a raw payload)\n",
            self.summary_cards.len(),
            raw_exports
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Artifact lineage panels\n\n");
        for panel in &self.lineage_panels {
            let disclosure = panel.lineage_disclosure();
            out.push_str(&format!(
                "- **{}** — kind `{}`, lineage `{}` → `{}`, producing run `{}`, deep link `{}`\n",
                panel.artifact_label,
                panel.artifact_kind.as_str(),
                panel.lineage_state.as_str(),
                disclosure.traceability_class.as_str(),
                panel.producing_run_id,
                panel.deep_link_kind.as_str(),
            ));
        }

        out.push_str("\n## Result summary cards\n\n");
        for card in &self.summary_cards {
            let disclosure = card.export_disclosure();
            out.push_str(&format!(
                "- **{}** — content `{}`, scope `{}` → `{}`, include-raw `{}`, deep link `{}`\n",
                card.card_label,
                card.summary_content_class.as_str(),
                card.export_scope_state.as_str(),
                disclosure.export_disposition.as_str(),
                card.include_raw_toggle_on,
                card.deep_link_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in artifact-lineage-panel / result-summary-card
/// export.
#[derive(Debug)]
pub enum ArtifactLineagePanelResultSummaryCardArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ArtifactLineagePanelResultSummaryCardViolation>),
}

impl fmt::Display for ArtifactLineagePanelResultSummaryCardArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "artifact lineage summary card export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "artifact lineage summary card export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ArtifactLineagePanelResultSummaryCardArtifactError {}

/// Validation failures emitted by
/// [`ArtifactLineagePanelResultSummaryCardControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactLineagePanelResultSummaryCardViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No artifact lineage panels are present.
    LineagePanelsMissing,
    /// An artifact lineage panel is incomplete.
    LineagePanelIncomplete,
    /// An artifact lineage panel carries the wrong frozen component class.
    LineagePanelWrongComponentClass,
    /// A lineage panel misrepresents its derived traceability class.
    TraceabilityMisrepresented,
    /// A lineage panel does not name its producing run.
    ProducingRunMissing,
    /// A lineage panel does not name its generator step.
    GeneratorStepMissing,
    /// A lineage panel does not name its environment / model fingerprint.
    EnvironmentFingerprintMissing,
    /// A lineage panel does not name its saved scope.
    SavedScopeMissing,
    /// A partially traced artifact does not name its partial-lineage state.
    PartialLineageNoteMissing,
    /// A broken-lineage artifact does not name its stale / diverged state.
    StaleOrDivergedNoteMissing,
    /// An unknown-upstream artifact does not name its unknown upstream.
    UnknownUpstreamNoteMissing,
    /// A regenerated artifact does not name its regenerated state.
    RegeneratedNoteMissing,
    /// A lineage panel does not name its lineage / run truth.
    LineageAndRunNoteMissing,
    /// A lineage panel omits a mandatory open / trace / export action.
    LineagePanelActionsIncomplete,
    /// The lineage panels do not cover every derived traceability class.
    TraceabilityClassCoverageMissing,
    /// The lineage panels do not cover every artifact kind class.
    ArtifactKindCoverageMissing,
    /// The lineage panels do not cover every lineage state.
    LineageStateCoverageMissing,
    /// No result summary cards are present.
    SummaryCardsMissing,
    /// A result summary card is incomplete.
    SummaryCardIncomplete,
    /// A result summary card carries the wrong frozen component class.
    SummaryCardWrongComponentClass,
    /// A summary card misrepresents its derived export disposition or include-raw toggle.
    ExportDispositionMisrepresented,
    /// A summary card does not name its headline metric.
    HeadlineMetricNoteMissing,
    /// A summary card does not name its artifact count.
    ArtifactCountMissing,
    /// A summary card does not name its freshness.
    FreshnessNoteMissing,
    /// A summary card does not name its support / report scope.
    SupportReportScopeNoteMissing,
    /// A summary card does not name its provenance.
    ProvenanceNoteMissing,
    /// A raw-included export does not name its raw-inclusion warning.
    RawInclusionWarningMissing,
    /// A withheld export does not name its withheld state.
    WithheldNoteMissing,
    /// A redacted export does not name its redaction state.
    SummaryRedactionNoteMissing,
    /// A summary card does not name its summary-versus-evidence-versus-raw handoff choice.
    SummaryEvidenceRawHandoffNoteMissing,
    /// A summary card omits a mandatory review / share-summary-only action.
    SummaryCardActionsIncomplete,
    /// The summary cards do not cover every derived export disposition.
    ExportDispositionCoverageMissing,
    /// The summary cards do not cover every summary content class.
    SummaryContentCoverageMissing,
    /// The summary cards do not cover every export scope.
    ExportScopeCoverageMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A component offers a deep-link action but its deep link does not resolve exactly.
    DeepLinkUnresolved,
    /// A component names a deep-link kind but not its stable reference.
    DeepLinkRefMissing,
    /// A component does not bind any disposition.
    DispositionsMissing,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A component does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A component masks its provenance or sensitivity state.
    ProvenanceOrSensitivityStateMasked,
    /// A component hides the producing run or lineage state.
    ProducingRunOrLineageStateHidden,
    /// A component exposes a raw payload by default.
    RawPayloadExposedByDefault,
    /// A component implies apples-to-apples without parity evidence.
    ApplesToApplesImpliedWithoutParity,
    /// A component invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Lineage / summary review does not satisfy required invariants.
    ArtifactReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl ArtifactLineagePanelResultSummaryCardViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::LineagePanelsMissing => "lineage_panels_missing",
            Self::LineagePanelIncomplete => "lineage_panel_incomplete",
            Self::LineagePanelWrongComponentClass => "lineage_panel_wrong_component_class",
            Self::TraceabilityMisrepresented => "traceability_misrepresented",
            Self::ProducingRunMissing => "producing_run_missing",
            Self::GeneratorStepMissing => "generator_step_missing",
            Self::EnvironmentFingerprintMissing => "environment_fingerprint_missing",
            Self::SavedScopeMissing => "saved_scope_missing",
            Self::PartialLineageNoteMissing => "partial_lineage_note_missing",
            Self::StaleOrDivergedNoteMissing => "stale_or_diverged_note_missing",
            Self::UnknownUpstreamNoteMissing => "unknown_upstream_note_missing",
            Self::RegeneratedNoteMissing => "regenerated_note_missing",
            Self::LineageAndRunNoteMissing => "lineage_and_run_note_missing",
            Self::LineagePanelActionsIncomplete => "lineage_panel_actions_incomplete",
            Self::TraceabilityClassCoverageMissing => "traceability_class_coverage_missing",
            Self::ArtifactKindCoverageMissing => "artifact_kind_coverage_missing",
            Self::LineageStateCoverageMissing => "lineage_state_coverage_missing",
            Self::SummaryCardsMissing => "summary_cards_missing",
            Self::SummaryCardIncomplete => "summary_card_incomplete",
            Self::SummaryCardWrongComponentClass => "summary_card_wrong_component_class",
            Self::ExportDispositionMisrepresented => "export_disposition_misrepresented",
            Self::HeadlineMetricNoteMissing => "headline_metric_note_missing",
            Self::ArtifactCountMissing => "artifact_count_missing",
            Self::FreshnessNoteMissing => "freshness_note_missing",
            Self::SupportReportScopeNoteMissing => "support_report_scope_note_missing",
            Self::ProvenanceNoteMissing => "provenance_note_missing",
            Self::RawInclusionWarningMissing => "raw_inclusion_warning_missing",
            Self::WithheldNoteMissing => "withheld_note_missing",
            Self::SummaryRedactionNoteMissing => "summary_redaction_note_missing",
            Self::SummaryEvidenceRawHandoffNoteMissing => {
                "summary_evidence_raw_handoff_note_missing"
            }
            Self::SummaryCardActionsIncomplete => "summary_card_actions_incomplete",
            Self::ExportDispositionCoverageMissing => "export_disposition_coverage_missing",
            Self::SummaryContentCoverageMissing => "summary_content_coverage_missing",
            Self::ExportScopeCoverageMissing => "export_scope_coverage_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::DeepLinkUnresolved => "deep_link_unresolved",
            Self::DeepLinkRefMissing => "deep_link_ref_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ProvenanceOrSensitivityStateMasked => "provenance_or_sensitivity_state_masked",
            Self::ProducingRunOrLineageStateHidden => "producing_run_or_lineage_state_hidden",
            Self::RawPayloadExposedByDefault => "raw_payload_exposed_by_default",
            Self::ApplesToApplesImpliedWithoutParity => "apples_to_apples_implied_without_parity",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ArtifactReviewIncomplete => "artifact_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable artifact-lineage-panel / result-summary-card
/// export.
pub fn current_artifact_lineage_panel_result_summary_card_export() -> Result<
    ArtifactLineagePanelResultSummaryCardControlsPacket,
    ArtifactLineagePanelResultSummaryCardArtifactError,
> {
    let packet: ArtifactLineagePanelResultSummaryCardControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-artifact-lineage-panel-result-summary-card-proof/support_export.json"
        )))
        .map_err(ArtifactLineagePanelResultSummaryCardArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ArtifactLineagePanelResultSummaryCardArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &ArtifactLineagePanelResultSummaryCardControlsPacket,
    violations: &mut Vec<ArtifactLineagePanelResultSummaryCardViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_SCHEMA_REF,
        ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_DOC_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_DOC_REF,
        M5_ARTIFACT_LINEAGE_PANEL_SCHEMA_REF,
        M5_RESULT_SUMMARY_CARD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ArtifactLineagePanelResultSummaryCardViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_lineage_panels(
    packet: &ArtifactLineagePanelResultSummaryCardControlsPacket,
    violations: &mut Vec<ArtifactLineagePanelResultSummaryCardViolation>,
) {
    if packet.lineage_panels.is_empty() {
        violations.push(ArtifactLineagePanelResultSummaryCardViolation::LineagePanelsMissing);
        return;
    }

    let mut traceability_classes: BTreeSet<ArtifactTraceabilityClass> = BTreeSet::new();
    let mut kinds: BTreeSet<M5ArtifactKindClass> = BTreeSet::new();
    let mut states: BTreeSet<M5LineageState> = BTreeSet::new();

    for panel in &packet.lineage_panels {
        let disclosure = panel.lineage_disclosure();
        traceability_classes.insert(disclosure.traceability_class);
        kinds.insert(panel.artifact_kind);
        states.insert(panel.lineage_state);

        if panel.panel_id.trim().is_empty()
            || panel.artifact_label.trim().is_empty()
            || panel.fields_shown.is_empty()
            || panel.surface_families.is_empty()
            || panel.deployment_lines.is_empty()
            || panel.consumer_surfaces.is_empty()
            || panel.source_contract_refs.is_empty()
        {
            violations.push(ArtifactLineagePanelResultSummaryCardViolation::LineagePanelIncomplete);
        }
        if panel.component != M5ExperimentComponentFamily::ArtifactLineagePanel {
            violations.push(
                ArtifactLineagePanelResultSummaryCardViolation::LineagePanelWrongComponentClass,
            );
        }
        if panel.traceability_class != disclosure.traceability_class
            || panel.claims_fully_traced != disclosure.is_fully_traced
        {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::TraceabilityMisrepresented);
        }
        if panel.producing_run_id.trim().is_empty() || panel.producing_run_label.trim().is_empty() {
            violations.push(ArtifactLineagePanelResultSummaryCardViolation::ProducingRunMissing);
        }
        if panel.generator_step_note.trim().is_empty() {
            violations.push(ArtifactLineagePanelResultSummaryCardViolation::GeneratorStepMissing);
        }
        if panel.environment_fingerprint_ref.trim().is_empty() {
            violations.push(
                ArtifactLineagePanelResultSummaryCardViolation::EnvironmentFingerprintMissing,
            );
        }
        if panel.saved_scope_note.trim().is_empty() {
            violations.push(ArtifactLineagePanelResultSummaryCardViolation::SavedScopeMissing);
        }
        if disclosure.needs_partial_lineage_note && panel.partial_lineage_note.trim().is_empty() {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::PartialLineageNoteMissing);
        }
        if disclosure.needs_stale_or_diverged_note && panel.stale_or_diverged_note.trim().is_empty()
        {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::StaleOrDivergedNoteMissing);
        }
        if disclosure.needs_unknown_upstream_note && panel.unknown_upstream_note.trim().is_empty() {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::UnknownUpstreamNoteMissing);
        }
        if disclosure.needs_regenerated_note && panel.regenerated_note.trim().is_empty() {
            violations.push(ArtifactLineagePanelResultSummaryCardViolation::RegeneratedNoteMissing);
        }
        if panel.lineage_and_run_note.trim().is_empty() {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::LineageAndRunNoteMissing);
        }
        if !panel.declares_mandatory_actions() {
            violations.push(
                ArtifactLineagePanelResultSummaryCardViolation::LineagePanelActionsIncomplete,
            );
        }
        validate_deep_link(
            panel.offers_deep_link_action(),
            panel.deep_link_kind,
            &panel.deep_link_ref,
            &panel.context_note,
            violations,
        );
        validate_common_control(
            &panel.dispositions,
            &panel.downgrade_triggers,
            panel.declares_mandatory_labels(),
            &panel.accessibility_routes,
            ControlInvariants {
                masks_provenance_or_sensitivity_state: panel.masks_provenance_or_sensitivity_state,
                hides_producing_run_or_lineage_state: panel.hides_producing_run_or_lineage_state,
                exposes_raw_payload_by_default: panel.exposes_raw_payload_by_default,
                implies_apples_to_apples_without_parity: panel
                    .implies_apples_to_apples_without_parity,
                invents_alternate_state_label: panel.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in ArtifactTraceabilityClass::ALL {
        if !traceability_classes.contains(&required) {
            violations.push(
                ArtifactLineagePanelResultSummaryCardViolation::TraceabilityClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5ArtifactKindClass::ALL {
        if !kinds.contains(&required) {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::ArtifactKindCoverageMissing);
            break;
        }
    }
    for required in M5LineageState::ALL {
        if !states.contains(&required) {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::LineageStateCoverageMissing);
            break;
        }
    }
}

fn validate_summary_cards(
    packet: &ArtifactLineagePanelResultSummaryCardControlsPacket,
    violations: &mut Vec<ArtifactLineagePanelResultSummaryCardViolation>,
) {
    if packet.summary_cards.is_empty() {
        violations.push(ArtifactLineagePanelResultSummaryCardViolation::SummaryCardsMissing);
        return;
    }

    let mut dispositions: BTreeSet<SummaryExportDisposition> = BTreeSet::new();
    let mut contents: BTreeSet<M5SummaryContentClass> = BTreeSet::new();
    let mut scopes: BTreeSet<M5SummaryExportScope> = BTreeSet::new();

    for card in &packet.summary_cards {
        let disclosure = card.export_disclosure();
        dispositions.insert(disclosure.export_disposition);
        contents.insert(card.summary_content_class);
        scopes.insert(card.export_scope_state);

        if card.card_id.trim().is_empty()
            || card.card_label.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.surface_families.is_empty()
            || card.deployment_lines.is_empty()
            || card.consumer_surfaces.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(ArtifactLineagePanelResultSummaryCardViolation::SummaryCardIncomplete);
        }
        if card.component != M5ExperimentComponentFamily::ResultSummaryCard {
            violations.push(
                ArtifactLineagePanelResultSummaryCardViolation::SummaryCardWrongComponentClass,
            );
        }
        if card.export_disposition != disclosure.export_disposition
            || card.claims_includes_raw_payload != disclosure.includes_raw_payload
            || card.claims_metadata_only != disclosure.is_metadata_only
            || card.include_raw_toggle_on != disclosure.includes_raw_payload
        {
            violations.push(
                ArtifactLineagePanelResultSummaryCardViolation::ExportDispositionMisrepresented,
            );
        }
        if card.headline_metric_note.trim().is_empty() {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::HeadlineMetricNoteMissing);
        }
        if card.artifact_count_label.trim().is_empty() {
            violations.push(ArtifactLineagePanelResultSummaryCardViolation::ArtifactCountMissing);
        }
        if card.freshness_note.trim().is_empty() {
            violations.push(ArtifactLineagePanelResultSummaryCardViolation::FreshnessNoteMissing);
        }
        if card.support_report_scope_note.trim().is_empty() {
            violations.push(
                ArtifactLineagePanelResultSummaryCardViolation::SupportReportScopeNoteMissing,
            );
        }
        if card.provenance_note.trim().is_empty() {
            violations.push(ArtifactLineagePanelResultSummaryCardViolation::ProvenanceNoteMissing);
        }
        if disclosure.needs_raw_inclusion_warning && card.raw_inclusion_warning.trim().is_empty() {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::RawInclusionWarningMissing);
        }
        if disclosure.needs_withheld_note && card.withheld_note.trim().is_empty() {
            violations.push(ArtifactLineagePanelResultSummaryCardViolation::WithheldNoteMissing);
        }
        if disclosure.needs_redaction_note && card.redaction_note.trim().is_empty() {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::SummaryRedactionNoteMissing);
        }
        if card.summary_evidence_raw_handoff_note.trim().is_empty() {
            violations.push(
                ArtifactLineagePanelResultSummaryCardViolation::SummaryEvidenceRawHandoffNoteMissing,
            );
        }
        if !card.declares_mandatory_actions() {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::SummaryCardActionsIncomplete);
        }
        validate_deep_link(
            card.offers_deep_link_action(),
            card.deep_link_kind,
            &card.deep_link_ref,
            &card.context_note,
            violations,
        );
        validate_common_control(
            &card.dispositions,
            &card.downgrade_triggers,
            card.declares_mandatory_labels(),
            &card.accessibility_routes,
            ControlInvariants {
                masks_provenance_or_sensitivity_state: card.masks_provenance_or_sensitivity_state,
                hides_producing_run_or_lineage_state: card.hides_producing_run_or_lineage_state,
                exposes_raw_payload_by_default: card.exposes_raw_payload_by_default,
                implies_apples_to_apples_without_parity: card
                    .implies_apples_to_apples_without_parity,
                invents_alternate_state_label: card.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in SummaryExportDisposition::ALL {
        if !dispositions.contains(&required) {
            violations.push(
                ArtifactLineagePanelResultSummaryCardViolation::ExportDispositionCoverageMissing,
            );
            break;
        }
    }
    for required in M5SummaryContentClass::ALL {
        if !contents.contains(&required) {
            violations.push(
                ArtifactLineagePanelResultSummaryCardViolation::SummaryContentCoverageMissing,
            );
            break;
        }
    }
    for required in M5SummaryExportScope::ALL {
        if !scopes.contains(&required) {
            violations
                .push(ArtifactLineagePanelResultSummaryCardViolation::ExportScopeCoverageMissing);
            break;
        }
    }
}

/// Validates the context and stable deep-link truth shared by both component vectors.
///
/// A component that offers a deep-link action must name a resolvable deep-link kind, a
/// component that names a resolvable kind must carry its stable reference, and every component
/// must name its context — so a next step is never an ephemeral overlay or hidden route.
fn validate_deep_link(
    offers_deep_link_action: bool,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    context_note: &str,
    violations: &mut Vec<ArtifactLineagePanelResultSummaryCardViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(ArtifactLineagePanelResultSummaryCardViolation::ContextNoteMissing);
    }
    if offers_deep_link_action && !deep_link_kind.is_resolvable() {
        violations.push(ArtifactLineagePanelResultSummaryCardViolation::DeepLinkUnresolved);
    }
    if deep_link_kind.is_resolvable() && deep_link_ref.trim().is_empty() {
        violations.push(ArtifactLineagePanelResultSummaryCardViolation::DeepLinkRefMissing);
    }
}

/// The five hard-invariant bools every component must keep `false`.
struct ControlInvariants {
    masks_provenance_or_sensitivity_state: bool,
    hides_producing_run_or_lineage_state: bool,
    exposes_raw_payload_by_default: bool,
    implies_apples_to_apples_without_parity: bool,
    invents_alternate_state_label: bool,
}

/// Validates the axes shared by both component vectors.
fn validate_common_control(
    dispositions: &[M5ExperimentDisposition],
    downgrade_triggers: &[M5ExperimentDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5ExperimentAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<ArtifactLineagePanelResultSummaryCardViolation>,
) {
    if dispositions.is_empty() {
        violations.push(ArtifactLineagePanelResultSummaryCardViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(ArtifactLineagePanelResultSummaryCardViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(ArtifactLineagePanelResultSummaryCardViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5ExperimentAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(ArtifactLineagePanelResultSummaryCardViolation::AccessibilityRouteMissing);
    }
    if invariants.masks_provenance_or_sensitivity_state {
        violations.push(
            ArtifactLineagePanelResultSummaryCardViolation::ProvenanceOrSensitivityStateMasked,
        );
    }
    if invariants.hides_producing_run_or_lineage_state {
        violations
            .push(ArtifactLineagePanelResultSummaryCardViolation::ProducingRunOrLineageStateHidden);
    }
    if invariants.exposes_raw_payload_by_default {
        violations.push(ArtifactLineagePanelResultSummaryCardViolation::RawPayloadExposedByDefault);
    }
    if invariants.implies_apples_to_apples_without_parity {
        violations.push(
            ArtifactLineagePanelResultSummaryCardViolation::ApplesToApplesImpliedWithoutParity,
        );
    }
    if invariants.invents_alternate_state_label {
        violations
            .push(ArtifactLineagePanelResultSummaryCardViolation::AlternateStateLabelInvented);
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
