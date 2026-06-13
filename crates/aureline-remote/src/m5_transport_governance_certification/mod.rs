//! Transport-governance certification packet that hardens only the claimed M5
//! enterprise, network, and deployment profiles whose shared transport
//! decisions, proxy inputs, trust material, host proof, denial vocabulary, and
//! mirror/offline continuity all pass together.
//!
//! The sibling lanes each own one slice of transport governance:
//! [`crate::networked_surface_transport_decision`] emits one inspectable
//! decision per network-capable action,
//! [`crate::networked_surface_proxy_resolution`] freezes the proxy precedence,
//! [`crate::networked_surface_transport_trust`] freezes the trust store and host
//! proof, [`crate::networked_surface_mirror_offline_continuity`] freezes the
//! mirror/offline route handling, and
//! [`crate::networked_surface_transport_automation`] gives the failure surfaces
//! one canonical denial vocabulary. This module is the **certification** layer
//! that binds those slices into one verdict per named deployment profile, so a
//! marketed M5 enterprise/network/deployment row only hardens where every proof
//! dimension is current at once.
//!
//! The certification matrix binds, for every required
//! [`CertificationProfileClass`] profile, one [`CertificationCell`] per
//! [`CertificationDimensionClass`] proof dimension. Each cell carries a typed
//! [`CertificationCellStateClass`] proof state, a [`ProofFreshnessClass`]
//! freshness, and an opaque evidence ref bound to the sibling lane's canonical
//! contract and doc. From those cells the layer derives one
//! [`CertificationVerdictClass`] verdict per profile and **auto-narrows** any
//! profile whose proof is missing, stale, partial, or whose continuity / denial
//! coverage is incomplete — instead of publishing a wider claim than the
//! evidence permits.
//!
//! The certified claim holds for a profile when **all** of the following are
//! verified simultaneously:
//!
//! 1. Every required proof dimension has a cell (no silent coverage gap).
//! 2. Every cell is [`CertificationCellStateClass::Pass`] (and
//!    [`ProofFreshnessClass::Fresh`]) or
//!    [`CertificationCellStateClass::Waived`] for a dimension that does not
//!    apply to the profile.
//! 3. No raw private material is present on any cell or row.
//! 4. Every covered surface resolved through the shared transport-governance
//!    layer (`no_bypass: true`); no private proxy stack, direct CA override,
//!    undeclared public fallback, or hidden direct-connect retry.
//! 5. Any offline-deferred or replay action is explicitly idempotent.
//! 6. No mirror-only or deny-all profile silently falls through to the public
//!    internet (`no_silent_public_fallthrough: true`).
//!
//! Four conditions force [`CertificationVerdictClass::Withdrawn`] immediately
//! and taint the whole packet: raw private material exposed, a bypass of the
//! shared governance layer, a non-idempotent action queued for replay, or a
//! silent mirror-to-public fallthrough. A missing required dimension, missing
//! continuity coverage, or missing denial vocabulary narrows the profile to
//! [`CertificationVerdictClass::HeldBack`]; a stale or partial proof narrows it
//! to [`CertificationVerdictClass::Narrowed`]. Release, shiproom, docs, and
//! support tooling ingest this one packet rather than publishing their own
//! transport status text.
//!
//! No raw URLs, hostnames, ports, paths, query strings, cookies, headers,
//! bearer or session tokens, private certificate bytes, or SSH private material
//! ever cross the boundary — only closed-vocabulary tokens, opaque refs, UTC
//! timestamps, counts, and plain-language summary sentences.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/network/m5-transport-governance-certification.md`
//! - Artifact: `artifacts/network/m5-transport-governance-certification.md`
//! - Schema:
//!   `schemas/network/m5_transport_governance_certification.schema.json`
//! - Contract ref: [`M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::finalize_qualification_rows_for_desktop_local_remote_helper::DeploymentProfileClass;
use crate::networked_surface_mirror_offline_continuity::MIRROR_OFFLINE_CONTINUITY_SHARED_CONTRACT_REF;
use crate::networked_surface_proxy_resolution::PROXY_RESOLUTION_SHARED_CONTRACT_REF;
use crate::networked_surface_transport_automation::{
    REQUIRED_DENIAL_CODES, TRANSPORT_AUTOMATION_SHARED_CONTRACT_REF,
};
use crate::networked_surface_transport_decision::TRANSPORT_DECISION_SHARED_CONTRACT_REF;
use crate::networked_surface_transport_matrix::ProofFreshnessClass;
use crate::networked_surface_transport_trust::TRANSPORT_TRUST_SHARED_CONTRACT_REF;

#[cfg(test)]
mod tests;

/// Named deployment profile a certification verdict is computed for.
///
/// Reuses the canonical [`DeploymentProfileClass`] deployment vocabulary so the
/// certified profiles in this packet are the same `local_oss`, `self_hosted`,
/// `managed`, and `air_gapped` profiles the qualification matrix and the rest of
/// the product already reason about.
pub type CertificationProfileClass = DeploymentProfileClass;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SHARED_CONTRACT_REF: &str =
    "remote:m5_transport_governance_certification:v1";

/// Record-kind tag for [`M5TransportGovernanceCertificationPage`] payloads.
pub const CERTIFICATION_PAGE_RECORD_KIND: &str =
    "remote_m5_transport_governance_certification_page_record";

/// Record-kind tag for [`CertificationCell`] payloads.
pub const CERTIFICATION_CELL_RECORD_KIND: &str =
    "remote_m5_transport_governance_certification_cell_record";

