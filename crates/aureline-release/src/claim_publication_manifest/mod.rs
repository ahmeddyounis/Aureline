//! Claim-publication manifest and automatic downgrade propagation.
//!
//! This module is the stable-line publication join over the stable claim
//! manifest, compatibility reports, reference-workspace reports, and
//! evaluation evidence pack. It answers one release-bearing question: what may
//! each public or enterprise-facing surface say today, and which report refs
//! force that wording to narrow?
//!
//! The checked-in manifest at
//! `artifacts/release/stable/claim-publication-manifest/manifest.json` is the
//! source consumed by release notes, website/docs, enterprise evaluation
//! packets, in-product badges, Help/About, service-health, CLI inspection,
//! support export, and public proof rows. The model enforces that each
//! projection renders from the same manifest id, carries report refs, does not
//! render a claim wider than the effective claim, and downgrades when stale,
//! missing, dropped, or unsigned evidence appears.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Supported schema version for the claim-publication manifest.
pub const CLAIM_PUBLICATION_MANIFEST_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the claim-publication manifest.
pub const CLAIM_PUBLICATION_MANIFEST_RECORD_KIND: &str = "claim_publication_manifest";

/// Repo-relative path to the checked-in claim-publication manifest.
pub const CLAIM_PUBLICATION_MANIFEST_PATH: &str =
    "artifacts/release/stable/claim-publication-manifest/manifest.json";

/// Embedded checked-in claim-publication manifest JSON.
pub const CLAIM_PUBLICATION_MANIFEST_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/stable/claim-publication-manifest/manifest.json"
));

/// Support class a claim is put forward as before downgrade resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// A certified workflow backed by a current reference-workspace report.
    Certified,
    /// A supported workflow backed by current report evidence.
    Supported,
    /// A limited workflow whose supported scope is deliberately narrow.
    Limited,
    /// A workflow that has no claim-bearing support.
    Unsupported,
}

impl SupportClass {
    /// Every support class, widest to narrowest.
    pub const ALL: [Self; 4] = [
        Self::Certified,
        Self::Supported,
        Self::Limited,
        Self::Unsupported,
    ];

    /// Returns the rank used for no-widening checks.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Certified => 4,
            Self::Supported => 3,
            Self::Limited => 2,
            Self::Unsupported => 0,
        }
    }
}

/// Effective claim rendered after evidence and compatibility resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectiveClaim {
    /// Certified wording may render.
    Certified,
    /// Supported wording may render.
    Supported,
    /// Only limited-scope wording may render.
    Limited,
    /// Retest-pending wording must render until evidence is refreshed.
    RetestPending,
    /// Unsupported wording must render.
    Unsupported,
}

impl EffectiveClaim {
    /// Every effective claim, widest to narrowest.
    pub const ALL: [Self; 5] = [
        Self::Certified,
        Self::Supported,
        Self::Limited,
        Self::RetestPending,
        Self::Unsupported,
    ];

    /// Returns the rank used for no-widening checks.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Certified => 4,
            Self::Supported => 3,
            Self::Limited => 2,
            Self::RetestPending => 1,
            Self::Unsupported => 0,
        }
    }
}

/// Freshness or availability state of a linked report ref.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceState {
    /// Evidence is current.
    Current,
    /// Evidence remains valid but is approaching refresh.
    DueForRefresh,
    /// Evidence is stale.
    Stale,
    /// Evidence is missing.
    Missing,
    /// The compatibility class dropped below the declared support class.
    Dropped,
}

impl EvidenceState {
    /// Every evidence state, freshest to least claim-bearing.
    pub const ALL: [Self; 5] = [
        Self::Current,
        Self::DueForRefresh,
        Self::Stale,
        Self::Missing,
        Self::Dropped,
    ];

    /// Whether this state forces claim narrowing.
    pub const fn forces_narrowing(self) -> bool {
        matches!(self, Self::Stale | Self::Missing | Self::Dropped)
    }
}

/// Closed reason a claim row narrows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimNarrowingReason {
    /// A linked evidence packet is stale.
    EvidenceStale,
    /// A linked evidence packet is missing.
    EvidenceMissing,
    /// A compatibility scorecard dropped below the declared support class.
    CompatibilityDropped,
    /// A reference-workspace report is stale.
    ReferenceWorkspaceStale,
    /// A reference-workspace report is missing.
    ReferenceWorkspaceMissing,
    /// The owning report row lacks sign-off.
    OwnerSignoffMissing,
    /// A private evaluation filter narrowed the public claim.
    PrivateFilterNarrowed,
}

