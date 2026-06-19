//! Governed README/changelog release-docs maintenance surfaces.
//!
//! This module owns the dedicated maintenance surfaces for release-facing prose
//! — README, changelog, onboarding notes, release notes, and migration notes —
//! and gives them the same branch/release/channel truth Aureline already
//! expects for code, release packets, and review surfaces. Each
//! [`ReleaseDocsMaintenanceSurface`] makes the artifact kind, the
//! branch/release/channel [`DocsPublishScope`], the pending suggestion refs, the
//! [`ReleaseDocsCompareEntry`] history, and the publish/export
//! [`DocsPublishBoundaryState`] visible **before** a user edits or exports text,
//! and keeps them inspectable **after** the user leaves the surface through the
//! [`ReleaseDocsReviewPacket`].
//!
//! The distinctive invariant this lane adds on top of [`crate::maintenance`] is
//! the [`ReleaseDocsEvidenceScope`]: a note drafted for the next beta or for a
//! private branch is labeled local/shared/prerelease so it can never masquerade
//! as the currently installed stable truth. Compare history stays reopenable,
//! diff review stays available, and the surfaces stay wired to the in-product
//! release center, help browser, and About panel rather than to a browser-only
//! or vendor-console-only path.
//!
//! The records carry only inspectable metadata — stable refs, scope tokens,
//! boundary states, and disclosure notes. Raw Markdown bodies, raw source files,
//! rendered HTML, raw diffs, raw URLs, and credentials never cross this
//! boundary; they are referenced through stable opaque refs so that release-docs
//! work can be reviewed and exported without screenshots or copy/paste
//! archaeology.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::maintenance::{
    DocsArtifactKind, DocsAudienceScope, DocsHandoffBanner, DocsMaintenanceAction,
    DocsMaintenanceFinding, DocsPublishBoundaryState, DocsPublishScope, DocsSourceVersionBadge,
    DocsSuggestionCard, DocsSuggestionTrigger, DOCS_MAINTENANCE_SCHEMA_VERSION,
    DOCS_SUGGESTION_CARD_RECORD_KIND,
};
use crate::{CitationConfidenceClass, DocsFreshnessClass, VersionMatchState};

/// Schema version shared by every release-docs maintenance record.
pub const RELEASE_DOCS_MAINTENANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for [`ReleaseDocsMaintenanceSurface`].
pub const RELEASE_DOCS_MAINTENANCE_SURFACE_RECORD_KIND: &str =
    "release_docs_maintenance_surface_record";

/// Stable record kind for [`ReleaseDocsMaintenanceContract`].
pub const RELEASE_DOCS_MAINTENANCE_CONTRACT_RECORD_KIND: &str =
    "release_docs_maintenance_contract_record";

/// Stable record kind for [`ReleaseDocsSurfaceProjection`].
pub const RELEASE_DOCS_SURFACE_PROJECTION_RECORD_KIND: &str =
    "release_docs_surface_projection_record";

/// Stable record kind for [`ReleaseDocsReviewPacket`].
pub const RELEASE_DOCS_REVIEW_PACKET_RECORD_KIND: &str = "release_docs_review_packet_record";

/// Stable id for the seeded release-docs maintenance contract.
pub const RELEASE_DOCS_MAINTENANCE_CONTRACT_ID: &str = "release-docs:readme-changelog:beta:v1";

/// Stable version ref for the seeded contract.
pub const RELEASE_DOCS_MAINTENANCE_VERSION_REF: &str =
    "release-docs-rev:readme-changelog:2026.06.01-01";

/// Repository-relative schema ref for release-docs surface records.
pub const RELEASE_DOCS_MAINTENANCE_SCHEMA_REF: &str =
    "schemas/docs/release-docs-maintenance.schema.json";

/// Repository-relative help/contract doc ref for the surface.
pub const RELEASE_DOCS_MAINTENANCE_DOC_REF: &str = "docs/help/readme-changelog-maintenance.md";

/// Repository-relative fixture directory ref for release-docs scenarios.
pub const RELEASE_DOCS_MAINTENANCE_FIXTURE_DIR: &str =
    "fixtures/docs/m5/readme-changelog-scenarios/";

/// Stable user-facing label for the open-source action.
pub const OPEN_RELEASE_DOCS_SOURCE_ACTION_LABEL: &str = "Open source";

/// Stable user-facing label for the diff-review action.
pub const REVIEW_RELEASE_DOCS_DIFF_ACTION_LABEL: &str = "Review diff";

/// Stable user-facing label for the reopen-comparison action.
pub const REOPEN_RELEASE_DOCS_COMPARE_ACTION_LABEL: &str = "Reopen comparison";

const GENERATED_AT: &str = "2026-06-01T15:00:00Z";

/// Local-only versus shared evidence scope for a release-docs artifact.
///
/// This distinguishes prose that is merely a local or branch draft from prose
/// that has actually been shared or installed. It exists so a note drafted for
/// the next beta or for a private branch is never presented as the currently
/// installed stable truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseDocsEvidenceScope {
    /// A local working draft that has never left the workspace.
    LocalDraft,
    /// A draft that lives on a private or feature branch and is not shared.
    PrivateBranch,
    /// Work shared for review inside a scoped review handoff.
    SharedReview,
    /// Work published to a prerelease/beta/next channel; not installed stable.
    SharedPrerelease,
    /// Prose that matches the currently installed stable docs.
    InstalledStable,
}

impl ReleaseDocsEvidenceScope {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDraft => "local_draft",
            Self::PrivateBranch => "private_branch",
            Self::SharedReview => "shared_review",
            Self::SharedPrerelease => "shared_prerelease",
            Self::InstalledStable => "installed_stable",
        }
    }

    /// Returns true when this scope is the currently installed stable truth.
    pub const fn is_installed_stable(self) -> bool {
        matches!(self, Self::InstalledStable)
    }

    /// Returns true when this scope has crossed out of the local workspace.
    pub const fn is_shared(self) -> bool {
        matches!(
            self,
            Self::SharedReview | Self::SharedPrerelease | Self::InstalledStable
        )
    }
}

/// What two revisions a release-docs comparison places side by side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseDocsCompareKind {
    /// The local working copy compared against the installed stable docs.
    WorkingVsInstalled,
    /// A branch head compared against a release tag.
    BranchVsRelease,
    /// Two historical revisions compared against each other.
    RevisionVsRevision,
    /// One channel compared against another (for example beta versus stable).
    ChannelVsChannel,
}

impl ReleaseDocsCompareKind {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkingVsInstalled => "working_vs_installed",
            Self::BranchVsRelease => "branch_vs_release",
            Self::RevisionVsRevision => "revision_vs_revision",
            Self::ChannelVsChannel => "channel_vs_channel",
        }
    }
}

/// In-product surface a release-docs maintenance row integrates with.
///
/// Every target here is an in-product Aureline surface; there is intentionally
/// no browser-only or vendor-console-only target, so a release-docs maintenance
/// path can never become a browser- or console-only path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseDocsIntegrationTarget {
    /// The release center.
    ReleaseCenter,
    /// The in-product help browser.
    HelpBrowser,
    /// The About panel.
    AboutPanel,
    /// The support-export surface.
    SupportExport,
}

impl ReleaseDocsIntegrationTarget {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::HelpBrowser => "help_browser",
            Self::AboutPanel => "about_panel",
            Self::SupportExport => "support_export",
        }
    }
}

