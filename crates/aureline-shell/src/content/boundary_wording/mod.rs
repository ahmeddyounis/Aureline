//! Hosted/local/self-hosted/commercial boundary-wording governance and
//! cross-surface copy-parity lint.
//!
//! This module materializes the canonical, export-safe catalog that governs how
//! Aureline talks about its hosting and commercial boundary. Where the
//! [safety-critical string catalog](../m5_safety_critical_string_catalog) locks the
//! *identity* of a message, the [action-label catalog](../m5_action_label_scope_parity)
//! locks its *scope honesty*, the [AI copy guardrails](../ai_copy_guardrails) lock its
//! *trust posture*, and the [content-ops metadata catalog](../content_ops_metadata)
//! locks its *provenance*, this catalog locks its *commercial-boundary honesty*: every
//! Hosted, Managed, Premium, Self-hosted, Local only, BYOK, or Trial claim declares
//! the actual product boundary it maps to, the identity / network / data / export /
//! rollback implications it carries, the alternative local/open paths that remain, and
//! the compatibility/support metadata it is anchored to.
//!
//! It is the boundary-honesty projection of the frozen
//! [content-wording matrix](../../../../../docs/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix.md):
//! the controlled hosting/edition vocabulary is owned by the
//! [deployment-profile register](../../../../../artifacts/governance/deployment_profiles.yaml)
//! and the [controlled glossary](../../../../../artifacts/copy/controlled_glossary.yaml);
//! this catalog proves those identities survive into the user-facing settings,
//! onboarding, marketplace, help/About, release-notes, and account/upgrade surfaces.
//!
//! Four honesty rules drive the validation:
//!
//! - **No boundary overstatement.** A [`BoundaryTerm`] claim can never imply more
//!   local independence than the [`ActualBoundaryPosture`] it maps to provides, so a
//!   surface can never label a managed/paid capability "Local only" or "Self-hosted".
//! - **No false vendor dependence.** When the product contract says a core workflow
//!   stays local-capable, a managed/paid claim must disclose the local / BYOK /
//!   self-hosted alternative that remains — boundary wording can never pressure users
//!   off a valid local or open path.
//! - **Narrowing or widening is machine-anchored.** A claim that narrows or widens a
//!   boundary references the underlying compatibility/support metadata
//!   ([`BoundaryWordingEntry::support_metadata_ref`]) instead of prose-only marketing.
//! - **One boundary concept, one set of facts.** Every surface that renders a shared
//!   [`BoundaryWordingEntry::concept_id`] must agree on the boundary term, the
//!   implication postures, the support metadata, the local-capability posture, and the
//!   disclosed alternatives. The copy-parity lint ([`BoundaryWordingCatalog::lint_parity`])
//!   reports any drift, so release/docs/help/UI review fails on parity or honesty drift
//!   even when the underlying feature code still works.
//!
//! Machine-facing identity stays locale-neutral — entry ids, concept ids, support and
//! source refs are lowercase ascii (`[a-z0-9_.]`) — while human prose localizes safely
//! around it, so a localized overlay can never fork a concept id or a support ref into
//! a different boundary claim. The localized overlay fixture proves it. The packet
//! carries no credential bodies or raw provider payloads, so settings, onboarding,
//! marketplace, help/About, release notes, and account/upgrade prompts can all
//! reconstruct the same boundary facts.
//!
//! The boundary schema is
//! [`schemas/content/m5-boundary-wording.schema.json`](../../../../../schemas/content/m5-boundary-wording.schema.json).
//! The contract doc is
//! [`docs/content/m5/m5_boundary_wording.md`](../../../../../docs/content/m5/m5_boundary_wording.md).
//! The protected fixture directory is
//! [`fixtures/content/m5-boundary-wording/`](../../../../../fixtures/content/m5-boundary-wording/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_boundary_wording_catalog, seeded_boundary_wording_catalog_localized,
    seeded_boundary_wording_catalog_offline_mirror, BOUNDARY_WORDING_CATALOG_ID,
};

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`BoundaryWordingCatalog`].
pub const BOUNDARY_WORDING_CATALOG_RECORD_KIND: &str = "m5_boundary_wording_catalog";

/// Schema version for boundary-wording catalog records.
pub const BOUNDARY_WORDING_CATALOG_SCHEMA_VERSION: u32 = 1;

/// Minimum number of distinct surfaces a shared boundary concept must span.
pub const SHARED_CONCEPT_MIN_SURFACES: usize = 3;

/// Repo-relative path of the boundary schema.
pub const BOUNDARY_WORDING_CATALOG_SCHEMA_REF: &str =
    "schemas/content/m5-boundary-wording.schema.json";

/// Repo-relative path of the catalog contract doc.
pub const BOUNDARY_WORDING_CATALOG_DOC_REF: &str = "docs/content/m5/m5_boundary_wording.md";

/// Repo-relative path of the frozen content-wording matrix this catalog projects.
pub const CONTENT_WORDING_MATRIX_DOC_REF: &str =
    "docs/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix.md";

/// Repo-relative path of the frozen UI copy contract.
pub const UI_COPY_CONTRACT_REF: &str = "docs/copy/ui_copy_contract.md";

/// Repo-relative path of the frozen naming / state-label contract that owns controlled
/// labels and glossary ownership.
pub const NAMING_LABEL_CONTRACT_REF: &str = "docs/copy/naming_and_state_label_contract.md";

/// Repo-relative path of the controlled glossary register.
pub const CONTROLLED_GLOSSARY_REF: &str = "artifacts/copy/controlled_glossary.yaml";

/// Repo-relative path of the deployment-profile register that owns the controlled
/// hosting-boundary and managed/local/self-hosted/open edition vocabulary.
pub const DEPLOYMENT_PROFILES_REF: &str = "artifacts/governance/deployment_profiles.yaml";

/// Repo-relative path of the product truth vocabulary register.
pub const PRODUCT_TRUTH_VOCABULARY_REF: &str = "artifacts/governance/product_truth_vocabulary.yaml";

/// Repo-relative path of the protected fixture directory.
pub const BOUNDARY_WORDING_CATALOG_FIXTURE_DIR: &str = "fixtures/content/m5-boundary-wording";

