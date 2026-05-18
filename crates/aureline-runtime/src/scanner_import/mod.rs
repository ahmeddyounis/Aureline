//! Imported scanner diagnostics and export-safe quality evidence packets.
//!
//! This module normalizes SARIF and structured scanner payloads into read-only
//! imported diagnostic rows, compact delta packets, review-quality packets,
//! CLI projections, support exports, and release evidence packets. Raw scanner
//! bodies stay behind opaque refs; consumers receive stable ids, class labels,
//! counts, and redaction-safe summaries.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use aureline_language::{
    DiagnosticAnchor, DiagnosticAnchorRemapStateClass, DiagnosticBus, DiagnosticBusSnapshot,
    DiagnosticEnvelope, DiagnosticEvidencePlaneClass, DiagnosticEvidenceRef,
    DiagnosticEvidenceRoleClass, DiagnosticFreshness, DiagnosticFreshnessClass,
    DiagnosticOriginClass, DiagnosticScope, DiagnosticSeverityClass, DiagnosticSourceDescriptor,
    DiagnosticSourceFamily, RedactionClass, RouterCompletenessClass, RouterLocalityClass,
    RouterScopeClaimClass, RouterSupportClass, ScopeLimitClass, DIAGNOSTIC_BUS_SCHEMA_VERSION,
};
use serde::{Deserialize, Serialize};

/// Schema version used by imported scanner alpha records.
pub const SCANNER_IMPORT_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Support-pack item id used by imported diagnostic support exports.
pub const IMPORTED_DIAGNOSTICS_SUPPORT_ITEM_ID: &str = "support.item.imported_diagnostics";

/// Stable record-kind tag for scanner import sessions.
pub const SCANNER_IMPORT_SESSION_RECORD_KIND: &str = "scanner_import_session_alpha";

/// Stable record-kind tag for scanner delta packets.
pub const DIAGNOSTIC_DELTA_RECORD_KIND: &str = "diagnostic_delta_alpha_record";

/// Stable record-kind tag for suppression and baseline registers.
pub const SUPPRESSION_BASELINE_REGISTER_RECORD_KIND: &str = "suppression_baseline_register_alpha";

/// Stable record-kind tag for diagnostic review packets.
pub const DIAGNOSTIC_REVIEW_PACKET_RECORD_KIND: &str = "diagnostic_review_packet_alpha";

/// Stable record-kind tag for imported scanner Problems projections.
pub const PROBLEMS_PROJECTION_RECORD_KIND: &str = "imported_scanner_problems_projection_alpha";

/// Stable record-kind tag for imported scanner support exports.
pub const SUPPORT_EXPORT_RECORD_KIND: &str = "imported_scanner_support_export_alpha";

/// Stable record-kind tag for review-quality packets.
pub const REVIEW_QUALITY_PACKET_RECORD_KIND: &str = "review_quality_packet";

/// Stable record-kind tag for scanner import CLI projections.
pub const CLI_PROJECTION_RECORD_KIND: &str = "scanner_import_cli_projection";

/// Stable record-kind tag for scanner import release packets.
pub const RELEASE_PACKET_RECORD_KIND: &str = "scanner_import_release_packet";

/// Error returned while normalizing a scanner import.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScannerImportError {
    /// The scanner payload was not valid JSON for the supported alpha shape.
    InvalidJson(String),
    /// The import request did not include an opaque raw-payload reference.
    MissingRawPayloadRef,
    /// The import request did not include an opaque source-artifact reference.
    MissingSourceArtifactRef,
    /// The payload had no scanner runs.
    NoRuns,
    /// The structured scanner payload omitted a required object.
    InvalidStructuredPayload(String),
}

impl fmt::Display for ScannerImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidJson(err) => write!(f, "scanner import payload is invalid JSON: {err}"),
            Self::MissingRawPayloadRef => {
                f.write_str("scanner import requires an opaque raw-payload ref")
            }
            Self::MissingSourceArtifactRef => {
                f.write_str("scanner import requires an opaque source-artifact ref")
            }
            Self::NoRuns => f.write_str("scanner import payload contains no runs"),
            Self::InvalidStructuredPayload(err) => {
                write!(f, "structured scanner import payload is invalid: {err}")
            }
        }
    }
}

impl Error for ScannerImportError {}

/// Request metadata that binds an imported scanner payload to Aureline truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerImportRequest {
    /// Stable import session id.
    pub import_id: String,
    /// Workspace id covered by the import.
    pub workspace_id: String,
    /// Diagnostic collection id that will receive imported rows.
    pub collection_id: String,
    /// Opaque artifact ref for the scanner payload.
    pub source_artifact_ref: String,
    /// Opaque ref for the preserved raw payload body.
    pub raw_payload_ref: String,
    /// Media type declared by the importer.
    pub media_type: String,
    /// Timestamp at which Aureline imported the payload.
    pub imported_at: String,
    /// Freshness state of the imported payload relative to the current target.
    #[serde(default)]
    pub import_freshness_class: ScannerImportFreshnessClass,
    /// Policy for retaining or omitting raw scanner payload backlinks.
    #[serde(default)]
    pub raw_payload_backlink_policy: ScannerRawPayloadBacklinkPolicy,
    /// Target scope declared for every run in this alpha import.
    pub target_scope: ScannerTargetScopeBinding,
    /// Revision binding used to prevent current-truth overclaiming.
    pub revision_binding: ScannerRevisionBinding,
    /// Rule-pack and baseline family binding for delta comparisons.
    pub rule_pack: ScannerRulePackBinding,
    /// Scanner category refs, such as security or compliance.
    #[serde(default)]
    pub scanner_category_refs: Vec<String>,
    /// Rule-family mappings that can offer a local confirmation path.
    #[serde(default)]
    pub rule_family_mappings: Vec<ScannerRuleFamilyMapping>,
    /// Accepted baseline entries used by the delta packet.
    #[serde(default)]
    pub baseline_entries: Vec<ScannerBaselineEntry>,
    /// Governed suppressions and waivers applied to imported findings.
    #[serde(default)]
    pub suppression_entries: Vec<ScannerSuppressionEntry>,
    /// Current local confirmations for mapped rule families.
    #[serde(default)]
    pub local_confirmations: Vec<ScannerLocalConfirmation>,
}

/// Declared freshness state for an imported scanner session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScannerImportFreshnessClass {
    /// Imported snapshot is current enough for inspection but is not live truth.
    ImportedSnapshot,
    /// Imported snapshot is stale and needs a local rerun or confirmation.
    StaleImportedSnapshot,
    /// Imported snapshot has been superseded by newer evidence.
    SupersededImportedSnapshot,
    /// The importer could not prove freshness.
    UnverifiedImportedSnapshot,
}

impl Default for ScannerImportFreshnessClass {
    fn default() -> Self {
        Self::ImportedSnapshot
    }
}

impl ScannerImportFreshnessClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImportedSnapshot => "imported_snapshot",
            Self::StaleImportedSnapshot => "stale_imported_snapshot",
            Self::SupersededImportedSnapshot => "superseded_imported_snapshot",
            Self::UnverifiedImportedSnapshot => "unverified_imported_snapshot",
        }
    }

    /// Projects scanner-import freshness into the diagnostic bus vocabulary.
    pub const fn diagnostic_freshness(self) -> DiagnosticFreshnessClass {
        match self {
            Self::ImportedSnapshot => DiagnosticFreshnessClass::ImportedSnapshot,
            Self::StaleImportedSnapshot => DiagnosticFreshnessClass::Stale,
            Self::SupersededImportedSnapshot => DiagnosticFreshnessClass::Superseded,
            Self::UnverifiedImportedSnapshot => DiagnosticFreshnessClass::Unverified,
        }
    }

    /// Returns true when review should prefer a local confirmation or rerun.
    pub const fn needs_confirmation(self) -> bool {
        !matches!(self, Self::ImportedSnapshot)
    }
}

/// Raw-payload backlink policy applied to scanner import packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScannerRawPayloadBacklinkPolicy {
    /// Opaque raw-payload refs are retained; raw bodies remain excluded.
    OpaqueRefsOnly,
    /// Raw-payload refs are redacted from ordinary exports.
    RedactedByPolicy,
    /// Raw-payload bodies are omitted because no durable ref is available.
    OmittedUnavailable,
    /// User explicitly opted into a richer restricted export elsewhere.
    ExplicitOptInRestricted,
}

impl Default for ScannerRawPayloadBacklinkPolicy {
    fn default() -> Self {
        Self::OpaqueRefsOnly
    }
}

impl ScannerRawPayloadBacklinkPolicy {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpaqueRefsOnly => "opaque_refs_only",
            Self::RedactedByPolicy => "redacted_by_policy",
            Self::OmittedUnavailable => "omitted_unavailable",
            Self::ExplicitOptInRestricted => "explicit_opt_in_restricted",
        }
    }

    /// Returns true when ordinary packets can expose raw-payload backlinks.
    pub const fn exposes_backlink_refs(self) -> bool {
        matches!(self, Self::OpaqueRefsOnly | Self::ExplicitOptInRestricted)
    }

    /// Returns true when exports must render payload redaction explicitly.
    pub const fn is_redacted(self) -> bool {
        matches!(self, Self::RedactedByPolicy | Self::OmittedUnavailable)
    }
}

/// Target scope covered by one scanner import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerTargetScopeBinding {
    /// Controlled target-scope class.
    pub target_scope_class: ScannerTargetScopeClass,
    /// Opaque target scope ref.
    pub scope_ref: String,
    /// Opaque execution context ref, when known.
    pub execution_context_ref: String,
    /// Opaque environment ref, when known.
    pub environment_ref: Option<String>,
    /// Export-safe scope summary.
    pub summary: String,
}

/// Target-scope vocabulary for scanner imports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScannerTargetScopeClass {
    /// Scan covers one current file or object.
    CurrentFile,
    /// Scan covers the admitted workspace root.
    CurrentRoot,
    /// Scan covers the selected workset.
    SelectedWorkset,
    /// Scan covers the whole workspace.
    Workspace,
    /// Scan covers changed files for a review.
    ChangedFiles,
    /// Scan covers a review diff.
    ReviewDiff,
    /// Scan covers a baseline family.
    BaselineFamily,
    /// Scan covers a release candidate.
    ReleaseCandidate,
    /// Scan came from a support export.
    SupportExport,
    /// Scan covers a provider project.
    ProviderProject,
    /// Scope is unknown and cannot claim completeness.
    TargetScopeUnknownRequiresReview,
}

/// Revision identity for imported scanner evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerRevisionBinding {
    /// Revision ref the scanner targeted.
    pub target_revision_ref: String,
    /// Current workspace revision ref, when known.
    pub current_revision_ref: Option<String>,
    /// Compatibility note for the revision binding.
    pub compatibility_note: String,
}

/// Rule-pack and baseline-family binding for scanner imports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerRulePackBinding {
    /// Opaque rule-pack ref.
    pub rule_pack_ref: String,
    /// Rule-pack version label.
    pub rule_pack_version: String,
    /// Opaque rule-pack digest ref.
    pub rule_pack_digest_ref: String,
    /// Opaque compatible baseline-family ref.
    pub baseline_family_ref: String,
    /// Compatibility class for the baseline family.
    pub baseline_family_state_class: ScannerBaselineFamilyStateClass,
}

/// Baseline-family compatibility class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScannerBaselineFamilyStateClass {
    /// Baseline comparison is compatible.
    Compatible,
    /// Baseline comparison is compatible only with local confirmation.
    CompatibleWithLocalConfirmation,
    /// Baseline is stale but still comparable.
    StaleButComparable,
    /// Rule-pack drift blocks baseline comparison.
    IncompatibleRulePack,
    /// Profile drift blocks baseline comparison.
    IncompatibleProfile,
    /// Mapping drift blocks baseline comparison.
    IncompatibleMappingFamily,
    /// Compatibility is unknown and requires review.
    CompatibilityUnknownRequiresReview,
}

impl ScannerBaselineFamilyStateClass {
    /// Returns true when baseline comparison must be downgraded or blocked.
    pub const fn blocks_compatible_delta(self) -> bool {
        matches!(
            self,
            Self::IncompatibleRulePack
                | Self::IncompatibleProfile
                | Self::IncompatibleMappingFamily
                | Self::CompatibilityUnknownRequiresReview
        )
    }
}

