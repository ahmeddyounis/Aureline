//! Canonical M5 workflow-bundle compatibility scorecards: one machine-readable
//! scorecard per claimed M5 launch-bundle that attaches honest compatibility,
//! imported-versus-native confidence, and certified-archetype linkage to the
//! bundle it scores so switching guidance and public-truth packs stay auditable.
//!
//! Each [`BundleScorecard`] sits one layer above a workflow-bundle manifest (see
//! [`crate::m5_workflow_bundle_manifests`]) and the certification claim a bundle is allowed to make
//! (see [`crate::certify_launch_bundles_imported_user_handoff_bundles_and`]). It records the source
//! manifest version it scores, the supported platforms, the bundle dependencies and their lifecycle
//! markers, the imported-versus-native confidence, the certified reference-workspace linkage, and
//! the current evidence freshness — and from those it computes an *effective* bundle class so an
//! imported or approximate bundle can no longer inherit native or certified language by inertia.
//!
//! The central rule is that the *claimed* class ([`BundleScorecard::claimed_class`]) is only what a
//! bundle source asserts; the *effective* class ([`BundleScorecard::effective_class`]) is what the
//! scorecard's evidence actually backs. The effective class is recomputed from the claimed class,
//! the [`ImportedVsNativeConfidence`], and the [`EvidenceFreshness`], and can never out-rank any of
//! them: a `Certified`-claimed bundle whose confidence is only `Bridged`, or whose evidence is
//! `Stale`, narrows to `Probable`; an `Approximated` or `Unverified` confidence narrows it to
//! `Imported` or `Preview`. Public copy therefore narrows automatically when proof is weak, stale,
//! or bounded rather than asserting parity the evidence no longer supports.
//!
//! Every scorecard joins to the existing proof formats instead of minting another unlinked one. It
//! carries a [`BundleScorecard::manifest_ref`] into the workflow-bundle manifest, a
//! [`BundleScorecard::compatibility_scorecard_ref`] into the compatibility-scorecard packet, an
//! [`BundleScorecard::archetype_cert_ref`] into the archetype-certification packet, and a
//! [`BundleScorecard::reference_workspace_ref`] into the certified reference workspace.
//!
//! One scorecard drives every consumer: the same packet feeds start center, migration center,
//! help/About, release center, and docs/help bundle surfaces through opaque surface refs, so public
//! copy ingests the same object model that support and release tooling read instead of cloning
//! status text.
//!
//! The packet is checked in at `artifacts/workspace/m5/m5-bundle-scorecards.json` and embedded here.
//! It is metadata-only: every field is a typed state, a count, or an opaque ref, and it carries no
//! credential bodies, raw provider payloads, raw local paths, or bundle binary contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::m5_admission_and_routing::M5Wedge;
pub use crate::m5_workflow_bundle_manifests::LifecycleStage;

/// Supported M5 bundle-scorecards packet schema version.
pub const M5_BUNDLE_SCORECARDS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_BUNDLE_SCORECARDS_RECORD_KIND: &str = "m5_bundle_scorecards_packet";

/// Repo-relative path to the checked-in packet.
pub const M5_BUNDLE_SCORECARDS_PATH: &str = "artifacts/workspace/m5/m5-bundle-scorecards.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_BUNDLE_SCORECARDS_SCHEMA_REF: &str =
    "schemas/workspace/m5-bundle-scorecards.schema.json";

/// Repo-relative path to the companion document.
pub const M5_BUNDLE_SCORECARDS_DOC_REF: &str = "docs/workspace/m5/m5-bundle-scorecards.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_BUNDLE_SCORECARDS_FIXTURE_DIR: &str = "fixtures/workspace/m5/m5-bundle-scorecards";

/// Embedded checked-in packet JSON.
pub const M5_BUNDLE_SCORECARDS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/workspace/m5/m5-bundle-scorecards.json"
));

/// The bundle class a scorecard reports.
///
/// The class is both what a bundle source *claims* and, after the scorecard's evidence is applied,
/// what it is *effectively* allowed to present as. The six classes stay distinct so an imported or
/// approximate bundle never silently reads as certified or native. They form an assurance ladder
/// (see [`Self::rank`]): a stronger claim is never reachable than the weakest of the claimed class,
/// the imported-versus-native confidence, and the evidence freshness allow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleScorecardClass {
    /// A certified bundle: native, currently proven, archetype-linked.
    Certified,
    /// A probable bundle: strong but not currently certified-native.
    Probable,
    /// A community-reviewed bundle with no certification claim.
    Community,
    /// An imported or bridged bundle pending review.
    Imported,
    /// A preview/experimental bundle.
    Preview,
    /// A local draft with no external claim.
    LocalDraft,
}