/// Repo-relative path of the checked support-export artifact.
pub const BOUNDARY_WORDING_CATALOG_ARTIFACT_REF: &str =
    "artifacts/content/m5-boundary-wording-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const BOUNDARY_WORDING_CATALOG_SUMMARY_REF: &str =
    "artifacts/content/m5-boundary-wording-proof/m5_boundary_wording.md";

/// The controlled hosting/commercial boundary term a surface may claim.
///
/// This is the closed vocabulary the lane governs. Open, local-independent language
/// (`LocalOnly`, `SelfHosted`, `Byok`) can never be applied when the actual boundary
/// requires managed services, and managed/paid language (`Hosted`, `Managed`,
/// `Premium`, `Trial`) can never imply vendor dependence where the product contract
/// keeps the core workflow local-capable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryTerm {
    /// Vendor-hosted capability running on managed infrastructure.
    Hosted,
    /// Managed edition with optional or required hosted services.
    Managed,
    /// Paid / premium edition or add-on.
    Premium,
    /// Self-hosted deployment the operator runs.
    SelfHosted,
    /// Local-only posture: no managed recall, sync, or hosted services.
    LocalOnly,
    /// Bring-your-own-key posture: the user supplies provider credentials.
    Byok,
    /// Time-limited trial of a managed/paid capability.
    Trial,
}

impl BoundaryTerm {
    /// Every boundary term, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Hosted,
        Self::Managed,
        Self::Premium,
        Self::SelfHosted,
        Self::LocalOnly,
        Self::Byok,
        Self::Trial,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hosted => "hosted",
            Self::Managed => "managed",
            Self::Premium => "premium",
            Self::SelfHosted => "self_hosted",
            Self::LocalOnly => "local_only",
            Self::Byok => "byok",
            Self::Trial => "trial",
        }
    }

    /// How much local independence from managed/remote vendor services the term
    /// *claims*, on a 1 (vendor-dependent) to 5 (fully local) scale. A term overstates
    /// the boundary when its claimed independence exceeds the actual posture's.
    pub const fn claimed_independence(self) -> u8 {
        match self {
            Self::LocalOnly => 5,
            Self::SelfHosted => 4,
            Self::Byok => 3,
            Self::Hosted | Self::Managed | Self::Premium | Self::Trial => 1,
        }
    }

    /// True when the term introduces a managed or paid capability and therefore must
    /// keep an export/rollback route and disclose any local/open alternative.
    pub const fn introduces_managed_or_paid(self) -> bool {
        matches!(
            self,
            Self::Hosted | Self::Managed | Self::Premium | Self::Trial
        )
    }

    /// True when the term itself names a local/open path users can stay on.
    pub const fn preserves_local_path(self) -> bool {
        matches!(self, Self::LocalOnly | Self::SelfHosted | Self::Byok)
    }
}

/// The actual product boundary a wording claim maps to.
///
/// Mirrors the frozen hosting-boundary / edition vocabulary owned by the
/// [deployment-profile register](DEPLOYMENT_PROFILES_REF). A claim's [`BoundaryTerm`]
/// can never imply more local independence than the posture it maps to provides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActualBoundaryPosture {
    /// Runs fully local with no managed dependency.
    LocalIndependent,
    /// Can be self-hosted by the operator.
    SelfHostable,
    /// Bring-your-own-key: local credentials, remote provider optional.
    Byok,
    /// Managed services are available but optional; the capability works without them.
    ManagedOptional,
    /// Managed services are required for the capability.
    ManagedRequired,
    /// A paid / commercial capability.
    CommercialPaid,
}

impl ActualBoundaryPosture {
    /// Every actual posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalIndependent,
        Self::SelfHostable,
        Self::Byok,
        Self::ManagedOptional,
        Self::ManagedRequired,
        Self::CommercialPaid,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalIndependent => "local_independent",
            Self::SelfHostable => "self_hostable",
            Self::Byok => "byok",
            Self::ManagedOptional => "managed_optional",
            Self::ManagedRequired => "managed_required",
            Self::CommercialPaid => "commercial_paid",
        }
    }

    /// Actual local independence on the same 1 (vendor-dependent) to 5 (fully local)
    /// scale as [`BoundaryTerm::claimed_independence`].
    pub const fn actual_independence(self) -> u8 {
        match self {
            Self::LocalIndependent => 5,
            Self::SelfHostable => 4,
            Self::Byok => 3,
            Self::ManagedOptional => 2,
            Self::ManagedRequired | Self::CommercialPaid => 1,
        }
    }
}

/// A claimed M5 user-facing surface a boundary claim renders on. The closed set is
/// exactly the surfaces a single boundary concept must not drift across.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundarySurface {
    /// Settings panes and their boundary labels.
    Settings,
    /// First-run and onboarding flows.
    Onboarding,
    /// Marketplace / extension listings.
    Marketplace,
    /// Help and About surfaces.
    HelpAbout,
    /// Release notes and release rows.
    ReleaseNotes,
    /// Account and upgrade prompts.
    AccountUpgradePrompt,
}

impl BoundarySurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Settings,
        Self::Onboarding,
        Self::Marketplace,
        Self::HelpAbout,
        Self::ReleaseNotes,
        Self::AccountUpgradePrompt,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Settings => "settings",
            Self::Onboarding => "onboarding",
            Self::Marketplace => "marketplace",
            Self::HelpAbout => "help_about",
            Self::ReleaseNotes => "release_notes",
            Self::AccountUpgradePrompt => "account_upgrade_prompt",
        }
    }

    /// True when the surface is an upgrade, account, or help surface that must disclose
    /// the local / BYOK / self-hosted alternatives when it introduces a managed or paid
    /// capability.
    pub const fn must_disclose_alternatives(self) -> bool {
        matches!(self, Self::HelpAbout | Self::AccountUpgradePrompt)
    }
}

/// Whether a claim narrows, widens, or merely states a boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryClaimKind {
    /// States an existing boundary without changing it.
    StatesBoundary,
    /// Narrows the boundary (restricts a previously broader capability).
    NarrowsBoundary,
    /// Widens the boundary (adds a managed/paid/self-hosted/local option).
    WidensBoundary,
}

