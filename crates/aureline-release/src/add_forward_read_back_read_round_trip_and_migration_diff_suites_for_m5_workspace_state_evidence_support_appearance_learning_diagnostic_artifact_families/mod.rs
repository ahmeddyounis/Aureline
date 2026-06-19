//! Typed M5 reader/writer compatibility suite: the canonical proof that every
//! durable M5 artifact family is reader/writer compatible across versions.
//!
//! Where the JSON Schema catalog speaks for the *package* each durable M5
//! artifact family publishes, and the public-contract publication matrix speaks
//! for *whether* a family has published the contract forms it needs, this suite
//! speaks for the *compatibility behavior*: for every family the catalog
//! publishes it carries checked-in prior/current/unsupported fixtures and a
//! migration-diff report proving forward-read, back-read, round-trip,
//! migration-diff, unknown-field preservation, additive-field tolerance,
//! downgrade narrowing, and the compare-only fallback. Each [`FamilyCompatSuite`]
//! binds one family to:
//!
//! - its [`ReaderWriterPosture`] (reused verbatim from the publication matrix)
//!   and the [`WriteBackPosture`] it derives — `reader_only` families are
//!   [`WriteBackPosture::CompareOnly`] (read and diff, never write back the
//!   user-owned artifact, a passing documented state); every other posture is
//!   [`WriteBackPosture::BackupThenWrite`] (write-back only with
//!   backup/compare-first),
//! - its prior/current/unsupported version triple and the additive optional field
//!   the current version adds,
//! - its checked-in fixtures and per-family migration-diff report, and
//! - its [`CompatCase`] set: one case per compatibility behavior, naming the
//!   reader and writer versions, the input fixture, the [`ExpectedOutcome`],
//!   whether unknown fields are preserved, and whether the case writes back.
//!
//! The suite turns compatibility from one-time release-note prose into repeatable
//! fixtures and diff reports: release and support packets link directly to the
//! [`M5ReaderWriterCompatSuite::operator_report_ref`] and the per-family
//! [`MigrationDiffSummary::report_ref`]. The [`CaseKind::Downgrade`] case proves a
//! family at an unsupported newer version narrows below the launch cutline rather
//! than silently upgrading.
//!
//! The suite is checked in at [`M5_READER_WRITER_COMPAT_SUITE_PATH`] and embedded
//! here, so this typed consumer and the CI validator agree on every suite without
//! a cargo build in CI. The model is metadata-only: every field is a typed state
//! or an opaque repo-relative ref or URI. It carries no surface payloads, rendered
//! bodies, signatures, or credential material.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Supported suite schema version.
pub const M5_READER_WRITER_COMPAT_SUITE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the suite.
pub const M5_READER_WRITER_COMPAT_SUITE_RECORD_KIND: &str = "m5_reader_writer_compat_suite";

/// Stable suite identifier.
pub const M5_READER_WRITER_COMPAT_SUITE_ID: &str = "m5_reader_writer_compat_suite:v1";

/// Repo-relative path to the checked-in suite.
pub const M5_READER_WRITER_COMPAT_SUITE_PATH: &str =
    "artifacts/contracts/m5-reader-writer-compat-suite.json";

/// Embedded checked-in suite JSON.
pub const M5_READER_WRITER_COMPAT_SUITE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/contracts/m5-reader-writer-compat-suite.json"
));

/// One reader/writer compatibility behavior a family suite exercises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaseKind {
    /// A prior-version reader reads a current-version artifact.
    ForwardRead,
    /// A current-version reader reads a prior-version artifact.
    BackRead,
    /// A parse/serialize round-trip preserves every field.
    RoundTrip,
    /// The prior-to-current change is additive-only.
    MigrationDiff,
    /// Vendor and future fields survive the read.
    UnknownFieldPreservation,
    /// The field added at the current version is optional.
    AdditiveField,
    /// An artifact at an unsupported newer version narrows below the cutline.
    Downgrade,
    /// A compare-only family is read and diffed but never written back.
    CompareOnly,
}

