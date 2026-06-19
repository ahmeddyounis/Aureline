//! Typed M5 public-contract certification register: the closeout certification join for the
//! whole M5 public-contract publication lane.
//!
//! Earlier rows publish the individual contract forms (the JSON Schema catalog, the OpenAPI
//! catalog, the WIT publication, the reader/writer compatibility suite, the
//! interchange-conformance register), the publication matrix that records *whether* each
//! family published its required forms, and the contract-health register that *enforces*
//! those forms with CI gates and a release-graph linkage. This register is the certification
//! layer above all of them: for every claimed M5 public artifact family it binds the
//! published contract form, the lifecycle metadata, the example corpus, the validator
//! coverage, the compatibility report, and the release-graph linkage into one
//! [`CertificationState`] and decides whether the family certifies its contract claim or is
//! narrowed/withheld.
//!
//! It reuses the contract-health register's per-family gate evaluation and the publication
//! matrix's lifecycle labels rather than minting a new vocabulary: a family certifies only
//! when every required [`Pillar`] is `current` and its published label matches its public
//! claim; a family may never certify a greener label than its public claim
//! ([`Row::claim_label`]); a family whose public claim already narrowed inherits that
//! narrowing; and a release-blocking family missing a required pillar withholds certification
//! and holds promotion ([`Promotion::decision`] is [`DecisionState::Hold`]).
//!
//! The register is checked in at `artifacts/certification/m5-public-contract-certification.json`
//! and embedded here, so this typed consumer and the CI validator agree on every family and
//! pillar without a cargo build in CI. The model is metadata-plus-state only: every field is a
//! typed state, an opaque repo-relative ref or URI, or a copy/export-safe summary. It carries
//! no credential bodies or raw provider payloads.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Supported register schema version.
pub const M5_PUBLIC_CONTRACT_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_PUBLIC_CONTRACT_CERTIFICATION_RECORD_KIND: &str = "m5_public_contract_certification";

/// Stable register identifier.
pub const M5_PUBLIC_CONTRACT_CERTIFICATION_REGISTER_ID: &str =
    "m5_public_contract_certification:v1";

/// Repo-relative path to the checked-in register.
pub const M5_PUBLIC_CONTRACT_CERTIFICATION_PATH: &str =
    "artifacts/certification/m5-public-contract-certification.json";

/// Embedded checked-in register JSON.
pub const M5_PUBLIC_CONTRACT_CERTIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/certification/m5-public-contract-certification.json"
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
    /// Every label, in declaration order (most-mature first).
    pub const ALL: [Self; 5] = [
        Self::Lts,
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Withdrawn,
    ];

    /// The maturity rank: a lower rank is a greener (more mature) claim.
    pub fn rank(self) -> usize {
        Self::ALL
            .iter()
            .position(|l| *l == self)
            .unwrap_or(usize::MAX)
    }
}

/// The published contract form a family carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractForm {
    /// JSON-Schema-backed contract document.
    JsonSchemaBackedContractDoc,
    /// JSON Schema registry.
    JsonSchemaRegistry,
    /// Record registry.
    RecordRegistry,
    /// Event-envelope schema.
    EventEnvelopeSchema,
    /// WIT world package.
    WitWorldPackage,
    /// OpenAPI family.
    OpenapiFamily,
    /// Field set.
    FieldSet,
    /// CLI structured output.
    CliStructuredOutput,
    /// Textual interchange contract.
    TextualInterchangeContract,
    /// Asset package manifest.
    AssetPackageManifest,
    /// Teaching content pack.
    TeachingContentPack,
}

impl ContractForm {
    /// Every contract form, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::JsonSchemaBackedContractDoc,
        Self::JsonSchemaRegistry,
        Self::RecordRegistry,
        Self::EventEnvelopeSchema,
        Self::WitWorldPackage,
        Self::OpenapiFamily,
        Self::FieldSet,
        Self::CliStructuredOutput,
        Self::TextualInterchangeContract,
        Self::AssetPackageManifest,
        Self::TeachingContentPack,
    ];
}