/// Record-kind tag for [`ProfileCertificationSnapshot`] payloads.
pub const CERTIFICATION_PROFILE_RECORD_KIND: &str =
    "remote_m5_transport_governance_certification_profile_record";

/// Record-kind tag for [`CertificationRow`] payloads.
pub const CERTIFICATION_ROW_RECORD_KIND: &str =
    "remote_m5_transport_governance_certification_row_record";

/// Record-kind tag for [`DimensionBinding`] payloads.
pub const CERTIFICATION_BINDING_RECORD_KIND: &str =
    "remote_m5_transport_governance_certification_binding_record";

/// Record-kind tag for [`CertificationDefect`] payloads.
pub const CERTIFICATION_DEFECT_RECORD_KIND: &str =
    "remote_m5_transport_governance_certification_defect_record";

/// Record-kind tag for [`CertificationSummary`] payloads.
pub const CERTIFICATION_SUMMARY_RECORD_KIND: &str =
    "remote_m5_transport_governance_certification_summary_record";

/// Record-kind tag for [`CertificationSupportExport`] payloads.
pub const CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "remote_m5_transport_governance_certification_support_export_record";

/// Repo-relative path of the stable doc for this certification layer.
pub const CERTIFICATION_DOC_REF: &str = "docs/network/m5-transport-governance-certification.md";

/// Repo-relative path of the artifact summary for this certification layer.
pub const CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/network/m5-transport-governance-certification.md";

/// Repo-relative ref to the canonical closeout evidence index this layer binds
/// into.
pub const CERTIFICATION_EVIDENCE_INDEX_REF: &str = "artifacts/release/m5/xt12-evidence-index.md";

/// Stable, ordered catalog of the field names a certification cell renders.
///
/// Product surfaces, CLI/headless output, and support exports MUST render a cell
/// through this exact ordered field set, so the tokens a user reads in the UI
/// are identical to the ones CLI output and support packets quote.
pub const CELL_FIELD_NAMES: [&str; 5] = [
    "profile",
    "dimension",
    "state",
    "freshness",
    "evidence_contract_ref",
];

// ---------------------------------------------------------------------------
// Certification dimension vocabulary
// ---------------------------------------------------------------------------

/// One transport-governance proof dimension a profile is certified against.
///
/// The six dimensions map one-to-one onto the sibling transport-governance
/// lanes: each [`CertificationCell`] binds to the dimension's canonical contract
/// ref so the certification is an index over the existing proof rather than a
/// second copy of the status text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDimensionClass {
    /// Shared transport-decision layer: one inspectable decision per action.
    TransportDecision,
    /// Proxy/PAC/manual/environment/system precedence resolution.
    ProxyResolution,
    /// Trust store, CA bundle, pin-set, and client-certificate posture.
    TrustStore,
    /// SSH/TLS host-proof state and rotation history.
    HostProof,
    /// Mirror-only, local-bundle, and offline continuity route handling.
    MirrorOffline,
    /// Canonical transport-denial vocabulary coverage.
    DenialVocabulary,
}

/// The six required proof dimensions, in certification order.
pub const REQUIRED_DIMENSIONS: [CertificationDimensionClass; 6] = [
    CertificationDimensionClass::TransportDecision,
    CertificationDimensionClass::ProxyResolution,
    CertificationDimensionClass::TrustStore,
    CertificationDimensionClass::HostProof,
    CertificationDimensionClass::MirrorOffline,
    CertificationDimensionClass::DenialVocabulary,
];

impl CertificationDimensionClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TransportDecision => "transport_decision",
            Self::ProxyResolution => "proxy_resolution",
            Self::TrustStore => "trust_store",
            Self::HostProof => "host_proof",
            Self::MirrorOffline => "mirror_offline",
            Self::DenialVocabulary => "denial_vocabulary",
        }
    }

    /// Human-readable dimension label safe for UI and exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::TransportDecision => "Shared transport decision",
            Self::ProxyResolution => "Proxy resolution",
            Self::TrustStore => "Trust store and CA bundle",
            Self::HostProof => "Host proof",
            Self::MirrorOffline => "Mirror / offline continuity",
            Self::DenialVocabulary => "Denial vocabulary",
        }
    }

    /// Canonical contract ref of the sibling lane that owns this dimension's
    /// underlying proof.
    pub const fn contract_ref(self) -> &'static str {
        match self {
            Self::TransportDecision => TRANSPORT_DECISION_SHARED_CONTRACT_REF,
            Self::ProxyResolution => PROXY_RESOLUTION_SHARED_CONTRACT_REF,
            Self::TrustStore | Self::HostProof => TRANSPORT_TRUST_SHARED_CONTRACT_REF,
            Self::MirrorOffline => MIRROR_OFFLINE_CONTINUITY_SHARED_CONTRACT_REF,
            Self::DenialVocabulary => TRANSPORT_AUTOMATION_SHARED_CONTRACT_REF,
        }
    }

    /// Repo-relative doc ref of the sibling lane that owns this dimension.
    pub const fn doc_ref(self) -> &'static str {
        match self {
            Self::TransportDecision => "docs/network/networked-surface-transport-decision.md",
            Self::ProxyResolution => "docs/network/networked-surface-proxy-resolution.md",
            Self::TrustStore | Self::HostProof => {
                "docs/network/networked-surface-transport-trust.md"
            }
            Self::MirrorOffline => "docs/network/networked-surface-mirror-offline-continuity.md",
            Self::DenialVocabulary => "docs/network/networked-surface-transport-automation.md",
        }
    }
}

// ---------------------------------------------------------------------------
// Certification cell-state vocabulary
// ---------------------------------------------------------------------------