impl BundleScorecardClass {
    /// Every bundle class, in declaration order (strongest to weakest).
    pub const ALL: [Self; 6] = [
        Self::Certified,
        Self::Probable,
        Self::Community,
        Self::Imported,
        Self::Preview,
        Self::LocalDraft,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Probable => "probable",
            Self::Community => "community",
            Self::Imported => "imported",
            Self::Preview => "preview",
            Self::LocalDraft => "local_draft",
        }
    }

    /// Assurance rank: higher is a stronger claim.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Certified => 5,
            Self::Probable => 4,
            Self::Community => 3,
            Self::Imported => 2,
            Self::Preview => 1,
            Self::LocalDraft => 0,
        }
    }

    /// The class at the given assurance rank.
    pub const fn from_rank(rank: u8) -> Self {
        match rank {
            5 => Self::Certified,
            4 => Self::Probable,
            3 => Self::Community,
            2 => Self::Imported,
            1 => Self::Preview,
            _ => Self::LocalDraft,
        }
    }

    /// Whether a bundle with this class may present as certified.
    pub const fn presents_as_certified(self) -> bool {
        matches!(self, Self::Certified)
    }
}

/// How close a bundle's behavior is to native Aureline behavior.
///
/// This is the imported-versus-native axis: a bundle may run natively, through a compatibility
/// bridge, through an approximate shim, or with no verified mapping at all. Anything other than
/// [`Self::Native`] caps how strong a class the scorecard can present, so approximate behavior can
/// never inherit native or certified language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportedVsNativeConfidence {
    /// Native behavior, mapped exactly.
    Native,
    /// Bridged behavior through a compatibility layer.
    Bridged,
    /// Approximate behavior through a shim.
    Approximated,
    /// No verified imported-versus-native mapping.
    Unverified,
}

impl ImportedVsNativeConfidence {
    /// Every confidence level, in declaration order (strongest to weakest).
    pub const ALL: [Self; 4] = [
        Self::Native,
        Self::Bridged,
        Self::Approximated,
        Self::Unverified,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Bridged => "bridged",
            Self::Approximated => "approximated",
            Self::Unverified => "unverified",
        }
    }

    /// The strongest class rank this confidence can back.
    pub const fn cap_rank(self) -> u8 {
        match self {
            Self::Native => 5,
            Self::Bridged => 4,
            Self::Approximated => 2,
            Self::Unverified => 1,
        }
    }

    /// Whether this confidence is fully native.
    pub const fn is_native(self) -> bool {
        matches!(self, Self::Native)
    }
}

/// How current the evidence backing a scorecard is.
///
/// Stale or missing evidence narrows the public copy automatically: a bundle whose proof has gone
/// past its window can no longer present as certified, and a bundle with no evidence at all narrows
/// further still.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// Fresh, current evidence.
    Fresh,
    /// Aging but still within the freshness window.
    Aging,
    /// Stale evidence past its freshness window.
    Stale,
    /// No evidence recorded.
    Missing,
}

impl EvidenceFreshness {
    /// Every freshness level, in declaration order (freshest to weakest).
    pub const ALL: [Self; 4] = [Self::Fresh, Self::Aging, Self::Stale, Self::Missing];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Aging => "aging",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }

    /// The strongest class rank this freshness can back.
    pub const fn cap_rank(self) -> u8 {
        match self {
            Self::Fresh => 5,
            Self::Aging => 5,
            Self::Stale => 4,
            Self::Missing => 2,
        }
    }

    /// Whether the evidence is stale or missing.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::Stale | Self::Missing)
    }
}

/// A platform a bundle declares support for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformClass {
    /// Linux.
    Linux,
    /// macOS.
    Macos,
    /// Windows.
    Windows,
    /// The web target.
    Web,
}

impl PlatformClass {
    /// Every platform, in declaration order.
    pub const ALL: [Self; 4] = [Self::Linux, Self::Macos, Self::Windows, Self::Web];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Linux => "linux",
            Self::Macos => "macos",
            Self::Windows => "windows",
            Self::Web => "web",
        }
    }
}