/// One reopenable comparison between two revisions of a release-docs artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseDocsCompareEntry {
    /// Stable comparison id.
    pub compare_id: String,
    /// What two revisions this comparison places side by side.
    pub compare_kind: ReleaseDocsCompareKind,
    /// Stable opaque ref for the base revision (never a raw body).
    pub base_ref: String,
    /// User-facing label for the base revision.
    pub base_label: String,
    /// Stable opaque ref for the target revision (never a raw body).
    pub target_ref: String,
    /// User-facing label for the target revision.
    pub target_label: String,
    /// Deterministic time the comparison was taken.
    pub compared_at: String,
    /// Stable opaque ref to the diff summary (never a raw diff body).
    pub diff_summary_ref: String,
    /// Section refs the comparison reports as changed.
    pub changed_section_refs: Vec<String>,
    /// Evidence scope the comparison reflects.
    pub evidence_scope: ReleaseDocsEvidenceScope,
    /// Whether the comparison can be reopened from history (must be true).
    pub reopenable: bool,
    /// Reopen-comparison action.
    pub reopen_action: DocsMaintenanceAction,
}

impl ReleaseDocsCompareEntry {
    fn validate(&self, surface_id: &str, findings: &mut Vec<ReleaseDocsFinding>) {
        if self.compare_id.trim().is_empty()
            || self.base_ref.trim().is_empty()
            || self.target_ref.trim().is_empty()
            || self.base_label.trim().is_empty()
            || self.target_label.trim().is_empty()
            || self.compared_at.trim().is_empty()
            || self.diff_summary_ref.trim().is_empty()
        {
            findings.push(ReleaseDocsFinding::new(
                surface_id,
                "compare_entry.identity",
                "compare entry id, base/target refs and labels, compared_at, and diff summary ref must be non-empty",
            ));
        }
        if self.base_ref == self.target_ref {
            findings.push(ReleaseDocsFinding::new(
                surface_id,
                "compare_entry.distinct_revisions",
                "compare entry base and target revisions must differ",
            ));
        }
        if !self.reopenable {
            findings.push(ReleaseDocsFinding::new(
                surface_id,
                "compare_entry.reopenable",
                "compare history entries must stay reopenable",
            ));
        }
        if !self.reopen_action.is_keyboard_action() {
            findings.push(ReleaseDocsFinding::new(
                surface_id,
                "compare_entry.reopen_action",
                "reopen-comparison action must be keyboard reachable and well formed",
            ));
        }
    }
}

/// One link from a release-docs surface into an in-product integration target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseDocsIntegrationAnchor {
    /// In-product target this anchor opens.
    pub target: ReleaseDocsIntegrationTarget,
    /// Stable surface ref the anchor opens.
    pub anchor_ref: String,
    /// Open action for the anchor.
    pub open_action: DocsMaintenanceAction,
}

impl ReleaseDocsIntegrationAnchor {
    fn validate(&self, surface_id: &str, findings: &mut Vec<ReleaseDocsFinding>) {
        if self.anchor_ref.trim().is_empty() {
            findings.push(ReleaseDocsFinding::new(
                surface_id,
                "integration_anchor.identity",
                "integration anchor ref must be non-empty",
            ));
        }
        if !self.open_action.is_keyboard_action() {
            findings.push(ReleaseDocsFinding::new(
                surface_id,
                "integration_anchor.open_action",
                "integration anchor open action must be keyboard reachable and well formed",
            ));
        }
    }
}

/// One dedicated README/changelog/onboarding release-docs maintenance surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseDocsMaintenanceSurface {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable surface id.
    pub surface_id: String,
    /// Artifact family the surface maintains.
    pub artifact_kind: DocsArtifactKind,
    /// Stable artifact ref (path or artifact id, never a raw body).
    pub artifact_ref: String,
    /// Audience the artifact targets.
    pub audience_scope: DocsAudienceScope,
    /// Branch/release/channel scope.
    pub publish_scope: DocsPublishScope,
    /// Local-only versus publish-boundary posture.
    pub publish_boundary_state: DocsPublishBoundaryState,
    /// Local-only versus shared evidence scope.
    pub evidence_scope: ReleaseDocsEvidenceScope,
    /// Whether the branch/release/channel scope is shown before edit or export
    /// (must be true).
    pub scope_visible_before_edit: bool,
    /// Human-readable scope summary shown before the user edits or exports.
    pub active_scope_summary: String,
    /// Whether maintenance stays on an in-product path rather than a
    /// browser-only or vendor-console-only path (must be true).
    pub in_product_maintenance_path: bool,
    /// Docs source/version badge.
    pub source_version_badge: DocsSourceVersionBadge,
    /// Pending suggestion-card count.
    pub pending_suggestion_count: usize,
    /// Pending suggestion-card refs referenced by this surface.
    pub pending_suggestion_refs: Vec<String>,
    /// Reopenable compare history.
    pub compare_history: Vec<ReleaseDocsCompareEntry>,
    /// In-product integration anchors (release center, help, About, support).
    pub integration_anchors: Vec<ReleaseDocsIntegrationAnchor>,
    /// Publish-boundary notes shown before apply or export.
    pub publish_boundary_notes: Vec<String>,
    /// Always-available open-source (switch to source / open file) action.
    pub open_source_action: DocsMaintenanceAction,
    /// Always-available diff-review action.
    pub open_diff_review_action: DocsMaintenanceAction,
    /// Apply or export action when the boundary allows it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_export_action: Option<DocsMaintenanceAction>,
    /// Surface refs that render this row.
    pub surface_refs: Vec<String>,
}