impl ClaimNarrowingReason {
    /// Every narrowing reason in declaration order.
    pub const ALL: [Self; 7] = [
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::CompatibilityDropped,
        Self::ReferenceWorkspaceStale,
        Self::ReferenceWorkspaceMissing,
        Self::OwnerSignoffMissing,
        Self::PrivateFilterNarrowed,
    ];
}

/// Consuming surface that renders the manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimSurface {
    /// Release notes or what's-new copy.
    ReleaseNotes,
    /// Website or docs publication.
    WebsiteDocs,
    /// Enterprise evaluation packet.
    EnterpriseEvaluationPacket,
    /// In-product support or compatibility badge.
    ProductBadges,
    /// Help and About surfaces.
    HelpAbout,
    /// Service-health surface.
    ServiceHealth,
    /// CLI or headless inspection surface.
    CliInspection,
    /// Support export projection.
    SupportExport,
    /// Public proof packet.
    PublicProofPacket,
}

impl ClaimSurface {
    /// Every consuming surface that must render from the manifest.
    pub const ALL: [Self; 9] = [
        Self::ReleaseNotes,
        Self::WebsiteDocs,
        Self::EnterpriseEvaluationPacket,
        Self::ProductBadges,
        Self::HelpAbout,
        Self::ServiceHealth,
        Self::CliInspection,
        Self::SupportExport,
        Self::PublicProofPacket,
    ];
}

/// Publication action selected for a surface projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationAction {
    /// Render Certified wording.
    PublishCertified,
    /// Render Supported wording.
    PublishSupported,
    /// Render Limited wording.
    DowngradeToLimited,
    /// Render Retest pending wording.
    DowngradeToRetestPending,
    /// Render Unsupported wording.
    DowngradeToUnsupported,
    /// Hold publication because a surface overclaims the manifest.
    HoldOverclaimingSurface,
}

impl PublicationAction {
    /// Every publication action in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PublishCertified,
        Self::PublishSupported,
        Self::DowngradeToLimited,
        Self::DowngradeToRetestPending,
        Self::DowngradeToUnsupported,
        Self::HoldOverclaimingSurface,
    ];
}

/// Family of report a claim entry links to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportFamily {
    /// Stable claim manifest baseline.
    ClaimManifest,
    /// Compatibility report or compatibility scorecard.
    CompatibilityReport,
    /// Reference-workspace report.
    ReferenceWorkspaceReport,
    /// Evaluation evidence pack.
    EvaluationEvidencePack,
    /// Proof packet.
    ProofPacket,
}

/// Publication decision for the whole manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimPublicationDecision {
    /// All rows publish at their declared support class.
    Proceed,
    /// Publication may proceed, but one or more rows render downgraded copy.
    ProceedWithDowngrades,
    /// Publication must hold because a surface overclaims or the manifest is invalid.
    Hold,
}

/// Validity window attached to a report or claim entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimValidityWindow {
    /// UTC date the evidence was captured, or null when missing.
    #[serde(default)]
    pub captured_at: Option<String>,
    /// UTC date the evidence expires, or null when missing.
    #[serde(default)]
    pub expires_at: Option<String>,
    /// Number of days the evidence may remain claim-bearing.
    pub window_days: u32,
}

/// A report reference linked to a claim entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimReportRef {
    /// Repo-relative report ref or anchored report-row ref.
    pub report_ref: String,
    /// Family of report being linked.
    pub report_family: ReportFamily,
    /// Support class the report currently backs.
    pub support_class: SupportClass,
    /// Named owner for this report.
    pub owner_ref: String,
    /// Whether the report owner signed off.
    pub owner_signed: bool,
    /// Validity window for this report.
    pub validity_window: ClaimValidityWindow,
    /// Evidence state earned by this report ref.
    pub evidence_state: EvidenceState,
}

/// One destination projection rendered from a claim entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceProjection {
    /// Consuming surface.
    pub surface_id: ClaimSurface,
    /// Destination row or packet ref.
    pub destination_ref: String,
    /// Claim label the destination renders.
    pub rendered_claim: EffectiveClaim,
    /// Badge label the destination renders.
    pub badge_label: String,
    /// Manifest id used as the source of truth.
    pub source_manifest_ref: String,
    /// Report refs carried by the projection.
    pub linked_report_refs: Vec<String>,
    /// Publication action selected for the projection.
    pub publication_action: PublicationAction,
}

impl SurfaceProjection {
    /// Whether this projection renders wider wording than the effective claim.
    pub fn overclaims(&self, entry: &ClaimPublicationEntry) -> bool {
        self.rendered_claim.rank() > entry.effective_claim.rank()
    }
}

