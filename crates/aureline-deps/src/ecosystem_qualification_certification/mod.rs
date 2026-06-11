//! Per-ecosystem dependency-intelligence, package-review, and code-quality or
//! scanner certification with a non-inheriting promotion gate.
//!
//! This module owns the canonical packet that certifies, for every marketed
//! ecosystem and every qualification lane, whether the lane carries a current
//! qualification packet and proof corpus of its own — rather than inheriting
//! trust from an adjacent lane or an older stable-line claim. Each
//! [`QualificationRow`] names its [`ClaimedEcosystem`], its [`QualificationLane`],
//! the maturity the underlying lane assessed, the certification freshness, and
//! the qualification packet and corpus that back it.
//!
//! The model is a promotion gate, not a label store. The maturity that may be
//! *published* for a row is derived deterministically from its inputs: a row
//! that is stale, mirror-blocked, scanner-underqualified, or missing
//! package/lockfile review or corpus evidence cannot publish a full
//! certification, and its [`NarrowingAction`] records exactly how the gate
//! narrowed it. Because [`QualificationRow::published_maturity`] and
//! [`QualificationRow::narrowing_action`] are validated against the recomputed
//! gate decision, release tooling can prove that underqualified rows narrow
//! automatically before publication and that no row publishes beyond what its
//! own evidence supports.
//!
//! Certification stays row-specific. The packet pins the claimed ecosystem and
//! lane vocabulary and requires one row for every (ecosystem, lane) cell, so a
//! strong ecosystem never implies maturity on an unrelated one, and a row never
//! covers an ecosystem the matrix does not claim.
//!
//! The packet is checked in at
//! `artifacts/deps/m5/ecosystem-qualification-certification.json` and embedded
//! here, so this typed consumer and any CI gate agree on every row without a
//! cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref.
//! It carries no credential bodies or raw provider payloads.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported ecosystem-qualification certification packet schema version.
pub const ECOSYSTEM_QUALIFICATION_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const ECOSYSTEM_QUALIFICATION_CERTIFICATION_RECORD_KIND: &str =
    "ecosystem_qualification_certification";

/// Repo-relative path to the checked-in packet.
pub const ECOSYSTEM_QUALIFICATION_CERTIFICATION_PATH: &str =
    "artifacts/deps/m5/ecosystem-qualification-certification.json";

/// Embedded checked-in packet JSON.
pub const ECOSYSTEM_QUALIFICATION_CERTIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/deps/m5/ecosystem-qualification-certification.json"
));

/// A marketed ecosystem the certification matrix makes claims about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimedEcosystem {
    /// Rust Cargo workspace and crate manifests.
    Cargo,
    /// Node package manifests using pnpm workspace semantics.
    NodePnpm,
    /// Python pip / project manifests.
    PythonPip,
}

impl ClaimedEcosystem {
    /// Every claimed ecosystem, in declaration order.
    pub const ALL: [Self; 3] = [Self::Cargo, Self::NodePnpm, Self::PythonPip];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cargo => "cargo",
            Self::NodePnpm => "node_pnpm",
            Self::PythonPip => "python_pip",
        }
    }
}

/// A qualification lane certified independently on every ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationLane {
    /// Advisory, vulnerability, license, notice, and SBOM intelligence.
    DependencyIntelligence,
    /// Package/manifest/lockfile mutation review.
    PackageReview,
    /// Live code-quality / quality-profile depth.
    CodeQuality,
    /// Imported scanner (SARIF) parity and maturity.
    ScannerImport,
}

impl QualificationLane {
    /// Every qualification lane, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DependencyIntelligence,
        Self::PackageReview,
        Self::CodeQuality,
        Self::ScannerImport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DependencyIntelligence => "dependency_intelligence",
            Self::PackageReview => "package_review",
            Self::CodeQuality => "code_quality",
            Self::ScannerImport => "scanner_import",
        }
    }
}