impl CaseKind {
    /// Every case kind, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ForwardRead,
        Self::BackRead,
        Self::RoundTrip,
        Self::MigrationDiff,
        Self::UnknownFieldPreservation,
        Self::AdditiveField,
        Self::Downgrade,
        Self::CompareOnly,
    ];
}

/// The expected result of a compatibility case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpectedOutcome {
    /// The artifact is read (and, where the posture allows, written back).
    Compatible,
    /// The artifact is read and diffed, but intentionally never written back.
    CompatibleCompareOnly,
    /// The family narrows below the launch cutline rather than upgrading.
    Narrowed,
    /// The artifact is rejected.
    Rejected,
}

impl ExpectedOutcome {
    /// Every outcome, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Compatible,
        Self::CompatibleCompareOnly,
        Self::Narrowed,
        Self::Rejected,
    ];
}

/// The reader/writer posture a family publishes (reused from the matrix).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReaderWriterPosture {
    /// Read-only consumer.
    ReaderOnly,
    /// Write-only producer.
    WriterOnly,
    /// Both reads and writes.
    ReadWrite,
    /// Bidirectional interchange.
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

    /// The write-back posture this reader/writer posture derives.
    ///
    /// A reader-only family is compare-only; every other posture writes back with
    /// backup/compare-first behavior.
    pub fn write_back_posture(self) -> WriteBackPosture {
        match self {
            Self::ReaderOnly => WriteBackPosture::CompareOnly,
            Self::WriterOnly | Self::ReadWrite | Self::BidirectionalInterchange => {
                WriteBackPosture::BackupThenWrite
            }
        }
    }
}

/// Whether a family is written back, and how.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteBackPosture {
    /// Read and diff, never write back the user-owned artifact.
    CompareOnly,
    /// Write back only with backup/compare-first behavior.
    BackupThenWrite,
}

impl WriteBackPosture {
    /// Every write-back posture, in declaration order.
    pub const ALL: [Self; 2] = [Self::CompareOnly, Self::BackupThenWrite];
}

/// The class of a prior-to-current contract change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeClass {
    /// No change between versions.
    Unchanged,
    /// Optional fields added only.
    Additive,
    /// Behavior changed without a wire break.
    Behavioral,
    /// A breaking change.
    Breaking,
}

impl ChangeClass {
    /// Every change class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Unchanged,
        Self::Additive,
        Self::Behavioral,
        Self::Breaking,
    ];
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

/// A surface that resolves a family's compatibility evidence.
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

/// One reader/writer compatibility case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompatCase {
    /// Stable case identifier (`<family>.<case_kind>`).
    pub case_id: String,
    /// The compatibility behavior this case exercises.
    pub case_kind: CaseKind,
    /// The version the reader is pinned to.
    pub reader_version: u32,
    /// The version the writer produced.
    pub writer_version: u32,
    /// Ref to the checked-in input fixture.
    pub input_fixture_ref: String,
    /// The expected outcome.
    pub expected_outcome: ExpectedOutcome,
    /// True when the case preserves unknown fields.
    pub preserves_unknown_fields: bool,
    /// True when the case writes the artifact back.
    pub writes_back: bool,
    /// True when a write-back is backup/compare-first.
    pub backup_first: bool,
    /// Human-readable note.
    pub note: String,
}

/// A compact prior-to-current migration diff, mirroring the standalone report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MigrationDiffSummary {
    /// The prior version.
    pub from_version: u32,
    /// The current version.
    pub to_version: u32,
    /// The class of the change.
    pub change_class: ChangeClass,
    /// True when the change is reader/writer compatible.
    pub compatible: bool,
    /// Fields added by the current version.
    pub added_fields: Vec<String>,
    /// Fields removed by the current version.
    pub removed_fields: Vec<String>,
    /// Fields whose type or meaning changed.
    pub changed_fields: Vec<String>,
    /// Ref to the standalone per-family migration-diff report.
    pub report_ref: String,
}

