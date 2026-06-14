//! Release certification rollup for source-first preview, inspect-to-source
//! fidelity, browser-runtime inspection, and round-trip honesty on every claimed
//! M5 framework or preview row.
//!
//! Where the earlier modules in this crate each own one preview/runtime truth
//! lane — [`crate::preview_session_descriptors`] (source-first preview sessions),
//! [`crate::inspect_to_source_tree`] (inspect-to-source mapping fidelity),
//! [`crate::browser_runtime_inspectors`] (DOM/CSS/network/storage browser-runtime
//! inspection), [`crate::visual_edit_transforms`] (real-source round-trip and
//! fallback), [`crate::preview_drift_recovery`] (drift/recovery drills), and
//! [`crate::extension_provider_conformance`] (provider conformance) — and the
//! [`crate::freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix`]
//! module freezes the *qualification matrix*, this module binds those lanes into a
//! single bounded **release certification** packet: the one canonical answer to
//! "for this claimed M5 framework / preview / browser-runtime row, is every
//! release-required proof lane currently backed by fresh evidence — and if not,
//! does the claim auto-narrow and does promotion block?"
//!
//! A [`CertificationRow`] reuses the frozen
//! [`crate::PreviewSurface`] vocabulary rather than minting synonyms, lists the
//! [`CertificationLane`]s release requires for that surface, and carries one
//! [`LaneProof`] per lane. The packet *auto-narrows*: a claimed row whose required
//! lanes are not all currently proven must carry an `effective_certification`
//! strictly below its claim, a recorded [`CertificationDowngradeTrigger`], a precise
//! degraded label, and `promotion_blocked = true` — so a release claim never outruns
//! the evidence that backs it. Stale source maps, unlabeled runtime targets, weak
//! provider replacement, and hidden inspect-only fallback regressions are recorded
//! as the narrowing trigger and block promotion (or visibly narrow) the affected
//! claim.
//!
//! Each [`LaneProof`] binds back to the canonical schema of the upstream B33 lane it
//! certifies (see [`CertificationLane::canonical_schema_ref`]), so product, docs,
//! diagnostics, provider conformance, and release surfaces ingest *this* one
//! certification result instead of re-narrating preview/runtime maturity by hand.
//!
//! [`PreviewRuntimeCertificationPacket::validate`] also refuses a row that lets a
//! runtime-only view masquerade as fresh proof, hides mapping uncertainty behind a
//! certified claim, auto-upgrades an inspect-only surface into a write-capable
//! designer flow, or blurs an embedded preview/browser boundary into product-native
//! authority.
//!
//! Raw URLs, hostnames, cookies, raw provider payloads, credentials, and raw runtime
//! handles never cross this boundary; the packet carries only typed class tokens,
//! booleans, and opaque evidence refs.
//!
//! The boundary schema is
//! [`schemas/preview/preview_runtime_certification.schema.json`](../../../../schemas/preview/preview_runtime_certification.schema.json).
//! The contract doc is
//! [`docs/preview/m5/preview_runtime_certification.md`](../../../../docs/preview/m5/preview_runtime_certification.md).
//! The protected fixture directory is
//! [`fixtures/preview/m5/preview_runtime_certification/`](../../../../fixtures/preview/m5/preview_runtime_certification/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    PreviewSurface, BROWSER_RUNTIME_INSPECTORS_SCHEMA_REF,
    EXTENSION_PROVIDER_CONFORMANCE_SCHEMA_REF, INSPECT_TO_SOURCE_TREE_SCHEMA_REF,
    PREVIEW_DRIFT_RECOVERY_DRILL_SET_SCHEMA_REF, PREVIEW_SESSION_DESCRIPTOR_SET_SCHEMA_REF,
    VISUAL_EDIT_TRANSFORMS_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`PreviewRuntimeCertificationPacket`].
pub const PREVIEW_RUNTIME_CERTIFICATION_RECORD_KIND: &str = "preview_runtime_certification";

/// Schema version for the preview/runtime certification packet.
pub const PREVIEW_RUNTIME_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PREVIEW_RUNTIME_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/preview/preview_runtime_certification.schema.json";

/// Repo-relative path of the contract doc.
pub const PREVIEW_RUNTIME_CERTIFICATION_DOC_REF: &str =
    "docs/preview/m5/preview_runtime_certification.md";