/// Rule-family mapping that can confirm imported evidence locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerRuleFamilyMapping {
    /// Stable rule family ref.
    pub rule_family_ref: String,
    /// Imported scanner rule-id prefix covered by this mapping.
    pub imported_rule_id_prefix: String,
    /// Local rule ref expected to confirm this family.
    pub local_rule_ref: String,
    /// Local provider or analyzer ref expected to confirm this family.
    pub local_provider_ref: String,
    /// Review action ref for running local confirmation.
    pub confirmation_action_ref: String,
    /// Export-safe mapping summary.
    pub summary: String,
}

/// Baseline entry used for delta comparison.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerBaselineEntry {
    /// Stable baseline entry id.
    pub baseline_entry_id: String,
    /// Opaque baseline snapshot ref.
    pub baseline_ref: String,
    /// Imported or local rule ref.
    pub rule_id_ref: String,
    /// Opaque anchor fingerprint ref, when comparable.
    pub anchor_fingerprint_ref: Option<String>,
    /// Owner ref for the accepted debt.
    pub owner_ref: String,
    /// Version of the baseline register entry.
    pub version: u32,
    /// True when the debt must appear in release/support packets.
    pub release_visible: bool,
    /// Export-safe baseline summary.
    pub summary: String,
}

/// Suppression or waiver entry applied during import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerSuppressionEntry {
    /// Stable suppression or waiver id.
    pub suppression_id: String,
    /// Imported or local rule ref.
    pub rule_id_ref: String,
    /// Opaque anchor fingerprint ref, when comparable.
    pub anchor_fingerprint_ref: Option<String>,
    /// Suppression or waiver state.
    pub debt_state: ScannerDebtState,
    /// Owner ref for the exception.
    pub owner_ref: String,
    /// Actor ref that created or last renewed the record.
    pub actor_ref: String,
    /// Opaque reason ref.
    pub reason_ref: String,
    /// Expiry timestamp for the record.
    pub expires_at: Option<String>,
    /// Reopen rule for the debt.
    pub reopen_rule_class: ScannerReopenRuleClass,
    /// Evidence refs attached to the record.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Version of the suppression register entry.
    pub version: u32,
    /// True when the debt must appear in release/support packets.
    pub release_visible: bool,
    /// Export-safe suppression summary.
    pub summary: String,
}

/// State of a suppression-like debt record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScannerDebtState {
    /// Finding remains present but is governed by a suppression.
    Suppressed,
    /// Finding remains present but has an approved waiver.
    Waived,
}

/// Reopen behavior for suppression and waiver debt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScannerReopenRuleClass {
    /// Reopen when the expiry timestamp passes.
    ReopenOnExpiry,
    /// Reopen when the rule pack changes.
    ReopenOnRuleChange,
    /// Reopen when anchor remapping fails.
    ReopenOnAnchorRemapFailure,
    /// Reopen when profile or target identity drifts.
    ReopenOnProfileOrTargetDrift,
    /// Reopen only during manual review.
    ManualReviewOnly,
}

/// Local confirmation that keeps imported and live truth distinct.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerLocalConfirmation {
    /// Stable local confirmation ref.
    pub local_confirmation_ref: String,
    /// Confirmed rule family ref.
    pub rule_family_ref: String,
    /// Imported or local rule ref.
    pub rule_id_ref: String,
    /// Opaque anchor fingerprint ref, when comparable.
    pub anchor_fingerprint_ref: Option<String>,
    /// Local diagnostic ref produced by the confirming analyzer.
    pub local_diagnostic_ref: String,
    /// Local run or session ref.
    pub local_run_ref: String,
    /// Timestamp at which local confirmation was produced.
    pub confirmed_at: String,
    /// Export-safe confirmation summary.
    pub summary: String,
}

/// Normalized imported scanner session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerImportSessionAlpha {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub scanner_import_schema_version: u32,
    /// Stable import session id.
    pub import_id: String,
    /// Workspace id covered by the import.
    pub workspace_id: String,
    /// Diagnostic collection id that receives imported rows.
    pub collection_id: String,
    /// Media type declared for the imported payload.
    pub media_type: String,
    /// Opaque source artifact ref.
    pub source_artifact_ref: String,
    /// Opaque raw-payload refs preserved for expert review.
    pub raw_payload_refs: Vec<String>,
    /// Freshness state of the imported payload relative to the current target.
    pub import_freshness_class: ScannerImportFreshnessClass,
    /// Policy for retaining or omitting raw scanner payload backlinks.
    pub raw_payload_backlink_policy: ScannerRawPayloadBacklinkPolicy,
    /// Target scope declared for the import.
    pub target_scope: ScannerTargetScopeBinding,
    /// Revision binding for the import.
    pub revision_binding: ScannerRevisionBinding,
    /// Run descriptors normalized from the payload.
    pub run_descriptors: Vec<ScannerRunDescriptor>,
    /// Imported findings normalized from the payload.
    pub findings: Vec<ImportedScannerFinding>,
    /// Delta packet for imported, baseline, suppression, and waiver state.
    pub delta_packet: ScannerDeltaPacketAlpha,
    /// Suppression and baseline register consumed by the packet.
    pub suppression_baseline_register: SuppressionBaselineRegisterAlpha,
    /// Review packet consumed by review and support surfaces.
    pub review_packet: DiagnosticReviewPacketAlpha,
    /// Redaction posture for the session.
    pub redaction_class: RedactionClass,
    /// Timestamp at which Aureline imported the payload.
    pub imported_at: String,
    /// Export-safe session summary.
    pub export_safe_summary: String,
}

impl ScannerImportSessionAlpha {
    /// Publishes imported findings into the shared diagnostic bus.
    pub fn publish_to_diagnostic_bus(&self, bus: &mut DiagnosticBus) {
        for diagnostic in self.diagnostic_envelopes() {
            bus.publish(diagnostic);
        }
    }

    /// Builds diagnostic bus envelopes for every current imported finding.
    pub fn diagnostic_envelopes(&self) -> Vec<DiagnosticEnvelope> {
        self.findings
            .iter()
            .map(|finding| self.diagnostic_envelope_for(finding))
            .collect()
    }

    /// Builds a Problems projection that keeps imported evidence read-only.
    pub fn problems_projection(
        &self,
        bus_snapshot: Option<&DiagnosticBusSnapshot>,
    ) -> ImportedScannerProblemsProjection {
        let rows = self
            .findings
            .iter()
            .map(|finding| ImportedScannerProblemRow {
                finding_id: finding.finding_id.clone(),
                diagnostic_id: finding.diagnostic_id.clone(),
                rule_id_ref: finding.rule_id_ref.clone(),
                severity_class: finding.severity_class,
                truth_class: finding.truth_class,
                fidelity_state_class: finding.fidelity_state_class,
                delta_state_class: finding.delta_state_class,
                remap_state_class: finding.anchor.remap_state_class,
                import_freshness_class: self.import_freshness_class,
                local_confirmation_state_class: finding.local_confirmation_state_class,
                local_confirmation_action_ref: finding.local_confirmation_action_ref.clone(),
                read_only: finding.read_only,
                raw_payload_ref: raw_payload_ref_for_policy(
                    &finding.raw_payload_ref,
                    self.raw_payload_backlink_policy,
                ),
                raw_payload_backlink_policy: self.raw_payload_backlink_policy,
                export_safe_summary: finding.export_safe_summary.clone(),
            })
            .collect::<Vec<_>>();
        ImportedScannerProblemsProjection {
            record_kind: PROBLEMS_PROJECTION_RECORD_KIND.to_owned(),
            scanner_import_schema_version: SCANNER_IMPORT_ALPHA_SCHEMA_VERSION,
            projection_id: format!("problems:scanner_import:{}", sanitize_ref(&self.import_id)),
            import_session_ref: self.import_id.clone(),
            diagnostic_bus_snapshot_ref: bus_snapshot.map(|snapshot| snapshot.snapshot_id.clone()),
            imported_count: rows.len(),
            locally_confirmed_count: rows
                .iter()
                .filter(|row| row.truth_class == ScannerFindingTruthClass::LocallyConfirmed)
                .count(),
            read_only_count: rows.iter().filter(|row| row.read_only).count(),
            rows,
            redaction_class: self.redaction_class,
            export_safe_summary: "Problems projection keeps scanner-import rows read-only and labeled as imported evidence.".into(),
        }
    }

    /// Builds an export-safe support packet for imported scanner evidence.
    pub fn support_export(
        &self,
        bus_snapshot: Option<&DiagnosticBusSnapshot>,
    ) -> ImportedScannerSupportExport {
        let rows = self
            .findings
            .iter()
            .map(|finding| ImportedScannerSupportRow {
                finding_id: finding.finding_id.clone(),
                diagnostic_id: finding.diagnostic_id.clone(),
                rule_id_ref: finding.rule_id_ref.clone(),
                rule_family_ref: finding.rule_family_ref.clone(),
                delta_state_class: finding.delta_state_class,
                truth_class: finding.truth_class,
                fidelity_state_class: finding.fidelity_state_class,
                local_confirmation_ref: finding.local_confirmation_ref.clone(),
                remap_state_class: finding.anchor.remap_state_class,
                import_freshness_class: self.import_freshness_class,
                local_confirmation_action_ref: finding.local_confirmation_action_ref.clone(),
                read_only: finding.read_only,
                raw_payload_ref: raw_payload_ref_for_policy(
                    &finding.raw_payload_ref,
                    self.raw_payload_backlink_policy,
                ),
                raw_payload_backlink_policy: self.raw_payload_backlink_policy,
                evidence_refs: evidence_refs_for_policy(
                    &finding.evidence_refs,
                    self.raw_payload_backlink_policy,
                ),
                redaction_class: finding.redaction_class,
                export_safe_summary: finding.export_safe_summary.clone(),
            })
            .collect::<Vec<_>>();
        ImportedScannerSupportExport {
            record_kind: SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            scanner_import_schema_version: SCANNER_IMPORT_ALPHA_SCHEMA_VERSION,
            export_id: format!("support_export:scanner_import:{}", sanitize_ref(&self.import_id)),
            support_pack_item_id: IMPORTED_DIAGNOSTICS_SUPPORT_ITEM_ID.to_owned(),
            import_session_ref: self.import_id.clone(),
            diagnostic_bus_snapshot_ref: bus_snapshot.map(|snapshot| snapshot.snapshot_id.clone()),
            diagnostic_delta_packet_ref: self.delta_packet.delta_packet_id.clone(),
            suppression_baseline_register_ref: self
                .suppression_baseline_register
                .register_id
                .clone(),
            review_packet_ref: self.review_packet.packet_id.clone(),
            imported_finding_count: self.findings.len(),
            locally_confirmed_count: self
                .findings
                .iter()
                .filter(|finding| finding.truth_class == ScannerFindingTruthClass::LocallyConfirmed)
                .count(),
            read_only_count: self.findings.iter().filter(|finding| finding.read_only).count(),
            release_visible_debt_count: self
                .suppression_baseline_register
                .release_visible_debt_count,
            delta_counts: self.delta_packet.delta_counts.clone(),
            raw_payload_refs: raw_payload_refs_for_policy(
                &self.raw_payload_refs,
                self.raw_payload_backlink_policy,
            ),
            raw_payload_backlink_policy: self.raw_payload_backlink_policy,
            payload_backlinks_redacted: self.raw_payload_backlink_policy.is_redacted(),
            raw_private_material_excluded: true,
            rows,
            redaction_class: self.redaction_class,
            export_safe_summary: "Support export includes imported scanner lineage, delta state, local-confirmation refs, and release-visible debt without raw scanner bodies.".into(),
        }
    }

    /// Builds a CLI/headless projection with no raw scanner bodies.
    pub fn cli_projection(&self) -> ScannerImportCliProjection {
        ScannerImportCliProjection::from_session(self)
    }

    /// Builds a release evidence packet for scanner-import quality claims.
    pub fn release_packet(
        &self,
        release_scope_ref: impl Into<String>,
    ) -> ScannerImportReleasePacket {
        ScannerImportReleasePacket::from_session(self, release_scope_ref)
    }