/// One family's reader/writer compatibility suite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyCompatSuite {
    /// Catalog family id (unique per suite).
    pub family_id: String,
    /// Stable package identifier (`m5.<family_id>`).
    pub package_id: String,
    /// Contract-family registry id.
    pub registry_family_id: String,
    /// Human-readable title.
    pub title: String,
    /// One-line summary.
    pub summary: String,
    /// The published contract form.
    pub contract_form: ContractForm,
    /// The contract-family maturity lane.
    pub maturity_lane: MaturityLane,
    /// The lifecycle label this family publishes.
    pub lifecycle_label: LifecycleLabel,
    /// The lifecycle label the matrix publishes after narrowing.
    pub published_label: LifecycleLabel,
    /// The reader/writer posture (from the matrix).
    pub reader_writer_posture: ReaderWriterPosture,
    /// The write-back posture this suite derives.
    pub write_back_posture: WriteBackPosture,
    /// What happens when required publication evidence is lost.
    pub downgrade_behavior: DowngradeBehavior,
    /// The record-kind tag value for this family.
    pub record_kind_value: String,
    /// The primary in-band schema version field.
    pub primary_version_field: String,
    /// Every in-band schema version field.
    pub version_field_names: Vec<String>,
    /// The primary stable object identity field.
    pub primary_identifier_field: String,
    /// The prior (published) version.
    pub prior_version: u32,
    /// The current (additive-minor-bumped) version.
    pub current_version: u32,
    /// A version beyond the published ceiling.
    pub unsupported_version: u32,
    /// The additive optional field the current version adds.
    pub added_field: String,
    /// Ref to the family's fixture directory.
    pub fixture_dir: String,
    /// Ref to the prior-version fixture.
    pub prior_fixture_ref: String,
    /// Ref to the current-version fixture.
    pub current_fixture_ref: String,
    /// Ref to the unsupported-version fixture.
    pub unsupported_fixture_ref: String,
    /// The compact prior-to-current migration diff.
    pub migration_diff: MigrationDiffSummary,
    /// The family's stable schema identifier (`$id`).
    pub schema_id: String,
    /// Repo-relative path to the family's package schema.
    pub schema_path: String,
    /// Ref to the family's JSON Schema catalog package.
    pub catalog_package_ref: String,
    /// Ref to the publication-matrix row.
    pub matrix_row_ref: String,
    /// Ref to the contract-family registry row.
    pub contract_family_ref: String,
    /// Ref to the doc that carries the family's compatibility note.
    pub compatibility_note_ref: String,
    /// Refs to the validators that gate this suite.
    pub validator_suite_refs: Vec<String>,
    /// Surfaces that resolve this suite's compatibility evidence.
    pub resolution_surfaces: Vec<ResolutionSurface>,
    /// The compatibility cases.
    pub cases: Vec<CompatCase>,
}

impl FamilyCompatSuite {
    /// True when this family is written back (with backup/compare-first).
    pub fn writes_back(&self) -> bool {
        self.write_back_posture == WriteBackPosture::BackupThenWrite
    }

    /// The case registered for `kind`, if any.
    pub fn case(&self, kind: CaseKind) -> Option<&CompatCase> {
        self.cases.iter().find(|c| c.case_kind == kind)
    }
}

/// The offline/mirror bundling declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OfflineBundle {
    /// True when the suite bundles into mirror artifact sets.
    pub mirrorable: bool,
    /// True when validation requires runtime service access.
    pub requires_runtime_service: bool,
    /// Bundle members (suite, fixtures, reports, operator report, validator).
    pub bundle_members: Vec<String>,
    /// Human-readable note.
    pub note: String,
}