/// Repo-relative path of the protected fixture directory.
pub const PREVIEW_RUNTIME_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/preview/m5/preview_runtime_certification";

/// Repo-relative path of the checked support-export artifact.
pub const PREVIEW_RUNTIME_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/preview/m5/preview_runtime_certification/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const PREVIEW_RUNTIME_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/preview/m5/preview_runtime_certification.md";

/// Closed certification-lane vocabulary. Names the proof dimensions a claimed M5
/// framework/preview/browser-runtime row must keep current before it can be
/// certified for release. Each lane binds to the canonical upstream B33 schema it
/// rolls up rather than re-deriving that lane's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationLane {
    /// Source-first preview: the canonical source drives a derivative preview with
    /// an honest source-sync posture.
    SourceFirstPreview,
    /// Inspect-to-source fidelity: component/DOM/widget nodes carry an honest
    /// exact / approximate / generated-only / runtime-only mapping quality.
    InspectToSourceFidelity,
    /// Browser-runtime inspection: DOM/CSS/network/storage inspection carries an
    /// honest target kind, attach depth, and freshness.
    BrowserRuntimeInspection,
    /// Round-trip honesty: a visual edit previews the real source diff before
    /// commit, or degrades to a labeled inspect-only / source-only fallback.
    RoundTripHonesty,
    /// Drift / recovery drills: hot-reload, stale-source-map, reconnect, and
    /// posture-flip drift preserve honest truth across recovery.
    DriftRecovery,
    /// Provider conformance: the provider backing the row declared enough to honestly
    /// do so and degrades to bounded truth when it cannot.
    ProviderConformance,
}

impl CertificationLane {
    /// Every certification lane, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SourceFirstPreview,
        Self::InspectToSourceFidelity,
        Self::BrowserRuntimeInspection,
        Self::RoundTripHonesty,
        Self::DriftRecovery,
        Self::ProviderConformance,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceFirstPreview => "source_first_preview",
            Self::InspectToSourceFidelity => "inspect_to_source_fidelity",
            Self::BrowserRuntimeInspection => "browser_runtime_inspection",
            Self::RoundTripHonesty => "round_trip_honesty",
            Self::DriftRecovery => "drift_recovery",
            Self::ProviderConformance => "provider_conformance",
        }
    }

    /// Repo-relative path of the canonical upstream B33 schema this lane rolls up.
    /// The bound [`LaneProof::source_lane_ref`] must equal this so the certification
    /// ingests the lane's truth instead of re-narrating it.
    pub const fn canonical_schema_ref(self) -> &'static str {
        match self {
            Self::SourceFirstPreview => PREVIEW_SESSION_DESCRIPTOR_SET_SCHEMA_REF,
            Self::InspectToSourceFidelity => INSPECT_TO_SOURCE_TREE_SCHEMA_REF,
            Self::BrowserRuntimeInspection => BROWSER_RUNTIME_INSPECTORS_SCHEMA_REF,
            Self::RoundTripHonesty => VISUAL_EDIT_TRANSFORMS_SCHEMA_REF,
            Self::DriftRecovery => PREVIEW_DRIFT_RECOVERY_DRILL_SET_SCHEMA_REF,
            Self::ProviderConformance => EXTENSION_PROVIDER_CONFORMANCE_SCHEMA_REF,
        }
    }
}

/// Closed lane-proof-status vocabulary. Names whether the lane's evidence is
/// currently proven for the row, so a missing or stale proof can never be silent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneProofStatus {
    /// Fresh, current proof — the lane is backed within the evidence-freshness SLO.
    Current,
    /// Proof exists but is stale (past the freshness SLO) and must be refreshed.
    Stale,
    /// No proof at all for this lane on this row.
    Missing,
    /// The lane does not apply to this surface (e.g. browser-runtime on a non-browser
    /// design render). A not-applicable lane is never release-required.
    NotApplicable,
}

impl LaneProofStatus {
    /// Stable token recorded in the proof.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this status counts as fresh, current proof.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }

    /// True when this status blocks certification of a required lane (no fresh proof
    /// is on hand).
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Stale | Self::Missing)
    }
}