impl ReleaseDocsMaintenanceSurface {
    fn validate(&self, findings: &mut Vec<ReleaseDocsFinding>) {
        if self.record_kind != RELEASE_DOCS_MAINTENANCE_SURFACE_RECORD_KIND {
            findings.push(ReleaseDocsFinding::new(
                &self.surface_id,
                "surface.record_kind",
                "surface record_kind is unsupported",
            ));
        }
        if self.schema_version != RELEASE_DOCS_MAINTENANCE_SCHEMA_VERSION {
            findings.push(ReleaseDocsFinding::new(
                &self.surface_id,
                "surface.schema_version",
                "surface schema version is unsupported",
            ));
        }
        if self.surface_id.trim().is_empty()
            || self.artifact_ref.trim().is_empty()
            || self.surface_refs.is_empty()
        {
            findings.push(ReleaseDocsFinding::new(
                &self.surface_id,
                "surface.identity",
                "surface id, artifact ref, and surface refs must be non-empty",
            ));
        }

        // Scope must be visible before the user edits or exports text.
        if !self.scope_visible_before_edit || self.active_scope_summary.trim().is_empty() {
            findings.push(ReleaseDocsFinding::new(
                &self.surface_id,
                "surface.scope_visible",
                "branch/release/channel scope must be shown before edit or export with a non-empty summary",
            ));
        }

        // The maintenance path must stay in-product.
        if !self.in_product_maintenance_path {
            findings.push(ReleaseDocsFinding::new(
                &self.surface_id,
                "surface.in_product_path",
                "release-docs maintenance must stay on an in-product path, not a browser-only or vendor-console-only path",
            ));
        }
        if self.integration_anchors.is_empty() {
            findings.push(ReleaseDocsFinding::new(
                &self.surface_id,
                "surface.integration_anchors",
                "surface must expose at least one in-product integration anchor",
            ));
        }

        // Pending-suggestion count must match its refs.
        if self.pending_suggestion_count != self.pending_suggestion_refs.len() {
            findings.push(ReleaseDocsFinding::new(
                &self.surface_id,
                "surface.suggestion_count",
                "pending suggestion count must match pending suggestion refs",
            ));
        }

        // Crossing a review/publish boundary requires an explicit scope.
        if self.publish_boundary_state.requires_scope() && !self.publish_scope.is_scoped() {
            findings.push(ReleaseDocsFinding::new(
                &self.surface_id,
                "surface.boundary_scope",
                "review or publish handoff surfaces must carry branch/release/channel scope",
            ));
        }

        // Masquerade guard: a non-installed-stable artifact must name the
        // branch/release/channel it is for, unless it is explicitly blocked for
        // lacking a scope. This stops a beta/private-branch note from floating
        // free and being mistaken for the installed stable truth.
        if !self.evidence_scope.is_installed_stable()
            && self.publish_boundary_state != DocsPublishBoundaryState::BlockedUnscoped
            && !self.publish_scope.is_scoped()
        {
            findings.push(ReleaseDocsFinding::new(
                &self.surface_id,
                "surface.unscoped_nonstable",
                "non-installed-stable docs must name the branch/release/channel they target so they cannot masquerade as installed stable",
            ));
        }

        // Stable-claim integrity: only prose that actually matches the installed
        // build may be labeled installed stable.
        if self.evidence_scope.is_installed_stable()
            && (self.source_version_badge.version_match_state != VersionMatchState::ExactBuildMatch
                || self.source_version_badge.freshness_class
                    != DocsFreshnessClass::AuthoritativeLive)
        {
            findings.push(ReleaseDocsFinding::new(
                &self.surface_id,
                "surface.stable_claim",
                "installed-stable scope requires an exact build match and authoritative-live freshness",
            ));
        }

        // Non-local surfaces must disclose the boundary before apply or export.
        if self.publish_boundary_state != DocsPublishBoundaryState::LocalOnly
            && self.publish_boundary_notes.is_empty()
        {
            findings.push(ReleaseDocsFinding::new(
                &self.surface_id,
                "surface.boundary_notes",
                "non-local surfaces must carry publish-boundary notes before apply or export",
            ));
        }

        // A blocked-unscoped surface must not expose an apply or export action.
        if self.publish_boundary_state == DocsPublishBoundaryState::BlockedUnscoped
            && self.apply_export_action.is_some()
        {
            findings.push(ReleaseDocsFinding::new(
                &self.surface_id,
                "surface.blocked_action",
                "blocked unscoped surfaces must not expose an apply or export action",
            ));
        }

        if !self.open_source_action.is_keyboard_action() {
            findings.push(ReleaseDocsFinding::new(
                &self.surface_id,
                "surface.open_source_action",
                "open-source action must be keyboard reachable and well formed",
            ));
        }
        if !self.open_diff_review_action.is_keyboard_action() {
            findings.push(ReleaseDocsFinding::new(
                &self.surface_id,
                "surface.diff_review_action",
                "diff-review action must be keyboard reachable and well formed",
            ));
        }
        if let Some(action) = &self.apply_export_action {
            if !action.is_keyboard_action() {
                findings.push(ReleaseDocsFinding::new(
                    &self.surface_id,
                    "surface.apply_export_action",
                    "apply or export action must be keyboard reachable and well formed",
                ));
            }
        }
        if !self.source_version_badge.is_complete() {
            findings.push(ReleaseDocsFinding::new(
                &self.surface_id,
                "surface.source_version_badge",
                "surface must carry a complete source/version badge",
            ));
        }

        for entry in &self.compare_history {
            entry.validate(&self.surface_id, findings);
        }
        for anchor in &self.integration_anchors {
            anchor.validate(&self.surface_id, findings);
        }
    }

    /// Validates this surface on its own, returning any findings.
    ///
    /// Cross-references to suggestion cards are validated at the contract level;
    /// this checks the surface's own integrity (scope visibility, the masquerade
    /// and stable-claim guards, counts, boundary notes, the blocked-unscoped
    /// apply guard, compare history, and integration anchors).
    pub fn validate_record(&self) -> Vec<ReleaseDocsFinding> {
        let mut findings = Vec::new();
        self.validate(&mut findings);
        findings
    }
}

/// Coverage summary for release-docs surface projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseDocsCoverage {
    /// Number of maintenance surfaces.
    pub surface_count: usize,
    /// Number of compare-history entries across all surfaces.
    pub compare_entry_count: usize,
    /// Number of integration anchors across all surfaces.
    pub integration_anchor_count: usize,
    /// Number of suggestion cards.
    pub suggestion_card_count: usize,
    /// Count by artifact-kind token.
    pub artifact_kind_counts: BTreeMap<String, usize>,
    /// Count by evidence-scope token.
    pub evidence_scope_counts: BTreeMap<String, usize>,
    /// Count by compare-kind token across all compare entries.
    pub compare_kind_counts: BTreeMap<String, usize>,
    /// Count by publish-boundary token across all surfaces.
    pub publish_boundary_counts: BTreeMap<String, usize>,
    /// Count by integration-target token across all anchors.
    pub integration_target_counts: BTreeMap<String, usize>,
}

/// Governed contract holding release-docs maintenance surfaces and the
/// suggestion cards they reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseDocsMaintenanceContract {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable contract id.
    pub contract_id: String,
    /// Stable contract version ref.
    pub contract_version_ref: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Related schema, docs, and fixture artifacts.
    pub contract_refs: BTreeMap<String, String>,
    /// Handoff banner shared by the contract surfaces.
    pub handoff_banner: DocsHandoffBanner,
    /// README/changelog/onboarding release-docs maintenance surfaces.
    pub surfaces: Vec<ReleaseDocsMaintenanceSurface>,
    /// Evidence-backed suggestion cards referenced by the surfaces.
    pub suggestion_cards: Vec<DocsSuggestionCard>,
    /// Material classes omitted from any export.
    pub omitted_material_classes: Vec<String>,
}

impl ReleaseDocsMaintenanceContract {
    /// Returns the surface with `surface_id`.
    pub fn surface(&self, surface_id: &str) -> Option<&ReleaseDocsMaintenanceSurface> {
        self.surfaces
            .iter()
            .find(|surface| surface.surface_id == surface_id)
    }

    /// Returns the suggestion card with `card_id`.
    pub fn suggestion_card(&self, card_id: &str) -> Option<&DocsSuggestionCard> {
        self.suggestion_cards
            .iter()
            .find(|card| card.card_id == card_id)
    }

    /// Projects render-ready surfaces and a coverage summary.
    pub fn surface_projection(&self) -> ReleaseDocsSurfaceProjection {
        ReleaseDocsSurfaceProjection {
            record_kind: RELEASE_DOCS_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: RELEASE_DOCS_MAINTENANCE_SCHEMA_VERSION,
            projection_id: "release-docs:readme-changelog:surface-projection:v1".to_owned(),
            generated_at: self.generated_at.clone(),
            contract_id: self.contract_id.clone(),
            contract_version_ref: self.contract_version_ref.clone(),
            handoff_banner: self.handoff_banner.clone(),
            surfaces: self.surfaces.clone(),
            suggestion_cards: self.suggestion_cards.clone(),
            coverage: self.coverage(),
        }
    }

