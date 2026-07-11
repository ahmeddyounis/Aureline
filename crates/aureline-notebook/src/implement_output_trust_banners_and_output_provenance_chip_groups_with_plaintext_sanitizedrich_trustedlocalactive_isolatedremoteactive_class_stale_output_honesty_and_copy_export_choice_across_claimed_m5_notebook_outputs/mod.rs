//! Two reusable M5 notebook components — the output trust banner and the output provenance chip
//! group — so a user can tell, before they copy, share, or act on a rich result, what an output
//! actually is and where it came from: the output trust banner names an output's trust class
//! (plain text, sanitized rich content, trusted local active content, or isolated remote active
//! content), its raw-versus-rendered representation, and whether it is live or stale after an edit,
//! a kernel restart, or a target / environment drift, and offers first-class open-raw / export /
//! copy actions so a rendered output never hides the raw or redacted representation; the output
//! provenance chip group names an output's cell / run identity, its origin class (produced by a
//! cell or run, imported, restored, external, or unknown), its attached artifacts, and its
//! persistence / retention cues, and offers inspect / view-artifacts / copy-lineage actions so an
//! output's producing run and lineage are never left implicit.
//!
//! Aureline's frozen notebook-kernel-output component matrix
//! ([`crate::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix`])
//! names the output trust banner and the output provenance chip group as two governed component
//! families and freezes their controlled vocabulary — the output trust classes (`trusted_output`,
//! `sanitized_output`, `sandboxed_output`, `raw_active_output`, `blocked_output`, `unknown_trust`)
//! and output freshness states (`live_output`, `stale_output`, `cached_output`, `cleared_output`,
//! `superseded_output`, `no_output`) a banner binds; the output provenance kinds
//! (`produced_by_cell`, `produced_by_run`, `imported_output`, `restored_output`, `external_output`,
//! `unknown_provenance`) and output provenance states (`provenance_complete`, `provenance_partial`,
//! `provenance_missing`, `execution_count_pinned`, `execution_count_drifted`, `provenance_stale`) a
//! chip group binds; the one controlled disposition vocabulary; the surface families; the
//! deployment lines; the consumer surfaces; the accessibility routes; the required labels; and the
//! downgrade triggers. This module *implements* that contract as two co-equal component vectors so
//! a claimed M5 notebook, output-viewer, AI-context, review, or support / export surface can project
//! a banner and a chip group that keep the same truth.
//!
//! The module has two derived resolvers:
//!
//! 1. [`resolve_output_trust_banner`] — takes a banner's output trust class and freshness state and
//!    derives its presentation class (plain text, sanitized rich, trusted local active, isolated
//!    remote active, blocked, or unknown), whether it carries active or isolated content, whether
//!    it is live, whether it may present as live, and which notes it must carry — so a raw or
//!    untrusted output never renders as active content, and a stale, cached, cleared, superseded, or
//!    absent output can never read as live truth.
//! 2. [`resolve_output_provenance_chip_group`] — takes a chip group's output provenance kind and
//!    provenance state and derives its origin class (cell-produced, run-produced, imported,
//!    restored, external, or unknown), its lineage resolution (fully resolved, partially resolved,
//!    unresolved, pinned, drifted, or stale), whether the origin is internal, whether it may claim a
//!    current pinned lineage, and which notes it must carry — so an imported, restored, external, or
//!    unknown output never reads as internally produced, and a partial, missing, drifted, or stale
//!    lineage never reads as a current pinned lineage.
//!
//! A single controls packet — [`OutputTrustBannerOutputProvenanceChipGroupControlsPacket`] — binds
//! one vector of output trust banners and one vector of output provenance chip groups to the same
//! trust, freshness, origin, lineage, representation, deep-link, and non-visual accessibility
//! vocabulary, so output trust truth and output provenance truth stay distinct and explicit across
//! notebook, output-viewer, AI-context, review, headless / export, and support consumers.
//!
//! The component family ([`M5NotebookKernelOutputComponentFamily`]), output trust class
//! ([`M5OutputTrustClass`]), output freshness state ([`M5OutputFreshnessState`]), output provenance
//! kind ([`M5OutputProvenanceKind`]), output provenance state ([`M5OutputProvenanceState`]),
//! disposition ([`M5NotebookKernelOutputDisposition`]), surface family
//! ([`M5NotebookKernelOutputSurfaceFamily`]), deployment line
//! ([`M5NotebookKernelOutputDeploymentLine`]), consumer surface
//! ([`M5NotebookKernelOutputConsumerSurface`]), accessibility route
//! ([`M5NotebookKernelOutputAccessibilityRoute`]), required label
//! ([`M5NotebookKernelOutputRequiredLabel`]), and downgrade trigger
//! ([`M5NotebookKernelOutputDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the two components
//! themselves: the derived presentation and origin / lineage classes, the raw-versus-rendered
//! representation mode, the bounded banner and chip actions, and the deep-link kinds. No M5
//! notebook surface invents a second output-trust or output-provenance grammar.
//!
//! Raw notebook payloads, pasted paths, credentials, and private endpoints stay outside the export
//! boundary; every context line, deep-link reference, and component identity is carried only as an
//! opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_output_trust_banner_output_provenance_chip_group_controls,
    seeded_output_trust_banner_output_provenance_chip_group_controls_output_provenance_chip_group_drifted,
    seeded_output_trust_banner_output_provenance_chip_group_controls_output_trust_banner_stale,
    OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_PACKET_ID,
};

// The output trust classes and freshness states, the output provenance kinds and states, the
// disposition vocabulary, and the surface / deployment / consumer / accessibility / label /
// downgrade vocabularies are frozen once, in the notebook-kernel-output component matrix. This lane
// reuses them verbatim so it never invents a parallel output-trust or output-provenance vocabulary.
pub use crate::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix::{
    M5NotebookKernelOutputAccessibilityRoute, M5NotebookKernelOutputComponentFamily,
    M5NotebookKernelOutputConsumerSurface, M5NotebookKernelOutputDeploymentLine,
    M5NotebookKernelOutputDisposition, M5NotebookKernelOutputDowngradeTrigger,
    M5NotebookKernelOutputRequiredLabel, M5NotebookKernelOutputSurfaceFamily, M5OutputFreshnessState,
    M5OutputProvenanceKind, M5OutputProvenanceState, M5OutputTrustClass,
    M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_DOC_REF, M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
    M5_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_REF, M5_OUTPUT_TRUST_BANNER_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`OutputTrustBannerOutputProvenanceChipGroupControlsPacket`].
pub const OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_RECORD_KIND: &str =
    "implement_output_trust_banners_and_output_provenance_chip_groups_with_plaintext_sanitizedrich_trustedlocalactive_isolatedremoteactive_class_stale_output_honesty_and_copy_export_choice_across_claimed_m5_notebook_outputs";

/// Schema version for M5 output-trust-banner / output-provenance-chip-group control records.
pub const OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_REF: &str =
    "schemas/ui/m5-output-trust-banner-output-provenance-chip-group-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_DOC_REF: &str =
    "docs/notebooks/m5_output_trust_banner_output_provenance_chip_group_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_FIXTURE_DIR: &str =
    "fixtures/ui/m5-output-trust-banner-output-provenance-chip-group-controls";

/// Repo-relative path of the checked support-export artifact.
pub const OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_ARTIFACT_REF: &str =
    "artifacts/release/m5-output-trust-banner-output-provenance-chip-group-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_CSV_REF: &str =
    "artifacts/release/m5-output-trust-banner-output-provenance-chip-group-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_REPORT_REF: &str =
    "artifacts/design/m5-output-trust-banner-output-provenance-chip-group.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link a notebook component binds its next step against, so an output
/// trust banner or output provenance chip group never routes through an ephemeral overlay — every
/// next step is a stable notebook / output location, output-viewer, docs, or support-bundle
/// reference the user can reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkKind {
    /// A stable notebook / cell / output location.
    NotebookLocation,
    /// A stable output-viewer reference.
    OutputViewer,
    /// A stable docs anchor.
    DocsAnchor,
    /// A stable support-bundle anchor.
    SupportBundle,
    /// No deep link is bound (the component names that it routes nowhere).
    NoDeepLink,
}

