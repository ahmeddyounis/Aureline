//! Typed M5 contract catalog: the one inspectable index that joins every
//! published M5 contract family to its lifecycle label, canonical schema/spec
//! identifier, compatibility note, offline/mirror posture, and a checked-in
//! sample payload gallery.
//!
//! Where the public-contract publication matrix records *whether* each M5
//! artifact family has published its contract forms, and the per-form catalogs
//! publish the JSON Schema packages, the OpenAPI service routes, and the WIT
//! capability worlds, this catalog is the *consuming* layer on top of all of
//! them: it lets users, admins, support, extension authors, and
//! self-host/mirror operators enumerate every published contract family from one
//! source and inspect a real, checked-in sample payload — offline — for each one.
//! Each [`ContractFamilyEntry`] binds one family to:
//!
//! - its contract form ([`ContractForm`]), maturity lane ([`MaturityLane`]), and
//!   the lifecycle label it publishes ([`LifecycleLabel`], the label the matrix
//!   publishes after narrowing),
//! - its canonical [`ContractIdentity`] (the schema or spec identifier and the
//!   per-form catalog it lives in), and the JSON Schema its gallery samples
//!   validate against ([`ContractFamilyEntry::json_schema_validation_ref`], when
//!   the family is JSON-Schema-backed),
//! - its compatibility note, its sample payload gallery
//!   ([`ContractFamilyEntry::example_gallery_ref`]), and its
//!   [`OfflineParity`] posture.
//!
//! Every entry's [`ContractFamilyEntry::lifecycle_label`] equals the publication
//! matrix's effective published label for that family, so a narrowed family
//! narrows here automatically and the catalog never advertises a greener label
//! than the matrix. Every entry points back to the canonical schema/spec
//! identifier, so the catalog is never the only source of truth, and every
//! gallery carries a partial/not-provided sample so stable partial outcomes are
//! never omitted.
//!
//! The catalog is checked in at `artifacts/contracts/m5-contract-catalog.json`
//! and embedded here, so this typed consumer and the CI validator agree on every
//! family without a cargo build in CI. The model is metadata-plus-sample only:
//! every field is a typed state, an opaque repo-relative ref or URI, or a
//! copy/export-safe sample reference. It carries no credential bodies or raw
//! provider payloads.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Supported catalog schema version.
pub const M5_CONTRACT_CATALOG_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the catalog.
pub const M5_CONTRACT_CATALOG_RECORD_KIND: &str = "m5_contract_catalog";

/// Stable catalog identifier.
pub const M5_CONTRACT_CATALOG_ID: &str = "m5_contract_catalog:v1";

/// Repo-relative path to the checked-in catalog.
pub const M5_CONTRACT_CATALOG_PATH: &str = "artifacts/contracts/m5-contract-catalog.json";

/// Embedded checked-in catalog JSON.
pub const M5_CONTRACT_CATALOG_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/contracts/m5-contract-catalog.json"
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
    /// Every label, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Lts,
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Withdrawn,
    ];

    /// True when the label publishes at or above the stable cutline.
    pub fn is_at_or_above_cutline(self) -> bool {
        matches!(self, Self::Lts | Self::Stable)
    }
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
}

/// The published contract form a family carries (the full publication-matrix set).
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
    /// A field set.
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

    /// The canonical contract-identity kind a form resolves to.
    pub const fn identity_kind(self) -> IdentityKind {
        match self {
            Self::OpenapiFamily => IdentityKind::OpenapiSpec,
            Self::WitWorldPackage => IdentityKind::WitWorld,
            _ => IdentityKind::JsonSchema,
        }
    }
}

/// The canonical contract-identity kind for a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityKind {
    /// A JSON Schema package identity.
    JsonSchema,
    /// An OpenAPI specification identity.
    OpenapiSpec,
    /// A WIT world package identity.
    WitWorld,
}

impl IdentityKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::JsonSchema, Self::OpenapiSpec, Self::WitWorld];
}

/// The class of a sample payload in a family's gallery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SampleClass {
    /// A fully-populated, version-stamped sample.
    Nominal,
    /// A stable partial/not-provided outcome sample.
    PartialOrNotProvided,
}