/// Maturity class of a qualification lane on an ecosystem.
///
/// Ordered low-to-high by [`MaturityClass::rank`]: an [`MaturityClass::Unsupported`]
/// lane carries no claim, and an [`MaturityClass::Certified`] lane carries a full,
/// current, evidence-backed claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaturityClass {
    /// Full, current, evidence-backed certification.
    Certified,
    /// Partial coverage; published as provisional/beta depth only.
    Provisional,
    /// Below the bar for any positive depth claim.
    Underqualified,
    /// Not supported on this ecosystem; carries no claim.
    Unsupported,
}

impl MaturityClass {
    /// Every maturity class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Certified,
        Self::Provisional,
        Self::Underqualified,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Provisional => "provisional",
            Self::Underqualified => "underqualified",
            Self::Unsupported => "unsupported",
        }
    }

    /// Monotonic rank; higher means a stronger claim.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Unsupported => 0,
            Self::Underqualified => 1,
            Self::Provisional => 2,
            Self::Certified => 3,
        }
    }

    /// The weaker (lower-rank) of two maturity classes.
    pub const fn min(self, other: Self) -> Self {
        if other.rank() < self.rank() {
            other
        } else {
            self
        }
    }
}

/// Freshness of a lane's certification relative to its freshness SLO.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationFreshness {
    /// Certification is current within its freshness SLO.
    Current,
    /// Certification is present but past its freshness SLO.
    Stale,
    /// Certification has expired and no longer backs a live claim.
    Expired,
    /// Certification freshness cannot be established.
    Unknown,
}

impl CertificationFreshness {
    /// Every freshness class, in declaration order.
    pub const ALL: [Self; 4] = [Self::Current, Self::Stale, Self::Expired, Self::Unknown];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::Unknown => "unknown",
        }
    }

    /// Whether the certification is current within its freshness SLO.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }

    /// Highest maturity this freshness alone permits a row to publish.
    ///
    /// Only a `current` certification may publish a full certification; a stale
    /// or unknown certification narrows to provisional, and an expired one
    /// narrows to underqualified.
    pub const fn maturity_ceiling(self) -> MaturityClass {
        match self {
            Self::Current => MaturityClass::Certified,
            Self::Stale | Self::Unknown => MaturityClass::Provisional,
            Self::Expired => MaturityClass::Underqualified,
        }
    }
}

/// A reason a lane cannot promote at full strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockingReason {
    /// The qualification packet is stale.
    Stale,
    /// Advisory/registry data is served from a blocked or unreachable mirror.
    MirrorBlocked,
    /// The imported scanner is below the parity bar for this lane.
    ScannerUnderqualified,
    /// Package/lockfile review evidence is missing for this lane.
    MissingPackageLockfileEvidence,
    /// The proof corpus is missing for this lane.
    MissingCorpus,
}

impl BlockingReason {
    /// Every blocking reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Stale,
        Self::MirrorBlocked,
        Self::ScannerUnderqualified,
        Self::MissingPackageLockfileEvidence,
        Self::MissingCorpus,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stale => "stale",
            Self::MirrorBlocked => "mirror_blocked",
            Self::ScannerUnderqualified => "scanner_underqualified",
            Self::MissingPackageLockfileEvidence => "missing_package_lockfile_evidence",
            Self::MissingCorpus => "missing_corpus",
        }
    }

    /// Highest maturity a row carrying this blocking reason may publish.
    ///
    /// A stale packet or blocked mirror narrows to provisional; a missing corpus,
    /// missing package/lockfile evidence, or an underqualified scanner narrows
    /// all the way to underqualified.
    pub const fn maturity_ceiling(self) -> MaturityClass {
        match self {
            Self::Stale | Self::MirrorBlocked => MaturityClass::Provisional,
            Self::ScannerUnderqualified
            | Self::MissingPackageLockfileEvidence
            | Self::MissingCorpus => MaturityClass::Underqualified,
        }
    }

    /// Whether this reason reflects missing evidence rather than staleness.
    pub const fn is_missing_evidence(self) -> bool {
        matches!(
            self,
            Self::MissingPackageLockfileEvidence | Self::MissingCorpus
        )
    }
}