    /// Returns true when the packet blocks exact delta or local-analysis claims.
    pub fn blocks_exact_delta_claims(&self) -> bool {
        self.delta_packet.compatibility_class.blocks_exact_delta()
            || self
                .findings
                .iter()
                .any(|finding| finding.fidelity_state_class.blocks_exact_delta())
    }

    fn diagnostic_envelope_for(&self, finding: &ImportedScannerFinding) -> DiagnosticEnvelope {
        let run = self
            .run_descriptors
            .iter()
            .find(|run| run.run_id == finding.run_id)
            .expect("imported finding references a run descriptor");
        DiagnosticEnvelope {
            record_kind: DiagnosticEnvelope::RECORD_KIND.into(),
            diagnostic_bus_schema_version: DIAGNOSTIC_BUS_SCHEMA_VERSION,
            diagnostic_id: finding.diagnostic_id.clone(),
            collection_id: self.collection_id.clone(),
            workspace_id: self.workspace_id.clone(),
            source: DiagnosticSourceDescriptor {
                source_descriptor_id: format!(
                    "source:scanner_import:{}",
                    sanitize_ref(&finding.finding_id)
                ),
                source_family: DiagnosticSourceFamily::ScannerImport,
                evidence_plane_class: DiagnosticEvidencePlaneClass::ImportedSnapshotEvidence,
                origin_class: DiagnosticOriginClass::ImportedSnapshot,
                producer_ref: run.tool_id.clone(),
                producer_version_ref: Some(run.tool_version.clone()),
                provider_id: run.provider_ref.clone(),
                router_host_ref: None,
                locality_class: RouterLocalityClass::ImportedSnapshot,
                support_class: RouterSupportClass::InspectOnly,
                summary: "Imported scanner source is inspect-only snapshot evidence.".into(),
            },
            severity_class: finding.severity_class,
            rule_id_ref: Some(finding.rule_id_ref.clone()),
            category_ref: finding.category_refs.first().cloned(),
            freshness: DiagnosticFreshness {
                freshness_class: self.import_freshness_class.diagnostic_freshness(),
                observed_at: run.scan_completed_at.clone(),
                epoch_ref: Some(self.import_id.clone()),
                invalidation_ref: None,
                summary: format!(
                    "Freshness is bound to the imported scan completion time and rendered as {}.",
                    self.import_freshness_class.as_str()
                ),
            },
            scope: DiagnosticScope {
                scope_claim_class: self.target_scope.target_scope_class.into(),
                completeness_class: finding.completeness_class,
                scope_limit_classes: self.target_scope.target_scope_class.scope_limits(),
                target_ref: self.target_scope.scope_ref.clone(),
                execution_context_id: self.target_scope.execution_context_ref.clone(),
                summary: self.target_scope.summary.clone(),
            },
            anchor: DiagnosticAnchor {
                anchor_family_id: finding.anchor.anchor_family_id.clone(),
                current_anchor_ref: finding.anchor.current_anchor_ref.clone(),
                path_ref: finding.anchor.artifact_ref.clone(),
                remap_state_class: finding.anchor.remap_state_class,
                summary: finding.anchor.summary.clone(),
            },
            evidence_refs: vec![
                DiagnosticEvidenceRef {
                    evidence_ref: raw_payload_ref_for_policy(
                        &finding.raw_payload_ref,
                        self.raw_payload_backlink_policy,
                    ),
                    evidence_role_class: DiagnosticEvidenceRoleClass::PrimarySource,
                    summary: "Raw scanner payload is retained by opaque ref only.".into(),
                },
                DiagnosticEvidenceRef {
                    evidence_ref: run.run_id.clone(),
                    evidence_role_class: DiagnosticEvidenceRoleClass::ProducerSession,
                    summary: "Run descriptor preserves tool, target, baseline, and mapping truth."
                        .into(),
                },
                DiagnosticEvidenceRef {
                    evidence_ref: self.delta_packet.delta_packet_id.clone(),
                    evidence_role_class: DiagnosticEvidenceRoleClass::RemapEvidence,
                    summary:
                        "Delta packet preserves baseline, suppression, waiver, and unmapped state."
                            .into(),
                },
            ],
            provider_status_refs: run.provider_ref.iter().cloned().collect(),
            router_decision_ref: None,
            redaction_class: self.redaction_class,
            captured_at: self.imported_at.clone(),
            export_safe_summary: finding.export_safe_summary.clone(),
        }
    }
}

/// Descriptor for one imported scanner run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerRunDescriptor {
    /// Stable run id.
    pub run_id: String,
    /// Scanner tool id.
    pub tool_id: String,
    /// Scanner display name.
    pub tool_name: String,
    /// Scanner tool version.
    pub tool_version: String,
    /// Import adapter id.
    pub adapter_id: String,
    /// Import adapter version.
    pub adapter_version: String,
    /// Provider ref, when the scan came from a provider.
    pub provider_ref: Option<String>,
    /// Rule-pack ref for this run.
    pub rule_pack_ref: String,
    /// Rule-pack version for this run.
    pub rule_pack_version: String,
    /// Target scope covered by this run.
    pub target_scope: ScannerTargetScopeBinding,
    /// Revision binding for this run.
    pub revision_binding: ScannerRevisionBinding,
    /// Media type of the imported payload.
    pub media_type: String,
    /// Source format class declared by the importer.
    pub source_format_class: ScannerSourceFormatClass,
    /// Freshness state for the imported run.
    pub import_freshness_class: ScannerImportFreshnessClass,
    /// Raw-payload backlink policy for this run.
    pub raw_payload_backlink_policy: ScannerRawPayloadBacklinkPolicy,
    /// Run-level mapping quality.
    pub mapping_quality_class: ScannerMappingQualityClass,
    /// Baseline family ref used for delta comparison.
    pub baseline_family_ref: String,
    /// Opaque raw-payload refs preserved for the run.
    pub raw_payload_refs: Vec<String>,
    /// Category refs reported for the run.
    pub category_refs: Vec<String>,
    /// Count of results in the run.
    pub result_count: usize,
    /// Scan completion timestamp.
    pub scan_completed_at: String,
    /// Export-safe run summary.
    pub export_safe_summary: String,
}

/// Mapping quality for imported scanner anchors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScannerMappingQualityClass {
    /// Every imported result had an admitted static anchor.
    ExactStatic,
    /// At least one imported result needed contextual or partial mapping.
    Contextual,
    /// No imported results can be projected to a safe anchor.
    Unmapped,
}

/// Scanner source format normalized through the shared import model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScannerSourceFormatClass {
    /// OASIS SARIF 2.1.0 JSON.
    Sarif21,
    /// Structured scanner JSON with explicit runs and results.
    StructuredScannerJson,
    /// Unknown or unsupported scanner shape retained for review.
    UnknownRequiresReview,
}

impl ScannerSourceFormatClass {
    /// Derives a source-format class from a declared media type.
    pub fn from_media_type(media_type: &str) -> Self {
        match media_type {
            "application/sarif+json" | "application/sarif-external-properties+json" => {
                Self::Sarif21
            }
            "application/vnd.aureline.scanner+json"
            | "application/vnd.aureline.structured-scanner+json"
            | "application/scanner+json" => Self::StructuredScannerJson,
            _ => Self::UnknownRequiresReview,
        }
    }

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sarif21 => "sarif_2_1_0",
            Self::StructuredScannerJson => "structured_scanner_json",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// Imported scanner finding projected into Problems and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedScannerFinding {
    /// Stable imported finding id.
    pub finding_id: String,
    /// Stable diagnostic id used by the diagnostic bus.
    pub diagnostic_id: String,
    /// Run descriptor ref.
    pub run_id: String,
    /// Rule id reported by the scanner.
    pub rule_id_ref: String,
    /// Rule family ref when the rule is mapped.
    pub rule_family_ref: Option<String>,
    /// Normalized severity class.
    pub severity_class: DiagnosticSeverityClass,
    /// Category refs attached to the finding.
    pub category_refs: Vec<String>,
    /// Current imported-vs-live truth class.
    pub truth_class: ScannerFindingTruthClass,
    /// Fidelity state that Problems, review, CLI, support, and release packets surface.
    pub fidelity_state_class: ScannerFindingFidelityClass,
    /// Delta state for the finding.
    pub delta_state_class: ScannerFindingDeltaState,
    /// Anchor projection for the finding.
    pub anchor: ImportedScannerAnchor,
    /// Diagnostic completeness class for the admitted scope.
    pub completeness_class: RouterCompletenessClass,
    /// Local confirmation state for this finding.
    pub local_confirmation_state_class: ScannerLocalConfirmationStateClass,
    /// Local confirmation action ref, when available.
    pub local_confirmation_action_ref: Option<String>,
    /// Local confirmation ref, when a current local run confirmed the finding.
    pub local_confirmation_ref: Option<String>,
    /// True because imported findings never mutate source directly.
    pub read_only: bool,
    /// Opaque raw-payload ref.
    pub raw_payload_ref: String,
    /// Supporting evidence refs.
    pub evidence_refs: Vec<String>,
    /// Redaction posture for this finding.
    pub redaction_class: RedactionClass,
    /// Export-safe finding summary.
    pub export_safe_summary: String,
}

/// Imported-vs-live truth class for a scanner finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScannerFindingTruthClass {
    /// Finding exists only as imported scanner evidence.
    ImportedOnly,
    /// A compatible local analyzer confirmed the same mapped family.
    LocallyConfirmed,
}

/// Fidelity state for an imported scanner finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScannerFindingFidelityClass {
    /// Imported finding has a static imported anchor and no additional degradation.
    ImportedStatic,
    /// Imported finding was confirmed by a compatible local run.
    LocallyConfirmed,
    /// Imported finding is stale relative to the current target.
    StaleImported,
    /// Imported finding was remapped contextually rather than exactly.
    RemappedContextual,
    /// Imported finding cannot be mapped to an admitted anchor.
    UnmappedAnchor,
    /// Baseline or rule-pack compatibility blocks a trustworthy delta claim.
    BaselineMismatch,
    /// Raw payload backlink was redacted or omitted by policy.
    RedactedPayload,
}

impl ScannerFindingFidelityClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImportedStatic => "imported_static",
            Self::LocallyConfirmed => "locally_confirmed",
            Self::StaleImported => "stale_imported",
            Self::RemappedContextual => "remapped_contextual",
            Self::UnmappedAnchor => "unmapped_anchor",
            Self::BaselineMismatch => "baseline_mismatch",
            Self::RedactedPayload => "redacted_payload",
        }
    }

    /// Returns true when the finding cannot support an exact delta claim.
    pub const fn blocks_exact_delta(self) -> bool {
        matches!(
            self,
            Self::StaleImported
                | Self::RemappedContextual
                | Self::UnmappedAnchor
                | Self::BaselineMismatch
                | Self::RedactedPayload
        )
    }
}

/// Local confirmation posture for imported evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScannerLocalConfirmationStateClass {
    /// Local confirmation can be requested for this mapped family.
    Available,
    /// Local confirmation already exists and is cited.
    Confirmed,
    /// Local confirmation is blocked because the anchor is unmapped.
    BlockedByUnmappedAnchor,
    /// No local confirmation path exists for this rule family.
    Unavailable,
}

/// Anchor projection for an imported scanner finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedScannerAnchor {
    /// Stable anchor family id.
    pub anchor_family_id: String,
    /// Opaque current anchor ref, if display is safe.
    pub current_anchor_ref: Option<String>,
    /// Opaque artifact ref, if known.
    pub artifact_ref: Option<String>,
    /// Opaque anchor fingerprint ref, if comparable.
    pub anchor_fingerprint_ref: Option<String>,
    /// Remap state used by inline, Problems, and export surfaces.
    pub remap_state_class: DiagnosticAnchorRemapStateClass,
    /// Mapping quality for this anchor.
    pub mapping_quality_class: ScannerMappingQualityClass,
    /// Export-safe anchor summary.
    pub summary: String,
}

/// Finding delta vocabulary used by scanner import alpha.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScannerFindingDeltaState {
    /// Finding is absent from the compatible baseline.
    New,
    /// Finding is absent from the current imported result set.
    Resolved,
    /// Finding persists from the compatible baseline.
    Persisting,
    /// Finding is present but governed by a suppression.
    Suppressed,
    /// Finding is present but governed by a waiver.
    Waived,
    /// Finding cannot be compared because mapping is unavailable.
    Unmapped,
}