/// Proof state of one profile/dimension certification cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationCellStateClass {
    /// Current proof is present and within its freshness window.
    Pass,
    /// Proof is present but carries caveats; narrows the profile.
    Partial,
    /// Proof exists but has expired beyond its freshness window; narrows the
    /// profile.
    Stale,
    /// No proof is bound for this dimension; holds the profile back.
    Missing,
    /// The dimension does not apply to this profile (e.g. proxy resolution on a
    /// no-egress local profile); recorded and treated as satisfied.
    Waived,
}

impl CertificationCellStateClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Missing => "missing",
            Self::Waived => "waived",
        }
    }

    /// Human-readable label safe for UI and exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pass => "Pass",
            Self::Partial => "Partial",
            Self::Stale => "Stale",
            Self::Missing => "Missing",
            Self::Waived => "Waived (not applicable)",
        }
    }

    /// Returns `true` when the cell satisfies certification: a fresh pass or a
    /// waived (not-applicable) dimension.
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Pass | Self::Waived)
    }
}

// ---------------------------------------------------------------------------
// Certification verdict and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Certification verdict for one profile and for the overall packet.
///
/// The verdict is derived, not asserted: it is computed by comparing the audit
/// defect list against the certification conditions. A caller may never bump a
/// profile to [`Self::Certified`] without a clean audit across every required
/// dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationVerdictClass {
    /// Every required dimension passes fresh (or is waived); the profile is
    /// certified.
    Certified,
    /// A stale or partial proof narrows the profile below certified.
    Narrowed,
    /// A required dimension, continuity, or denial coverage is missing; the
    /// profile is held back.
    HeldBack,
    /// A hard guardrail was violated; the profile is withdrawn and the whole
    /// packet is tainted.
    Withdrawn,
}

impl CertificationVerdictClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Narrowed => "narrowed",
            Self::HeldBack => "held_back",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Returns `true` when the profile is certified.
    pub const fn is_certified(self) -> bool {
        matches!(self, Self::Certified)
    }

    /// Severity rank (higher is more severe) used to combine verdicts.
    const fn severity(self) -> u8 {
        match self {
            Self::Certified => 0,
            Self::Narrowed => 1,
            Self::HeldBack => 2,
            Self::Withdrawn => 3,
        }
    }
}

/// Typed reason a profile or the packet was narrowed below
/// [`CertificationVerdictClass::Certified`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationNarrowReasonClass {
    /// No narrowing — the profile is certified.
    NotNarrowed,
    /// A cell or row exposes raw private material; withdraws the packet.
    RawMaterialExposed,
    /// A covered surface resolved outside the shared transport-governance layer;
    /// withdraws the packet.
    SharedGovernanceBypassed,
    /// A non-idempotent action was queued for offline replay; withdraws the
    /// packet.
    NonIdempotentReplayQueued,
    /// A mirror-only or deny-all profile silently fell through to the public
    /// internet; withdraws the packet.
    SilentPublicFallthrough,
    /// A required deployment profile has no certification record; holds back.
    RequiredProfileMissing,
    /// A required transport proof dimension has no current proof; holds back.
    TransportProofMissing,
    /// Mirror/offline continuity coverage is missing; holds back.
    ContinuityCoverageMissing,
    /// The canonical denial vocabulary coverage is missing; holds back.
    DenialVocabularyMissing,
    /// A bound transport proof has expired beyond its window; narrows.
    TransportProofStale,
    /// A bound transport proof is present but only partial; narrows.
    TransportProofPartial,
}

impl CertificationNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::RawMaterialExposed => "raw_material_exposed",
            Self::SharedGovernanceBypassed => "shared_governance_bypassed",
            Self::NonIdempotentReplayQueued => "non_idempotent_replay_queued",
            Self::SilentPublicFallthrough => "silent_public_fallthrough",
            Self::RequiredProfileMissing => "required_profile_missing",
            Self::TransportProofMissing => "transport_proof_missing",
            Self::ContinuityCoverageMissing => "continuity_coverage_missing",
            Self::DenialVocabularyMissing => "denial_vocabulary_missing",
            Self::TransportProofStale => "transport_proof_stale",
            Self::TransportProofPartial => "transport_proof_partial",
        }
    }

    /// Returns `true` when this reason is a hard guardrail that withdraws the
    /// packet.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::RawMaterialExposed
                | Self::SharedGovernanceBypassed
                | Self::NonIdempotentReplayQueued
                | Self::SilentPublicFallthrough
        )
    }

    /// Returns `true` when this reason holds the profile back (a missing proof).
    pub const fn is_heldback_reason(self) -> bool {
        matches!(
            self,
            Self::RequiredProfileMissing
                | Self::TransportProofMissing
                | Self::ContinuityCoverageMissing
                | Self::DenialVocabularyMissing
        )
    }

    /// The certification verdict this narrow reason maps to.
    pub const fn verdict(self) -> CertificationVerdictClass {
        if self.is_withdrawal_reason() {
            CertificationVerdictClass::Withdrawn
        } else if self.is_heldback_reason() {
            CertificationVerdictClass::HeldBack
        } else if matches!(self, Self::NotNarrowed) {
            CertificationVerdictClass::Certified
        } else {
            CertificationVerdictClass::Narrowed
        }
    }
}

// ---------------------------------------------------------------------------
// Dimension binding (evidence index row)
// ---------------------------------------------------------------------------

