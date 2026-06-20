//! Typed M5 JSON Schema catalog: the canonical index of every durable M5
//! artifact family that publishes a checked-in JSON Schema package.
//!
//! Where the public-contract publication matrix speaks for *whether* each M5
//! artifact family has published the contract forms it needs, this catalog
//! speaks for the *JSON Schema package* itself: for every family the matrix puts
//! forward as a JSON-Schema-backed contract it publishes one package under
//! `schemas/public/m5-json/` with an explicit in-band schema version field, a
//! lifecycle/stability label, a field-level compatibility contract, an example
//! payload, and a round-trip fixture. Each [`JsonSchemaPackage`] binds one
//! family to:
//!
//! - its contract form ([`ContractForm`]), stability lane ([`MaturityLane`]),
//!   and the lifecycle label it publishes ([`LifecycleLabel`], the label the
//!   matrix effectively publishes after narrowing),
//! - its stable schema identifier ([`JsonSchemaPackage::schema_id`]) and the
//!   checked-in schema file ([`JsonSchemaPackage::schema_path`]),
//! - its in-band version field(s) and primary stable object identity, and
//! - its [`FieldContract`]: the additive-field rule, required-field policy,
//!   unknown-field policy, downgrade behavior, and migration-note hooks.
//!
//! The catalog is the source surfaces resolve a schema identifier and lifecycle
//! label from: export/import, support export, and docs/help all read the same
//! [`JsonSchemaPackage::schema_id`] and [`JsonSchemaPackage::lifecycle_label`]
//! instead of restating field semantics. Every package preserves unknown fields
//! ([`UnknownFieldPolicy::Preserve`]) so durable artifacts round-trip through
//! export and offline-mirror flows without loss.
//!
//! The catalog is checked in at
//! `artifacts/contracts/m5-json-schema-catalog.json` and embedded here, so this
//! typed consumer and the CI validator agree on every package without a cargo
//! build in CI. The model is metadata-only: every field is a typed state or an
//! opaque repo-relative ref or URI. It carries no surface payloads, rendered
//! bodies, signatures, or credential material.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Supported catalog schema version.
pub const M5_JSON_SCHEMA_CATALOG_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the catalog.
pub const M5_JSON_SCHEMA_CATALOG_RECORD_KIND: &str = "m5_json_schema_catalog";

/// Stable catalog identifier.
pub const M5_JSON_SCHEMA_CATALOG_ID: &str = "m5_json_schema_catalog:v1";

/// Repo-relative path to the checked-in catalog.
pub const M5_JSON_SCHEMA_CATALOG_PATH: &str = "artifacts/contracts/m5-json-schema-catalog.json";

/// Embedded checked-in catalog JSON.
pub const M5_JSON_SCHEMA_CATALOG_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/contracts/m5-json-schema-catalog.json"
));

/// The lifecycle/stability label a package publishes.
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

/// The published contract form a family carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractForm {
    /// A JSON-Schema-backed contract document.
    JsonSchemaBackedContractDoc,
    /// A registry of typed records.
    RecordRegistry,
    /// An event-envelope schema.
    EventEnvelopeSchema,
    /// CLI/headless structured output.
    CliStructuredOutput,
    /// An asset-package manifest.
    AssetPackageManifest,
    /// A teaching content pack.
    TeachingContentPack,
    /// An OpenAPI specification family.
    OpenapiFamily,
}

impl ContractForm {
    /// Every form, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::JsonSchemaBackedContractDoc,
        Self::RecordRegistry,
        Self::EventEnvelopeSchema,
        Self::CliStructuredOutput,
        Self::AssetPackageManifest,
        Self::TeachingContentPack,
        Self::OpenapiFamily,
    ];
}

/// How new fields land in a package without a major bump.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdditiveFieldRule {
    /// New fields are added only as optional members in additive minor bumps.
    AdditiveMinorOptionalOnly,
}

impl AdditiveFieldRule {
    /// Every rule, in declaration order.
    pub const ALL: [Self; 1] = [Self::AdditiveMinorOptionalOnly];
}

/// The required-field-set policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredFieldPolicy {
    /// The required-field set is frozen until a major bump.
    FrozenRequiredSet,
}

impl RequiredFieldPolicy {
    /// Every policy, in declaration order.
    pub const ALL: [Self; 1] = [Self::FrozenRequiredSet];
}

/// How a reader treats unknown fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnknownFieldPolicy {
    /// Unknown fields are preserved on round-trip.
    Preserve,
    /// Unknown fields are rejected.
    RejectUnknown,
}