    /// Projects a metadata-only, screenshot-free review packet.
    ///
    /// The packet preserves the pending suggestions, compare history, and
    /// publish/export boundaries so they stay inspectable after the user leaves
    /// the surface.
    pub fn review_packet(
        &self,
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> ReleaseDocsReviewPacket {
        ReleaseDocsReviewPacket {
            record_kind: RELEASE_DOCS_REVIEW_PACKET_RECORD_KIND.to_owned(),
            schema_version: RELEASE_DOCS_MAINTENANCE_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            generated_at: generated_at.into(),
            source_contract_id: self.contract_id.clone(),
            contract_version_ref: self.contract_version_ref.clone(),
            handoff_banner: self.handoff_banner.clone(),
            surfaces: self.surfaces.clone(),
            suggestion_cards: self.suggestion_cards.clone(),
            omitted_material_classes: self.omitted_material_classes.clone(),
            raw_document_bodies_exported: false,
        }
    }

    /// Validates the contract and every nested record.
    pub fn validate(&self) -> Vec<ReleaseDocsFinding> {
        let mut findings = Vec::new();
        if self.record_kind != RELEASE_DOCS_MAINTENANCE_CONTRACT_RECORD_KIND {
            findings.push(ReleaseDocsFinding::new(
                &self.contract_id,
                "contract.record_kind",
                "contract record_kind is unsupported",
            ));
        }
        if self.schema_version != RELEASE_DOCS_MAINTENANCE_SCHEMA_VERSION {
            findings.push(ReleaseDocsFinding::new(
                &self.contract_id,
                "contract.schema_version",
                "contract schema version is unsupported",
            ));
        }
        if self.contract_id.trim().is_empty()
            || self.contract_version_ref.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(ReleaseDocsFinding::new(
                &self.contract_id,
                "contract.identity",
                "contract id, version ref, and generated_at must be non-empty",
            ));
        }
        if self.omitted_material_classes.is_empty() {
            findings.push(ReleaseDocsFinding::new(
                &self.contract_id,
                "contract.omitted_classes",
                "contract must disclose omitted material classes",
            ));
        }
        if self.surfaces.is_empty() {
            findings.push(ReleaseDocsFinding::new(
                &self.contract_id,
                "contract.coverage",
                "contract must cover at least one release-docs maintenance surface",
            ));
        }

        self.validate_handoff_banner(&mut findings);

        for surface in &self.surfaces {
            surface.validate(&mut findings);
            if surface.pending_suggestion_count != surface.pending_suggestion_refs.len() {
                continue;
            }
            for card_ref in &surface.pending_suggestion_refs {
                if self.suggestion_card(card_ref).is_none() {
                    findings.push(ReleaseDocsFinding::new(
                        &surface.surface_id,
                        "surface.unknown_suggestion_ref",
                        "surface references an unknown suggestion card",
                    ));
                }
            }
        }

        // Reuse the maintenance suggestion-card invariants so pending
        // suggestions stay diff-first and evidence-backed.
        for card in &self.suggestion_cards {
            for maintenance_finding in card.validate_record() {
                findings.push(ReleaseDocsFinding::from_maintenance(maintenance_finding));
            }
        }

        self.assert_required_coverage(&mut findings);
        findings
    }

    fn validate_handoff_banner(&self, findings: &mut Vec<ReleaseDocsFinding>) {
        if !self.handoff_banner.screenshot_free_review {
            findings.push(ReleaseDocsFinding::new(
                &self.handoff_banner.banner_id,
                "handoff_banner.screenshot_free",
                "handoff banner must support screenshot-free review",
            ));
        }
        if self.handoff_banner.publish_boundary_state.requires_scope()
            && !self.handoff_banner.publish_scope.is_scoped()
        {
            findings.push(ReleaseDocsFinding::new(
                &self.handoff_banner.banner_id,
                "handoff_banner.publish_scope",
                "review or publish handoff banners must carry branch/release/channel scope",
            ));
        }
    }

    fn assert_required_coverage(&self, findings: &mut Vec<ReleaseDocsFinding>) {
        for scope in [
            ReleaseDocsEvidenceScope::LocalDraft,
            ReleaseDocsEvidenceScope::PrivateBranch,
            ReleaseDocsEvidenceScope::SharedReview,
            ReleaseDocsEvidenceScope::SharedPrerelease,
            ReleaseDocsEvidenceScope::InstalledStable,
        ] {
            if !self
                .surfaces
                .iter()
                .any(|surface| surface.evidence_scope == scope)
            {
                findings.push(ReleaseDocsFinding::new(
                    &self.contract_id,
                    "contract.evidence_scope_coverage",
                    format!("contract must exercise evidence scope {}", scope.as_str()),
                ));
            }
        }
        for boundary in [
            DocsPublishBoundaryState::LocalOnly,
            DocsPublishBoundaryState::ReviewHandoffScoped,
            DocsPublishBoundaryState::PublishHandoffScoped,
            DocsPublishBoundaryState::BlockedUnscoped,
        ] {
            if !self
                .surfaces
                .iter()
                .any(|surface| surface.publish_boundary_state == boundary)
            {
                findings.push(ReleaseDocsFinding::new(
                    &self.contract_id,
                    "contract.publish_boundary_coverage",
                    format!(
                        "contract must exercise publish boundary {}",
                        boundary.as_str()
                    ),
                ));
            }
        }
        for kind in [
            ReleaseDocsCompareKind::WorkingVsInstalled,
            ReleaseDocsCompareKind::BranchVsRelease,
            ReleaseDocsCompareKind::RevisionVsRevision,
            ReleaseDocsCompareKind::ChannelVsChannel,
        ] {
            if !self.surfaces.iter().any(|surface| {
                surface
                    .compare_history
                    .iter()
                    .any(|entry| entry.compare_kind == kind)
            }) {
                findings.push(ReleaseDocsFinding::new(
                    &self.contract_id,
                    "contract.compare_kind_coverage",
                    format!("contract must exercise compare kind {}", kind.as_str()),
                ));
            }
        }
        for target in [
            ReleaseDocsIntegrationTarget::ReleaseCenter,
            ReleaseDocsIntegrationTarget::HelpBrowser,
            ReleaseDocsIntegrationTarget::AboutPanel,
            ReleaseDocsIntegrationTarget::SupportExport,
        ] {
            if !self.surfaces.iter().any(|surface| {
                surface
                    .integration_anchors
                    .iter()
                    .any(|anchor| anchor.target == target)
            }) {
                findings.push(ReleaseDocsFinding::new(
                    &self.contract_id,
                    "contract.integration_target_coverage",
                    format!("contract must integrate the {} surface", target.as_str()),
                ));
            }
        }
        for kind in [DocsArtifactKind::Readme, DocsArtifactKind::Changelog] {
            if !self
                .surfaces
                .iter()
                .any(|surface| surface.artifact_kind == kind)
            {
                findings.push(ReleaseDocsFinding::new(
                    &self.contract_id,
                    "contract.artifact_kind_coverage",
                    format!("contract must cover artifact kind {}", kind.as_str()),
                ));
            }
        }
    }

    fn coverage(&self) -> ReleaseDocsCoverage {
        let mut artifact_kind_counts = BTreeMap::new();
        let mut evidence_scope_counts = BTreeMap::new();
        let mut compare_kind_counts = BTreeMap::new();
        let mut publish_boundary_counts = BTreeMap::new();
        let mut integration_target_counts = BTreeMap::new();
        let mut compare_entry_count = 0;
        let mut integration_anchor_count = 0;

        for surface in &self.surfaces {
            *artifact_kind_counts
                .entry(surface.artifact_kind.as_str().to_owned())
                .or_insert(0) += 1;
            *evidence_scope_counts
                .entry(surface.evidence_scope.as_str().to_owned())
                .or_insert(0) += 1;
            *publish_boundary_counts
                .entry(surface.publish_boundary_state.as_str().to_owned())
                .or_insert(0) += 1;
            for entry in &surface.compare_history {
                compare_entry_count += 1;
                *compare_kind_counts
                    .entry(entry.compare_kind.as_str().to_owned())
                    .or_insert(0) += 1;
            }
            for anchor in &surface.integration_anchors {
                integration_anchor_count += 1;
                *integration_target_counts
                    .entry(anchor.target.as_str().to_owned())
                    .or_insert(0) += 1;
            }
        }

        ReleaseDocsCoverage {
            surface_count: self.surfaces.len(),
            compare_entry_count,
            integration_anchor_count,
            suggestion_card_count: self.suggestion_cards.len(),
            artifact_kind_counts,
            evidence_scope_counts,
            compare_kind_counts,
            publish_boundary_counts,
            integration_target_counts,
        }
    }
}

