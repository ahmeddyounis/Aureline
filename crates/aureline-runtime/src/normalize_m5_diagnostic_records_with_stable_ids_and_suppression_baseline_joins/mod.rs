//! Delivery-grade normalization of M5 finding records onto one reopenable
//! diagnostic identity with stable ids and attached suppression / baseline joins.
//!
//! M5 widens the set of surfaces that mint or preserve findings — notebook cells,
//! framework packs, request / API tooling, data tooling, preview runtimes, package
//! lanes, the language-provider plane, the editor-structural guard, and imported
//! scanner snapshots. The freeze in
//! [`crate::freeze_the_m5_diagnostic_record_source_collection_snapshot_and_anchor_remap_matrix`]
//! bound each *surface* to its source / collection / remap / quality-session
//! contract; this module binds each *finding* to the single canonical
//! [`DiagnosticRecord`](crate::diagnostics::DiagnosticRecord) so editor markers,
//! Problems rows, review annotations, CLI / headless output, AI evidence, and
//! support export all reopen one normalized record instead of translating
//! display text or provider-native ids.
//!
//! A [`NormalizedDiagnosticRecordEntry`] reuses the v1 canonical
//! [`DiagnosticRecord`] directly — it does not mint a second diagnostic store —
//! and adds the three M5 record-level guarantees this delivery owns:
//!
//! 1. **Reopen without translation loss.** Each entry carries a
//!    [`DiagnosticReopenHandle`] for every required consumer surface; every handle
//!    cites the canonical [`DiagnosticRecord::diagnostic_id`] rather than a
//!    provider-native id and preserves the source / freshness / remap detail, so a
//!    record reopened from the editor, Problems, review, CLI, AI evidence, or
//!    support export resolves to the same finding.
//! 2. **Stable identity across refreshes and surface hops.** A
//!    [`DiagnosticStableIdentityFamily`] records the canonical id and anchor family
//!    plus the observations — initial emit, ordinary refresh, adapter refresh,
//!    surface hop, presentation change, re-export — that all resolved to the *same*
//!    id, so identity survives ordinary repaint and adapter refresh inside one
//!    compatible anchor/remap family instead of regenerating on every surface hop.
//! 3. **Suppression / baseline joins attached to the record.** A
//!    [`DiagnosticSuppressionJoin`] and [`DiagnosticBaselineJoin`] bind the
//!    governing suppression and baseline records to the diagnostic by its canonical
//!    id, and the join must be reflected on the record's own
//!    [`DiagnosticRecord::suppression_refs`] /
//!    [`DiagnosticRecord::baseline_refs`] rather than hidden in feature-local
//!    metadata.
//!
//! The set *auto-downgrades*: an entry that cannot prove its stable identity,
//! cannot reopen from a required surface, or lacks the normalized provenance a
//! claim needs carries an `effective_qualification` strictly below its claim, a
//! recorded downgrade trigger, and a precise degraded label — so a record claim
//! never outruns the evidence backing it.
//!
//! [`NormalizedDiagnosticRecordSetPacket::validate`] refuses an entry whose join
//! is detached from its record, whose identity family disagrees with the record's
//! id or anchor family, whose reopen handle resolves to a different id, that
//! flattens unlike sources, or that lets convenience presentation erase the
//! source kind, imported-versus-live class, freshness, or confidence from the
//! detail / export paths.
//!
//! Raw source bytes, raw provider payloads, raw scanner reports, credentials, and
//! raw artifact bodies never cross this boundary; the packet carries only typed
//! class tokens, booleans, opaque ids, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/quality/diagnostic-record.schema.json`](../../../../schemas/quality/diagnostic-record.schema.json).
//! The help doc is
//! [`docs/help/diagnostic-records.md`](../../../../docs/help/diagnostic-records.md).
//! The protected fixture directory is
//! [`fixtures/quality/m5/diagnostic-records/`](../../../../fixtures/quality/m5/diagnostic-records/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::diagnostics::{DiagnosticRecord, DiagnosticSurfaceClass};
use crate::freeze_the_m5_diagnostic_record_source_collection_snapshot_and_anchor_remap_matrix::M5DiagnosticSurface;
use crate::quality::{
    BaselineCompatibilityStateClass, QualityDebtReopenStateClass, QualityTargetScopeClass,
};