/// One claim entry in the publication manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimPublicationEntry {
    /// Stable entry id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// Support class requested before downgrade propagation.
    pub declared_support_class: SupportClass,
    /// Effective claim after linked report refs are resolved.
    pub effective_claim: EffectiveClaim,
    /// Caveat text rendered with public and private surfaces.
    pub scope_caveat: String,
    /// Primary owner for the claim.
    pub owner_ref: String,
    /// Validity window for the claim row.
    pub validity_window: ClaimValidityWindow,
    /// Report refs backing this claim row.
    pub linked_report_refs: Vec<ClaimReportRef>,
    /// Active narrowing reasons after report resolution.
    pub active_narrowing_reasons: Vec<ClaimNarrowingReason>,
    /// Surface projections that render this entry.
    pub surface_projections: Vec<SurfaceProjection>,
    /// Reviewable rationale for the effective claim.
    pub rationale: String,
}

impl ClaimPublicationEntry {
    /// Whether the entry renders a lower claim than its declared support class.
    pub fn is_downgraded(&self) -> bool {
        self.effective_claim.rank() < self.declared_support_class.rank()
    }

    /// Whether the entry currently renders Unsupported.
    pub fn is_unsupported(&self) -> bool {
        self.effective_claim == EffectiveClaim::Unsupported
    }

    /// Whether this Certified entry has a current reference-workspace report.
    pub fn has_current_certified_reference_report(&self) -> bool {
        self.linked_report_refs.iter().any(|report| {
            report.report_family == ReportFamily::ReferenceWorkspaceReport
                && report.support_class == SupportClass::Certified
                && report.evidence_state == EvidenceState::Current
                && report.owner_signed
        })
    }
}

/// Downgrade rule that maps a reason to the rendered claim ceiling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimDowngradeRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// Reason this rule watches.
    pub trigger_reason: ClaimNarrowingReason,
    /// Default action for this reason.
    pub default_action: PublicationAction,
    /// Maximum claim allowed when the rule fires.
    pub target_claim: EffectiveClaim,
    /// Whether a wider destination row holds publication.
    pub blocks_publication_on_overclaim: bool,
    /// Reviewable reason the rule exists.
    pub rationale: String,
}

/// Audience filter for private evaluation or pilot materials.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvaluationFilter {
    /// Stable filter id.
    pub filter_id: String,
    /// Audience this filter serves.
    pub audience: String,
    /// Entry ids included in the private packet.
    pub included_entry_refs: Vec<String>,
    /// Entry ids excluded from the private packet.
    pub excluded_entry_refs: Vec<String>,
    /// Public manifest id that caps this filter.
    pub public_ceiling_manifest_ref: String,
    /// Widest effective claim the filter may render.
    pub max_effective_claim: EffectiveClaim,
}

/// Recorded publication verdict for the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimPublicationRecord {
    /// Gate this verdict governs.
    pub publication_gate: String,
    /// Publication decision.
    pub decision: ClaimPublicationDecision,
    /// Entry ids blocking publication.
    pub blocking_entry_ids: Vec<String>,
    /// Surface refs blocking publication.
    pub blocking_surface_refs: Vec<String>,
    /// Reviewable rationale for the verdict.
    pub rationale: String,
}

/// Summary counts carried by the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimPublicationSummary {
    /// Total claim entries.
    pub total_entries: usize,
    /// Certified entries with current reference-workspace reports.
    pub certified_entries_current: usize,
    /// Entries whose effective claim is below the declared support class.
    pub entries_downgraded: usize,
    /// Entries currently rendered Unsupported.
    pub unsupported_entries: usize,
    /// Total surface projection rows.
    pub surface_projection_rows: usize,
    /// Projection rows that render wider than the effective claim.
    pub overclaiming_surface_rows: usize,
}

/// The stable claim-publication manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimPublicationManifest {
    /// Artifact schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable manifest id.
    pub manifest_id: String,
    /// Lifecycle status of the manifest.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Stable claim manifest ref this publication ingests.
    pub claim_manifest_ref: String,
    /// Compatibility report ref this publication ingests.
    pub compatibility_report_ref: String,
    /// Reference-workspace report index ref this publication ingests.
    pub reference_workspace_report_index_ref: String,
    /// Evaluation evidence pack ref this publication ingests.
    pub evaluation_evidence_pack_ref: String,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed effective-claim vocabulary.
    pub effective_claims: Vec<EffectiveClaim>,
    /// Closed evidence-state vocabulary.
    pub evidence_states: Vec<EvidenceState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<ClaimNarrowingReason>,
    /// Closed consuming-surface vocabulary.
    pub surface_ids: Vec<ClaimSurface>,
    /// Closed publication-action vocabulary.
    pub publication_actions: Vec<PublicationAction>,
    /// Claim entries.
    pub entries: Vec<ClaimPublicationEntry>,
    /// Downgrade rules.
    pub downgrade_rules: Vec<ClaimDowngradeRule>,
    /// Private evaluation filters.
    pub evaluation_filters: Vec<EvaluationFilter>,
    /// Recorded publication verdict.
    pub publication: ClaimPublicationRecord,
    /// Summary counts.
    pub summary: ClaimPublicationSummary,
}