impl DeepLinkKind {
    /// Every deep-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotebookLocation,
        Self::OutputViewer,
        Self::DocsAnchor,
        Self::SupportBundle,
        Self::NoDeepLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookLocation => "notebook_location",
            Self::OutputViewer => "output_viewer",
            Self::DocsAnchor => "docs_anchor",
            Self::SupportBundle => "support_bundle",
            Self::NoDeepLink => "no_deep_link",
        }
    }

    /// True when this kind names a resolvable deep-link target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDeepLink)
    }
}

// ---- output-trust-banner vocabulary -------------------------------------

/// Derived output trust presentation class a banner may present.
///
/// This is the banner honesty axis: the class is derived from the frozen output trust class, never
/// asserted, so a raw or untrusted output can never read as trusted active content and a user can
/// always tell whether an output is plain text, sanitized rich content, trusted local active
/// content, or isolated remote active content before they copy, share, or act on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputTrustPresentationClass {
    /// Plain-text output — no rich rendering and no active content (a raw / untrusted output is
    /// shown as inert plain text rather than run).
    PlainText,
    /// Sanitized rich content — rendered rich output with active content stripped.
    SanitizedRich,
    /// Trusted local active content — active content from a trusted, local kernel.
    TrustedLocalActive,
    /// Isolated remote active content — active content run only inside an isolated sandbox.
    IsolatedRemoteActive,
    /// Blocked content — the output was withheld by policy.
    BlockedContent,
    /// Unknown content — the output's trust class could not be determined.
    UnknownContent,
}

impl OutputTrustPresentationClass {
    /// Every presentation class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PlainText,
        Self::SanitizedRich,
        Self::TrustedLocalActive,
        Self::IsolatedRemoteActive,
        Self::BlockedContent,
        Self::UnknownContent,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlainText => "plain_text",
            Self::SanitizedRich => "sanitized_rich",
            Self::TrustedLocalActive => "trusted_local_active",
            Self::IsolatedRemoteActive => "isolated_remote_active",
            Self::BlockedContent => "blocked_content",
            Self::UnknownContent => "unknown_content",
        }
    }

    /// True when the presentation class carries executable active content.
    pub const fn is_active_content(self) -> bool {
        matches!(self, Self::TrustedLocalActive | Self::IsolatedRemoteActive)
    }

    /// True when the active content runs only inside an isolated sandbox.
    pub const fn is_isolated_active(self) -> bool {
        matches!(self, Self::IsolatedRemoteActive)
    }

    /// True when the output carries rendered rich content (sanitized or active).
    pub const fn is_rich_content(self) -> bool {
        matches!(
            self,
            Self::SanitizedRich | Self::TrustedLocalActive | Self::IsolatedRemoteActive
        )
    }
}

/// Raw-versus-rendered representation mode a banner names, so a rendered output never hides the
/// available raw or redacted representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputRepresentationMode {
    /// The banner is rendering the rich representation (raw source stays available).
    RenderedRich,
    /// The banner is showing the raw source representation.
    RawSource,
    /// The banner is showing a redacted representation (raw stays available behind review).
    RedactedRepresentation,
}

impl OutputRepresentationMode {
    /// Every representation mode, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::RenderedRich,
        Self::RawSource,
        Self::RedactedRepresentation,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RenderedRich => "rendered_rich",
            Self::RawSource => "raw_source",
            Self::RedactedRepresentation => "redacted_representation",
        }
    }
}

/// One keyboard-complete default action an output trust banner offers, so a banner never hides its
/// open-raw / export / copy affordance behind a pointer-only gesture. `OpenRaw`, `ExportOutput`,
/// and `CopyOutput` are always offered so copy / export choice preserves the trust class and the
/// raw-versus-rendered representation instead of flattening the output into ambiguous evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputBannerAction {
    /// Open the raw representation of the output (always available).
    OpenRaw,
    /// Export the output with its trust class and representation preserved (always available).
    ExportOutput,
    /// Copy the output with its trust class and representation preserved (always available).
    CopyOutput,
    /// Clear a stale output.
    ClearStaleOutput,
    /// Rerun the producing cell to refresh a stale output.
    RerunToRefresh,
    /// Open the stable notebook / output-viewer / docs / support deep link.
    OpenDeepLink,
}

impl OutputBannerAction {
    /// Every banner action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenRaw,
        Self::ExportOutput,
        Self::CopyOutput,
        Self::ClearStaleOutput,
        Self::RerunToRefresh,
        Self::OpenDeepLink,
    ];

    /// The default actions every keyboard-complete banner must offer.
    pub const MANDATORY: [Self; 3] = [Self::OpenRaw, Self::ExportOutput, Self::CopyOutput];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenRaw => "open_raw",
            Self::ExportOutput => "export_output",
            Self::CopyOutput => "copy_output",
            Self::ClearStaleOutput => "clear_stale_output",
            Self::RerunToRefresh => "rerun_to_refresh",
            Self::OpenDeepLink => "open_deep_link",
        }
    }
}

/// Disclosures an output trust banner must carry, derived from the trust class and freshness state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputTrustBannerDisclosure {
    /// The derived presentation class this banner may present.
    pub presentation_class: OutputTrustPresentationClass,
    /// Whether the output carries active content.
    pub is_active_content: bool,
    /// Whether the active content is isolated / sandboxed.
    pub is_isolated_active: bool,
    /// Whether the output carries rendered rich content.
    pub is_rich_content: bool,
    /// Whether the output is live (fresh from its producing run).
    pub is_live: bool,
    /// Whether the banner may present the output as live.
    pub may_present_as_live: bool,
    /// Whether the banner must carry an explicit sanitized-content note.
    pub needs_sanitized_note: bool,
    /// Whether the banner must carry an explicit active-content note.
    pub needs_active_content_note: bool,
    /// Whether the banner must carry an explicit isolation / sandbox note.
    pub needs_isolation_note: bool,
    /// Whether the banner must carry an explicit blocked-content note.
    pub needs_blocked_note: bool,
    /// Whether the banner must carry an explicit unknown-trust note.
    pub needs_unknown_trust_note: bool,
    /// Whether the banner must carry an explicit stale-output note.
    pub needs_stale_note: bool,
    /// Whether the banner must carry an explicit cached-output note.
    pub needs_cached_note: bool,
    /// Whether the banner must carry an explicit cleared / no-output note.
    pub needs_cleared_note: bool,
}

/// Resolves the trust and freshness truth an output trust banner may present.
///
/// A `trusted_output` is trusted local active content, a `sanitized_output` is sanitized rich
/// content, a `sandboxed_output` is isolated remote active content, a `raw_active_output` degrades
/// to inert plain text (raw source shown literally, never run as active content), a `blocked_output`
/// is blocked content, and an `unknown_trust` output is unknown content — so a raw or untrusted
/// output never renders as trusted active content. An output may present as live only when its
/// freshness is `live_output`; a stale, cached, cleared, superseded, or absent output can never read
/// as live truth, and each non-live freshness state carries its own note.
pub fn resolve_output_trust_banner(
    trust: M5OutputTrustClass,
    freshness: M5OutputFreshnessState,
) -> OutputTrustBannerDisclosure {
    use M5OutputFreshnessState as Fresh;
    use M5OutputTrustClass as Trust;
    use OutputTrustPresentationClass as Class;

    let presentation_class = match trust {
        Trust::TrustedOutput => Class::TrustedLocalActive,
        Trust::SanitizedOutput => Class::SanitizedRich,
        Trust::SandboxedOutput => Class::IsolatedRemoteActive,
        Trust::RawActiveOutput => Class::PlainText,
        Trust::BlockedOutput => Class::BlockedContent,
        Trust::UnknownTrust => Class::UnknownContent,
    };

    let is_live = matches!(freshness, Fresh::LiveOutput);

    OutputTrustBannerDisclosure {
        presentation_class,
        is_active_content: presentation_class.is_active_content(),
        is_isolated_active: presentation_class.is_isolated_active(),
        is_rich_content: presentation_class.is_rich_content(),
        is_live,
        may_present_as_live: is_live,
        needs_sanitized_note: matches!(presentation_class, Class::SanitizedRich),
        needs_active_content_note: presentation_class.is_active_content(),
        needs_isolation_note: presentation_class.is_isolated_active(),
        needs_blocked_note: matches!(presentation_class, Class::BlockedContent),
        needs_unknown_trust_note: matches!(presentation_class, Class::UnknownContent),
        needs_stale_note: matches!(freshness, Fresh::StaleOutput | Fresh::SupersededOutput),
        needs_cached_note: matches!(freshness, Fresh::CachedOutput),
        needs_cleared_note: matches!(freshness, Fresh::ClearedOutput | Fresh::NoOutput),
    }
}

