//! Typed M5 public-contract publication matrix: the canonical inventory of every
//! M5 artifact family the source docs treat as a published contract.
//!
//! Where the depth-claim manifest speaks for the *depth claim* each feature
//! family publishes, the qualification matrix speaks for the *compatibility
//! boundary* each surface exposes, and the certification-train evidence index
//! ties those together, this matrix speaks for the *contract publication* of each
//! family: whether it has published the contract forms (JSON Schema, WIT,
//! OpenAPI, Markdown summary, example payloads, migration notes), the validator
//! suite, and the release-packet linkage required before it may carry a Stable
//! contract claim. Each [`M5PublicContractRow`] binds one family to:
//!
//! - its contract form ([`ContractForm`]), stability lane ([`MaturityLane`]),
//!   reader/writer posture ([`ReaderWriterPosture`]), and packaging need
//!   ([`PackagingNeed`]),
//! - the lifecycle label it is put forward at ([`M5PublicContractRow::claim_label`])
//!   and the label it effectively publishes after narrowing
//!   ([`M5PublicContractRow::published_label`]),
//! - one [`PublicationRequirement`] per [`PublicationArtifactKind`], recording
//!   whether the form is required before promotion and its [`PublicationState`],
//! - the active [`GapReason`]s and the overall [`RowState`].
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between a family that may publish a Stable contract claim and one that must
//! narrow below it: a family missing any *required* publication evidence raises
//! the matching [`GapReason`] and narrows. The [`M5PublicContractStopRule`] set
//! names the closed conditions that gate promotion — one per [`GapReason`] — and
//! [`M5PublicContractMatrix::promotion`] records the proceed/hold verdict.
//!
//! The matrix is checked in at
//! `artifacts/contracts/m5-stability-lifecycle-map.json` and embedded here, so
//! this typed consumer and the CI validator agree on every family without a cargo
//! build in CI. The model is metadata-only: every field is a typed state or an
//! opaque repo-relative ref. It carries no surface payloads, rendered bodies,
//! signatures, or credential material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_matrix::{
    LaunchCutline, PromotionDecision, PromotionDecisionRecord, StableClaimLevel,
};

/// Supported matrix schema version.
pub const M5_PUBLIC_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the matrix.
pub const M5_PUBLIC_CONTRACT_RECORD_KIND: &str = "m5_public_contract_matrix";

/// Repo-relative path to the checked-in matrix.
pub const M5_PUBLIC_CONTRACT_PATH: &str = "artifacts/contracts/m5-stability-lifecycle-map.json";

/// Embedded checked-in matrix JSON.
pub const M5_PUBLIC_CONTRACT_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/contracts/m5-stability-lifecycle-map.json"
));

/// The published contract form a family carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractForm {
    /// A JSON-Schema-backed contract document.
    JsonSchemaBackedContractDoc,
    /// A registry of JSON Schemas.
    JsonSchemaRegistry,
    /// A registry of typed records.
    RecordRegistry,
    /// An event-envelope schema.
    EventEnvelopeSchema,
    /// A component-model WIT world package.
    WitWorldPackage,
    /// An OpenAPI specification family.
    OpenapiFamily,
    /// A field set (a stable set of fields without a full schema doc).
    FieldSet,
    /// CLI/headless structured output.
    CliStructuredOutput,
    /// A textual interchange contract.
    TextualInterchangeContract,
    /// An asset-package manifest.
    AssetPackageManifest,
    /// A teaching content pack.
    TeachingContentPack,
}

impl ContractForm {
    /// Every form, in declaration order.
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

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JsonSchemaBackedContractDoc => "json_schema_backed_contract_doc",
            Self::JsonSchemaRegistry => "json_schema_registry",
            Self::RecordRegistry => "record_registry",
            Self::EventEnvelopeSchema => "event_envelope_schema",
            Self::WitWorldPackage => "wit_world_package",
            Self::OpenapiFamily => "openapi_family",
            Self::FieldSet => "field_set",
            Self::CliStructuredOutput => "cli_structured_output",
            Self::TextualInterchangeContract => "textual_interchange_contract",
            Self::AssetPackageManifest => "asset_package_manifest",
            Self::TeachingContentPack => "teaching_content_pack",
        }
    }
}

/// The category grouping a family belongs to (reused compatibility-surface lexicon).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractCategory {
    /// Settings and profile.
    SettingsAndProfile,
    /// Workspace and state.
    WorkspaceAndState,
    /// Extensions and host.
    ExtensionsAndHost,
    /// Command and automation.
    CommandAndAutomation,
    /// AI and language.
    AiAndLanguage,
    /// Editor and text.
    EditorAndText,
    /// Terminal and run.
    TerminalAndRun,
    /// Debug and diagnostics.
    DebugAndDiagnostics,
    /// Merge and history.
    MergeAndHistory,
    /// Portability and migration.
    PortabilityAndMigration,
    /// Locale and translation.
    LocaleAndTranslation,
    /// Design and theme.
    DesignAndTheme,
    /// Accessibility and input.
    AccessibilityAndInput,
    /// Voice and consent.
    VoiceAndConsent,
    /// Service and API.
    ServiceAndApi,
    /// Review and hosted.
    ReviewAndHosted,
    /// Release and build.
    ReleaseAndBuild,
    /// Support and export.
    SupportAndExport,
    /// Governance and policy.
    GovernanceAndPolicy,
    /// Docs and teaching.
    DocsAndTeaching,
    /// Notification and attention.
    NotificationAndAttention,
    /// Certification and reference.
    CertificationAndReference,
}