impl BoundaryClaimKind {
    /// Every claim kind, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::StatesBoundary,
        Self::NarrowsBoundary,
        Self::WidensBoundary,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatesBoundary => "states_boundary",
            Self::NarrowsBoundary => "narrows_boundary",
            Self::WidensBoundary => "widens_boundary",
        }
    }

    /// True when the claim moves a boundary and therefore must reference the underlying
    /// compatibility/support metadata instead of prose-only marketing.
    pub const fn requires_support_metadata(self) -> bool {
        matches!(self, Self::NarrowsBoundary | Self::WidensBoundary)
    }
}

/// One of the five implication dimensions a boundary claim must explain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryImplication {
    /// Whether sign-in / account / identity posture is needed.
    Identity,
    /// Whether network / remote host access is needed.
    Network,
    /// What data participates and where it stays.
    Data,
    /// Whether an export route remains.
    Export,
    /// Whether a rollback / revert route remains.
    Rollback,
}

impl BoundaryImplication {
    /// Every implication dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Identity,
        Self::Network,
        Self::Data,
        Self::Export,
        Self::Rollback,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::Network => "network",
            Self::Data => "data",
            Self::Export => "export",
            Self::Rollback => "rollback",
        }
    }
}

/// The posture a boundary claim carries for one implication dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImplicationPosture {
    /// The boundary requires this (e.g. identity for a hosted capability).
    Required,
    /// The boundary may use this but does not require it.
    Optional,
    /// The boundary does not require this (e.g. no network for a local-only capability).
    NotRequired,
    /// The capability remains available (e.g. an export or rollback route remains).
    Retained,
    /// The capability is satisfied entirely locally (e.g. data never leaves the device).
    LocalOnly,
}

impl ImplicationPosture {
    /// Every implication posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Required,
        Self::Optional,
        Self::NotRequired,
        Self::Retained,
        Self::LocalOnly,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
            Self::NotRequired => "not_required",
            Self::Retained => "retained",
            Self::LocalOnly => "local_only",
        }
    }
}

/// A local/open alternative path a boundary claim can disclose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlternativePath {
    /// A fully local path with no managed dependency.
    LocalOnly,
    /// A bring-your-own-key path.
    Byok,
    /// A self-hosted path.
    SelfHosted,
    /// An export route off the capability.
    Export,
    /// A rollback / revert route.
    Rollback,
}

impl AlternativePath {
    /// Every alternative path, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOnly,
        Self::Byok,
        Self::SelfHosted,
        Self::Export,
        Self::Rollback,
    ];

    /// Locale-neutral token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Byok => "byok",
            Self::SelfHosted => "self_hosted",
            Self::Export => "export",
            Self::Rollback => "rollback",
        }
    }

    /// True when the path is one of the local/open alternatives an upgrade/account/help
    /// surface must offer when it introduces a managed or paid capability.
    pub const fn is_local_or_open(self) -> bool {
        matches!(self, Self::LocalOnly | Self::Byok | Self::SelfHosted)
    }
}

/// One implication statement: a posture and disclosure for one dimension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImplicationStatement {
    /// The dimension this statement explains.
    pub dimension: BoundaryImplication,
    /// The posture the boundary claim carries for the dimension.
    pub posture: ImplicationPosture,
    /// Human-prose explanation of the implication.
    pub disclosure: String,
}

/// One alternative-path disclosure attached to a boundary claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlternativePathDisclosure {
    /// The alternative path.
    pub path: AlternativePath,
    /// True when the alternative is available per the product contract.
    pub available: bool,
    /// Human-prose explanation of the alternative.
    pub disclosure: String,
    /// Compatibility/support metadata ref backing an available alternative. Required
    /// when [`AlternativePathDisclosure::available`] is true.
    pub reference_ref: Option<String>,
}

/// One governed boundary-wording entry: a typed honesty packet for one boundary claim
/// rendered on one surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryWordingEntry {
    /// Stable, locale-neutral entry id (e.g. `entry.cloud_sync.settings`).
    pub entry_id: String,
    /// Stable, locale-neutral shared concept id; the parity key. Every surface that
    /// renders the same boundary concept shares this id.
    pub concept_id: String,
    /// The controlled boundary term the surface claims.
    pub term: BoundaryTerm,
    /// The surface this claim renders on.
    pub surface: BoundarySurface,
    /// Canonical (default-locale) human-readable wording.
    pub canonical_text: String,
    /// Whether the claim states, narrows, or widens a boundary.
    pub claim_kind: BoundaryClaimKind,
    /// Compatibility/support metadata ref the claim is anchored to. Required when the
    /// claim narrows or widens a boundary.
    pub support_metadata_ref: Option<String>,
    /// The actual product boundary the claim maps to.
    pub actual_boundary_posture: ActualBoundaryPosture,
    /// True when the claim introduces a managed or paid capability.
    pub introduces_managed_or_paid: bool,
    /// True when the product contract keeps the core workflow local-capable, so the
    /// claim must never imply vendor dependence.
    pub core_workflow_remains_local: bool,
    /// The identity/network/data/export/rollback implication statements.
    pub implications: Vec<ImplicationStatement>,
    /// The local/open alternative paths the claim discloses.
    pub alternative_paths: Vec<AlternativePathDisclosure>,
    /// Where the wording came from (a glossary term, source message id, or docs anchor).
    pub source_ref: String,
}

impl BoundaryWordingEntry {
    /// The posture declared for a dimension, if any.
    pub fn posture_for(&self, dimension: BoundaryImplication) -> Option<ImplicationPosture> {
        self.implications
            .iter()
            .find(|s| s.dimension == dimension)
            .map(|s| s.posture)
    }

    /// Whether an alternative path is disclosed as available.
    pub fn alternative_available(&self, path: AlternativePath) -> bool {
        self.alternative_paths
            .iter()
            .any(|a| a.path == path && a.available)
    }

    /// True when the entry overstates the boundary: the term claims more local
    /// independence than the actual posture provides.
    pub fn overstates_boundary(&self) -> bool {
        self.term.claimed_independence() > self.actual_boundary_posture.actual_independence()
    }

    /// True when the entry implies vendor dependence even though the core workflow
    /// stays local-capable and no local/open alternative is disclosed as available.
    pub fn implies_vendor_dependence(&self) -> bool {
        self.core_workflow_remains_local
            && self.term.introduces_managed_or_paid()
            && !AlternativePath::ALL
                .iter()
                .filter(|p| p.is_local_or_open())
                .any(|p| self.alternative_available(*p))
    }