/// Delta packet for imported scanner findings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerDeltaPacketAlpha {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub diagnostic_delta_schema_version: u32,
    /// Stable delta packet id.
    pub delta_packet_id: String,
    /// Source import session ref.
    pub import_session_ref: String,
    /// Baseline family ref.
    pub baseline_family_ref: String,
    /// Compatibility class for the delta packet.
    pub compatibility_class: ScannerDeltaCompatibilityClass,
    /// Per-state counts.
    pub delta_counts: ScannerDeltaCounts,
    /// Per-finding delta rows.
    pub finding_deltas: Vec<ScannerFindingDelta>,
    /// Redaction posture for the packet.
    pub redaction_class: RedactionClass,
    /// Export-safe packet summary.
    pub export_safe_summary: String,
}

/// Delta comparison compatibility class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScannerDeltaCompatibilityClass {
    /// Tool, rule-pack, profile, mapping, and target refs are compatible.
    CompatibleExact,
    /// Comparison is compatible but local confirmation is recommended.
    CompatibleWithLocalConfirmation,
    /// Comparison is blocked by profile or tool mismatch.
    BlockedProfileOrToolMismatch,
    /// Comparison is blocked by rule-pack mismatch.
    BlockedRulePackMismatch,
    /// Comparison is blocked by anchor mapping uncertainty.
    BlockedAnchorMappingUncertain,
    /// Comparison is not comparable and requires review.
    NotComparableUnknownRequiresReview,
}

impl ScannerDeltaCompatibilityClass {
    /// Returns true when consumers must not claim an exact compatible delta.
    pub const fn blocks_exact_delta(self) -> bool {
        !matches!(
            self,
            Self::CompatibleExact | Self::CompatibleWithLocalConfirmation
        )
    }

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompatibleExact => "compatible_exact",
            Self::CompatibleWithLocalConfirmation => "compatible_with_local_confirmation",
            Self::BlockedProfileOrToolMismatch => "blocked_profile_or_tool_mismatch",
            Self::BlockedRulePackMismatch => "blocked_rule_pack_mismatch",
            Self::BlockedAnchorMappingUncertain => "blocked_anchor_mapping_uncertain",
            Self::NotComparableUnknownRequiresReview => "not_comparable_unknown_requires_review",
        }
    }
}

/// Counts for the six alpha finding-delta states.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ScannerDeltaCounts {
    /// Count of new findings.
    pub new_count: usize,
    /// Count of resolved baseline findings.
    pub resolved_count: usize,
    /// Count of findings persisting from baseline.
    pub persisting_count: usize,
    /// Count of suppressed findings.
    pub suppressed_count: usize,
    /// Count of waived findings.
    pub waived_count: usize,
    /// Count of unmapped findings.
    pub unmapped_count: usize,
}

impl ScannerDeltaCounts {
    /// Builds counts from delta rows.
    pub fn from_deltas(deltas: &[ScannerFindingDelta]) -> Self {
        let mut counts = Self::default();
        for delta in deltas {
            match delta.delta_state_class {
                ScannerFindingDeltaState::New => counts.new_count += 1,
                ScannerFindingDeltaState::Resolved => counts.resolved_count += 1,
                ScannerFindingDeltaState::Persisting => counts.persisting_count += 1,
                ScannerFindingDeltaState::Suppressed => counts.suppressed_count += 1,
                ScannerFindingDeltaState::Waived => counts.waived_count += 1,
                ScannerFindingDeltaState::Unmapped => counts.unmapped_count += 1,
            }
        }
        counts
    }
}

/// Per-finding delta row for scanner import alpha.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerFindingDelta {
    /// Stable delta id.
    pub delta_id: String,
    /// Delta state.
    pub delta_state_class: ScannerFindingDeltaState,
    /// Current imported finding ref, when present.
    pub current_finding_ref: Option<String>,
    /// Baseline entry ref, when present.
    pub baseline_ref: Option<String>,
    /// Suppression or waiver ref, when present.
    pub suppression_ref: Option<String>,
    /// Local confirmation ref, when present.
    pub local_confirmation_ref: Option<String>,
    /// Compatibility note for this delta row.
    pub compatibility_note: String,
    /// Supporting evidence refs for this delta row.
    pub supporting_evidence_refs: Vec<String>,
    /// Export-safe delta summary.
    pub export_safe_summary: String,
}

/// Register for suppression, waiver, and baseline debt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuppressionBaselineRegisterAlpha {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub scanner_import_schema_version: u32,
    /// Stable register id.
    pub register_id: String,
    /// Source import session ref.
    pub import_session_ref: String,
    /// Baseline family ref.
    pub baseline_family_ref: String,
    /// Version of this register.
    pub register_version: u32,
    /// Baseline entries used for comparison.
    pub baseline_entries: Vec<ScannerBaselineEntry>,
    /// Suppression and waiver entries used for comparison.
    pub suppression_entries: Vec<ScannerSuppressionEntry>,
    /// Count of release-visible debt records.
    pub release_visible_debt_count: usize,
    /// Redaction posture for the register.
    pub redaction_class: RedactionClass,
    /// Export-safe register summary.
    pub export_safe_summary: String,
}

/// Review packet that keeps imported, live, and debt state visible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticReviewPacketAlpha {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub scanner_import_schema_version: u32,
    /// Stable review packet id.
    pub packet_id: String,
    /// Change or target scope ref reviewed by this packet.
    pub change_scope_ref: String,
    /// Source import session ref.
    pub import_session_ref: String,
    /// Diagnostic delta packet ref.
    pub diagnostic_delta_packet_ref: String,
    /// Suppression/baseline register ref.
    pub suppression_baseline_register_ref: String,
    /// Imported finding ids included in review.
    pub included_finding_ids: Vec<String>,
    /// Export-safe finding rows that preserve imported/local and fidelity labels.
    pub finding_rows: Vec<ScannerReviewFindingRow>,
    /// Quality-action refs available outside imported read-only rows.
    pub quality_action_refs: Vec<String>,
    /// Local confirmation actions available for mapped rule families.
    pub local_confirmation_actions: Vec<ScannerLocalConfirmationAction>,
    /// Count of imported findings.
    pub imported_finding_count: usize,
    /// Count of locally confirmed imported findings.
    pub locally_confirmed_finding_count: usize,
    /// Count of release-visible suppression or baseline debt records.
    pub release_visible_debt_count: usize,
    /// Delta counts projected into the review packet.
    pub delta_counts: ScannerDeltaCounts,
    /// Raw-payload backlink policy visible to review consumers.
    pub raw_payload_backlink_policy: ScannerRawPayloadBacklinkPolicy,
    /// Profile drift note for review.
    pub profile_drift_note: String,
    /// Redaction posture for review export.
    pub redaction_class: RedactionClass,
    /// Export-safe review summary.
    pub export_safe_summary: String,
}

/// One finding row inside a review-quality packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerReviewFindingRow {
    /// Stable imported finding id.
    pub finding_id: String,
    /// Diagnostic bus id.
    pub diagnostic_id: String,
    /// Rule id ref.
    pub rule_id_ref: String,
    /// Imported-vs-live truth class.
    pub truth_class: ScannerFindingTruthClass,
    /// Fidelity state for the finding.
    pub fidelity_state_class: ScannerFindingFidelityClass,
    /// Delta state for the finding.
    pub delta_state_class: ScannerFindingDeltaState,
    /// Anchor remap state used by UI and export consumers.
    pub remap_state_class: DiagnosticAnchorRemapStateClass,
    /// Freshness state of the import that produced the finding.
    pub import_freshness_class: ScannerImportFreshnessClass,
    /// True when the finding remains inspect-only imported evidence.
    pub read_only: bool,
    /// Raw-payload backlink policy applied to this row.
    pub raw_payload_backlink_policy: ScannerRawPayloadBacklinkPolicy,
    /// Export-safe row summary.
    pub export_safe_summary: String,
}

/// Local confirmation action advertised by review and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerLocalConfirmationAction {
    /// Stable action ref.
    pub action_ref: String,
    /// Rule family this action can confirm.
    pub rule_family_ref: String,
    /// Local provider ref used by the confirmation path.
    pub local_provider_ref: String,
    /// Local rule ref used by the confirmation path.
    pub local_rule_ref: String,
    /// True because confirmation is required before mutation claims.
    pub required_before_mutation: bool,
    /// Export-safe action summary.
    pub summary: String,
}

/// Problems projection for imported scanner findings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedScannerProblemsProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub scanner_import_schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// Source import session ref.
    pub import_session_ref: String,
    /// Diagnostic bus snapshot ref, when attached.
    pub diagnostic_bus_snapshot_ref: Option<String>,
    /// Count of imported rows.
    pub imported_count: usize,
    /// Count of locally confirmed rows.
    pub locally_confirmed_count: usize,
    /// Count of rows that remain read-only.
    pub read_only_count: usize,
    /// Problems rows.
    pub rows: Vec<ImportedScannerProblemRow>,
    /// Redaction posture for the projection.
    pub redaction_class: RedactionClass,
    /// Export-safe projection summary.
    pub export_safe_summary: String,
}

/// One Problems row for an imported scanner finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedScannerProblemRow {
    /// Stable imported finding id.
    pub finding_id: String,
    /// Diagnostic bus id.
    pub diagnostic_id: String,
    /// Rule id ref.
    pub rule_id_ref: String,
    /// Normalized severity class.
    pub severity_class: DiagnosticSeverityClass,
    /// Imported-vs-live truth class.
    pub truth_class: ScannerFindingTruthClass,
    /// Fidelity state for the finding.
    pub fidelity_state_class: ScannerFindingFidelityClass,
    /// Delta state.
    pub delta_state_class: ScannerFindingDeltaState,
    /// Anchor remap state used by Problems and inline projections.
    pub remap_state_class: DiagnosticAnchorRemapStateClass,
    /// Freshness state of the import that produced this row.
    pub import_freshness_class: ScannerImportFreshnessClass,
    /// Local confirmation state.
    pub local_confirmation_state_class: ScannerLocalConfirmationStateClass,
    /// Local confirmation action ref, when available.
    pub local_confirmation_action_ref: Option<String>,
    /// True when the row is read-only.
    pub read_only: bool,
    /// Opaque raw-payload ref.
    pub raw_payload_ref: String,
    /// Raw-payload backlink policy visible in Problems details.
    pub raw_payload_backlink_policy: ScannerRawPayloadBacklinkPolicy,
    /// Export-safe row summary.
    pub export_safe_summary: String,
}

/// Support-export packet for imported scanner evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedScannerSupportExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub scanner_import_schema_version: u32,
    /// Stable support export id.
    pub export_id: String,
    /// Support-pack item id.
    pub support_pack_item_id: String,
    /// Source import session ref.
    pub import_session_ref: String,
    /// Diagnostic bus snapshot ref, when attached.
    pub diagnostic_bus_snapshot_ref: Option<String>,
    /// Diagnostic delta packet ref.
    pub diagnostic_delta_packet_ref: String,
    /// Suppression/baseline register ref.
    pub suppression_baseline_register_ref: String,
    /// Diagnostic review packet ref.
    pub review_packet_ref: String,
    /// Count of imported findings.
    pub imported_finding_count: usize,
    /// Count of locally confirmed findings.
    pub locally_confirmed_count: usize,
    /// Count of findings that remain read-only.
    pub read_only_count: usize,
    /// Count of release-visible debt records.
    pub release_visible_debt_count: usize,
    /// Delta counts projected into the support packet.
    pub delta_counts: ScannerDeltaCounts,
    /// Opaque raw payload refs.
    pub raw_payload_refs: Vec<String>,
    /// Raw-payload backlink policy for this support export.
    pub raw_payload_backlink_policy: ScannerRawPayloadBacklinkPolicy,
    /// True when raw-payload refs are redacted or omitted by policy.
    pub payload_backlinks_redacted: bool,
    /// True when raw scanner bodies and private source material are excluded.
    pub raw_private_material_excluded: bool,
    /// Support rows for imported findings.
    pub rows: Vec<ImportedScannerSupportRow>,
    /// Redaction posture for the support export.
    pub redaction_class: RedactionClass,
    /// Export-safe support summary.
    pub export_safe_summary: String,
}