/// A public-contract pillar a family must publish to certify a contract claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PillarKind {
    /// The published machine-readable contract form (JSON Schema / WIT / OpenAPI).
    PublishedContractForm,
    /// The explicit version field plus lifecycle label.
    LifecycleMetadata,
    /// The example payload corpus.
    ExampleCorpus,
    /// The validator suite wired into CI.
    ValidatorCoverage,
    /// The compatibility / migration report.
    CompatibilityReport,
    /// The release packet plus build identity linkage.
    ReleaseGraphLinkage,
}

impl PillarKind {
    /// Every pillar kind, in evaluation order.
    pub const ALL: [Self; 6] = [
        Self::PublishedContractForm,
        Self::LifecycleMetadata,
        Self::ExampleCorpus,
        Self::ValidatorCoverage,
        Self::CompatibilityReport,
        Self::ReleaseGraphLinkage,
    ];
}

/// The evidence state of a single pillar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceState {
    /// The pillar's evidence is published and fresh.
    Current,
    /// The pillar's evidence is due for refresh, breached, or downgraded.
    Stale,
    /// The pillar's required evidence is missing.
    Missing,
}

impl EvidenceState {
    /// Every evidence state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Current, Self::Stale, Self::Missing];
}

/// A family's overall certification state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationState {
    /// Every required pillar is current and the label matches the public claim.
    Certified,
    /// The public claim already narrowed below its marketed label; the certification inherits it.
    NarrowedRowDowngraded,
    /// A required pillar is stale; the certified claim narrows below the cutline.
    NarrowedStale,
    /// The family narrows pending a retest.
    NarrowedRetestPending,
    /// A release-blocking family is missing a required pillar; certification is withheld.
    Withheld,
}

impl CertificationState {
    /// Every certification state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Certified,
        Self::NarrowedRowDowngraded,
        Self::NarrowedStale,
        Self::NarrowedRetestPending,
        Self::Withheld,
    ];

    /// True when the family is fully certified.
    pub fn is_certified(self) -> bool {
        matches!(self, Self::Certified)
    }

    /// True when the family narrowed (any narrowed_* state).
    pub fn is_narrowed(self) -> bool {
        matches!(
            self,
            Self::NarrowedRowDowngraded | Self::NarrowedStale | Self::NarrowedRetestPending
        )
    }
}

/// A copy-safe reason a family narrowed or withheld certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationReason {
    /// The public claim already narrowed below its marketed label.
    RowDowngraded,
    /// The published contract form is missing.
    SchemaSpecPackageMissing,
    /// The lifecycle metadata is missing.
    LifecycleMetadataMissing,
    /// The example corpus is missing.
    ExampleCorpusMissing,
    /// The validator coverage is missing.
    ValidatorCoverageMissing,
    /// The compatibility report is missing.
    CompatibilityReportMissing,
    /// The release packet linkage is missing.
    ReleasePacketUnlinked,
    /// A required pillar is stale.
    EvidenceStale,
    /// A required pillar is missing.
    EvidenceMissing,
    /// A retest is pending.
    RetestPending,
    /// The mirror bundle or offline pack lacks the matching contract assets.
    MirrorParityIncomplete,
}

impl CertificationReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::RowDowngraded,
        Self::SchemaSpecPackageMissing,
        Self::LifecycleMetadataMissing,
        Self::ExampleCorpusMissing,
        Self::ValidatorCoverageMissing,
        Self::CompatibilityReportMissing,
        Self::ReleasePacketUnlinked,
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::RetestPending,
        Self::MirrorParityIncomplete,
    ];
}

/// A remediation action that clears a narrowing or hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopAction {
    /// Hold certification.
    HoldCertification,
    /// Hold promotion.
    HoldPromotion,
    /// Narrow the contract claim.
    NarrowClaim,
    /// Publish the contract form.
    PublishContractForm,
    /// Publish the lifecycle metadata.
    PublishLifecycleMetadata,
    /// Publish the example corpus.
    PublishExampleCorpus,
    /// Wire the validator coverage.
    WireValidatorCoverage,
    /// Publish the compatibility report.
    PublishCompatibilityReport,
    /// Link the release packet.
    LinkReleasePacket,
    /// Refresh stale evidence.
    RefreshEvidence,
    /// Schedule a retest.
    ScheduleRetest,
    /// Republish the mirror/offline bundle.
    RepublishMirrorBundle,
}