/// One declared dependency a scored bundle carries, with its lifecycle marker.
///
/// A dependency is a diffable reference, never an opaque blob: it names a stable id, the
/// [`LifecycleStage`] of the capability it depends on, a one-line label, and an opaque ref. A
/// non-stable lifecycle stage is a disclosed dependency marker the scorecard surfaces rather than
/// buries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BundleDependency {
    /// Stable dependency identifier within the scorecard.
    pub dependency_id: String,
    /// The lifecycle stage of the capability this dependency depends on.
    pub lifecycle_stage: LifecycleStage,
    /// A human-readable, one-line dependency label.
    pub label: String,
    /// Opaque registry/repo ref backing the dependency.
    pub dependency_ref: String,
}

impl BundleDependency {
    /// Whether the dependency is internally consistent.
    pub fn is_consistent(&self) -> bool {
        !self.dependency_id.trim().is_empty()
            && !self.label.trim().is_empty()
            && !self.dependency_ref.trim().is_empty()
    }

    /// Whether this dependency depends on a non-stable capability.
    pub fn is_non_stable(&self) -> bool {
        self.lifecycle_stage.is_non_stable()
    }
}

/// One compatibility scorecard for a single claimed M5 launch bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BundleScorecard {
    /// Stable scorecard identifier.
    pub scorecard_id: String,
    /// The bundle id this scorecard scores; matches the workflow-bundle manifest.
    pub bundle_id: String,
    /// The M5 launch wedge this bundle composes.
    pub wedge: M5Wedge,
    /// The version of the source workflow-bundle manifest this scorecard scores.
    pub source_manifest_version: String,
    /// The bundle class the bundle source claims.
    pub claimed_class: BundleScorecardClass,
    /// The effective bundle class after confidence and freshness are applied. Must equal the value
    /// computed by [`BundleScorecard::computed_effective_class`].
    pub effective_class: BundleScorecardClass,
    /// How close the bundle's behavior is to native.
    pub imported_vs_native_confidence: ImportedVsNativeConfidence,
    /// How current the evidence backing this scorecard is.
    pub evidence_freshness: EvidenceFreshness,
    /// The platforms this bundle supports. Non-empty, de-duplicated.
    pub supported_platforms: Vec<PlatformClass>,
    /// Whether support is bounded to a subset of platforms. Must equal whether
    /// [`Self::supported_platforms`] omits any platform.
    pub platform_bounded: bool,
    /// The bundle's declared dependencies and their lifecycle markers.
    pub bundle_dependencies: Vec<BundleDependency>,
    /// Whether the scorecard presents as certified. Must equal whether the effective class is
    /// certified.
    pub presents_as_certified: bool,
    /// Opaque ref into the workflow-bundle manifest this scorecard scores.
    pub manifest_ref: String,
    /// Opaque ref into the compatibility-scorecard packet this scorecard joins.
    pub compatibility_scorecard_ref: String,
    /// Opaque ref into the archetype-certification packet this scorecard joins.
    pub archetype_cert_ref: String,
    /// Opaque ref into the certified reference workspace backing this scorecard.
    pub reference_workspace_ref: String,
    /// Accountable owner.
    pub owner: String,
    /// Caveats shown on the bundle. Required whenever the scorecard narrows.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Start-center consumer ref.
    pub start_center_ref: String,
    /// Migration-center consumer ref.
    pub migration_center_ref: String,
    /// Help/About consumer ref.
    pub help_about_ref: String,
    /// Release-center consumer ref.
    pub release_center_ref: String,
    /// Docs/help consumer ref.
    pub docs_help_ref: String,
    /// Support-export surface ref.
    pub support_export_ref: String,
    /// Diagnostics surface ref.
    pub diagnostics_ref: String,
    /// A reviewer note summarizing the scorecard.
    pub note: String,
}

impl BundleScorecard {
    /// The effective class computed from the claimed class, the confidence, and the freshness.
    ///
    /// The result can never out-rank any of the three inputs: the effective rank is the minimum of
    /// the claimed class rank, the confidence cap, and the freshness cap.
    pub fn computed_effective_class(&self) -> BundleScorecardClass {
        let rank = self
            .claimed_class
            .rank()
            .min(self.imported_vs_native_confidence.cap_rank())
            .min(self.evidence_freshness.cap_rank());
        BundleScorecardClass::from_rank(rank)
    }

    /// Whether the recorded effective class matches the recomputed one.
    pub fn effective_class_consistent(&self) -> bool {
        self.effective_class == self.computed_effective_class()
    }

    /// Whether the scorecard was narrowed below its claimed class.
    pub fn was_downgraded(&self) -> bool {
        self.effective_class != self.claimed_class
    }