impl UnknownFieldPolicy {
    /// Every policy, in declaration order.
    pub const ALL: [Self; 2] = [Self::Preserve, Self::RejectUnknown];
}

/// What happens to a family that loses required publication evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeBehavior {
    /// The family narrows below the launch cutline.
    NarrowBelowCutline,
    /// The artifact is rejected.
    Reject,
}

impl DowngradeBehavior {
    /// Every behavior, in declaration order.
    pub const ALL: [Self; 2] = [Self::NarrowBelowCutline, Self::Reject];
}

/// A surface that resolves a package's schema identifier and lifecycle label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionSurface {
    /// Export/import flows.
    ExportImport,
    /// Support export flows.
    SupportExport,
    /// Docs/help surfaces.
    DocsHelp,
    /// CLI inspection.
    CliInspect,
}

impl ResolutionSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExportImport,
        Self::SupportExport,
        Self::DocsHelp,
        Self::CliInspect,
    ];
}

/// The field-level compatibility contract a package commits to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldContract {
    /// How new fields land without a major bump.
    pub additive_field_rule: AdditiveFieldRule,
    /// The required-field-set policy.
    pub required_field_policy: RequiredFieldPolicy,
    /// How unknown fields are treated.
    pub unknown_field_policy: UnknownFieldPolicy,
    /// What happens when required publication evidence is lost.
    pub downgrade_behavior: DowngradeBehavior,
    /// Docs a reader consults for migration/deprecation behavior.
    pub migration_note_hooks: Vec<String>,
}

/// One published JSON Schema package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonSchemaPackage {
    /// Stable package identifier (`m5.<family_id>`).
    pub package_id: String,
    /// Catalog family id (unique per package).
    pub family_id: String,
    /// Contract-family registry id this package draws its envelope from.
    pub registry_family_id: String,
    /// Human-readable title.
    pub title: String,
    /// One-line summary.
    pub summary: String,
    /// The published contract form.
    pub contract_form: ContractForm,
    /// The contract-family maturity lane.
    pub maturity_lane: MaturityLane,
    /// The lifecycle label this package publishes.
    pub lifecycle_label: LifecycleLabel,
    /// The stable schema identifier (`$id`).
    pub schema_id: String,
    /// Repo-relative path to the checked-in schema file.
    pub schema_path: String,
    /// The record-kind tag field name (always `record_kind`).
    pub record_kind_field: String,
    /// The primary in-band schema version field.
    pub primary_version_field: String,
    /// Every in-band schema version field.
    pub version_field_names: Vec<String>,
    /// The primary stable object identity field.
    pub primary_identifier_field: String,
    /// The field-level compatibility contract.
    pub field_contract: FieldContract,
    /// Human-readable compatibility note.
    pub compatibility_note: String,
    /// Ref to the doc that carries the family's compatibility note.
    pub compatibility_note_ref: String,
    /// Ref to the checked-in example payload.
    pub example_payload_ref: String,
    /// Ref to the checked-in round-trip fixture.
    pub roundtrip_fixture_ref: String,
    /// Ref to the publication-matrix row.
    pub matrix_row_ref: String,
    /// Ref to the contract-family registry row.
    pub contract_family_ref: String,
    /// Refs to the validators that gate this package.
    pub validator_suite_refs: Vec<String>,
    /// Surfaces that resolve this package's schema id and lifecycle label.
    pub resolution_surfaces: Vec<ResolutionSurface>,
}

impl JsonSchemaPackage {
    /// True when this package publishes at or above the stable cutline.
    pub fn publishes_stable(&self) -> bool {
        matches!(
            self.lifecycle_label,
            LifecycleLabel::Lts | LifecycleLabel::Stable
        )
    }

    /// True when this package preserves unknown fields on round-trip.
    pub fn preserves_unknown_fields(&self) -> bool {
        self.field_contract.unknown_field_policy == UnknownFieldPolicy::Preserve
    }
}

/// The offline/mirror bundling declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OfflineBundle {
    /// True when the package set bundles into mirror artifact sets.
    pub mirrorable: bool,
    /// True when validation requires runtime service access.
    pub requires_runtime_service: bool,
    /// Bundle members (schema home, examples, fixtures, validator).
    pub bundle_members: Vec<String>,
    /// Human-readable note.
    pub note: String,
}