impl SampleClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 2] = [Self::Nominal, Self::PartialOrNotProvided];
}

/// The mirror/offline packaging need of a family's service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackagingNeed {
    /// Local-only.
    LocalOnly,
    /// Mirror-packaged for offline/air-gapped use.
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
}

/// A surface that renders from this one catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogSurface {
    /// Help/About.
    HelpAbout,
    /// SDK docs.
    SdkDocs,
    /// Docs center.
    DocsCenter,
    /// Support export.
    SupportExport,
    /// In-product CLI inspection.
    CliInspect,
}

impl CatalogSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HelpAbout,
        Self::SdkDocs,
        Self::DocsCenter,
        Self::SupportExport,
        Self::CliInspect,
    ];
}

/// The canonical contract identity for a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractIdentity {
    /// The contract-identity kind.
    pub identity_kind: IdentityKind,
    /// The stable schema or spec identifier (a `$id` URI or a stable catalog id).
    pub schema_or_spec_id: String,
    /// Repo-relative ref to the schema or spec document.
    pub schema_or_spec_ref: String,
    /// Repo-relative ref to the per-form catalog this identity lives in.
    pub form_catalog_ref: String,
}

/// A family's offline/mirror inspection posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OfflineParity {
    /// True when the contract and its samples can be inspected from a mirror.
    pub mirror_inspectable: bool,
    /// True when inspecting the contract requires a live service.
    pub requires_runtime_service: bool,
    /// The mirror/offline packaging need of the family's service.
    pub packaging_need: PackagingNeed,
}

/// One published contract-family catalog entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractFamilyEntry {
    /// Stable family id (links to the contract-family registry and the matrix).
    pub family_id: String,
    /// Human-readable title.
    pub title: String,
    /// Reviewable one-line summary.
    pub summary: String,
    /// The owning crate or lane.
    pub owning_package: String,
    /// The category grouping (matrix lexicon).
    pub category: String,
    /// The contract form.
    pub contract_form: ContractForm,
    /// The contract-family maturity lane.
    pub maturity_lane: MaturityLane,
    /// The lifecycle label the family is put forward at.
    pub claim_label: LifecycleLabel,
    /// The lifecycle label the family publishes after narrowing.
    pub lifecycle_label: LifecycleLabel,
    /// Whether the family narrows below its claim label.
    pub narrowed: bool,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// Active gap reasons narrowing the family (matrix lexicon).
    pub active_gap_reasons: Vec<String>,
    /// The canonical contract identity.
    pub contract_identity: ContractIdentity,
    /// The JSON Schema the gallery samples validate against, if the family is
    /// JSON-Schema-backed.
    pub json_schema_validation_ref: Option<String>,
    /// Human-readable compatibility note.
    pub compatibility_note: String,
    /// Ref to the doc that carries the family's compatibility note.
    pub compatibility_note_ref: String,
    /// Ref to the family's sample payload gallery.
    pub example_gallery_ref: String,
    /// Number of samples in the gallery.
    pub sample_count: usize,
    /// The sample classes the gallery publishes.
    pub sample_classes: Vec<SampleClass>,
    /// The offline/mirror inspection posture.
    pub offline_parity: OfflineParity,
    /// Ref to the publication-matrix row.
    pub matrix_row_ref: String,
    /// Ref to the contract-family registry row.
    pub contract_family_ref: String,
    /// Publication destinations the matrix records for this family.
    pub publication_destinations: Vec<String>,
    /// Surfaces that render this entry.
    pub catalog_surfaces: Vec<CatalogSurface>,
}

impl ContractFamilyEntry {
    /// True when the family publishes at or above the stable cutline.
    pub fn publishes_stable(&self) -> bool {
        self.lifecycle_label.is_at_or_above_cutline()
    }

    /// True when the gallery publishes a partial/not-provided sample.
    pub fn has_partial_sample(&self) -> bool {
        self.sample_classes
            .contains(&SampleClass::PartialOrNotProvided)
    }
}