    /// The per-dimension posture map used by the parity lint.
    fn implication_postures(&self) -> BTreeMap<&'static str, &'static str> {
        self.implications
            .iter()
            .map(|s| (s.dimension.as_str(), s.posture.as_str()))
            .collect()
    }

    /// The per-path availability map used by the parity lint.
    fn alternative_availability(&self) -> BTreeMap<&'static str, bool> {
        self.alternative_paths
            .iter()
            .map(|a| (a.path.as_str(), a.available))
            .collect()
    }
}

/// Catalog-level boundary-honesty review block.
///
/// Every flag is a hard invariant; all must hold for the catalog to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryTrustReview {
    /// Surfaces explain the boundary with one controlled vocabulary.
    pub one_controlled_vocabulary_across_surfaces: bool,
    /// Wording never overstates the actual product boundary.
    pub wording_never_overstates_actual_boundary: bool,
    /// Narrowing or widening references the underlying compatibility/support metadata.
    pub narrowing_or_widening_references_support_metadata: bool,
    /// Identity, network, data, export, and rollback implications are all explained.
    pub identity_network_data_export_rollback_explained: bool,
    /// Upgrade/account/help surfaces disclose local/BYOK/self-hosted alternatives.
    pub upgrade_prompts_disclose_local_byok_self_hosted_alternatives: bool,
    /// Wording never pressures users away from a valid local or open path.
    pub never_pressures_away_from_local_or_open_path: bool,
    /// Wording never implies vendor dependence where the core workflow stays local.
    pub never_implies_vendor_dependence_when_core_local: bool,
    /// Managed/paid introductions keep an export and rollback route.
    pub managed_or_paid_introductions_keep_export_and_rollback: bool,
    /// The copy-parity lint blocks cross-surface boundary drift.
    pub copy_parity_lint_blocks_cross_surface_drift: bool,
    /// Review can fail on parity or honesty drift without a feature-code change.
    pub review_can_fail_on_parity_or_honesty_drift_without_code_change: bool,
    /// Claims are machine-anchored to compatibility/support metadata.
    pub machine_anchored_to_compatibility_and_support_metadata: bool,
    /// One catalog is the source of truth, not parallel boundary-prose islands.
    pub one_catalog_not_parallel_boundary_prose_islands: bool,
}

/// Per-surface parity projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryParityProjection {
    /// Settings resolves boundary wording through the catalog.
    pub settings_resolves_through_catalog: bool,
    /// Onboarding resolves boundary wording through the catalog.
    pub onboarding_resolves_through_catalog: bool,
    /// Marketplace resolves boundary wording through the catalog.
    pub marketplace_resolves_through_catalog: bool,
    /// Help/About resolves boundary wording through the catalog.
    pub help_about_resolves_through_catalog: bool,
    /// Release notes resolve boundary wording through the catalog.
    pub release_notes_resolve_through_catalog: bool,
    /// Account/upgrade prompts resolve boundary wording through the catalog.
    pub account_upgrade_prompt_resolves_through_catalog: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the catalog claim.
    pub auto_narrow_on_stale: bool,
}

/// Release and mirror/offline parity posture for the catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting mirror/offline packet.
    pub mirror_offline_packet_ref: String,
    /// True when support/export parity is required for every entry.
    pub support_export_parity_required: bool,
    /// True when mirror/offline parity is required for every entry.
    pub mirror_offline_parity_required: bool,
}

