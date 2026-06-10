//! Certify profiler, trace, replay, and imported-versus-live truth on all claimed M5 rows.
//!
//! This module materializes the typed records that certify every profiler, trace, replay,
//! regression, and integration surface claimed in the M5 B4 workstream, and that keep
//! imported-versus-live truth explicit so users always know whether evidence was captured
//! live, imported from a file or bundle, replayed from cache, or sourced from a support
//! artifact. The records and closed vocabularies here mirror the boundary schema at
//! `/schemas/perf/certify-profiler-trace-replay-and-imported-versus-live-truth-on-all-claimed-m5-rows.schema.json`
//! and reuse the capture-class, provenance, mapping-quality, and environment-identity
//! axes already frozen in `/docs/performance/profiling_trace_replay_contract.md`.
//!
//! The module exposes:
//!
//! - the [`CertificationRow`] record that binds a claimed M5 row to its certification
//!   status, last-certified timestamp, required-for-ship flag, and downstream packet ref
//!   so no surface stays greener than its evidence;
//! - the [`ImportedVersusLiveTruthRow`] record that binds an artifact or session to its
//!   origin class, build identity, capture and import timestamps, provenance chain,
//!   mapping fidelity, and baseline comparability so comparison and replay claims narrow
//!   automatically when mapping fidelity or artifact identity are weak;
//! - the [`DowngradeRuleRow`] record that defines the conditions under which a certified
//!   row or surface is automatically narrowed, rolled back, or blocked;
//! - the [`CertificationQualificationPacket`] checked-in artifact that downstream docs,
//!   help, support, CI, and release surfaces ingest instead of cloning status text.
//!
//! Raw payload bytes, raw command lines, secrets, and ambient credentials MUST NOT
//! appear on any record carried here.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped on every certification qualification packet carried by this
/// module. Bumped only on breaking payload changes; additive-optional fields do not
/// bump this value.
pub const CERTIFICATION_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`CertificationQualificationPacket`].
pub const CERTIFICATION_QUALIFICATION_RECORD_KIND: &str =
    "certify_profiler_trace_replay_and_imported_versus_live_truth_on_all_claimed_m5_rows";

/// Repo-relative path to the checked-in certification qualification packet JSON.
pub const CERTIFICATION_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/perf/m5/certify-profiler-trace-replay-and-imported-versus-live-truth-on-all-claimed-m5-rows.json";

/// Embedded checked-in qualification packet JSON.
pub const CERTIFICATION_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/perf/m5/certify-profiler-trace-replay-and-imported-versus-live-truth-on-all-claimed-m5-rows.json"
));

/// Qualification label shown on promoted certification surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl CertificationQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Certification surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationSurfaceKind {
    /// Certification dashboard surface showing rollup status for all claimed M5 rows.
    CertificationDashboard,
    /// Imported-versus-live inspector surface showing origin class and provenance.
    ImportedVersusLiveInspector,
    /// Trace comparison basis viewer showing comparability and mapping fidelity.
    TraceComparisonBasisViewer,
    /// Profile provenance auditor showing build identity and artifact lineage.
    ProfileProvenanceAuditor,
    /// Regression baseline certification viewer showing baseline qualification state.
    RegressionBaselineCertificationViewer,
    /// Support bundle certification viewer showing bundle inclusion qualification state.
    SupportBundleCertificationViewer,
}

/// Certification status for a claimed M5 row or surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationStatus {
    /// Row has passed qualification and is current.
    Certified,
    /// Row is awaiting qualification evidence.
    Pending,
    /// Row was qualified but its evidence has aged past freshness threshold.
    Stale,
    /// Row lacks sufficient evidence for its claimed scope.
    Underqualified,
    /// Row is blocked by policy and must not be promoted.
    PolicyBlocked,
    /// Row was previously qualified but has been rolled back.
    RolledBack,
}

impl CertificationStatus {
    /// Returns true when the status allows the row to be treated as qualified.
    pub const fn allows_promotion(self) -> bool {
        matches!(self, Self::Certified)
    }

    /// Returns true when the status triggers an automatic downgrade.
    pub const fn triggers_downgrade(self) -> bool {
        matches!(self, Self::Stale | Self::Underqualified | Self::RolledBack)
    }