    /// Whether platform support is bounded to a subset of platforms.
    pub fn computed_platform_bounded(&self) -> bool {
        PlatformClass::ALL
            .iter()
            .any(|p| !self.supported_platforms.contains(p))
    }

    /// Whether the recorded platform-bounded flag matches the supported-platform set.
    pub fn platform_bounded_consistent(&self) -> bool {
        self.platform_bounded == self.computed_platform_bounded()
    }

    /// Whether the supported-platform list is non-empty and free of duplicates.
    pub fn platforms_well_formed(&self) -> bool {
        if self.supported_platforms.is_empty() {
            return false;
        }
        let unique: BTreeSet<_> = self.supported_platforms.iter().collect();
        unique.len() == self.supported_platforms.len()
    }

    /// Whether the recorded certified presentation matches the effective class.
    pub fn presentation_consistent(&self) -> bool {
        self.presents_as_certified == self.effective_class.presents_as_certified()
    }

    /// Whether a caveat is required on this scorecard.
    ///
    /// A scorecard needs a caveat whenever its public copy must narrow: it was downgraded, it does
    /// not present as a fully certified native bundle, its confidence is not native, its evidence is
    /// stale or missing, or its support is platform-bounded. Only a certified, native, fresh,
    /// full-platform bundle escapes a caveat.
    pub fn caveats_required(&self) -> bool {
        !(self.claimed_class == BundleScorecardClass::Certified
            && self.effective_class == BundleScorecardClass::Certified
            && self.imported_vs_native_confidence.is_native()
            && self.evidence_freshness == EvidenceFreshness::Fresh
            && !self.platform_bounded)
    }

    /// Whether any dependency depends on a non-stable capability.
    pub fn has_non_stable_dependencies(&self) -> bool {
        self.bundle_dependencies.iter().any(|d| d.is_non_stable())
    }

    /// Whether the scorecard joins to the manifest, compatibility, archetype, and reference-workspace
    /// proofs instead of standing alone.
    pub fn linkage_complete(&self) -> bool {
        !self.manifest_ref.trim().is_empty()
            && !self.compatibility_scorecard_ref.trim().is_empty()
            && !self.archetype_cert_ref.trim().is_empty()
            && !self.reference_workspace_ref.trim().is_empty()
    }

    /// Whether every consumer surface ref is present.
    pub fn consumers_complete(&self) -> bool {
        [
            &self.start_center_ref,
            &self.migration_center_ref,
            &self.help_about_ref,
            &self.release_center_ref,
            &self.docs_help_ref,
            &self.support_export_ref,
            &self.diagnostics_ref,
        ]
        .iter()
        .all(|r| !r.trim().is_empty())
    }

    /// Whether the scorecard is internally consistent against the honesty contract.
    pub fn is_consistent(&self) -> bool {
        self.effective_class_consistent()
            && self.platform_bounded_consistent()
            && self.platforms_well_formed()
            && self.presentation_consistent()
            && self.linkage_complete()
            && self.consumers_complete()
            && self
                .bundle_dependencies
                .iter()
                .all(BundleDependency::is_consistent)
            && !self.scorecard_id.trim().is_empty()
            && !self.bundle_id.trim().is_empty()
            && !self.source_manifest_version.trim().is_empty()
            && !self.owner.trim().is_empty()
            && !self.note.trim().is_empty()
            && (!self.caveats_required() || self.caveats.iter().any(|c| !c.trim().is_empty()))
    }
}

/// Summary counts rolled up across every bundle scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5BundleScorecardsSummary {
    /// Total scorecards.
    pub total_scorecards: usize,
    /// Scorecards claiming a certified class.
    pub claimed_certified: usize,
    /// Scorecards claiming a probable class.
    pub claimed_probable: usize,
    /// Scorecards claiming a community class.
    pub claimed_community: usize,
    /// Scorecards claiming an imported class.
    pub claimed_imported: usize,
    /// Scorecards claiming a preview class.
    pub claimed_preview: usize,
    /// Scorecards claiming a local-draft class.
    pub claimed_local_draft: usize,
    /// Scorecards whose effective class is certified.
    pub effective_certified: usize,
    /// Scorecards whose effective class is probable.
    pub effective_probable: usize,
    /// Scorecards whose effective class is community.
    pub effective_community: usize,
    /// Scorecards whose effective class is imported.
    pub effective_imported: usize,
    /// Scorecards whose effective class is preview.
    pub effective_preview: usize,
    /// Scorecards whose effective class is local-draft.
    pub effective_local_draft: usize,
    /// Scorecards that present as certified.
    pub presents_as_certified: usize,
    /// Scorecards narrowed below their claimed class.
    pub downgraded_scorecards: usize,
    /// Scorecards with platform-bounded support.
    pub platform_bounded_scorecards: usize,
    /// Scorecards whose evidence is stale or missing.
    pub stale_or_missing_evidence: usize,
    /// Scorecards whose confidence is not fully native.
    pub non_native_confidence: usize,
    /// Scorecards that carry at least one non-stable dependency.
    pub scorecards_with_non_stable_dependencies: usize,
    /// Total dependencies across all scorecards.
    pub total_dependencies: usize,
    /// Dependencies on a non-stable capability.
    pub non_stable_dependencies: usize,
}