/// The offline/mirror bundling declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OfflineBundle {
    /// True when the catalog set bundles into mirror artifact sets.
    pub mirrorable: bool,
    /// True when inspection requires runtime service access.
    pub requires_runtime_service: bool,
    /// Bundle members (catalog, galleries, schemas, docs, validator).
    pub bundle_members: Vec<String>,
    /// Human-readable note.
    pub note: String,
}

/// Summary counts over the family set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ContractCatalogSummary {
    /// Total families.
    pub total_families: usize,
    /// Families publishing at the stable label.
    pub families_stable_label: usize,
    /// Families publishing at the beta label.
    pub families_beta_label: usize,
    /// Families narrowed below their claim label.
    pub families_narrowed: usize,
    /// Release-blocking families.
    pub release_blocking_families: usize,
    /// Families whose identity is a JSON Schema.
    pub json_schema_identity_families: usize,
    /// Families whose identity is an OpenAPI spec.
    pub openapi_identity_families: usize,
    /// Families whose identity is a WIT world.
    pub wit_identity_families: usize,
    /// Families whose gallery samples validate against a JSON Schema.
    pub families_with_json_schema_validation: usize,
    /// Families whose gallery includes a partial/not-provided sample.
    pub families_with_partial_sample: usize,
    /// Total samples across all galleries.
    pub total_samples: usize,
    /// Gallery files (one per family).
    pub gallery_files: usize,
}

/// A structural validation violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ContractCatalogViolation {
    /// Stable check id.
    pub check_id: String,
    /// Human-readable detail.
    pub detail: String,
}

impl std::fmt::Display for M5ContractCatalogViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.check_id, self.detail)
    }
}

/// One support/inspect export row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContractCatalogExportRow {
    /// Stable family id.
    pub family_id: String,
    /// The lifecycle label the family publishes.
    pub lifecycle_label: LifecycleLabel,
    /// The contract-identity kind.
    pub identity_kind: IdentityKind,
    /// The canonical schema or spec identifier.
    pub schema_or_spec_id: String,
    /// Whether the family narrows below its claim label.
    pub narrowed: bool,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The sample payload gallery ref.
    pub example_gallery_ref: String,
}

/// Export projection for Help/About, SDK, support, and in-product inspect surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContractCatalogExportProjection {
    /// Catalog identifier.
    pub catalog_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Export rows.
    pub rows: Vec<M5ContractCatalogExportRow>,
}

/// The typed M5 contract catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ContractCatalog {
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
    /// Narrative companion document.
    pub overview_page: String,
    /// Evidence/proof packet.
    pub evidence_page: String,
    /// Help-center catalog doc.
    pub help_catalog_page: String,
    /// SDK samples doc.
    pub sdk_samples_page: String,
    /// Ref to the public-contract publication matrix.
    pub publication_matrix_ref: String,
    /// Ref to the JSON Schema catalog.
    pub json_schema_catalog_ref: String,
    /// Ref to the OpenAPI catalog.
    pub openapi_catalog_ref: String,
    /// Ref to the WIT contract publication.
    pub wit_publication_ref: String,
    /// Ref to the contract-family registry.
    pub contract_family_registry_ref: String,
    /// Ref to the canonical M5 evidence index.
    pub evidence_index_ref: String,
    /// The sample-gallery home.
    pub gallery_home: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<LifecycleLabel>,
    /// Closed maturity-lane vocabulary.
    pub maturity_lanes: Vec<MaturityLane>,
    /// Closed contract-form vocabulary.
    pub contract_forms: Vec<ContractForm>,
    /// Closed identity-kind vocabulary.
    pub identity_kinds: Vec<IdentityKind>,
    /// Closed sample-class vocabulary.
    pub sample_classes: Vec<SampleClass>,
    /// Closed packaging-need vocabulary.
    pub packaging_needs: Vec<PackagingNeed>,
    /// Closed catalog-surface vocabulary.
    pub catalog_surfaces: Vec<CatalogSurface>,
    /// The offline/mirror bundling declaration.
    pub offline_bundle: OfflineBundle,
    /// The published contract families.
    pub families: Vec<ContractFamilyEntry>,
    /// Summary counts.
    pub summary: M5ContractCatalogSummary,
}