/// Stable record-kind tag carried by [`NormalizedDiagnosticRecordSetPacket`].
pub const M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_RECORD_KIND: &str =
    "m5_normalized_diagnostic_record_set";

/// Schema version for the M5 normalized diagnostic-record set.
pub const M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_SCHEMA_REF: &str =
    "schemas/quality/diagnostic-record.schema.json";

/// Repo-relative path of the help doc.
pub const M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_DOC_REF: &str = "docs/help/diagnostic-records.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_ARTIFACT_REF: &str =
    "artifacts/m5/diagnostics/diagnostic-record-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_SUMMARY_REF: &str =
    "artifacts/m5/diagnostics/diagnostic-record-proof/support_export.md";

/// Repo-relative path of the canonical v1 diagnostic-record plane schema this set
/// extends rather than replaces.
pub const CANONICAL_DIAGNOSTIC_RECORD_SCHEMA_REF: &str =
    "schemas/diagnostics/diagnostic_record.schema.json";

/// Consumer surfaces a normalized record must reopen from without
/// provider-specific translation loss.
///
/// These are the surfaces named by the acceptance criteria: a record opened from
/// any of them resolves to the same canonical diagnostic id. The v1
/// [`DiagnosticSurfaceClass::Output`] timeline is covered through the record's own
/// surface refs and is not a required *reopen* entry point.
pub const REQUIRED_REOPEN_SURFACES: [DiagnosticSurfaceClass; 6] = [
    DiagnosticSurfaceClass::Editor,
    DiagnosticSurfaceClass::Problems,
    DiagnosticSurfaceClass::Review,
    DiagnosticSurfaceClass::CliExplain,
    DiagnosticSurfaceClass::AiEvidence,
    DiagnosticSurfaceClass::SupportExport,
];

/// One stable, same-identity handle a consumer surface uses to reopen a record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticReopenHandle {
    /// Surface this handle reopens the record from.
    pub surface_class: DiagnosticSurfaceClass,
    /// Stable surface-local ref that cites the canonical diagnostic id.
    pub stable_surface_ref: String,
    /// Canonical diagnostic id this handle resolves back to.
    pub resolves_diagnostic_id: String,
    /// True when the handle cites the canonical id rather than a provider-native id.
    pub cites_canonical_id: bool,
    /// True when reopening from this surface preserves source / freshness / remap
    /// detail rather than collapsing it to display text.
    pub preserves_detail: bool,
}

impl DiagnosticReopenHandle {
    /// Whether this handle reopens the named record without translation loss.
    pub fn reopens(&self, diagnostic_id: &str) -> bool {
        self.cites_canonical_id
            && self.preserves_detail
            && self.resolves_diagnostic_id == diagnostic_id
            && !self.stable_surface_ref.trim().is_empty()
    }
}

/// Context in which a diagnostic identity was re-observed.
///
/// Used to prove a canonical id survives ordinary refresh, adapter refresh,
/// surface hops, and presentational changes instead of regenerating.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticIdentityContextClass {
    /// First emission of the finding.
    InitialEmit,
    /// Ordinary re-analysis or repaint of the same scope.
    OrdinaryRefresh,
    /// Producer adapter refreshed without a semantic change to the finding.
    AdapterRefresh,
    /// The finding was reopened from a different consumer surface.
    SurfaceHop,
    /// A presentational change (clustering, density, theme) without new evidence.
    PresentationChange,
    /// The finding was re-exported into a support or evidence bundle.
    ReExport,
}

impl DiagnosticIdentityContextClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InitialEmit => "initial_emit",
            Self::OrdinaryRefresh => "ordinary_refresh",
            Self::AdapterRefresh => "adapter_refresh",
            Self::SurfaceHop => "surface_hop",
            Self::PresentationChange => "presentation_change",
            Self::ReExport => "re_export",
        }
    }

    /// Whether this context is a refresh that must not reissue the id.
    pub const fn is_refresh(self) -> bool {
        matches!(self, Self::OrdinaryRefresh | Self::AdapterRefresh)
    }
}