/// An output trust banner naming an output's trust class, its raw-versus-rendered representation,
/// its live-versus-stale freshness, its derived presentation class, bounded open-raw / export / copy
/// actions, and a stable deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputTrustBanner {
    /// Frozen component this control implements; must be `output_trust_banner`.
    pub component: M5NotebookKernelOutputComponentFamily,
    /// Stable banner id.
    pub banner_id: String,
    /// Human-readable banner label; required and non-empty.
    pub banner_label: String,
    /// Output trust class, reused from the frozen matrix.
    pub trust_class: M5OutputTrustClass,
    /// Output freshness state, reused from the frozen matrix.
    pub freshness_state: M5OutputFreshnessState,
    /// Raw-versus-rendered representation this banner is currently showing.
    pub representation_mode: OutputRepresentationMode,
    /// Derived presentation class (must equal the resolved class).
    pub presentation_class: OutputTrustPresentationClass,
    /// Whether the banner claims active content (must equal the derived truth).
    pub claims_active_content: bool,
    /// Whether the banner claims the output is live. May be `true` only when the derived truth
    /// allows it.
    pub claims_live: bool,
    /// Sanitized-content note; required when the output is sanitized rich content.
    pub sanitized_note: String,
    /// Active-content note; required when the output carries active content.
    pub active_content_note: String,
    /// Isolation / sandbox note; required when the output is isolated remote active content.
    pub isolation_note: String,
    /// Blocked-content note; required when the output is blocked.
    pub blocked_note: String,
    /// Unknown-trust note; required when the output's trust class is unknown.
    pub unknown_trust_note: String,
    /// Stale-output note; required when the output is stale or superseded.
    pub stale_note: String,
    /// Cached-output note; required when the output is cached.
    pub cached_note: String,
    /// Cleared / no-output note; required when the output is cleared or absent.
    pub cleared_note: String,
    /// Trust class label; always required so the trust class is never hidden behind a hover-only
    /// affordance.
    pub trust_class_label: String,
    /// Representation label; always required so the raw-versus-rendered state stays explicit.
    pub representation_label: String,
    /// Freshness label; always required so the live-versus-stale state stays explicit.
    pub freshness_label: String,
    /// Copy / export choice note; always required so copy and export preserve the trust class and
    /// the raw-versus-rendered representation instead of flattening the output into ambiguous
    /// evidence.
    pub copy_export_choice_note: String,
    /// Context note; always required so the banner names what the output truth means here.
    pub context_note: String,
    /// Kind of stable deep link this banner binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include open-raw / export / copy).
    pub banner_actions: Vec<OutputBannerAction>,
    /// Dispositions this banner binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5NotebookKernelOutputDisposition>,
    /// Downgrade triggers this banner can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Mandatory labels this banner can show (must include the mandatory labels).
    pub required_labels: Vec<M5NotebookKernelOutputRequiredLabel>,
    /// Claimed M5 surface families that render this banner.
    pub surface_families: Vec<M5NotebookKernelOutputSurfaceFamily>,
    /// Deployment lines this banner keeps the same truth across.
    pub deployment_lines: Vec<M5NotebookKernelOutputDeploymentLine>,
    /// Non-visual accessibility routes this banner offers.
    pub accessibility_routes: Vec<M5NotebookKernelOutputAccessibilityRoute>,
    /// Notebook subsystems that consume this banner's projection.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this banner.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never presents stale output as live. MUST be `false`.
    pub presents_stale_output_as_live: bool,
    /// Hard invariant: never hides the raw / sanitized / active trust class behind a hover-only
    /// affordance. MUST be `false`.
    pub hides_trust_class_behind_hover_only: bool,
    /// Hard invariant: never flattens the output into ambiguous evidence on copy / export. MUST be
    /// `false`.
    pub flattens_output_into_ambiguous_evidence: bool,
    /// Hard invariant: never severs the output's canonical provenance. MUST be `false`.
    pub severs_output_provenance: bool,
}

impl OutputTrustBanner {
    /// Trust / freshness disclosures this banner must carry, derived from the frozen states.
    pub fn trust_disclosure(&self) -> OutputTrustBannerDisclosure {
        resolve_output_trust_banner(self.trust_class, self.freshness_state)
    }

    /// Whether the banner offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<OutputBannerAction> = self.banner_actions.iter().copied().collect();
        OutputBannerAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the banner declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5NotebookKernelOutputRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5NotebookKernelOutputRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the banner offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.banner_actions
            .contains(&OutputBannerAction::OpenDeepLink)
    }
}

// ---- output-provenance-chip-group vocabulary ----------------------------

/// Derived output origin class a provenance chip group may present.
///
/// This is the chip-group origin axis: the class is derived from the frozen output provenance kind,
/// never asserted, so an imported, restored, external, or unknown output can never read as
/// internally produced and a user can always tell where an output came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputOriginClass {
    /// Produced by a cell in this notebook.
    CellProduced,
    /// Produced by a run of this notebook.
    RunProduced,
    /// Imported from another source.
    ImportedOrigin,
    /// Restored from a saved / checkpointed state.
    RestoredOrigin,
    /// Produced by an external source.
    ExternalOrigin,
    /// Origin could not be determined.
    UnknownOrigin,
}

impl OutputOriginClass {
    /// Every origin class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CellProduced,
        Self::RunProduced,
        Self::ImportedOrigin,
        Self::RestoredOrigin,
        Self::ExternalOrigin,
        Self::UnknownOrigin,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CellProduced => "cell_produced",
            Self::RunProduced => "run_produced",
            Self::ImportedOrigin => "imported_origin",
            Self::RestoredOrigin => "restored_origin",
            Self::ExternalOrigin => "external_origin",
            Self::UnknownOrigin => "unknown_origin",
        }
    }

    /// True when the output was produced internally (by a cell or run of this notebook).
    pub const fn is_internal(self) -> bool {
        matches!(self, Self::CellProduced | Self::RunProduced)
    }
}

/// Derived lineage resolution a provenance chip group may present.
///
/// This is the chip-group lineage axis: the resolution is derived from the frozen output provenance
/// state, never asserted, so a partial, missing, drifted, or stale lineage can never read as a
/// current pinned lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputProvenanceResolution {
    /// Lineage is fully resolved.
    FullyResolved,
    /// Lineage is only partially resolved.
    PartiallyResolved,
    /// Lineage is unresolved.
    Unresolved,
    /// Execution-count lineage is pinned to a run.
    LineagePinned,
    /// Execution-count lineage has drifted from the run.
    LineageDrifted,
    /// Lineage resolution is stale.
    ResolutionStale,
}

impl OutputProvenanceResolution {
    /// Every lineage resolution, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullyResolved,
        Self::PartiallyResolved,
        Self::Unresolved,
        Self::LineagePinned,
        Self::LineageDrifted,
        Self::ResolutionStale,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyResolved => "fully_resolved",
            Self::PartiallyResolved => "partially_resolved",
            Self::Unresolved => "unresolved",
            Self::LineagePinned => "lineage_pinned",
            Self::LineageDrifted => "lineage_drifted",
            Self::ResolutionStale => "resolution_stale",
        }
    }

    /// True only when the lineage is a current, resolved lineage (fully resolved or pinned).
    pub const fn is_current_lineage(self) -> bool {
        matches!(self, Self::FullyResolved | Self::LineagePinned)
    }
}

