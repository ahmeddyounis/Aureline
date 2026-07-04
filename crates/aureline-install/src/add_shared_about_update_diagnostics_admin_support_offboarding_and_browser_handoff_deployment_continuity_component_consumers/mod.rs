//! Shared About/update, diagnostics/support, admin/offboarding, browser-handoff,
//! and docs/help/support-export consumers for the frozen M5 deployment/continuity
//! components.
//!
//! This module is the M05-833 first-consumer adoption lane over the frozen M5
//! deployment/continuity component matrix
//! ([`crate::freeze_the_m5_deployment_continuity_component_matrix`]) and the
//! M05-829..832 primitive resolvers (deployment profile, deployment summary,
//! mirror transition, and handler-ownership / channel-association review). Where
//! the freeze matrix defines the nine reusable install-profile, side-by-side
//! import, rollout-ring, deployment-summary, residual-dependency,
//! control-plane/data-plane, mirror/offline, mode-change, and
//! channel-association primitives, and the sibling resolvers narrow their
//! per-surface truth, this lane proves those nine families are reusable
//! *primitives* rather than one About page, one diagnostics pane, or one
//! admin-only dashboard by adopting them across the four claimed M5 deployment
//! consumer lanes plus a docs/help + support-export lane:
//!
//! 1. an About / update consumer,
//! 2. a diagnostics / support flow,
//! 3. an admin / offboarding flow,
//! 4. a browser / deep-link or handler-review flow, and
//! 5. a docs / help + support-export lane (AC3).
//!
//! Each [`DeploymentConsumerRow`] points back to exactly one canonical component
//! family (its primitive schema + release-proof packet) instead of cloning
//! surface-local install / deployment vocabulary, and every consumer — even a
//! read-only, inspect-only, export-only, or policy-blocked one — keeps the
//! identical operating-mode, ownership / scope, provenance / freshness, residual
//! dependency, and continuity-state labels and the identical degraded-state
//! vocabulary. A narrower consumer discloses the reduction with a
//! reduced-capability banner (and, when it punts to another surface, a companion
//! / browser / handoff note) rather than renaming or dropping governed state, so
//! side-by-side and browser-handoff lanes never drift operating-mode truth.
//!
//! The packet is metadata-only: raw config bytes, credentials, license keys,
//! mirror URLs, provider cursors, and raw device identifiers never cross this
//! boundary; the packet carries only typed class tokens, opaque install /
//! channel / mirror / handler refs, booleans, and redacted labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-deployment-continuity-component-consumer.schema.json`](../../../../schemas/ui/m5-deployment-continuity-component-consumer.schema.json).
//! The contract doc is
//! [`docs/deployment/m5_deployment_continuity_component_consumer_contract.md`](../../../../docs/deployment/m5_deployment_continuity_component_consumer_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_deployment_continuity_component_matrix::{
    M5DeploymentComponentFamily, DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_SCHEMA_REF,
};

/// Schema version stamped on the M05-833 consumer packet.
pub const DEPLOYMENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`DeploymentConsumerPacket`].
pub const DEPLOYMENT_CONSUMER_RECORD_KIND: &str =
    "m5_deployment_continuity_component_consumer_packet";

/// Stable record-kind tag carried by each [`DeploymentConsumerRow`].
pub const DEPLOYMENT_CONSUMER_ROW_RECORD_KIND: &str =
    "m5_deployment_continuity_component_consumer_row";

/// Repo-relative path of the boundary schema.
pub const DEPLOYMENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-deployment-continuity-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const DEPLOYMENT_CONSUMER_DOC_REF: &str =
    "docs/deployment/m5_deployment_continuity_component_consumer_contract.md";

/// Repo-relative path of the frozen deployment/continuity component matrix these
/// consumers adopt.
pub const DEPLOYMENT_CONSUMER_MATRIX_REF: &str = DEPLOYMENT_CONTINUITY_COMPONENT_MATRIX_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const DEPLOYMENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-deployment-continuity-component-consumers";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const DEPLOYMENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-deployment-continuity-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const DEPLOYMENT_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-deployment-continuity-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const DEPLOYMENT_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-deployment-continuity-component-consumer-proof/report.md";

/// The controlled label families a consumer must preserve identically across
/// every surface. These are the track-invariant truth pillars of the
/// deployment/continuity components: operating mode, ownership / scope,
/// provenance / freshness, residual dependency, and continuity state. The union
/// of every row's `preserved_label_families` must cover this set.
pub const REQUIRED_LABEL_FAMILIES: [&str; 5] = [
    "operating_mode",
    "ownership_or_scope",
    "provenance_freshness",
    "residual_dependency",
    "continuity_state",
];

/// The canonical primitive schema that defines a family's contract. Consumers
/// must point at this schema instead of inventing a surface-local one.
///
/// The nine frozen families are narrowed by four sibling resolvers: the
/// install-profile / side-by-side / rollout-ring families by the M05-829
/// deployment-profile primitive, the deployment-summary / residual-dependency /
/// control-plane families by the M05-830 deployment-summary primitive, the
/// mirror / mode-change families by the M05-831 mirror-transition primitive, and
/// the channel-association review family by the M05-832 handler-ownership
/// primitive.
pub fn canonical_schema_ref_for(family: M5DeploymentComponentFamily) -> &'static str {
    use M5DeploymentComponentFamily::*;
    match family {
        InstallProfileCard | SideBySideImportSheet | RolloutRingRow => {
            crate::M5_DEPLOYMENT_PROFILE_SCHEMA_REF
        }
        DeploymentSummaryCard | ResidualDependencyRow | ControlPlaneDataPlaneStatusStrip => {
            crate::M5_DEPLOYMENT_SUMMARY_SCHEMA_REF
        }
        MirrorOfflineArtifactRow | ModeChangeReviewSheet => crate::M5_MIRROR_TRANSITION_SCHEMA_REF,
        ChannelAssociationReviewRow => crate::M5_HANDLER_OWNERSHIP_SCHEMA_REF,
    }
}