/// The action the publication gate takes on a row relative to a full
/// certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingAction {
    /// No narrowing; the row publishes a full certification.
    None,
    /// Narrow the published claim to provisional/beta depth.
    NarrowToProvisional,
    /// Narrow the published claim to underqualified (no positive depth claim).
    NarrowToUnderqualified,
    /// Withhold the lane from publication entirely.
    WithholdFromPublication,
}

impl NarrowingAction {
    /// Every narrowing action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::None,
        Self::NarrowToProvisional,
        Self::NarrowToUnderqualified,
        Self::WithholdFromPublication,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::NarrowToProvisional => "narrow_to_provisional",
            Self::NarrowToUnderqualified => "narrow_to_underqualified",
            Self::WithholdFromPublication => "withhold_from_publication",
        }
    }

    /// The narrowing action implied by a published maturity.
    pub const fn for_published(maturity: MaturityClass) -> Self {
        match maturity {
            MaturityClass::Certified => Self::None,
            MaturityClass::Provisional => Self::NarrowToProvisional,
            MaturityClass::Underqualified => Self::NarrowToUnderqualified,
            MaturityClass::Unsupported => Self::WithholdFromPublication,
        }
    }
}

/// Support class carried alongside a lane's certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Fully supported on the stable line.
    Supported,
    /// Provider-managed; support follows the managed provider.
    Managed,
    /// Community-maintained; no first-party support guarantee.
    Community,
    /// Out of support / deprecated.
    Unsupported,
}

impl SupportClass {
    /// Every support class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Supported,
        Self::Managed,
        Self::Community,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Managed => "managed",
            Self::Community => "community",
            Self::Unsupported => "unsupported",
        }
    }
}

/// One certification row for an (ecosystem, lane) cell of the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationRow {
    /// Stable row id.
    pub row_id: String,
    /// Ecosystem this row certifies.
    pub ecosystem: ClaimedEcosystem,
    /// Qualification lane this row certifies.
    pub lane: QualificationLane,
    /// Maturity the underlying lane assessed, before the publication gate.
    pub declared_maturity: MaturityClass,
    /// Maturity actually published after the gate narrows the row.
    ///
    /// Must equal [`QualificationRow::effective_maturity`]; validation rejects a
    /// row that publishes beyond what its evidence supports.
    pub published_maturity: MaturityClass,
    /// Certification freshness relative to the lane's freshness SLO.
    pub certification_freshness: CertificationFreshness,
    /// Support class carried alongside the certification.
    pub support_class: SupportClass,
    /// Action the gate takes on this row; must equal the recomputed narrowing.
    pub narrowing_action: NarrowingAction,
    /// Reasons this lane cannot promote at full strength.
    #[serde(default)]
    pub blocking_reasons: Vec<BlockingReason>,
    /// Ref to the lane's own qualification proof packet.
    pub qualification_packet_ref: String,
    /// Ref to the lane's own proof corpus.
    pub corpus_ref: String,
    /// Source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl QualificationRow {
    /// Whether the row carries its own qualification packet and corpus and is
    /// not flagged as missing review or corpus evidence.
    pub fn has_required_evidence(&self) -> bool {
        !self.qualification_packet_ref.trim().is_empty()
            && !self.corpus_ref.trim().is_empty()
            && !self
                .blocking_reasons
                .iter()
                .any(|r| r.is_missing_evidence())
    }

    /// The maturity the publication gate permits this row to publish.
    ///
    /// Starts from [`QualificationRow::declared_maturity`] and lowers it to the
    /// weakest ceiling implied by the certification freshness, every blocking
    /// reason, and any missing required evidence — so a stale, mirror-blocked,
    /// scanner-underqualified, or evidence-missing row can never publish a full
    /// certification.
    pub fn effective_maturity(&self) -> MaturityClass {
        let mut ceiling = self
            .declared_maturity
            .min(self.certification_freshness.maturity_ceiling());
        for reason in &self.blocking_reasons {
            ceiling = ceiling.min(reason.maturity_ceiling());
        }
        if !self.has_required_evidence() {
            ceiling = ceiling.min(MaturityClass::Underqualified);
        }
        ceiling
    }

    /// The narrowing action the gate must record for this row.
    pub fn required_narrowing(&self) -> NarrowingAction {
        NarrowingAction::for_published(self.effective_maturity())
    }

    /// Whether the row may publish a full certification.
    pub fn is_promotable(&self) -> bool {
        self.effective_maturity() == MaturityClass::Certified
    }

    /// Whether the stored published maturity and narrowing action agree with the
    /// recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_maturity == self.effective_maturity()
            && self.narrowing_action == self.required_narrowing()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcosystemQualificationCertificationSummary {
    /// Total rows.
    pub total_rows: usize,
    /// Number of claimed ecosystems.
    pub claimed_ecosystem_count: usize,
    /// Number of qualification lanes.
    pub lane_count: usize,
    /// Rows published as certified.
    pub certified_rows: usize,
    /// Rows published as provisional.
    pub provisional_rows: usize,
    /// Rows published as underqualified.
    pub underqualified_rows: usize,
    /// Rows published as unsupported.
    pub unsupported_rows: usize,
    /// Rows that may publish a full certification.
    pub promotable_rows: usize,
    /// Rows the gate narrowed in any way.
    pub narrowed_rows: usize,
    /// Rows the gate withheld from publication.
    pub withheld_rows: usize,
    /// Rows with current certification freshness.
    pub current_freshness_rows: usize,
    /// Rows carrying at least one blocking reason.
    pub rows_with_blocking_reasons: usize,
}