impl StopAction {
    /// Every stop action, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::HoldCertification,
        Self::HoldPromotion,
        Self::NarrowClaim,
        Self::PublishContractForm,
        Self::PublishLifecycleMetadata,
        Self::PublishExampleCorpus,
        Self::WireValidatorCoverage,
        Self::PublishCompatibilityReport,
        Self::LinkReleasePacket,
        Self::RefreshEvidence,
        Self::ScheduleRetest,
        Self::RepublishMirrorBundle,
    ];
}

/// The mirror/offline publication parity state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorParityState {
    /// Mirror/offline assets are current.
    Current,
    /// Mirror/offline assets are stale.
    Stale,
    /// Mirror/offline assets are unpublished.
    Unpublished,
    /// Mirror/offline publication does not apply (local-only family).
    NotApplicable,
}

impl MirrorParityState {
    /// Every mirror-parity state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Current,
        Self::Stale,
        Self::Unpublished,
        Self::NotApplicable,
    ];

    /// True when the family is publishable to mirror/offline channels.
    pub fn is_publishable(self) -> bool {
        matches!(self, Self::Current | Self::NotApplicable)
    }
}

/// A downstream surface that consumes this certification packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Claim-publication flow.
    ClaimPublication,
    /// Release-center flow.
    ReleaseCenter,
    /// Support-center flow.
    SupportCenter,
    /// SDK/docs publication flow.
    SdkDocsPublication,
}

impl ConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ClaimPublication,
        Self::ReleaseCenter,
        Self::SupportCenter,
        Self::SdkDocsPublication,
    ];
}

/// The top-level promotion decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionState {
    /// The certification packet is clear to publish.
    Proceed,
    /// A release-blocking family withheld certification; promotion is held.
    Hold,
}

impl DecisionState {
    /// Every decision state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Proceed, Self::Hold];
}

/// The per-row blocker decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerDecision {
    /// The family certifies or narrows cleanly.
    Clear,
    /// The family withholds certification and holds promotion.
    Hold,
}

/// The kind of machine-readable contract package a family ships.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityKind {
    /// JSON Schema.
    JsonSchema,
    /// OpenAPI spec.
    OpenapiSpec,
    /// WIT world.
    WitWorld,
}

/// The exact build identity the contract set rides.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIdentity {
    /// Ref to the build-identity artifact.
    pub build_identity_ref: String,
    /// The release-candidate ref.
    pub release_candidate_ref: String,
    /// The artifact-graph ref.
    pub artifact_graph_ref: String,
    /// The pinned toolchain channel.
    pub toolchain_channel: String,
    /// A note on how the build identity is resolved.
    pub note: String,
}

/// The certification cutline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchCutline {
    /// The cutline level.
    pub cutline_level: LifecycleLabel,
    /// Labels at or above the cutline.
    pub above_cutline_levels: Vec<LifecycleLabel>,
    /// Labels below the cutline.
    pub below_cutline_levels: Vec<LifecycleLabel>,
    /// A description of the cutline rule.
    pub description: String,
}

/// A certification stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human title.
    pub title: String,
    /// The reason that triggers the rule.
    pub trigger_reason: CertificationReason,
    /// The labels the rule applies to.
    pub applies_to_labels: Vec<LifecycleLabel>,
    /// The default remediation action.
    pub default_action: StopAction,
    /// True when the rule blocks promotion.
    pub blocks_promotion: bool,
    /// The rationale.
    pub rationale: String,
}

/// The machine-readable contract package identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageIdentity {
    /// The package kind.
    pub identity_kind: IdentityKind,
    /// The schema/spec id (URI or stable token).
    pub schema_or_spec_id: String,
    /// The schema/spec ref.
    pub schema_or_spec_ref: String,
    /// The package kind.
    pub package_kind: IdentityKind,
    /// The package version.
    pub package_version: u32,
    /// The in-band version field name.
    pub in_band_version_field: String,
}

/// The release-graph linkage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphLinkage {
    /// The release-candidate ref.
    pub release_candidate_ref: String,
    /// The release-packet ref.
    pub release_packet_ref: String,
    /// The build-identity ref.
    pub build_identity_ref: String,
    /// The artifact-graph node ref.
    pub artifact_graph_node_ref: String,
    /// The mirror-parity state.
    pub mirror_parity: MirrorParityState,
    /// True when offline-verifiable.
    pub offline_verifiable: bool,
}