/// One keyboard-complete default action an output provenance chip group offers, so a chip group
/// never hides its inspect / view-artifacts / copy affordance behind a pointer-only gesture.
/// `InspectProvenance`, `ViewArtifacts`, and `CopyLineageIdentity` are always offered so an output's
/// cell / run identity, origin, attached artifacts, and lineage stay visible and copyable in
/// notebook, AI-context, and support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputChipAction {
    /// Inspect the output's provenance and lineage (always available).
    InspectProvenance,
    /// View the output's attached artifacts (always available).
    ViewArtifacts,
    /// Copy the stable lineage identity (always available).
    CopyLineageIdentity,
    /// View the output's persistence / retention posture.
    ViewPersistence,
    /// Export the output's provenance record.
    ExportProvenance,
    /// Open the stable notebook / output-viewer / docs / support deep link.
    OpenDeepLink,
}

impl OutputChipAction {
    /// Every chip action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InspectProvenance,
        Self::ViewArtifacts,
        Self::CopyLineageIdentity,
        Self::ViewPersistence,
        Self::ExportProvenance,
        Self::OpenDeepLink,
    ];

    /// The default actions every keyboard-complete chip group must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::InspectProvenance,
        Self::ViewArtifacts,
        Self::CopyLineageIdentity,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectProvenance => "inspect_provenance",
            Self::ViewArtifacts => "view_artifacts",
            Self::CopyLineageIdentity => "copy_lineage_identity",
            Self::ViewPersistence => "view_persistence",
            Self::ExportProvenance => "export_provenance",
            Self::OpenDeepLink => "open_deep_link",
        }
    }
}

/// Disclosures an output provenance chip group must carry, derived from the provenance kind and
/// provenance state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputProvenanceChipGroupDisclosure {
    /// The derived origin class this chip group may present.
    pub origin_class: OutputOriginClass,
    /// The derived lineage resolution this chip group may present.
    pub resolution_class: OutputProvenanceResolution,
    /// Whether the output was produced internally.
    pub is_internal_origin: bool,
    /// Whether the chip group may claim a current, pinned lineage.
    pub may_claim_current_lineage: bool,
    /// Whether the chip group must carry an explicit external-origin note.
    pub needs_external_note: bool,
    /// Whether the chip group must carry an explicit partial-lineage note.
    pub needs_partial_note: bool,
    /// Whether the chip group must carry an explicit missing-lineage note.
    pub needs_missing_note: bool,
    /// Whether the chip group must carry an explicit execution-count-drift note.
    pub needs_drift_note: bool,
    /// Whether the chip group must carry an explicit stale-lineage note.
    pub needs_stale_note: bool,
}

/// Resolves the origin and lineage truth an output provenance chip group may present.
///
/// A `produced_by_cell` output is cell-produced and a `produced_by_run` output is run-produced (both
/// internal); an `imported_output`, `restored_output`, `external_output`, or `unknown_provenance`
/// output is imported, restored, external, or unknown — so an output Aureline did not produce here
/// can never read as internally produced. A current, pinned lineage may be claimed only when the
/// provenance state is `provenance_complete` or `execution_count_pinned`; a partial, missing,
/// drifted, or stale lineage can never read as a current pinned lineage and carries its own note.
pub fn resolve_output_provenance_chip_group(
    kind: M5OutputProvenanceKind,
    state: M5OutputProvenanceState,
) -> OutputProvenanceChipGroupDisclosure {
    use M5OutputProvenanceKind as Kind;
    use M5OutputProvenanceState as State;
    use OutputOriginClass as Origin;
    use OutputProvenanceResolution as Resolution;

    let origin_class = match kind {
        Kind::ProducedByCell => Origin::CellProduced,
        Kind::ProducedByRun => Origin::RunProduced,
        Kind::ImportedOutput => Origin::ImportedOrigin,
        Kind::RestoredOutput => Origin::RestoredOrigin,
        Kind::ExternalOutput => Origin::ExternalOrigin,
        Kind::UnknownProvenance => Origin::UnknownOrigin,
    };

    let resolution_class = match state {
        State::ProvenanceComplete => Resolution::FullyResolved,
        State::ProvenancePartial => Resolution::PartiallyResolved,
        State::ProvenanceMissing => Resolution::Unresolved,
        State::ExecutionCountPinned => Resolution::LineagePinned,
        State::ExecutionCountDrifted => Resolution::LineageDrifted,
        State::ProvenanceStale => Resolution::ResolutionStale,
    };

    OutputProvenanceChipGroupDisclosure {
        origin_class,
        resolution_class,
        is_internal_origin: origin_class.is_internal(),
        may_claim_current_lineage: resolution_class.is_current_lineage(),
        needs_external_note: !origin_class.is_internal(),
        needs_partial_note: matches!(resolution_class, Resolution::PartiallyResolved),
        needs_missing_note: matches!(resolution_class, Resolution::Unresolved),
        needs_drift_note: matches!(resolution_class, Resolution::LineageDrifted),
        needs_stale_note: matches!(resolution_class, Resolution::ResolutionStale),
    }
}

/// An output provenance chip group naming an output's cell / run identity, its origin class, its
/// attached artifacts, its persistence / retention cues, its derived lineage resolution, bounded
/// inspect / view-artifacts / copy actions, and a stable deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputProvenanceChipGroup {
    /// Frozen component this control implements; must be `output_provenance_chip_group`.
    pub component: M5NotebookKernelOutputComponentFamily,
    /// Stable chip-group id.
    pub group_id: String,
    /// Human-readable chip-group label; required and non-empty.
    pub group_label: String,
    /// Output provenance kind, reused from the frozen matrix.
    pub provenance_kind: M5OutputProvenanceKind,
    /// Output provenance state, reused from the frozen matrix.
    pub provenance_state: M5OutputProvenanceState,
    /// Derived origin class (must equal the resolved class).
    pub origin_class: OutputOriginClass,
    /// Derived lineage resolution (must equal the resolved resolution).
    pub resolution_class: OutputProvenanceResolution,
    /// Whether the chip group claims the output is internally produced (must equal the derived
    /// truth).
    pub claims_internal_origin: bool,
    /// Whether the chip group claims a current, pinned lineage. May be `true` only when the derived
    /// truth allows it.
    pub claims_current_lineage: bool,
    /// External-origin note; required when the origin is imported / restored / external / unknown.
    pub external_note: String,
    /// Partial-lineage note; required when the lineage is only partially resolved.
    pub partial_note: String,
    /// Missing-lineage note; required when the lineage is unresolved.
    pub missing_note: String,
    /// Execution-count-drift note; required when the execution-count lineage drifted.
    pub drift_note: String,
    /// Stale-lineage note; required when the lineage resolution is stale.
    pub stale_note: String,
    /// Cell / run identity label; always required so an output's producing cell and run stay
    /// explicit.
    pub cell_run_identity_label: String,
    /// Origin class label; always required so where the output came from stays explicit.
    pub origin_class_label: String,
    /// Attached-artifacts label; always required so an output's attached artifacts are never left
    /// implicit.
    pub attached_artifacts_label: String,
    /// Persistence / retention note; always required so persistence and retention cues stay
    /// explicit.
    pub persistence_retention_note: String,
    /// Context note; always required so the chip group names what the provenance truth means here.
    pub context_note: String,
    /// Kind of stable deep link this chip group binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include inspect / view-artifacts / copy).
    pub chip_actions: Vec<OutputChipAction>,
    /// Dispositions this chip group binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5NotebookKernelOutputDisposition>,
    /// Downgrade triggers this chip group can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Mandatory labels this chip group can show (must include the mandatory labels).
    pub required_labels: Vec<M5NotebookKernelOutputRequiredLabel>,
    /// Claimed M5 surface families that render this chip group.
    pub surface_families: Vec<M5NotebookKernelOutputSurfaceFamily>,
    /// Deployment lines this chip group keeps the same truth across.
    pub deployment_lines: Vec<M5NotebookKernelOutputDeploymentLine>,
    /// Non-visual accessibility routes this chip group offers.
    pub accessibility_routes: Vec<M5NotebookKernelOutputAccessibilityRoute>,
    /// Notebook subsystems that consume this chip group's projection.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this chip group.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never presents stale output as live. MUST be `false`.
    pub presents_stale_output_as_live: bool,
    /// Hard invariant: never hides the raw / sanitized / active trust class behind a hover-only
    /// affordance. MUST be `false`.
    pub hides_trust_class_behind_hover_only: bool,
    /// Hard invariant: never flattens the output into ambiguous evidence on copy / export. MUST be
    /// `false`.
    pub flattens_output_into_ambiguous_evidence: bool,
    /// Hard invariant: never severs the output's canonical provenance. MUST be `false`.
    pub severs_output_provenance: bool,
}