/// The canonical release-proof packet that defines a family's first resolved
/// truth. Consumers point back to this packet rather than cloning it.
pub fn canonical_packet_ref_for(family: M5DeploymentComponentFamily) -> &'static str {
    use M5DeploymentComponentFamily::*;
    match family {
        InstallProfileCard | SideBySideImportSheet | RolloutRingRow => {
            crate::M5_DEPLOYMENT_PROFILE_ARTIFACT_REF
        }
        DeploymentSummaryCard | ResidualDependencyRow | ControlPlaneDataPlaneStatusStrip => {
            crate::M5_DEPLOYMENT_SUMMARY_ARTIFACT_REF
        }
        MirrorOfflineArtifactRow | ModeChangeReviewSheet => {
            crate::M5_MIRROR_TRANSITION_ARTIFACT_REF
        }
        ChannelAssociationReviewRow => crate::M5_HANDLER_OWNERSHIP_ARTIFACT_REF,
    }
}

/// The five claimed M5 deployment consumer classes that must each adopt at least
/// one canonical component family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerGroup {
    /// An About or update consumer.
    AboutUpdate,
    /// A diagnostics / support flow.
    DiagnosticsSupport,
    /// An admin / offboarding flow.
    AdminOffboarding,
    /// A browser / deep-link or handler-review flow.
    BrowserHandoff,
    /// A docs / help + support-export lane (AC3).
    DocsHelpRelease,
}

impl ConsumerGroup {
    /// Every consumer group that must be present for cross-surface reuse.
    pub const ALL: [ConsumerGroup; 5] = [
        ConsumerGroup::AboutUpdate,
        ConsumerGroup::DiagnosticsSupport,
        ConsumerGroup::AdminOffboarding,
        ConsumerGroup::BrowserHandoff,
        ConsumerGroup::DocsHelpRelease,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AboutUpdate => "about_update",
            Self::DiagnosticsSupport => "diagnostics_support",
            Self::AdminOffboarding => "admin_offboarding",
            Self::BrowserHandoff => "browser_handoff",
            Self::DocsHelpRelease => "docs_help_release",
        }
    }
}

/// The concrete M5 deployment surface a component is embedded in. Each surface
/// belongs to exactly one [`ConsumerGroup`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentConsumerSurface {
    /// The product About page.
    AboutPage,
    /// The update center / update flow.
    UpdateCenter,
    /// The diagnostics pane.
    DiagnosticsPane,
    /// The support-bundle export flow.
    SupportBundleFlow,
    /// The admin / fleet dashboard.
    AdminFleetDashboard,
    /// The offboarding / uninstall flow.
    OffboardingUninstallFlow,
    /// A browser deep-link handoff surface.
    BrowserDeepLinkHandoff,
    /// A handler-review prompt.
    HandlerReviewPrompt,
    /// The docs / help center.
    HelpCenterDocs,
    /// The support / export replay surface.
    SupportExportReplay,
    /// The release-proof evidence surface.
    ReleaseProofSurface,
}

impl M5DeploymentConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [M5DeploymentConsumerSurface; 11] = [
        M5DeploymentConsumerSurface::AboutPage,
        M5DeploymentConsumerSurface::UpdateCenter,
        M5DeploymentConsumerSurface::DiagnosticsPane,
        M5DeploymentConsumerSurface::SupportBundleFlow,
        M5DeploymentConsumerSurface::AdminFleetDashboard,
        M5DeploymentConsumerSurface::OffboardingUninstallFlow,
        M5DeploymentConsumerSurface::BrowserDeepLinkHandoff,
        M5DeploymentConsumerSurface::HandlerReviewPrompt,
        M5DeploymentConsumerSurface::HelpCenterDocs,
        M5DeploymentConsumerSurface::SupportExportReplay,
        M5DeploymentConsumerSurface::ReleaseProofSurface,
    ];

    /// The consumer group this surface belongs to.
    pub const fn consumer_group(self) -> ConsumerGroup {
        match self {
            Self::AboutPage | Self::UpdateCenter => ConsumerGroup::AboutUpdate,
            Self::DiagnosticsPane | Self::SupportBundleFlow => ConsumerGroup::DiagnosticsSupport,
            Self::AdminFleetDashboard | Self::OffboardingUninstallFlow => {
                ConsumerGroup::AdminOffboarding
            }
            Self::BrowserDeepLinkHandoff | Self::HandlerReviewPrompt => {
                ConsumerGroup::BrowserHandoff
            }
            Self::HelpCenterDocs | Self::SupportExportReplay | Self::ReleaseProofSurface => {
                ConsumerGroup::DocsHelpRelease
            }
        }
    }

    /// True when this surface is a docs / help reference surface (AC3).
    pub const fn is_docs_help(self) -> bool {
        matches!(self, Self::HelpCenterDocs)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AboutPage => "about_page",
            Self::UpdateCenter => "update_center",
            Self::DiagnosticsPane => "diagnostics_pane",
            Self::SupportBundleFlow => "support_bundle_flow",
            Self::AdminFleetDashboard => "admin_fleet_dashboard",
            Self::OffboardingUninstallFlow => "offboarding_uninstall_flow",
            Self::BrowserDeepLinkHandoff => "browser_deep_link_handoff",
            Self::HandlerReviewPrompt => "handler_review_prompt",
            Self::HelpCenterDocs => "help_center_docs",
            Self::SupportExportReplay => "support_export_replay",
            Self::ReleaseProofSurface => "release_proof_surface",
        }
    }
}