/// A redaction-safe export row projected from a certification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EcosystemQualificationCertificationExportRow {
    /// Row id.
    pub row_id: String,
    /// Ecosystem token.
    pub ecosystem: String,
    /// Lane token.
    pub lane: String,
    /// Declared maturity token.
    pub declared_maturity: String,
    /// Published maturity token.
    pub published_maturity: String,
    /// Certification freshness token.
    pub certification_freshness: String,
    /// Support class token.
    pub support_class: String,
    /// Narrowing action token.
    pub narrowing_action: String,
    /// Blocking reason tokens.
    pub blocking_reasons: Vec<String>,
    /// Qualification packet ref.
    pub qualification_packet_ref: String,
    /// Proof corpus ref.
    pub corpus_ref: String,
    /// Whether the row publishes a full certification.
    pub publication_ready: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EcosystemQualificationCertificationExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<EcosystemQualificationCertificationExportRow>,
    /// Whether every row's published maturity and narrowing agree with the gate.
    pub all_rows_gate_consistent: bool,
    /// Rows that may publish a full certification.
    pub promotable_count: usize,
    /// Rows the gate narrowed in any way.
    pub narrowed_count: usize,
    /// Rows the gate withheld from publication.
    pub withheld_count: usize,
}

/// The typed ecosystem-qualification certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EcosystemQualificationCertification {
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
    /// Marketed ecosystems the matrix claims; one row per ecosystem and lane.
    pub claimed_ecosystems: Vec<ClaimedEcosystem>,
    /// Closed qualification-lane vocabulary.
    pub lanes: Vec<QualificationLane>,
    /// Closed maturity-class vocabulary.
    pub maturity_classes: Vec<MaturityClass>,
    /// Closed certification-freshness vocabulary.
    pub certification_freshness_classes: Vec<CertificationFreshness>,
    /// Closed blocking-reason vocabulary.
    pub blocking_reasons: Vec<BlockingReason>,
    /// Closed narrowing-action vocabulary.
    pub narrowing_actions: Vec<NarrowingAction>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Certification rows, one per (ecosystem, lane) cell.
    #[serde(default)]
    pub rows: Vec<QualificationRow>,
    /// Summary counts.
    pub summary: EcosystemQualificationCertificationSummary,
}