/// A binding from one certification dimension to the sibling lane that owns its
/// underlying proof.
///
/// The page exposes one binding per dimension so the certification packet is
/// the canonical evidence index for transport governance — release, docs, and
/// support surfaces follow these refs rather than cloning per-lane status text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionBinding {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// The certification dimension.
    pub dimension: CertificationDimensionClass,
    /// Stable token for [`Self::dimension`].
    pub dimension_token: String,
    /// Human-readable dimension label.
    pub dimension_label: String,
    /// Canonical contract ref of the sibling lane owning the proof.
    pub evidence_contract_ref: String,
    /// Repo-relative doc ref of the sibling lane owning the proof.
    pub evidence_doc_ref: String,
}

impl DimensionBinding {
    fn new(dimension: CertificationDimensionClass) -> Self {
        Self {
            record_kind: CERTIFICATION_BINDING_RECORD_KIND.to_owned(),
            schema_version: M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SHARED_CONTRACT_REF
                .to_owned(),
            dimension,
            dimension_token: dimension.as_str().to_owned(),
            dimension_label: dimension.label().to_owned(),
            evidence_contract_ref: dimension.contract_ref().to_owned(),
            evidence_doc_ref: dimension.doc_ref().to_owned(),
        }
    }
}

/// The full ordered set of dimension bindings (one per required dimension).
pub fn dimension_bindings() -> Vec<DimensionBinding> {
    REQUIRED_DIMENSIONS
        .iter()
        .map(|d| DimensionBinding::new(*d))
        .collect()
}

// ---------------------------------------------------------------------------
// Certification cell (per profile/dimension)
// ---------------------------------------------------------------------------

/// Inspectable, redaction-safe proof cell for one profile and one dimension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationCell {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Deployment profile this cell certifies.
    pub profile: CertificationProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Proof dimension this cell certifies.
    pub dimension: CertificationDimensionClass,
    /// Stable token for [`Self::dimension`].
    pub dimension_token: String,
    /// Proof state for this cell.
    pub state: CertificationCellStateClass,
    /// Stable token for [`Self::state`].
    pub state_token: String,
    /// Freshness of the bound proof.
    pub freshness: ProofFreshnessClass,
    /// Stable token for [`Self::freshness`].
    pub freshness_token: String,
    /// Canonical contract ref of the sibling lane the proof is bound to.
    pub evidence_contract_ref: String,
    /// Export-safe plain-language note for this cell.
    pub note: String,
}

impl CertificationCell {
    /// Construct a cell, filling token fields from the typed values.
    pub fn new(
        profile: CertificationProfileClass,
        dimension: CertificationDimensionClass,
        state: CertificationCellStateClass,
        freshness: ProofFreshnessClass,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: CERTIFICATION_CELL_RECORD_KIND.to_owned(),
            schema_version: M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SHARED_CONTRACT_REF
                .to_owned(),
            profile,
            profile_token: profile.as_str().to_owned(),
            dimension,
            dimension_token: dimension.as_str().to_owned(),
            state,
            state_token: state.as_str().to_owned(),
            freshness,
            freshness_token: freshness.as_str().to_owned(),
            evidence_contract_ref: dimension.contract_ref().to_owned(),
            note: note.into(),
        }
    }

    /// Returns `true` when the cell satisfies certification.
    ///
    /// A [`CertificationCellStateClass::Pass`] cell only satisfies when its
    /// freshness is [`ProofFreshnessClass::Fresh`]; a pass whose proof has gone
    /// stale is treated as a stale cell, not a satisfied one.
    pub fn is_satisfied(&self) -> bool {
        match self.state {
            CertificationCellStateClass::Waived => true,
            CertificationCellStateClass::Pass => self.freshness == ProofFreshnessClass::Fresh,
            _ => false,
        }
    }

    /// The typed narrow reason this cell contributes, or `None` when satisfied.
    pub fn narrow_reason(&self) -> Option<CertificationNarrowReasonClass> {
        use CertificationCellStateClass as S;
        match self.state {
            S::Waived => None,
            S::Pass => {
                if self.freshness == ProofFreshnessClass::Fresh {
                    None
                } else {
                    Some(CertificationNarrowReasonClass::TransportProofStale)
                }
            }
            S::Partial => Some(CertificationNarrowReasonClass::TransportProofPartial),
            S::Stale => Some(CertificationNarrowReasonClass::TransportProofStale),
            S::Missing => Some(match self.dimension {
                CertificationDimensionClass::MirrorOffline => {
                    CertificationNarrowReasonClass::ContinuityCoverageMissing
                }
                CertificationDimensionClass::DenialVocabulary => {
                    CertificationNarrowReasonClass::DenialVocabularyMissing
                }
                _ => CertificationNarrowReasonClass::TransportProofMissing,
            }),
        }
    }

    /// Render the cell's fields as ordered `(name, value)` pairs through the
    /// shared [`CELL_FIELD_NAMES`] catalog.
    pub fn render_fields(&self) -> Vec<(String, String)> {
        let values: [&str; CELL_FIELD_NAMES.len()] = [
            self.profile_token.as_str(),
            self.dimension_token.as_str(),
            self.state_token.as_str(),
            self.freshness_token.as_str(),
            self.evidence_contract_ref.as_str(),
        ];
        CELL_FIELD_NAMES
            .iter()
            .zip(values.iter())
            .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
            .collect()
    }

    /// Render the cell as CLI/headless `key=value` lines.
    pub fn render_cli_lines(&self) -> Vec<String> {
        self.render_fields()
            .into_iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect()
    }

    /// Returns `true` when the rendered field names match [`CELL_FIELD_NAMES`]
    /// in order.
    pub fn fields_at_parity(&self) -> bool {
        let rendered = self.render_fields();
        rendered.len() == CELL_FIELD_NAMES.len()
            && rendered
                .iter()
                .zip(CELL_FIELD_NAMES.iter())
                .all(|((name, _), expected)| name == *expected)
    }
}