/// A single certification pillar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pillar {
    /// The pillar kind.
    pub pillar_kind: PillarKind,
    /// The pillar title.
    pub title: String,
    /// True when the pillar is required.
    pub required: bool,
    /// The pillar's evidence state.
    pub evidence_state: EvidenceState,
    /// The upstream contract artifact that certifies this pillar.
    pub certifying_artifact_ref: String,
    /// Source refs (evidence) for the pillar.
    pub source_refs: Vec<String>,
    /// A copy-safe detail string.
    pub detail: String,
}

/// The per-row proof refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    /// Ref into the contract-health register.
    pub health_row_ref: String,
    /// Ref into the publication matrix.
    pub matrix_row_ref: String,
    /// Ref into the contract catalog.
    pub catalog_entry_ref: String,
    /// Ref to the form catalog that certifies the contract form.
    pub contract_form_catalog_ref: String,
    /// Ref to the compatibility report.
    pub compatibility_report_ref: String,
    /// The release-packet ref.
    pub release_packet_ref: String,
}

/// The per-row blocker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertBlocker {
    /// The blocker decision.
    pub decision: BlockerDecision,
    /// The pillar kinds blocking certification.
    pub blocking_pillar_kinds: Vec<PillarKind>,
    /// True when a retest is needed.
    pub retest_needed: bool,
    /// A copy-safe summary.
    pub summary: String,
}

/// A single family certification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Row {
    /// The family id.
    pub family_id: String,
    /// The family title.
    pub title: String,
    /// A copy-safe summary.
    pub summary: String,
    /// The owning package.
    pub owning_package: String,
    /// The owner DRI.
    pub owner_dri: String,
    /// The contract category.
    pub category: String,
    /// The published contract form.
    pub contract_form: ContractForm,
    /// True when the family is release-blocking.
    pub release_blocking: bool,
    /// The marketed lifecycle claim.
    pub claim_label: LifecycleLabel,
    /// The published label (post upstream narrowing).
    pub source_published_label: LifecycleLabel,
    /// The certified label.
    pub certified_label: LifecycleLabel,
    /// The contract version.
    pub contract_version: u32,
    /// The machine-readable package identity.
    pub package_identity: PackageIdentity,
    /// The release-graph linkage.
    pub graph_linkage: GraphLinkage,
    /// The mirror-parity state (mirrors `graph_linkage.mirror_parity`).
    pub mirror_parity: MirrorParityState,
    /// The certification pillars, one per kind.
    pub pillars: Vec<Pillar>,
    /// The certification state.
    pub certification_state: CertificationState,
    /// The active certification reasons.
    pub active_certification_reasons: Vec<CertificationReason>,
    /// The active stop actions.
    pub stop_actions: Vec<StopAction>,
    /// The proof refs.
    pub proof: Proof,
    /// The per-row blocker.
    pub blocker: CertBlocker,
    /// The rationale.
    pub rationale: String,
}

impl Row {
    /// Recomputes the certification state from the row's own pillars and labels.
    ///
    /// Mirrors the regenerator's derivation from the upstream contract-health state.
    pub fn computed_certification_state(&self) -> CertificationState {
        let any_missing = self
            .pillars
            .iter()
            .any(|p| p.required && p.evidence_state == EvidenceState::Missing);
        let any_stale = self
            .pillars
            .iter()
            .any(|p| p.required && p.evidence_state == EvidenceState::Stale);
        let downgraded = self.source_published_label.rank() > self.claim_label.rank();
        let retest = self.blocker.retest_needed;

        if self.release_blocking && any_missing {
            CertificationState::Withheld
        } else if any_stale || any_missing {
            if downgraded {
                CertificationState::NarrowedRowDowngraded
            } else if retest {
                CertificationState::NarrowedRetestPending
            } else {
                CertificationState::NarrowedStale
            }
        } else if downgraded {
            CertificationState::NarrowedRowDowngraded
        } else {
            CertificationState::Certified
        }
    }