/// Summary counts over the suite set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ReaderWriterCompatSuiteSummary {
    /// Total family suites.
    pub total_suites: usize,
    /// Suites that write back.
    pub write_back_suites: usize,
    /// Suites that are compare-only.
    pub compare_only_suites: usize,
    /// Total compatibility cases.
    pub total_cases: usize,
    /// Forward-read cases.
    pub forward_read_cases: usize,
    /// Back-read cases.
    pub back_read_cases: usize,
    /// Round-trip cases.
    pub round_trip_cases: usize,
    /// Migration-diff cases.
    pub migration_diff_cases: usize,
    /// Unknown-field-preservation cases.
    pub unknown_field_cases: usize,
    /// Additive-field cases.
    pub additive_field_cases: usize,
    /// Downgrade cases.
    pub downgrade_cases: usize,
    /// Compare-only cases.
    pub compare_only_cases: usize,
    /// Cases expecting a narrow-below-cutline outcome.
    pub narrowing_cases: usize,
    /// Migration-diff reports.
    pub migration_diff_reports: usize,
    /// Families whose prior-to-current change is additive.
    pub families_with_additive_change: usize,
    /// Suites whose cases preserve unknown fields.
    pub suites_preserving_unknown: usize,
    /// Checked-in fixtures (prior/current/unsupported per family).
    pub fixtures_total: usize,
}

/// A structural validation violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ReaderWriterCompatSuiteViolation {
    /// Stable check id.
    pub check_id: String,
    /// Human-readable detail.
    pub detail: String,
}

/// The typed M5 reader/writer compatibility suite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ReaderWriterCompatSuite {
    /// Suite schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable suite identifier.
    pub suite_id: String,
    /// Lifecycle status of this suite artifact.
    pub status: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// Evidence packet.
    pub evidence_page: String,
    /// SDK catalog doc.
    pub sdk_catalog_page: String,
    /// Ref to the JSON Schema catalog.
    pub json_schema_catalog_ref: String,
    /// Ref to the public-contract publication matrix.
    pub publication_matrix_ref: String,
    /// Ref to the contract-family registry.
    pub contract_family_registry_ref: String,
    /// Ref to the canonical M5 evidence index.
    pub evidence_index_ref: String,
    /// Home of the per-family fixtures.
    pub fixture_home: String,
    /// Home of the per-family migration-diff reports.
    pub migration_diff_report_home: String,
    /// Ref to the operator-facing report.
    pub operator_report_ref: String,
    /// Closed case-kind vocabulary.
    pub case_kinds: Vec<CaseKind>,
    /// Closed expected-outcome vocabulary.
    pub expected_outcomes: Vec<ExpectedOutcome>,
    /// Closed reader/writer-posture vocabulary.
    pub reader_writer_postures: Vec<ReaderWriterPosture>,
    /// Closed write-back-posture vocabulary.
    pub write_back_postures: Vec<WriteBackPosture>,
    /// Closed change-class vocabulary.
    pub change_classes: Vec<ChangeClass>,
    /// Closed downgrade-behavior vocabulary.
    pub downgrade_behaviors: Vec<DowngradeBehavior>,
    /// Closed resolution-surface vocabulary.
    pub resolution_surfaces: Vec<ResolutionSurface>,
    /// The offline/mirror bundling declaration.
    pub offline_bundle: OfflineBundle,
    /// The per-family suites.
    pub suites: Vec<FamilyCompatSuite>,
    /// Summary counts.
    pub summary: M5ReaderWriterCompatSuiteSummary,
}

impl M5ReaderWriterCompatSuite {
    /// Returns the suite registered for `family_id`.
    pub fn suite(&self, family_id: &str) -> Option<&FamilyCompatSuite> {
        self.suites.iter().find(|s| s.family_id == family_id)
    }

    /// Suites that write back.
    pub fn write_back_suites(&self) -> Vec<&FamilyCompatSuite> {
        self.suites.iter().filter(|s| s.writes_back()).collect()
    }

    /// Suites that are compare-only.
    pub fn compare_only_suites(&self) -> Vec<&FamilyCompatSuite> {
        self.suites.iter().filter(|s| !s.writes_back()).collect()
    }