/// One redaction-safe export row projected from a scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleScorecardsExportRow {
    /// Scorecard id.
    pub scorecard_id: String,
    /// Bundle id.
    pub bundle_id: String,
    /// Wedge token.
    pub wedge: M5Wedge,
    /// Source manifest version.
    pub source_manifest_version: String,
    /// Claimed class token.
    pub claimed_class: BundleScorecardClass,
    /// Effective class token.
    pub effective_class: BundleScorecardClass,
    /// Imported-versus-native confidence token.
    pub imported_vs_native_confidence: ImportedVsNativeConfidence,
    /// Evidence freshness token.
    pub evidence_freshness: EvidenceFreshness,
    /// Whether support is platform-bounded.
    pub platform_bounded: bool,
    /// Whether the bundle presents as certified.
    pub presents_as_certified: bool,
    /// Whether the scorecard was narrowed below its claimed class.
    pub was_downgraded: bool,
}

/// A redaction-safe export projection of the whole packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BundleScorecardsExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected scorecard rows.
    pub scorecards: Vec<M5BundleScorecardsExportRow>,
    /// Whether every scorecard is gate-consistent.
    pub all_scorecards_consistent: bool,
    /// Scorecards that present as certified.
    pub presents_as_certified: usize,
    /// Scorecards narrowed below their claimed class.
    pub downgraded_scorecards: usize,
}

/// The typed M5 bundle-scorecards packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5BundleScorecardsPacket {
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
    /// Scheme the packet mints stable scorecard identities under.
    pub scorecard_identity_scheme: String,
    /// Closed wedge vocabulary.
    pub wedges: Vec<M5Wedge>,
    /// Closed bundle-class vocabulary.
    pub bundle_classes: Vec<BundleScorecardClass>,
    /// Closed imported-versus-native confidence vocabulary.
    pub confidence_levels: Vec<ImportedVsNativeConfidence>,
    /// Closed evidence-freshness vocabulary.
    pub freshness_levels: Vec<EvidenceFreshness>,
    /// Closed platform vocabulary.
    pub platform_classes: Vec<PlatformClass>,
    /// Closed dependency lifecycle-stage vocabulary.
    pub dependency_lifecycle_stages: Vec<LifecycleStage>,
    /// One scorecard per claimed M5 launch bundle.
    #[serde(default)]
    pub scorecards: Vec<BundleScorecard>,
    /// Summary counts.
    pub summary: M5BundleScorecardsSummary,
}

impl M5BundleScorecardsPacket {
    /// Returns the scorecard for the given bundle id.
    pub fn scorecard(&self, bundle_id: &str) -> Option<&BundleScorecard> {
        self.scorecards.iter().find(|s| s.bundle_id == bundle_id)
    }

    /// Scorecards scoring a bundle for the given wedge.
    pub fn scorecards_for_wedge(&self, wedge: M5Wedge) -> impl Iterator<Item = &BundleScorecard> {
        self.scorecards.iter().filter(move |s| s.wedge == wedge)
    }

    /// Scorecards whose effective class is the given class.
    pub fn scorecards_with_effective_class(
        &self,
        class: BundleScorecardClass,
    ) -> impl Iterator<Item = &BundleScorecard> {
        self.scorecards
            .iter()
            .filter(move |s| s.effective_class == class)
    }

    /// Whether every M5 wedge has a scorecard.
    pub fn covers_every_wedge(&self) -> bool {
        M5Wedge::ALL
            .iter()
            .all(|wedge| self.scorecards_for_wedge(*wedge).next().is_some())
    }