impl OutputProvenanceChipGroup {
    /// Origin / lineage disclosures this chip group must carry, derived from the frozen states.
    pub fn provenance_disclosure(&self) -> OutputProvenanceChipGroupDisclosure {
        resolve_output_provenance_chip_group(self.provenance_kind, self.provenance_state)
    }

    /// Whether the chip group offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<OutputChipAction> = self.chip_actions.iter().copied().collect();
        OutputChipAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the chip group declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5NotebookKernelOutputRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5NotebookKernelOutputRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the chip group offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.chip_actions.contains(&OutputChipAction::OpenDeepLink)
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance output-trust / output-provenance review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputTrustProvenanceReview {
    /// The banner names each output's trust class.
    pub banner_shows_trust_class: bool,
    /// The banner names each output's raw-versus-rendered representation.
    pub banner_shows_raw_vs_rendered: bool,
    /// The banner names each output's live-versus-stale state.
    pub banner_shows_stale_state: bool,
    /// The banner offers open-raw and export actions.
    pub banner_offers_open_raw_and_export: bool,
    /// The chip group names each output's cell / run identity.
    pub chip_shows_cell_run_identity: bool,
    /// The chip group names each output's origin class.
    pub chip_shows_origin_class: bool,
    /// The chip group names each output's attached artifacts.
    pub chip_shows_attached_artifacts: bool,
    /// The chip group names each output's persistence / retention cues.
    pub chip_shows_persistence_or_retention: bool,
    /// Trust class, presentation, origin, and lineage are derived from state, never asserted.
    pub trust_and_provenance_derived_never_asserted: bool,
    /// Stale output is never presented as live truth.
    pub stale_output_never_presented_as_live: bool,
    /// The trust class is never hidden behind a hover-only affordance.
    pub trust_class_never_hover_only: bool,
    /// Copy / export preserves the trust class and the raw-versus-rendered representation.
    pub copy_export_preserves_trust_and_representation: bool,
    /// An output's canonical provenance is never severed.
    pub output_provenance_never_severed: bool,
    /// Output trust stays visible in notebook, AI-context, and support exports.
    pub output_trust_visible_in_notebook_ai_support: bool,
    /// Every next step names one stable notebook / output-viewer / docs / support deep link.
    pub every_next_step_names_stable_deep_link: bool,
    /// Banners and chip groups stay consistent across edit, viewer, AI, review, and support
    /// surfaces.
    pub banner_and_chip_consistent_across_surfaces: bool,
    /// No component widens export scope or exposes raw payloads by default.
    pub no_component_widens_export_scope_or_exposes_raw_by_default: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl OutputTrustProvenanceReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.banner_shows_trust_class
            && self.banner_shows_raw_vs_rendered
            && self.banner_shows_stale_state
            && self.banner_offers_open_raw_and_export
            && self.chip_shows_cell_run_identity
            && self.chip_shows_origin_class
            && self.chip_shows_attached_artifacts
            && self.chip_shows_persistence_or_retention
            && self.trust_and_provenance_derived_never_asserted
            && self.stale_output_never_presented_as_live
            && self.trust_class_never_hover_only
            && self.copy_export_preserves_trust_and_representation
            && self.output_provenance_never_severed
            && self.output_trust_visible_in_notebook_ai_support
            && self.every_next_step_names_stable_deep_link
            && self.banner_and_chip_consistent_across_surfaces
            && self.no_component_widens_export_scope_or_exposes_raw_by_default
            && self.components_stable_across_deployment_lines
            && self.no_surface_invents_alternate_state_label
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputTrustProvenanceConsumerProjection {
    /// The output-viewer surface reads a single canonical source.
    pub output_viewer_reads_single_source: bool,
    /// The notebook output surface shows the trust class.
    pub notebook_output_shows_trust_class: bool,
    /// The AI-context surface shows the output provenance.
    pub ai_context_shows_output_provenance: bool,
    /// The support export shows the trust class and provenance.
    pub support_export_shows_trust_and_provenance: bool,
    /// Copy / export preserves the representation choice.
    pub copy_export_preserves_representation: bool,
    /// Help / docs shows component truth.
    pub help_docs_shows_component_truth: bool,
}

impl OutputTrustProvenanceConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.output_viewer_reads_single_source
            && self.notebook_output_shows_trust_class
            && self.ai_context_shows_output_provenance
            && self.support_export_shows_trust_and_provenance
            && self.copy_export_preserves_representation
            && self.help_docs_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputTrustProvenanceProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`OutputTrustBannerOutputProvenanceChipGroupControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputTrustBannerOutputProvenanceChipGroupControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Output trust banners.
    pub trust_banners: Vec<OutputTrustBanner>,
    /// Output provenance chip groups.
    pub provenance_chip_groups: Vec<OutputProvenanceChipGroup>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Output review block.
    pub output_review: OutputTrustProvenanceReview,
    /// Consumer projection block.
    pub consumer_projection: OutputTrustProvenanceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: OutputTrustProvenanceProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe output-trust-banner / output-provenance-chip-group controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputTrustBannerOutputProvenanceChipGroupControlsPacket {
    /// Record kind; must equal
    /// [`OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Output trust banners.
    pub trust_banners: Vec<OutputTrustBanner>,
    /// Output provenance chip groups.
    pub provenance_chip_groups: Vec<OutputProvenanceChipGroup>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5NotebookKernelOutputDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Output review block.
    pub output_review: OutputTrustProvenanceReview,
    /// Consumer projection block.
    pub consumer_projection: OutputTrustProvenanceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: OutputTrustProvenanceProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl OutputTrustBannerOutputProvenanceChipGroupControlsPacket {
    /// Builds an output-trust-banner / output-provenance-chip-group controls packet from stable-lane
    /// input.
    pub fn new(input: OutputTrustBannerOutputProvenanceChipGroupControlsPacketInput) -> Self {
        Self {
            record_kind: OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_RECORD_KIND.to_owned(),
            schema_version: OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            trust_banners: input.trust_banners,
            provenance_chip_groups: input.provenance_chip_groups,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            output_review: input.output_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the output-trust-banner / output-provenance-chip-group control invariants.
    pub fn validate(&self) -> Vec<OutputTrustBannerOutputProvenanceChipGroupViolation> {
        let mut violations = Vec::new();

        if self.record_kind != OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_RECORD_KIND {
            violations.push(OutputTrustBannerOutputProvenanceChipGroupViolation::WrongRecordKind);
        }
        if self.schema_version != OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_VERSION {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(OutputTrustBannerOutputProvenanceChipGroupViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::DowngradeTriggersMissing,
            );
        }
        if self.consumer_surfaces.is_empty() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_trust_banners(self, &mut violations);
        validate_provenance_chip_groups(self, &mut violations);

        if !self.output_review.all_hold() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::OutputReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::ConsumerProjectionIncomplete,
            );
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::ProofFreshnessIncomplete,
            );
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("output trust banner output provenance chip group packet serializes"),
        ) {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("output trust banner output provenance chip group packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component,id,class_or_kind,freshness_or_state,derived,live_or_current,deep_link_kind\n",
        );
        for banner in &self.trust_banners {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "output_trust_banner",
                csv_field(&banner.banner_id),
                banner.trust_class.as_str(),
                banner.freshness_state.as_str(),
                banner.trust_disclosure().presentation_class.as_str(),
                banner.trust_disclosure().is_live,
                banner.deep_link_kind.as_str(),
            ));
        }
        for group in &self.provenance_chip_groups {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "output_provenance_chip_group",
                csv_field(&group.group_id),
                group.provenance_kind.as_str(),
                group.provenance_state.as_str(),
                group.provenance_disclosure().origin_class.as_str(),
                group.provenance_disclosure().may_claim_current_lineage,
                group.deep_link_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let not_live = self
            .trust_banners
            .iter()
            .filter(|banner| !banner.trust_disclosure().is_live)
            .count();
        let not_current = self
            .provenance_chip_groups
            .iter()
            .filter(|group| !group.provenance_disclosure().may_claim_current_lineage)
            .count();

        let mut out = String::new();
        out.push_str("# Output trust banners and output provenance chip groups\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Output trust banners: {} ({} not live)\n",
            self.trust_banners.len(),
            not_live
        ));
        out.push_str(&format!(
            "- Output provenance chip groups: {} ({} not a current pinned lineage)\n",
            self.provenance_chip_groups.len(),
            not_current
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Output trust banners\n\n");
        for banner in &self.trust_banners {
            out.push_str(&format!(
                "- **{}** — trust `{}`, freshness `{}` → `{}`, representation `{}`, deep link `{}`\n",
                banner.banner_label,
                banner.trust_class.as_str(),
                banner.freshness_state.as_str(),
                banner.trust_disclosure().presentation_class.as_str(),
                banner.representation_mode.as_str(),
                banner.deep_link_kind.as_str(),
            ));
        }

        out.push_str("\n## Output provenance chip groups\n\n");
        for group in &self.provenance_chip_groups {
            out.push_str(&format!(
                "- **{}** — kind `{}`, state `{}` → `{}` / `{}`, deep link `{}`\n",
                group.group_label,
                group.provenance_kind.as_str(),
                group.provenance_state.as_str(),
                group.provenance_disclosure().origin_class.as_str(),
                group.provenance_disclosure().resolution_class.as_str(),
                group.deep_link_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in output-trust-banner / output-provenance-chip-group
/// export.
#[derive(Debug)]
pub enum OutputTrustBannerOutputProvenanceChipGroupArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<OutputTrustBannerOutputProvenanceChipGroupViolation>),
}

impl fmt::Display for OutputTrustBannerOutputProvenanceChipGroupArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "output trust banner output provenance chip group export parse failed: {error}"
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
                    "output trust banner output provenance chip group export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for OutputTrustBannerOutputProvenanceChipGroupArtifactError {}

/// Validation failures emitted by
/// [`OutputTrustBannerOutputProvenanceChipGroupControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputTrustBannerOutputProvenanceChipGroupViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No output trust banners are present.
    TrustBannersMissing,
    /// An output trust banner is incomplete.
    TrustBannerIncomplete,
    /// An output trust banner carries the wrong frozen component class.
    TrustBannerWrongComponentClass,
    /// A banner misrepresents its derived presentation class.
    PresentationClassMisrepresented,
    /// A banner presents a non-live output as live.
    StaleOutputClaimedLive,
    /// A sanitized banner does not name its sanitized content.
    SanitizedNoteMissing,
    /// An active-content banner does not name its active content.
    ActiveContentNoteMissing,
    /// An isolated banner does not name its isolation.
    IsolationNoteMissing,
    /// A blocked banner does not name its blocked content.
    BlockedNoteMissing,
    /// An unknown-trust banner does not name its unknown trust class.
    UnknownTrustNoteMissing,
    /// A stale / superseded banner does not name its staleness.
    StaleNoteMissing,
    /// A cached banner does not name its cache.
    CachedNoteMissing,
    /// A cleared / no-output banner does not name its cleared state.
    ClearedNoteMissing,
    /// A banner does not name its trust class.
    TrustClassLabelMissing,
    /// A banner does not name its raw-versus-rendered representation.
    RepresentationLabelMissing,
    /// A banner does not name its live-versus-stale freshness.
    FreshnessLabelMissing,
    /// A banner does not name its copy / export choice.
    CopyExportChoiceNoteMissing,
    /// A banner omits a mandatory open-raw / export / copy action.
    BannerActionsIncomplete,
    /// The banners do not cover every output trust class.
    OutputTrustClassCoverageMissing,
    /// The banners do not cover every output freshness state.
    OutputFreshnessStateCoverageMissing,
    /// The banners do not cover every derived presentation class.
    OutputPresentationClassCoverageMissing,
    /// No output provenance chip groups are present.
    ChipGroupsMissing,
    /// An output provenance chip group is incomplete.
    ChipGroupIncomplete,
    /// An output provenance chip group carries the wrong frozen component class.
    ChipGroupWrongComponentClass,
    /// A chip group misrepresents its derived origin or lineage class.
    ProvenanceMisrepresented,
    /// A chip group claims a current lineage when the lineage does not allow it.
    CurrentLineageOverclaimed,
    /// An external-origin chip group does not name its external origin.
    ExternalNoteMissing,
    /// A partial-lineage chip group does not name its partial lineage.
    PartialNoteMissing,
    /// A missing-lineage chip group does not name its missing lineage.
    MissingNoteMissing,
    /// A drifted chip group does not name its execution-count drift.
    DriftNoteMissing,
    /// A stale-lineage chip group does not name its staleness.
    StaleLineageNoteMissing,
    /// A chip group does not name its cell / run identity.
    CellRunIdentityLabelMissing,
    /// A chip group does not name its origin class.
    OriginClassLabelMissing,
    /// A chip group does not name its attached artifacts.
    AttachedArtifactsLabelMissing,
    /// A chip group does not name its persistence / retention cues.
    PersistenceRetentionNoteMissing,
    /// A chip group omits a mandatory inspect / view-artifacts / copy action.
    ChipActionsIncomplete,
    /// The chip groups do not cover every output provenance kind.
    OutputProvenanceKindCoverageMissing,
    /// The chip groups do not cover every output provenance state.
    OutputProvenanceStateCoverageMissing,
    /// The chip groups do not cover every derived origin class.
    OutputOriginClassCoverageMissing,
    /// The chip groups do not cover every derived lineage resolution.
    OutputProvenanceResolutionCoverageMissing,
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
    /// A component presents stale output as live.
    StaleShownAsLive,
    /// A component hides the trust class behind a hover-only affordance.
    TrustClassHoverOnly,
    /// A component flattens the output into ambiguous evidence on copy / export.
    OutputFlattenedIntoAmbiguousEvidence,
    /// A component severs the output's canonical provenance.
    ProvenanceSevered,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Output review does not satisfy required invariants.
    OutputReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl OutputTrustBannerOutputProvenanceChipGroupViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::TrustBannersMissing => "trust_banners_missing",
            Self::TrustBannerIncomplete => "trust_banner_incomplete",
            Self::TrustBannerWrongComponentClass => "trust_banner_wrong_component_class",
            Self::PresentationClassMisrepresented => "presentation_class_misrepresented",
            Self::StaleOutputClaimedLive => "stale_output_claimed_live",
            Self::SanitizedNoteMissing => "sanitized_note_missing",
            Self::ActiveContentNoteMissing => "active_content_note_missing",
            Self::IsolationNoteMissing => "isolation_note_missing",
            Self::BlockedNoteMissing => "blocked_note_missing",
            Self::UnknownTrustNoteMissing => "unknown_trust_note_missing",
            Self::StaleNoteMissing => "stale_note_missing",
            Self::CachedNoteMissing => "cached_note_missing",
            Self::ClearedNoteMissing => "cleared_note_missing",
            Self::TrustClassLabelMissing => "trust_class_label_missing",
            Self::RepresentationLabelMissing => "representation_label_missing",
            Self::FreshnessLabelMissing => "freshness_label_missing",
            Self::CopyExportChoiceNoteMissing => "copy_export_choice_note_missing",
            Self::BannerActionsIncomplete => "banner_actions_incomplete",
            Self::OutputTrustClassCoverageMissing => "output_trust_class_coverage_missing",
            Self::OutputFreshnessStateCoverageMissing => "output_freshness_state_coverage_missing",
            Self::OutputPresentationClassCoverageMissing => {
                "output_presentation_class_coverage_missing"
            }
            Self::ChipGroupsMissing => "chip_groups_missing",
            Self::ChipGroupIncomplete => "chip_group_incomplete",
            Self::ChipGroupWrongComponentClass => "chip_group_wrong_component_class",
            Self::ProvenanceMisrepresented => "provenance_misrepresented",
            Self::CurrentLineageOverclaimed => "current_lineage_overclaimed",
            Self::ExternalNoteMissing => "external_note_missing",
            Self::PartialNoteMissing => "partial_note_missing",
            Self::MissingNoteMissing => "missing_note_missing",
            Self::DriftNoteMissing => "drift_note_missing",
            Self::StaleLineageNoteMissing => "stale_lineage_note_missing",
            Self::CellRunIdentityLabelMissing => "cell_run_identity_label_missing",
            Self::OriginClassLabelMissing => "origin_class_label_missing",
            Self::AttachedArtifactsLabelMissing => "attached_artifacts_label_missing",
            Self::PersistenceRetentionNoteMissing => "persistence_retention_note_missing",
            Self::ChipActionsIncomplete => "chip_actions_incomplete",
            Self::OutputProvenanceKindCoverageMissing => "output_provenance_kind_coverage_missing",
            Self::OutputProvenanceStateCoverageMissing => {
                "output_provenance_state_coverage_missing"
            }
            Self::OutputOriginClassCoverageMissing => "output_origin_class_coverage_missing",
            Self::OutputProvenanceResolutionCoverageMissing => {
                "output_provenance_resolution_coverage_missing"
            }
            Self::ContextNoteMissing => "context_note_missing",
            Self::DeepLinkUnresolved => "deep_link_unresolved",
            Self::DeepLinkRefMissing => "deep_link_ref_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::StaleShownAsLive => "stale_shown_as_live",
            Self::TrustClassHoverOnly => "trust_class_hover_only",
            Self::OutputFlattenedIntoAmbiguousEvidence => {
                "output_flattened_into_ambiguous_evidence"
            }
            Self::ProvenanceSevered => "provenance_severed",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::OutputReviewIncomplete => "output_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable output-trust-banner / output-provenance-chip-group
/// export.
pub fn current_output_trust_banner_output_provenance_chip_group_export() -> Result<
    OutputTrustBannerOutputProvenanceChipGroupControlsPacket,
    OutputTrustBannerOutputProvenanceChipGroupArtifactError,
> {
    let packet: OutputTrustBannerOutputProvenanceChipGroupControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-output-trust-banner-output-provenance-chip-group-proof/support_export.json"
        )))
        .map_err(OutputTrustBannerOutputProvenanceChipGroupArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(OutputTrustBannerOutputProvenanceChipGroupArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &OutputTrustBannerOutputProvenanceChipGroupControlsPacket,
    violations: &mut Vec<OutputTrustBannerOutputProvenanceChipGroupViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_REF,
        OUTPUT_TRUST_BANNER_OUTPUT_PROVENANCE_CHIP_GROUP_DOC_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
        M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_DOC_REF,
        M5_OUTPUT_TRUST_BANNER_SCHEMA_REF,
        M5_OUTPUT_PROVENANCE_CHIP_GROUP_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_trust_banners(
    packet: &OutputTrustBannerOutputProvenanceChipGroupControlsPacket,
    violations: &mut Vec<OutputTrustBannerOutputProvenanceChipGroupViolation>,
) {
    if packet.trust_banners.is_empty() {
        violations.push(OutputTrustBannerOutputProvenanceChipGroupViolation::TrustBannersMissing);
        return;
    }

    let mut presentation_classes: BTreeSet<OutputTrustPresentationClass> = BTreeSet::new();
    let mut trust_classes: BTreeSet<M5OutputTrustClass> = BTreeSet::new();
    let mut freshness_states: BTreeSet<M5OutputFreshnessState> = BTreeSet::new();

    for banner in &packet.trust_banners {
        let disclosure = banner.trust_disclosure();
        presentation_classes.insert(disclosure.presentation_class);
        trust_classes.insert(banner.trust_class);
        freshness_states.insert(banner.freshness_state);

        if banner.banner_id.trim().is_empty()
            || banner.banner_label.trim().is_empty()
            || banner.fields_shown.is_empty()
            || banner.surface_families.is_empty()
            || banner.deployment_lines.is_empty()
            || banner.consumer_surfaces.is_empty()
            || banner.source_contract_refs.is_empty()
        {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::TrustBannerIncomplete);
        }
        if banner.component != M5NotebookKernelOutputComponentFamily::OutputTrustBanner {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::TrustBannerWrongComponentClass,
            );
        }
        if banner.presentation_class != disclosure.presentation_class
            || banner.claims_active_content != disclosure.is_active_content
        {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::PresentationClassMisrepresented,
            );
        }
        if banner.claims_live && !disclosure.may_present_as_live {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::StaleOutputClaimedLive);
        }
        if disclosure.needs_sanitized_note && banner.sanitized_note.trim().is_empty() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::SanitizedNoteMissing);
        }
        if disclosure.needs_active_content_note && banner.active_content_note.trim().is_empty() {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::ActiveContentNoteMissing,
            );
        }
        if disclosure.needs_isolation_note && banner.isolation_note.trim().is_empty() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::IsolationNoteMissing);
        }
        if disclosure.needs_blocked_note && banner.blocked_note.trim().is_empty() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::BlockedNoteMissing);
        }
        if disclosure.needs_unknown_trust_note && banner.unknown_trust_note.trim().is_empty() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::UnknownTrustNoteMissing);
        }
        if disclosure.needs_stale_note && banner.stale_note.trim().is_empty() {
            violations.push(OutputTrustBannerOutputProvenanceChipGroupViolation::StaleNoteMissing);
        }
        if disclosure.needs_cached_note && banner.cached_note.trim().is_empty() {
            violations.push(OutputTrustBannerOutputProvenanceChipGroupViolation::CachedNoteMissing);
        }
        if disclosure.needs_cleared_note && banner.cleared_note.trim().is_empty() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::ClearedNoteMissing);
        }
        if banner.trust_class_label.trim().is_empty() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::TrustClassLabelMissing);
        }
        if banner.representation_label.trim().is_empty() {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::RepresentationLabelMissing,
            );
        }
        if banner.freshness_label.trim().is_empty() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::FreshnessLabelMissing);
        }
        if banner.copy_export_choice_note.trim().is_empty() {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::CopyExportChoiceNoteMissing,
            );
        }
        if !banner.declares_mandatory_actions() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::BannerActionsIncomplete);
        }
        validate_deep_link(
            banner.offers_deep_link_action(),
            banner.deep_link_kind,
            &banner.deep_link_ref,
            &banner.context_note,
            violations,
        );
        validate_common_control(
            &banner.dispositions,
            &banner.downgrade_triggers,
            banner.declares_mandatory_labels(),
            &banner.accessibility_routes,
            ControlInvariants {
                presents_stale_output_as_live: banner.presents_stale_output_as_live,
                hides_trust_class_behind_hover_only: banner.hides_trust_class_behind_hover_only,
                flattens_output_into_ambiguous_evidence: banner
                    .flattens_output_into_ambiguous_evidence,
                severs_output_provenance: banner.severs_output_provenance,
            },
            violations,
        );
    }

    for required in OutputTrustPresentationClass::ALL {
        if !presentation_classes.contains(&required) {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::OutputPresentationClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5OutputTrustClass::ALL {
        if !trust_classes.contains(&required) {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::OutputTrustClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5OutputFreshnessState::ALL {
        if !freshness_states.contains(&required) {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::OutputFreshnessStateCoverageMissing,
            );
            break;
        }
    }
}

fn validate_provenance_chip_groups(
    packet: &OutputTrustBannerOutputProvenanceChipGroupControlsPacket,
    violations: &mut Vec<OutputTrustBannerOutputProvenanceChipGroupViolation>,
) {
    if packet.provenance_chip_groups.is_empty() {
        violations.push(OutputTrustBannerOutputProvenanceChipGroupViolation::ChipGroupsMissing);
        return;
    }

    let mut origin_classes: BTreeSet<OutputOriginClass> = BTreeSet::new();
    let mut resolutions: BTreeSet<OutputProvenanceResolution> = BTreeSet::new();
    let mut kinds: BTreeSet<M5OutputProvenanceKind> = BTreeSet::new();
    let mut states: BTreeSet<M5OutputProvenanceState> = BTreeSet::new();

    for group in &packet.provenance_chip_groups {
        let disclosure = group.provenance_disclosure();
        origin_classes.insert(disclosure.origin_class);
        resolutions.insert(disclosure.resolution_class);
        kinds.insert(group.provenance_kind);
        states.insert(group.provenance_state);

        if group.group_id.trim().is_empty()
            || group.group_label.trim().is_empty()
            || group.fields_shown.is_empty()
            || group.surface_families.is_empty()
            || group.deployment_lines.is_empty()
            || group.consumer_surfaces.is_empty()
            || group.source_contract_refs.is_empty()
        {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::ChipGroupIncomplete);
        }
        if group.component != M5NotebookKernelOutputComponentFamily::OutputProvenanceChipGroup {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::ChipGroupWrongComponentClass,
            );
        }
        if group.origin_class != disclosure.origin_class
            || group.resolution_class != disclosure.resolution_class
            || group.claims_internal_origin != disclosure.is_internal_origin
        {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::ProvenanceMisrepresented,
            );
        }
        if group.claims_current_lineage && !disclosure.may_claim_current_lineage {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::CurrentLineageOverclaimed,
            );
        }
        if disclosure.needs_external_note && group.external_note.trim().is_empty() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::ExternalNoteMissing);
        }
        if disclosure.needs_partial_note && group.partial_note.trim().is_empty() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::PartialNoteMissing);
        }
        if disclosure.needs_missing_note && group.missing_note.trim().is_empty() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::MissingNoteMissing);
        }
        if disclosure.needs_drift_note && group.drift_note.trim().is_empty() {
            violations.push(OutputTrustBannerOutputProvenanceChipGroupViolation::DriftNoteMissing);
        }
        if disclosure.needs_stale_note && group.stale_note.trim().is_empty() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::StaleLineageNoteMissing);
        }
        if group.cell_run_identity_label.trim().is_empty() {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::CellRunIdentityLabelMissing,
            );
        }
        if group.origin_class_label.trim().is_empty() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::OriginClassLabelMissing);
        }
        if group.attached_artifacts_label.trim().is_empty() {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::AttachedArtifactsLabelMissing,
            );
        }
        if group.persistence_retention_note.trim().is_empty() {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::PersistenceRetentionNoteMissing,
            );
        }
        if !group.declares_mandatory_actions() {
            violations
                .push(OutputTrustBannerOutputProvenanceChipGroupViolation::ChipActionsIncomplete);
        }
        validate_deep_link(
            group.offers_deep_link_action(),
            group.deep_link_kind,
            &group.deep_link_ref,
            &group.context_note,
            violations,
        );
        validate_common_control(
            &group.dispositions,
            &group.downgrade_triggers,
            group.declares_mandatory_labels(),
            &group.accessibility_routes,
            ControlInvariants {
                presents_stale_output_as_live: group.presents_stale_output_as_live,
                hides_trust_class_behind_hover_only: group.hides_trust_class_behind_hover_only,
                flattens_output_into_ambiguous_evidence: group
                    .flattens_output_into_ambiguous_evidence,
                severs_output_provenance: group.severs_output_provenance,
            },
            violations,
        );
    }

    for required in OutputOriginClass::ALL {
        if !origin_classes.contains(&required) {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::OutputOriginClassCoverageMissing,
            );
            break;
        }
    }
    for required in OutputProvenanceResolution::ALL {
        if !resolutions.contains(&required) {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::OutputProvenanceResolutionCoverageMissing,
            );
            break;
        }
    }
    for required in M5OutputProvenanceKind::ALL {
        if !kinds.contains(&required) {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::OutputProvenanceKindCoverageMissing,
            );
            break;
        }
    }
    for required in M5OutputProvenanceState::ALL {
        if !states.contains(&required) {
            violations.push(
                OutputTrustBannerOutputProvenanceChipGroupViolation::OutputProvenanceStateCoverageMissing,
            );
            break;
        }
    }
}