/// The rendering authority a consumer exercises over a canonical component.
///
/// A consumer may narrow authority (read-only, inspect-only, compare-only,
/// export-only, policy-blocked) but never rename or drop the governed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityMode {
    /// Full-interactive control (change mode, promote ring, re-point mirror,
    /// reassign handler).
    FullInteractive,
    /// Read-only projection of the component.
    ReadOnly,
    /// Inspect-only: read every governed label but take no action.
    InspectOnly,
    /// Compare-only: read differences but take no action.
    CompareOnly,
    /// Export-only: reconstruct the component from an export packet.
    ExportOnly,
    /// Policy-blocked: the component is visible but action is gated.
    PolicyBlocked,
}

impl AuthorityMode {
    /// Every authority mode, in declaration order.
    pub const ALL: [AuthorityMode; 6] = [
        AuthorityMode::FullInteractive,
        AuthorityMode::ReadOnly,
        AuthorityMode::InspectOnly,
        AuthorityMode::CompareOnly,
        AuthorityMode::ExportOnly,
        AuthorityMode::PolicyBlocked,
    ];

    /// Returns true when the consumer narrows below full-interactive authority
    /// and therefore must disclose the reduction with a banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullInteractive)
    }

    /// The banner `capability_state` label this authority maps to.
    pub const fn capability_state(self) -> &'static str {
        match self {
            Self::FullInteractive => "full",
            Self::ReadOnly => "read_only",
            Self::InspectOnly => "inspect_only",
            Self::CompareOnly => "compare_only",
            Self::ExportOnly => "export_only",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The surface a narrower consumer hands off to when it cannot render the full
/// component locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffTarget {
    /// No handoff: the consumer renders the component in-place.
    None,
    /// Punt to the companion app.
    CompanionApp,
    /// Punt to a read-only browser surface.
    BrowserReadonly,
    /// Punt to a portable handoff / support packet.
    HandoffPacket,
    /// Punt to the desktop primary install / deployment UI.
    DesktopPrimary,
    /// Punt to a headless CLI.
    CliHeadless,
}

impl HandoffTarget {
    /// Returns true when the consumer punts to another surface and therefore
    /// must carry a companion / browser / handoff note.
    pub const fn requires_note(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::HandoffPacket => "handoff_packet",
            Self::DesktopPrimary => "desktop_primary",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// Whether the consumer preserves the canonical component's controlled labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelParityState {
    /// Full operating-mode / ownership / freshness / residual / continuity label
    /// parity.
    Preserved,
    /// Reduced interactivity, disclosed, but the labels are still preserved.
    DisclosedNarrowed,
    /// A label was renamed, flattened, or dropped (red; blocks review).
    RenamedOrDropped,
}

impl LabelParityState {
    /// Returns true when no controlled label is renamed or dropped.
    pub const fn keeps_labels(self) -> bool {
        !matches!(self, Self::RenamedOrDropped)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::RenamedOrDropped => "renamed_or_dropped",
        }
    }
}

/// The copy / export parity a consumer keeps for the adopted component: the
/// governed labels must be copyable as text / JSON / Markdown, and a
/// screenshot-only export is prohibited (it would lose the machine-readable
/// operating-mode / freshness identity and state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExportParity {
    /// The copy formats the consumer offers (must include text / json /
    /// markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The export fields the consumer preserves.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl CopyExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a
    /// screenshot-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.screenshot_only_prohibited
    }
}

/// The reduced-capability banner a narrower consumer shows to disclose the
/// control it drops relative to the full deployment surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedCapabilityBanner {
    /// Stable banner id.
    pub banner_id: String,
    /// The visible, non-generic banner label.
    pub visible_label: String,
    /// The capability state; must match the row's `authority_mode`.
    pub capability_state: String,
    /// The capabilities the narrowed surface is missing relative to full.
    #[serde(default)]
    pub missing_capabilities: Vec<String>,
}