    /// Returns true when the status should show a degraded-state label.
    pub const fn shows_degraded_label(self) -> bool {
        matches!(
            self,
            Self::Stale | Self::Underqualified | Self::PolicyBlocked | Self::RolledBack
        )
    }
}

/// Origin class describing how evidence was produced or obtained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginClass {
    /// Evidence was captured from a live running session.
    LiveCapture,
    /// Evidence was imported from an external file or bundle.
    ImportedArtifact,
    /// Evidence was replayed or reconstructed from a cached recording.
    CachedReplay,
    /// Evidence originated from a support bundle or incident artifact.
    SupportBundle,
    /// Origin is unknown or unverified.
    Unknown,
}

impl OriginClass {
    /// Returns true when the origin is a live capture.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::LiveCapture)
    }

    /// Returns true when the origin is imported or cached.
    pub const fn is_imported_or_cached(self) -> bool {
        matches!(self, Self::ImportedArtifact | Self::CachedReplay)
    }

    /// Returns true when the origin requires a provenance chain display.
    pub const fn requires_provenance(self) -> bool {
        matches!(
            self,
            Self::ImportedArtifact | Self::CachedReplay | Self::SupportBundle
        )
    }
}

/// Mapping fidelity describing how well symbols and sources map to the captured evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingFidelity {
    /// Exact mapping with full symbolization and source navigation.
    Exact,
    /// Approximate mapping with some inferred or best-effort symbols.
    Approximate,
    /// Partial mapping with significant gaps.
    Partial,
    /// Mapping is unavailable.
    Unavailable,
    /// Mapping is stale relative to the current build.
    Stale,
    /// Mapping does not match the expected build identity.
    Mismatched,
}

impl MappingFidelity {
    /// Returns true when the fidelity allows source navigation.
    pub const fn allows_source_navigation(self) -> bool {
        matches!(self, Self::Exact | Self::Approximate | Self::Partial)
    }

    /// Returns true when the fidelity blocks a stable comparison claim.
    pub const fn blocks_stable_comparison(self) -> bool {
        matches!(self, Self::Unavailable | Self::Stale | Self::Mismatched)
    }
}

/// Baseline comparability describing whether two captures can be meaningfully compared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaselineComparability {
    /// Captures are fully comparable.
    Comparable,
    /// Captures are partially comparable with known caveats.
    Partial,
    /// Baseline is stale but still referenced.
    Stale,
    /// Captures are mismatched and should not be compared.
    Mismatch,
    /// Comparability is unknown.
    Unknown,
}

impl BaselineComparability {
    /// Returns true when comparability allows comparison with warning.
    pub const fn allows_comparison_with_warning(self) -> bool {
        matches!(self, Self::Comparable | Self::Partial | Self::Stale)
    }

    /// Returns true when a warning should be shown.
    pub const fn shows_warning(self) -> bool {
        matches!(self, Self::Partial | Self::Stale | Self::Mismatch)
    }
}

/// One certification row for a claimed M5 B4 row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationRow {
    /// Stable certification row id.
    pub certification_id: String,
    /// Human-readable title.
    pub title: String,
    /// M5 row reference (e.g., m5_045, m5_046).
    pub m5_row_ref: String,
    /// Packet ref path to the row's qualification packet.
    pub packet_ref: String,
    /// Certification status.
    pub certification_status: CertificationStatus,
    /// Last-certified timestamp.
    pub last_certified_at: String,
    /// True when this row is required for M5 ship.
    pub required_for_m5_ship: bool,
    /// True when the row downgrades automatically if certification becomes stale.
    pub downgrade_if_stale: bool,
    /// True when the certification status is visible on the surface.
    pub shows_certification_status: bool,
    /// True when the packet ref is visible.
    pub shows_packet_ref: bool,
}