    /// Whether every scorecard is internally consistent against the gate.
    pub fn all_scorecards_consistent(&self) -> bool {
        self.scorecards.iter().all(BundleScorecard::is_consistent)
    }

    /// Recomputes the summary from the scorecards.
    pub fn computed_summary(&self) -> M5BundleScorecardsSummary {
        let count_claimed = |class: BundleScorecardClass| {
            self.scorecards
                .iter()
                .filter(|s| s.claimed_class == class)
                .count()
        };
        let count_effective = |class: BundleScorecardClass| {
            self.scorecards
                .iter()
                .filter(|s| s.effective_class == class)
                .count()
        };
        let deps = || {
            self.scorecards
                .iter()
                .flat_map(|s| s.bundle_dependencies.iter())
        };
        M5BundleScorecardsSummary {
            total_scorecards: self.scorecards.len(),
            claimed_certified: count_claimed(BundleScorecardClass::Certified),
            claimed_probable: count_claimed(BundleScorecardClass::Probable),
            claimed_community: count_claimed(BundleScorecardClass::Community),
            claimed_imported: count_claimed(BundleScorecardClass::Imported),
            claimed_preview: count_claimed(BundleScorecardClass::Preview),
            claimed_local_draft: count_claimed(BundleScorecardClass::LocalDraft),
            effective_certified: count_effective(BundleScorecardClass::Certified),
            effective_probable: count_effective(BundleScorecardClass::Probable),
            effective_community: count_effective(BundleScorecardClass::Community),
            effective_imported: count_effective(BundleScorecardClass::Imported),
            effective_preview: count_effective(BundleScorecardClass::Preview),
            effective_local_draft: count_effective(BundleScorecardClass::LocalDraft),
            presents_as_certified: self
                .scorecards
                .iter()
                .filter(|s| s.presents_as_certified)
                .count(),
            downgraded_scorecards: self
                .scorecards
                .iter()
                .filter(|s| s.was_downgraded())
                .count(),
            platform_bounded_scorecards: self
                .scorecards
                .iter()
                .filter(|s| s.platform_bounded)
                .count(),
            stale_or_missing_evidence: self
                .scorecards
                .iter()
                .filter(|s| s.evidence_freshness.is_stale())
                .count(),
            non_native_confidence: self
                .scorecards
                .iter()
                .filter(|s| !s.imported_vs_native_confidence.is_native())
                .count(),
            scorecards_with_non_stable_dependencies: self
                .scorecards
                .iter()
                .filter(|s| s.has_non_stable_dependencies())
                .count(),
            total_dependencies: deps().count(),
            non_stable_dependencies: deps().filter(|d| d.is_non_stable()).count(),
        }
    }