/// Closed certification-class ladder a claimed M5 row carries. Higher means a
/// stronger public release claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationClass {
    /// Release-certified: every release-required lane is currently proven.
    Certified,
    /// Publicly claimed at beta depth and still hardening.
    Beta,
    /// Narrow public preview.
    Preview,
    /// Held below preview pending current proof; not a public claim.
    Held,
    /// Promotion blocked outright; not available for release.
    Blocked,
}

impl CertificationClass {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Held => "held",
            Self::Blocked => "blocked",
        }
    }

    /// Whether this class is a publicly claimed, promotable lane.
    pub const fn is_promotable(self) -> bool {
        matches!(self, Self::Certified | Self::Beta | Self::Preview)
    }

    /// Ordinal rank used to compare claim severity; higher is a stronger claim, so a
    /// narrowing must move strictly lower.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Blocked => 0,
            Self::Held => 1,
            Self::Preview => 2,
            Self::Beta => 3,
            Self::Certified => 4,
        }
    }
}

/// Closed downgrade-trigger vocabulary. Names why a claimed row narrowed below its
/// claim and blocked promotion; the chrome quotes the trigger verbatim instead of a
/// generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDowngradeTrigger {
    /// A release-required lane has no current proof at all.
    MissingLaneProof,
    /// A release-required lane's proof went stale and must be refreshed.
    StaleLaneProof,
    /// The source map backing the row went stale.
    StaleSourceMap,
    /// A browser-runtime target kind could not be identified / labeled.
    UnlabeledRuntimeTarget,
    /// A weaker provider would replace a stronger one without a bounded downgrade.
    WeakProviderReplacement,
    /// An inspect-only fallback was hidden behind a write-capable claim.
    HiddenInspectOnlyFallback,
    /// An upstream lane narrowed and dragged this certification row down with it.
    UpstreamLaneNarrowed,
    /// Policy narrowed the row below its claim.
    PolicyNarrowed,
}

impl CertificationDowngradeTrigger {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingLaneProof => "missing_lane_proof",
            Self::StaleLaneProof => "stale_lane_proof",
            Self::StaleSourceMap => "stale_source_map",
            Self::UnlabeledRuntimeTarget => "unlabeled_runtime_target",
            Self::WeakProviderReplacement => "weak_provider_replacement",
            Self::HiddenInspectOnlyFallback => "hidden_inspect_only_fallback",
            Self::UpstreamLaneNarrowed => "upstream_lane_narrowed",
            Self::PolicyNarrowed => "policy_narrowed",
        }
    }
}

/// One lane's proof status for a single certification row, bound to the canonical
/// upstream B33 schema it rolls up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneProof {
    /// The certification lane this proof covers.
    pub lane: CertificationLane,
    /// Current proof status of the lane on this row.
    pub status: LaneProofStatus,
    /// Repo-relative ref of the canonical upstream lane schema this proof ingests;
    /// must equal [`CertificationLane::canonical_schema_ref`] for [`LaneProof::lane`].
    pub source_lane_ref: String,
    /// Opaque evidence packet ref backing this lane proof.
    pub evidence_ref: String,
    /// RFC 3339 timestamp of the last evidence refresh for this lane; required when
    /// the proof is current or stale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_refresh: Option<String>,
}

impl LaneProof {
    /// Whether this proof blocks certification of its lane (no fresh proof on hand).
    pub fn is_blocking(&self) -> bool {
        self.status.is_blocking()
    }

    /// Whether the proof binds to the canonical upstream schema for its lane.
    pub fn binds_canonical_lane(&self) -> bool {
        self.source_lane_ref == self.lane.canonical_schema_ref()
    }

    /// Whether every dimension required to record this proof is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        self.binds_canonical_lane()
            && !self.evidence_ref.trim().is_empty()
            && match self.status {
                LaneProofStatus::Current | LaneProofStatus::Stale => self
                    .last_refresh
                    .as_ref()
                    .is_some_and(|refreshed| !refreshed.trim().is_empty()),
                LaneProofStatus::Missing | LaneProofStatus::NotApplicable => true,
            }
    }
}