// ---------------------------------------------------------------------------
// Profile certification snapshot (per profile, input)
// ---------------------------------------------------------------------------

/// Per-profile certification input: the cells for every dimension plus the
/// profile-level guardrail flags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileCertificationSnapshot {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Deployment profile certified.
    pub profile: CertificationProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Human-readable profile label.
    pub profile_label: String,
    /// One cell per certified dimension.
    pub cells: Vec<CertificationCell>,
    /// `true` when no raw private material is present on this profile's record.
    pub raw_private_material_excluded: bool,
    /// `true` when every covered surface resolved through the shared
    /// transport-governance layer.
    pub no_bypass: bool,
    /// `true` when every offline-deferred or replay action is idempotent.
    pub replay_idempotent_only: bool,
    /// `true` when no mirror-only or deny-all route silently fell through to the
    /// public internet.
    pub no_silent_public_fallthrough: bool,
}

impl ProfileCertificationSnapshot {
    /// Construct a profile snapshot from its cells with clean guardrail flags.
    pub fn new(profile: CertificationProfileClass, cells: Vec<CertificationCell>) -> Self {
        Self {
            record_kind: CERTIFICATION_PROFILE_RECORD_KIND.to_owned(),
            schema_version: M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SHARED_CONTRACT_REF
                .to_owned(),
            profile,
            profile_token: profile.as_str().to_owned(),
            profile_label: profile.label().to_owned(),
            cells,
            raw_private_material_excluded: true,
            no_bypass: true,
            replay_idempotent_only: true,
            no_silent_public_fallthrough: true,
        }
    }

    /// The cell for one dimension, if present.
    pub fn cell_for(&self, dimension: CertificationDimensionClass) -> Option<&CertificationCell> {
        self.cells.iter().find(|c| c.dimension == dimension)
    }

    /// Returns `true` when every required dimension has a cell.
    pub fn covers_all_dimensions(&self) -> bool {
        REQUIRED_DIMENSIONS
            .iter()
            .all(|d| self.cell_for(*d).is_some())
    }