/// One observation that a canonical identity resolved to the same id and anchor
/// family in a given context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticIdentityObservation {
    /// Context in which the identity was observed.
    pub context_class: DiagnosticIdentityContextClass,
    /// Diagnostic id observed in this context.
    pub observed_diagnostic_id: String,
    /// Anchor family id observed in this context.
    pub observed_anchor_family_id: String,
    /// Export-safe note about the observation.
    pub note: String,
}

/// Proof that a canonical diagnostic id is stable within one compatible
/// anchor/remap family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticStableIdentityFamily {
    /// Canonical diagnostic id; must equal the record's id.
    pub diagnostic_id: String,
    /// Anchor family shared by compatible remaps; must equal the record's family.
    pub anchor_family_id: String,
    /// Observations that all resolved to the same id and anchor family.
    pub observations: Vec<DiagnosticIdentityObservation>,
}

impl DiagnosticStableIdentityFamily {
    /// Whether every observation resolved to this family's id and anchor family.
    pub fn all_observations_stable(&self) -> bool {
        self.observations.iter().all(|observation| {
            observation.observed_diagnostic_id == self.diagnostic_id
                && observation.observed_anchor_family_id == self.anchor_family_id
        })
    }

    /// Whether the observations cover at least one refresh, one surface hop, and
    /// one presentational change, so stability is proven beyond a single emit.
    pub fn covers_refresh_surface_hop_and_presentation(&self) -> bool {
        let has_refresh = self
            .observations
            .iter()
            .any(|observation| observation.context_class.is_refresh());
        let has_surface_hop = self.observations.iter().any(|observation| {
            observation.context_class == DiagnosticIdentityContextClass::SurfaceHop
        });
        let has_presentation = self.observations.iter().any(|observation| {
            observation.context_class == DiagnosticIdentityContextClass::PresentationChange
        });
        has_refresh && has_surface_hop && has_presentation
    }

    /// Whether the family proves a durable, stable identity.
    pub fn is_proven(&self) -> bool {
        !self.diagnostic_id.trim().is_empty()
            && !self.anchor_family_id.trim().is_empty()
            && self.all_observations_stable()
            && self.covers_refresh_surface_hop_and_presentation()
    }
}

/// Typed join binding a governing suppression record to a normalized diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticSuppressionJoin {
    /// Stable join id.
    pub join_id: String,
    /// Canonical diagnostic id this join attaches to.
    pub diagnostic_id: String,
    /// Governing suppression record id.
    pub suppression_id: String,
    /// Suppression scope.
    pub scope_class: QualityTargetScopeClass,
    /// Reopen / debt state of the suppression at observation time.
    pub reopen_state_class: QualityDebtReopenStateClass,
    /// Release-visible debt flag carried through from the suppression record.
    pub release_visible: bool,
    /// True when this join is reflected on the record's `suppression_refs` rather
    /// than hidden in feature-local metadata.
    pub attached_to_record: bool,
    /// Export-safe summary.
    pub summary: String,
}

/// Typed join binding a governing baseline record to a normalized diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticBaselineJoin {
    /// Stable join id.
    pub join_id: String,
    /// Canonical diagnostic id this join attaches to.
    pub diagnostic_id: String,
    /// Governing baseline record id.
    pub baseline_id: String,
    /// Baseline compatibility state for this diagnostic's target / profile.
    pub compatibility_state_class: BaselineCompatibilityStateClass,
    /// True when the diagnostic is an accepted finding in the baseline.
    pub accepted_in_baseline: bool,
    /// True when this join is reflected on the record's `baseline_refs` rather than
    /// hidden in feature-local metadata.
    pub attached_to_record: bool,
    /// Export-safe summary.
    pub summary: String,
}

/// Headline qualification a normalized record entry may claim or hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NormalizedRecordQualificationClass {
    /// Held below preview until the record's normalized identity is established.
    Held,
    /// Claimed at preview maturity.
    Preview,
    /// Claimed at beta maturity.
    Beta,
    /// Claimed at stable maturity.
    Stable,
}

impl NormalizedRecordQualificationClass {
    /// Stable token recorded in the set.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Held => "held",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
        }
    }

    /// Whether this class carries a public claim above held.
    pub const fn is_claimed(self) -> bool {
        !matches!(self, Self::Held)
    }

    /// Monotonic rank used to compare claimed and effective qualifications.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Held => 0,
            Self::Preview => 1,
            Self::Beta => 2,
            Self::Stable => 3,
        }
    }
}