/// Surface projection for release-docs maintenance records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseDocsSurfaceProjection {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source contract id.
    pub contract_id: String,
    /// Source contract version ref.
    pub contract_version_ref: String,
    /// Handoff banner rendered with the surfaces.
    pub handoff_banner: DocsHandoffBanner,
    /// README/changelog/onboarding release-docs maintenance surfaces.
    pub surfaces: Vec<ReleaseDocsMaintenanceSurface>,
    /// Evidence-backed suggestion cards.
    pub suggestion_cards: Vec<DocsSuggestionCard>,
    /// Coverage summary for review and release packets.
    pub coverage: ReleaseDocsCoverage,
}

/// Metadata-only, screenshot-free review packet for release-docs maintenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseDocsReviewPacket {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Export generation timestamp.
    pub generated_at: String,
    /// Source contract id.
    pub source_contract_id: String,
    /// Source contract version ref.
    pub contract_version_ref: String,
    /// Handoff banner preserving local-only versus publish-boundary state.
    pub handoff_banner: DocsHandoffBanner,
    /// README/changelog/onboarding release-docs maintenance surfaces.
    pub surfaces: Vec<ReleaseDocsMaintenanceSurface>,
    /// Evidence-backed suggestion cards.
    pub suggestion_cards: Vec<DocsSuggestionCard>,
    /// Material classes omitted from the export.
    pub omitted_material_classes: Vec<String>,
    /// Whether raw document bodies were exported (must be false).
    pub raw_document_bodies_exported: bool,
}

impl ReleaseDocsReviewPacket {
    /// Validates packet reconstruction against `contract`.
    pub fn validate_against_contract(
        &self,
        contract: &ReleaseDocsMaintenanceContract,
    ) -> Result<(), Vec<ReleaseDocsFinding>> {
        let mut findings = Vec::new();
        if self.record_kind != RELEASE_DOCS_REVIEW_PACKET_RECORD_KIND {
            findings.push(ReleaseDocsFinding::new(
                &self.packet_id,
                "review_packet.record_kind",
                "review packet record_kind is unsupported",
            ));
        }
        if self.schema_version != RELEASE_DOCS_MAINTENANCE_SCHEMA_VERSION {
            findings.push(ReleaseDocsFinding::new(
                &self.packet_id,
                "review_packet.schema_version",
                "review packet schema version is unsupported",
            ));
        }
        if self.packet_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            findings.push(ReleaseDocsFinding::new(
                &self.packet_id,
                "review_packet.identity",
                "review packet id and generated_at must be non-empty",
            ));
        }
        if self.source_contract_id != contract.contract_id
            || self.contract_version_ref != contract.contract_version_ref
        {
            findings.push(ReleaseDocsFinding::new(
                &self.packet_id,
                "review_packet.contract_ref",
                "review packet contract refs drifted",
            ));
        }
        if self.raw_document_bodies_exported || self.omitted_material_classes.is_empty() {
            findings.push(ReleaseDocsFinding::new(
                &self.packet_id,
                "review_packet.raw_bodies",
                "review packet must omit raw document bodies and disclose omitted classes",
            ));
        }
        if self.surfaces != contract.surfaces
            || self.suggestion_cards != contract.suggestion_cards
            || self.handoff_banner != contract.handoff_banner
        {
            findings.push(ReleaseDocsFinding::new(
                &self.packet_id,
                "review_packet.row_drift",
                "review packet drifted from contract records",
            ));
        }
        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }

    /// Deterministic JSON serialization for support/export fixtures.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("release-docs review packet serializes")
    }
}

/// Validation finding for release-docs maintenance records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseDocsFinding {
    /// Surface or object that failed validation.
    pub row_ref: String,
    /// Stable validation check id.
    pub check_id: String,
    /// Reviewable validation message.
    pub message: String,
}

impl ReleaseDocsFinding {
    fn new(
        row_ref: impl Into<String>,
        check_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            row_ref: row_ref.into(),
            check_id: check_id.into(),
            message: message.into(),
        }
    }

    fn from_maintenance(finding: DocsMaintenanceFinding) -> Self {
        Self {
            row_ref: finding.row_ref,
            check_id: finding.check_id,
            message: finding.message,
        }
    }
}

/// Lightweight integrity helpers shared by release-docs records.
trait ReleaseDocsActionExt {
    /// Returns true when the action is keyboard reachable and well formed.
    fn is_keyboard_action(&self) -> bool;
}

impl ReleaseDocsActionExt for DocsMaintenanceAction {
    fn is_keyboard_action(&self) -> bool {
        !self.action_ref.trim().is_empty()
            && !self.action_label.trim().is_empty()
            && self.keyboard_reachable
    }
}

trait ReleaseDocsBadgeExt {
    /// Returns true when every badge field is present.
    fn is_complete(&self) -> bool;
}

impl ReleaseDocsBadgeExt for DocsSourceVersionBadge {
    fn is_complete(&self) -> bool {
        ![
            &self.source_class_token,
            &self.source_pack_ref,
            &self.source_revision_ref,
            &self.version_or_revision_ref,
            &self.source_build_at,
            &self.running_build_identity_ref,
        ]
        .iter()
        .any(|value| value.trim().is_empty())
    }
}

fn action(action_ref: &str, action_label: &str) -> DocsMaintenanceAction {
    DocsMaintenanceAction::new(action_ref, action_label)
}

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn installed_badge(source_pack_ref: &str) -> DocsSourceVersionBadge {
    DocsSourceVersionBadge {
        source_class_token: "project_docs".to_owned(),
        source_pack_ref: source_pack_ref.to_owned(),
        source_revision_ref: format!("docs-source-rev:{source_pack_ref}:2026.06.01-01"),
        version_or_revision_ref: "rev:aureline-docs:2026.06.01-stable".to_owned(),
        source_build_at: "2026-06-01T14:30:00Z".to_owned(),
        running_build_identity_ref: "build:aureline:2026.06.01-stable".to_owned(),
        freshness_class: DocsFreshnessClass::AuthoritativeLive,
        version_match_state: VersionMatchState::ExactBuildMatch,
    }
}

fn prerelease_badge(source_pack_ref: &str) -> DocsSourceVersionBadge {
    DocsSourceVersionBadge {
        source_class_token: "project_docs".to_owned(),
        source_pack_ref: source_pack_ref.to_owned(),
        source_revision_ref: format!("docs-source-rev:{source_pack_ref}:2026.06.01-beta"),
        version_or_revision_ref: "rev:aureline-docs:2026.06.01-beta".to_owned(),
        source_build_at: "2026-06-01T14:30:00Z".to_owned(),
        running_build_identity_ref: "build:aureline:2026.06.01-beta".to_owned(),
        freshness_class: DocsFreshnessClass::WarmCached,
        version_match_state: VersionMatchState::CompatibleMinorDrift,
    }
}

fn reopen_action(suffix: &str) -> DocsMaintenanceAction {
    action(
        &format!("action:release-docs.reopen-compare:{suffix}"),
        REOPEN_RELEASE_DOCS_COMPARE_ACTION_LABEL,
    )
}

fn integration_anchor(
    target: ReleaseDocsIntegrationTarget,
    anchor_ref: &str,
    action_suffix: &str,
    action_label: &str,
) -> ReleaseDocsIntegrationAnchor {
    ReleaseDocsIntegrationAnchor {
        target,
        anchor_ref: anchor_ref.to_owned(),
        open_action: action(
            &format!("action:release-docs.open-integration:{action_suffix}"),
            action_label,
        ),
    }
}