impl EcosystemQualificationCertification {
    /// Returns the row for an (ecosystem, lane) cell.
    pub fn row(
        &self,
        ecosystem: ClaimedEcosystem,
        lane: QualificationLane,
    ) -> Option<&QualificationRow> {
        self.rows
            .iter()
            .find(|r| r.ecosystem == ecosystem && r.lane == lane)
    }

    /// Rows that may publish a full certification.
    pub fn promotable_rows(&self) -> impl Iterator<Item = &QualificationRow> {
        self.rows.iter().filter(|r| r.is_promotable())
    }

    /// Rows the gate narrowed in any way.
    pub fn narrowed_rows(&self) -> impl Iterator<Item = &QualificationRow> {
        self.rows
            .iter()
            .filter(|r| r.required_narrowing() != NarrowingAction::None)
    }

    /// Rows the gate withheld from publication.
    pub fn withheld_rows(&self) -> impl Iterator<Item = &QualificationRow> {
        self.rows
            .iter()
            .filter(|r| r.required_narrowing() == NarrowingAction::WithholdFromPublication)
    }

    /// Whether every row's stored published maturity and narrowing action agree
    /// with the recomputed gate decision.
    pub fn all_rows_gate_consistent(&self) -> bool {
        self.rows.iter().all(|r| r.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> EcosystemQualificationCertificationSummary {
        let count_published = |maturity: MaturityClass| {
            self.rows
                .iter()
                .filter(|r| r.published_maturity == maturity)
                .count()
        };
        EcosystemQualificationCertificationSummary {
            total_rows: self.rows.len(),
            claimed_ecosystem_count: self.claimed_ecosystems.len(),
            lane_count: self.lanes.len(),
            certified_rows: count_published(MaturityClass::Certified),
            provisional_rows: count_published(MaturityClass::Provisional),
            underqualified_rows: count_published(MaturityClass::Underqualified),
            unsupported_rows: count_published(MaturityClass::Unsupported),
            promotable_rows: self.promotable_rows().count(),
            narrowed_rows: self.narrowed_rows().count(),
            withheld_rows: self.withheld_rows().count(),
            current_freshness_rows: self
                .rows
                .iter()
                .filter(|r| r.certification_freshness.is_current())
                .count(),
            rows_with_blocking_reasons: self
                .rows
                .iter()
                .filter(|r| !r.blocking_reasons.is_empty())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — Help/About,
    /// docs/migration, support exports, and release/public-truth packets —
    /// render instead of restating status text by hand.
    pub fn export_projection(&self) -> EcosystemQualificationCertificationExportProjection {
        let rows = self
            .rows
            .iter()
            .map(|row| EcosystemQualificationCertificationExportRow {
                row_id: row.row_id.clone(),
                ecosystem: row.ecosystem.as_str().to_owned(),
                lane: row.lane.as_str().to_owned(),
                declared_maturity: row.declared_maturity.as_str().to_owned(),
                published_maturity: row.published_maturity.as_str().to_owned(),
                certification_freshness: row.certification_freshness.as_str().to_owned(),
                support_class: row.support_class.as_str().to_owned(),
                narrowing_action: row.narrowing_action.as_str().to_owned(),
                blocking_reasons: row
                    .blocking_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                qualification_packet_ref: row.qualification_packet_ref.clone(),
                corpus_ref: row.corpus_ref.clone(),
                publication_ready: row.is_promotable(),
                summary: format!(
                    "{} / {}: declared {}, published {} ({}), freshness {}",
                    row.ecosystem.as_str(),
                    row.lane.as_str(),
                    row.declared_maturity.as_str(),
                    row.published_maturity.as_str(),
                    row.narrowing_action.as_str(),
                    row.certification_freshness.as_str()
                ),
            })
            .collect();
        EcosystemQualificationCertificationExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_rows_gate_consistent: self.all_rows_gate_consistent(),
            promotable_count: self.promotable_rows().count(),
            narrowed_count: self.narrowed_rows().count(),
            withheld_count: self.withheld_rows().count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<EcosystemQualificationCertificationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<ClaimedEcosystem> = self.claimed_ecosystems.iter().copied().collect();

        let mut seen_rows = BTreeSet::new();
        let mut seen_cells = BTreeSet::new();
        for row in &self.rows {
            if !seen_rows.insert(row.row_id.clone()) {
                violations.push(
                    EcosystemQualificationCertificationViolation::DuplicateRowId {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if !seen_cells.insert((row.ecosystem, row.lane)) {
                violations.push(
                    EcosystemQualificationCertificationViolation::DuplicateMatrixCell {
                        ecosystem: row.ecosystem.as_str(),
                        lane: row.lane.as_str(),
                    },
                );
            }
            if !claimed.contains(&row.ecosystem) {
                violations.push(
                    EcosystemQualificationCertificationViolation::UnclaimedEcosystemRow {
                        row_id: row.row_id.clone(),
                        ecosystem: row.ecosystem.as_str(),
                    },
                );
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed (ecosystem, lane) cell must carry its own row, so a lane
        // never inherits trust from an adjacent cell.
        for &ecosystem in &self.claimed_ecosystems {
            for &lane in &self.lanes {
                if !seen_cells.contains(&(ecosystem, lane)) {
                    violations.push(
                        EcosystemQualificationCertificationViolation::MissingMatrixCell {
                            ecosystem: ecosystem.as_str(),
                            lane: lane.as_str(),
                        },
                    );
                }
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(EcosystemQualificationCertificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(
        &self,
        violations: &mut Vec<EcosystemQualificationCertificationViolation>,
    ) {
        if self.schema_version != ECOSYSTEM_QUALIFICATION_CERTIFICATION_SCHEMA_VERSION {
            violations.push(
                EcosystemQualificationCertificationViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != ECOSYSTEM_QUALIFICATION_CERTIFICATION_RECORD_KIND {
            violations.push(
                EcosystemQualificationCertificationViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(EcosystemQualificationCertificationViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "claimed_ecosystems",
                self.claimed_ecosystems == ClaimedEcosystem::ALL.to_vec(),
            ),
            ("lanes", self.lanes == QualificationLane::ALL.to_vec()),
            (
                "maturity_classes",
                self.maturity_classes == MaturityClass::ALL.to_vec(),
            ),
            (
                "certification_freshness_classes",
                self.certification_freshness_classes == CertificationFreshness::ALL.to_vec(),
            ),
            (
                "blocking_reasons",
                self.blocking_reasons == BlockingReason::ALL.to_vec(),
            ),
            (
                "narrowing_actions",
                self.narrowing_actions == NarrowingAction::ALL.to_vec(),
            ),
            (
                "support_classes",
                self.support_classes == SupportClass::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(
                    EcosystemQualificationCertificationViolation::ClosedVocabularyMismatch {
                        field,
                    },
                );
            }
        }
    }

    fn validate_row(
        &self,
        row: &QualificationRow,
        violations: &mut Vec<EcosystemQualificationCertificationViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("qualification_packet_ref", &row.qualification_packet_ref),
            ("corpus_ref", &row.corpus_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(EcosystemQualificationCertificationViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.blocking_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(
                    EcosystemQualificationCertificationViolation::DuplicateBlockingReason {
                        row_id: row.row_id.clone(),
                        reason: reason.as_str(),
                    },
                );
            }
        }

        // The published maturity must equal the gate's recomputed decision, so a
        // row can never publish beyond what its freshness, blocking reasons, and
        // evidence support.
        let effective = row.effective_maturity();
        if row.published_maturity != effective {
            violations.push(
                EcosystemQualificationCertificationViolation::OverstatedPublishedMaturity {
                    row_id: row.row_id.clone(),
                    published: row.published_maturity.as_str(),
                    computed: effective.as_str(),
                },
            );
        }

        // The recorded narrowing action must match the published maturity, so
        // release tooling proves underqualified rows narrow automatically.
        let required = row.required_narrowing();
        if row.narrowing_action != required {
            violations.push(
                EcosystemQualificationCertificationViolation::NarrowingActionMismatch {
                    row_id: row.row_id.clone(),
                    declared: row.narrowing_action.as_str(),
                    required: required.as_str(),
                },
            );
        }

        // A promotable row must be genuinely clean: current freshness and no
        // blocking reason. This is the non-inheritance guardrail.
        if row.is_promotable()
            && (!row.certification_freshness.is_current() || !row.blocking_reasons.is_empty())
        {
            violations.push(
                EcosystemQualificationCertificationViolation::PromotedRowNotClean {
                    row_id: row.row_id.clone(),
                },
            );
        }
    }
}

/// A validation violation for the ecosystem-qualification certification packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EcosystemQualificationCertificationViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// An (ecosystem, lane) cell carries more than one row.
    DuplicateMatrixCell {
        /// Ecosystem token.
        ecosystem: &'static str,
        /// Lane token.
        lane: &'static str,
    },
    /// A claimed (ecosystem, lane) cell has no row.
    MissingMatrixCell {
        /// Ecosystem token.
        ecosystem: &'static str,
        /// Lane token.
        lane: &'static str,
    },
    /// A row covers an ecosystem the matrix does not claim.
    UnclaimedEcosystemRow {
        /// Row id.
        row_id: String,
        /// Ecosystem token.
        ecosystem: &'static str,
    },
    /// A row lists a blocking reason more than once.
    DuplicateBlockingReason {
        /// Row id.
        row_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A row publishes a maturity beyond what its evidence supports.
    OverstatedPublishedMaturity {
        /// Row id.
        row_id: String,
        /// Published-maturity token.
        published: &'static str,
        /// Computed effective-maturity token.
        computed: &'static str,
    },
    /// A row's narrowing action disagrees with its published maturity.
    NarrowingActionMismatch {
        /// Row id.
        row_id: String,
        /// Declared narrowing token.
        declared: &'static str,
        /// Required narrowing token.
        required: &'static str,
    },
    /// A promotable row still carries a blocking reason or non-current freshness.
    PromotedRowNotClean {
        /// Row id.
        row_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for EcosystemQualificationCertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id } => {
                write!(f, "duplicate row id {row_id}")
            }
            Self::DuplicateMatrixCell { ecosystem, lane } => {
                write!(f, "duplicate matrix cell for {ecosystem}/{lane}")
            }
            Self::MissingMatrixCell { ecosystem, lane } => {
                write!(f, "missing matrix cell for {ecosystem}/{lane}")
            }
            Self::UnclaimedEcosystemRow { row_id, ecosystem } => {
                write!(f, "row {row_id} covers unclaimed ecosystem {ecosystem}")
            }
            Self::DuplicateBlockingReason { row_id, reason } => {
                write!(f, "row {row_id} repeats blocking reason {reason}")
            }
            Self::OverstatedPublishedMaturity {
                row_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "row {row_id} publishes maturity {published} but the gate computes {computed}"
                )
            }
            Self::NarrowingActionMismatch {
                row_id,
                declared,
                required,
            } => {
                write!(
                    f,
                    "row {row_id} records narrowing {declared} but the gate requires {required}"
                )
            }
            Self::PromotedRowNotClean { row_id } => {
                write!(
                    f,
                    "row {row_id} is promotable but carries a blocking reason or non-current freshness"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for EcosystemQualificationCertificationViolation {}

/// Loads the embedded ecosystem-qualification certification packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`EcosystemQualificationCertification`].
pub fn current_ecosystem_qualification_certification(
) -> Result<EcosystemQualificationCertification, serde_json::Error> {
    serde_json::from_str(ECOSYSTEM_QUALIFICATION_CERTIFICATION_JSON)
}

#[cfg(test)]
mod tests;