    /// Recomputes the summary block from the suites.
    pub fn computed_summary(&self) -> M5ReaderWriterCompatSuiteSummary {
        let case_count = |kind: CaseKind| {
            self.suites
                .iter()
                .flat_map(|s| &s.cases)
                .filter(|c| c.case_kind == kind)
                .count()
        };
        let total_cases: usize = self.suites.iter().map(|s| s.cases.len()).sum();
        M5ReaderWriterCompatSuiteSummary {
            total_suites: self.suites.len(),
            write_back_suites: self.suites.iter().filter(|s| s.writes_back()).count(),
            compare_only_suites: self.suites.iter().filter(|s| !s.writes_back()).count(),
            total_cases,
            forward_read_cases: case_count(CaseKind::ForwardRead),
            back_read_cases: case_count(CaseKind::BackRead),
            round_trip_cases: case_count(CaseKind::RoundTrip),
            migration_diff_cases: case_count(CaseKind::MigrationDiff),
            unknown_field_cases: case_count(CaseKind::UnknownFieldPreservation),
            additive_field_cases: case_count(CaseKind::AdditiveField),
            downgrade_cases: case_count(CaseKind::Downgrade),
            compare_only_cases: case_count(CaseKind::CompareOnly),
            narrowing_cases: self
                .suites
                .iter()
                .flat_map(|s| &s.cases)
                .filter(|c| c.expected_outcome == ExpectedOutcome::Narrowed)
                .count(),
            migration_diff_reports: self.suites.len(),
            families_with_additive_change: self
                .suites
                .iter()
                .filter(|s| s.migration_diff.change_class == ChangeClass::Additive)
                .count(),
            suites_preserving_unknown: self.suites.len(),
            fixtures_total: 3 * self.suites.len(),
        }
    }