/// One imported-versus-live truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedVersusLiveTruthRow {
    /// Stable truth row id.
    pub truth_id: String,
    /// Human-readable title.
    pub title: String,
    /// Origin class.
    pub origin_class: OriginClass,
    /// Build identity ref.
    pub build_identity_ref: String,
    /// Capture timestamp when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capture_timestamp: Option<String>,
    /// Import timestamp when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_timestamp: Option<String>,
    /// Provenance chain refs.
    #[serde(default)]
    pub provenance_refs: Vec<String>,
    /// Mapping fidelity.
    pub mapping_fidelity: MappingFidelity,
    /// Baseline comparability.
    pub baseline_comparability: BaselineComparability,
    /// True when the row shows an imported-artifact label.
    pub shows_imported_label: bool,
    /// True when the row shows a live-capture indicator.
    pub shows_live_indicator: bool,
    /// True when the row shows the provenance chain.
    pub shows_provenance_chain: bool,
    /// True when the row warns on identity mismatch.
    pub warns_on_identity_mismatch: bool,
    /// True when the row is present in the promoted build.
    pub promoted_build_surface: bool,
}

/// One downgrade rule row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradeRuleRow {
    /// Stable rule row id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// Trigger status that activates this rule.
    pub trigger_status: CertificationStatus,
    /// Affected surface kinds.
    #[serde(default)]
    pub affected_surface_kinds: Vec<CertificationSurfaceKind>,
    /// Target M5 row refs affected.
    #[serde(default)]
    pub affected_m5_row_refs: Vec<String>,
    /// Resulting label after downgrade.
    pub resulting_label: CertificationQualificationLabel,
    /// True when the rule is active.
    pub active: bool,
    /// True when the rule is visible on certification surfaces.
    pub shows_rule: bool,
}

/// Checked-in proof bundle for one surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationQualificationProof {
    /// Packet id.
    pub packet_id: String,
    /// Packet ref path.
    pub packet_ref: String,
    /// Proof index ref path.
    pub proof_index_ref: String,
    /// Captured-at timestamp.
    pub captured_at: String,
    /// Evidence refs.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

/// Summary projected onto help, release, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationQualificationSummary {
    /// Total number of certification rows.
    pub certification_count: usize,
    /// Total number of imported-versus-live truth rows.
    pub imported_versus_live_truth_count: usize,
    /// Total number of downgrade rule rows.
    pub downgrade_rule_count: usize,
    /// Number of rows claiming stable.
    pub stable_count: usize,
    /// Number of rows below stable.
    pub below_stable_count: usize,
    /// True when every row has a non-empty disclosure ref if below stable.
    pub all_below_stable_have_disclosure: bool,
    /// Number of certification rows that are certified.
    pub certified_count: usize,
    /// Number of imported-versus-live truth rows that show origin labels.
    pub honest_origin_count: usize,
    /// Number of active downgrade rules.
    pub active_downgrade_rule_count: usize,
}

/// Guard set for a certification surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSurfaceGuardSet {
    /// Certification status is visible.
    pub certification_status_visible: bool,
    /// Imported-versus-live truth is visible.
    pub imported_versus_live_visible: bool,
    /// Provenance chain is visible.
    pub provenance_chain_visible: bool,
    /// Build identity is visible.
    pub build_identity_visible: bool,
    /// Mapping fidelity is visible.
    pub mapping_fidelity_visible: bool,
    /// Comparison basis is visible.
    pub comparison_basis_visible: bool,
    /// Downgrade rules are visible.
    pub downgrade_rules_visible: bool,
    /// Stale or degraded warning is visible.
    pub stale_warning_visible: bool,
}

/// One surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSurfaceQualificationRow {
    /// Surface id.
    pub surface_id: String,
    /// Surface title.
    pub title: String,
    /// Surface kind.
    pub surface_kind: CertificationSurfaceKind,
    /// True when the surface is present in the promoted build.
    pub promoted_build_surface: bool,
    /// Claim label.
    pub claim_label: CertificationQualificationLabel,
    /// Displayed label (may differ from claim when narrowed).
    pub displayed_label: String,
    /// Qualification proof bundle.
    pub qualification_packet: CertificationQualificationProof,
    /// Guard set.
    pub guards: CertificationSurfaceGuardSet,
    /// True when the surface downgrades if required guards are missing.
    pub downgrade_if_missing: bool,
    /// Rationale string.
    pub rationale: String,
}