/// One support-export row for an imported scanner finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedScannerSupportRow {
    /// Stable imported finding id.
    pub finding_id: String,
    /// Diagnostic bus id.
    pub diagnostic_id: String,
    /// Rule id ref.
    pub rule_id_ref: String,
    /// Rule family ref, when mapped.
    pub rule_family_ref: Option<String>,
    /// Delta state.
    pub delta_state_class: ScannerFindingDeltaState,
    /// Imported-vs-live truth class.
    pub truth_class: ScannerFindingTruthClass,
    /// Fidelity state for the finding.
    pub fidelity_state_class: ScannerFindingFidelityClass,
    /// Local confirmation ref, when present.
    pub local_confirmation_ref: Option<String>,
    /// Anchor remap state used by support consumers.
    pub remap_state_class: DiagnosticAnchorRemapStateClass,
    /// Freshness state of the import that produced this row.
    pub import_freshness_class: ScannerImportFreshnessClass,
    /// Local confirmation action ref, when present.
    pub local_confirmation_action_ref: Option<String>,
    /// True when the imported row is read-only.
    pub read_only: bool,
    /// Opaque raw-payload ref.
    pub raw_payload_ref: String,
    /// Raw-payload backlink policy for this row.
    pub raw_payload_backlink_policy: ScannerRawPayloadBacklinkPolicy,
    /// Supporting evidence refs.
    pub evidence_refs: Vec<String>,
    /// Redaction posture for the row.
    pub redaction_class: RedactionClass,
    /// Export-safe row summary.
    pub export_safe_summary: String,
}

/// CLI/headless projection for scanner import sessions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerImportCliProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub scanner_import_schema_version: u32,
    /// Stable CLI projection id.
    pub projection_id: String,
    /// Source import session ref.
    pub import_session_ref: String,
    /// Source format class used by the import.
    pub source_format_class: ScannerSourceFormatClass,
    /// Imported payload freshness state.
    pub import_freshness_class: ScannerImportFreshnessClass,
    /// Delta compatibility class.
    pub compatibility_class: ScannerDeltaCompatibilityClass,
    /// Delta counts projected into CLI JSON.
    pub delta_counts: ScannerDeltaCounts,
    /// Count of imported findings.
    pub imported_finding_count: usize,
    /// Count of locally confirmed findings.
    pub locally_confirmed_count: usize,
    /// Count of read-only findings.
    pub read_only_count: usize,
    /// True when exact delta claims are blocked.
    pub exact_delta_claim_blocked: bool,
    /// Raw-payload refs retained according to policy.
    pub raw_payload_refs: Vec<String>,
    /// Raw-payload backlink policy for the projection.
    pub raw_payload_backlink_policy: ScannerRawPayloadBacklinkPolicy,
    /// CLI rows with stable labels and no raw source.
    pub rows: Vec<ScannerImportCliRow>,
    /// Redaction posture for the CLI projection.
    pub redaction_class: RedactionClass,
    /// Export-safe projection summary.
    pub export_safe_summary: String,
}

impl ScannerImportCliProjection {
    /// Builds a CLI projection from a normalized scanner import session.
    pub fn from_session(session: &ScannerImportSessionAlpha) -> Self {
        let source_format_class = ScannerSourceFormatClass::from_media_type(&session.media_type);
        let rows = session
            .findings
            .iter()
            .map(|finding| ScannerImportCliRow {
                finding_id: finding.finding_id.clone(),
                diagnostic_id: finding.diagnostic_id.clone(),
                rule_id_ref: finding.rule_id_ref.clone(),
                severity_class: finding.severity_class,
                truth_class: finding.truth_class,
                fidelity_state_class: finding.fidelity_state_class,
                delta_state_class: finding.delta_state_class,
                remap_state_class: finding.anchor.remap_state_class,
                import_freshness_class: session.import_freshness_class,
                local_confirmation_ref: finding.local_confirmation_ref.clone(),
                read_only: finding.read_only,
                export_safe_summary: finding.export_safe_summary.clone(),
            })
            .collect::<Vec<_>>();
        Self {
            record_kind: CLI_PROJECTION_RECORD_KIND.to_owned(),
            scanner_import_schema_version: SCANNER_IMPORT_ALPHA_SCHEMA_VERSION,
            projection_id: format!("cli:scanner_import:{}", sanitize_ref(&session.import_id)),
            import_session_ref: session.import_id.clone(),
            source_format_class,
            import_freshness_class: session.import_freshness_class,
            compatibility_class: session.delta_packet.compatibility_class,
            delta_counts: session.delta_packet.delta_counts.clone(),
            imported_finding_count: session.findings.len(),
            locally_confirmed_count: session
                .findings
                .iter()
                .filter(|finding| finding.truth_class == ScannerFindingTruthClass::LocallyConfirmed)
                .count(),
            read_only_count: session.findings.iter().filter(|finding| finding.read_only).count(),
            exact_delta_claim_blocked: session.blocks_exact_delta_claims(),
            raw_payload_refs: raw_payload_refs_for_policy(
                &session.raw_payload_refs,
                session.raw_payload_backlink_policy,
            ),
            raw_payload_backlink_policy: session.raw_payload_backlink_policy,
            rows,
            redaction_class: session.redaction_class,
            export_safe_summary:
                "CLI projection preserves scanner-import labels, delta state, and raw-payload backlink policy without raw source."
                    .into(),
        }
    }
}

/// One CLI/headless row for an imported scanner finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerImportCliRow {
    /// Stable imported finding id.
    pub finding_id: String,
    /// Diagnostic bus id.
    pub diagnostic_id: String,
    /// Rule id ref.
    pub rule_id_ref: String,
    /// Normalized severity class.
    pub severity_class: DiagnosticSeverityClass,
    /// Imported-vs-live truth class.
    pub truth_class: ScannerFindingTruthClass,
    /// Fidelity state for the finding.
    pub fidelity_state_class: ScannerFindingFidelityClass,
    /// Delta state for the finding.
    pub delta_state_class: ScannerFindingDeltaState,
    /// Anchor remap state.
    pub remap_state_class: DiagnosticAnchorRemapStateClass,
    /// Freshness state of the import that produced this row.
    pub import_freshness_class: ScannerImportFreshnessClass,
    /// Local confirmation ref, when present.
    pub local_confirmation_ref: Option<String>,
    /// True when the imported row is read-only.
    pub read_only: bool,
    /// Export-safe row summary.
    pub export_safe_summary: String,
}

/// Release evidence packet for imported scanner parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScannerImportReleasePacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub scanner_import_schema_version: u32,
    /// Stable release packet id.
    pub packet_id: String,
    /// Release or candidate scope ref.
    pub release_scope_ref: String,
    /// Source import session ref.
    pub import_session_ref: String,
    /// Diagnostic delta packet ref.
    pub diagnostic_delta_packet_ref: String,
    /// Suppression/baseline register ref.
    pub suppression_baseline_register_ref: String,
    /// Review packet ref.
    pub review_packet_ref: String,
    /// Source format class used by the import.
    pub source_format_class: ScannerSourceFormatClass,
    /// Imported payload freshness state.
    pub import_freshness_class: ScannerImportFreshnessClass,
    /// Delta compatibility class.
    pub compatibility_class: ScannerDeltaCompatibilityClass,
    /// Baseline family ref used for delta comparison.
    pub baseline_family_ref: String,
    /// Delta counts projected into the release packet.
    pub delta_counts: ScannerDeltaCounts,
    /// Count of active release-visible suppressions and waivers.
    pub active_suppression_or_waiver_count: usize,
    /// Count of release-visible baseline debt entries.
    pub release_visible_baseline_count: usize,
    /// Count of imported findings.
    pub imported_finding_count: usize,
    /// Count of locally confirmed findings.
    pub locally_confirmed_count: usize,
    /// True when imported findings remain read-only.
    pub imported_findings_read_only: bool,
    /// True when exact delta claims are blocked.
    pub exact_delta_claim_blocked: bool,
    /// Raw-payload backlink policy for release evidence.
    pub raw_payload_backlink_policy: ScannerRawPayloadBacklinkPolicy,
    /// Raw-payload refs retained according to policy.
    pub raw_payload_refs: Vec<String>,
    /// Release-visible parity note.
    pub parity_note: String,
    /// Redaction posture for release evidence.
    pub redaction_class: RedactionClass,
    /// Export-safe packet summary.
    pub export_safe_summary: String,
}

impl ScannerImportReleasePacket {
    /// Builds a release evidence packet from a normalized scanner import session.
    pub fn from_session(
        session: &ScannerImportSessionAlpha,
        release_scope_ref: impl Into<String>,
    ) -> Self {
        let active_suppression_or_waiver_count = session
            .suppression_baseline_register
            .suppression_entries
            .iter()
            .filter(|entry| entry.release_visible)
            .count();
        let release_visible_baseline_count = session
            .suppression_baseline_register
            .baseline_entries
            .iter()
            .filter(|entry| entry.release_visible)
            .count();
        Self {
            record_kind: RELEASE_PACKET_RECORD_KIND.to_owned(),
            scanner_import_schema_version: SCANNER_IMPORT_ALPHA_SCHEMA_VERSION,
            packet_id: format!("release:scanner_import:{}", sanitize_ref(&session.import_id)),
            release_scope_ref: release_scope_ref.into(),
            import_session_ref: session.import_id.clone(),
            diagnostic_delta_packet_ref: session.delta_packet.delta_packet_id.clone(),
            suppression_baseline_register_ref: session
                .suppression_baseline_register
                .register_id
                .clone(),
            review_packet_ref: session.review_packet.packet_id.clone(),
            source_format_class: ScannerSourceFormatClass::from_media_type(&session.media_type),
            import_freshness_class: session.import_freshness_class,
            compatibility_class: session.delta_packet.compatibility_class,
            baseline_family_ref: session.delta_packet.baseline_family_ref.clone(),
            delta_counts: session.delta_packet.delta_counts.clone(),
            active_suppression_or_waiver_count,
            release_visible_baseline_count,
            imported_finding_count: session.findings.len(),
            locally_confirmed_count: session
                .findings
                .iter()
                .filter(|finding| finding.truth_class == ScannerFindingTruthClass::LocallyConfirmed)
                .count(),
            imported_findings_read_only: session.findings.iter().all(|finding| finding.read_only),
            exact_delta_claim_blocked: session.blocks_exact_delta_claims(),
            raw_payload_backlink_policy: session.raw_payload_backlink_policy,
            raw_payload_refs: raw_payload_refs_for_policy(
                &session.raw_payload_refs,
                session.raw_payload_backlink_policy,
            ),
            parity_note: release_parity_note(session),
            redaction_class: session.redaction_class,
            export_safe_summary:
                "Release packet preserves scanner tool/version, baseline delta state, active debt, imported labels, and raw-payload backlink policy."
                    .into(),
        }
    }
}

/// Normalizes a structured scanner JSON payload into an imported session.
///
/// The structured shape uses the same run/result concepts as SARIF but is
/// intentionally smaller so scanners without native SARIF output can still
/// flow through the same delta, review, CLI, support, and release packets.
///
/// # Errors
///
/// Returns [`ScannerImportError`] when the request is missing required opaque
/// refs, the payload is invalid JSON, or the structured payload has no runs.
pub fn materialize_structured_scanner_import_session(
    request: ScannerImportRequest,
    structured_payload: &str,
) -> Result<ScannerImportSessionAlpha, ScannerImportError> {
    let structured: StructuredScannerLog = serde_json::from_str(structured_payload)
        .map_err(|err| ScannerImportError::InvalidJson(err.to_string()))?;
    if structured.runs.is_empty() {
        return Err(ScannerImportError::NoRuns);
    }
    let sarif_payload = structured_scanner_to_sarif_json(structured)?;
    materialize_sarif_import_session(request, &sarif_payload)
}