    /// Recomputes the per-row blocker decision from the certification state.
    pub fn computed_blocker_decision(&self) -> BlockerDecision {
        if self.computed_certification_state() == CertificationState::Withheld {
            BlockerDecision::Hold
        } else {
            BlockerDecision::Clear
        }
    }

    /// True when the certified label is greener (more mature) than the public claim.
    pub fn certified_label_greener_than_claim(&self) -> bool {
        self.certified_label.rank() < self.claim_label.rank()
    }
}

/// The top-level promotion decision block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Promotion {
    /// The promotion-gate name.
    pub promotion_gate: String,
    /// The decision.
    pub decision: DecisionState,
    /// The withheld release-blocking family ids.
    pub blocking_family_ids: Vec<String>,
    /// The rationale.
    pub rationale: String,
}

/// Summary counts recomputed from the rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Summary {
    /// Total claimed families.
    pub total_families: usize,
    /// Release-blocking families.
    pub release_blocking_families: usize,
    /// Certified families.
    pub certified_families: usize,
    /// Narrowed families (any narrowed_* state).
    pub narrowed_families: usize,
    /// Withheld families.
    pub withheld_families: usize,
    /// Families holding promotion.
    pub families_held: usize,
    /// Families certifying below their marketed claim.
    pub families_narrowed_below_claim: usize,
    /// Families publishable to mirror/offline channels.
    pub mirror_publishable_families: usize,
    /// Total pillars evaluated.
    pub total_pillars_evaluated: usize,
    /// Current pillars.
    pub pillars_current: usize,
    /// Stale pillars.
    pub pillars_stale: usize,
    /// Missing pillars.
    pub pillars_missing: usize,
}

/// The full M5 public-contract certification register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicContractCertificationRegister {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Register id.
    pub register_id: String,
    /// Status.
    pub status: String,
    /// Current-as-of date.
    pub as_of: String,
    /// Overview page.
    pub overview_page: String,
    /// Evidence page.
    pub evidence_page: String,
    /// Help page.
    pub help_page: String,
    /// Report page.
    pub report_page: String,
    /// Shiproom dashboard page.
    pub shiproom_dashboard_page: String,
    /// Ref to the contract-health register.
    pub contract_health_ref: String,
    /// Ref to the publication matrix.
    pub publication_matrix_ref: String,
    /// Ref to the contract catalog.
    pub contract_catalog_ref: String,
    /// Ref to the JSON Schema catalog.
    pub json_schema_catalog_ref: String,
    /// Ref to the OpenAPI catalog.
    pub openapi_catalog_ref: String,
    /// Ref to the WIT publication.
    pub wit_publication_ref: String,
    /// Ref to the reader/writer compatibility suite.
    pub reader_writer_compat_ref: String,
    /// Ref to the interchange-conformance register.
    pub interchange_conformance_ref: String,
    /// Ref to the canonical M5 evidence index.
    pub evidence_index_ref: String,
    /// Ref to the build-identity artifact.
    pub build_identity_ref: String,
    /// The build identity the contract set rides.
    pub build_identity: BuildIdentity,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<LifecycleLabel>,
    /// Closed contract-form vocabulary.
    pub contract_forms: Vec<ContractForm>,
    /// Closed pillar-kind vocabulary.
    pub pillar_kinds: Vec<PillarKind>,
    /// Closed evidence-state vocabulary.
    pub evidence_states: Vec<EvidenceState>,
    /// Closed certification-state vocabulary.
    pub certification_states: Vec<CertificationState>,
    /// Closed certification-reason vocabulary.
    pub certification_reasons: Vec<CertificationReason>,
    /// Closed stop-action vocabulary.
    pub stop_actions: Vec<StopAction>,
    /// Closed mirror-parity vocabulary.
    pub mirror_parity_states: Vec<MirrorParityState>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<ConsumerSurface>,
    /// Closed decision-state vocabulary.
    pub decision_states: Vec<DecisionState>,
    /// The certification cutline.
    pub launch_cutline: LaunchCutline,
    /// The release-blocking family ids.
    pub release_blocking_family_refs: Vec<String>,
    /// The certification stop rules.
    pub stop_rules: Vec<StopRule>,
    /// The certification rows.
    pub rows: Vec<Row>,
    /// The promotion decision.
    pub promotion: Promotion,
    /// Summary counts.
    pub summary: Summary,
}