/// Constructor input for [`BoundaryWordingCatalog::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryWordingCatalogInput {
    /// Stable catalog id.
    pub catalog_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Reference locale of the default copy (e.g. `en`).
    pub reference_locale: String,
    /// Boundary-wording entries.
    pub entries: Vec<BoundaryWordingEntry>,
    /// Shared concept ids that must span multiple surfaces.
    pub shared_concept_ids: Vec<String>,
    /// Trust review block.
    pub trust_review: BoundaryTrustReview,
    /// Parity projection block.
    pub parity_projection: BoundaryParityProjection,
    /// Proof freshness block.
    pub proof_freshness: BoundaryProofFreshness,
    /// Release posture.
    pub release_posture: BoundaryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe boundary-wording catalog packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryWordingCatalog {
    /// Record kind; must equal [`BOUNDARY_WORDING_CATALOG_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`BOUNDARY_WORDING_CATALOG_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable catalog id.
    pub catalog_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Reference locale of the default copy.
    pub reference_locale: String,
    /// Closed boundary-term inventory (locale-neutral tokens).
    pub term_inventory: Vec<String>,
    /// Closed actual-posture inventory (locale-neutral tokens).
    pub actual_posture_inventory: Vec<String>,
    /// Closed surface inventory (locale-neutral tokens).
    pub surface_inventory: Vec<String>,
    /// Closed claim-kind inventory (locale-neutral tokens).
    pub claim_kind_inventory: Vec<String>,
    /// Closed implication-dimension inventory (locale-neutral tokens).
    pub implication_inventory: Vec<String>,
    /// Closed implication-posture inventory (locale-neutral tokens).
    pub implication_posture_inventory: Vec<String>,
    /// Closed alternative-path inventory (locale-neutral tokens).
    pub alternative_path_inventory: Vec<String>,
    /// Boundary-wording entries.
    pub entries: Vec<BoundaryWordingEntry>,
    /// Shared concept ids that must span multiple surfaces.
    pub shared_concept_ids: Vec<String>,
    /// Trust review block.
    pub trust_review: BoundaryTrustReview,
    /// Parity projection block.
    pub parity_projection: BoundaryParityProjection,
    /// Proof freshness block.
    pub proof_freshness: BoundaryProofFreshness,
    /// Release posture.
    pub release_posture: BoundaryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl BoundaryWordingCatalog {
    /// Builds a catalog packet from lane input, filling the closed inventories from the
    /// canonical enum token lists.
    pub fn new(input: BoundaryWordingCatalogInput) -> Self {
        Self {
            record_kind: BOUNDARY_WORDING_CATALOG_RECORD_KIND.to_owned(),
            schema_version: BOUNDARY_WORDING_CATALOG_SCHEMA_VERSION,
            catalog_id: input.catalog_id,
            catalog_label: input.catalog_label,
            reference_locale: input.reference_locale,
            term_inventory: token_list(&BoundaryTerm::ALL, BoundaryTerm::as_str),
            actual_posture_inventory: token_list(
                &ActualBoundaryPosture::ALL,
                ActualBoundaryPosture::as_str,
            ),
            surface_inventory: token_list(&BoundarySurface::ALL, BoundarySurface::as_str),
            claim_kind_inventory: token_list(&BoundaryClaimKind::ALL, BoundaryClaimKind::as_str),
            implication_inventory: token_list(
                &BoundaryImplication::ALL,
                BoundaryImplication::as_str,
            ),
            implication_posture_inventory: token_list(
                &ImplicationPosture::ALL,
                ImplicationPosture::as_str,
            ),
            alternative_path_inventory: token_list(&AlternativePath::ALL, AlternativePath::as_str),
            entries: input.entries,
            shared_concept_ids: input.shared_concept_ids,
            trust_review: input.trust_review,
            parity_projection: input.parity_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Resolves an entry by id.
    pub fn entry(&self, entry_id: &str) -> Option<&BoundaryWordingEntry> {
        self.entries.iter().find(|e| e.entry_id == entry_id)
    }

    /// All entries that render a concept, in catalog order.
    pub fn entries_for_concept(&self, concept_id: &str) -> Vec<&BoundaryWordingEntry> {
        self.entries
            .iter()
            .filter(|e| e.concept_id == concept_id)
            .collect()
    }

    /// Maps each concept id to the distinct surfaces it renders on.
    pub fn concept_surfaces(&self) -> BTreeMap<String, BTreeSet<&'static str>> {
        let mut map: BTreeMap<String, BTreeSet<&'static str>> = BTreeMap::new();
        for entry in &self.entries {
            map.entry(entry.concept_id.clone())
                .or_default()
                .insert(entry.surface.as_str());
        }
        map
    }

    /// Renders the deterministic boundary-explanation line for an entry so settings,
    /// onboarding, marketplace, help/About, release notes, and account/upgrade prompts
    /// can explain the boundary with one controlled vocabulary. Returns `None` if the
    /// entry id is unknown.
    pub fn render_boundary_explanation(&self, entry_id: &str) -> Option<String> {
        let entry = self.entry(entry_id)?;
        let mut out = format!(
            "{} [term: {}; actual: {}; surface: {}; claim: {}]",
            entry.canonical_text,
            entry.term.as_str(),
            entry.actual_boundary_posture.as_str(),
            entry.surface.as_str(),
            entry.claim_kind.as_str(),
        );
        for implication in BoundaryImplication::ALL {
            if let Some(posture) = entry.posture_for(implication) {
                out.push_str(&format!("; {}: {}", implication.as_str(), posture.as_str()));
            }
        }
        let alternatives: Vec<&str> = entry
            .alternative_paths
            .iter()
            .filter(|a| a.available)
            .map(|a| a.path.as_str())
            .collect();
        if !alternatives.is_empty() {
            out.push_str(&format!("; alternatives: {}", alternatives.join(", ")));
        }
        if let Some(support) = &entry.support_metadata_ref {
            out.push_str(&format!("; support: {support}"));
        }
        Some(out)
    }

    /// Lints cross-surface copy parity: every surface that renders a shared boundary
    /// concept must agree on the boundary term, the support metadata, the per-dimension
    /// implication postures, the local-capability posture, and the disclosed
    /// alternatives. Returns one [`ParityFinding`] per drift, so release/docs/help/UI
    /// review can fail on parity drift even when feature code still works.
    pub fn lint_parity(&self) -> Vec<ParityFinding> {
        let mut findings = Vec::new();
        let mut by_concept: BTreeMap<&str, Vec<&BoundaryWordingEntry>> = BTreeMap::new();
        for entry in &self.entries {
            by_concept
                .entry(entry.concept_id.as_str())
                .or_default()
                .push(entry);
        }
        for (concept_id, entries) in by_concept {
            let Some((reference, rest)) = entries.split_first() else {
                continue;
            };
            for other in rest {
                if reference.term != other.term {
                    findings.push(ParityFinding::new(
                        concept_id,
                        ParityFindingKind::TermDrift,
                        reference,
                        other,
                        &format!(
                            "term {} vs {}",
                            reference.term.as_str(),
                            other.term.as_str()
                        ),
                    ));
                }
                if reference.support_metadata_ref != other.support_metadata_ref {
                    findings.push(ParityFinding::new(
                        concept_id,
                        ParityFindingKind::SupportMetadataDrift,
                        reference,
                        other,
                        "support metadata refs differ for the same concept",
                    ));
                }
                if reference.implication_postures() != other.implication_postures() {
                    findings.push(ParityFinding::new(
                        concept_id,
                        ParityFindingKind::ImplicationPostureDrift,
                        reference,
                        other,
                        "identity/network/data/export/rollback postures differ",
                    ));
                }
                if reference.core_workflow_remains_local != other.core_workflow_remains_local {
                    findings.push(ParityFinding::new(
                        concept_id,
                        ParityFindingKind::LocalCapabilityPostureDrift,
                        reference,
                        other,
                        "core-workflow-remains-local posture differs",
                    ));
                }
                if reference.alternative_availability() != other.alternative_availability() {
                    findings.push(ParityFinding::new(
                        concept_id,
                        ParityFindingKind::AlternativeAvailabilityDrift,
                        reference,
                        other,
                        "disclosed alternative availability differs",
                    ));
                }
            }
        }
        findings
    }

    /// Validates every catalog invariant.
    pub fn validate(&self) -> Vec<BoundaryViolation> {
        let mut violations = Vec::new();

        if self.record_kind != BOUNDARY_WORDING_CATALOG_RECORD_KIND {
            violations.push(BoundaryViolation::WrongRecordKind);
        }
        if self.schema_version != BOUNDARY_WORDING_CATALOG_SCHEMA_VERSION {
            violations.push(BoundaryViolation::WrongSchemaVersion);
        }
        if self.catalog_id.trim().is_empty()
            || self.catalog_label.trim().is_empty()
            || self.reference_locale.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(BoundaryViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_inventories(self, &mut violations);
        validate_entries(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_shared_concepts(self, &mut violations);
        if !self.lint_parity().is_empty() {
            violations.push(BoundaryViolation::ParityDrift);
        }
        validate_trust_review(self, &mut violations);
        validate_parity_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("boundary wording catalog serializes"),
        ) {
            violations.push(BoundaryViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("boundary wording catalog serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Boundary Wording: Hosted/Local/Self-hosted/Commercial Honesty\n\n");
        out.push_str(&format!("- Catalog: `{}`\n", self.catalog_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalog_label));
        out.push_str(&format!(
            "- Reference locale: `{}`\n",
            self.reference_locale
        ));
        out.push_str(&format!("- Entries: {}\n", self.entries.len()));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Boundary claims by concept\n\n");
        for (concept_id, surfaces) in self.concept_surfaces() {
            out.push_str(&format!(
                "- `{}` — surfaces: {}\n",
                concept_id,
                surfaces.into_iter().collect::<Vec<_>>().join(", ")
            ));
            for entry in self.entries_for_concept(&concept_id) {
                if let Some(line) = self.render_boundary_explanation(&entry.entry_id) {
                    out.push_str(&format!("  - `{}` — {}\n", entry.entry_id, line));
                }
            }
        }

        out.push_str(&self.render_parity_report());
        out
    }

    /// Deterministic copy-parity diff report.
    pub fn render_parity_report(&self) -> String {
        let mut out = String::new();
        out.push_str("\n## Copy-parity lint\n\n");
        let findings = self.lint_parity();
        if findings.is_empty() {
            out.push_str("No cross-surface boundary drift detected.\n");
        } else {
            for finding in findings {
                out.push_str(&format!(
                    "- `{}` [{}]: {} vs {} — {}\n",
                    finding.concept_id,
                    finding.kind.as_str(),
                    finding.surface_a,
                    finding.surface_b,
                    finding.detail
                ));
            }
        }
        out
    }
}

/// The kind of cross-surface parity drift a [`ParityFinding`] reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParityFindingKind {
    /// Two surfaces use different boundary terms for the same concept.
    TermDrift,
    /// Two surfaces reference different support metadata for the same concept.
    SupportMetadataDrift,
    /// Two surfaces declare different implication postures for the same concept.
    ImplicationPostureDrift,
    /// Two surfaces declare different local-capability postures for the same concept.
    LocalCapabilityPostureDrift,
    /// Two surfaces disclose different alternative availability for the same concept.
    AlternativeAvailabilityDrift,
}

impl ParityFindingKind {
    /// Stable token used in reports and tests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TermDrift => "term_drift",
            Self::SupportMetadataDrift => "support_metadata_drift",
            Self::ImplicationPostureDrift => "implication_posture_drift",
            Self::LocalCapabilityPostureDrift => "local_capability_posture_drift",
            Self::AlternativeAvailabilityDrift => "alternative_availability_drift",
        }
    }
}

/// One cross-surface copy-parity drift finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParityFinding {
    /// The drifting concept id.
    pub concept_id: String,
    /// The kind of drift.
    pub kind: ParityFindingKind,
    /// The reference surface token.
    pub surface_a: String,
    /// The drifting surface token.
    pub surface_b: String,
    /// Human-prose detail of the drift.
    pub detail: String,
}

impl ParityFinding {
    fn new(
        concept_id: &str,
        kind: ParityFindingKind,
        reference: &BoundaryWordingEntry,
        other: &BoundaryWordingEntry,
        detail: &str,
    ) -> Self {
        Self {
            concept_id: concept_id.to_owned(),
            kind,
            surface_a: reference.surface.as_str().to_owned(),
            surface_b: other.surface.as_str().to_owned(),
            detail: detail.to_owned(),
        }
    }
}

/// Errors emitted when reading the checked-in catalog export.
#[derive(Debug)]
pub enum BoundaryArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<BoundaryViolation>),
}