/// Normalizes a SARIF-shaped scanner payload into an imported session.
///
/// # Errors
///
/// Returns [`ScannerImportError`] when the request is missing required opaque
/// refs, the payload is not valid JSON, or the payload contains no runs.
pub fn materialize_sarif_import_session(
    request: ScannerImportRequest,
    sarif_payload: &str,
) -> Result<ScannerImportSessionAlpha, ScannerImportError> {
    if request.raw_payload_ref.trim().is_empty() {
        return Err(ScannerImportError::MissingRawPayloadRef);
    }
    if request.source_artifact_ref.trim().is_empty() {
        return Err(ScannerImportError::MissingSourceArtifactRef);
    }

    let sarif: SarifLog = serde_json::from_str(sarif_payload)
        .map_err(|err| ScannerImportError::InvalidJson(err.to_string()))?;
    if sarif.runs.is_empty() {
        return Err(ScannerImportError::NoRuns);
    }

    let baseline_lookup = request
        .baseline_entries
        .iter()
        .map(|entry| {
            (
                FindingMatchKey::from_rule_anchor(
                    &entry.rule_id_ref,
                    &entry.anchor_fingerprint_ref,
                ),
                entry,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let suppression_lookup = request
        .suppression_entries
        .iter()
        .map(|entry| {
            (
                FindingMatchKey::from_rule_anchor(
                    &entry.rule_id_ref,
                    &entry.anchor_fingerprint_ref,
                ),
                entry,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let confirmation_lookup = request
        .local_confirmations
        .iter()
        .map(|entry| {
            (
                FindingMatchKey::from_rule_anchor(
                    &entry.rule_id_ref,
                    &entry.anchor_fingerprint_ref,
                ),
                entry,
            )
        })
        .collect::<BTreeMap<_, _>>();

    let mut run_descriptors = Vec::new();
    let mut findings = Vec::new();
    let mut deltas = Vec::new();
    let mut seen_baseline_keys = BTreeSet::new();

    for (run_index, run) in sarif.runs.iter().enumerate() {
        let run_id = run
            .automation_details
            .as_ref()
            .and_then(|details| details.id.as_ref())
            .cloned()
            .unwrap_or_else(|| {
                format!(
                    "run:scanner_import:{}:{run_index}",
                    sanitize_ref(&request.import_id)
                )
            });
        let tool_name = run
            .tool
            .driver
            .name
            .clone()
            .unwrap_or_else(|| "scanner".into());
        let tool_version = run
            .tool
            .driver
            .semantic_version
            .clone()
            .or_else(|| run.tool.driver.version.clone())
            .unwrap_or_else(|| "unknown".into());
        let tool_id = format!("tool:scanner:{}", sanitize_ref(&tool_name));

        let mut run_mapping_quality = ScannerMappingQualityClass::ExactStatic;
        for (result_index, result) in run.results.iter().enumerate() {
            let rule_id_ref = result.rule_id.clone().unwrap_or_else(|| {
                format!("rule:scanner:unknown:{}", sanitize_ref(&request.import_id))
            });
            let location = result.primary_location();
            let anchor_fingerprint_ref = result
                .anchor_fingerprint_ref()
                .or_else(|| location.map(|loc| opaque_anchor_fingerprint(&rule_id_ref, loc)));
            let current_anchor_ref = location.map(opaque_current_anchor_ref);
            let artifact_ref = location.map(opaque_artifact_ref);
            let declared_remap_state = result.declared_remap_state();
            let mapping_quality_class = if current_anchor_ref.is_none() {
                ScannerMappingQualityClass::Unmapped
            } else if declared_remap_state == Some(DiagnosticAnchorRemapStateClass::Contextual) {
                ScannerMappingQualityClass::Contextual
            } else {
                ScannerMappingQualityClass::ExactStatic
            };
            if mapping_quality_class != ScannerMappingQualityClass::ExactStatic {
                run_mapping_quality = ScannerMappingQualityClass::Contextual;
            }
            let key = FindingMatchKey::from_rule_anchor(&rule_id_ref, &anchor_fingerprint_ref);
            let baseline_entry = baseline_lookup.get(&key).copied();
            if baseline_entry.is_some() {
                seen_baseline_keys.insert(key.clone());
            }
            let suppression_entry = suppression_lookup.get(&key).copied();
            let rule_family_mapping = request
                .rule_family_mappings
                .iter()
                .find(|mapping| rule_id_ref.starts_with(&mapping.imported_rule_id_prefix));
            let local_confirmation = confirmation_lookup.get(&key).copied();
            let local_confirmation_state_class = if local_confirmation.is_some() {
                ScannerLocalConfirmationStateClass::Confirmed
            } else if current_anchor_ref.is_none() {
                ScannerLocalConfirmationStateClass::BlockedByUnmappedAnchor
            } else if rule_family_mapping.is_some() {
                ScannerLocalConfirmationStateClass::Available
            } else {
                ScannerLocalConfirmationStateClass::Unavailable
            };
            let truth_class = if local_confirmation.is_some() {
                ScannerFindingTruthClass::LocallyConfirmed
            } else {
                ScannerFindingTruthClass::ImportedOnly
            };
            let delta_state_class = delta_state_for(
                current_anchor_ref.as_ref(),
                baseline_entry,
                suppression_entry,
            );
            let fidelity_state_class = fidelity_state_for(
                mapping_quality_class,
                truth_class,
                request.import_freshness_class,
                request.raw_payload_backlink_policy,
                request.rule_pack.baseline_family_state_class,
            );
            let finding_id = format!(
                "finding:scanner_import:{}:{run_index}:{result_index}",
                sanitize_ref(&request.import_id)
            );
            let diagnostic_id = format!("diagnostic:scanner_import:{}", sanitize_ref(&finding_id));
            let anchor = ImportedScannerAnchor {
                anchor_family_id: format!(
                    "anchorfam:scanner:{}:{}",
                    sanitize_ref(&rule_id_ref),
                    anchor_fingerprint_ref
                        .as_deref()
                        .map(sanitize_ref)
                        .unwrap_or_else(|| "unmapped".into())
                ),
                current_anchor_ref,
                artifact_ref,
                anchor_fingerprint_ref: anchor_fingerprint_ref.clone(),
                remap_state_class: declared_remap_state.unwrap_or_else(|| {
                    if mapping_quality_class == ScannerMappingQualityClass::Unmapped {
                        DiagnosticAnchorRemapStateClass::Unmapped
                    } else {
                        DiagnosticAnchorRemapStateClass::ImportedStatic
                    }
                }),
                mapping_quality_class,
                summary: if mapping_quality_class == ScannerMappingQualityClass::Unmapped {
                    "Imported scanner result has no admitted editor anchor.".into()
                } else {
                    "Imported scanner result has a static imported anchor that requires disclosure."
                        .into()
                },
            };
            let evidence_refs = evidence_refs_for(
                &request.raw_payload_ref,
                baseline_entry,
                suppression_entry,
                local_confirmation,
            );
            let finding = ImportedScannerFinding {
                finding_id: finding_id.clone(),
                diagnostic_id,
                run_id: run_id.clone(),
                rule_id_ref: rule_id_ref.clone(),
                rule_family_ref: rule_family_mapping.map(|mapping| mapping.rule_family_ref.clone()),
                severity_class: severity_from_sarif(result.level.as_deref()),
                category_refs: categories_for_result(&request, result),
                truth_class,
                fidelity_state_class,
                delta_state_class,
                anchor,
                completeness_class: if mapping_quality_class == ScannerMappingQualityClass::Unmapped
                {
                    RouterCompletenessClass::PartialForClaimedScope
                } else {
                    RouterCompletenessClass::CompleteForClaimedScope
                },
                local_confirmation_state_class,
                local_confirmation_action_ref: rule_family_mapping
                    .map(|mapping| mapping.confirmation_action_ref.clone()),
                local_confirmation_ref: local_confirmation
                    .map(|confirmation| confirmation.local_confirmation_ref.clone()),
                read_only: true,
                raw_payload_ref: request.raw_payload_ref.clone(),
                evidence_refs: evidence_refs.clone(),
                redaction_class: RedactionClass::MetadataSafeDefault,
                export_safe_summary: finding_summary(delta_state_class, truth_class),
            };
            deltas.push(ScannerFindingDelta {
                delta_id: format!("delta:scanner_import:{}", sanitize_ref(&finding_id)),
                delta_state_class,
                current_finding_ref: Some(finding_id.clone()),
                baseline_ref: baseline_entry.map(|entry| entry.baseline_ref.clone()),
                suppression_ref: suppression_entry.map(|entry| entry.suppression_id.clone()),
                local_confirmation_ref: local_confirmation
                    .map(|confirmation| confirmation.local_confirmation_ref.clone()),
                compatibility_note: compatibility_note_for(delta_state_class),
                supporting_evidence_refs: evidence_refs,
                export_safe_summary: delta_summary(delta_state_class),
            });
            findings.push(finding);
        }

        run_descriptors.push(ScannerRunDescriptor {
            run_id,
            tool_id,
            tool_name,
            tool_version,
            adapter_id: "adapter:sarif_import_alpha".into(),
            adapter_version: "0.1.0".into(),
            provider_ref: run
                .automation_details
                .as_ref()
                .and_then(|details| details.guid.as_ref())
                .map(|guid| format!("provider:scanner:{}", sanitize_ref(guid))),
            rule_pack_ref: request.rule_pack.rule_pack_ref.clone(),
            rule_pack_version: request.rule_pack.rule_pack_version.clone(),
            target_scope: request.target_scope.clone(),
            revision_binding: request.revision_binding.clone(),
            media_type: request.media_type.clone(),
            source_format_class: ScannerSourceFormatClass::from_media_type(&request.media_type),
            import_freshness_class: request.import_freshness_class,
            raw_payload_backlink_policy: request.raw_payload_backlink_policy,
            mapping_quality_class: run_mapping_quality,
            baseline_family_ref: request.rule_pack.baseline_family_ref.clone(),
            raw_payload_refs: vec![request.raw_payload_ref.clone()],
            category_refs: request.scanner_category_refs.clone(),
            result_count: run.results.len(),
            scan_completed_at: request.imported_at.clone(),
            export_safe_summary: "Scanner run descriptor preserves tool, target, revision, baseline, and raw-payload refs.".into(),
        });
    }

    for baseline_entry in &request.baseline_entries {
        let key = FindingMatchKey::from_rule_anchor(
            &baseline_entry.rule_id_ref,
            &baseline_entry.anchor_fingerprint_ref,
        );
        if !seen_baseline_keys.contains(&key) {
            deltas.push(ScannerFindingDelta {
                delta_id: format!(
                    "delta:scanner_import:resolved:{}",
                    sanitize_ref(&baseline_entry.baseline_entry_id)
                ),
                delta_state_class: ScannerFindingDeltaState::Resolved,
                current_finding_ref: None,
                baseline_ref: Some(baseline_entry.baseline_ref.clone()),
                suppression_ref: None,
                local_confirmation_ref: None,
                compatibility_note: compatibility_note_for(ScannerFindingDeltaState::Resolved),
                supporting_evidence_refs: vec![baseline_entry.baseline_ref.clone()],
                export_safe_summary: delta_summary(ScannerFindingDeltaState::Resolved),
            });
        }
    }

    let delta_counts = ScannerDeltaCounts::from_deltas(&deltas);
    let compatibility_class = compatibility_class_for(&request.rule_pack, &delta_counts);
    let release_visible_debt_count = request
        .baseline_entries
        .iter()
        .filter(|entry| entry.release_visible)
        .count()
        + request
            .suppression_entries
            .iter()
            .filter(|entry| entry.release_visible)
            .count();
    let register = SuppressionBaselineRegisterAlpha {
        record_kind: SUPPRESSION_BASELINE_REGISTER_RECORD_KIND.into(),
        scanner_import_schema_version: SCANNER_IMPORT_ALPHA_SCHEMA_VERSION,
        register_id: format!(
            "suppression_baseline:scanner_import:{}",
            sanitize_ref(&request.import_id)
        ),
        import_session_ref: request.import_id.clone(),
        baseline_family_ref: request.rule_pack.baseline_family_ref.clone(),
        register_version: 1,
        baseline_entries: request.baseline_entries.clone(),
        suppression_entries: request.suppression_entries.clone(),
        release_visible_debt_count,
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe_summary:
            "Suppression and baseline debt remains versioned, owned, and release-visible.".into(),
    };
    let delta_packet = ScannerDeltaPacketAlpha {
        record_kind: DIAGNOSTIC_DELTA_RECORD_KIND.into(),
        diagnostic_delta_schema_version: SCANNER_IMPORT_ALPHA_SCHEMA_VERSION,
        delta_packet_id: format!("diagnostic_delta:scanner_import:{}", sanitize_ref(&request.import_id)),
        import_session_ref: request.import_id.clone(),
        baseline_family_ref: request.rule_pack.baseline_family_ref.clone(),
        compatibility_class,
        delta_counts,
        finding_deltas: deltas,
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe_summary: "Diagnostic delta packet compares imported, baseline, suppression, waiver, and unmapped states without provider-specific dashboards.".into(),
    };
    let local_confirmation_actions = local_confirmation_actions(&request.rule_family_mappings);
    let review_packet = DiagnosticReviewPacketAlpha {
        record_kind: DIAGNOSTIC_REVIEW_PACKET_RECORD_KIND.into(),
        scanner_import_schema_version: SCANNER_IMPORT_ALPHA_SCHEMA_VERSION,
        packet_id: format!("review_packet:scanner_import:{}", sanitize_ref(&request.import_id)),
        change_scope_ref: request.target_scope.scope_ref.clone(),
        import_session_ref: request.import_id.clone(),
        diagnostic_delta_packet_ref: delta_packet.delta_packet_id.clone(),
        suppression_baseline_register_ref: register.register_id.clone(),
        included_finding_ids: findings.iter().map(|finding| finding.finding_id.clone()).collect(),
        finding_rows: review_finding_rows(
            &findings,
            request.import_freshness_class,
            request.raw_payload_backlink_policy,
        ),
        quality_action_refs: Vec::new(),
        local_confirmation_actions,
        imported_finding_count: findings.len(),
        locally_confirmed_finding_count: findings
            .iter()
            .filter(|finding| finding.truth_class == ScannerFindingTruthClass::LocallyConfirmed)
            .count(),
        release_visible_debt_count,
        delta_counts: delta_packet.delta_counts.clone(),
        raw_payload_backlink_policy: request.raw_payload_backlink_policy,
        profile_drift_note: "Imported scanner evidence stays read-only unless a compatible local confirmation or separate quality action is present.".into(),
        redaction_class: RedactionClass::MetadataSafeDefault,
        export_safe_summary: "Review packet preserves imported/live labels, confirmation actions, and release-visible debt.".into(),
    };

    Ok(ScannerImportSessionAlpha {
        record_kind: SCANNER_IMPORT_SESSION_RECORD_KIND.into(),
        scanner_import_schema_version: SCANNER_IMPORT_ALPHA_SCHEMA_VERSION,
        import_id: request.import_id,
        workspace_id: request.workspace_id,
        collection_id: request.collection_id,
        media_type: request.media_type,
        source_artifact_ref: request.source_artifact_ref,
        raw_payload_refs: vec![request.raw_payload_ref],
        import_freshness_class: request.import_freshness_class,
        raw_payload_backlink_policy: request.raw_payload_backlink_policy,
        target_scope: request.target_scope,
        revision_binding: request.revision_binding,
        run_descriptors,
        findings,
        delta_packet,
        suppression_baseline_register: register,
        review_packet,
        redaction_class: RedactionClass::MetadataSafeDefault,
        imported_at: request.imported_at,
        export_safe_summary: "Imported scanner session preserves run lineage, raw-payload refs, delta state, local-confirmation posture, and release-visible debt.".into(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FindingMatchKey {
    rule_id_ref: String,
    anchor_fingerprint_ref: Option<String>,
}

impl FindingMatchKey {
    fn from_rule_anchor(rule_id_ref: &str, anchor_fingerprint_ref: &Option<String>) -> Self {
        Self {
            rule_id_ref: rule_id_ref.to_owned(),
            anchor_fingerprint_ref: anchor_fingerprint_ref.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct StructuredScannerLog {
    #[serde(default)]
    runs: Vec<StructuredScannerRun>,
}

#[derive(Debug, Deserialize)]
struct StructuredScannerRun {
    run_id: String,
    tool_name: String,
    tool_version: String,
    #[serde(default)]
    provider_ref: Option<String>,
    #[serde(default)]
    results: Vec<StructuredScannerResult>,
}

#[derive(Debug, Deserialize)]
struct StructuredScannerResult {
    rule_id_ref: String,
    severity_class: DiagnosticSeverityClass,
    #[serde(default)]
    category_refs: Vec<String>,
    #[serde(default)]
    artifact_uri: Option<String>,
    #[serde(default)]
    start_line: Option<u32>,
    #[serde(default)]
    start_column: Option<u32>,
    #[serde(default)]
    anchor_fingerprint_key: Option<String>,
    #[serde(default)]
    remap_state_class: Option<DiagnosticAnchorRemapStateClass>,
    #[serde(default)]
    message_ref: Option<String>,
}

fn structured_scanner_to_sarif_json(
    structured: StructuredScannerLog,
) -> Result<String, ScannerImportError> {
    let runs = structured
        .runs
        .into_iter()
        .map(|run| {
            let results = run
                .results
                .into_iter()
                .map(|result| {
                    let locations = if let Some(uri) = result.artifact_uri {
                        vec![serde_json::json!({
                            "physicalLocation": {
                                "artifactLocation": { "uri": uri },
                                "region": {
                                    "startLine": result.start_line.unwrap_or(1),
                                    "startColumn": result.start_column.unwrap_or(1)
                                }
                            }
                        })]
                    } else {
                        Vec::new()
                    };
                    let mut properties = serde_json::Map::new();
                    if let Some(anchor) = result.anchor_fingerprint_key {
                        properties.insert(
                            "aurelineAnchorFingerprintRef".to_owned(),
                            serde_json::Value::String(anchor),
                        );
                    }
                    if let Some(category) = result.category_refs.first() {
                        properties.insert(
                            "categoryRef".to_owned(),
                            serde_json::Value::String(category.clone()),
                        );
                    }
                    if let Some(remap_state) = result.remap_state_class {
                        properties.insert(
                            "aurelineRemapState".to_owned(),
                            serde_json::Value::String(remap_state_to_property(remap_state)),
                        );
                    }
                    serde_json::json!({
                        "ruleId": result.rule_id_ref,
                        "level": severity_to_sarif_level(result.severity_class),
                        "message": {
                            "text": result.message_ref.unwrap_or_else(|| "Structured scanner finding body withheld from normalized exports.".to_owned())
                        },
                        "locations": locations,
                        "properties": properties
                    })
                })
                .collect::<Vec<_>>();
            serde_json::json!({
                "automationDetails": {
                    "id": run.run_id,
                    "guid": run.provider_ref
                },
                "tool": {
                    "driver": {
                        "name": run.tool_name,
                        "semanticVersion": run.tool_version
                    }
                },
                "results": results
            })
        })
        .collect::<Vec<_>>();
    serde_json::to_string(&serde_json::json!({
        "version": "2.1.0",
        "runs": runs
    }))
    .map_err(|err| ScannerImportError::InvalidStructuredPayload(err.to_string()))
}

#[derive(Debug, Deserialize)]
struct SarifLog {
    #[allow(dead_code)]
    version: Option<String>,
    #[serde(default)]
    runs: Vec<SarifRun>,
}

#[derive(Debug, Deserialize)]
struct SarifRun {
    tool: SarifTool,
    #[serde(default)]
    results: Vec<SarifResult>,
    #[serde(rename = "automationDetails")]
    automation_details: Option<SarifAutomationDetails>,
}

#[derive(Debug, Deserialize)]
struct SarifTool {
    driver: SarifDriver,
}

#[derive(Debug, Deserialize)]
struct SarifDriver {
    name: Option<String>,
    version: Option<String>,
    #[serde(rename = "semanticVersion")]
    semantic_version: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SarifAutomationDetails {
    id: Option<String>,
    guid: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SarifResult {
    #[serde(rename = "ruleId")]
    rule_id: Option<String>,
    level: Option<String>,
    #[allow(dead_code)]
    message: Option<SarifMessage>,
    #[serde(default)]
    locations: Vec<SarifLocation>,
    #[serde(rename = "partialFingerprints", default)]
    partial_fingerprints: BTreeMap<String, String>,
    #[serde(default)]
    properties: BTreeMap<String, serde_json::Value>,
}

impl SarifResult {
    fn primary_location(&self) -> Option<&SarifPhysicalLocation> {
        self.locations
            .first()
            .and_then(|location| location.physical_location.as_ref())
    }

    fn anchor_fingerprint_ref(&self) -> Option<String> {
        property_string(&self.properties, "aurelineAnchorFingerprintRef")
            .or_else(|| self.partial_fingerprints.values().next().cloned())
            .map(|value| format!("anchor_fingerprint:{}", sanitize_ref(&value)))
    }

    fn declared_remap_state(&self) -> Option<DiagnosticAnchorRemapStateClass> {
        match property_string(&self.properties, "aurelineRemapState").as_deref() {
            Some("exact") => Some(DiagnosticAnchorRemapStateClass::Exact),
            Some("contextual") => Some(DiagnosticAnchorRemapStateClass::Contextual),
            Some("stale") => Some(DiagnosticAnchorRemapStateClass::Stale),
            Some("unmapped") => Some(DiagnosticAnchorRemapStateClass::Unmapped),
            Some("imported_static") => Some(DiagnosticAnchorRemapStateClass::ImportedStatic),
            Some("not_applicable") => Some(DiagnosticAnchorRemapStateClass::NotApplicable),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct SarifMessage {
    #[allow(dead_code)]
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SarifLocation {
    #[serde(rename = "physicalLocation")]
    physical_location: Option<SarifPhysicalLocation>,
}

#[derive(Debug, Deserialize)]
struct SarifPhysicalLocation {
    #[serde(rename = "artifactLocation")]
    artifact_location: Option<SarifArtifactLocation>,
    region: Option<SarifRegion>,
}

#[derive(Debug, Deserialize)]
struct SarifArtifactLocation {
    uri: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SarifRegion {
    #[serde(rename = "startLine")]
    start_line: Option<u32>,
    #[serde(rename = "startColumn")]
    start_column: Option<u32>,
}

impl From<ScannerTargetScopeClass> for RouterScopeClaimClass {
    fn from(scope: ScannerTargetScopeClass) -> Self {
        match scope {
            ScannerTargetScopeClass::CurrentFile => Self::SingleFile,
            ScannerTargetScopeClass::CurrentRoot
            | ScannerTargetScopeClass::SelectedWorkset
            | ScannerTargetScopeClass::ChangedFiles
            | ScannerTargetScopeClass::ReviewDiff => Self::LoadedSlice,
            ScannerTargetScopeClass::Workspace => Self::WholeWorkspace,
            ScannerTargetScopeClass::BaselineFamily
            | ScannerTargetScopeClass::ReleaseCandidate
            | ScannerTargetScopeClass::ProviderProject => Self::TargetGraph,
            ScannerTargetScopeClass::SupportExport
            | ScannerTargetScopeClass::TargetScopeUnknownRequiresReview => Self::Unavailable,
        }
    }
}

impl ScannerTargetScopeClass {
    fn scope_limits(self) -> Vec<ScopeLimitClass> {
        match self {
            Self::ChangedFiles | Self::ReviewDiff => vec![ScopeLimitClass::DiffOrReviewSliceOnly],
            Self::SelectedWorkset => vec![ScopeLimitClass::ActiveWorksetOnly],
            Self::CurrentFile => vec![ScopeLimitClass::SingleFileOnly],
            _ => Vec::new(),
        }
    }
}

fn property_string(properties: &BTreeMap<String, serde_json::Value>, key: &str) -> Option<String> {
    properties
        .get(key)
        .and_then(serde_json::Value::as_str)
        .map(ToOwned::to_owned)
}

fn severity_from_sarif(level: Option<&str>) -> DiagnosticSeverityClass {
    match level {
        Some("error") => DiagnosticSeverityClass::Error,
        Some("warning") => DiagnosticSeverityClass::Warning,
        Some("note") => DiagnosticSeverityClass::Notice,
        Some("none") => DiagnosticSeverityClass::Hint,
        _ => DiagnosticSeverityClass::Warning,
    }
}

fn severity_to_sarif_level(severity: DiagnosticSeverityClass) -> &'static str {
    match severity {
        DiagnosticSeverityClass::Error => "error",
        DiagnosticSeverityClass::Warning => "warning",
        DiagnosticSeverityClass::Notice => "note",
        DiagnosticSeverityClass::Hint => "none",
    }
}

fn remap_state_to_property(remap_state: DiagnosticAnchorRemapStateClass) -> String {
    match remap_state {
        DiagnosticAnchorRemapStateClass::Exact => "exact",
        DiagnosticAnchorRemapStateClass::Contextual => "contextual",
        DiagnosticAnchorRemapStateClass::Stale => "stale",
        DiagnosticAnchorRemapStateClass::Unmapped => "unmapped",
        DiagnosticAnchorRemapStateClass::ImportedStatic => "imported_static",
        DiagnosticAnchorRemapStateClass::NotApplicable => "not_applicable",
    }
    .to_owned()
}

fn categories_for_result(request: &ScannerImportRequest, result: &SarifResult) -> Vec<String> {
    let mut categories = request.scanner_category_refs.clone();
    if let Some(category) = property_string(&result.properties, "categoryRef") {
        categories.push(category);
    }
    categories.sort();
    categories.dedup();
    categories
}

fn delta_state_for(
    current_anchor_ref: Option<&String>,
    baseline_entry: Option<&ScannerBaselineEntry>,
    suppression_entry: Option<&ScannerSuppressionEntry>,
) -> ScannerFindingDeltaState {
    if current_anchor_ref.is_none() {
        return ScannerFindingDeltaState::Unmapped;
    }
    if let Some(entry) = suppression_entry {
        return match entry.debt_state {
            ScannerDebtState::Suppressed => ScannerFindingDeltaState::Suppressed,
            ScannerDebtState::Waived => ScannerFindingDeltaState::Waived,
        };
    }
    if baseline_entry.is_some() {
        ScannerFindingDeltaState::Persisting
    } else {
        ScannerFindingDeltaState::New
    }
}

fn evidence_refs_for(
    raw_payload_ref: &str,
    baseline_entry: Option<&ScannerBaselineEntry>,
    suppression_entry: Option<&ScannerSuppressionEntry>,
    local_confirmation: Option<&ScannerLocalConfirmation>,
) -> Vec<String> {
    let mut refs = vec![raw_payload_ref.to_owned()];
    if let Some(entry) = baseline_entry {
        refs.push(entry.baseline_ref.clone());
    }
    if let Some(entry) = suppression_entry {
        refs.push(entry.suppression_id.clone());
        refs.extend(entry.evidence_refs.clone());
    }
    if let Some(confirmation) = local_confirmation {
        refs.push(confirmation.local_confirmation_ref.clone());
        refs.push(confirmation.local_run_ref.clone());
    }
    refs.sort();
    refs.dedup();
    refs
}

fn local_confirmation_actions(
    mappings: &[ScannerRuleFamilyMapping],
) -> Vec<ScannerLocalConfirmationAction> {
    mappings
        .iter()
        .map(|mapping| ScannerLocalConfirmationAction {
            action_ref: mapping.confirmation_action_ref.clone(),
            rule_family_ref: mapping.rule_family_ref.clone(),
            local_provider_ref: mapping.local_provider_ref.clone(),
            local_rule_ref: mapping.local_rule_ref.clone(),
            required_before_mutation: true,
            summary: mapping.summary.clone(),
        })
        .collect()
}

fn review_finding_rows(
    findings: &[ImportedScannerFinding],
    import_freshness_class: ScannerImportFreshnessClass,
    raw_payload_backlink_policy: ScannerRawPayloadBacklinkPolicy,
) -> Vec<ScannerReviewFindingRow> {
    findings
        .iter()
        .map(|finding| ScannerReviewFindingRow {
            finding_id: finding.finding_id.clone(),
            diagnostic_id: finding.diagnostic_id.clone(),
            rule_id_ref: finding.rule_id_ref.clone(),
            truth_class: finding.truth_class,
            fidelity_state_class: finding.fidelity_state_class,
            delta_state_class: finding.delta_state_class,
            remap_state_class: finding.anchor.remap_state_class,
            import_freshness_class,
            read_only: finding.read_only,
            raw_payload_backlink_policy,
            export_safe_summary: finding.export_safe_summary.clone(),
        })
        .collect()
}

fn fidelity_state_for(
    mapping_quality_class: ScannerMappingQualityClass,
    truth_class: ScannerFindingTruthClass,
    import_freshness_class: ScannerImportFreshnessClass,
    raw_payload_backlink_policy: ScannerRawPayloadBacklinkPolicy,
    baseline_family_state_class: ScannerBaselineFamilyStateClass,
) -> ScannerFindingFidelityClass {
    if raw_payload_backlink_policy.is_redacted() {
        return ScannerFindingFidelityClass::RedactedPayload;
    }
    if baseline_family_state_class.blocks_compatible_delta() {
        return ScannerFindingFidelityClass::BaselineMismatch;
    }
    if import_freshness_class.needs_confirmation() {
        return ScannerFindingFidelityClass::StaleImported;
    }
    if mapping_quality_class == ScannerMappingQualityClass::Unmapped {
        return ScannerFindingFidelityClass::UnmappedAnchor;
    }
    if truth_class == ScannerFindingTruthClass::LocallyConfirmed {
        return ScannerFindingFidelityClass::LocallyConfirmed;
    }
    if mapping_quality_class == ScannerMappingQualityClass::Contextual {
        return ScannerFindingFidelityClass::RemappedContextual;
    }
    ScannerFindingFidelityClass::ImportedStatic
}

fn raw_payload_refs_for_policy(
    refs: &[String],
    policy: ScannerRawPayloadBacklinkPolicy,
) -> Vec<String> {
    if policy.exposes_backlink_refs() {
        refs.to_vec()
    } else {
        vec![raw_payload_ref_for_policy(
            "raw_payload:scanner_import",
            policy,
        )]
    }
}

fn evidence_refs_for_policy(
    refs: &[String],
    policy: ScannerRawPayloadBacklinkPolicy,
) -> Vec<String> {
    if policy.exposes_backlink_refs() {
        return refs.to_vec();
    }
    refs.iter()
        .map(|reference| {
            if reference.starts_with("raw_payload:") {
                raw_payload_ref_for_policy(reference, policy)
            } else {
                reference.clone()
            }
        })
        .collect()
}

fn raw_payload_ref_for_policy(
    raw_payload_ref: &str,
    policy: ScannerRawPayloadBacklinkPolicy,
) -> String {
    match policy {
        ScannerRawPayloadBacklinkPolicy::OpaqueRefsOnly
        | ScannerRawPayloadBacklinkPolicy::ExplicitOptInRestricted => raw_payload_ref.to_owned(),
        ScannerRawPayloadBacklinkPolicy::RedactedByPolicy => {
            "raw_payload:redacted_by_policy".into()
        }
        ScannerRawPayloadBacklinkPolicy::OmittedUnavailable => {
            "raw_payload:omitted_unavailable".into()
        }
    }
}

fn release_parity_note(session: &ScannerImportSessionAlpha) -> String {
    if session.blocks_exact_delta_claims() {
        format!(
            "Scanner import is {} with {} compatibility; release evidence must keep imported labels and cannot claim live local parity.",
            session.import_freshness_class.as_str(),
            session.delta_packet.compatibility_class.as_str()
        )
    } else if session
        .findings
        .iter()
        .any(|finding| finding.truth_class == ScannerFindingTruthClass::LocallyConfirmed)
    {
        "Compatible local confirmations are linked without replacing the imported evidence chain."
            .into()
    } else {
        "Imported scanner evidence is comparable for the declared baseline but remains read-only imported evidence."
            .into()
    }
}

fn compatibility_class_for(
    rule_pack: &ScannerRulePackBinding,
    delta_counts: &ScannerDeltaCounts,
) -> ScannerDeltaCompatibilityClass {
    match rule_pack.baseline_family_state_class {
        ScannerBaselineFamilyStateClass::Compatible => {
            if delta_counts.unmapped_count == 0 {
                ScannerDeltaCompatibilityClass::CompatibleExact
            } else {
                ScannerDeltaCompatibilityClass::BlockedAnchorMappingUncertain
            }
        }
        ScannerBaselineFamilyStateClass::CompatibleWithLocalConfirmation => {
            ScannerDeltaCompatibilityClass::CompatibleWithLocalConfirmation
        }
        ScannerBaselineFamilyStateClass::StaleButComparable => {
            ScannerDeltaCompatibilityClass::CompatibleWithLocalConfirmation
        }
        ScannerBaselineFamilyStateClass::IncompatibleRulePack => {
            ScannerDeltaCompatibilityClass::BlockedRulePackMismatch
        }
        ScannerBaselineFamilyStateClass::IncompatibleProfile => {
            ScannerDeltaCompatibilityClass::BlockedProfileOrToolMismatch
        }
        ScannerBaselineFamilyStateClass::IncompatibleMappingFamily => {
            ScannerDeltaCompatibilityClass::BlockedAnchorMappingUncertain
        }
        ScannerBaselineFamilyStateClass::CompatibilityUnknownRequiresReview => {
            ScannerDeltaCompatibilityClass::NotComparableUnknownRequiresReview
        }
    }
}

fn compatibility_note_for(delta_state: ScannerFindingDeltaState) -> String {
    match delta_state {
        ScannerFindingDeltaState::New => {
            "Finding is absent from the compatible baseline and remains imported read-only evidence."
        }
        ScannerFindingDeltaState::Resolved => {
            "Baseline finding is absent from the imported result set and is represented as resolved debt."
        }
        ScannerFindingDeltaState::Persisting => {
            "Finding matches a compatible baseline entry and remains visible as persisting debt."
        }
        ScannerFindingDeltaState::Suppressed => {
            "Finding is present but governed by a versioned suppression record."
        }
        ScannerFindingDeltaState::Waived => {
            "Finding is present but governed by a versioned waiver record."
        }
        ScannerFindingDeltaState::Unmapped => {
            "Finding lacks an admitted anchor, so comparison cannot claim new or persisting state."
        }
    }
    .into()
}

fn finding_summary(
    delta_state: ScannerFindingDeltaState,
    truth_class: ScannerFindingTruthClass,
) -> String {
    let truth = match truth_class {
        ScannerFindingTruthClass::ImportedOnly => "imported only",
        ScannerFindingTruthClass::LocallyConfirmed => "locally confirmed with imported lineage",
    };
    format!(
        "Imported scanner finding is {truth}, read-only, and classified as {:?}.",
        delta_state
    )
}

fn delta_summary(delta_state: ScannerFindingDeltaState) -> String {
    format!(
        "Delta row uses the {:?} state and cites supporting evidence refs.",
        delta_state
    )
}

fn opaque_anchor_fingerprint(rule_id_ref: &str, location: &SarifPhysicalLocation) -> String {
    let uri = location
        .artifact_location
        .as_ref()
        .and_then(|artifact| artifact.uri.as_deref())
        .unwrap_or("unknown_artifact");
    let line = location
        .region
        .as_ref()
        .and_then(|region| region.start_line)
        .unwrap_or(0);
    format!(
        "anchor_fingerprint:{}",
        stable_hash_hex(&format!("{rule_id_ref}:{uri}:{line}"))
    )
}

fn opaque_current_anchor_ref(location: &SarifPhysicalLocation) -> String {
    let uri = location
        .artifact_location
        .as_ref()
        .and_then(|artifact| artifact.uri.as_deref())
        .unwrap_or("unknown_artifact");
    let line = location
        .region
        .as_ref()
        .and_then(|region| region.start_line)
        .unwrap_or(0);
    let column = location
        .region
        .as_ref()
        .and_then(|region| region.start_column)
        .unwrap_or(0);
    format!(
        "anchor:scanner:{}",
        stable_hash_hex(&format!("{uri}:{line}:{column}"))
    )
}

fn opaque_artifact_ref(location: &SarifPhysicalLocation) -> String {
    let uri = location
        .artifact_location
        .as_ref()
        .and_then(|artifact| artifact.uri.as_deref())
        .unwrap_or("unknown_artifact");
    format!("artifact:scanner:{}", stable_hash_hex(uri))
}

fn stable_hash_hex(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn sanitize_ref(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}