    /// Validates the suite's structural invariants.
    ///
    /// Mirrors the CI validator's semantic invariants. The checked-in suite
    /// returns no violations; each negative fixture returns at least one.
    pub fn validate(&self) -> Vec<M5ReaderWriterCompatSuiteViolation> {
        let mut out = Vec::new();
        let mut push = |check_id: &str, detail: String| {
            out.push(M5ReaderWriterCompatSuiteViolation {
                check_id: check_id.to_string(),
                detail,
            })
        };

        if self.schema_version != M5_READER_WRITER_COMPAT_SUITE_SCHEMA_VERSION {
            push(
                "suite.schema_version",
                format!("unexpected schema_version {}", self.schema_version),
            );
        }
        if self.record_kind != M5_READER_WRITER_COMPAT_SUITE_RECORD_KIND {
            push(
                "suite.record_kind",
                format!("unexpected record_kind {}", self.record_kind),
            );
        }
        if self.suite_id != M5_READER_WRITER_COMPAT_SUITE_ID {
            push(
                "suite.suite_id",
                format!("unexpected suite_id {}", self.suite_id),
            );
        }

        if self.case_kinds != CaseKind::ALL {
            push(
                "vocab.case_kinds",
                "case_kinds off the canonical list".into(),
            );
        }
        if self.expected_outcomes != ExpectedOutcome::ALL {
            push(
                "vocab.expected_outcomes",
                "expected_outcomes off the canonical list".into(),
            );
        }
        if self.reader_writer_postures != ReaderWriterPosture::ALL {
            push(
                "vocab.reader_writer_postures",
                "reader_writer_postures off the canonical list".into(),
            );
        }
        if self.write_back_postures != WriteBackPosture::ALL {
            push(
                "vocab.write_back_postures",
                "write_back_postures off the canonical list".into(),
            );
        }
        if self.change_classes != ChangeClass::ALL {
            push(
                "vocab.change_classes",
                "change_classes off the canonical list".into(),
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

        let mut seen_family: BTreeSet<&str> = BTreeSet::new();
        for s in &self.suites {
            let fid = s.family_id.as_str();
            if !seen_family.insert(fid) {
                push(
                    "suites.duplicate_family",
                    format!("duplicate family_id {fid}"),
                );
            }
            if s.package_id != format!("m5.{fid}") {
                push(
                    "suites.package_id_shape",
                    format!("{fid}: package_id must be 'm5.<family_id>'"),
                );
            }
            if s.write_back_posture != s.reader_writer_posture.write_back_posture() {
                push(
                    "suites.write_back_posture",
                    format!("{fid}: write_back_posture disagrees with reader_writer_posture"),
                );
            }
            if !(s.prior_version < s.current_version && s.current_version < s.unsupported_version) {
                push(
                    "suites.version_triple",
                    format!("{fid}: version triple must be strictly increasing"),
                );
            }
            if s.version_field_names.is_empty() {
                push(
                    "suites.empty_version_fields",
                    format!("{fid}: empty version_field_names"),
                );
            }
            if !s.version_field_names.contains(&s.primary_version_field) {
                push(
                    "suites.primary_version_field",
                    format!("{fid}: primary_version_field not in version_field_names"),
                );
            }

            if s.cases.is_empty() {
                push("suites.no_cases", format!("{fid}: no cases"));
            }
            let kinds: BTreeSet<CaseKind> = s.cases.iter().map(|c| c.case_kind).collect();
            for required in [
                CaseKind::ForwardRead,
                CaseKind::BackRead,
                CaseKind::AdditiveField,
                CaseKind::UnknownFieldPreservation,
                CaseKind::MigrationDiff,
                CaseKind::Downgrade,
            ] {
                if !kinds.contains(&required) {
                    push(
                        "suites.missing_case_kind",
                        format!("{fid}: missing required case kind {required:?}"),
                    );
                }
            }
            let writes_back = s.writes_back();
            if writes_back && !kinds.contains(&CaseKind::RoundTrip) {
                push(
                    "suites.missing_round_trip",
                    format!("{fid}: write-back family needs a round_trip case"),
                );
            }
            if !writes_back && !kinds.contains(&CaseKind::CompareOnly) {
                push(
                    "suites.missing_compare_only",
                    format!("{fid}: compare-only family needs a compare_only case"),
                );
            }
            if writes_back && kinds.contains(&CaseKind::CompareOnly) {
                push(
                    "suites.unexpected_compare_only",
                    format!("{fid}: write-back family must not carry a compare_only case"),
                );
            }
            if !writes_back && kinds.contains(&CaseKind::RoundTrip) {
                push(
                    "suites.unexpected_round_trip",
                    format!("{fid}: compare-only family must not carry a round_trip case"),
                );
            }

            for c in &s.cases {
                if !writes_back && c.writes_back {
                    push(
                        "cases.compare_only_writes_back",
                        format!("{fid}: compare-only family case writes back"),
                    );
                }
                if c.backup_first && !c.writes_back {
                    push(
                        "cases.backup_without_write",
                        format!("{fid}: backup_first set without writes_back"),
                    );
                }
                if c.case_kind == CaseKind::Downgrade
                    && c.expected_outcome != ExpectedOutcome::Narrowed
                {
                    push(
                        "cases.downgrade_outcome",
                        format!("{fid}: downgrade case must expect narrowed"),
                    );
                }
                if !CaseKind::ALL.contains(&c.case_kind) {
                    push(
                        "cases.unknown_case_kind",
                        format!("{fid}: case_kind off the canonical list"),
                    );
                }
            }

            let diff = &s.migration_diff;
            if diff.change_class != ChangeClass::Additive {
                push(
                    "suites.change_class",
                    format!("{fid}: migration_diff change_class must be additive"),
                );
            }
            if !diff.compatible {
                push(
                    "suites.diff_incompatible",
                    format!("{fid}: migration_diff must be compatible"),
                );
            }
            if !diff.removed_fields.is_empty() {
                push(
                    "suites.diff_removed_fields",
                    format!("{fid}: additive migration_diff must remove no fields"),
                );
            }
            if !diff.changed_fields.is_empty() {
                push(
                    "suites.diff_changed_fields",
                    format!("{fid}: additive migration_diff must change no fields"),
                );
            }
            if diff.added_fields.is_empty() {
                push(
                    "suites.diff_no_added_fields",
                    format!("{fid}: additive migration_diff must add a field"),
                );
            }
        }

        if self.summary != self.computed_summary() {
            push(
                "summary.count_mismatch",
                "summary counts disagree with the suites".into(),
            );
        }

        out
    }
}

/// Parses the embedded checked-in suite into the typed model.
pub fn current_m5_reader_writer_compat_suite(
) -> Result<M5ReaderWriterCompatSuite, serde_json::Error> {
    serde_json::from_str(M5_READER_WRITER_COMPAT_SUITE_JSON)
}

#[cfg(test)]
mod tests;