impl fmt::Display for BoundaryArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "boundary wording catalog export parse failed: {error}"
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
                    "boundary wording catalog export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for BoundaryArtifactError {}

/// Validation failures emitted by [`BoundaryWordingCatalog::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoundaryViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A closed inventory drifted from the canonical token list.
    InventoryDrift,
    /// An entry is incomplete (text, source, implications).
    EntryIncomplete,
    /// An entry id, concept id, support ref, or source ref is not locale-neutral.
    EntryTokenNotLocaleNeutral,
    /// An entry id is duplicated.
    DuplicateEntry,
    /// A narrowing/widening claim is missing its support metadata ref.
    NarrowingWideningMissingSupportMetadata,
    /// An entry does not explain all five implication dimensions.
    ImplicationDimensionMissing,
    /// A boundary term overstates the actual product boundary.
    BoundaryOverstatesActualPosture,
    /// A managed/paid claim implies vendor dependence where the core workflow is local.
    ImpliesVendorDependenceWhenCoreLocal,
    /// A managed/paid introduction does not keep an export and rollback route.
    ManagedOrPaidMissingExportOrRollback,
    /// An upgrade/account/help surface introduces a managed/paid capability without
    /// disclosing a local/BYOK/self-hosted alternative.
    UpgradeSurfaceMissingAlternativeDisclosure,
    /// An alternative-path disclosure is incomplete.
    AlternativeDisclosureIncomplete,
    /// A boundary term, surface, claim kind, posture, dimension, or alternative path is
    /// never represented.
    CoverageGap,
    /// A shared concept does not span enough surfaces.
    SharedConceptParityInsufficient,
    /// Two surfaces drifted on the same boundary concept.
    ParityDrift,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Parity projection does not satisfy required invariants.
    ParityProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl BoundaryViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::InventoryDrift => "inventory_drift",
            Self::EntryIncomplete => "entry_incomplete",
            Self::EntryTokenNotLocaleNeutral => "entry_token_not_locale_neutral",
            Self::DuplicateEntry => "duplicate_entry",
            Self::NarrowingWideningMissingSupportMetadata => {
                "narrowing_widening_missing_support_metadata"
            }
            Self::ImplicationDimensionMissing => "implication_dimension_missing",
            Self::BoundaryOverstatesActualPosture => "boundary_overstates_actual_posture",
            Self::ImpliesVendorDependenceWhenCoreLocal => {
                "implies_vendor_dependence_when_core_local"
            }
            Self::ManagedOrPaidMissingExportOrRollback => {
                "managed_or_paid_missing_export_or_rollback"
            }
            Self::UpgradeSurfaceMissingAlternativeDisclosure => {
                "upgrade_surface_missing_alternative_disclosure"
            }
            Self::AlternativeDisclosureIncomplete => "alternative_disclosure_incomplete",
            Self::CoverageGap => "coverage_gap",
            Self::SharedConceptParityInsufficient => "shared_concept_parity_insufficient",
            Self::ParityDrift => "parity_drift",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ParityProjectionIncomplete => "parity_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in catalog export.