impl ContractCategory {
    /// Every category, in declaration order.
    pub const ALL: [Self; 22] = [
        Self::SettingsAndProfile,
        Self::WorkspaceAndState,
        Self::ExtensionsAndHost,
        Self::CommandAndAutomation,
        Self::AiAndLanguage,
        Self::EditorAndText,
        Self::TerminalAndRun,
        Self::DebugAndDiagnostics,
        Self::MergeAndHistory,
        Self::PortabilityAndMigration,
        Self::LocaleAndTranslation,
        Self::DesignAndTheme,
        Self::AccessibilityAndInput,
        Self::VoiceAndConsent,
        Self::ServiceAndApi,
        Self::ReviewAndHosted,
        Self::ReleaseAndBuild,
        Self::SupportAndExport,
        Self::GovernanceAndPolicy,
        Self::DocsAndTeaching,
        Self::NotificationAndAttention,
        Self::CertificationAndReference,
    ];
}

/// The contract-family registry maturity lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaturityLane {
    /// Stable and claim-bearing.
    Stable,
    /// Beta and claim-bearing.
    Beta,
    /// Seeded but not yet stable.
    Experimental,
    /// Internal-only machine-readable surface.
    Internal,
}

impl MaturityLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 4] = [Self::Stable, Self::Beta, Self::Experimental, Self::Internal];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Experimental => "experimental",
            Self::Internal => "internal",
        }
    }
}

/// The reader/writer posture of a contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReaderWriterPosture {
    /// Read-only contract.
    ReaderOnly,
    /// Write-only contract.
    WriterOnly,
    /// Both produced and consumed within the product.
    ReadWrite,
    /// Bidirectional interchange across a boundary (import and export).
    BidirectionalInterchange,
}

impl ReaderWriterPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReaderOnly,
        Self::WriterOnly,
        Self::ReadWrite,
        Self::BidirectionalInterchange,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReaderOnly => "reader_only",
            Self::WriterOnly => "writer_only",
            Self::ReadWrite => "read_write",
            Self::BidirectionalInterchange => "bidirectional_interchange",
        }
    }
}

/// The mirror/offline packaging need of a contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackagingNeed {
    /// Local-only; no mirror or managed packaging needed.
    LocalOnly,
    /// Must be mirror-packaged for offline/air-gapped use.
    Mirrored,
    /// Managed-service packaged.
    Managed,
    /// Reached through a connected-provider browser handoff.
    BrowserHandoff,
}

impl PackagingNeed {
    /// Every need, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalOnly,
        Self::Mirrored,
        Self::Managed,
        Self::BrowserHandoff,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Mirrored => "mirrored",
            Self::Managed => "managed",
            Self::BrowserHandoff => "browser_handoff",
        }
    }
}

/// A contract form a family may have to publish before promotion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationArtifactKind {
    /// A JSON Schema.
    JsonSchema,
    /// A WIT world package.
    WitWorld,
    /// An OpenAPI specification.
    OpenapiSpec,
    /// A Markdown contract summary.
    MarkdownSummary,
    /// Example payloads / corpus.
    ExamplePayloads,
    /// Migration / deprecation notes.
    MigrationNotes,
    /// A validator suite.
    ValidatorSuite,
}

impl PublicationArtifactKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::JsonSchema,
        Self::WitWorld,
        Self::OpenapiSpec,
        Self::MarkdownSummary,
        Self::ExamplePayloads,
        Self::MigrationNotes,
        Self::ValidatorSuite,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JsonSchema => "json_schema",
            Self::WitWorld => "wit_world",
            Self::OpenapiSpec => "openapi_spec",
            Self::MarkdownSummary => "markdown_summary",
            Self::ExamplePayloads => "example_payloads",
            Self::MigrationNotes => "migration_notes",
            Self::ValidatorSuite => "validator_suite",
        }
    }

    /// The gap reason raised when this required form is not published.
    pub const fn unpublished_gap(self) -> GapReason {
        match self {
            Self::JsonSchema => GapReason::JsonSchemaUnpublished,
            Self::WitWorld => GapReason::WitWorldUnpublished,
            Self::OpenapiSpec => GapReason::OpenapiSpecUnpublished,
            Self::MarkdownSummary => GapReason::MarkdownSummaryUnpublished,
            Self::ExamplePayloads => GapReason::ExamplePayloadsUnpublished,
            Self::MigrationNotes => GapReason::MigrationNotesUnpublished,
            Self::ValidatorSuite => GapReason::ValidatorSuiteUnpublished,
        }
    }
}

/// The publication state of one contract form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationState {
    /// Fully published.
    Published,
    /// Partially published (for example a seed only).
    Partial,
    /// Required but missing.
    Missing,
    /// Not applicable to this contract form.
    NotApplicable,
}

impl PublicationState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Published,
        Self::Partial,
        Self::Missing,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::Partial => "partial",
            Self::Missing => "missing",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this state satisfies a required publication.
    pub const fn is_published(self) -> bool {
        matches!(self, Self::Published)
    }
}

/// A closed reason a family's contract claim narrows below the cutline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapReason {
    /// The required JSON Schema is unpublished.
    JsonSchemaUnpublished,
    /// The required WIT world is unpublished.
    WitWorldUnpublished,
    /// The required OpenAPI spec is unpublished.
    OpenapiSpecUnpublished,
    /// The required Markdown summary is unpublished.
    MarkdownSummaryUnpublished,
    /// The required example payloads are unpublished.
    ExamplePayloadsUnpublished,
    /// The required migration notes are unpublished.
    MigrationNotesUnpublished,
    /// The required validator suite is unwired.
    ValidatorSuiteUnpublished,
    /// The release packet is unlinked.
    ReleasePacketUnlinked,
}