impl M5ContractCatalog {
    /// Returns the entry registered for `family_id`.
    pub fn family(&self, family_id: &str) -> Option<&ContractFamilyEntry> {
        self.families.iter().find(|f| f.family_id == family_id)
    }

    /// Resolves the canonical schema/spec identifier and lifecycle label for a
    /// family.
    ///
    /// This is the lookup Help/About, SDK docs, support export, and the
    /// in-product inspect surface share so they quote one contract identity and
    /// one lifecycle label per family.
    pub fn resolve_contract(&self, family_id: &str) -> Option<(&str, LifecycleLabel)> {
        self.family(family_id).map(|f| {
            (
                f.contract_identity.schema_or_spec_id.as_str(),
                f.lifecycle_label,
            )
        })
    }

    /// Families publishing at or above the stable cutline.
    pub fn stable_families(&self) -> Vec<&ContractFamilyEntry> {
        self.families
            .iter()
            .filter(|f| f.publishes_stable())
            .collect()
    }

    /// Families narrowed below their claim label.
    pub fn narrowed_families(&self) -> Vec<&ContractFamilyEntry> {
        self.families.iter().filter(|f| f.narrowed).collect()
    }

    /// Recomputes the summary block from the families.
    pub fn computed_summary(&self) -> M5ContractCatalogSummary {
        let count = |f: &dyn Fn(&ContractFamilyEntry) -> bool| {
            self.families.iter().filter(|e| f(e)).count()
        };
        M5ContractCatalogSummary {
            total_families: self.families.len(),
            families_stable_label: count(&|e| e.lifecycle_label == LifecycleLabel::Stable),
            families_beta_label: count(&|e| e.lifecycle_label == LifecycleLabel::Beta),
            families_narrowed: count(&|e| e.narrowed),
            release_blocking_families: count(&|e| e.release_blocking),
            json_schema_identity_families: count(&|e| {
                e.contract_identity.identity_kind == IdentityKind::JsonSchema
            }),
            openapi_identity_families: count(&|e| {
                e.contract_identity.identity_kind == IdentityKind::OpenapiSpec
            }),
            wit_identity_families: count(&|e| {
                e.contract_identity.identity_kind == IdentityKind::WitWorld
            }),
            families_with_json_schema_validation: count(&|e| {
                e.json_schema_validation_ref.is_some()
            }),
            families_with_partial_sample: count(&|e| e.has_partial_sample()),
            total_samples: self.families.iter().map(|e| e.sample_count).sum(),
            gallery_files: self.families.len(),
        }
    }

    /// Produces an export/inspect-safe projection downstream surfaces render
    /// instead of cloning catalog text.
    pub fn support_export_projection(&self) -> M5ContractCatalogExportProjection {
        M5ContractCatalogExportProjection {
            catalog_id: self.catalog_id.clone(),
            as_of: self.as_of.clone(),
            rows: self
                .families
                .iter()
                .map(|f| M5ContractCatalogExportRow {
                    family_id: f.family_id.clone(),
                    lifecycle_label: f.lifecycle_label,
                    identity_kind: f.contract_identity.identity_kind,
                    schema_or_spec_id: f.contract_identity.schema_or_spec_id.clone(),
                    narrowed: f.narrowed,
                    release_blocking: f.release_blocking,
                    example_gallery_ref: f.example_gallery_ref.clone(),
                })
                .collect(),
        }
    }