/// Trigger that fired an auto-downgrade on a normalized record entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NormalizedRecordDowngradeTrigger {
    /// The stable-identity family could not be proven.
    UnprovenStableIdentity,
    /// A required consumer surface cannot reopen the record.
    MissingReopenSurface,
    /// The record lacks the normalized source / tool / origin provenance a claim
    /// needs.
    MissingNormalizedProvenance,
    /// Evidence is stale beyond the freshness window.
    StaleEvidenceWindow,
}

impl NormalizedRecordDowngradeTrigger {
    /// Stable token recorded in the set.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnprovenStableIdentity => "unproven_stable_identity",
            Self::MissingReopenSurface => "missing_reopen_surface",
            Self::MissingNormalizedProvenance => "missing_normalized_provenance",
            Self::StaleEvidenceWindow => "stale_evidence_window",
        }
    }
}

/// One M5 finding normalized onto the canonical diagnostic record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedDiagnosticRecordEntry {
    /// Stable entry id.
    pub entry_id: String,
    /// M5 surface that produced or preserved the finding.
    pub surface: M5DiagnosticSurface,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Canonical v1 diagnostic record, reused rather than re-modeled.
    pub record: DiagnosticRecord,
    /// Proof that the record's id is stable across refreshes and surface hops.
    pub identity_family: DiagnosticStableIdentityFamily,
    /// Reopen handles, one per required consumer surface.
    pub reopen_handles: Vec<DiagnosticReopenHandle>,
    /// Suppression joins attached to this record.
    pub suppression_joins: Vec<DiagnosticSuppressionJoin>,
    /// Baseline joins attached to this record.
    pub baseline_joins: Vec<DiagnosticBaselineJoin>,
    /// Headline qualification publicly claimed for this record.
    pub claimed_qualification: NormalizedRecordQualificationClass,
    /// Effective qualification after auto-downgrade; equals the claim when the
    /// normalized identity is complete and ranks strictly below it otherwise.
    pub effective_qualification: NormalizedRecordQualificationClass,
    /// Trigger that fired the downgrade, required when the entry is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<NormalizedRecordDowngradeTrigger>,
    /// Precise degraded label, required when the entry is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,
    /// Evidence packet refs backing this entry.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this entry.
    pub source_contract_refs: Vec<String>,
}

impl NormalizedDiagnosticRecordEntry {
    /// Canonical diagnostic id this entry normalizes.
    pub fn diagnostic_id(&self) -> &str {
        &self.record.diagnostic_id
    }

    /// Whether this entry carries a public claim.
    pub fn is_claimed(&self) -> bool {
        self.claimed_qualification.is_claimed()
    }

    /// Reopen handle for one surface, when present.
    pub fn reopen_handle_for(
        &self,
        surface_class: DiagnosticSurfaceClass,
    ) -> Option<&DiagnosticReopenHandle> {
        self.reopen_handles
            .iter()
            .find(|handle| handle.surface_class == surface_class)
    }

    /// Whether every required consumer surface can reopen this record without
    /// translation loss.
    pub fn reopen_complete(&self) -> bool {
        REQUIRED_REOPEN_SURFACES.iter().all(|surface_class| {
            self.reopen_handle_for(*surface_class)
                .is_some_and(|handle| handle.reopens(self.diagnostic_id()))
        })
    }

    /// Whether the identity family proves a stable id that matches the record.
    pub fn identity_proven(&self) -> bool {
        self.identity_family.is_proven()
            && self.identity_family.diagnostic_id == self.record.diagnostic_id
            && self.identity_family.anchor_family_id == self.record.anchor_remap.anchor_family_id
    }

    /// Whether the record carries the normalized provenance a claim needs.
    pub fn normalized_provenance_ok(&self) -> bool {
        self.record.can_emit_beta_source()
    }