/// One consumer adopting one canonical deployment/continuity component family on
/// one M5 deployment surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentConsumerRow {
    /// Record kind; must equal [`DEPLOYMENT_CONSUMER_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DEPLOYMENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed consumer class.
    pub consumer_group: ConsumerGroup,
    /// The concrete deployment surface; must belong to `consumer_group`.
    pub consumer_surface: M5DeploymentConsumerSurface,
    /// The single canonical component family this consumer reuses.
    pub component_family: M5DeploymentComponentFamily,
    /// The canonical primitive schema for the family. Must equal
    /// `canonical_schema_ref_for(component_family)`.
    pub canonical_family_schema_ref: String,
    /// The canonical release-proof packet(s) this consumer points back to. Must
    /// contain `canonical_packet_ref_for(component_family)`.
    #[serde(default)]
    pub canonical_packet_refs: Vec<String>,
    /// True when the consumer references the canonical family rather than
    /// cloning surface-local install / deployment prose.
    pub references_canonical_not_local_prose: bool,
    /// The rendering authority the consumer exercises.
    pub authority_mode: AuthorityMode,
    /// The controlled label families the consumer preserves verbatim (subset of
    /// [`REQUIRED_LABEL_FAMILIES`]).
    #[serde(default)]
    pub preserved_label_families: Vec<String>,
    /// The degraded-state vocabulary the consumer keeps visible even when
    /// narrowed.
    #[serde(default)]
    pub degraded_state_vocab: Vec<String>,
    /// Whether the consumer keeps the controlled labels.
    pub label_parity: LabelParityState,
    /// The surface a narrower consumer hands off to, if any.
    pub handoff_target: HandoffTarget,
    /// The companion / browser / handoff note ref; required when
    /// `handoff_target` is not `None`.
    #[serde(default)]
    pub handoff_note_ref: String,
    /// The reduced-capability banner, present only when the consumer narrows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduced_capability_banner: Option<ReducedCapabilityBanner>,
    /// The copy / export parity of the adopted component.
    pub copy_export: CopyExportParity,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the adoption was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl DeploymentConsumerRow {
    /// Returns true when the consumer narrows below full authority.
    pub fn is_narrowed(&self) -> bool {
        self.authority_mode.is_narrowed()
    }

    /// The surface's declared group matches the row's declared group.
    pub fn surface_group_consistent(&self) -> bool {
        self.consumer_surface.consumer_group() == self.consumer_group
    }

    /// AC1 (canonical): the consumer points back to exactly one canonical family
    /// — the declared schema matches the family, a release-proof packet is
    /// referenced, and no surface-local prose is cloned.
    pub fn points_to_canonical_family(&self) -> bool {
        self.canonical_family_schema_ref == canonical_schema_ref_for(self.component_family)
            && self
                .canonical_packet_refs
                .iter()
                .any(|p| p == canonical_packet_ref_for(self.component_family))
            && self.references_canonical_not_local_prose
    }

    /// AC2 (parity): the consumer preserves the family's controlled label
    /// families and degraded-state vocabulary rather than renaming or omitting
    /// them.
    pub fn preserves_labels(&self) -> bool {
        self.label_parity.keeps_labels()
            && !self.preserved_label_families.is_empty()
            && self
                .preserved_label_families
                .iter()
                .all(|f| REQUIRED_LABEL_FAMILIES.contains(&f.as_str()))
            && !self.degraded_state_vocab.is_empty()
    }

    /// AC2 (disclosure): a narrower consumer discloses the reduction with a
    /// reduced-capability banner whose state matches the authority mode, and
    /// carries a companion / browser / handoff note whenever it punts to another
    /// surface.
    pub fn discloses_narrowing(&self) -> bool {
        if self.is_narrowed() {
            match &self.reduced_capability_banner {
                None => return false,
                Some(banner) => {
                    if banner.banner_id.trim().is_empty()
                        || banner.visible_label.trim().is_empty()
                        || label_is_generic(&banner.visible_label)
                        || banner.capability_state != self.authority_mode.capability_state()
                        || banner.capability_state == "full"
                        || banner.missing_capabilities.is_empty()
                    {
                        return false;
                    }
                }
            }
            // A narrowed consumer that keeps every label is disclosed-narrowed,
            // never plain preserved.
            if self.label_parity == LabelParityState::Preserved {
                return false;
            }
        } else if self.reduced_capability_banner.is_some() {
            // A full-interactive consumer must not carry a spurious banner.
            return false;
        }
        if self.handoff_target.requires_note() && self.handoff_note_ref.trim().is_empty() {
            return false;
        }
        true
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == DEPLOYMENT_CONSUMER_ROW_RECORD_KIND
            && self.schema_version == DEPLOYMENT_CONSUMER_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.canonical_family_schema_ref.trim().is_empty()
            && !self.canonical_packet_refs.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "surface={surface} group={group} family={family} authority={authority} \
label_parity={label_parity} handoff={handoff}",
            surface = self.consumer_surface.as_str(),
            group = self.consumer_group.as_str(),
            family = self.component_family.as_str(),
            authority = self.authority_mode.capability_state(),
            label_parity = self.label_parity.as_str(),
            handoff = self.handoff_target.as_str(),
        )
    }
}

/// Rolled-up summary of an M05-833 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentConsumerSummary {
    pub row_count: usize,
    pub consumer_group_count: usize,
    pub consumer_surface_count: usize,
    pub component_family_count: usize,
    pub all_rows_point_to_canonical_family: bool,
    pub all_rows_preserve_labels: bool,
    pub all_narrowed_rows_disclose: bool,
    pub all_rows_have_copy_export: bool,
    pub about_update_consumer_present: bool,
    pub diagnostics_support_consumer_present: bool,
    pub admin_offboarding_consumer_present: bool,
    pub browser_handoff_consumer_present: bool,
    pub docs_help_release_consumer_present: bool,
    pub docs_help_reference_present: bool,
    pub label_family_coverage_complete: bool,
    pub families_reused_across_groups: usize,
}

/// Constructor input for [`DeploymentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentConsumerPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<DeploymentConsumerRow>,
}

/// Checked-in M05-833 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentConsumerPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<DeploymentConsumerRow>,
    pub summary: DeploymentConsumerSummary,
}