    /// Validates the catalog's structural invariants.
    ///
    /// Mirrors the CI validator's semantic invariants. The checked-in catalog
    /// returns no violations; each negative fixture returns at least one.
    pub fn validate(&self) -> Vec<M5ContractCatalogViolation> {
        let mut out = Vec::new();
        let mut push = |check_id: &str, detail: String| {
            out.push(M5ContractCatalogViolation {
                check_id: check_id.to_string(),
                detail,
            })
        };

        if self.schema_version != M5_CONTRACT_CATALOG_SCHEMA_VERSION {
            push(
                "catalog.schema_version",
                format!("unexpected schema_version {}", self.schema_version),
            );
        }
        if self.record_kind != M5_CONTRACT_CATALOG_RECORD_KIND {
            push(
                "catalog.record_kind",
                format!("unexpected record_kind {}", self.record_kind),
            );
        }
        if self.catalog_id != M5_CONTRACT_CATALOG_ID {
            push(
                "catalog.catalog_id",
                format!("unexpected catalog_id {}", self.catalog_id),
            );
        }

        if self.lifecycle_labels != LifecycleLabel::ALL {
            push(
                "vocab.lifecycle_labels",
                "lifecycle_labels off the canonical list".into(),
            );
        }
        if self.maturity_lanes != MaturityLane::ALL {
            push(
                "vocab.maturity_lanes",
                "maturity_lanes off the canonical list".into(),
            );
        }
        if self.contract_forms != ContractForm::ALL {
            push(
                "vocab.contract_forms",
                "contract_forms off the canonical list".into(),
            );
        }
        if self.identity_kinds != IdentityKind::ALL {
            push(
                "vocab.identity_kinds",
                "identity_kinds off the canonical list".into(),
            );
        }
        if self.sample_classes != SampleClass::ALL {
            push(
                "vocab.sample_classes",
                "sample_classes off the canonical list".into(),
            );
        }
        if self.packaging_needs != PackagingNeed::ALL {
            push(
                "vocab.packaging_needs",
                "packaging_needs off the canonical list".into(),
            );
        }
        if self.catalog_surfaces != CatalogSurface::ALL {
            push(
                "vocab.catalog_surfaces",
                "catalog_surfaces off the canonical list".into(),
            );
        }
        if self.offline_bundle.requires_runtime_service {
            push(
                "offline.requires_runtime_service",
                "the catalog bundle must inspect offline".into(),
            );
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for fam in &self.families {
            if !seen.insert(fam.family_id.as_str()) {
                push(
                    "families.duplicate_family_id",
                    format!("duplicate family_id {}", fam.family_id),
                );
            }
            if fam.contract_identity.identity_kind != fam.contract_form.identity_kind() {
                push(
                    "families.identity_kind_mismatch",
                    format!(
                        "{}: identity_kind disagrees with its contract form",
                        fam.family_id
                    ),
                );
            }
            if fam.sample_classes != SampleClass::ALL {
                push(
                    "families.missing_partial_sample",
                    format!(
                        "{}: sample_classes must be the full closed set",
                        fam.family_id
                    ),
                );
            }
            if fam.sample_count != SampleClass::ALL.len() {
                push(
                    "families.sample_count",
                    format!(
                        "{}: sample_count must equal the sample-class count",
                        fam.family_id
                    ),
                );
            }
            if fam.offline_parity.requires_runtime_service {
                push(
                    "families.requires_runtime_service",
                    format!(
                        "{}: offline_parity must inspect without a live service",
                        fam.family_id
                    ),
                );
            }
            let expected_gallery = format!("{}{}.json", self.gallery_home, fam.family_id);
            if fam.example_gallery_ref != expected_gallery {
                push(
                    "families.gallery_ref",
                    format!(
                        "{}: example_gallery_ref must point at the family gallery",
                        fam.family_id
                    ),
                );
            }
            if fam.publishes_stable() && fam.lifecycle_label != fam.claim_label && !fam.narrowed {
                push(
                    "families.published_label",
                    format!(
                        "{}: a stable family must publish its claim label",
                        fam.family_id
                    ),
                );
            }
        }

        if self.summary != self.computed_summary() {
            push(
                "summary.count_mismatch",
                "summary counts disagree with the families".into(),
            );
        }

        out
    }
}

/// Parses the embedded checked-in catalog into the typed model.
pub fn current_m5_contract_catalog() -> Result<M5ContractCatalog, serde_json::Error> {
    serde_json::from_str(M5_CONTRACT_CATALOG_JSON)
}

#[cfg(test)]
mod tests;