    /// The first profile-level guardrail breach, if any.
    fn guardrail_breach(&self) -> Option<CertificationNarrowReasonClass> {
        if !self.raw_private_material_excluded {
            Some(CertificationNarrowReasonClass::RawMaterialExposed)
        } else if !self.no_bypass {
            Some(CertificationNarrowReasonClass::SharedGovernanceBypassed)
        } else if !self.replay_idempotent_only {
            Some(CertificationNarrowReasonClass::NonIdempotentReplayQueued)
        } else if !self.no_silent_public_fallthrough {
            Some(CertificationNarrowReasonClass::SilentPublicFallthrough)
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Certification snapshot (all profiles)
// ---------------------------------------------------------------------------

/// Aggregate of every profile certification snapshot in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TransportGovernanceCertificationSnapshot {
    /// All profile snapshots, in profile order.
    pub profiles: Vec<ProfileCertificationSnapshot>,
}

impl M5TransportGovernanceCertificationSnapshot {
    /// The set of profile tokens covered by this snapshot.
    pub fn covered_profile_tokens(&self) -> BTreeSet<&str> {
        self.profiles
            .iter()
            .map(|p| p.profile_token.as_str())
            .collect()
    }

    /// Every cell across every profile, in order.
    pub fn all_cells(&self) -> Vec<&CertificationCell> {
        self.profiles.iter().flat_map(|p| p.cells.iter()).collect()
    }
}

// ---------------------------------------------------------------------------
// Certification row (derived per profile)
// ---------------------------------------------------------------------------

/// Derived certification row for one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Profile token for this row.
    pub profile_token: String,
    /// Human-readable profile label.
    pub profile_label: String,
    /// Derived verdict token.
    pub verdict_token: String,
    /// Why the row was narrowed (or `not_narrowed` when certified).
    pub narrow_reason_token: String,
    /// Number of required dimensions whose cell is satisfied.
    pub satisfied_dimension_count: usize,
    /// Total number of required dimensions.
    pub total_dimension_count: usize,
    /// Per-dimension cell-state tokens, keyed by dimension token.
    pub cell_states: BTreeMap<String, String>,
    /// `true` when no raw private material is present.
    pub raw_private_material_excluded: bool,
    /// `true` when the profile resolved through the shared governance layer.
    pub no_bypass: bool,
    /// `true` when replay queues are idempotent-only.
    pub replay_idempotent_only: bool,
    /// `true` when no silent mirror-to-public fallthrough occurred.
    pub no_silent_public_fallthrough: bool,
    /// Plain-language summary of the verdict for this row.
    pub plain_language_summary: String,
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// Aggregate banner emitted with the certification page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CertificationSummary {
    /// Total row count (one row per covered profile).
    pub row_count: usize,
    /// Rows certified.
    pub certified_row_count: usize,
    /// Rows narrowed.
    pub narrowed_row_count: usize,
    /// Rows held back.
    pub held_back_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Total certification cells across all profiles.
    pub cell_count: usize,
    /// Cell counts by state token.
    pub cell_state_counts: BTreeMap<String, usize>,
    /// Profile tokens covered by the page.
    pub profiles_covered: Vec<String>,
    /// Dimension tokens certified by the page.
    pub dimensions_certified: Vec<String>,
    /// Canonical denial-code tokens the certification binds as reusable
    /// vocabulary.
    pub denial_vocabulary: Vec<String>,
    /// Overall verdict token derived from all rows.
    pub overall_verdict_token: String,
}

impl CertificationSummary {
    fn build(
        rows: &[CertificationRow],
        snapshot: &M5TransportGovernanceCertificationSnapshot,
        defects: &[CertificationDefect],
    ) -> Self {
        let mut certified = 0usize;
        let mut narrowed = 0usize;
        let mut held = 0usize;
        let mut withdrawn = 0usize;
        for row in rows {
            match row.verdict_token.as_str() {
                "certified" => certified += 1,
                "narrowed" => narrowed += 1,
                "held_back" => held += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
        }
        let has_withdrawal = defects
            .iter()
            .any(|d| d.narrow_reason.is_withdrawal_reason());
        let has_heldback = defects.iter().any(|d| d.narrow_reason.is_heldback_reason());
        let overall = if has_withdrawal || withdrawn > 0 {
            CertificationVerdictClass::Withdrawn
        } else if has_heldback || held > 0 {
            CertificationVerdictClass::HeldBack
        } else if !defects.is_empty() || narrowed > 0 {
            CertificationVerdictClass::Narrowed
        } else {
            CertificationVerdictClass::Certified
        };
        let mut cell_state_counts: BTreeMap<String, usize> = BTreeMap::new();
        for cell in snapshot.all_cells() {
            *cell_state_counts
                .entry(cell.state_token.clone())
                .or_insert(0) += 1;
        }
        Self {
            row_count: rows.len(),
            certified_row_count: certified,
            narrowed_row_count: narrowed,
            held_back_row_count: held,
            withdrawn_row_count: withdrawn,
            cell_count: snapshot.all_cells().len(),
            cell_state_counts,
            profiles_covered: snapshot
                .covered_profile_tokens()
                .into_iter()
                .map(str::to_owned)
                .collect(),
            dimensions_certified: REQUIRED_DIMENSIONS
                .iter()
                .map(|d| d.as_str().to_owned())
                .collect(),
            denial_vocabulary: REQUIRED_DENIAL_CODES
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            overall_verdict_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defect
// ---------------------------------------------------------------------------

/// Typed defect emitted by the certification audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: CertificationNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject of the defect (profile token, `profile:dimension`, or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl CertificationDefect {
    fn new(
        narrow_reason: CertificationNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: CERTIFICATION_DEFECT_RECORD_KIND.to_owned(),
            schema_version: M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SHARED_CONTRACT_REF
                .to_owned(),
            defect_id: format!(
                "remote:defect:m5-transport-governance-certification:{}:{}",
                narrow_reason.as_str(),
                &source_str
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source: source_str,
            note: note.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Certification page (proof packet)
// ---------------------------------------------------------------------------

/// Stable transport-governance certification proof packet for the claimed M5
/// enterprise/network/deployment profiles.
///
/// This packet is the single inspectable record that proves which marketed M5
/// profiles end the milestone with current transport, proxy, trust, host-proof,
/// denial-vocabulary, and mirror/offline proof — and which must narrow or hold.
/// Release center, shiproom, Help/About, docs, support exports, and diagnostics
/// ingest this packet rather than publishing their own transport status text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TransportGovernanceCertificationPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// UTC instant when the packet was generated.
    pub generated_at: String,
    /// Repo-relative ref to the canonical closeout evidence index.
    pub evidence_index_ref: String,
    /// Aggregate summary derived from all rows.
    pub summary: CertificationSummary,
    /// Per-dimension evidence bindings (the transport-governance index).
    pub dimension_bindings: Vec<DimensionBinding>,
    /// Per-profile certification rows.
    pub rows: Vec<CertificationRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<CertificationDefect>,
    /// The certification snapshot embedded as evidence.
    pub certification_snapshot: M5TransportGovernanceCertificationSnapshot,
}

impl M5TransportGovernanceCertificationPage {
    /// Build the certification page from a snapshot.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        certification_snapshot: M5TransportGovernanceCertificationSnapshot,
    ) -> Self {
        let defects = audit_snapshot(&certification_snapshot);
        let rows = derive_rows(&certification_snapshot, &defects);
        let summary = CertificationSummary::build(&rows, &certification_snapshot, &defects);
        Self {
            record_kind: CERTIFICATION_PAGE_RECORD_KIND.to_owned(),
            schema_version: M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SHARED_CONTRACT_REF
                .to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            evidence_index_ref: CERTIFICATION_EVIDENCE_INDEX_REF.to_owned(),
            summary,
            dimension_bindings: dimension_bindings(),
            rows,
            defects,
            certification_snapshot,
        }
    }

    /// Returns `true` when the overall verdict is certified.
    pub fn is_certified(&self) -> bool {
        self.summary.overall_verdict_token == CertificationVerdictClass::Certified.as_str()
    }

    /// Returns `true` when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// Returns `true` when every required profile has a certification record.
    pub fn covers_all_required_profiles(&self) -> bool {
        let covered = self.certification_snapshot.covered_profile_tokens();
        REQUIRED_PROFILES
            .iter()
            .all(|p| covered.contains(p.as_str()))
    }

    /// Returns `true` when every profile certifies every required dimension.
    pub fn covers_all_dimensions(&self) -> bool {
        self.certification_snapshot
            .profiles
            .iter()
            .all(|p| p.covers_all_dimensions())
    }

    /// Returns `true` when the page binds every required dimension to its
    /// sibling lane evidence.
    pub fn binds_all_dimensions(&self) -> bool {
        REQUIRED_DIMENSIONS.iter().all(|d| {
            self.dimension_bindings
                .iter()
                .any(|b| b.dimension == *d && b.evidence_contract_ref == d.contract_ref())
        })
    }

    /// Returns `true` when every cell renders at field-catalog parity.
    pub fn all_cells_at_field_parity(&self) -> bool {
        self.certification_snapshot
            .all_cells()
            .iter()
            .all(|c| c.fields_at_parity())
    }

    /// Returns `true` when every profile excludes raw private material.
    pub fn raw_private_material_excluded(&self) -> bool {
        self.certification_snapshot
            .profiles
            .iter()
            .all(|p| p.raw_private_material_excluded)
    }
}

/// The four named deployment profiles a clean certification packet must cover.
pub const REQUIRED_PROFILES: [CertificationProfileClass; 4] = [
    CertificationProfileClass::LocalOss,
    CertificationProfileClass::SelfHosted,
    CertificationProfileClass::Managed,
    CertificationProfileClass::AirGapped,
];

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that carries the certification page plus a
/// metadata-safe defect roll-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// UTC export timestamp.
    pub generated_at: String,
    /// The certification page embedded as evidence.
    pub page: M5TransportGovernanceCertificationPage,
    /// Narrow-reason class values present in the page's defect list.
    pub narrow_reasons_present: Vec<CertificationNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// `true` when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl CertificationSupportExport {
    /// Wrap a certification page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: M5TransportGovernanceCertificationPage,
    ) -> Self {
        let mut reasons: Vec<CertificationNarrowReasonClass> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }
        reasons.sort();
        Self {
            record_kind: CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SHARED_CONTRACT_REF
                .to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_private_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Audit and validate functions (public API)
// ---------------------------------------------------------------------------

/// Re-run the certification audit over the snapshot embedded in a page.
pub fn audit_certification_page(
    page: &M5TransportGovernanceCertificationPage,
) -> Vec<CertificationDefect> {
    audit_snapshot(&page.certification_snapshot)
}

/// Validate a certification page; returns `Ok` when the audit is clean.
pub fn validate_certification_page(
    page: &M5TransportGovernanceCertificationPage,
) -> Result<(), Vec<CertificationDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal audit logic
// ---------------------------------------------------------------------------

fn audit_snapshot(
    snapshot: &M5TransportGovernanceCertificationSnapshot,
) -> Vec<CertificationDefect> {
    let mut defects: Vec<CertificationDefect> = Vec::new();

    // Hard guardrails first — any one of these withdraws the packet and makes
    // no further check meaningful.
    for profile in &snapshot.profiles {
        if let Some(breach) = profile.guardrail_breach() {
            defects.push(CertificationDefect::new(
                breach,
                profile.profile_token.clone(),
                format!(
                    "profile '{}' violates a transport-governance guardrail ({}); the packet is withdrawn",
                    profile.profile_token,
                    breach.as_str()
                ),
            ));
            return defects;
        }
    }

    // Required-profile coverage.
    let covered = snapshot.covered_profile_tokens();
    for required in &REQUIRED_PROFILES {
        if !covered.contains(required.as_str()) {
            defects.push(CertificationDefect::new(
                CertificationNarrowReasonClass::RequiredProfileMissing,
                required.as_str(),
                format!(
                    "required profile '{}' has no certification record; the packet is held back",
                    required.as_str()
                ),
            ));
        }
    }

    // Per-profile dimension coverage and per-cell proof state.
    for profile in &snapshot.profiles {
        for dimension in &REQUIRED_DIMENSIONS {
            match profile.cell_for(*dimension) {
                None => {
                    let reason = match dimension {
                        CertificationDimensionClass::MirrorOffline => {
                            CertificationNarrowReasonClass::ContinuityCoverageMissing
                        }
                        CertificationDimensionClass::DenialVocabulary => {
                            CertificationNarrowReasonClass::DenialVocabularyMissing
                        }
                        _ => CertificationNarrowReasonClass::TransportProofMissing,
                    };
                    defects.push(CertificationDefect::new(
                        reason,
                        format!("{}:{}", profile.profile_token, dimension.as_str()),
                        format!(
                            "profile '{}' has no certification cell for dimension '{}'; the profile is held back",
                            profile.profile_token,
                            dimension.as_str()
                        ),
                    ));
                }
                Some(cell) => {
                    if let Some(reason) = cell.narrow_reason() {
                        defects.push(CertificationDefect::new(
                            reason,
                            format!("{}:{}", profile.profile_token, dimension.as_str()),
                            format!(
                                "profile '{}' dimension '{}' is '{}' ({}); the profile narrows below certified",
                                profile.profile_token,
                                dimension.as_str(),
                                cell.state_token,
                                cell.freshness_token
                            ),
                        ));
                    }
                }
            }
        }
    }

    defects
}

fn derive_rows(
    snapshot: &M5TransportGovernanceCertificationSnapshot,
    page_defects: &[CertificationDefect],
) -> Vec<CertificationRow> {
    let withdrawal_reason = page_defects
        .iter()
        .find(|d| d.narrow_reason.is_withdrawal_reason())
        .map(|d| d.narrow_reason);

    snapshot
        .profiles
        .iter()
        .map(|profile| {
            let governing_reason = governing_reason_for(profile, page_defects, withdrawal_reason);
            let verdict = governing_reason.verdict();
            let satisfied = REQUIRED_DIMENSIONS
                .iter()
                .filter(|d| {
                    profile
                        .cell_for(**d)
                        .map(|c| c.is_satisfied())
                        .unwrap_or(false)
                })
                .count();
            let cell_states: BTreeMap<String, String> = profile
                .cells
                .iter()
                .map(|c| (c.dimension_token.clone(), c.state_token.clone()))
                .collect();
            let summary =
                build_row_summary(&profile.profile_token, verdict, governing_reason, satisfied);
            CertificationRow {
                record_kind: CERTIFICATION_ROW_RECORD_KIND.to_owned(),
                schema_version: M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SCHEMA_VERSION,
                shared_contract_ref: M5_TRANSPORT_GOVERNANCE_CERTIFICATION_SHARED_CONTRACT_REF
                    .to_owned(),
                profile_token: profile.profile_token.clone(),
                profile_label: profile.profile_label.clone(),
                verdict_token: verdict.as_str().to_owned(),
                narrow_reason_token: governing_reason.as_str().to_owned(),
                satisfied_dimension_count: satisfied,
                total_dimension_count: REQUIRED_DIMENSIONS.len(),
                cell_states,
                raw_private_material_excluded: profile.raw_private_material_excluded,
                no_bypass: profile.no_bypass,
                replay_idempotent_only: profile.replay_idempotent_only,
                no_silent_public_fallthrough: profile.no_silent_public_fallthrough,
                plain_language_summary: summary,
            }
        })
        .collect()
}

/// The narrow reason that governs one profile's verdict: a packet-wide
/// withdrawal taints every row; otherwise the most severe per-cell reason wins.
fn governing_reason_for(
    profile: &ProfileCertificationSnapshot,
    page_defects: &[CertificationDefect],
    withdrawal_reason: Option<CertificationNarrowReasonClass>,
) -> CertificationNarrowReasonClass {
    if let Some(reason) = withdrawal_reason {
        return reason;
    }
    let prefix = format!("{}:", profile.profile_token);
    let mut governing = CertificationNarrowReasonClass::NotNarrowed;
    for defect in page_defects {
        let applies = defect.source == profile.profile_token || defect.source.starts_with(&prefix);
        if applies && defect.narrow_reason.verdict().severity() > governing.verdict().severity() {
            governing = defect.narrow_reason;
        }
    }
    governing
}

fn build_row_summary(
    profile_token: &str,
    verdict: CertificationVerdictClass,
    reason: CertificationNarrowReasonClass,
    satisfied: usize,
) -> String {
    match verdict {
        CertificationVerdictClass::Certified => format!(
            "Profile '{}' is certified: all {} transport-governance dimensions pass fresh (or are \
             waived), every surface resolved through the shared governance layer, replay queues \
             are idempotent-only, and no mirror-only route silently fell through to the public \
             internet.",
            profile_token,
            REQUIRED_DIMENSIONS.len()
        ),
        _ => format!(
            "Profile '{}' narrowed to {} ({}): {}/{} dimensions satisfied; see the defect list for \
             details.",
            profile_token,
            verdict.as_str(),
            reason.as_str(),
            satisfied,
            REQUIRED_DIMENSIONS.len()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded certified certification page consumed by the headless
/// example, the integration tests, and the fixture generator.
///
/// The seeded page produces zero defects: every required profile covers every
/// required dimension, every cell passes fresh (or is waived for a dimension
/// that does not apply to the profile), no raw private material is present,
/// every profile resolved through the shared governance layer, replay queues are
/// idempotent-only, and no mirror-only route silently falls through.
pub fn seeded_m5_transport_governance_certification_page() -> M5TransportGovernanceCertificationPage
{
    M5TransportGovernanceCertificationPage::new(
        "remote:m5_transport_governance_certification:default",
        "M5 transport-governance certification — certified packet",
        "2026-06-01T00:00:00Z",
        seeded_m5_transport_governance_certification_snapshot(),
    )
}

/// Build the seeded certification snapshot used by the seeded page.
pub fn seeded_m5_transport_governance_certification_snapshot(
) -> M5TransportGovernanceCertificationSnapshot {
    M5TransportGovernanceCertificationSnapshot {
        profiles: vec![
            seeded_profile(CertificationProfileClass::LocalOss),
            seeded_profile(CertificationProfileClass::SelfHosted),
            seeded_profile(CertificationProfileClass::Managed),
            seeded_profile(CertificationProfileClass::AirGapped),
        ],
    }
}

/// Build one seeded, certified profile snapshot.
///
/// Each profile passes every required dimension fresh. Dimensions that do not
/// apply to a profile are waived rather than asserted: a no-egress local profile
/// waives proxy resolution and host proof; every other profile certifies all
/// six dimensions.
fn seeded_profile(profile: CertificationProfileClass) -> ProfileCertificationSnapshot {
    let fresh = ProofFreshnessClass::Fresh;
    let cells = REQUIRED_DIMENSIONS
        .iter()
        .map(|dimension| {
            let waived = profile == CertificationProfileClass::LocalOss
                && matches!(
                    dimension,
                    CertificationDimensionClass::ProxyResolution
                        | CertificationDimensionClass::HostProof
                );
            if waived {
                CertificationCell::new(
                    profile,
                    *dimension,
                    CertificationCellStateClass::Waived,
                    fresh,
                    format!(
                        "Dimension '{}' does not apply to the local-OSS no-egress profile; waived.",
                        dimension.as_str()
                    ),
                )
            } else {
                CertificationCell::new(
                    profile,
                    *dimension,
                    CertificationCellStateClass::Pass,
                    fresh,
                    format!(
                        "Profile '{}' has current, in-window proof for '{}' bound to '{}'.",
                        profile.as_str(),
                        dimension.as_str(),
                        dimension.contract_ref()
                    ),
                )
            }
        })
        .collect();
    ProfileCertificationSnapshot::new(profile, cells)
}