    /// Whether the suppression / baseline joins are referentially consistent and
    /// attached to the record rather than hidden in feature-local metadata.
    ///
    /// A join present here must carry the canonical id, must declare itself
    /// attached, and must be reflected on the record's own refs. This holds for
    /// every entry, claimed or held, and is enforced as a structural invariant by
    /// [`NormalizedDiagnosticRecordSetPacket::validate`].
    pub fn joins_attached(&self) -> bool {
        let suppression_ok = self.suppression_joins.iter().all(|join| {
            join.diagnostic_id == self.record.diagnostic_id
                && join.attached_to_record
                && self.record.suppression_refs.contains(&join.suppression_id)
        });
        let baseline_ok = self.baseline_joins.iter().all(|join| {
            join.diagnostic_id == self.record.diagnostic_id
                && join.attached_to_record
                && self.record.baseline_refs.contains(&join.baseline_id)
        });
        suppression_ok && baseline_ok
    }

    /// Whether every reopen handle present resolves to this record's canonical id.
    fn reopen_handles_consistent(&self) -> bool {
        self.reopen_handles
            .iter()
            .all(|handle| handle.resolves_diagnostic_id == self.record.diagnostic_id)
    }

    /// Whether the normalized identity is complete: stable id proven, reopenable
    /// from every required surface, and backed by normalized provenance.
    pub fn identity_complete(&self) -> bool {
        self.identity_proven() && self.reopen_complete() && self.normalized_provenance_ok()
    }

    /// Whether the entry must downgrade below its claim.
    pub fn needs_downgrade(&self) -> bool {
        !self.identity_complete()
    }

    /// Whether the effective qualification and downgrade evidence are consistent.
    pub fn downgrade_consistent(&self) -> bool {
        if self.needs_downgrade() {
            self.effective_qualification.rank() < self.claimed_qualification.rank()
                && self.downgrade_trigger.is_some()
                && self
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.effective_qualification == self.claimed_qualification
        }
    }

    /// Whether the identity family and joins reference this record's own id.
    pub fn referential_integrity_ok(&self) -> bool {
        self.identity_family.diagnostic_id == self.record.diagnostic_id
            && self.identity_family.anchor_family_id == self.record.anchor_remap.anchor_family_id
            && self.reopen_handles_consistent()
            && self
                .suppression_joins
                .iter()
                .all(|join| join.diagnostic_id == self.record.diagnostic_id)
            && self
                .baseline_joins
                .iter()
                .all(|join| join.diagnostic_id == self.record.diagnostic_id)
    }

    /// Whether this entry holds every structural invariant the set requires,
    /// independent of whether it is downgraded.
    pub fn is_structurally_complete(&self) -> bool {
        self.downgrade_consistent()
            && self.referential_integrity_ok()
            && self.joins_attached()
            && !self.entry_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.record.diagnostic_id.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Set-level guardrail invariants that must all hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedDiagnosticRecordSetGuardrails {
    /// Stable ids survive ordinary refresh and surface hops within one family.
    pub stable_ids_survive_refresh_and_surface_hop: bool,
    /// Unlike sources are never flattened into one synthetic finding.
    pub unlike_sources_never_flattened: bool,
    /// Convenience clustering never erases source / freshness / remap provenance.
    pub clustering_never_erases_provenance: bool,
    /// Imported-versus-live class stays explicit on every record.
    pub imported_live_class_explicit: bool,
    /// Source kind, freshness, and confidence stay on the detail / export paths.
    pub freshness_and_confidence_in_detail_paths: bool,
    /// Suppression / baseline joins stay attached to the normalized records.
    pub suppression_baseline_joins_attached_to_records: bool,
    /// Every mutating fix route is a typed quality-action proposal.
    pub mutating_fixes_are_typed_proposals: bool,
    /// Records auto-downgrade when their normalized identity is incomplete.
    pub records_auto_downgrade_on_incomplete_identity: bool,
}

impl NormalizedDiagnosticRecordSetGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.stable_ids_survive_refresh_and_surface_hop
            && self.unlike_sources_never_flattened
            && self.clustering_never_erases_provenance
            && self.imported_live_class_explicit
            && self.freshness_and_confidence_in_detail_paths
            && self.suppression_baseline_joins_attached_to_records
            && self.mutating_fixes_are_typed_proposals
            && self.records_auto_downgrade_on_incomplete_identity
    }
}