/// Seed describing one pending suggestion card the surfaces reference.
struct CardSeed {
    card_id: &'static str,
    artifact_kind: DocsArtifactKind,
    target_artifact_ref: &'static str,
    trigger: DocsSuggestionTrigger,
    trigger_source_ref: &'static str,
    confidence_class: CitationConfidenceClass,
    apply_posture: crate::maintenance::DocsSuggestionApplyPosture,
    review_diff_ref: Option<&'static str>,
    evidence: &'static [&'static str],
    publish_boundary_state: DocsPublishBoundaryState,
}

fn suggestion_card(seed: CardSeed) -> DocsSuggestionCard {
    let card_id = seed.card_id;
    DocsSuggestionCard {
        record_kind: DOCS_SUGGESTION_CARD_RECORD_KIND.to_owned(),
        schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
        card_id: card_id.to_owned(),
        suggestion_ref: format!("docs-suggestion:{card_id}"),
        artifact_kind: seed.artifact_kind,
        target_artifact_ref: seed.target_artifact_ref.to_owned(),
        target_section_ref: None,
        trigger: seed.trigger,
        trigger_source_ref: seed.trigger_source_ref.to_owned(),
        confidence_class: seed.confidence_class,
        freshness_class: DocsFreshnessClass::WarmCached,
        version_match_state: VersionMatchState::CompatibleMinorDrift,
        apply_posture: seed.apply_posture,
        review_diff_ref: seed.review_diff_ref.map(str::to_owned),
        evidence_refs: refs(seed.evidence),
        open_evidence_action: action(
            &format!("action:release-docs.open-evidence:{card_id}"),
            "Open evidence",
        ),
        open_review_diff_action: seed.review_diff_ref.map(|_| {
            action(
                &format!("action:release-docs.review-diff:{card_id}"),
                REVIEW_RELEASE_DOCS_DIFF_ACTION_LABEL,
            )
        }),
        generated_text_disclosed: true,
        silent_rewrite_blocked: true,
        publish_boundary_state: seed.publish_boundary_state,
        surface_refs: refs(&["surface:release_docs:suggestion_cards"]),
    }
}