impl DeploymentConsumerPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: DeploymentConsumerPacketInput) -> Self {
        let mut packet = Self {
            schema_version: DEPLOYMENT_CONSUMER_SCHEMA_VERSION,
            record_kind: DEPLOYMENT_CONSUMER_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: DeploymentConsumerSummary {
                row_count: 0,
                consumer_group_count: 0,
                consumer_surface_count: 0,
                component_family_count: 0,
                all_rows_point_to_canonical_family: false,
                all_rows_preserve_labels: false,
                all_narrowed_rows_disclose: false,
                all_rows_have_copy_export: false,
                about_update_consumer_present: false,
                diagnostics_support_consumer_present: false,
                admin_offboarding_consumer_present: false,
                browser_handoff_consumer_present: false,
                docs_help_release_consumer_present: false,
                docs_help_reference_present: false,
                label_family_coverage_complete: false,
                families_reused_across_groups: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5DeploymentComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// The union of every row's preserved label families.
    pub fn covered_label_families(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.preserved_label_families.iter().cloned())
            .collect()
    }

    /// The count of component families adopted by two or more distinct consumer
    /// groups — the strongest evidence that a family is a reusable primitive.
    pub fn families_reused_across_groups(&self) -> usize {
        M5DeploymentComponentFamily::ALL
            .iter()
            .filter(|family| {
                let groups: BTreeSet<ConsumerGroup> = self
                    .rows
                    .iter()
                    .filter(|r| r.component_family == **family)
                    .map(|r| r.consumer_group)
                    .collect();
                groups.len() >= 2
            })
            .count()
    }

    /// Whether some docs / help surface references the canonical families (AC3).
    pub fn has_docs_help_reference(&self) -> bool {
        self.rows
            .iter()
            .any(|r| r.consumer_surface.is_docs_help() && r.references_canonical_not_local_prose)
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> DeploymentConsumerSummary {
        let mut groups = BTreeSet::new();
        let mut surfaces = BTreeSet::new();
        let mut families = BTreeSet::new();
        for row in &self.rows {
            groups.insert(row.consumer_group);
            surfaces.insert(row.consumer_surface);
            families.insert(row.component_family);
        }

        let has_group = |g: ConsumerGroup| groups.contains(&g);
        let covered = self.covered_label_families();

        DeploymentConsumerSummary {
            row_count: self.rows.len(),
            consumer_group_count: groups.len(),
            consumer_surface_count: surfaces.len(),
            component_family_count: families.len(),
            all_rows_point_to_canonical_family: self
                .rows
                .iter()
                .all(DeploymentConsumerRow::points_to_canonical_family),
            all_rows_preserve_labels: self
                .rows
                .iter()
                .all(DeploymentConsumerRow::preserves_labels),
            all_narrowed_rows_disclose: self
                .rows
                .iter()
                .all(DeploymentConsumerRow::discloses_narrowing),
            all_rows_have_copy_export: self.rows.iter().all(|r| r.copy_export.is_complete()),
            about_update_consumer_present: has_group(ConsumerGroup::AboutUpdate),
            diagnostics_support_consumer_present: has_group(ConsumerGroup::DiagnosticsSupport),
            admin_offboarding_consumer_present: has_group(ConsumerGroup::AdminOffboarding),
            browser_handoff_consumer_present: has_group(ConsumerGroup::BrowserHandoff),
            docs_help_release_consumer_present: has_group(ConsumerGroup::DocsHelpRelease),
            docs_help_reference_present: self.has_docs_help_reference(),
            label_family_coverage_complete: REQUIRED_LABEL_FAMILIES
                .iter()
                .all(|f| covered.contains(*f)),
            families_reused_across_groups: self.families_reused_across_groups(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<DeploymentConsumerViolation> {
        let mut violations = Vec::new();

        if self.schema_version != DEPLOYMENT_CONSUMER_SCHEMA_VERSION {
            violations.push(DeploymentConsumerViolation::SchemaVersion {
                expected: DEPLOYMENT_CONSUMER_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != DEPLOYMENT_CONSUMER_RECORD_KIND {
            violations.push(DeploymentConsumerViolation::RecordKind {
                expected: DEPLOYMENT_CONSUMER_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(DeploymentConsumerViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_groups = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(DeploymentConsumerViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_groups.insert(row.consumer_group);

            if !row.is_complete() {
                violations.push(DeploymentConsumerViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // The concrete surface must belong to the declared consumer group.
            if !row.surface_group_consistent() {
                violations.push(DeploymentConsumerViolation::SurfaceGroupMismatch {
                    id: row.row_id.clone(),
                });
            }

            // AC1: exactly one canonical family, no cloned surface-local prose.
            if !row.points_to_canonical_family() {
                violations.push(DeploymentConsumerViolation::NotCanonicalFamily {
                    id: row.row_id.clone(),
                });
            }

            // AC2: controlled label families / degraded vocab preserved.
            if !row.preserves_labels() {
                violations.push(DeploymentConsumerViolation::LabelParityBroken {
                    id: row.row_id.clone(),
                });
            }

            // AC2: narrower consumers disclose reduction with banner + note.
            if !row.discloses_narrowing() {
                violations.push(DeploymentConsumerViolation::NarrowedWithoutDisclosure {
                    id: row.row_id.clone(),
                });
            }

            // Copy / export parity: text / JSON / Markdown, screenshot prohibited.
            if !row.copy_export.is_complete() {
                violations.push(DeploymentConsumerViolation::MissingCopyExportParity {
                    id: row.row_id.clone(),
                });
            }
        }

        // Cross-surface reuse spans all five claimed consumer classes.
        for group in ConsumerGroup::ALL {
            if !seen_groups.contains(&group) {
                violations.push(DeploymentConsumerViolation::MissingConsumerGroup { group });
            }
        }

        // Every frozen family is adopted by at least one consumer.
        let families = self.represented_families();
        for family in M5DeploymentComponentFamily::ALL {
            if !families.contains(&family) {
                violations.push(DeploymentConsumerViolation::MissingFamilyCoverage { family });
            }
        }

        // AC1: at least one family is reused across two or more consumer groups
        // so multiple M5 surfaces point back to one canonical family.
        if self.families_reused_across_groups() == 0 {
            violations.push(DeploymentConsumerViolation::NoFamilyReusedAcrossGroups);
        }

        // AC2: the controlled label families are collectively preserved.
        let covered = self.covered_label_families();
        for family in REQUIRED_LABEL_FAMILIES {
            if !covered.contains(family) {
                violations.push(DeploymentConsumerViolation::MissingLabelFamily {
                    family: family.to_owned(),
                });
            }
        }

        // AC3: a docs / help consumer references the canonical components rather
        // than cloning local install / deployment vocabulary.
        if !self.has_docs_help_reference() {
            violations.push(DeploymentConsumerViolation::MissingDocsHelpReference);
        }

        if self.summary != self.computed_summary() {
            violations.push(DeploymentConsumerViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("consumer packet serializes"),
        ) {
            violations.push(DeploymentConsumerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("consumer packet serializes")
    }

    /// Deterministic CSV of the adoption rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,consumer_group,consumer_surface,component_family,authority,label_parity,handoff\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{group},{surface},{family},{authority},{label_parity},{handoff}\n",
                id = row.row_id,
                group = row.consumer_group.as_str(),
                surface = row.consumer_surface.as_str(),
                family = row.component_family.as_str(),
                authority = row.authority_mode.capability_state(),
                label_parity = row.label_parity.as_str(),
                handoff = row.handoff_target.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Deployment/Continuity Component Consumers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Rows: {} across {} consumer groups and {} / {} frozen families\n",
            self.summary.row_count,
            self.summary.consumer_group_count,
            self.represented_families().len(),
            M5DeploymentComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Families reused across groups: {}\n",
            self.summary.families_reused_across_groups,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in consumer export.
pub fn current_m5_deployment_continuity_component_consumers_export(
) -> Result<DeploymentConsumerPacket, DeploymentConsumerArtifactError> {
    let packet: DeploymentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-deployment-continuity-component-consumer-proof/support_export.json"
    )))
    .map_err(DeploymentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DeploymentConsumerArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in consumer export.
#[derive(Debug)]
pub enum DeploymentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<DeploymentConsumerViolation>),
}

impl fmt::Display for DeploymentConsumerArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "consumer export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "consumer export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for DeploymentConsumerArtifactError {}

/// Validation failure for M05-833 consumer packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeploymentConsumerViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    SurfaceGroupMismatch { id: String },
    NotCanonicalFamily { id: String },
    LabelParityBroken { id: String },
    NarrowedWithoutDisclosure { id: String },
    MissingCopyExportParity { id: String },
    MissingConsumerGroup { group: ConsumerGroup },
    MissingFamilyCoverage { family: M5DeploymentComponentFamily },
    NoFamilyReusedAcrossGroups,
    MissingLabelFamily { family: String },
    MissingDocsHelpReference,
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for DeploymentConsumerViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete consumer row: {id}"),
            Self::SurfaceGroupMismatch { id } => {
                write!(
                    f,
                    "row {id} declares a surface that does not belong to its consumer group"
                )
            }
            Self::NotCanonicalFamily { id } => {
                write!(
                    f,
                    "row {id} does not point back to exactly one canonical component family"
                )
            }
            Self::LabelParityBroken { id } => {
                write!(
                    f,
                    "row {id} renames or drops a canonical operating-mode, ownership/scope, \
provenance/freshness, residual-dependency, or continuity-state label"
                )
            }
            Self::NarrowedWithoutDisclosure { id } => {
                write!(
                    f,
                    "row {id} narrows authority without a reduced-capability banner or handoff note"
                )
            }
            Self::MissingCopyExportParity { id } => {
                write!(
                    f,
                    "row {id} is missing text / JSON / Markdown copy-export parity"
                )
            }
            Self::MissingConsumerGroup { group } => {
                write!(f, "consumer group {group:?} is not adopted in the packet")
            }
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not adopted in the packet"
                )
            }
            Self::NoFamilyReusedAcrossGroups => write!(
                f,
                "no component family is adopted across two or more consumer groups"
            ),
            Self::MissingLabelFamily { family } => {
                write!(
                    f,
                    "controlled label family {family} is not preserved anywhere"
                )
            }
            Self::MissingDocsHelpReference => write!(
                f,
                "no docs / help consumer references the canonical component families"
            ),
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for DeploymentConsumerViolation {}

/// Whether a banner label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "read only"
            | "read-only"
            | "offline"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in consumer packet. This is the one source of
/// truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_m5_deployment_continuity_component_consumers_packet() -> DeploymentConsumerPacket {
    DeploymentConsumerPacket::new(DeploymentConsumerPacketInput {
        packet_id: "m5-deployment-continuity-component-consumers:stable:0001".to_owned(),
        as_of: "2026-07-04T00:00:00Z".to_owned(),
        matrix_ref: DEPLOYMENT_CONSUMER_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:deployment-continuity-consumer:{id}")]
}

fn copy_export(fields: &[&str]) -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn labels(families: &[&str]) -> Vec<String> {
    families.iter().map(|f| (*f).to_owned()).collect()
}

fn degraded_vocab() -> Vec<String> {
    vec![
        "control_plane_impaired".to_owned(),
        "mirror_stale".to_owned(),
        "offline_cache_only".to_owned(),
        "rollout_paused".to_owned(),
        "residual_vendor_dependency".to_owned(),
        "state_root_unavailable".to_owned(),
    ]
}

fn banner(
    id: &str,
    label: &str,
    authority: AuthorityMode,
    missing: &[&str],
) -> ReducedCapabilityBanner {
    ReducedCapabilityBanner {
        banner_id: id.to_owned(),
        visible_label: label.to_owned(),
        capability_state: authority.capability_state().to_owned(),
        missing_capabilities: missing.iter().map(|m| (*m).to_owned()).collect(),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    consumer_surface: M5DeploymentConsumerSurface,
    component_family: M5DeploymentComponentFamily,
    authority_mode: AuthorityMode,
    label_families: &[&str],
    export_fields: &[&str],
    handoff_target: HandoffTarget,
    handoff_note_ref: &str,
    reduced_capability_banner: Option<ReducedCapabilityBanner>,
) -> DeploymentConsumerRow {
    let label_parity = if authority_mode.is_narrowed() {
        LabelParityState::DisclosedNarrowed
    } else {
        LabelParityState::Preserved
    };
    DeploymentConsumerRow {
        record_kind: DEPLOYMENT_CONSUMER_ROW_RECORD_KIND.to_owned(),
        schema_version: DEPLOYMENT_CONSUMER_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        consumer_group: consumer_surface.consumer_group(),
        consumer_surface,
        component_family,
        canonical_family_schema_ref: canonical_schema_ref_for(component_family).to_owned(),
        canonical_packet_refs: vec![canonical_packet_ref_for(component_family).to_owned()],
        references_canonical_not_local_prose: true,
        authority_mode,
        preserved_label_families: labels(label_families),
        degraded_state_vocab: degraded_vocab(),
        label_parity,
        handoff_target,
        handoff_note_ref: handoff_note_ref.to_owned(),
        reduced_capability_banner,
        copy_export: copy_export(export_fields),
        source_refs: vec![DEPLOYMENT_CONSUMER_MATRIX_REF.to_owned()],
        observed_at: "2026-07-04T00:00:00Z".to_owned(),
        evidence_refs: ev(row_id),
    }
}

fn seeded_rows() -> Vec<DeploymentConsumerRow> {
    use AuthorityMode::*;
    use M5DeploymentComponentFamily::*;
    use M5DeploymentConsumerSurface::*;

    vec![
        // --- About / update consumer ---------------------------------------
        // About page rendering the install-profile card full-interactive.
        row(
            "consumer:about-update:install-profile-card",
            AboutPage,
            InstallProfileCard,
            FullInteractive,
            &["operating_mode", "ownership_or_scope"],
            &["install_mode", "channel", "updater_owner", "state_root"],
            HandoffTarget::None,
            "",
            None,
        ),
        // About page rendering the deployment summary card full-interactive.
        row(
            "consumer:about-update:deployment-summary-card",
            AboutPage,
            DeploymentSummaryCard,
            FullInteractive,
            &["operating_mode", "ownership_or_scope"],
            &["operating_mode", "tenant_region", "control_plane", "data_plane"],
            HandoffTarget::None,
            "",
            None,
        ),
        // Update center rendering the rollout-ring row full-interactive.
        row(
            "consumer:about-update:rollout-ring-row",
            UpdateCenter,
            RolloutRingRow,
            FullInteractive,
            &["operating_mode", "continuity_state"],
            &["ring", "promotion_state", "rollback_available"],
            HandoffTarget::None,
            "",
            None,
        ),
        // Update center hosting the mode-change review sheet full-interactive.
        row(
            "consumer:about-update:mode-change-review-sheet",
            UpdateCenter,
            ModeChangeReviewSheet,
            FullInteractive,
            &["operating_mode", "continuity_state"],
            &["from_mode", "to_mode", "boundary_change", "cache_rollback"],
            HandoffTarget::None,
            "",
            None,
        ),
        // --- Diagnostics / support flow ------------------------------------
        // Diagnostics pane reusing the install-profile card read-only (2nd group).
        row(
            "consumer:diagnostics-support:install-profile-card",
            DiagnosticsPane,
            InstallProfileCard,
            ReadOnly,
            &["operating_mode", "ownership_or_scope"],
            &["install_mode", "channel", "updater_owner", "state_root"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:diagnostics-support:install-profile-card",
                "Read-only diagnostics install profile: read install mode, channel, updater owner, and state roots; changing them stays on the About / update surface",
                ReadOnly,
                &["change_channel", "reassign_updater_owner"],
            )),
        ),
        // Diagnostics pane reusing the control-plane/data-plane strip read-only.
        row(
            "consumer:diagnostics-support:control-plane-data-plane-strip",
            DiagnosticsPane,
            ControlPlaneDataPlaneStatusStrip,
            ReadOnly,
            &["operating_mode", "continuity_state"],
            &["control_plane_state", "data_plane_state", "local_runtime_unaffected"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:diagnostics-support:control-plane-data-plane-strip",
                "Read-only plane status: read control-plane versus data-plane state so a managed control-plane outage never reads as a broken local runtime",
                ReadOnly,
                &["retry_control_plane", "open_incident"],
            )),
        ),
        // Support bundle flow reconstructing the mirror/offline artifact row export-only.
        row(
            "consumer:diagnostics-support:mirror-offline-artifact-row",
            SupportBundleFlow,
            MirrorOfflineArtifactRow,
            ExportOnly,
            &["provenance_freshness", "continuity_state"],
            &["mirror_source", "freshness", "signature_state"],
            HandoffTarget::HandoffPacket,
            "handoff:diagnostics-support:mirror-artifact-support-packet",
            Some(banner(
                "banner:diagnostics-support:mirror-offline-artifact-row",
                "Export-only support replay: reconstruct mirror source, freshness, and signature truth from the support packet; re-point the mirror in the desktop app",
                ExportOnly,
                &["repoint_mirror", "reverify_signature"],
            )),
        ),
        // Support bundle flow reusing the residual-dependency row read-only.
        row(
            "consumer:diagnostics-support:residual-dependency-row",
            SupportBundleFlow,
            ResidualDependencyRow,
            ReadOnly,
            &["residual_dependency", "provenance_freshness"],
            &["vendor_dependency", "dependency_class", "required_for_operation"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:diagnostics-support:residual-dependency-row",
                "Read-only residual dependency: read any remaining license, update, identity, telemetry, or model dependency a self-hosted install still carries",
                ReadOnly,
                &["edit_dependency_policy"],
            )),
        ),
        // --- Admin / offboarding flow --------------------------------------
        // Admin fleet dashboard reusing the deployment summary card full-interactive (2nd group).
        row(
            "consumer:admin-offboarding:deployment-summary-card",
            AdminFleetDashboard,
            DeploymentSummaryCard,
            FullInteractive,
            &["operating_mode", "ownership_or_scope"],
            &["operating_mode", "tenant_region", "control_plane", "data_plane"],
            HandoffTarget::None,
            "",
            None,
        ),
        // Admin fleet dashboard reusing the rollout-ring row inspect-only (2nd group).
        row(
            "consumer:admin-offboarding:rollout-ring-row",
            AdminFleetDashboard,
            RolloutRingRow,
            InspectOnly,
            &["operating_mode", "continuity_state"],
            &["ring", "promotion_state", "rollback_available"],
            HandoffTarget::CompanionApp,
            "handoff:admin-offboarding:rollout-ring-open-in-console",
            Some(banner(
                "banner:admin-offboarding:rollout-ring-row",
                "Inspect-only fleet rollout view: read the ring and promotion state before promoting; promotion is driven from the managed console",
                InspectOnly,
                &["promote_ring", "pause_ring"],
            )),
        ),
        // Admin fleet dashboard reusing the install-profile card read-only (3rd group).
        row(
            "consumer:admin-offboarding:install-profile-card",
            AdminFleetDashboard,
            InstallProfileCard,
            ReadOnly,
            &["operating_mode", "ownership_or_scope"],
            &["install_mode", "channel", "updater_owner", "state_root"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:admin-offboarding:install-profile-card",
                "Read-only fleet install profile: read each managed install's mode, channel, updater owner, and state roots; policy changes stay in the admin console",
                ReadOnly,
                &["change_channel", "override_state_root"],
            )),
        ),
        // Offboarding / uninstall flow driving the side-by-side import sheet full-interactive.
        row(
            "consumer:admin-offboarding:side-by-side-import-sheet",
            OffboardingUninstallFlow,
            SideBySideImportSheet,
            FullInteractive,
            &["ownership_or_scope", "continuity_state"],
            &["import_source", "handler_ownership", "isolation_preserved"],
            HandoffTarget::None,
            "",
            None,
        ),
        // --- Browser / deep-link or handler-review flow --------------------
        // Handler-review prompt driving the channel-association review row full-interactive.
        row(
            "consumer:browser-handoff:channel-association-review-row",
            HandlerReviewPrompt,
            ChannelAssociationReviewRow,
            FullInteractive,
            &["ownership_or_scope", "continuity_state"],
            &["channel", "handler_association", "current_owner", "reviewed_before_apply"],
            HandoffTarget::None,
            "",
            None,
        ),
        // Browser deep-link handoff reusing the mirror/offline artifact row read-only (2nd group).
        row(
            "consumer:browser-handoff:mirror-offline-artifact-row",
            BrowserDeepLinkHandoff,
            MirrorOfflineArtifactRow,
            ReadOnly,
            &["provenance_freshness", "continuity_state"],
            &["mirror_source", "freshness", "signature_state"],
            HandoffTarget::BrowserReadonly,
            "handoff:browser-handoff:open-mirror-artifact-in-desktop",
            Some(banner(
                "banner:browser-handoff:mirror-offline-artifact-row",
                "Read-only browser artifact view: read mirror source and freshness so a stale mirrored artifact never reads as a live source; open the desktop app to act",
                ReadOnly,
                &["repoint_mirror", "reverify_signature"],
            )),
        ),
        // --- Docs / help + support-export lane (AC3) -----------------------
        // Docs / help center referencing the deployment summary card read-only (AC3, 3rd group).
        row(
            "consumer:docs-help-release:deployment-summary-card-docs",
            HelpCenterDocs,
            DeploymentSummaryCard,
            ReadOnly,
            &["operating_mode", "ownership_or_scope"],
            &["operating_mode", "tenant_region", "control_plane", "data_plane"],
            HandoffTarget::None,
            "",
            Some(banner(
                "banner:docs-help-release:deployment-summary-card-docs",
                "Read-only help reference: explains desktop / managed / self-hosted / portable / air-gapped operating mode and control-plane-versus-data-plane truth for each deployment surface",
                ReadOnly,
                &["change_operating_mode"],
            )),
        ),
        // Support export replay reconstructing the residual-dependency row export-only (2nd group).
        row(
            "consumer:docs-help-release:residual-dependency-row",
            SupportExportReplay,
            ResidualDependencyRow,
            ExportOnly,
            &["residual_dependency", "provenance_freshness"],
            &["vendor_dependency", "dependency_class", "required_for_operation"],
            HandoffTarget::HandoffPacket,
            "handoff:docs-help-release:residual-dependency-support-packet",
            Some(banner(
                "banner:docs-help-release:residual-dependency-row",
                "Export-only support replay: reconstruct the remaining vendor dependency a self-hosted install carries from the support packet; open the admin console to change it",
                ExportOnly,
                &["edit_dependency_policy"],
            )),
        ),
        // Release-proof surface reconstructing the rollout-ring row export-only (3rd group).
        row(
            "consumer:docs-help-release:rollout-ring-row",
            ReleaseProofSurface,
            RolloutRingRow,
            ExportOnly,
            &["operating_mode", "continuity_state"],
            &["ring", "promotion_state", "rollback_available"],
            HandoffTarget::HandoffPacket,
            "handoff:docs-help-release:rollout-ring-release-packet",
            Some(banner(
                "banner:docs-help-release:rollout-ring-row",
                "Export-only release evidence: reconstruct the ring and promotion state from the release-proof packet; promotion is driven from the managed console",
                ExportOnly,
                &["promote_ring", "pause_ring"],
            )),
        ),
    ]
}