/// An export/inspect-safe projection of one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportRow {
    /// The family id.
    pub family_id: String,
    /// The marketed claim.
    pub claim_label: LifecycleLabel,
    /// The certified label.
    pub certified_label: LifecycleLabel,
    /// The certification state.
    pub certification_state: CertificationState,
    /// The per-row blocker decision.
    pub decision: BlockerDecision,
    /// True when release-blocking.
    pub release_blocking: bool,
    /// The active certification reasons.
    pub active_certification_reasons: Vec<CertificationReason>,
}

/// An export/inspect-safe projection of the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportProjection {
    /// The register id.
    pub register_id: String,
    /// The current-as-of date.
    pub as_of: String,
    /// The promotion decision.
    pub decision: DecisionState,
    /// The projected rows.
    pub rows: Vec<ExportRow>,
}

/// A structural-invariant violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    /// The check id.
    pub check_id: String,
    /// A human detail.
    pub detail: String,
}

impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.check_id, self.detail)
    }
}

impl M5PublicContractCertificationRegister {
    /// Returns the row registered for `family_id`.
    pub fn row(&self, family_id: &str) -> Option<&Row> {
        self.rows.iter().find(|r| r.family_id == family_id)
    }

    /// Resolves the certified label, certification state, and per-row decision for a family.
    /// This is the lookup claim-publication, release-center, support-center, and SDK/docs
    /// publication share.
    pub fn resolve_certification(
        &self,
        family_id: &str,
    ) -> Option<(LifecycleLabel, CertificationState, BlockerDecision)> {
        self.row(family_id)
            .map(|r| (r.certified_label, r.certification_state, r.blocker.decision))
    }

    /// Families that withheld certification.
    pub fn withheld_rows(&self) -> Vec<&Row> {
        self.rows
            .iter()
            .filter(|r| r.certification_state == CertificationState::Withheld)
            .collect()
    }