    /// Projects a redaction-safe export view of the packet.
    pub fn export_projection(&self) -> M5BundleScorecardsExportProjection {
        let scorecards = self
            .scorecards
            .iter()
            .map(|s| M5BundleScorecardsExportRow {
                scorecard_id: s.scorecard_id.clone(),
                bundle_id: s.bundle_id.clone(),
                wedge: s.wedge,
                source_manifest_version: s.source_manifest_version.clone(),
                claimed_class: s.claimed_class,
                effective_class: s.effective_class,
                imported_vs_native_confidence: s.imported_vs_native_confidence,
                evidence_freshness: s.evidence_freshness,
                platform_bounded: s.platform_bounded,
                presents_as_certified: s.presents_as_certified,
                was_downgraded: s.was_downgraded(),
            })
            .collect();
        let summary = self.computed_summary();
        M5BundleScorecardsExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            scorecards,
            all_scorecards_consistent: self.all_scorecards_consistent(),
            presents_as_certified: summary.presents_as_certified,
            downgraded_scorecards: summary.downgraded_scorecards,
        }
    }

    /// Validates the packet against its honesty contract, returning every violation.
    pub fn validate(&self) -> Vec<M5BundleScorecardsViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        let mut seen_ids = BTreeSet::new();
        for scorecard in &self.scorecards {
            if !seen_ids.insert(scorecard.bundle_id.clone()) {
                violations.push(M5BundleScorecardsViolation::DuplicateBundleId {
                    bundle_id: scorecard.bundle_id.clone(),
                });
            }
            self.validate_scorecard(scorecard, &mut violations);
        }
        for wedge in M5Wedge::ALL {
            if self.scorecards_for_wedge(wedge).next().is_none() {
                violations.push(M5BundleScorecardsViolation::MissingWedgeCoverage { wedge });
            }
        }
        if self.summary != self.computed_summary() {
            violations.push(M5BundleScorecardsViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5BundleScorecardsViolation>) {
        if self.schema_version != M5_BUNDLE_SCORECARDS_SCHEMA_VERSION {
            violations.push(M5BundleScorecardsViolation::SchemaVersionMismatch {
                found: self.schema_version,
            });
        }
        if self.record_kind != M5_BUNDLE_SCORECARDS_RECORD_KIND {
            violations.push(M5BundleScorecardsViolation::RecordKindMismatch {
                found: self.record_kind.clone(),
            });
        }
        let vocab_ok = self.wedges == M5Wedge::ALL
            && self.bundle_classes == BundleScorecardClass::ALL
            && self.confidence_levels == ImportedVsNativeConfidence::ALL
            && self.freshness_levels == EvidenceFreshness::ALL
            && self.platform_classes == PlatformClass::ALL
            && self.dependency_lifecycle_stages == LifecycleStage::ALL;
        if !vocab_ok {
            violations.push(M5BundleScorecardsViolation::VocabularyMismatch);
        }
        if self.scorecards.is_empty() {
            violations.push(M5BundleScorecardsViolation::NoScorecards);
        }
    }

    fn validate_scorecard(
        &self,
        scorecard: &BundleScorecard,
        violations: &mut Vec<M5BundleScorecardsViolation>,
    ) {
        let id = scorecard.bundle_id.clone();
        if scorecard.scorecard_id.trim().is_empty() || id.trim().is_empty() {
            violations.push(M5BundleScorecardsViolation::EmptyIdentity);
        }
        if scorecard.source_manifest_version.trim().is_empty() {
            violations.push(M5BundleScorecardsViolation::EmptyManifestVersion {
                bundle_id: id.clone(),
            });
        }
        if !scorecard.effective_class_consistent() {
            violations.push(M5BundleScorecardsViolation::EffectiveClassMismatch {
                bundle_id: id.clone(),
                recorded: scorecard.effective_class,
                computed: scorecard.computed_effective_class(),
            });
        }
        if !scorecard.platform_bounded_consistent() || !scorecard.platforms_well_formed() {
            violations.push(M5BundleScorecardsViolation::PlatformMismatch {
                bundle_id: id.clone(),
            });
        }
        if !scorecard.presentation_consistent() {
            violations.push(M5BundleScorecardsViolation::CertifiedPresentationMismatch {
                bundle_id: id.clone(),
            });
        }
        if !scorecard.linkage_complete() {
            violations.push(M5BundleScorecardsViolation::MissingLinkage {
                bundle_id: id.clone(),
            });
        }
        if !scorecard.consumers_complete() {
            violations.push(M5BundleScorecardsViolation::MissingConsumerRef {
                bundle_id: id.clone(),
            });
        }
        for dependency in &scorecard.bundle_dependencies {
            if !dependency.is_consistent() {
                violations.push(M5BundleScorecardsViolation::InconsistentDependency {
                    bundle_id: id.clone(),
                    dependency_id: dependency.dependency_id.clone(),
                });
            }
        }
        if scorecard.caveats_required() && scorecard.caveats.iter().all(|c| c.trim().is_empty()) {
            violations.push(M5BundleScorecardsViolation::MissingCaveat {
                bundle_id: id.clone(),
            });
        }
        if scorecard.owner.trim().is_empty() || scorecard.note.trim().is_empty() {
            violations.push(M5BundleScorecardsViolation::MissingSurfaceRef { bundle_id: id });
        }
    }
}