/// Declares which consumer surfaces reopen the normalized record directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedDiagnosticRecordConsumerProjection {
    /// Editor decorations / markers reopen the record.
    pub editor_reopens_record: bool,
    /// Problems rows reopen the record.
    pub problems_reopens_record: bool,
    /// Review annotations reopen the record.
    pub review_reopens_record: bool,
    /// CLI / headless explain output reopens the record.
    pub cli_headless_reopens_record: bool,
    /// AI evidence references reopen the record.
    pub ai_evidence_reopens_record: bool,
    /// Support export reopens the record.
    pub support_export_reopens_record: bool,
    /// Compact surfaces collapse presentation but preserve class in detail paths.
    pub compact_surfaces_preserve_class_in_detail: bool,
}

impl NormalizedDiagnosticRecordConsumerProjection {
    /// Whether every consumer projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.editor_reopens_record
            && self.problems_reopens_record
            && self.review_reopens_record
            && self.cli_headless_reopens_record
            && self.ai_evidence_reopens_record
            && self.support_export_reopens_record
            && self.compact_surfaces_preserve_class_in_detail
    }
}

/// Evidence freshness window for the set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedDiagnosticRecordEvidenceFreshness {
    /// Freshness SLO in hours; zero is invalid.
    pub evidence_freshness_slo_hours: u32,
    /// Timestamp of the last evidence refresh.
    pub last_evidence_refresh: String,
    /// Whether an entry auto-downgrades when its evidence is stale.
    pub auto_downgrade_on_stale: bool,
}