/// One claimed M5 framework/preview/browser-runtime row in the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationRow {
    /// Stable row id.
    pub row_id: String,
    /// Claimed preview/runtime surface, reusing the frozen matrix vocabulary.
    pub surface: PreviewSurface,
    /// Human-readable label of the claimed M5 row.
    pub claimed_surface_label: String,
    /// Lanes release requires to be currently proven for this surface.
    pub required_lanes: Vec<CertificationLane>,
    /// Per-lane proof status; must cover every required lane.
    pub lane_proofs: Vec<LaneProof>,
    /// Headline certification publicly claimed for this row.
    pub claimed_certification: CertificationClass,
    /// Effective certification after auto-narrowing; equals the claim when every
    /// required lane is currently proven, and ranks strictly below it otherwise.
    pub effective_certification: CertificationClass,
    /// Whether promotion is blocked. Must be true exactly when a release-required lane
    /// lacks current proof.
    pub promotion_blocked: bool,
    /// Whether the row claims a write-capable designer flow. A row whose round-trip
    /// honesty is not currently proven must never claim this.
    pub claims_write_capable: bool,
    /// Trigger that fired the narrowing / block; required when the row is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrow_trigger: Option<CertificationDowngradeTrigger>,
    /// Precise degraded label; required when the row is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,
    /// Human-readable label summary safe to render on the row.
    pub label_summary: String,
    /// ISO 8601 UTC timestamp the certification state was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Canonical upstream lane contract refs consumed by this row.
    pub source_lane_refs: Vec<String>,
}

impl CertificationRow {
    /// Whether this row carries a public claim.
    pub fn is_claimed(&self) -> bool {
        self.claimed_certification.is_promotable()
    }

    /// The proof recorded for a lane, if any.
    pub fn proof_for(&self, lane: CertificationLane) -> Option<&LaneProof> {
        self.lane_proofs.iter().find(|proof| proof.lane == lane)
    }

    /// Whether every required lane has a recorded proof entry and the required-lane
    /// list is non-empty.
    pub fn coverage_complete(&self) -> bool {
        !self.required_lanes.is_empty()
            && self
                .required_lanes
                .iter()
                .all(|lane| self.proof_for(*lane).is_some())
    }

    /// Whether any release-required lane lacks current proof (missing or stale).
    pub fn has_blocking_gap(&self) -> bool {
        self.required_lanes.iter().any(|lane| {
            self.proof_for(*lane)
                .map_or(true, |proof| proof.status.is_blocking())
        })
    }

    /// Whether the row must narrow below its claim because a required lane is not
    /// currently proven.
    pub fn needs_narrowing(&self) -> bool {
        self.has_blocking_gap()
    }

    /// Whether the effective certification and narrowing evidence are consistent.
    ///
    /// When every required lane is currently proven the effective certification equals
    /// the claim; otherwise it must rank strictly below the claim and carry both a
    /// recorded narrowing trigger and a precise degraded label.
    pub fn narrowing_consistent(&self) -> bool {
        if self.needs_narrowing() {
            self.effective_certification.rank() < self.claimed_certification.rank()
                && self.narrow_trigger.is_some()
                && self
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.effective_certification == self.claimed_certification
                && self.narrow_trigger.is_none()
                && self.degraded_label.is_none()
        }
    }

    /// Whether promotion gating is consistent: a row with a blocking gap blocks
    /// promotion and its effective certification is not promotable; a fully proven row
    /// is not blocked.
    pub fn promotion_consistent(&self) -> bool {
        if self.needs_narrowing() {
            self.promotion_blocked && !self.effective_certification.is_promotable()
        } else {
            !self.promotion_blocked
        }
    }