/// Returns the seeded README/changelog release-docs maintenance contract.
pub fn seeded_release_docs_maintenance_contract() -> ReleaseDocsMaintenanceContract {
    use crate::maintenance::DocsSuggestionApplyPosture;

    let handoff_banner = DocsHandoffBanner {
        banner_id: "release-docs-handoff-banner:readme-changelog:beta:v1".to_owned(),
        publish_boundary_state: DocsPublishBoundaryState::ReviewHandoffScoped,
        publish_scope: DocsPublishScope {
            branch_scope: Some("branch:release/beta-2".to_owned()),
            release_scope: Some("release:beta-2".to_owned()),
            channel_scope: Some("beta".to_owned()),
        },
        local_only_note: None,
        publish_handoff_note: Some(
            "Release-docs review packet — beta scope; not published to stable docs.".to_owned(),
        ),
        screenshot_free_review: true,
    };

    let suggestion_cards = vec![
        suggestion_card(CardSeed {
            card_id: "release-docs-suggestion:readme:command-rename",
            artifact_kind: DocsArtifactKind::Readme,
            target_artifact_ref: "docs-artifact:readme:root",
            trigger: DocsSuggestionTrigger::CodeDiff,
            trigger_source_ref: "diff:commands.rename-open-folder",
            confidence_class: CitationConfidenceClass::EvidenceBacked,
            apply_posture: DocsSuggestionApplyPosture::ApplyAfterReview,
            review_diff_ref: Some("diff:release-docs-suggestion:readme.command-rename"),
            evidence: &[
                "evidence:command-descriptor:open-folder",
                "evidence:diff:commands.rename-open-folder",
            ],
            publish_boundary_state: DocsPublishBoundaryState::LocalOnly,
        }),
        suggestion_card(CardSeed {
            card_id: "release-docs-suggestion:changelog:support-window",
            artifact_kind: DocsArtifactKind::Changelog,
            target_artifact_ref: "docs-artifact:changelog:beta",
            trigger: DocsSuggestionTrigger::ReleaseNoteDrift,
            trigger_source_ref: "claim-row:beta-support-window",
            confidence_class: CitationConfidenceClass::EvidenceBacked,
            apply_posture: DocsSuggestionApplyPosture::ApplyAfterReview,
            review_diff_ref: Some("diff:release-docs-suggestion:changelog.support-window"),
            evidence: &[
                "evidence:claim-row:beta-support-window",
                "evidence:compatibility-row:tsjs-launch",
            ],
            publish_boundary_state: DocsPublishBoundaryState::PublishHandoffScoped,
        }),
        suggestion_card(CardSeed {
            card_id: "release-docs-suggestion:onboarding:reviewer-note",
            artifact_kind: DocsArtifactKind::OnboardingNote,
            target_artifact_ref: "docs-artifact:onboarding-note:first-run",
            trigger: DocsSuggestionTrigger::HumanNote,
            trigger_source_ref: "human-note:onboarding.reviewer-01",
            confidence_class: CitationConfidenceClass::Inferred,
            apply_posture: DocsSuggestionApplyPosture::ReviewDiffOnly,
            review_diff_ref: Some("diff:release-docs-suggestion:onboarding.reviewer-note"),
            evidence: &["evidence:human-note:onboarding.reviewer-01"],
            publish_boundary_state: DocsPublishBoundaryState::ReviewHandoffScoped,
        }),
    ];

    let surfaces = vec![
        // Installed stable README — local edit against installed stable truth.
        ReleaseDocsMaintenanceSurface {
            record_kind: RELEASE_DOCS_MAINTENANCE_SURFACE_RECORD_KIND.to_owned(),
            schema_version: RELEASE_DOCS_MAINTENANCE_SCHEMA_VERSION,
            surface_id: "release-docs-surface:readme:installed-stable".to_owned(),
            artifact_kind: DocsArtifactKind::Readme,
            artifact_ref: "docs-artifact:readme:root".to_owned(),
            audience_scope: DocsAudienceScope::PublicReader,
            publish_scope: DocsPublishScope::default(),
            publish_boundary_state: DocsPublishBoundaryState::LocalOnly,
            evidence_scope: ReleaseDocsEvidenceScope::InstalledStable,
            scope_visible_before_edit: true,
            active_scope_summary:
                "Installed stable README — matches the running build; edits stay local until reviewed."
                    .to_owned(),
            in_product_maintenance_path: true,
            source_version_badge: installed_badge("project:readme"),
            pending_suggestion_count: 1,
            pending_suggestion_refs: refs(&["release-docs-suggestion:readme:command-rename"]),
            compare_history: vec![ReleaseDocsCompareEntry {
                compare_id: "release-docs-compare:readme:working-vs-installed".to_owned(),
                compare_kind: ReleaseDocsCompareKind::WorkingVsInstalled,
                base_ref: "rev:aureline-docs:2026.06.01-stable".to_owned(),
                base_label: "Installed stable".to_owned(),
                target_ref: "rev:aureline-docs:working".to_owned(),
                target_label: "Local working copy".to_owned(),
                compared_at: "2026-06-01T14:45:00Z".to_owned(),
                diff_summary_ref: "diff-summary:readme:working-vs-installed".to_owned(),
                changed_section_refs: refs(&["section:readme#commands"]),
                evidence_scope: ReleaseDocsEvidenceScope::InstalledStable,
                reopenable: true,
                reopen_action: reopen_action("readme.working-vs-installed"),
            }],
            integration_anchors: vec![
                integration_anchor(
                    ReleaseDocsIntegrationTarget::HelpBrowser,
                    "surface:help_browser:readme",
                    "readme.help",
                    "Open in help browser",
                ),
                integration_anchor(
                    ReleaseDocsIntegrationTarget::AboutPanel,
                    "surface:about_panel:readme",
                    "readme.about",
                    "Open in About",
                ),
            ],
            publish_boundary_notes: Vec::new(),
            open_source_action: action(
                "action:release-docs.open-source:readme",
                OPEN_RELEASE_DOCS_SOURCE_ACTION_LABEL,
            ),
            open_diff_review_action: action(
                "action:release-docs.review-diff:readme",
                REVIEW_RELEASE_DOCS_DIFF_ACTION_LABEL,
            ),
            apply_export_action: Some(action(
                "action:release-docs.apply-local:readme",
                "Apply locally",
            )),
            surface_refs: refs(&["surface:release_docs:surfaces"]),
        },
        // Beta changelog — prerelease channel, publish handoff scoped.
        ReleaseDocsMaintenanceSurface {
            record_kind: RELEASE_DOCS_MAINTENANCE_SURFACE_RECORD_KIND.to_owned(),
            schema_version: RELEASE_DOCS_MAINTENANCE_SCHEMA_VERSION,
            surface_id: "release-docs-surface:changelog:beta".to_owned(),
            artifact_kind: DocsArtifactKind::Changelog,
            artifact_ref: "docs-artifact:changelog:beta".to_owned(),
            audience_scope: DocsAudienceScope::ReleaseManager,
            publish_scope: DocsPublishScope {
                branch_scope: Some("branch:release/beta-2".to_owned()),
                release_scope: Some("release:beta-2".to_owned()),
                channel_scope: Some("beta".to_owned()),
            },
            publish_boundary_state: DocsPublishBoundaryState::PublishHandoffScoped,
            evidence_scope: ReleaseDocsEvidenceScope::SharedPrerelease,
            scope_visible_before_edit: true,
            active_scope_summary:
                "Beta changelog — release beta-2 on the beta channel; not the installed stable changelog."
                    .to_owned(),
            in_product_maintenance_path: true,
            source_version_badge: prerelease_badge("project:changelog"),
            pending_suggestion_count: 1,
            pending_suggestion_refs: refs(&["release-docs-suggestion:changelog:support-window"]),
            compare_history: vec![ReleaseDocsCompareEntry {
                compare_id: "release-docs-compare:changelog:channel".to_owned(),
                compare_kind: ReleaseDocsCompareKind::ChannelVsChannel,
                base_ref: "channel:stable".to_owned(),
                base_label: "Stable channel".to_owned(),
                target_ref: "channel:beta".to_owned(),
                target_label: "Beta channel".to_owned(),
                compared_at: "2026-06-01T14:46:00Z".to_owned(),
                diff_summary_ref: "diff-summary:changelog:beta-vs-stable".to_owned(),
                changed_section_refs: refs(&[
                    "section:changelog#unreleased",
                    "section:changelog#support",
                ]),
                evidence_scope: ReleaseDocsEvidenceScope::SharedPrerelease,
                reopenable: true,
                reopen_action: reopen_action("changelog.channel"),
            }],
            integration_anchors: vec![
                integration_anchor(
                    ReleaseDocsIntegrationTarget::ReleaseCenter,
                    "surface:release_center:changelog",
                    "changelog.release-center",
                    "Open in release center",
                ),
                integration_anchor(
                    ReleaseDocsIntegrationTarget::SupportExport,
                    "surface:support_export:changelog",
                    "changelog.support-export",
                    "Open support export",
                ),
            ],
            publish_boundary_notes: refs(&[
                "Beta changelog — scoped to the beta channel; not stable docs.",
                "Publish handoff required before this leaves review.",
            ]),
            open_source_action: action(
                "action:release-docs.open-source:changelog",
                OPEN_RELEASE_DOCS_SOURCE_ACTION_LABEL,
            ),
            open_diff_review_action: action(
                "action:release-docs.review-diff:changelog",
                REVIEW_RELEASE_DOCS_DIFF_ACTION_LABEL,
            ),
            apply_export_action: Some(action(
                "action:release-docs.export-handoff:changelog",
                "Export publish handoff",
            )),
            surface_refs: refs(&["surface:release_docs:surfaces"]),
        },
        // Onboarding note — shared for review, review handoff scoped.
        ReleaseDocsMaintenanceSurface {
            record_kind: RELEASE_DOCS_MAINTENANCE_SURFACE_RECORD_KIND.to_owned(),
            schema_version: RELEASE_DOCS_MAINTENANCE_SCHEMA_VERSION,
            surface_id: "release-docs-surface:onboarding:shared-review".to_owned(),
            artifact_kind: DocsArtifactKind::OnboardingNote,
            artifact_ref: "docs-artifact:onboarding-note:first-run".to_owned(),
            audience_scope: DocsAudienceScope::EndUser,
            publish_scope: DocsPublishScope {
                branch_scope: Some("branch:docs/onboarding-beta".to_owned()),
                release_scope: None,
                channel_scope: Some("beta".to_owned()),
            },
            publish_boundary_state: DocsPublishBoundaryState::ReviewHandoffScoped,
            evidence_scope: ReleaseDocsEvidenceScope::SharedReview,
            scope_visible_before_edit: true,
            active_scope_summary:
                "Onboarding note — staged for review on the beta onboarding branch; stays inside review scope."
                    .to_owned(),
            in_product_maintenance_path: true,
            source_version_badge: prerelease_badge("project:onboarding"),
            pending_suggestion_count: 1,
            pending_suggestion_refs: refs(&["release-docs-suggestion:onboarding:reviewer-note"]),
            compare_history: vec![ReleaseDocsCompareEntry {
                compare_id: "release-docs-compare:onboarding:branch-vs-release".to_owned(),
                compare_kind: ReleaseDocsCompareKind::BranchVsRelease,
                base_ref: "release:beta-1".to_owned(),
                base_label: "Release beta-1".to_owned(),
                target_ref: "branch:docs/onboarding-beta".to_owned(),
                target_label: "Onboarding beta branch".to_owned(),
                compared_at: "2026-06-01T14:47:00Z".to_owned(),
                diff_summary_ref: "diff-summary:onboarding:branch-vs-release".to_owned(),
                changed_section_refs: refs(&["section:onboarding#first-run"]),
                evidence_scope: ReleaseDocsEvidenceScope::SharedReview,
                reopenable: true,
                reopen_action: reopen_action("onboarding.branch-vs-release"),
            }],
            integration_anchors: vec![integration_anchor(
                ReleaseDocsIntegrationTarget::HelpBrowser,
                "surface:help_browser:onboarding",
                "onboarding.help",
                "Open in help browser",
            )],
            publish_boundary_notes: refs(&[
                "Onboarding note staged for review; stays inside review scope.",
            ]),
            open_source_action: action(
                "action:release-docs.open-source:onboarding",
                OPEN_RELEASE_DOCS_SOURCE_ACTION_LABEL,
            ),
            open_diff_review_action: action(
                "action:release-docs.review-diff:onboarding",
                REVIEW_RELEASE_DOCS_DIFF_ACTION_LABEL,
            ),
            apply_export_action: Some(action(
                "action:release-docs.export-review:onboarding",
                "Export review packet",
            )),
            surface_refs: refs(&["surface:release_docs:surfaces"]),
        },
        // Release notes — private branch draft, publish blocked for lacking scope.
        ReleaseDocsMaintenanceSurface {
            record_kind: RELEASE_DOCS_MAINTENANCE_SURFACE_RECORD_KIND.to_owned(),
            schema_version: RELEASE_DOCS_MAINTENANCE_SCHEMA_VERSION,
            surface_id: "release-docs-surface:release-notes:blocked".to_owned(),
            artifact_kind: DocsArtifactKind::ReleaseNotes,
            artifact_ref: "docs-artifact:release-notes:next".to_owned(),
            audience_scope: DocsAudienceScope::PublicReader,
            publish_scope: DocsPublishScope::default(),
            publish_boundary_state: DocsPublishBoundaryState::BlockedUnscoped,
            evidence_scope: ReleaseDocsEvidenceScope::PrivateBranch,
            scope_visible_before_edit: true,
            active_scope_summary:
                "Release notes draft — on a private branch with no release/channel scope; publish is blocked."
                    .to_owned(),
            in_product_maintenance_path: true,
            source_version_badge: prerelease_badge("project:release-notes"),
            pending_suggestion_count: 0,
            pending_suggestion_refs: Vec::new(),
            compare_history: vec![ReleaseDocsCompareEntry {
                compare_id: "release-docs-compare:release-notes:revision".to_owned(),
                compare_kind: ReleaseDocsCompareKind::RevisionVsRevision,
                base_ref: "rev:release-notes:2026.05.20".to_owned(),
                base_label: "Revision 2026.05.20".to_owned(),
                target_ref: "rev:release-notes:2026.06.01".to_owned(),
                target_label: "Revision 2026.06.01".to_owned(),
                compared_at: "2026-06-01T14:48:00Z".to_owned(),
                diff_summary_ref: "diff-summary:release-notes:revision".to_owned(),
                changed_section_refs: refs(&["section:release-notes#highlights"]),
                evidence_scope: ReleaseDocsEvidenceScope::PrivateBranch,
                reopenable: true,
                reopen_action: reopen_action("release-notes.revision"),
            }],
            integration_anchors: vec![integration_anchor(
                ReleaseDocsIntegrationTarget::SupportExport,
                "surface:support_export:release-notes",
                "release-notes.support-export",
                "Open support export",
            )],
            publish_boundary_notes: refs(&[
                "Publish blocked: no branch/release/channel scope was provided.",
                "Add a release/channel scope to enable a publish handoff.",
            ]),
            open_source_action: action(
                "action:release-docs.open-source:release-notes",
                OPEN_RELEASE_DOCS_SOURCE_ACTION_LABEL,
            ),
            open_diff_review_action: action(
                "action:release-docs.review-diff:release-notes",
                REVIEW_RELEASE_DOCS_DIFF_ACTION_LABEL,
            ),
            apply_export_action: None,
            surface_refs: refs(&["surface:release_docs:surfaces"]),
        },
        // README draft for the next beta — local draft, explicitly scoped to next.
        ReleaseDocsMaintenanceSurface {
            record_kind: RELEASE_DOCS_MAINTENANCE_SURFACE_RECORD_KIND.to_owned(),
            schema_version: RELEASE_DOCS_MAINTENANCE_SCHEMA_VERSION,
            surface_id: "release-docs-surface:readme:next-draft".to_owned(),
            artifact_kind: DocsArtifactKind::Readme,
            artifact_ref: "docs-artifact:readme:next".to_owned(),
            audience_scope: DocsAudienceScope::Maintainer,
            publish_scope: DocsPublishScope {
                branch_scope: Some("branch:docs/readme-next".to_owned()),
                release_scope: None,
                channel_scope: Some("next".to_owned()),
            },
            publish_boundary_state: DocsPublishBoundaryState::LocalOnly,
            evidence_scope: ReleaseDocsEvidenceScope::LocalDraft,
            scope_visible_before_edit: true,
            active_scope_summary:
                "README draft for the next channel — a local draft; not the installed stable README."
                    .to_owned(),
            in_product_maintenance_path: true,
            source_version_badge: prerelease_badge("project:readme-next"),
            pending_suggestion_count: 0,
            pending_suggestion_refs: Vec::new(),
            compare_history: vec![ReleaseDocsCompareEntry {
                compare_id: "release-docs-compare:readme-next:working-vs-installed".to_owned(),
                compare_kind: ReleaseDocsCompareKind::WorkingVsInstalled,
                base_ref: "rev:aureline-docs:2026.06.01-stable".to_owned(),
                base_label: "Installed stable".to_owned(),
                target_ref: "branch:docs/readme-next".to_owned(),
                target_label: "Next-channel draft".to_owned(),
                compared_at: "2026-06-01T14:49:00Z".to_owned(),
                diff_summary_ref: "diff-summary:readme-next:working-vs-installed".to_owned(),
                changed_section_refs: refs(&["section:readme#install"]),
                evidence_scope: ReleaseDocsEvidenceScope::LocalDraft,
                reopenable: true,
                reopen_action: reopen_action("readme-next.working-vs-installed"),
            }],
            integration_anchors: vec![integration_anchor(
                ReleaseDocsIntegrationTarget::ReleaseCenter,
                "surface:release_center:readme-next",
                "readme-next.release-center",
                "Open in release center",
            )],
            publish_boundary_notes: Vec::new(),
            open_source_action: action(
                "action:release-docs.open-source:readme-next",
                OPEN_RELEASE_DOCS_SOURCE_ACTION_LABEL,
            ),
            open_diff_review_action: action(
                "action:release-docs.review-diff:readme-next",
                REVIEW_RELEASE_DOCS_DIFF_ACTION_LABEL,
            ),
            apply_export_action: Some(action(
                "action:release-docs.apply-draft:readme-next",
                "Apply to draft",
            )),
            surface_refs: refs(&["surface:release_docs:surfaces"]),
        },
    ];

    ReleaseDocsMaintenanceContract {
        record_kind: RELEASE_DOCS_MAINTENANCE_CONTRACT_RECORD_KIND.to_owned(),
        schema_version: RELEASE_DOCS_MAINTENANCE_SCHEMA_VERSION,
        contract_id: RELEASE_DOCS_MAINTENANCE_CONTRACT_ID.to_owned(),
        contract_version_ref: RELEASE_DOCS_MAINTENANCE_VERSION_REF.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        contract_refs: BTreeMap::from([
            (
                "surface_schema".to_owned(),
                RELEASE_DOCS_MAINTENANCE_SCHEMA_REF.to_owned(),
            ),
            (
                "help_doc".to_owned(),
                RELEASE_DOCS_MAINTENANCE_DOC_REF.to_owned(),
            ),
            (
                "fixtures".to_owned(),
                RELEASE_DOCS_MAINTENANCE_FIXTURE_DIR.to_owned(),
            ),
        ]),
        handoff_banner,
        surfaces,
        suggestion_cards,
        omitted_material_classes: refs(&[
            "raw_document_body",
            "rendered_html",
            "raw_source_file",
            "raw_diff_body",
            "raw_docs_url",
            "private_workspace_path",
            "account_identifier",
        ]),
    }
}

/// Returns the seeded release-docs maintenance surface projection.
pub fn seeded_release_docs_surface_projection() -> ReleaseDocsSurfaceProjection {
    seeded_release_docs_maintenance_contract().surface_projection()
}

/// Returns the seeded release-docs maintenance review packet.
pub fn seeded_release_docs_review_packet() -> ReleaseDocsReviewPacket {
    seeded_release_docs_maintenance_contract().review_packet(
        "release-docs-review-packet:readme-changelog:001",
        GENERATED_AT,
    )
}

/// Validates all seeded release-docs maintenance records.
pub fn validate_seeded_release_docs_maintenance() -> Result<(), Vec<ReleaseDocsFinding>> {
    let contract = seeded_release_docs_maintenance_contract();
    let packet = contract.review_packet(
        "release-docs-review-packet:readme-changelog:001",
        GENERATED_AT,
    );
    let mut findings = contract.validate();
    if let Err(mut packet_findings) = packet.validate_against_contract(&contract) {
        findings.append(&mut packet_findings);
    }
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

#[cfg(test)]
mod tests;