impl GapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::JsonSchemaUnpublished,
        Self::WitWorldUnpublished,
        Self::OpenapiSpecUnpublished,
        Self::MarkdownSummaryUnpublished,
        Self::ExamplePayloadsUnpublished,
        Self::MigrationNotesUnpublished,
        Self::ValidatorSuiteUnpublished,
        Self::ReleasePacketUnlinked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JsonSchemaUnpublished => "json_schema_unpublished",
            Self::WitWorldUnpublished => "wit_world_unpublished",
            Self::OpenapiSpecUnpublished => "openapi_spec_unpublished",
            Self::MarkdownSummaryUnpublished => "markdown_summary_unpublished",
            Self::ExamplePayloadsUnpublished => "example_payloads_unpublished",
            Self::MigrationNotesUnpublished => "migration_notes_unpublished",
            Self::ValidatorSuiteUnpublished => "validator_suite_unpublished",
            Self::ReleasePacketUnlinked => "release_packet_unlinked",
        }
    }

    /// The default remediation action a stop rule prescribes for this reason.
    pub const fn action(self) -> RemediationAction {
        match self {
            Self::ExamplePayloadsUnpublished => RemediationAction::PublishExamplePayloads,
            Self::ValidatorSuiteUnpublished => RemediationAction::WireValidatorSuite,
            Self::ReleasePacketUnlinked => RemediationAction::LinkReleasePacket,
            _ => RemediationAction::PublishContractForm,
        }
    }
}

/// The remediation action a stop rule prescribes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemediationAction {
    /// Hold promotion until the gap clears.
    HoldPromotion,
    /// Narrow the claim below the cutline.
    NarrowLabel,
    /// Publish the missing contract form.
    PublishContractForm,
    /// Publish the missing example payloads.
    PublishExamplePayloads,
    /// Wire the missing validator suite.
    WireValidatorSuite,
    /// Link the missing release packet.
    LinkReleasePacket,
}

impl RemediationAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HoldPromotion,
        Self::NarrowLabel,
        Self::PublishContractForm,
        Self::PublishExamplePayloads,
        Self::WireValidatorSuite,
        Self::LinkReleasePacket,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowLabel => "narrow_label",
            Self::PublishContractForm => "publish_contract_form",
            Self::PublishExamplePayloads => "publish_example_payloads",
            Self::WireValidatorSuite => "wire_validator_suite",
            Self::LinkReleasePacket => "link_release_packet",
        }
    }
}

/// The overall publication state of a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowState {
    /// All required forms are published; the family holds its claim label.
    Published,
    /// A required form is unpublished; the family narrows below the cutline.
    Narrowed,
}

impl RowState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Published, Self::Narrowed];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::Narrowed => "narrowed",
        }
    }
}

/// One contract-form publication requirement cell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationRequirement {
    /// The contract form this cell speaks for.
    pub artifact_kind: PublicationArtifactKind,
    /// Whether the form must be published before promotion.
    pub required: bool,
    /// The publication state earned.
    pub state: PublicationState,
    /// Refs to the published artifacts. Empty only on missing/not-applicable cells.
    #[serde(default)]
    pub refs: Vec<String>,
}

/// One matrix stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PublicContractStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched family fires this rule.
    pub trigger_reason: GapReason,
    /// Lifecycle labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default remediation action.
    pub default_action: RemediationAction,
    /// Whether firing this rule blocks promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One contract-family row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PublicContractRow {
    /// Stable family id (links to the contract-family registry).
    pub family_id: String,
    /// Human-readable title.
    pub title: String,
    /// Reviewable one-line summary.
    pub summary: String,
    /// The owning crate or lane.
    pub owning_package: String,
    /// Owner DRI handle.
    pub owner_dri: String,
    /// The category grouping.
    pub category: ContractCategory,
    /// The contract form.
    pub contract_form: ContractForm,
    /// The contract-family registry maturity lane.
    pub maturity_lane: MaturityLane,
    /// The reader/writer posture.
    pub reader_writer_posture: ReaderWriterPosture,
    /// The mirror/offline packaging need.
    pub packaging_need: PackagingNeed,
    /// The lifecycle label the family is put forward at.
    pub claim_label: StableClaimLevel,
    /// The lifecycle label the family effectively publishes after narrowing.
    pub published_label: StableClaimLevel,
    /// The overall row state.
    pub row_state: RowState,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// Ref into the contract-family registry.
    pub contract_family_ref: String,
    /// Ref into the compatibility-surface inventory.
    pub compatibility_surface_ref: String,
    /// Optional ref into the qualification matrix.
    #[serde(default)]
    pub qualification_row_ref: Option<String>,
    /// Release-packet dependency (claim manifest, qualification row, or evidence index).
    pub release_packet_ref: String,
    /// Refs to the example corpus.
    #[serde(default)]
    pub example_corpus_refs: Vec<String>,
    /// Refs to the validator suite.
    #[serde(default)]
    pub validator_suite_refs: Vec<String>,
    /// One publication requirement per contract form.
    pub publication_requirements: Vec<PublicationRequirement>,
    /// Active gap reasons narrowing the family.
    #[serde(default)]
    pub active_gap_reasons: Vec<GapReason>,
    /// Publication destinations that render this row.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl M5PublicContractRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the claim's label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when a gap reason is active on the row.
    pub fn has_gap(&self, reason: GapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// Returns the requirement cell for `kind`, if any.
    pub fn requirement(&self, kind: PublicationArtifactKind) -> Option<&PublicationRequirement> {
        self.publication_requirements
            .iter()
            .find(|cell| cell.artifact_kind == kind)
    }

    /// Recomputes the active gap reasons from the publication requirements and the
    /// release-packet linkage, in canonical [`GapReason::ALL`] order.
    pub fn computed_gap_reasons(&self) -> Vec<GapReason> {
        let mut found: BTreeSet<GapReason> = BTreeSet::new();
        for cell in &self.publication_requirements {
            if cell.required && !cell.state.is_published() {
                found.insert(cell.artifact_kind.unpublished_gap());
            }
        }
        if self.release_packet_ref.trim().is_empty() {
            found.insert(GapReason::ReleasePacketUnlinked);
        }
        GapReason::ALL
            .into_iter()
            .filter(|reason| found.contains(reason))
            .collect()
    }
}