/// A single way the packet can fail its honesty contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5BundleScorecardsViolation {
    /// The schema version does not match the supported version.
    SchemaVersionMismatch {
        /// The version found in the packet.
        found: u32,
    },
    /// The record kind does not match the canonical tag.
    RecordKindMismatch {
        /// The record kind found in the packet.
        found: String,
    },
    /// A closed vocabulary array does not match its canonical `ALL`.
    VocabularyMismatch,
    /// The packet carries no scorecards.
    NoScorecards,
    /// An M5 wedge has no scorecard.
    MissingWedgeCoverage {
        /// The uncovered wedge.
        wedge: M5Wedge,
    },
    /// Two scorecards share a bundle id.
    DuplicateBundleId {
        /// The duplicated id.
        bundle_id: String,
    },
    /// A scorecard or bundle id is empty.
    EmptyIdentity,
    /// A scorecard lacks a source manifest version.
    EmptyManifestVersion {
        /// The offending bundle id.
        bundle_id: String,
    },
    /// A scorecard's recorded effective class diverges from the recomputed class.
    EffectiveClassMismatch {
        /// The offending bundle id.
        bundle_id: String,
        /// The recorded effective class.
        recorded: BundleScorecardClass,
        /// The recomputed effective class.
        computed: BundleScorecardClass,
    },
    /// A scorecard's platform set is malformed or its bounded flag diverges.
    PlatformMismatch {
        /// The offending bundle id.
        bundle_id: String,
    },
    /// A scorecard's certified presentation diverges from its effective class.
    CertifiedPresentationMismatch {
        /// The offending bundle id.
        bundle_id: String,
    },
    /// A scorecard does not join to the manifest, compatibility, archetype, and reference proofs.
    MissingLinkage {
        /// The offending bundle id.
        bundle_id: String,
    },
    /// A scorecard is missing a consumer surface ref.
    MissingConsumerRef {
        /// The offending bundle id.
        bundle_id: String,
    },
    /// A dependency is internally inconsistent.
    InconsistentDependency {
        /// The offending bundle id.
        bundle_id: String,
        /// The offending dependency id.
        dependency_id: String,
    },
    /// A scorecard that needs a caveat carries none.
    MissingCaveat {
        /// The offending bundle id.
        bundle_id: String,
    },
    /// A scorecard is missing an owner or note.
    MissingSurfaceRef {
        /// The offending bundle id.
        bundle_id: String,
    },
    /// The recorded summary diverges from the recomputed summary.
    SummaryMismatch,
}

impl fmt::Display for M5BundleScorecardsViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersionMismatch { found } => {
                write!(f, "schema_version mismatch: found {found}")
            }
            Self::RecordKindMismatch { found } => write!(f, "record_kind mismatch: found {found}"),
            Self::VocabularyMismatch => {
                write!(f, "a closed vocabulary array diverges from its canonical set")
            }
            Self::NoScorecards => write!(f, "packet carries no bundle scorecards"),
            Self::MissingWedgeCoverage { wedge } => {
                write!(f, "wedge {} has no bundle scorecard", wedge.as_str())
            }
            Self::DuplicateBundleId { bundle_id } => {
                write!(f, "duplicate bundle id: {bundle_id}")
            }
            Self::EmptyIdentity => write!(f, "a scorecard has an empty scorecard or bundle id"),
            Self::EmptyManifestVersion { bundle_id } => {
                write!(f, "scorecard {bundle_id} has an empty source manifest version")
            }
            Self::EffectiveClassMismatch {
                bundle_id,
                recorded,
                computed,
            } => write!(
                f,
                "scorecard {bundle_id} records effective class {} but evidence backs {}",
                recorded.as_str(),
                computed.as_str()
            ),
            Self::PlatformMismatch { bundle_id } => write!(
                f,
                "scorecard {bundle_id} has a malformed platform set or a divergent bounded flag"
            ),
            Self::CertifiedPresentationMismatch { bundle_id } => write!(
                f,
                "scorecard {bundle_id} certified presentation diverges from its effective class"
            ),
            Self::MissingLinkage { bundle_id } => write!(
                f,
                "scorecard {bundle_id} does not join to manifest, compatibility, archetype, and reference proofs"
            ),
            Self::MissingConsumerRef { bundle_id } => {
                write!(f, "scorecard {bundle_id} is missing a consumer surface ref")
            }
            Self::InconsistentDependency {
                bundle_id,
                dependency_id,
            } => write!(
                f,
                "scorecard {bundle_id} dependency {dependency_id} is inconsistent"
            ),
            Self::MissingCaveat { bundle_id } => {
                write!(f, "scorecard {bundle_id} needs a caveat but carries none")
            }
            Self::MissingSurfaceRef { bundle_id } => {
                write!(f, "scorecard {bundle_id} is missing an owner or note")
            }
            Self::SummaryMismatch => write!(f, "summary diverges from the recomputed summary"),
        }
    }
}

impl Error for M5BundleScorecardsViolation {}

/// Loads the embedded canonical M5 bundle-scorecards packet.
///
/// # Errors
///
/// Returns a deserialization error if the embedded JSON does not parse into the typed packet.
pub fn current_m5_bundle_scorecards_packet() -> Result<M5BundleScorecardsPacket, serde_json::Error>
{
    serde_json::from_str(M5_BUNDLE_SCORECARDS_JSON)
}

#[cfg(test)]
mod tests;