/// Validates the context and stable deep-link truth shared by both component vectors.
///
/// A component that offers a deep-link action must name a resolvable deep-link kind, a component
/// that names a resolvable kind must carry its stable reference, and every component must name its
/// context — so a next step is never an ephemeral overlay or hidden route.
fn validate_deep_link(
    offers_deep_link_action: bool,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    context_note: &str,
    violations: &mut Vec<OutputTrustBannerOutputProvenanceChipGroupViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(OutputTrustBannerOutputProvenanceChipGroupViolation::ContextNoteMissing);
    }
    if offers_deep_link_action && !deep_link_kind.is_resolvable() {
        violations.push(OutputTrustBannerOutputProvenanceChipGroupViolation::DeepLinkUnresolved);
    }
    if deep_link_kind.is_resolvable() && deep_link_ref.trim().is_empty() {
        violations.push(OutputTrustBannerOutputProvenanceChipGroupViolation::DeepLinkRefMissing);
    }
}

/// The four hard-invariant bools every component must keep `false`.
struct ControlInvariants {
    presents_stale_output_as_live: bool,
    hides_trust_class_behind_hover_only: bool,
    flattens_output_into_ambiguous_evidence: bool,
    severs_output_provenance: bool,
}

/// Validates the axes shared by both component vectors.
fn validate_common_control(
    dispositions: &[M5NotebookKernelOutputDisposition],
    downgrade_triggers: &[M5NotebookKernelOutputDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5NotebookKernelOutputAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<OutputTrustBannerOutputProvenanceChipGroupViolation>,
) {
    if dispositions.is_empty() {
        violations.push(OutputTrustBannerOutputProvenanceChipGroupViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations
            .push(OutputTrustBannerOutputProvenanceChipGroupViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations
            .push(OutputTrustBannerOutputProvenanceChipGroupViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes
            .contains(&M5NotebookKernelOutputAccessibilityRoute::KeyboardFocusable)
    {
        violations
            .push(OutputTrustBannerOutputProvenanceChipGroupViolation::AccessibilityRouteMissing);
    }
    if invariants.presents_stale_output_as_live {
        violations.push(OutputTrustBannerOutputProvenanceChipGroupViolation::StaleShownAsLive);
    }
    if invariants.hides_trust_class_behind_hover_only {
        violations.push(OutputTrustBannerOutputProvenanceChipGroupViolation::TrustClassHoverOnly);
    }
    if invariants.flattens_output_into_ambiguous_evidence {
        violations.push(
            OutputTrustBannerOutputProvenanceChipGroupViolation::OutputFlattenedIntoAmbiguousEvidence,
        );
    }
    if invariants.severs_output_provenance {
        violations.push(OutputTrustBannerOutputProvenanceChipGroupViolation::ProvenanceSevered);
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