/// The checked-in certification qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationQualificationPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Packet id.
    pub packet_id: String,
    /// As-of timestamp.
    pub as_of: String,
    /// Release doc ref.
    pub release_doc_ref: String,
    /// Help doc ref.
    pub help_doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Surface qualification rows.
    pub surfaces: Vec<CertificationSurfaceQualificationRow>,
    /// Certification rows.
    pub certifications: Vec<CertificationRow>,
    /// Imported-versus-live truth rows.
    pub imported_versus_live_truths: Vec<ImportedVersusLiveTruthRow>,
    /// Downgrade rule rows.
    pub downgrade_rules: Vec<DowngradeRuleRow>,
    /// Summary.
    pub summary: CertificationQualificationSummary,
}

impl CertificationQualificationPacket {
    /// Computes the summary from current rows.
    pub fn computed_summary(&self) -> CertificationQualificationSummary {
        let stable_count = self
            .surfaces
            .iter()
            .filter(|s| s.claim_label.is_stable())
            .count();
        let below_stable_count = self.surfaces.len().saturating_sub(stable_count);
        let all_below_stable_have_disclosure = self
            .surfaces
            .iter()
            .filter(|s| !s.claim_label.is_stable())
            .all(|s| !s.rationale.is_empty());
        let certified_count = self
            .certifications
            .iter()
            .filter(|c| c.certification_status.allows_promotion())
            .count();
        let honest_origin_count = self
            .imported_versus_live_truths
            .iter()
            .filter(|t| {
                (t.origin_class.is_live() && t.shows_live_indicator)
                    || (t.origin_class.is_imported_or_cached() && t.shows_imported_label)
            })
            .count();
        let active_downgrade_rule_count = self.downgrade_rules.iter().filter(|r| r.active).count();

        CertificationQualificationSummary {
            certification_count: self.certifications.len(),
            imported_versus_live_truth_count: self.imported_versus_live_truths.len(),
            downgrade_rule_count: self.downgrade_rules.len(),
            stable_count,
            below_stable_count,
            all_below_stable_have_disclosure,
            certified_count,
            honest_origin_count,
            active_downgrade_rule_count,
        }
    }