pub fn current_boundary_wording_catalog_export(
) -> Result<BoundaryWordingCatalog, BoundaryArtifactError> {
    let packet: BoundaryWordingCatalog = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/content/m5-boundary-wording-proof/support_export.json"
    )))
    .map_err(BoundaryArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(BoundaryArtifactError::Validation(violations))
    }
}

/// True when `token` is a locale-neutral machine identifier: non-empty and only
/// lowercase ascii letters, digits, `_`, and `.`.
fn is_locale_neutral(token: &str) -> bool {
    !token.is_empty()
        && token
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.')
}

fn token_list<T: Copy>(all: &[T], as_str: fn(T) -> &'static str) -> Vec<String> {
    all.iter().map(|t| as_str(*t).to_owned()).collect()
}

fn validate_source_contracts(
    packet: &BoundaryWordingCatalog,
    violations: &mut Vec<BoundaryViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        BOUNDARY_WORDING_CATALOG_SCHEMA_REF,
        BOUNDARY_WORDING_CATALOG_DOC_REF,
        CONTENT_WORDING_MATRIX_DOC_REF,
        UI_COPY_CONTRACT_REF,
        NAMING_LABEL_CONTRACT_REF,
        CONTROLLED_GLOSSARY_REF,
        DEPLOYMENT_PROFILES_REF,
        PRODUCT_TRUTH_VOCABULARY_REF,
    ] {
        if !refs.contains(required) {
            violations.push(BoundaryViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_inventories(packet: &BoundaryWordingCatalog, violations: &mut Vec<BoundaryViolation>) {
    if packet.term_inventory != token_list(&BoundaryTerm::ALL, BoundaryTerm::as_str)
        || packet.actual_posture_inventory
            != token_list(&ActualBoundaryPosture::ALL, ActualBoundaryPosture::as_str)
        || packet.surface_inventory != token_list(&BoundarySurface::ALL, BoundarySurface::as_str)
        || packet.claim_kind_inventory
            != token_list(&BoundaryClaimKind::ALL, BoundaryClaimKind::as_str)
        || packet.implication_inventory
            != token_list(&BoundaryImplication::ALL, BoundaryImplication::as_str)
        || packet.implication_posture_inventory
            != token_list(&ImplicationPosture::ALL, ImplicationPosture::as_str)
        || packet.alternative_path_inventory
            != token_list(&AlternativePath::ALL, AlternativePath::as_str)
    {
        violations.push(BoundaryViolation::InventoryDrift);
    }
}

fn validate_entries(packet: &BoundaryWordingCatalog, violations: &mut Vec<BoundaryViolation>) {
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();

    for entry in &packet.entries {
        if entry.canonical_text.trim().is_empty()
            || entry.source_ref.trim().is_empty()
            || entry.implications.is_empty()
        {
            violations.push(BoundaryViolation::EntryIncomplete);
        }
        if !is_locale_neutral(&entry.entry_id)
            || !is_locale_neutral(&entry.concept_id)
            || !is_locale_neutral(&entry.source_ref)
        {
            violations.push(BoundaryViolation::EntryTokenNotLocaleNeutral);
        }
        if let Some(support) = &entry.support_metadata_ref {
            if !is_locale_neutral(support) {
                violations.push(BoundaryViolation::EntryTokenNotLocaleNeutral);
            }
        }
        if !seen_ids.insert(entry.entry_id.as_str()) {
            violations.push(BoundaryViolation::DuplicateEntry);
        }

        validate_entry_support_metadata(entry, violations);
        validate_entry_implications(entry, violations);
        validate_entry_honesty(entry, violations);
        validate_entry_alternatives(entry, violations);
    }
}

fn validate_entry_support_metadata(
    entry: &BoundaryWordingEntry,
    violations: &mut Vec<BoundaryViolation>,
) {
    // A claim that narrows or widens a boundary references the underlying
    // compatibility/support metadata instead of prose-only marketing.
    if entry.claim_kind.requires_support_metadata() {
        let ok = entry
            .support_metadata_ref
            .as_deref()
            .map(|r| !r.trim().is_empty())
            .unwrap_or(false);
        if !ok {
            violations.push(BoundaryViolation::NarrowingWideningMissingSupportMetadata);
        }
    }
}

fn validate_entry_implications(
    entry: &BoundaryWordingEntry,
    violations: &mut Vec<BoundaryViolation>,
) {
    // Every claim explains all five implication dimensions exactly once with a
    // non-empty disclosure.
    let mut seen: BTreeSet<BoundaryImplication> = BTreeSet::new();
    for statement in &entry.implications {
        if statement.disclosure.trim().is_empty() || !seen.insert(statement.dimension) {
            violations.push(BoundaryViolation::ImplicationDimensionMissing);
        }
    }
    if BoundaryImplication::ALL.iter().any(|d| !seen.contains(d)) {
        violations.push(BoundaryViolation::ImplicationDimensionMissing);
    }
}

fn validate_entry_honesty(entry: &BoundaryWordingEntry, violations: &mut Vec<BoundaryViolation>) {
    // No boundary overstatement: the term never claims more independence than the
    // actual posture provides.
    if entry.overstates_boundary() {
        violations.push(BoundaryViolation::BoundaryOverstatesActualPosture);
    }

    // No false vendor dependence: a managed/paid claim with a local-capable core must
    // disclose a local/BYOK/self-hosted alternative.
    if entry.implies_vendor_dependence() {
        violations.push(BoundaryViolation::ImpliesVendorDependenceWhenCoreLocal);
    }

    // A managed/paid introduction keeps an export and rollback route.
    if entry.introduces_managed_or_paid
        && (entry.posture_for(BoundaryImplication::Export) != Some(ImplicationPosture::Retained)
            || entry.posture_for(BoundaryImplication::Rollback)
                != Some(ImplicationPosture::Retained))
    {
        violations.push(BoundaryViolation::ManagedOrPaidMissingExportOrRollback);
    }

    // An upgrade/account/help surface that introduces a managed/paid capability
    // discloses a local/BYOK/self-hosted alternative.
    if entry.surface.must_disclose_alternatives()
        && entry.introduces_managed_or_paid
        && !AlternativePath::ALL
            .iter()
            .filter(|p| p.is_local_or_open())
            .any(|p| entry.alternative_available(*p))
    {
        violations.push(BoundaryViolation::UpgradeSurfaceMissingAlternativeDisclosure);
    }
}

fn validate_entry_alternatives(
    entry: &BoundaryWordingEntry,
    violations: &mut Vec<BoundaryViolation>,
) {
    let mut seen: BTreeSet<AlternativePath> = BTreeSet::new();
    for disclosure in &entry.alternative_paths {
        if disclosure.disclosure.trim().is_empty() || !seen.insert(disclosure.path) {
            violations.push(BoundaryViolation::AlternativeDisclosureIncomplete);
        }
        // An available alternative is machine-anchored to compatibility/support metadata.
        if disclosure.available {
            let ok = disclosure
                .reference_ref
                .as_deref()
                .map(|r| is_locale_neutral(r))
                .unwrap_or(false);
            if !ok {
                violations.push(BoundaryViolation::AlternativeDisclosureIncomplete);
            }
        }
    }
}

fn validate_coverage(packet: &BoundaryWordingCatalog, violations: &mut Vec<BoundaryViolation>) {
    let terms: BTreeSet<BoundaryTerm> = packet.entries.iter().map(|e| e.term).collect();
    let postures: BTreeSet<ActualBoundaryPosture> = packet
        .entries
        .iter()
        .map(|e| e.actual_boundary_posture)
        .collect();
    let surfaces: BTreeSet<BoundarySurface> = packet.entries.iter().map(|e| e.surface).collect();
    let claim_kinds: BTreeSet<BoundaryClaimKind> =
        packet.entries.iter().map(|e| e.claim_kind).collect();
    let dimensions: BTreeSet<BoundaryImplication> = packet
        .entries
        .iter()
        .flat_map(|e| e.implications.iter().map(|s| s.dimension))
        .collect();
    let impl_postures: BTreeSet<ImplicationPosture> = packet
        .entries
        .iter()
        .flat_map(|e| e.implications.iter().map(|s| s.posture))
        .collect();
    let alt_paths: BTreeSet<AlternativePath> = packet
        .entries
        .iter()
        .flat_map(|e| e.alternative_paths.iter().map(|a| a.path))
        .collect();

    let covered = BoundaryTerm::ALL.iter().all(|t| terms.contains(t))
        && ActualBoundaryPosture::ALL
            .iter()
            .all(|p| postures.contains(p))
        && BoundarySurface::ALL.iter().all(|s| surfaces.contains(s))
        && BoundaryClaimKind::ALL
            .iter()
            .all(|c| claim_kinds.contains(c))
        && BoundaryImplication::ALL
            .iter()
            .all(|d| dimensions.contains(d))
        && ImplicationPosture::ALL
            .iter()
            .all(|p| impl_postures.contains(p))
        && AlternativePath::ALL.iter().all(|a| alt_paths.contains(a));

    if !covered {
        violations.push(BoundaryViolation::CoverageGap);
    }
}

fn validate_shared_concepts(
    packet: &BoundaryWordingCatalog,
    violations: &mut Vec<BoundaryViolation>,
) {
    if packet.shared_concept_ids.is_empty() {
        violations.push(BoundaryViolation::SharedConceptParityInsufficient);
        return;
    }
    let surfaces = packet.concept_surfaces();
    for concept_id in &packet.shared_concept_ids {
        let spans = surfaces.get(concept_id).map(BTreeSet::len).unwrap_or(0);
        if spans < SHARED_CONCEPT_MIN_SURFACES {
            violations.push(BoundaryViolation::SharedConceptParityInsufficient);
        }
    }
}

fn validate_trust_review(packet: &BoundaryWordingCatalog, violations: &mut Vec<BoundaryViolation>) {
    let review = &packet.trust_review;
    for ok in [
        review.one_controlled_vocabulary_across_surfaces,
        review.wording_never_overstates_actual_boundary,
        review.narrowing_or_widening_references_support_metadata,
        review.identity_network_data_export_rollback_explained,
        review.upgrade_prompts_disclose_local_byok_self_hosted_alternatives,
        review.never_pressures_away_from_local_or_open_path,
        review.never_implies_vendor_dependence_when_core_local,
        review.managed_or_paid_introductions_keep_export_and_rollback,
        review.copy_parity_lint_blocks_cross_surface_drift,
        review.review_can_fail_on_parity_or_honesty_drift_without_code_change,
        review.machine_anchored_to_compatibility_and_support_metadata,
        review.one_catalog_not_parallel_boundary_prose_islands,
    ] {
        if !ok {
            violations.push(BoundaryViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_parity_projection(
    packet: &BoundaryWordingCatalog,
    violations: &mut Vec<BoundaryViolation>,
) {
    let projection = &packet.parity_projection;
    for ok in [
        projection.settings_resolves_through_catalog,
        projection.onboarding_resolves_through_catalog,
        projection.marketplace_resolves_through_catalog,
        projection.help_about_resolves_through_catalog,
        projection.release_notes_resolve_through_catalog,
        projection.account_upgrade_prompt_resolves_through_catalog,
    ] {
        if !ok {
            violations.push(BoundaryViolation::ParityProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &BoundaryWordingCatalog,
    violations: &mut Vec<BoundaryViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(BoundaryViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &BoundaryWordingCatalog,
    violations: &mut Vec<BoundaryViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
    {
        violations.push(BoundaryViolation::ReleasePostureIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Rewrites a human prose run into a pseudo-localized form by wrapping it in locale
/// markers. Machine-facing identity (ids, refs, tokens) never passes through this
/// function, so a localized overlay can never fork the meaning of an entry.
pub fn pseudo_localize_prose(prose: &str) -> String {
    let trimmed = prose.trim();
    if trimmed.is_empty() {
        return prose.to_owned();
    }
    let leading = &prose[..prose.len() - prose.trim_start().len()];
    let trailing = &prose[prose.trim_end().len()..];
    format!("{leading}\u{27e6}{trimmed}\u{27e7}{trailing}")
}