/// The lifecycle label a gapped family narrows to: one step below the cutline.
pub fn narrow_floor(claim: StableClaimLevel) -> StableClaimLevel {
    if claim.is_at_or_above_cutline() {
        return StableClaimLevel::Beta;
    }
    match claim {
        StableClaimLevel::Beta => StableClaimLevel::Preview,
        StableClaimLevel::Preview | StableClaimLevel::Withdrawn => StableClaimLevel::Withdrawn,
        // Already handled by the cutline branch, but keep the match total.
        StableClaimLevel::Lts | StableClaimLevel::Stable => StableClaimLevel::Beta,
    }
}

/// Summary counts carried by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PublicContractSummary {
    /// Total rows.
    pub total_rows: usize,
    /// Distinct families.
    pub total_families: usize,
    /// Rows publishing at their claim label.
    pub rows_published: usize,
    /// Rows narrowed below their claim label.
    pub rows_narrowed: usize,
    /// Release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows publishing at their claim label.
    pub release_blocking_published: usize,
    /// Release-blocking rows narrowed.
    pub release_blocking_narrowed: usize,
    /// Rows in the stable maturity lane.
    pub stable_lane_rows: usize,
    /// Rows in the beta maturity lane.
    pub beta_lane_rows: usize,
    /// Rows in the experimental maturity lane.
    pub experimental_lane_rows: usize,
    /// Rows in the internal maturity lane.
    pub internal_lane_rows: usize,
    /// Rows requiring a JSON Schema.
    pub rows_requiring_json_schema: usize,
    /// Rows requiring a WIT world.
    pub rows_requiring_wit_world: usize,
    /// Rows requiring an OpenAPI spec.
    pub rows_requiring_openapi_spec: usize,
    /// Rows requiring a Markdown summary.
    pub rows_requiring_markdown_summary: usize,
    /// Rows requiring example payloads.
    pub rows_requiring_example_payloads: usize,
    /// Rows requiring migration notes.
    pub rows_requiring_migration_notes: usize,
    /// Rows requiring a validator suite.
    pub rows_requiring_validator_suite: usize,
    /// Total required publications across all rows.
    pub total_required_publications: usize,
    /// Total required publications that are published.
    pub total_published_publications: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Rows carrying at least one active gap reason.
    pub rows_with_active_gap: usize,
    /// Stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicContractExportRow {
    /// Stable family id.
    pub family_id: String,
    /// The contract form.
    pub contract_form: ContractForm,
    /// The maturity lane.
    pub maturity_lane: MaturityLane,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The claim label.
    pub claim_label: StableClaimLevel,
    /// The effective published label.
    pub published_label: StableClaimLevel,
    /// Whether the family publishes at or above the cutline.
    pub publishes_stable: bool,
    /// The overall row state.
    pub row_state: RowState,
    /// The release-packet dependency.
    pub release_packet_ref: String,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<GapReason>,
}

/// Export projection for Help/About, SDK/docs, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicContractExportProjection {
    /// Matrix identifier.
    pub matrix_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5PublicContractExportRow>,
}

/// The typed M5 public-contract publication matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PublicContractMatrix {
    /// Matrix schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable matrix identifier.
    pub matrix_id: String,
    /// Lifecycle status of this matrix artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the claim manifest this matrix threads into.
    pub claim_manifest_ref: String,
    /// Ref to the contract-family registry.
    pub contract_family_registry_ref: String,
    /// Ref to the compatibility-surface inventory.
    pub compatibility_surface_inventory_ref: String,
    /// Ref to the qualification matrix.
    pub qualification_matrix_ref: String,
    /// Ref to the canonical M5 evidence index.
    pub evidence_index_ref: String,
    /// Closed contract-form vocabulary.
    pub contract_forms: Vec<ContractForm>,
    /// Closed category vocabulary.
    pub contract_categories: Vec<ContractCategory>,
    /// Closed maturity-lane vocabulary.
    pub maturity_lanes: Vec<MaturityLane>,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed reader/writer-posture vocabulary.
    pub reader_writer_postures: Vec<ReaderWriterPosture>,
    /// Closed packaging-need vocabulary.
    pub packaging_needs: Vec<PackagingNeed>,
    /// Closed publication-artifact-kind vocabulary.
    pub publication_artifact_kinds: Vec<PublicationArtifactKind>,
    /// Closed publication-state vocabulary.
    pub publication_states: Vec<PublicationState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<GapReason>,
    /// Closed remediation-action vocabulary.
    pub remediation_actions: Vec<RemediationAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking family ids the matrix must cover.
    pub release_blocking_family_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<M5PublicContractStopRule>,
    /// Contract-family rows.
    pub rows: Vec<M5PublicContractRow>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5PublicContractSummary,
}