/// Constructor input for a [`NormalizedDiagnosticRecordSetPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedDiagnosticRecordSetPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Per-record entries.
    pub entries: Vec<NormalizedDiagnosticRecordEntry>,
    /// Guardrail invariants block.
    pub guardrails: NormalizedDiagnosticRecordSetGuardrails,
    /// Consumer projection block.
    pub consumer_projection: NormalizedDiagnosticRecordConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: NormalizedDiagnosticRecordEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 normalized diagnostic-record set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedDiagnosticRecordSetPacket {
    /// Record kind; must equal [`M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Per-record entries.
    pub entries: Vec<NormalizedDiagnosticRecordEntry>,
    /// Guardrail invariants block.
    pub guardrails: NormalizedDiagnosticRecordSetGuardrails,
    /// Consumer projection block.
    pub consumer_projection: NormalizedDiagnosticRecordConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: NormalizedDiagnosticRecordEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl NormalizedDiagnosticRecordSetPacket {
    /// Builds an M5 normalized diagnostic-record set packet.
    pub fn new(input: NormalizedDiagnosticRecordSetPacketInput) -> Self {
        Self {
            record_kind: M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_RECORD_KIND.to_owned(),
            schema_version: M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            set_label: input.set_label,
            entries: input.entries,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            evidence_freshness: input.evidence_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surfaces represented by some entry in this set.
    pub fn represented_surfaces(&self) -> BTreeSet<M5DiagnosticSurface> {
        self.entries.iter().map(|entry| entry.surface).collect()
    }

    /// Canonical diagnostic ids represented in this set.
    pub fn represented_diagnostic_ids(&self) -> BTreeSet<String> {
        self.entries
            .iter()
            .map(|entry| entry.record.diagnostic_id.clone())
            .collect()
    }

    /// Count of entries whose effective qualification was downgraded below its
    /// claim.
    pub fn downgraded_entry_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.needs_downgrade())
            .count()
    }

    /// Count of entries holding a public claim.
    pub fn claimed_entry_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.is_claimed())
            .count()
    }

    /// Validates the M5 normalized diagnostic-record set invariants.
    pub fn validate(&self) -> Vec<NormalizedDiagnosticRecordViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_RECORD_KIND {
            violations.push(NormalizedDiagnosticRecordViolation::WrongRecordKind);
        }
        if self.schema_version != M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_SCHEMA_VERSION {
            violations.push(NormalizedDiagnosticRecordViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.set_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(NormalizedDiagnosticRecordViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_entries(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_evidence_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("normalized diagnostic-record set serializes"),
        ) {
            violations.push(NormalizedDiagnosticRecordViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("normalized diagnostic-record set serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Normalized Diagnostic-Record Set\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.set_label));
        out.push_str(&format!("- Minted: `{}`\n", self.minted_at));
        out.push_str(&format!("- Entries: {}\n", self.entries.len()));
        out.push_str(&format!(
            "- Claimed entries: {}\n",
            self.claimed_entry_count()
        ));
        out.push_str(&format!(
            "- Downgraded entries: {}\n\n",
            self.downgraded_entry_count()
        ));

        out.push_str(
            "| Surface | Diagnostic id | Source | Freshness | Remap | Reopen | Suppr | Baseline | Claimed | Effective |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
        for entry in &self.entries {
            out.push_str(&format!(
                "| {} | `{}` | {} | {} | {} | {}/{} | {} | {} | {} | {} |\n",
                entry.surface.as_str(),
                entry.record.diagnostic_id,
                entry.record.source.source_kind.as_str(),
                entry.record.freshness_class.as_str(),
                entry.record.anchor_remap.remap_state_class.as_str(),
                entry.reopen_handles.len(),
                REQUIRED_REOPEN_SURFACES.len(),
                entry.suppression_joins.len(),
                entry.baseline_joins.len(),
                entry.claimed_qualification.as_str(),
                entry.effective_qualification.as_str(),
            ));
        }

        out.push('\n');
        for entry in &self.entries {
            if let Some(label) = &entry.degraded_label {
                out.push_str(&format!(
                    "- Degraded: `{}` — {}\n",
                    entry.record.diagnostic_id, label
                ));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or
/// validate.
#[derive(Debug)]
pub enum NormalizedDiagnosticRecordArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<NormalizedDiagnosticRecordViolation>),
}

impl fmt::Display for NormalizedDiagnosticRecordArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => write!(
                f,
                "normalized diagnostic-record set support export parse error: {err}"
            ),
            Self::Validation(violations) => write!(
                f,
                "normalized diagnostic-record set support export failed validation: {violations:?}"
            ),
        }
    }
}

impl Error for NormalizedDiagnosticRecordArtifactError {}

/// Invariant violations reported by
/// [`NormalizedDiagnosticRecordSetPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NormalizedDiagnosticRecordViolation {
    /// Record kind is wrong.
    WrongRecordKind,
    /// Schema version is wrong.
    WrongSchemaVersion,
    /// Packet identity fields are missing.
    MissingIdentity,
    /// Required canonical source contracts are missing.
    MissingSourceContracts,
    /// A required diagnostic-producing surface is unrepresented.
    RequiredSurfaceMissing,
    /// Two entries share the same canonical diagnostic id.
    DuplicateDiagnosticId,
    /// No consistent downgraded entry demonstrates the auto-downgrade rule.
    DowngradedEntryCaseMissing,
    /// An entry failed its structural completeness invariants.
    EntryStructurallyIncomplete,
    /// An entry with an incomplete identity was not downgraded.
    EntryNotDowngradedOnIncompleteIdentity,
    /// A downgraded entry is missing its precise label or trigger.
    DowngradedEntryMissingLabelOrTrigger,
    /// An identity family or join disagrees with the record's own id / family.
    RecordReferentialMismatch,
    /// A suppression or baseline join is detached from its record.
    JoinDetachedFromRecord,
    /// A reopen handle is malformed or resolves to the wrong id.
    ReopenHandleInvalid,
    /// An entry is missing backing evidence refs.
    EntryEvidenceMissing,
    /// Guardrail block is incomplete.
    GuardrailsIncomplete,
    /// Consumer projection block is incomplete.
    ConsumerProjectionIncomplete,
    /// Evidence freshness block is incomplete.
    EvidenceFreshnessIncomplete,
    /// Export-safe JSON carried forbidden boundary material.
    RawBoundaryMaterialInExport,
}

impl NormalizedDiagnosticRecordViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::DuplicateDiagnosticId => "duplicate_diagnostic_id",
            Self::DowngradedEntryCaseMissing => "downgraded_entry_case_missing",
            Self::EntryStructurallyIncomplete => "entry_structurally_incomplete",
            Self::EntryNotDowngradedOnIncompleteIdentity => {
                "entry_not_downgraded_on_incomplete_identity"
            }
            Self::DowngradedEntryMissingLabelOrTrigger => {
                "downgraded_entry_missing_label_or_trigger"
            }
            Self::RecordReferentialMismatch => "record_referential_mismatch",
            Self::JoinDetachedFromRecord => "join_detached_from_record",
            Self::ReopenHandleInvalid => "reopen_handle_invalid",
            Self::EntryEvidenceMissing => "entry_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Loads and validates the checked support-export artifact.
///
/// This is the canonical entry point downstream support, AI evidence, review, and
/// release-visible debt surfaces use to ingest the normalized records instead of
/// cloning provider-local state.
///
/// # Errors
///
/// Returns [`NormalizedDiagnosticRecordArtifactError`] when the artifact cannot be
/// parsed or fails validation.
pub fn current_m5_normalized_diagnostic_record_set_export(
) -> Result<NormalizedDiagnosticRecordSetPacket, NormalizedDiagnosticRecordArtifactError> {
    let packet: NormalizedDiagnosticRecordSetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/m5/diagnostics/diagnostic-record-proof/support_export.json"
    )))
    .map_err(NormalizedDiagnosticRecordArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(NormalizedDiagnosticRecordArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &NormalizedDiagnosticRecordSetPacket,
    violations: &mut Vec<NormalizedDiagnosticRecordViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_SCHEMA_REF,
        M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_DOC_REF,
        M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_ARTIFACT_REF,
        CANONICAL_DIAGNOSTIC_RECORD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(NormalizedDiagnosticRecordViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &NormalizedDiagnosticRecordSetPacket,
    violations: &mut Vec<NormalizedDiagnosticRecordViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in M5DiagnosticSurface::ALL {
        if !surfaces.contains(&required) {
            violations.push(NormalizedDiagnosticRecordViolation::RequiredSurfaceMissing);
            break;
        }
    }

    if packet.represented_diagnostic_ids().len() != packet.entries.len() {
        violations.push(NormalizedDiagnosticRecordViolation::DuplicateDiagnosticId);
    }

    if !packet
        .entries
        .iter()
        .any(|entry| entry.needs_downgrade() && entry.downgrade_consistent())
    {
        violations.push(NormalizedDiagnosticRecordViolation::DowngradedEntryCaseMissing);
    }
}

fn validate_entries(
    packet: &NormalizedDiagnosticRecordSetPacket,
    violations: &mut Vec<NormalizedDiagnosticRecordViolation>,
) {
    for entry in &packet.entries {
        if !entry.is_structurally_complete() {
            violations.push(NormalizedDiagnosticRecordViolation::EntryStructurallyIncomplete);
        }
        if entry.needs_downgrade()
            && entry.effective_qualification.rank() >= entry.claimed_qualification.rank()
        {
            violations
                .push(NormalizedDiagnosticRecordViolation::EntryNotDowngradedOnIncompleteIdentity);
        }
        if entry.needs_downgrade()
            && (entry.downgrade_trigger.is_none()
                || !entry
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations
                .push(NormalizedDiagnosticRecordViolation::DowngradedEntryMissingLabelOrTrigger);
        }
        if !entry.referential_integrity_ok() {
            violations.push(NormalizedDiagnosticRecordViolation::RecordReferentialMismatch);
        }
        if !entry.joins_attached() {
            violations.push(NormalizedDiagnosticRecordViolation::JoinDetachedFromRecord);
        }
        if entry
            .reopen_handles
            .iter()
            .any(|handle| handle.resolves_diagnostic_id != entry.record.diagnostic_id)
        {
            violations.push(NormalizedDiagnosticRecordViolation::ReopenHandleInvalid);
        }
        if entry.evidence_refs.is_empty() || entry.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(NormalizedDiagnosticRecordViolation::EntryEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &NormalizedDiagnosticRecordSetPacket,
    violations: &mut Vec<NormalizedDiagnosticRecordViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(NormalizedDiagnosticRecordViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &NormalizedDiagnosticRecordSetPacket,
    violations: &mut Vec<NormalizedDiagnosticRecordViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(NormalizedDiagnosticRecordViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_evidence_freshness(
    packet: &NormalizedDiagnosticRecordSetPacket,
    violations: &mut Vec<NormalizedDiagnosticRecordViolation>,
) {
    if packet.evidence_freshness.evidence_freshness_slo_hours == 0
        || packet
            .evidence_freshness
            .last_evidence_refresh
            .trim()
            .is_empty()
    {
        violations.push(NormalizedDiagnosticRecordViolation::EvidenceFreshnessIncomplete);
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "narrowed"
            | "downgraded"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