    /// Whether a write-capable claim is honestly backed: it appears only when the
    /// round-trip honesty lane is currently proven and the row is not narrowed.
    pub fn write_capability_ok(&self) -> bool {
        if self.claims_write_capable {
            !self.needs_narrowing()
                && self
                    .proof_for(CertificationLane::RoundTripHonesty)
                    .is_some_and(|proof| proof.status.is_current())
        } else {
            true
        }
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "surface={surface} claim={claim} effective={effective} blocked={blocked} \
write_capable={write}",
            surface = self.surface.as_str(),
            claim = self.claimed_certification.as_str(),
            effective = self.effective_certification.as_str(),
            blocked = self.promotion_blocked,
            write = self.claims_write_capable,
        )
    }

    /// Whether every dimension required to record this row is present and internally
    /// consistent.
    pub fn is_complete(&self) -> bool {
        !self.row_id.trim().is_empty()
            && !self.claimed_surface_label.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.observed_at.trim().is_empty()
            && self.coverage_complete()
            && self.lane_proofs.iter().all(LaneProof::is_complete)
            && self.narrowing_consistent()
            && self.promotion_consistent()
            && self.write_capability_ok()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.source_lane_refs.is_empty()
            && self.source_lane_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block for the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationGuardrails {
    /// Source remains canonical; the certification packet is derivative, never a
    /// second writable truth model.
    pub source_canonical_no_second_writable_model: bool,
    /// Runtime state and extension-private wording never hide source-mapping
    /// uncertainty behind a certified label.
    pub runtime_state_never_hides_source_mapping_uncertainty: bool,
    /// Inspect-only rows are never auto-upgraded into write-capable designer flows.
    pub inspect_only_never_auto_upgraded_to_write: bool,
    /// Embedded preview / browser boundaries are not blurred into product authority.
    pub embedded_boundaries_not_blurred_into_product: bool,
    /// A claimed row lacking current proof on a required lane auto-narrows below its
    /// claim.
    pub claimed_rows_auto_narrow_without_current_proof: bool,
    /// Stale source maps, unlabeled runtime targets, weak provider replacement, and
    /// hidden inspect-only fallback regressions block promotion or visibly narrow.
    pub regressions_block_promotion_or_narrow: bool,
    /// Product, docs/help, diagnostics, and release surfaces ingest this one
    /// certification result instead of cloning preview/runtime maturity text.
    pub single_certification_result_no_manual_clone: bool,
}

impl CertificationGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.source_canonical_no_second_writable_model
            && self.runtime_state_never_hides_source_mapping_uncertainty
            && self.inspect_only_never_auto_upgraded_to_write
            && self.embedded_boundaries_not_blurred_into_product
            && self.claimed_rows_auto_narrow_without_current_proof
            && self.regressions_block_promotion_or_narrow
            && self.single_certification_result_no_manual_clone
    }
}

/// Consumer-projection block for the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationConsumerProjection {
    /// Product surfaces ingest this certification instead of cloning maturity text.
    pub product_ingests_certification: bool,
    /// Docs/help ingests the same certification.
    pub docs_help_ingests_certification: bool,
    /// Diagnostics ingests the same certification.
    pub diagnostics_ingests_certification: bool,
    /// Extension/provider conformance ingests the same certification.
    pub provider_conformance_ingests_certification: bool,
    /// Release-control surfaces ingest the same certification.
    pub release_control_ingests_certification: bool,
    /// Narrowed or blocked rows are visibly labeled below current in every surface.
    pub narrowed_or_blocked_rows_labeled_below_current: bool,
}

impl CertificationConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_certification
            && self.docs_help_ingests_certification
            && self.diagnostics_ingests_certification
            && self.provider_conformance_ingests_certification
            && self.release_control_ingests_certification
            && self.narrowed_or_blocked_rows_labeled_below_current
    }
}