impl M5PublicContractMatrix {
    /// Returns the row registered for `family_id`.
    pub fn row(&self, family_id: &str) -> Option<&M5PublicContractRow> {
        self.rows.iter().find(|row| row.family_id == family_id)
    }

    /// Rows publishing at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&M5PublicContractRow> {
        self.rows.iter().filter(|r| r.publishes_stable()).collect()
    }

    /// Rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&M5PublicContractRow> {
        self.rows.iter().filter(|r| !r.publishes_stable()).collect()
    }

    /// Release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&M5PublicContractRow> {
        self.rows.iter().filter(|r| r.release_blocking).collect()
    }

    /// Rows whose contract form is `form`.
    pub fn rows_for_form(&self, form: ContractForm) -> Vec<&M5PublicContractRow> {
        self.rows
            .iter()
            .filter(|r| r.contract_form == form)
            .collect()
    }

    /// True when `rule` fires: a watched family carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &M5PublicContractStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label) && row.has_gap(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the rows and stop rules.
    pub fn computed_promotion_decision(&self) -> PromotionDecision {
        if self
            .stop_rules
            .iter()
            .any(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Blocking, firing stop-rule ids, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Family ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only families whose claim is at or above the cutline count: a family whose
    /// claim is already below the cutline is not a *promotion* blocker.
    pub fn computed_blocking_family_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<GapReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row.claim_holds_stable()
                && row
                    .active_gap_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.family_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    fn required_kind_count(&self, kind: PublicationArtifactKind) -> usize {
        self.rows
            .iter()
            .filter(|row| {
                row.requirement(kind)
                    .map(|cell| cell.required)
                    .unwrap_or(false)
            })
            .count()
    }

    /// Recomputes the summary block from the rows and stop rules.
    pub fn computed_summary(&self) -> M5PublicContractSummary {
        let lane = |lane: MaturityLane| {
            self.rows
                .iter()
                .filter(|row| row.maturity_lane == lane)
                .count()
        };
        let release_blocking = self.release_blocking_rows();
        let mut families: BTreeSet<&str> = BTreeSet::new();
        for row in &self.rows {
            families.insert(row.family_id.as_str());
        }
        let total_required: usize = self
            .rows
            .iter()
            .flat_map(|row| row.publication_requirements.iter())
            .filter(|cell| cell.required)
            .count();
        let total_published: usize = self
            .rows
            .iter()
            .flat_map(|row| row.publication_requirements.iter())
            .filter(|cell| cell.required && cell.state.is_published())
            .count();
        M5PublicContractSummary {
            total_rows: self.rows.len(),
            total_families: families.len(),
            rows_published: self
                .rows
                .iter()
                .filter(|row| row.row_state == RowState::Published)
                .count(),
            rows_narrowed: self
                .rows
                .iter()
                .filter(|row| row.row_state == RowState::Narrowed)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_published: release_blocking
                .iter()
                .filter(|row| row.row_state == RowState::Published)
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| row.row_state == RowState::Narrowed)
                .count(),
            stable_lane_rows: lane(MaturityLane::Stable),
            beta_lane_rows: lane(MaturityLane::Beta),
            experimental_lane_rows: lane(MaturityLane::Experimental),
            internal_lane_rows: lane(MaturityLane::Internal),
            rows_requiring_json_schema: self
                .required_kind_count(PublicationArtifactKind::JsonSchema),
            rows_requiring_wit_world: self.required_kind_count(PublicationArtifactKind::WitWorld),
            rows_requiring_openapi_spec: self
                .required_kind_count(PublicationArtifactKind::OpenapiSpec),
            rows_requiring_markdown_summary: self
                .required_kind_count(PublicationArtifactKind::MarkdownSummary),
            rows_requiring_example_payloads: self
                .required_kind_count(PublicationArtifactKind::ExamplePayloads),
            rows_requiring_migration_notes: self
                .required_kind_count(PublicationArtifactKind::MigrationNotes),
            rows_requiring_validator_suite: self
                .required_kind_count(PublicationArtifactKind::ValidatorSuite),
            total_required_publications: total_required,
            total_published_publications: total_published,
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            rows_with_active_gap: self
                .rows
                .iter()
                .filter(|row| !row.active_gap_reasons.is_empty())
                .count(),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection downstream surfaces render
    /// instead of cloning publication status text.
    pub fn support_export_projection(&self) -> M5PublicContractExportProjection {
        M5PublicContractExportProjection {
            matrix_id: self.matrix_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5PublicContractExportRow {
                    family_id: row.family_id.clone(),
                    contract_form: row.contract_form,
                    maturity_lane: row.maturity_lane,
                    release_blocking: row.release_blocking,
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    row_state: row.row_state,
                    release_packet_ref: row.release_packet_ref.clone(),
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the matrix, returning every violation found.
    pub fn validate(&self) -> Vec<M5PublicContractViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        if self.rows.is_empty() {
            violations.push(M5PublicContractViolation::EmptyMatrix);
        }
        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.family_id.clone()) {
                violations.push(M5PublicContractViolation::DuplicateFamilyId {
                    family_id: row.family_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5PublicContractViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5PublicContractViolation>) {
        if self.schema_version != M5_PUBLIC_CONTRACT_SCHEMA_VERSION {
            violations.push(M5PublicContractViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_PUBLIC_CONTRACT_RECORD_KIND {
            violations.push(M5PublicContractViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("matrix_id", &self.matrix_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            (
                "contract_family_registry_ref",
                &self.contract_family_registry_ref,
            ),
            (
                "compatibility_surface_inventory_ref",
                &self.compatibility_surface_inventory_ref,
            ),
            ("qualification_matrix_ref", &self.qualification_matrix_ref),
            ("evidence_index_ref", &self.evidence_index_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5PublicContractViolation::EmptyField {
                    family_id: "<matrix>".to_owned(),
                    field_name: field,
                });
            }
        }

        if self.contract_forms != ContractForm::ALL.to_vec() {
            violations.push(M5PublicContractViolation::ClosedVocabularyMismatch {
                field: "contract_forms",
            });
        }
        if self.contract_categories != ContractCategory::ALL.to_vec() {
            violations.push(M5PublicContractViolation::ClosedVocabularyMismatch {
                field: "contract_categories",
            });
        }
        if self.maturity_lanes != MaturityLane::ALL.to_vec() {
            violations.push(M5PublicContractViolation::ClosedVocabularyMismatch {
                field: "maturity_lanes",
            });
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(M5PublicContractViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.reader_writer_postures != ReaderWriterPosture::ALL.to_vec() {
            violations.push(M5PublicContractViolation::ClosedVocabularyMismatch {
                field: "reader_writer_postures",
            });
        }
        if self.packaging_needs != PackagingNeed::ALL.to_vec() {
            violations.push(M5PublicContractViolation::ClosedVocabularyMismatch {
                field: "packaging_needs",
            });
        }
        if self.publication_artifact_kinds != PublicationArtifactKind::ALL.to_vec() {
            violations.push(M5PublicContractViolation::ClosedVocabularyMismatch {
                field: "publication_artifact_kinds",
            });
        }
        if self.publication_states != PublicationState::ALL.to_vec() {
            violations.push(M5PublicContractViolation::ClosedVocabularyMismatch {
                field: "publication_states",
            });
        }
        if self.gap_reasons != GapReason::ALL.to_vec() {
            violations.push(M5PublicContractViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.remediation_actions != RemediationAction::ALL.to_vec() {
            violations.push(M5PublicContractViolation::ClosedVocabularyMismatch {
                field: "remediation_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(M5PublicContractViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5PublicContractViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5PublicContractViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5PublicContractViolation::EmptyField {
                family_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5PublicContractViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5PublicContractViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5PublicContractViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5PublicContractViolation::EmptyField {
                        family_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5PublicContractViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }
        for reason in GapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(M5PublicContractViolation::GapReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &M5PublicContractRow,
        violations: &mut Vec<M5PublicContractViolation>,
    ) {
        for (field, value) in [
            ("family_id", &row.family_id),
            ("title", &row.title),
            ("summary", &row.summary),
            ("owning_package", &row.owning_package),
            ("owner_dri", &row.owner_dri),
            ("contract_family_ref", &row.contract_family_ref),
            ("compatibility_surface_ref", &row.compatibility_surface_ref),
            ("release_packet_ref", &row.release_packet_ref),
            ("rationale", &row.rationale),
        ] {
            if value.trim().is_empty() {
                violations.push(M5PublicContractViolation::EmptyField {
                    family_id: row.family_id.clone(),
                    field_name: field,
                });
            }
        }

        // Exactly one requirement cell per artifact kind.
        let mut seen_kinds: BTreeSet<PublicationArtifactKind> = BTreeSet::new();
        for cell in &row.publication_requirements {
            if !seen_kinds.insert(cell.artifact_kind) {
                violations.push(M5PublicContractViolation::DuplicateRequirementKind {
                    family_id: row.family_id.clone(),
                    kind: cell.artifact_kind,
                });
            }
            // A required form may not be marked not-applicable.
            if cell.required && cell.state == PublicationState::NotApplicable {
                violations.push(M5PublicContractViolation::RequiredButNotApplicable {
                    family_id: row.family_id.clone(),
                    kind: cell.artifact_kind,
                });
            }
            // A published cell must carry at least one ref.
            if cell.state.is_published() && cell.refs.is_empty() {
                violations.push(M5PublicContractViolation::PublishedRequirementWithoutRefs {
                    family_id: row.family_id.clone(),
                    kind: cell.artifact_kind,
                });
            }
        }
        for kind in PublicationArtifactKind::ALL {
            if !seen_kinds.contains(&kind) {
                violations.push(M5PublicContractViolation::RequirementCoverageIncomplete {
                    family_id: row.family_id.clone(),
                    kind,
                });
            }
        }

        // The derived gap reasons must equal the recorded ones.
        let computed = row.computed_gap_reasons();
        if computed != row.active_gap_reasons {
            violations.push(M5PublicContractViolation::GapReasonsMismatch {
                family_id: row.family_id.clone(),
            });
        }

        // The ceiling: a row may not publish wider than its claim.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(M5PublicContractViolation::PublishedWiderThanClaim {
                family_id: row.family_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        let has_gaps = !computed.is_empty();
        match row.row_state {
            RowState::Published => {
                if !row.active_gap_reasons.is_empty() {
                    violations.push(M5PublicContractViolation::PublishedWithActiveGap {
                        family_id: row.family_id.clone(),
                    });
                }
                if row.published_label != row.claim_label {
                    violations.push(M5PublicContractViolation::PublishedLabelNotEqualClaim {
                        family_id: row.family_id.clone(),
                        claim: row.claim_label,
                        published: row.published_label,
                    });
                }
                for cell in &row.publication_requirements {
                    if cell.required && !cell.state.is_published() {
                        violations.push(
                            M5PublicContractViolation::PublishedWithUnpublishedRequirement {
                                family_id: row.family_id.clone(),
                                kind: cell.artifact_kind,
                            },
                        );
                    }
                }
                if row.validator_suite_refs.is_empty() {
                    violations.push(M5PublicContractViolation::PublishedWithoutValidator {
                        family_id: row.family_id.clone(),
                    });
                }
                if row.example_corpus_refs.is_empty() {
                    violations.push(M5PublicContractViolation::PublishedWithoutExamples {
                        family_id: row.family_id.clone(),
                    });
                }
            }
            RowState::Narrowed => {
                if !has_gaps {
                    violations.push(M5PublicContractViolation::NarrowingWithoutReason {
                        family_id: row.family_id.clone(),
                    });
                }
                if row.active_gap_reasons.is_empty() {
                    violations.push(M5PublicContractViolation::NarrowingWithoutReason {
                        family_id: row.family_id.clone(),
                    });
                }
                if row.publishes_stable() {
                    violations.push(M5PublicContractViolation::NarrowedButPublishesStable {
                        family_id: row.family_id.clone(),
                        published: row.published_label,
                    });
                }
                let floor = narrow_floor(row.claim_label);
                if row.published_label != floor {
                    violations.push(M5PublicContractViolation::NarrowedToWrongFloor {
                        family_id: row.family_id.clone(),
                        published: row.published_label,
                        expected: floor,
                    });
                }
            }
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5PublicContractViolation>) {
        let covered: BTreeSet<&str> = self
            .release_blocking_rows()
            .into_iter()
            .map(|row| row.family_id.as_str())
            .collect();
        for declared in &self.release_blocking_family_refs {
            if !covered.contains(declared.as_str()) {
                violations.push(M5PublicContractViolation::ReleaseBlockingFamilyUncovered {
                    family_id: declared.clone(),
                });
            }
        }
        let declared: BTreeSet<&str> = self
            .release_blocking_family_refs
            .iter()
            .map(String::as_str)
            .collect();
        for row in self.release_blocking_rows() {
            if !declared.contains(row.family_id.as_str()) {
                violations.push(M5PublicContractViolation::ReleaseBlockingRowNotDeclared {
                    family_id: row.family_id.clone(),
                });
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<M5PublicContractViolation>) {
        let computed_decision = self.computed_promotion_decision();
        if self.promotion.decision != computed_decision {
            violations.push(M5PublicContractViolation::PromotionDecisionInconsistent {
                declared: self.promotion.decision,
                computed: computed_decision,
            });
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(M5PublicContractViolation::PromotionBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_family_ids() {
            violations.push(M5PublicContractViolation::PromotionBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation found in the matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5PublicContractViolation {
    /// Unsupported schema version.
    UnsupportedSchemaVersion {
        /// The recorded version.
        actual: u32,
    },
    /// Unsupported record kind.
    UnsupportedRecordKind {
        /// The recorded kind.
        actual: String,
    },
    /// A required string field was empty.
    EmptyField {
        /// The owning family id (or a sentinel).
        family_id: String,
        /// The empty field's name.
        field_name: &'static str,
    },
    /// A closed-vocabulary field does not match the canonical lexicon.
    ClosedVocabularyMismatch {
        /// The field name.
        field: &'static str,
    },
    /// The matrix has no rows.
    EmptyMatrix,
    /// The matrix has no stop rules.
    NoStopRules,
    /// Two stop rules share an id.
    DuplicateStopRuleId {
        /// The duplicated rule id.
        rule_id: String,
    },
    /// A stop rule names no labels.
    StopRuleWithoutLabels {
        /// The rule id.
        rule_id: String,
    },
    /// A gap reason has no stop rule.
    GapReasonWithoutStopRule {
        /// The uncovered reason.
        reason: GapReason,
    },
    /// Two rows share a family id.
    DuplicateFamilyId {
        /// The duplicated family id.
        family_id: String,
    },
    /// A row repeats a requirement kind.
    DuplicateRequirementKind {
        /// The owning family id.
        family_id: String,
        /// The duplicated kind.
        kind: PublicationArtifactKind,
    },
    /// A row is missing a requirement cell for a kind.
    RequirementCoverageIncomplete {
        /// The owning family id.
        family_id: String,
        /// The missing kind.
        kind: PublicationArtifactKind,
    },
    /// A required form is marked not-applicable.
    RequiredButNotApplicable {
        /// The owning family id.
        family_id: String,
        /// The contradictory kind.
        kind: PublicationArtifactKind,
    },
    /// A published requirement carries no refs.
    PublishedRequirementWithoutRefs {
        /// The owning family id.
        family_id: String,
        /// The kind.
        kind: PublicationArtifactKind,
    },
    /// The recorded gap reasons disagree with the derived ones.
    GapReasonsMismatch {
        /// The owning family id.
        family_id: String,
    },
    /// A row publishes wider than its claim.
    PublishedWiderThanClaim {
        /// The owning family id.
        family_id: String,
        /// The claim label.
        claim: StableClaimLevel,
        /// The published label.
        published: StableClaimLevel,
    },
    /// A published row carries an active gap.
    PublishedWithActiveGap {
        /// The owning family id.
        family_id: String,
    },
    /// A published row's label is not equal to its claim.
    PublishedLabelNotEqualClaim {
        /// The owning family id.
        family_id: String,
        /// The claim label.
        claim: StableClaimLevel,
        /// The published label.
        published: StableClaimLevel,
    },
    /// A published row leaves a required form unpublished.
    PublishedWithUnpublishedRequirement {
        /// The owning family id.
        family_id: String,
        /// The unpublished kind.
        kind: PublicationArtifactKind,
    },
    /// A published row wires no validator suite.
    PublishedWithoutValidator {
        /// The owning family id.
        family_id: String,
    },
    /// A published row publishes no example corpus.
    PublishedWithoutExamples {
        /// The owning family id.
        family_id: String,
    },
    /// A narrowed row names no gap reason.
    NarrowingWithoutReason {
        /// The owning family id.
        family_id: String,
    },
    /// A narrowed row still publishes at or above the cutline.
    NarrowedButPublishesStable {
        /// The owning family id.
        family_id: String,
        /// The published label.
        published: StableClaimLevel,
    },
    /// A narrowed row narrowed to the wrong floor.
    NarrowedToWrongFloor {
        /// The owning family id.
        family_id: String,
        /// The published label.
        published: StableClaimLevel,
        /// The expected floor.
        expected: StableClaimLevel,
    },
    /// A declared release-blocking family has no covering row.
    ReleaseBlockingFamilyUncovered {
        /// The uncovered family id.
        family_id: String,
    },
    /// A release-blocking row is not declared.
    ReleaseBlockingRowNotDeclared {
        /// The undeclared family id.
        family_id: String,
    },
    /// The promotion decision disagrees with the firing rules.
    PromotionDecisionInconsistent {
        /// The declared decision.
        declared: PromotionDecision,
        /// The computed decision.
        computed: PromotionDecision,
    },
    /// A promotion blocking set disagrees with the firing rules.
    PromotionBlockingSetMismatch {
        /// The field name.
        field: &'static str,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5PublicContractViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported schema version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported record kind {actual}")
            }
            Self::EmptyField {
                family_id,
                field_name,
            } => write!(f, "{family_id}: empty field {field_name}"),
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "closed vocabulary mismatch: {field}")
            }
            Self::EmptyMatrix => write!(f, "matrix has no rows"),
            Self::NoStopRules => write!(f, "matrix has no stop rules"),
            Self::DuplicateStopRuleId { rule_id } => {
                write!(f, "duplicate stop-rule id {rule_id}")
            }
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} names no labels")
            }
            Self::GapReasonWithoutStopRule { reason } => {
                write!(f, "gap reason {} has no stop rule", reason.as_str())
            }
            Self::DuplicateFamilyId { family_id } => {
                write!(f, "duplicate family id {family_id}")
            }
            Self::DuplicateRequirementKind { family_id, kind } => {
                write!(f, "{family_id} repeats requirement kind {}", kind.as_str())
            }
            Self::RequirementCoverageIncomplete { family_id, kind } => write!(
                f,
                "{family_id} is missing requirement kind {}",
                kind.as_str()
            ),
            Self::RequiredButNotApplicable { family_id, kind } => write!(
                f,
                "{family_id} marks required {} as not_applicable",
                kind.as_str()
            ),
            Self::PublishedRequirementWithoutRefs { family_id, kind } => {
                write!(f, "{family_id} published {} without refs", kind.as_str())
            }
            Self::GapReasonsMismatch { family_id } => write!(
                f,
                "{family_id} active_gap_reasons disagree with the derived reasons"
            ),
            Self::PublishedWiderThanClaim {
                family_id,
                claim,
                published,
            } => write!(
                f,
                "{family_id} publishes {} wider than claim {}",
                published.as_str(),
                claim.as_str()
            ),
            Self::PublishedWithActiveGap { family_id } => {
                write!(f, "{family_id} is published with an active gap")
            }
            Self::PublishedLabelNotEqualClaim {
                family_id,
                claim,
                published,
            } => write!(
                f,
                "{family_id} published {} does not equal claim {}",
                published.as_str(),
                claim.as_str()
            ),
            Self::PublishedWithUnpublishedRequirement { family_id, kind } => write!(
                f,
                "{family_id} is published but required {} is unpublished",
                kind.as_str()
            ),
            Self::PublishedWithoutValidator { family_id } => {
                write!(f, "{family_id} is published without a validator suite")
            }
            Self::PublishedWithoutExamples { family_id } => {
                write!(f, "{family_id} is published without an example corpus")
            }
            Self::NarrowingWithoutReason { family_id } => {
                write!(f, "{family_id} narrows without a gap reason")
            }
            Self::NarrowedButPublishesStable {
                family_id,
                published,
            } => write!(
                f,
                "{family_id} narrows but publishes {} at or above the cutline",
                published.as_str()
            ),
            Self::NarrowedToWrongFloor {
                family_id,
                published,
                expected,
            } => write!(
                f,
                "{family_id} narrowed to {} but the floor is {}",
                published.as_str(),
                expected.as_str()
            ),
            Self::ReleaseBlockingFamilyUncovered { family_id } => {
                write!(f, "release-blocking family {family_id} has no covering row")
            }
            Self::ReleaseBlockingRowNotDeclared { family_id } => write!(
                f,
                "release-blocking row {family_id} is not declared in release_blocking_family_refs"
            ),
            Self::PromotionDecisionInconsistent { declared, computed } => write!(
                f,
                "promotion {} disagrees with computed {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with rows"),
        }
    }
}

impl Error for M5PublicContractViolation {}

/// Loads the embedded M5 public-contract matrix.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in matrix no longer matches
/// [`M5PublicContractMatrix`].
pub fn current_m5_public_contract_matrix() -> Result<M5PublicContractMatrix, serde_json::Error> {
    serde_json::from_str(M5_PUBLIC_CONTRACT_JSON)
}

#[cfg(test)]
mod tests;