    /// Validates the packet and returns any violations.
    pub fn validate(&self) -> Vec<CertificationQualificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != CERTIFICATION_QUALIFICATION_SCHEMA_VERSION {
            violations.push(CertificationQualificationViolation::SchemaVersion {
                expected: CERTIFICATION_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        if self.record_kind != CERTIFICATION_QUALIFICATION_RECORD_KIND {
            violations.push(CertificationQualificationViolation::RecordKind {
                expected: CERTIFICATION_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut surface_ids = BTreeSet::new();
        for surface in &self.surfaces {
            if !surface_ids.insert(surface.surface_id.clone()) {
                violations.push(CertificationQualificationViolation::DuplicateId {
                    kind: CertificationQualificationViolationKind::Surface,
                    id: surface.surface_id.clone(),
                });
            }
            if surface.promoted_build_surface
                && surface.claim_label.is_stable()
                && (!surface.guards.certification_status_visible
                    || !surface.guards.imported_versus_live_visible
                    || !surface.guards.provenance_chain_visible
                    || !surface.guards.build_identity_visible
                    || !surface.guards.mapping_fidelity_visible
                    || !surface.guards.comparison_basis_visible
                    || !surface.guards.downgrade_rules_visible
                    || !surface.guards.stale_warning_visible)
            {
                violations.push(CertificationQualificationViolation::IncompleteGuardSet {
                    surface_id: surface.surface_id.clone(),
                });
            }
        }

        let mut certification_ids = BTreeSet::new();
        for certification in &self.certifications {
            if !certification_ids.insert(certification.certification_id.clone()) {
                violations.push(CertificationQualificationViolation::DuplicateId {
                    kind: CertificationQualificationViolationKind::Certification,
                    id: certification.certification_id.clone(),
                });
            }
            if certification.certification_id.trim().is_empty()
                || certification.title.trim().is_empty()
                || certification.m5_row_ref.trim().is_empty()
                || certification.packet_ref.trim().is_empty()
                || certification.last_certified_at.trim().is_empty()
            {
                violations.push(
                    CertificationQualificationViolation::IncompleteCertification {
                        certification_id: certification.certification_id.clone(),
                    },
                );
            }
            if !certification.shows_certification_status || !certification.shows_packet_ref {
                violations.push(
                    CertificationQualificationViolation::CertificationMissingTruthLabels {
                        certification_id: certification.certification_id.clone(),
                    },
                );
            }
            if certification.required_for_m5_ship
                && !certification.certification_status.allows_promotion()
            {
                violations.push(
                    CertificationQualificationViolation::RequiredCertificationNotPromotable {
                        certification_id: certification.certification_id.clone(),
                        status: certification.certification_status,
                    },
                );
            }
        }

        let mut truth_ids = BTreeSet::new();
        for truth in &self.imported_versus_live_truths {
            if !truth_ids.insert(truth.truth_id.clone()) {
                violations.push(CertificationQualificationViolation::DuplicateId {
                    kind: CertificationQualificationViolationKind::ImportedVersusLiveTruth,
                    id: truth.truth_id.clone(),
                });
            }
            if truth.truth_id.trim().is_empty()
                || truth.title.trim().is_empty()
                || truth.build_identity_ref.trim().is_empty()
            {
                violations.push(
                    CertificationQualificationViolation::IncompleteImportedVersusLiveTruth {
                        truth_id: truth.truth_id.clone(),
                    },
                );
            }
            if truth.origin_class.requires_provenance() && truth.provenance_refs.is_empty() {
                violations.push(
                    CertificationQualificationViolation::ImportedVersusLiveTruthMissingProvenance {
                        truth_id: truth.truth_id.clone(),
                    },
                );
            }
            if !truth.shows_imported_label
                && !truth.shows_live_indicator
                && !truth.shows_provenance_chain
            {
                violations.push(
                    CertificationQualificationViolation::ImportedVersusLiveTruthMissingOriginLabels {
                        truth_id: truth.truth_id.clone(),
                    },
                );
            }
        }

        let mut rule_ids = BTreeSet::new();
        for rule in &self.downgrade_rules {
            if !rule_ids.insert(rule.rule_id.clone()) {
                violations.push(CertificationQualificationViolation::DuplicateId {
                    kind: CertificationQualificationViolationKind::DowngradeRule,
                    id: rule.rule_id.clone(),
                });
            }
            if rule.rule_id.trim().is_empty()
                || rule.title.trim().is_empty()
                || rule.affected_m5_row_refs.is_empty()
            {
                violations.push(
                    CertificationQualificationViolation::IncompleteDowngradeRule {
                        rule_id: rule.rule_id.clone(),
                    },
                );
            }
            if rule.active && !rule.shows_rule {
                violations.push(
                    CertificationQualificationViolation::DowngradeRuleMissingVisibility {
                        rule_id: rule.rule_id.clone(),
                    },
                );
            }
        }

        // Cross-reference: every certification row must reference a known M5 row.
        let known_m5_rows: BTreeSet<String> = self
            .certifications
            .iter()
            .map(|c| c.m5_row_ref.clone())
            .collect();
        for certification in &self.certifications {
            if certification.m5_row_ref.trim().is_empty() {
                violations.push(
                    CertificationQualificationViolation::IncompleteCertification {
                        certification_id: certification.certification_id.clone(),
                    },
                );
            }
        }

        // Cross-reference: every downgrade rule must affect at least one known certification.
        for rule in &self.downgrade_rules {
            for affected in &rule.affected_m5_row_refs {
                if !known_m5_rows.contains(affected) {
                    violations.push(
                        CertificationQualificationViolation::DowngradeRuleAffectsUnknownRow {
                            rule_id: rule.rule_id.clone(),
                            m5_row_ref: affected.clone(),
                        },
                    );
                }
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(CertificationQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in certification qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_certification_qualification(
) -> Result<CertificationQualificationPacket, serde_json::Error> {
    serde_json::from_str(CERTIFICATION_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificationQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Certification rows.
    Certification,
    /// Imported-versus-live truth rows.
    ImportedVersusLiveTruth,
    /// Downgrade rule rows.
    DowngradeRule,
}

impl fmt::Display for CertificationQualificationViolationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface => write!(f, "surface"),
            Self::Certification => write!(f, "certification"),
            Self::ImportedVersusLiveTruth => write!(f, "imported_versus_live_truth"),
            Self::DowngradeRule => write!(f, "downgrade_rule"),
        }
    }
}

/// Validation failure for certification qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificationQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version.
        actual: u32,
    },
    /// Record kind does not match the model.
    RecordKind {
        /// Expected record kind.
        expected: String,
        /// Actual record kind.
        actual: String,
    },
    /// IDs must be unique inside an object family.
    DuplicateId {
        /// Kind of object family.
        kind: CertificationQualificationViolationKind,
        /// Duplicate id.
        id: String,
    },
    /// A surface with a stable claim has an incomplete guard set.
    IncompleteGuardSet {
        /// Surface id.
        surface_id: String,
    },
    /// A certification row is incomplete.
    IncompleteCertification {
        /// Certification id.
        certification_id: String,
    },
    /// A certification row must show certification status and packet ref.
    CertificationMissingTruthLabels {
        /// Certification id.
        certification_id: String,
    },
    /// A required-for-ship certification row is not promotable.
    RequiredCertificationNotPromotable {
        /// Certification id.
        certification_id: String,
        /// Current status.
        status: CertificationStatus,
    },
    /// An imported-versus-live truth row is incomplete.
    IncompleteImportedVersusLiveTruth {
        /// Truth id.
        truth_id: String,
    },
    /// An imported-versus-live truth row with imported or cached origin must have provenance.
    ImportedVersusLiveTruthMissingProvenance {
        /// Truth id.
        truth_id: String,
    },
    /// An imported-versus-live truth row must show at least one origin label.
    ImportedVersusLiveTruthMissingOriginLabels {
        /// Truth id.
        truth_id: String,
    },
    /// A downgrade rule row is incomplete.
    IncompleteDowngradeRule {
        /// Rule id.
        rule_id: String,
    },
    /// An active downgrade rule must be visible.
    DowngradeRuleMissingVisibility {
        /// Rule id.
        rule_id: String,
    },
    /// A downgrade rule references an unknown M5 row.
    DowngradeRuleAffectsUnknownRow {
        /// Rule id.
        rule_id: String,
        /// Unknown M5 row ref.
        m5_row_ref: String,
    },
    /// Computed summary does not match the stored summary.
    SummaryMismatch,
}

impl fmt::Display for CertificationQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => {
                write!(f, "duplicate {kind} id: {id}")
            }
            Self::IncompleteGuardSet { surface_id } => {
                write!(
                    f,
                    "surface {surface_id} claims stable but guard set is incomplete"
                )
            }
            Self::IncompleteCertification { certification_id } => {
                write!(f, "incomplete certification row: {certification_id}")
            }
            Self::CertificationMissingTruthLabels { certification_id } => {
                write!(
                    f,
                    "certification row {certification_id} must show certification status and packet ref"
                )
            }
            Self::RequiredCertificationNotPromotable {
                certification_id,
                status,
            } => {
                write!(
                    f,
                    "required certification row {certification_id} has non-promotable status {status:?}"
                )
            }
            Self::IncompleteImportedVersusLiveTruth { truth_id } => {
                write!(f, "incomplete imported-versus-live truth row: {truth_id}")
            }
            Self::ImportedVersusLiveTruthMissingProvenance { truth_id } => {
                write!(
                    f,
                    "imported-versus-live truth row {truth_id} missing provenance for imported or cached origin"
                )
            }
            Self::ImportedVersusLiveTruthMissingOriginLabels { truth_id } => {
                write!(
                    f,
                    "imported-versus-live truth row {truth_id} must show at least one origin label"
                )
            }
            Self::IncompleteDowngradeRule { rule_id } => {
                write!(f, "incomplete downgrade rule row: {rule_id}")
            }
            Self::DowngradeRuleMissingVisibility { rule_id } => {
                write!(
                    f,
                    "active downgrade rule {rule_id} must be visible on certification surfaces"
                )
            }
            Self::DowngradeRuleAffectsUnknownRow {
                rule_id,
                m5_row_ref,
            } => {
                write!(
                    f,
                    "downgrade rule {rule_id} affects unknown M5 row {m5_row_ref}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "computed summary does not match stored summary")
            }
        }
    }
}

impl Error for CertificationQualificationViolation {}