/// Evidence freshness block for the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationEvidenceFreshness {
    /// Evidence-freshness SLO in hours.
    pub evidence_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last evidence refresh.
    pub last_evidence_refresh: String,
    /// True when stale evidence automatically narrows claimed rows.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`PreviewRuntimeCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewRuntimeCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Per-row certifications.
    pub rows: Vec<CertificationRow>,
    /// Guardrail invariants block.
    pub guardrails: CertificationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: CertificationConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: CertificationEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe preview/runtime release certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewRuntimeCertificationPacket {
    /// Record kind; must equal [`PREVIEW_RUNTIME_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PREVIEW_RUNTIME_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Per-row certifications.
    pub rows: Vec<CertificationRow>,
    /// Guardrail invariants block.
    pub guardrails: CertificationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: CertificationConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: CertificationEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PreviewRuntimeCertificationPacket {
    /// Builds a preview/runtime release certification packet.
    pub fn new(input: PreviewRuntimeCertificationPacketInput) -> Self {
        Self {
            record_kind: PREVIEW_RUNTIME_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: PREVIEW_RUNTIME_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            evidence_freshness: input.evidence_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surfaces represented by some row in this certification.
    pub fn represented_surfaces(&self) -> BTreeSet<PreviewSurface> {
        self.rows.iter().map(|row| row.surface).collect()
    }

    /// Lanes required by some row in this certification.
    pub fn represented_lanes(&self) -> BTreeSet<CertificationLane> {
        self.rows
            .iter()
            .flat_map(|row| row.required_lanes.iter().copied())
            .collect()
    }

    /// Count of rows whose effective certification is release-certified.
    pub fn certified_row_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.effective_certification == CertificationClass::Certified)
            .count()
    }

    /// Count of rows whose effective certification was narrowed below its claim.
    pub fn narrowed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.needs_narrowing()).count()
    }

    /// Count of rows whose promotion is blocked.
    pub fn blocked_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.promotion_blocked).count()
    }

    /// Count of rows holding a public claim.
    pub fn claimed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.is_claimed()).count()
    }

    /// Validates the preview/runtime certification invariants.
    pub fn validate(&self) -> Vec<PreviewRuntimeCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PREVIEW_RUNTIME_CERTIFICATION_RECORD_KIND {
            violations.push(PreviewRuntimeCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != PREVIEW_RUNTIME_CERTIFICATION_SCHEMA_VERSION {
            violations.push(PreviewRuntimeCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PreviewRuntimeCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_evidence_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("preview runtime certification packet serializes"),
        ) {
            violations.push(PreviewRuntimeCertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("preview runtime certification packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Source-First Preview / Browser-Runtime Release Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Rows: {} ({} claimed, {} certified, {} narrowed, {} blocked)\n",
            self.rows.len(),
            self.claimed_row_count(),
            self.certified_row_count(),
            self.narrowed_row_count(),
            self.blocked_row_count(),
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {}\n",
            self.represented_surfaces().len(),
            PreviewSurface::ALL.len()
        ));
        out.push_str(&format!(
            "- Lanes covered: {} / {}\n",
            self.represented_lanes().len(),
            CertificationLane::ALL.len()
        ));
        out.push_str(&format!(
            "- Evidence freshness SLO: {} hours (last refresh: {})\n",
            self.evidence_freshness.evidence_freshness_slo_hours,
            self.evidence_freshness.last_evidence_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}): claim `{}` -> effective `{}`\n",
                row.row_id,
                row.surface.as_str(),
                row.claimed_certification.as_str(),
                row.effective_certification.as_str()
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!("  - {}\n", row.chip_tokens()));
            let lanes = row
                .lane_proofs
                .iter()
                .map(|proof| format!("{}={}", proof.lane.as_str(), proof.status.as_str()))
                .collect::<Vec<_>>()
                .join(" ");
            out.push_str(&format!("  - lanes: {lanes}\n"));
            if let Some(label) = &row.degraded_label {
                out.push_str(&format!("  - Degraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum PreviewRuntimeCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PreviewRuntimeCertificationViolation>),
}

impl fmt::Display for PreviewRuntimeCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "preview runtime certification export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "preview runtime certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PreviewRuntimeCertificationArtifactError {}

/// Validation failures emitted by [`PreviewRuntimeCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreviewRuntimeCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required preview/runtime surface is represented by no row.
    RequiredSurfaceMissing,
    /// A required certification lane is required by no row.
    RequiredLaneMissing,
    /// No row demonstrates auto-narrowing-and-blocking on a missing/stale lane proof.
    NarrowedRowCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A row's required lanes are not all covered by a proof entry.
    RowCoverageIncomplete,
    /// A row's lane proof is incomplete or does not bind its canonical lane schema.
    LaneProofIncomplete,
    /// A claimed row was not narrowed below its claim despite a blocking lane gap.
    RowNotNarrowedOnBlockingGap,
    /// A narrowed row lacks a precise degraded label or narrowing trigger.
    NarrowedRowMissingLabelOrTrigger,
    /// A row's promotion gating is inconsistent with its blocking gap.
    PromotionGatingInconsistent,
    /// A write-capable claim is not backed by current round-trip-honesty proof.
    WriteCapabilityUnbacked,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Evidence freshness block is incomplete.
    EvidenceFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl PreviewRuntimeCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::RequiredLaneMissing => "required_lane_missing",
            Self::NarrowedRowCaseMissing => "narrowed_row_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::RowCoverageIncomplete => "row_coverage_incomplete",
            Self::LaneProofIncomplete => "lane_proof_incomplete",
            Self::RowNotNarrowedOnBlockingGap => "row_not_narrowed_on_blocking_gap",
            Self::NarrowedRowMissingLabelOrTrigger => "narrowed_row_missing_label_or_trigger",
            Self::PromotionGatingInconsistent => "promotion_gating_inconsistent",
            Self::WriteCapabilityUnbacked => "write_capability_unbacked",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable certification export.
pub fn current_m5_preview_runtime_certification_export(
) -> Result<PreviewRuntimeCertificationPacket, PreviewRuntimeCertificationArtifactError> {
    let packet: PreviewRuntimeCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/preview/m5/preview_runtime_certification/support_export.json"
    )))
    .map_err(PreviewRuntimeCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PreviewRuntimeCertificationArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &PreviewRuntimeCertificationPacket,
    violations: &mut Vec<PreviewRuntimeCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PREVIEW_RUNTIME_CERTIFICATION_SCHEMA_REF,
        PREVIEW_RUNTIME_CERTIFICATION_DOC_REF,
        PREVIEW_RUNTIME_CERTIFICATION_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(PreviewRuntimeCertificationViolation::MissingSourceContracts);
            return;
        }
    }
    // The certification rolls up the canonical B33 lane schemas; each must be cited.
    for lane in CertificationLane::ALL {
        if !refs.contains(lane.canonical_schema_ref()) {
            violations.push(PreviewRuntimeCertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_coverage(
    packet: &PreviewRuntimeCertificationPacket,
    violations: &mut Vec<PreviewRuntimeCertificationViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in PreviewSurface::ALL {
        if !surfaces.contains(&required) {
            violations.push(PreviewRuntimeCertificationViolation::RequiredSurfaceMissing);
            break;
        }
    }

    let lanes = packet.represented_lanes();
    for required in CertificationLane::ALL {
        if !lanes.contains(&required) {
            violations.push(PreviewRuntimeCertificationViolation::RequiredLaneMissing);
            break;
        }
    }

    if !packet.rows.iter().any(|row| {
        row.needs_narrowing()
            && row.narrowing_consistent()
            && row.promotion_consistent()
            && row.promotion_blocked
    }) {
        violations.push(PreviewRuntimeCertificationViolation::NarrowedRowCaseMissing);
    }
}

fn validate_rows(
    packet: &PreviewRuntimeCertificationPacket,
    violations: &mut Vec<PreviewRuntimeCertificationViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(PreviewRuntimeCertificationViolation::RowIncomplete);
        }
        if !row.coverage_complete() {
            violations.push(PreviewRuntimeCertificationViolation::RowCoverageIncomplete);
        }
        if !row.lane_proofs.iter().all(LaneProof::is_complete) {
            violations.push(PreviewRuntimeCertificationViolation::LaneProofIncomplete);
        }
        if row.needs_narrowing()
            && row.effective_certification.rank() >= row.claimed_certification.rank()
        {
            violations.push(PreviewRuntimeCertificationViolation::RowNotNarrowedOnBlockingGap);
        }
        if row.needs_narrowing()
            && (row.narrow_trigger.is_none()
                || !row
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations.push(PreviewRuntimeCertificationViolation::NarrowedRowMissingLabelOrTrigger);
        }
        if !row.promotion_consistent() {
            violations.push(PreviewRuntimeCertificationViolation::PromotionGatingInconsistent);
        }
        if !row.write_capability_ok() {
            violations.push(PreviewRuntimeCertificationViolation::WriteCapabilityUnbacked);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(PreviewRuntimeCertificationViolation::RowEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &PreviewRuntimeCertificationPacket,
    violations: &mut Vec<PreviewRuntimeCertificationViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(PreviewRuntimeCertificationViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &PreviewRuntimeCertificationPacket,
    violations: &mut Vec<PreviewRuntimeCertificationViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(PreviewRuntimeCertificationViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_evidence_freshness(
    packet: &PreviewRuntimeCertificationPacket,
    violations: &mut Vec<PreviewRuntimeCertificationViolation>,
) {
    if packet.evidence_freshness.evidence_freshness_slo_hours == 0
        || packet
            .evidence_freshness
            .last_evidence_refresh
            .trim()
            .is_empty()
    {
        violations.push(PreviewRuntimeCertificationViolation::EvidenceFreshnessIncomplete);
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
///
/// A generic provider error must never stand in for a precise narrowing truth.
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
            | "blocked"
            | "stale"
            | "not certified"
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
                || lower.contains("set-cookie")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
