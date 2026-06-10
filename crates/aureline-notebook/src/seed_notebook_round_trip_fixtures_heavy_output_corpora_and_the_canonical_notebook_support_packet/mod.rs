//! Notebook round-trip fixtures, heavy-output corpora, and the canonical
//! notebook support packet.
//!
//! This module seeds the bounded fixture set and heavy-output corpus that keep
//! M5 notebook round-trip and output-truth claims evidence-based. It builds on
//! the canonical document model, save/repair/round-trip safety, and output-trust
//! records already landed in B1, and emits the canonical checked-in support
//! packet that downstream docs, help, CI, and support surfaces ingest instead of
//! cloning status text.
//!
//! The module exposes:
//!
//! - the [`NotebookRoundTripFixture`] record that describes a seed fixture for
//!   round-trip testing (fixture kind, exercised assertion kinds, expected
//!   result, and loss posture);
//! - the [`HeavyOutputCorpusEntry`] record that describes a heavy-output
//!   scenario (size bucket, output count, rich-output presence, trust
//!   implication, and virtualization strategy);
//! - the [`NotebookSupportPacket`] checked-in artifact that downstream
//!   surfaces ingest as the canonical control source for this lane.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw attachment bytes, raw
//! output bytes, and raw URLs MUST NOT appear on any record carried here. Only
//! opaque handles and closed-vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every record carried by this module. Bumped only
/// on breaking payload changes; additive-optional fields do not bump this
/// value.
pub const NOTEBOOK_SUPPORT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookRoundTripFixture`] payloads.
pub const NOTEBOOK_ROUND_TRIP_FIXTURE_RECORD_KIND: &str = "notebook_round_trip_fixture";

/// Stable record-kind tag for serialized [`HeavyOutputCorpusEntry`] payloads.
pub const HEAVY_OUTPUT_CORPUS_ENTRY_RECORD_KIND: &str = "heavy_output_corpus_entry";

/// Stable record-kind tag for the checked-in [`NotebookSupportPacket`].
pub const NOTEBOOK_SUPPORT_PACKET_RECORD_KIND: &str = "notebook_support_packet";

/// Repo-relative path to the checked-in support packet JSON.
pub const NOTEBOOK_SUPPORT_PACKET_PATH: &str =
    "artifacts/notebook/m5/seed_notebook_round_trip_fixtures_heavy_output_corpora_and_the_canonical_notebook_support_packet.json";

/// Embedded checked-in support packet JSON.
pub const NOTEBOOK_SUPPORT_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/seed_notebook_round_trip_fixtures_heavy_output_corpora_and_the_canonical_notebook_support_packet.json"
));

macro_rules! closed_vocab {
    (
        $(#[$type_doc:meta])*
        $name:ident {
            $(
                $(#[$variant_doc:meta])*
                $variant:ident => $token:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$type_doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum $name {
            $(
                $(#[$variant_doc])*
                #[serde(rename = $token)]
                $variant
            ),+
        }

        impl $name {
            /// Stable closed-vocabulary token recorded in records, schemas,
            /// fixtures, and exports.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $token),+
                }
            }
        }
    };
}

closed_vocab!(
    /// Fixture-kind class. Names the specific round-trip fixture scenario so
    /// the test matrix and CI surfaces know exactly what is being exercised.
    NotebookRoundTripFixtureKindClass {
        CleanCanonical => "clean_canonical",
        AttachmentHeavy => "attachment_heavy",
        MetadataRich => "metadata_rich",
        UnknownNamespaceDense => "unknown_namespace_dense",
        CorruptedThenRepaired => "corrupted_then_repaired",
        ExportOnly => "export_only",
        NoKernelEditable => "no_kernel_editable",
        CellIdStress => "cell_id_stress",
    }
);

closed_vocab!(
    /// Size-bucket class for heavy-output corpus entries. Distinguishes small,
    /// medium, large, and very-large total output sizes so virtualization and
    /// trust decisions are explicit.
    HeavyOutputCorpusSizeBucketClass {
        Small => "small",
        Medium => "medium",
        Large => "large",
        VeryLarge => "very_large",
    }
);

closed_vocab!(
    /// Trust-implication class for heavy-output corpus entries. Names whether
    /// the output is trusted inline, trusted but virtualized, sanitized inline,
    /// sanitized and virtualized, sandboxed, or blocked by policy.
    HeavyOutputCorpusTrustImplicationClass {
        TrustedInline => "trusted_inline",
        TrustedVirtualized => "trusted_virtualized",
        SanitizedInline => "sanitized_inline",
        SanitizedVirtualized => "sanitized_virtualized",
        Sandboxed => "sandboxed",
        Blocked => "blocked",
    }
);

closed_vocab!(
    /// Virtualization class for heavy-output corpus entries. Names the display
    /// strategy applied to heavy outputs so the chrome never freezes or
    /// silently truncates.
    HeavyOutputCorpusVirtualizationClass {
        None => "none",
        Truncated => "truncated",
        Paginated => "paginated",
        Externalized => "externalized",
        LazyLoaded => "lazy_loaded",
    }
);

closed_vocab!(
    /// Coverage class for the support packet. Names whether the packet claims
    /// full coverage, partial coverage, fixture-only coverage, or corpus-only
    /// coverage.
    NotebookSupportPacketCoverageClass {
        Full => "full",
        Partial => "partial",
        FixtureOnly => "fixture_only",
        CorpusOnly => "corpus_only",
    }
);

/// Generic finding shape used by every record validator in this module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookSupportFinding {
    /// Stable check id (e.g. `notebook_round_trip_fixture.loss_summary_required`).
    pub check_id: String,
    /// Subject row id (record id, fixture id, corpus entry id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl NotebookSupportFinding {
    fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

/// Typed validation finding for a [`NotebookRoundTripFixture`].
pub type NotebookRoundTripFixtureFinding = NotebookSupportFinding;

/// Typed validation finding for a [`HeavyOutputCorpusEntry`].
pub type HeavyOutputCorpusEntryFinding = NotebookSupportFinding;

/// Typed validation finding for a [`NotebookSupportPacket`].
pub type NotebookSupportPacketFinding = NotebookSupportFinding;

/// Canonical notebook round-trip fixture record. Describes a seed fixture
/// scenario for round-trip testing, the assertion kinds it exercises, the
/// expected result, and any expected loss.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookRoundTripFixture {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_support_schema_version: u32,
    /// Stable opaque fixture id.
    pub fixture_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Fixture-kind class.
    pub fixture_kind_class: NotebookRoundTripFixtureKindClass,
    /// Opaque refs to the round-trip assertion kinds this fixture exercises.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assertion_kind_class_refs: Vec<String>,
    /// Expected result class token (e.g. `pass`, `fail`, `partial`,
    /// `blocked_by_format_boundary`).
    pub expected_result_class: String,
    /// Loss summary when expected result is not `pass`; MUST be `None` when
    /// expected result is `pass`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loss_summary: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookRoundTripFixture {
    /// Returns typed truth-rule findings; an empty vector means the fixture is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookRoundTripFixtureFinding> {
        let mut findings = Vec::new();
        let subject = self.fixture_id.as_str();

        if self.record_kind != NOTEBOOK_ROUND_TRIP_FIXTURE_RECORD_KIND {
            findings.push(NotebookRoundTripFixtureFinding::new(
                "notebook_round_trip_fixture.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_ROUND_TRIP_FIXTURE_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_support_schema_version != NOTEBOOK_SUPPORT_SCHEMA_VERSION {
            findings.push(NotebookRoundTripFixtureFinding::new(
                "notebook_round_trip_fixture.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_SUPPORT_SCHEMA_VERSION}, found {}",
                    self.notebook_support_schema_version
                ),
            ));
        }

        let is_pass = self.expected_result_class == "pass";

        if !is_pass && self.loss_summary.is_none() {
            findings.push(NotebookRoundTripFixtureFinding::new(
                "notebook_round_trip_fixture.loss_summary_required",
                subject,
                "non-pass expected_result_class requires a loss_summary",
            ));
        }
        if is_pass && self.loss_summary.is_some() {
            findings.push(NotebookRoundTripFixtureFinding::new(
                "notebook_round_trip_fixture.loss_summary_not_allowed",
                subject,
                "pass expected_result_class must not carry a loss_summary",
            ));
        }

        findings
    }
}

/// Canonical heavy-output corpus entry record. Describes a heavy-output
/// notebook scenario, its size bucket, trust implication, and virtualization
/// strategy so the chrome never silently freezes or escalates trust.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeavyOutputCorpusEntry {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_support_schema_version: u32,
    /// Stable opaque corpus entry id.
    pub corpus_entry_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Size-bucket class.
    pub size_bucket_class: HeavyOutputCorpusSizeBucketClass,
    /// Number of outputs in the notebook.
    pub output_count: u32,
    /// Whether the notebook contains rich outputs (images, widgets, HTML).
    pub contains_rich_output: bool,
    /// Trust-implication class.
    pub trust_implication_class: HeavyOutputCorpusTrustImplicationClass,
    /// Virtualization class.
    pub virtualization_class: HeavyOutputCorpusVirtualizationClass,
    /// Export-safe summary line.
    pub summary: String,
}

impl HeavyOutputCorpusEntry {
    /// Returns typed truth-rule findings; an empty vector means the entry is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<HeavyOutputCorpusEntryFinding> {
        let mut findings = Vec::new();
        let subject = self.corpus_entry_id.as_str();

        if self.record_kind != HEAVY_OUTPUT_CORPUS_ENTRY_RECORD_KIND {
            findings.push(HeavyOutputCorpusEntryFinding::new(
                "heavy_output_corpus_entry.record_kind",
                subject,
                format!(
                    "record_kind must be '{HEAVY_OUTPUT_CORPUS_ENTRY_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_support_schema_version != NOTEBOOK_SUPPORT_SCHEMA_VERSION {
            findings.push(HeavyOutputCorpusEntryFinding::new(
                "heavy_output_corpus_entry.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_SUPPORT_SCHEMA_VERSION}, found {}",
                    self.notebook_support_schema_version
                ),
            ));
        }

        if self.output_count == 0 {
            findings.push(HeavyOutputCorpusEntryFinding::new(
                "heavy_output_corpus_entry.output_count",
                subject,
                "output_count must be greater than zero",
            ));
        }

        if matches!(
            self.size_bucket_class,
            HeavyOutputCorpusSizeBucketClass::Small
        ) && !matches!(
            self.virtualization_class,
            HeavyOutputCorpusVirtualizationClass::None
        ) {
            findings.push(HeavyOutputCorpusEntryFinding::new(
                "heavy_output_corpus_entry.small_no_virtualization",
                subject,
                "small size bucket must use none virtualization",
            ));
        }

        findings
    }
}

/// Checked-in notebook support packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookSupportPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: fixture-kind classes.
    pub fixture_kind_classes: Vec<NotebookRoundTripFixtureKindClass>,
    /// Closed vocabulary: size-bucket classes.
    pub size_bucket_classes: Vec<HeavyOutputCorpusSizeBucketClass>,
    /// Closed vocabulary: trust-implication classes.
    pub trust_implication_classes: Vec<HeavyOutputCorpusTrustImplicationClass>,
    /// Closed vocabulary: virtualization classes.
    pub virtualization_classes: Vec<HeavyOutputCorpusVirtualizationClass>,
    /// Closed vocabulary: coverage classes.
    pub coverage_classes: Vec<NotebookSupportPacketCoverageClass>,
    /// Worked example round-trip fixtures.
    pub example_round_trip_fixtures: Vec<NotebookRoundTripFixture>,
    /// Worked example heavy-output corpus entries.
    pub example_heavy_output_corpus_entries: Vec<HeavyOutputCorpusEntry>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookSupportPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookSupportPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_SUPPORT_SCHEMA_VERSION {
            findings.push(NotebookSupportPacketFinding::new(
                "notebook_support_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_SUPPORT_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_SUPPORT_PACKET_RECORD_KIND {
            findings.push(NotebookSupportPacketFinding::new(
                "notebook_support_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_SUPPORT_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.fixture_kind_classes.len() != NotebookRoundTripFixtureKindClass::ALL.len() {
            findings.push(NotebookSupportPacketFinding::new(
                "notebook_support_packet.fixture_kind_classes_coverage",
                subject,
                "fixture_kind_classes must list every variant",
            ));
        }
        if self.size_bucket_classes.len() != HeavyOutputCorpusSizeBucketClass::ALL.len() {
            findings.push(NotebookSupportPacketFinding::new(
                "notebook_support_packet.size_bucket_classes_coverage",
                subject,
                "size_bucket_classes must list every variant",
            ));
        }
        if self.trust_implication_classes.len() != HeavyOutputCorpusTrustImplicationClass::ALL.len()
        {
            findings.push(NotebookSupportPacketFinding::new(
                "notebook_support_packet.trust_implication_classes_coverage",
                subject,
                "trust_implication_classes must list every variant",
            ));
        }
        if self.virtualization_classes.len() != HeavyOutputCorpusVirtualizationClass::ALL.len() {
            findings.push(NotebookSupportPacketFinding::new(
                "notebook_support_packet.virtualization_classes_coverage",
                subject,
                "virtualization_classes must list every variant",
            ));
        }
        if self.coverage_classes.len() != NotebookSupportPacketCoverageClass::ALL.len() {
            findings.push(NotebookSupportPacketFinding::new(
                "notebook_support_packet.coverage_classes_coverage",
                subject,
                "coverage_classes must list every variant",
            ));
        }

        for fixture in &self.example_round_trip_fixtures {
            findings.extend(fixture.validate().into_iter().map(|f| {
                NotebookSupportPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for entry in &self.example_heavy_output_corpus_entries {
            findings.extend(entry.validate().into_iter().map(|f| {
                NotebookSupportPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

impl NotebookRoundTripFixtureKindClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CleanCanonical,
        Self::AttachmentHeavy,
        Self::MetadataRich,
        Self::UnknownNamespaceDense,
        Self::CorruptedThenRepaired,
        Self::ExportOnly,
        Self::NoKernelEditable,
        Self::CellIdStress,
    ];
}

impl HeavyOutputCorpusSizeBucketClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [Self::Small, Self::Medium, Self::Large, Self::VeryLarge];
}

impl HeavyOutputCorpusTrustImplicationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TrustedInline,
        Self::TrustedVirtualized,
        Self::SanitizedInline,
        Self::SanitizedVirtualized,
        Self::Sandboxed,
        Self::Blocked,
    ];
}

impl HeavyOutputCorpusVirtualizationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::None,
        Self::Truncated,
        Self::Paginated,
        Self::Externalized,
        Self::LazyLoaded,
    ];
}

impl NotebookSupportPacketCoverageClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Full,
        Self::Partial,
        Self::FixtureOnly,
        Self::CorpusOnly,
    ];
}

/// Parses the checked-in notebook support packet JSON.
pub fn current_notebook_support_packet() -> Result<NotebookSupportPacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_SUPPORT_PACKET_JSON)
}

#[cfg(test)]
mod tests;