impl ClaimPublicationManifest {
    /// Returns the claim entry registered for `entry_id`.
    pub fn entry(&self, entry_id: &str) -> Option<&ClaimPublicationEntry> {
        self.entries.iter().find(|entry| entry.entry_id == entry_id)
    }

    /// Returns every downgraded entry.
    pub fn downgraded_entries(&self) -> Vec<&ClaimPublicationEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.is_downgraded())
            .collect()
    }

    /// Returns surface projection rows that overclaim their entry.
    pub fn overclaiming_surface_rows(&self) -> Vec<(&ClaimPublicationEntry, &SurfaceProjection)> {
        self.entries
            .iter()
            .flat_map(|entry| {
                entry
                    .surface_projections
                    .iter()
                    .filter(move |projection| projection.overclaims(entry))
                    .map(move |projection| (entry, projection))
            })
            .collect()
    }

    /// Recomputes the publication decision from surface overclaiming and downgrades.
    pub fn computed_publication_decision(&self) -> ClaimPublicationDecision {
        if !self.overclaiming_surface_rows().is_empty() {
            ClaimPublicationDecision::Hold
        } else if self
            .entries
            .iter()
            .any(ClaimPublicationEntry::is_downgraded)
        {
            ClaimPublicationDecision::ProceedWithDowngrades
        } else {
            ClaimPublicationDecision::Proceed
        }
    }

    /// Recomputes entry ids that block publication.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for (entry, _) in self.overclaiming_surface_rows() {
            ids.insert(entry.entry_id.clone());
        }
        ids.into_iter().collect()
    }

    /// Recomputes surface refs that block publication.
    pub fn computed_blocking_surface_refs(&self) -> Vec<String> {
        let mut refs: Vec<String> = self
            .overclaiming_surface_rows()
            .into_iter()
            .map(|(_, projection)| projection.destination_ref.clone())
            .collect();
        refs.sort();
        refs.dedup();
        refs
    }

    /// Recomputes the manifest summary.
    pub fn computed_summary(&self) -> ClaimPublicationSummary {
        ClaimPublicationSummary {
            total_entries: self.entries.len(),
            certified_entries_current: self
                .entries
                .iter()
                .filter(|entry| {
                    entry.declared_support_class == SupportClass::Certified
                        && entry.has_current_certified_reference_report()
                })
                .count(),
            entries_downgraded: self
                .entries
                .iter()
                .filter(|entry| entry.is_downgraded())
                .count(),
            unsupported_entries: self
                .entries
                .iter()
                .filter(|entry| entry.is_unsupported())
                .count(),
            surface_projection_rows: self
                .entries
                .iter()
                .map(|entry| entry.surface_projections.len())
                .sum(),
            overclaiming_surface_rows: self.overclaiming_surface_rows().len(),
        }
    }

    /// Produces a redaction-safe projection for docs, Help/About, CLI, and support export.
    pub fn surface_projection(&self) -> ClaimPublicationSurfaceExport {
        ClaimPublicationSurfaceExport {
            manifest_id: self.manifest_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            entries: self
                .entries
                .iter()
                .map(|entry| ClaimPublicationSurfaceEntry {
                    entry_id: entry.entry_id.clone(),
                    declared_support_class: entry.declared_support_class,
                    effective_claim: entry.effective_claim,
                    active_narrowing_reasons: entry.active_narrowing_reasons.clone(),
                    surface_ids: entry
                        .surface_projections
                        .iter()
                        .map(|projection| projection.surface_id)
                        .collect(),
                    report_refs: entry
                        .linked_report_refs
                        .iter()
                        .map(|report| report.report_ref.clone())
                        .collect(),
                })
                .collect(),
        }
    }

    /// Validates the manifest, returning every violation found.
    pub fn validate(&self) -> Vec<ClaimPublicationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);
        self.validate_entries(&mut violations);
        self.validate_evaluation_filters(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(ClaimPublicationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<ClaimPublicationViolation>) {
        if self.schema_version != CLAIM_PUBLICATION_MANIFEST_SCHEMA_VERSION {
            violations.push(ClaimPublicationViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != CLAIM_PUBLICATION_MANIFEST_RECORD_KIND {
            violations.push(ClaimPublicationViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("manifest_id", &self.manifest_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("compatibility_report_ref", &self.compatibility_report_ref),
            (
                "reference_workspace_report_index_ref",
                &self.reference_workspace_report_index_ref,
            ),
            (
                "evaluation_evidence_pack_ref",
                &self.evaluation_evidence_pack_ref,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(ClaimPublicationViolation::EmptyField {
                    id: "<manifest>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.support_classes != SupportClass::ALL.to_vec() {
            violations.push(ClaimPublicationViolation::ClosedVocabularyMismatch {
                field: "support_classes",
            });
        }
        if self.effective_claims != EffectiveClaim::ALL.to_vec() {
            violations.push(ClaimPublicationViolation::ClosedVocabularyMismatch {
                field: "effective_claims",
            });
        }
        if self.evidence_states != EvidenceState::ALL.to_vec() {
            violations.push(ClaimPublicationViolation::ClosedVocabularyMismatch {
                field: "evidence_states",
            });
        }
        if self.narrowing_reasons != ClaimNarrowingReason::ALL.to_vec() {
            violations.push(ClaimPublicationViolation::ClosedVocabularyMismatch {
                field: "narrowing_reasons",
            });
        }
        if self.surface_ids != ClaimSurface::ALL.to_vec() {
            violations.push(ClaimPublicationViolation::ClosedVocabularyMismatch {
                field: "surface_ids",
            });
        }
        if self.publication_actions != PublicationAction::ALL.to_vec() {
            violations.push(ClaimPublicationViolation::ClosedVocabularyMismatch {
                field: "publication_actions",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<ClaimPublicationViolation>) {
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.downgrade_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(ClaimPublicationViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(ClaimPublicationViolation::EmptyField {
                        id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            covered.insert(rule.trigger_reason);
        }
        for reason in ClaimNarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations.push(ClaimPublicationViolation::NarrowingReasonWithoutRule { reason });
            }
        }
    }

    fn validate_entries(&self, violations: &mut Vec<ClaimPublicationViolation>) {
        if self.entries.is_empty() {
            violations.push(ClaimPublicationViolation::NoEntries);
        }
        let mut seen = BTreeSet::new();
        for entry in &self.entries {
            if !seen.insert(entry.entry_id.clone()) {
                violations.push(ClaimPublicationViolation::DuplicateEntryId {
                    entry_id: entry.entry_id.clone(),
                });
            }
            self.validate_entry(entry, violations);
        }
    }

    fn validate_entry(
        &self,
        entry: &ClaimPublicationEntry,
        violations: &mut Vec<ClaimPublicationViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &entry.entry_id),
            ("title", &entry.title),
            ("scope_caveat", &entry.scope_caveat),
            ("owner_ref", &entry.owner_ref),
            ("rationale", &entry.rationale),
        ] {
            if value.trim().is_empty() {
                violations.push(ClaimPublicationViolation::EmptyField {
                    id: entry.entry_id.clone(),
                    field_name: field,
                });
            }
        }
        if entry.effective_claim.rank() > entry.declared_support_class.rank() {
            violations.push(ClaimPublicationViolation::EffectiveClaimWiderThanDeclared {
                entry_id: entry.entry_id.clone(),
                declared: entry.declared_support_class,
                effective: entry.effective_claim,
            });
        }
        if entry.linked_report_refs.is_empty() {
            violations.push(ClaimPublicationViolation::EntryWithoutReportRefs {
                entry_id: entry.entry_id.clone(),
            });
        }
        if entry.validity_window.window_days == 0 {
            violations.push(ClaimPublicationViolation::InvalidValidityWindow {
                id: entry.entry_id.clone(),
            });
        }

        let required_reasons = self.required_reasons(entry);
        for reason in &required_reasons {
            if !entry.active_narrowing_reasons.contains(reason) {
                violations.push(ClaimPublicationViolation::MissingRequiredNarrowingReason {
                    entry_id: entry.entry_id.clone(),
                    reason: *reason,
                });
            }
        }
        if required_reasons.is_empty() && !entry.active_narrowing_reasons.is_empty() {
            violations.push(ClaimPublicationViolation::HeldEntryWithNarrowingReason {
                entry_id: entry.entry_id.clone(),
            });
        }
        if !required_reasons.is_empty() && !entry.is_downgraded() {
            violations.push(ClaimPublicationViolation::NarrowingEntryNotDowngraded {
                entry_id: entry.entry_id.clone(),
            });
        }
        if entry.declared_support_class == SupportClass::Certified
            && !entry.has_current_certified_reference_report()
        {
            violations.push(
                ClaimPublicationViolation::CertifiedEntryWithoutCurrentReport {
                    entry_id: entry.entry_id.clone(),
                },
            );
        }

        self.validate_report_refs(entry, violations);
        self.validate_surface_projections(entry, violations);
    }

    fn required_reasons(&self, entry: &ClaimPublicationEntry) -> BTreeSet<ClaimNarrowingReason> {
        let mut reasons = BTreeSet::new();
        for report in &entry.linked_report_refs {
            match (report.report_family, report.evidence_state) {
                (ReportFamily::ReferenceWorkspaceReport, EvidenceState::Stale) => {
                    reasons.insert(ClaimNarrowingReason::ReferenceWorkspaceStale);
                }
                (ReportFamily::ReferenceWorkspaceReport, EvidenceState::Missing) => {
                    reasons.insert(ClaimNarrowingReason::ReferenceWorkspaceMissing);
                }
                (ReportFamily::CompatibilityReport, EvidenceState::Dropped) => {
                    reasons.insert(ClaimNarrowingReason::CompatibilityDropped);
                }
                (_, EvidenceState::Stale) => {
                    reasons.insert(ClaimNarrowingReason::EvidenceStale);
                }
                (_, EvidenceState::Missing) => {
                    reasons.insert(ClaimNarrowingReason::EvidenceMissing);
                }
                (_, EvidenceState::Dropped) => {
                    reasons.insert(ClaimNarrowingReason::CompatibilityDropped);
                }
                (_, EvidenceState::Current | EvidenceState::DueForRefresh) => {}
            }
            if !report.owner_signed {
                reasons.insert(ClaimNarrowingReason::OwnerSignoffMissing);
            }
        }
        reasons
    }

    fn validate_report_refs(
        &self,
        entry: &ClaimPublicationEntry,
        violations: &mut Vec<ClaimPublicationViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for report in &entry.linked_report_refs {
            if !seen.insert(report.report_ref.clone()) {
                violations.push(ClaimPublicationViolation::DuplicateReportRef {
                    entry_id: entry.entry_id.clone(),
                    report_ref: report.report_ref.clone(),
                });
            }
            for (field, value) in [
                ("report_ref", &report.report_ref),
                ("owner_ref", &report.owner_ref),
            ] {
                if value.trim().is_empty() {
                    violations.push(ClaimPublicationViolation::EmptyField {
                        id: entry.entry_id.clone(),
                        field_name: field,
                    });
                }
            }
            if report.validity_window.window_days == 0 {
                violations.push(ClaimPublicationViolation::InvalidValidityWindow {
                    id: report.report_ref.clone(),
                });
            }
        }
    }

    fn validate_surface_projections(
        &self,
        entry: &ClaimPublicationEntry,
        violations: &mut Vec<ClaimPublicationViolation>,
    ) {
        let entry_report_refs: BTreeSet<&str> = entry
            .linked_report_refs
            .iter()
            .map(|report| report.report_ref.as_str())
            .collect();
        let mut surfaces = BTreeMap::new();
        for projection in &entry.surface_projections {
            surfaces
                .entry(projection.surface_id)
                .and_modify(|count| *count += 1)
                .or_insert(1usize);
            if projection.source_manifest_ref != self.manifest_id {
                violations.push(ClaimPublicationViolation::ProjectionWrongManifest {
                    entry_id: entry.entry_id.clone(),
                    destination_ref: projection.destination_ref.clone(),
                });
            }
            if projection.destination_ref.trim().is_empty()
                || projection.badge_label.trim().is_empty()
            {
                violations.push(ClaimPublicationViolation::EmptyField {
                    id: entry.entry_id.clone(),
                    field_name: "surface_projection",
                });
            }
            if projection.linked_report_refs.is_empty() {
                violations.push(ClaimPublicationViolation::ProjectionWithoutReportRefs {
                    entry_id: entry.entry_id.clone(),
                    surface_id: projection.surface_id,
                });
            }
            for report_ref in &projection.linked_report_refs {
                if !entry_report_refs.contains(report_ref.as_str()) {
                    violations.push(ClaimPublicationViolation::ProjectionUnknownReportRef {
                        entry_id: entry.entry_id.clone(),
                        destination_ref: projection.destination_ref.clone(),
                        report_ref: report_ref.clone(),
                    });
                }
            }
            if projection.overclaims(entry) {
                violations.push(ClaimPublicationViolation::SurfaceOverclaims {
                    entry_id: entry.entry_id.clone(),
                    surface_id: projection.surface_id,
                    rendered: projection.rendered_claim,
                    effective: entry.effective_claim,
                });
            }
        }
        for surface in ClaimSurface::ALL {
            match surfaces.get(&surface) {
                Some(1) => {}
                Some(_) => violations.push(ClaimPublicationViolation::DuplicateSurfaceProjection {
                    entry_id: entry.entry_id.clone(),
                    surface_id: surface,
                }),
                None => violations.push(ClaimPublicationViolation::MissingSurfaceProjection {
                    entry_id: entry.entry_id.clone(),
                    surface_id: surface,
                }),
            }
        }
    }

    fn validate_evaluation_filters(&self, violations: &mut Vec<ClaimPublicationViolation>) {
        let entry_ranks: BTreeMap<&str, u8> = self
            .entries
            .iter()
            .map(|entry| (entry.entry_id.as_str(), entry.effective_claim.rank()))
            .collect();
        for filter in &self.evaluation_filters {
            if filter.public_ceiling_manifest_ref != self.manifest_id {
                violations.push(ClaimPublicationViolation::EvaluationFilterWrongManifest {
                    filter_id: filter.filter_id.clone(),
                });
            }
            if filter.included_entry_refs.is_empty() {
                violations.push(ClaimPublicationViolation::EvaluationFilterEmpty {
                    filter_id: filter.filter_id.clone(),
                });
            }
            let mut widest_included_public_claim = None;
            for entry_ref in &filter.included_entry_refs {
                match entry_ranks.get(entry_ref.as_str()) {
                    Some(rank) => {
                        widest_included_public_claim = Some(
                            widest_included_public_claim
                                .map(|current: u8| current.max(*rank))
                                .unwrap_or(*rank),
                        );
                    }
                    None => violations.push(ClaimPublicationViolation::UnknownEntryRef {
                        id: filter.filter_id.clone(),
                        entry_id: entry_ref.clone(),
                    }),
                }
            }
            if widest_included_public_claim
                .map(|rank| filter.max_effective_claim.rank() > rank)
                .unwrap_or(false)
            {
                violations.push(ClaimPublicationViolation::EvaluationFilterOverclaims {
                    filter_id: filter.filter_id.clone(),
                    entry_id: "<filter>".to_owned(),
                });
            }
            for entry_ref in &filter.excluded_entry_refs {
                if !entry_ranks.contains_key(entry_ref.as_str()) {
                    violations.push(ClaimPublicationViolation::UnknownEntryRef {
                        id: filter.filter_id.clone(),
                        entry_id: entry_ref.clone(),
                    });
                }
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<ClaimPublicationViolation>) {
        if self.publication.publication_gate.trim().is_empty()
            || self.publication.rationale.trim().is_empty()
        {
            violations.push(ClaimPublicationViolation::EmptyField {
                id: "<publication>".to_owned(),
                field_name: "publication",
            });
        }
        if self.publication.decision != self.computed_publication_decision() {
            violations.push(ClaimPublicationViolation::PublicationDecisionMismatch {
                declared: self.publication.decision,
                computed: self.computed_publication_decision(),
            });
        }
        if self.publication.blocking_entry_ids != self.computed_blocking_entry_ids() {
            violations.push(ClaimPublicationViolation::PublicationBlockingSetMismatch {
                field: "blocking_entry_ids",
            });
        }
        if self.publication.blocking_surface_refs != self.computed_blocking_surface_refs() {
            violations.push(ClaimPublicationViolation::PublicationBlockingSetMismatch {
                field: "blocking_surface_refs",
            });
        }
    }
}

/// Redaction-safe surface export of the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimPublicationSurfaceExport {
    /// Manifest id used to generate the export.
    pub manifest_id: String,
    /// Manifest as-of date.
    pub as_of: String,
    /// Publication decision.
    pub publication_decision: ClaimPublicationDecision,
    /// Exported entries.
    pub entries: Vec<ClaimPublicationSurfaceEntry>,
}

/// One redaction-safe exported claim entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimPublicationSurfaceEntry {
    /// Stable entry id.
    pub entry_id: String,
    /// Declared support class.
    pub declared_support_class: SupportClass,
    /// Effective claim after downgrade propagation.
    pub effective_claim: EffectiveClaim,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<ClaimNarrowingReason>,
    /// Surfaces that render this entry.
    pub surface_ids: Vec<ClaimSurface>,
    /// Linked report refs.
    pub report_refs: Vec<String>,
}

/// Validation violation for the claim-publication manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaimPublicationViolation {
    /// The manifest carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the manifest.
        actual: u32,
    },
    /// The manifest carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the manifest.
        actual: String,
    },
    /// A closed vocabulary is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row, report, or section id.
        id: String,
        /// Offending field name.
        field_name: &'static str,
    },
    /// No claim entries exist.
    NoEntries,
    /// A claim entry id appears more than once.
    DuplicateEntryId {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// A downgrade rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A narrowing reason has no downgrade rule.
    NarrowingReasonWithoutRule {
        /// Uncovered reason.
        reason: ClaimNarrowingReason,
    },
    /// An effective claim is wider than the declared support class.
    EffectiveClaimWiderThanDeclared {
        /// Entry id.
        entry_id: String,
        /// Declared support class.
        declared: SupportClass,
        /// Effective claim.
        effective: EffectiveClaim,
    },
    /// An entry has no linked report refs.
    EntryWithoutReportRefs {
        /// Entry id.
        entry_id: String,
    },
    /// A validity window is invalid.
    InvalidValidityWindow {
        /// Entry or report id.
        id: String,
    },
    /// A report ref appears more than once on an entry.
    DuplicateReportRef {
        /// Entry id.
        entry_id: String,
        /// Duplicate report ref.
        report_ref: String,
    },
    /// A required narrowing reason is missing.
    MissingRequiredNarrowingReason {
        /// Entry id.
        entry_id: String,
        /// Missing reason.
        reason: ClaimNarrowingReason,
    },
    /// A held entry carries narrowing reasons.
    HeldEntryWithNarrowingReason {
        /// Entry id.
        entry_id: String,
    },
    /// An entry with narrowing evidence did not downgrade.
    NarrowingEntryNotDowngraded {
        /// Entry id.
        entry_id: String,
    },
    /// A Certified entry lacks a current signed reference-workspace report.
    CertifiedEntryWithoutCurrentReport {
        /// Entry id.
        entry_id: String,
    },
    /// A projection points at a different manifest id.
    ProjectionWrongManifest {
        /// Entry id.
        entry_id: String,
        /// Destination ref.
        destination_ref: String,
    },
    /// A projection has no report refs.
    ProjectionWithoutReportRefs {
        /// Entry id.
        entry_id: String,
        /// Surface id.
        surface_id: ClaimSurface,
    },
    /// A projection references a report not linked by the entry.
    ProjectionUnknownReportRef {
        /// Entry id.
        entry_id: String,
        /// Destination ref.
        destination_ref: String,
        /// Unknown report ref.
        report_ref: String,
    },
    /// A projection renders wider than the entry's effective claim.
    SurfaceOverclaims {
        /// Entry id.
        entry_id: String,
        /// Surface id.
        surface_id: ClaimSurface,
        /// Rendered claim.
        rendered: EffectiveClaim,
        /// Effective claim.
        effective: EffectiveClaim,
    },
    /// A required surface is missing.
    MissingSurfaceProjection {
        /// Entry id.
        entry_id: String,
        /// Surface id.
        surface_id: ClaimSurface,
    },
    /// A surface appears more than once for an entry.
    DuplicateSurfaceProjection {
        /// Entry id.
        entry_id: String,
        /// Surface id.
        surface_id: ClaimSurface,
    },
    /// An evaluation filter points at the wrong public manifest.
    EvaluationFilterWrongManifest {
        /// Filter id.
        filter_id: String,
    },
    /// An evaluation filter includes no entries.
    EvaluationFilterEmpty {
        /// Filter id.
        filter_id: String,
    },
    /// A filter or projection references an unknown entry.
    UnknownEntryRef {
        /// Referrer id.
        id: String,
        /// Unknown entry id.
        entry_id: String,
    },
    /// A private evaluation filter would render wider than a public entry.
    EvaluationFilterOverclaims {
        /// Filter id.
        filter_id: String,
        /// Entry id that would be overclaimed.
        entry_id: String,
    },
    /// Publication decision disagrees with the computed decision.
    PublicationDecisionMismatch {
        /// Declared decision.
        declared: ClaimPublicationDecision,
        /// Computed decision.
        computed: ClaimPublicationDecision,
    },
    /// Publication blocking set disagrees with the computed set.
    PublicationBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// Summary counts disagree with recomputed counts.
    SummaryMismatch,
}

/// Parses the checked-in claim-publication manifest.
///
/// # Errors
///
/// Returns a JSON parse error when the embedded manifest no longer matches the
/// typed model.
pub fn current_claim_publication_manifest() -> Result<ClaimPublicationManifest, serde_json::Error> {
    serde_json::from_str(CLAIM_PUBLICATION_MANIFEST_JSON)
}