/// Summary counts over the package set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5JsonSchemaCatalogSummary {
    /// Total packages.
    pub total_packages: usize,
    /// Packages published at the stable label.
    pub stable_label_packages: usize,
    /// Packages published at the beta label.
    pub beta_label_packages: usize,
    /// Packages that preserve unknown fields.
    pub preserve_unknown_packages: usize,
    /// Packages with at least one migration-note hook.
    pub packages_with_migration_hooks: usize,
    /// Packages with a round-trip fixture.
    pub packages_with_roundtrip_fixture: usize,
    /// Checked-in schema files.
    pub schema_files: usize,
    /// Checked-in example payloads.
    pub example_payloads: usize,
    /// Checked-in round-trip fixtures.
    pub roundtrip_fixtures: usize,
}

/// A structural validation violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5JsonSchemaCatalogViolation {
    /// Stable check id.
    pub check_id: String,
    /// Human-readable detail.
    pub detail: String,
}

/// The typed M5 JSON Schema catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5JsonSchemaCatalog {
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
    /// Human-readable companion document.
    pub overview_page: String,
    /// SDK catalog doc.
    pub sdk_catalog_page: String,
    /// Ref to the public-contract publication matrix.
    pub publication_matrix_ref: String,
    /// Ref to the contract-family registry.
    pub contract_family_registry_ref: String,
    /// Ref to the canonical M5 evidence index.
    pub evidence_index_ref: String,
    /// Base URI every package `$id` extends.
    pub schema_base_uri: String,
    /// Schema home for the published packages.
    pub schema_home: String,
    /// The JSON Schema dialect the packages declare.
    pub json_schema_dialect: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<LifecycleLabel>,
    /// Closed maturity-lane vocabulary.
    pub maturity_lanes: Vec<MaturityLane>,
    /// Closed contract-form vocabulary.
    pub contract_forms: Vec<ContractForm>,
    /// Closed additive-field-rule vocabulary.
    pub additive_field_rules: Vec<AdditiveFieldRule>,
    /// Closed required-field-policy vocabulary.
    pub required_field_policies: Vec<RequiredFieldPolicy>,
    /// Closed unknown-field-policy vocabulary.
    pub unknown_field_policies: Vec<UnknownFieldPolicy>,
    /// Closed downgrade-behavior vocabulary.
    pub downgrade_behaviors: Vec<DowngradeBehavior>,
    /// Closed resolution-surface vocabulary.
    pub resolution_surfaces: Vec<ResolutionSurface>,
    /// The offline/mirror bundling declaration.
    pub offline_bundle: OfflineBundle,
    /// The published packages.
    pub packages: Vec<JsonSchemaPackage>,
    /// Summary counts.
    pub summary: M5JsonSchemaCatalogSummary,
}

impl M5JsonSchemaCatalog {
    /// Returns the package registered for `family_id`.
    pub fn package(&self, family_id: &str) -> Option<&JsonSchemaPackage> {
        self.packages.iter().find(|p| p.family_id == family_id)
    }

    /// Resolves the schema identifier and lifecycle label for a family.
    ///
    /// This is the lookup export/import, support export, and docs/help share so
    /// they quote one schema identity and one lifecycle label per family.
    pub fn resolve_schema_label(&self, family_id: &str) -> Option<(&str, LifecycleLabel)> {
        self.package(family_id)
            .map(|p| (p.schema_id.as_str(), p.lifecycle_label))
    }

    /// Packages published at or above the stable cutline.
    pub fn stable_packages(&self) -> Vec<&JsonSchemaPackage> {
        self.packages
            .iter()
            .filter(|p| p.publishes_stable())
            .collect()
    }

    /// Packages publishing at a given lifecycle label.
    pub fn packages_for_label(&self, label: LifecycleLabel) -> Vec<&JsonSchemaPackage> {
        self.packages
            .iter()
            .filter(|p| p.lifecycle_label == label)
            .collect()
    }

    /// Recomputes the summary block from the packages.
    pub fn computed_summary(&self) -> M5JsonSchemaCatalogSummary {
        let count =
            |f: &dyn Fn(&JsonSchemaPackage) -> bool| self.packages.iter().filter(|p| f(p)).count();
        M5JsonSchemaCatalogSummary {
            total_packages: self.packages.len(),
            stable_label_packages: count(&|p| p.lifecycle_label == LifecycleLabel::Stable),
            beta_label_packages: count(&|p| p.lifecycle_label == LifecycleLabel::Beta),
            preserve_unknown_packages: count(&|p| p.preserves_unknown_fields()),
            packages_with_migration_hooks: count(&|p| {
                !p.field_contract.migration_note_hooks.is_empty()
            }),
            packages_with_roundtrip_fixture: count(&|p| !p.roundtrip_fixture_ref.is_empty()),
            schema_files: self.packages.len(),
            example_payloads: self.packages.len(),
            roundtrip_fixtures: self.packages.len(),
        }
    }