    /// True when the certification packet holds promotion.
    pub fn holds_promotion(&self) -> bool {
        self.promotion.decision == DecisionState::Hold
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> Summary {
        let count = |f: &dyn Fn(&Row) -> bool| self.rows.iter().filter(|r| f(r)).count();
        let all_pillars = || self.rows.iter().flat_map(|r| &r.pillars);
        Summary {
            total_families: self.rows.len(),
            release_blocking_families: count(&|r| r.release_blocking),
            certified_families: count(&|r| r.certification_state == CertificationState::Certified),
            narrowed_families: count(&|r| r.certification_state.is_narrowed()),
            withheld_families: count(&|r| r.certification_state == CertificationState::Withheld),
            families_held: count(&|r| r.blocker.decision == BlockerDecision::Hold),
            families_narrowed_below_claim: count(&|r| {
                r.certified_label.rank() > r.claim_label.rank()
            }),
            mirror_publishable_families: count(&|r| r.mirror_parity.is_publishable()),
            total_pillars_evaluated: all_pillars().count(),
            pillars_current: all_pillars()
                .filter(|p| p.evidence_state == EvidenceState::Current)
                .count(),
            pillars_stale: all_pillars()
                .filter(|p| p.evidence_state == EvidenceState::Stale)
                .count(),
            pillars_missing: all_pillars()
                .filter(|p| p.evidence_state == EvidenceState::Missing)
                .count(),
        }
    }

    /// Produces an export/inspect-safe projection downstream surfaces render instead of
    /// cloning register text.
    pub fn support_export_projection(&self) -> ExportProjection {
        ExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|r| ExportRow {
                    family_id: r.family_id.clone(),
                    claim_label: r.claim_label,
                    certified_label: r.certified_label,
                    certification_state: r.certification_state,
                    decision: r.blocker.decision,
                    release_blocking: r.release_blocking,
                    active_certification_reasons: r.active_certification_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register's structural invariants.
    ///
    /// Mirrors the CI validator's semantic invariants. The checked-in register returns no
    /// violations; each structurally-parseable negative fixture returns at least one.
    pub fn validate(&self) -> Vec<Violation> {
        let mut out = Vec::new();
        let mut push = |check_id: &str, detail: String| {
            out.push(Violation {
                check_id: check_id.to_string(),
                detail,
            })
        };

        if self.schema_version != M5_PUBLIC_CONTRACT_CERTIFICATION_SCHEMA_VERSION {
            push(
                "register.schema_version",
                format!("unexpected schema_version {}", self.schema_version),
            );
        }
        if self.record_kind != M5_PUBLIC_CONTRACT_CERTIFICATION_RECORD_KIND {
            push(
                "register.record_kind",
                format!("unexpected record_kind {}", self.record_kind),
            );
        }
        if self.register_id != M5_PUBLIC_CONTRACT_CERTIFICATION_REGISTER_ID {
            push(
                "register.register_id",
                format!("unexpected register_id {}", self.register_id),
            );
        }

        if self.lifecycle_labels != LifecycleLabel::ALL {
            push("vocab.lifecycle_labels", "off the canonical list".into());
        }
        if self.contract_forms != ContractForm::ALL {
            push("vocab.contract_forms", "off the canonical list".into());
        }
        if self.pillar_kinds != PillarKind::ALL {
            push("vocab.pillar_kinds", "off the canonical list".into());
        }
        if self.evidence_states != EvidenceState::ALL {
            push("vocab.evidence_states", "off the canonical list".into());
        }
        if self.certification_states != CertificationState::ALL {
            push(
                "vocab.certification_states",
                "off the canonical list".into(),
            );
        }
        if self.certification_reasons != CertificationReason::ALL {
            push(
                "vocab.certification_reasons",
                "off the canonical list".into(),
            );
        }
        if self.stop_actions != StopAction::ALL {
            push("vocab.stop_actions", "off the canonical list".into());
        }
        if self.mirror_parity_states != MirrorParityState::ALL {
            push(
                "vocab.mirror_parity_states",
                "off the canonical list".into(),
            );
        }
        if self.consumer_surfaces != ConsumerSurface::ALL {
            push("vocab.consumer_surfaces", "off the canonical list".into());
        }
        if self.decision_states != DecisionState::ALL {
            push("vocab.decision_states", "off the canonical list".into());
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.family_id.as_str()) {
                push(
                    "rows.duplicate_family_id",
                    format!("duplicate family_id {}", row.family_id),
                );
            }

            let kinds: Vec<PillarKind> = row.pillars.iter().map(|p| p.pillar_kind).collect();
            if kinds != PillarKind::ALL.to_vec() {
                push(
                    "rows.pillar_coverage",
                    format!(
                        "{}: pillars must be exactly the pillar-kind set in order",
                        row.family_id
                    ),
                );
            }

            let expected_state = row.computed_certification_state();
            if row.certification_state != expected_state {
                push(
                    "rows.certification_state",
                    format!(
                        "{}: certification_state disagrees with the pillars",
                        row.family_id
                    ),
                );
            }

            if row.certified_label_greener_than_claim() {
                push(
                    "rows.claim_parity",
                    format!(
                        "{}: certified label is greener than the public claim",
                        row.family_id
                    ),
                );
            }

            let expected_blocker = row.computed_blocker_decision();
            if row.blocker.decision != expected_blocker {
                push(
                    "rows.blocker_decision",
                    format!(
                        "{}: blocker decision disagrees with the certification state",
                        row.family_id
                    ),
                );
            }
        }

        // Top-level promotion decision recomputed from the withheld, release-blocking rows.
        let blocking: Vec<String> = self
            .rows
            .iter()
            .filter(|r| {
                r.computed_certification_state() == CertificationState::Withheld
                    && r.release_blocking
            })
            .map(|r| r.family_id.clone())
            .collect();
        if self.promotion.blocking_family_ids != blocking {
            push(
                "promotion.block",
                "blocking_family_ids disagree with the withheld rows".into(),
            );
        }
        let expected_decision = if blocking.is_empty() {
            DecisionState::Proceed
        } else {
            DecisionState::Hold
        };
        if self.promotion.decision != expected_decision {
            push(
                "promotion.decision",
                "top-level decision disagrees with the withheld rows".into(),
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
pub fn current_m5_public_contract_certification_register(
) -> Result<M5PublicContractCertificationRegister, serde_json::Error> {
    serde_json::from_str(M5_PUBLIC_CONTRACT_CERTIFICATION_JSON)
}

#[cfg(test)]
mod tests;