    /// Validates the catalog's structural invariants.
    ///
    /// Mirrors the CI validator's semantic invariants. The checked-in catalog
    /// returns no violations; each negative fixture returns at least one.
    pub fn validate(&self) -> Vec<M5JsonSchemaCatalogViolation> {
        let mut out = Vec::new();
        let mut push = |check_id: &str, detail: String| {
            out.push(M5JsonSchemaCatalogViolation {
                check_id: check_id.to_string(),
                detail,
            })
        };

        if self.schema_version != M5_JSON_SCHEMA_CATALOG_SCHEMA_VERSION {
            push(
                "catalog.schema_version",
                format!("unexpected schema_version {}", self.schema_version),
            );
        }
        if self.record_kind != M5_JSON_SCHEMA_CATALOG_RECORD_KIND {
            push(
                "catalog.record_kind",
                format!("unexpected record_kind {}", self.record_kind),
            );
        }
        if self.catalog_id != M5_JSON_SCHEMA_CATALOG_ID {
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
        if self.additive_field_rules != AdditiveFieldRule::ALL {
            push(
                "vocab.additive_field_rules",
                "additive_field_rules off the canonical list".into(),
            );
        }
        if self.required_field_policies != RequiredFieldPolicy::ALL {
            push(
                "vocab.required_field_policies",
                "required_field_policies off the canonical list".into(),
            );
        }
        if self.unknown_field_policies != UnknownFieldPolicy::ALL {
            push(
                "vocab.unknown_field_policies",
                "unknown_field_policies off the canonical list".into(),
            );
        }
        if self.downgrade_behaviors != DowngradeBehavior::ALL {
            push(
                "vocab.downgrade_behaviors",
                "downgrade_behaviors off the canonical list".into(),
            );
        }
        if self.resolution_surfaces != ResolutionSurface::ALL {
            push(
                "vocab.resolution_surfaces",
                "resolution_surfaces off the canonical list".into(),
            );
        }

        let mut seen_pkg: BTreeSet<&str> = BTreeSet::new();
        let mut seen_family: BTreeSet<&str> = BTreeSet::new();
        for pkg in &self.packages {
            if !seen_pkg.insert(pkg.package_id.as_str()) {
                push(
                    "packages.duplicate_package_id",
                    format!("duplicate package_id {}", pkg.package_id),
                );
            }
            if !seen_family.insert(pkg.family_id.as_str()) {
                push(
                    "packages.duplicate_family_id",
                    format!("duplicate family_id {}", pkg.family_id),
                );
            }
            if pkg.package_id != format!("m5.{}", pkg.family_id) {
                push(
                    "packages.package_id_shape",
                    format!("{}: package_id must be 'm5.<family_id>'", pkg.family_id),
                );
            }
            if pkg.record_kind_field != "record_kind" {
                push(
                    "packages.record_kind_field",
                    format!("{}: record_kind_field must be 'record_kind'", pkg.family_id),
                );
            }
            if pkg.version_field_names.is_empty() {
                push(
                    "packages.empty_version_fields",
                    format!("{}: empty version_field_names", pkg.family_id),
                );
            }
            if !pkg.version_field_names.contains(&pkg.primary_version_field) {
                push(
                    "packages.primary_version_field",
                    format!(
                        "{}: primary_version_field not in version_field_names",
                        pkg.family_id
                    ),
                );
            }
            if pkg.field_contract.migration_note_hooks.is_empty() {
                push(
                    "packages.empty_migration_hooks",
                    format!("{}: empty migration_note_hooks", pkg.family_id),
                );
            }
        }

        if self.summary != self.computed_summary() {
            push(
                "summary.count_mismatch",
                "summary counts disagree with the packages".into(),
            );
        }

        out
    }
}

/// Parses the embedded checked-in catalog into the typed model.
pub fn current_m5_json_schema_catalog() -> Result<M5JsonSchemaCatalog, serde_json::Error> {
    serde_json::from_str(M5_JSON_SCHEMA_CATALOG_JSON)
}

#[cfg(test)]
mod tests;
