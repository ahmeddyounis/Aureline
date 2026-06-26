//! Capstone certification that ties the M5 content-design, controlled-vocabulary,
//! content-ops metadata, and commercial-boundary wording lanes into one
//! certifiable content-truth story per governed wording object.
//!
//! The product treats writing principles, controlled vocabulary, action-label and
//! error-copy patterns, AI copy guardrails, content-ops metadata, and
//! commercial-boundary honesty as governed contracts, not copy polish. Each of
//! those concepts already has its own frozen catalog and proof lane; the frozen
//! [content-wording matrix][matrix] enumerates them as eight governed object
//! kinds. This lane is the final row: it certifies, for every governed wording
//! object, that its protected wording is **currently proven** — its proof packet
//! is fresh, its controlled terms render in parity across every consumer surface,
//! and its content-ops metadata is complete — and it auto-narrows any object whose
//! proof is stale, whose wording drifted, or whose metadata is missing before the
//! object can keep a Stable wording claim.
//!
//! Three records carry the truth:
//!
//! - the per-object **certification row** ([`ContentCertificationRow`]): one row
//!   per [`M5ContentObjectKind`] naming the object's protected concept, the proof
//!   packets that keep it current, its proof freshness, copy-parity, and
//!   content-ops metadata posture, the consumer surfaces it must stay aligned
//!   across, any active waiver, and a derived green/yellow/red
//!   [`ContentRowStatus`].
//! - the release **certification packet**
//!   ([`ContentDesignCertificationPacket`]): the full set of rows with derived
//!   per-row status, aggregated green/yellow/red counts, the active waivers, the
//!   exact stale-proof causes ([`StaleProofCause`]), and the blocking findings the
//!   lane refuses to ship with.
//! - the **content-truth dashboard** ([`ContentTruthDashboard`]): a light
//!   projection of the packet that release / public-truth automation reads to
//!   auto-narrow marketed wording rows when evidence, copy-parity, or metadata
//!   freshness falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to
//! `yellow` the moment its frozen qualification is below Stable, its proof is a
//! disclosed cache/warming/waivered-stale posture, its wording carries a disclosed
//! drift, or its content-ops metadata is partial; it drops to `red` if it hides a
//! wording drift, loses content-ops metadata, claims Stable on stale or
//! unverified proof with no waiver, or claims Stable with no backing proof at all.
//! That derivation is the auto-narrowing the acceptance criteria require: a
//! marketed wording row cannot keep a green content-truth claim once its
//! underlying proof goes stale, missing, or drifted.
//!
//! The records are inspectable, serde-serializable truth packets that carry no
//! raw message bodies, raw provider payloads, credentials, or untranslated
//! free-text prose — only stable ids, closed vocabulary, counts, refs, and short
//! labels. They are consumed by the headless inspector
//! (`aureline_shell_m5_content_design_certification`), the support-export wrapper,
//! the docs page under [`M5_CONTENT_CERT_PUBLISHED_DOC_REF`], the published report
//! and dashboard under [`M5_CONTENT_CERT_PUBLISHED_REPORT_REF`] and
//! [`M5_CONTENT_CERT_PUBLISHED_DASHBOARD_REF`], and the boundary schema
//! [`M5_CONTENT_CERT_SOURCE_SCHEMA_REF`].
//!
//! The object-kind, qualification, consumer-surface, downgrade-trigger, and
//! freshness vocabulary is re-exported by reference from the already-frozen
//! [content-wording matrix][matrix]; the certified object rows are pulled straight
//! from that matrix's seeded packet, so this lane mints no parallel wording
//! vocabulary and cannot drift from the contracts it certifies. Only the
//! certification-specific vocabulary ([`ContentRowStatus`], [`CopyParityState`],
//! [`ContentOpsMetadataState`], [`ContentCertificationWaiver`], [`StaleProofCause`],
//! [`ContentCertificationFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix as matrix;

pub use matrix::{
    ContentFreshnessState, M5ContentConsumerSurface, M5ContentDowngradeTrigger,
    M5ContentObjectKind, M5ContentQualificationClass,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_content_design_certification_packet,
    seeded_content_design_certification_packet_ai_overclaim_blocked,
    seeded_content_design_certification_packet_content_ops_stale, SEED_BUILD_IDENTITY_REF,
    SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_CONTENT_CERT_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_CONTENT_CERT_SHARED_CONTRACT_REF: &str = "shell:m5_content_design_certification:v1";

/// Stable record kind for [`ContentDesignCertificationPacket`] payloads.
pub const M5_CONTENT_CERT_PACKET_RECORD_KIND: &str =
    "shell_m5_content_design_certification_packet_record";

/// Stable record kind for [`ContentTruthDashboard`] payloads.
pub const M5_CONTENT_CERT_DASHBOARD_RECORD_KIND: &str = "shell_m5_content_truth_dashboard_record";

/// Stable record kind for [`ContentDesignCertificationSupportExport`] payloads.
pub const M5_CONTENT_CERT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_content_design_certification_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_CONTENT_CERT_PACKET_ID: &str = "m5-content-design-certification:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_CONTENT_CERT_DASHBOARD_ID: &str = "m5-content-truth-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_CONTENT_CERT_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-content-design-certification:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_CONTENT_CERT_SOURCE_SCHEMA_REF: &str =
    "schemas/release/m5-content-design-certification.schema.json";

/// Published markdown report ref reviewers reopen the certification from.
pub const M5_CONTENT_CERT_PUBLISHED_REPORT_REF: &str =
    "artifacts/release/m5-content-design-certification/m5_content_design_certification.md";

/// Published support-export artifact ref.
pub const M5_CONTENT_CERT_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-content-design-certification/support_export.json";

/// Published content-truth dashboard ref the public-truth automation consumes.
pub const M5_CONTENT_CERT_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/content/m5-content-truth-dashboard.json";

/// Published companion doc ref.
pub const M5_CONTENT_CERT_PUBLISHED_DOC_REF: &str =
    "docs/release/m5-content-design-certification.md";

/// Repo-relative ref to the frozen content-wording matrix schema.
pub const M5_CONTENT_CERT_MATRIX_SCHEMA_REF: &str = matrix::M5_CONTENT_WORDING_MATRIX_SCHEMA_REF;

/// Every governed content object the certification must cover, in canonical order.
///
/// These are exactly the object kinds the frozen content-wording matrix freezes;
/// the lane certifies none beyond them and refuses to ship if any is missing.
pub const REQUIRED_OBJECT_KINDS: [M5ContentObjectKind; 8] = M5ContentObjectKind::ALL;

/// The derived content-truth light a governed wording object carries.
///
/// `green` means the object's protected wording is currently proven at full
/// standing. `yellow` is a disclosed narrowing (the object is honestly narrowed
/// below Stable, runs on a disclosed cache/warming/waivered-stale proof, discloses
/// a wording drift, or carries partial content-ops metadata). `red` is blocked:
/// the object hides a wording drift, lost content-ops metadata, claims Stable on
/// stale/unverified/unbacked proof, and may not keep a marketed wording claim
/// until it is repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentRowStatus {
    /// The protected wording is currently proven at full standing.
    Green,
    /// The wording claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The wording claim is blocked and may not be marketed until repaired.
    Red,
}

impl ContentRowStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// `true` when the row keeps a publishable (green or yellow) wording claim.
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Green | Self::Yellow)
    }
}

/// The copy-parity posture of a governed object across its consumer surfaces.
///
/// `in_parity` means every consumer surface renders the same controlled wording.
/// `disclosed_drift` means a known wording difference exists and is disclosed
/// (and must be waivered). `undisclosed_drift` is a hidden difference — always a
/// blocker the surface cannot market past.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyParityState {
    /// Every consumer surface renders the same controlled wording.
    InParity,
    /// A known wording difference exists and is disclosed (and waivered).
    DisclosedDrift,
    /// A wording difference exists with no disclosure — a blocker.
    UndisclosedDrift,
}

impl CopyParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InParity => "in_parity",
            Self::DisclosedDrift => "disclosed_drift",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }

    /// `true` when the parity is at full standing.
    pub const fn is_in_parity(self) -> bool {
        matches!(self, Self::InParity)
    }
}

/// The content-ops metadata posture of a governed object.
///
/// `complete` means version/source/compatibility metadata is present.
/// `partial` discloses that some metadata is missing. `missing` means required
/// metadata is absent — a blocker for the content-ops object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentOpsMetadataState {
    /// Version / source / compatibility metadata is present.
    Complete,
    /// Some metadata is missing and disclosed.
    Partial,
    /// Required metadata is absent.
    Missing,
}

impl ContentOpsMetadataState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Missing => "missing",
        }
    }

    /// `true` when the metadata is complete.
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Short, reviewer-facing label for a governed content object's protected concept.
pub const fn protected_concept_label(kind: M5ContentObjectKind) -> &'static str {
    match kind {
        M5ContentObjectKind::SafetyCriticalUiString => "Safety-critical UI strings",
        M5ContentObjectKind::GlossaryTerm => "Controlled glossary terms",
        M5ContentObjectKind::ActionLabelPattern => "Verb-first action labels",
        M5ContentObjectKind::ErrorRecoveryBlock => "Error / recovery copy",
        M5ContentObjectKind::AiCopyGuardrail => "AI copy guardrails",
        M5ContentObjectKind::CountScopePhraseSet => "Count / scope language",
        M5ContentObjectKind::ContentOpsArtifact => "Content-ops metadata",
        M5ContentObjectKind::CommercialBoundaryWording => "Commercial-boundary wording",
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red proof or
/// copy-parity posture stay narrowed (yellow) rather than blocked — never lets a
/// hidden drift, missing metadata, or unbacked claim hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentCertificationWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The governed object the waiver applies to.
    pub object_kind: M5ContentObjectKind,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row
    /// blocks.
    pub expires_at: String,
}

impl ContentCertificationWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed object's wording claim.
///
/// The trigger token mirrors the frozen [`M5ContentDowngradeTrigger`] vocabulary
/// so a cause never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleProofCause {
    /// The governed object the cause applies to.
    pub object_kind: M5ContentObjectKind,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5ContentDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a
    /// non-disclosed cause is a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl StaleProofCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed wording object, certified across its proof, parity, and metadata
/// posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentCertificationRow {
    /// The governed object being certified.
    pub object_kind: M5ContentObjectKind,
    /// The object's frozen qualification class from the content-wording matrix.
    pub matrix_qualification: M5ContentQualificationClass,
    /// Owner role accountable for keeping this object's wording governed.
    pub owner_role: String,
    /// Short protected-concept label.
    pub protected_concept: String,
    /// Proof packet refs that keep this object current. Pulled from the matrix.
    pub proof_packet_refs: Vec<String>,
    /// RFC 3339 timestamp of the last proof refresh for this object.
    pub last_proof_refresh: String,
    /// Proof freshness posture.
    pub proof_freshness: ContentFreshnessState,
    /// Copy-parity posture across the object's consumer surfaces.
    pub copy_parity: CopyParityState,
    /// Content-ops metadata posture.
    pub metadata_state: ContentOpsMetadataState,
    /// Consumer surfaces this object must stay aligned across. Pulled from the
    /// matrix.
    pub consumer_surfaces: Vec<M5ContentConsumerSurface>,
    /// Downgrade triggers that apply to this object. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5ContentDowngradeTrigger>,
    /// Active waiver, when a disclosed narrowing is in force.
    pub active_waiver: Option<ContentCertificationWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: ContentRowStatus,
    /// The exact stale-proof causes that narrowed or blocked this row.
    pub stale_proof_causes: Vec<StaleProofCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl ContentCertificationRow {
    /// `true` when this object's frozen qualification keeps a Stable claim.
    pub fn is_stable_qualified(&self) -> bool {
        self.matrix_qualification.is_stable()
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// The drift trigger this object reports for a copy-parity drift.
    fn parity_drift_trigger(&self) -> M5ContentDowngradeTrigger {
        match self.object_kind {
            M5ContentObjectKind::CommercialBoundaryWording => {
                M5ContentDowngradeTrigger::CommercialBoundaryDrift
            }
            _ => M5ContentDowngradeTrigger::ControlledTermDrift,
        }
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        // A hidden wording drift always blocks.
        if matches!(self.copy_parity, CopyParityState::UndisclosedDrift) {
            return true;
        }
        // The content-ops object losing its metadata always blocks.
        if matches!(self.object_kind, M5ContentObjectKind::ContentOpsArtifact)
            && matches!(self.metadata_state, ContentOpsMetadataState::Missing)
        {
            return true;
        }
        if self.is_stable_qualified() {
            // A Stable claim on unverified proof, or with no backing proof at all,
            // always blocks.
            if matches!(self.proof_freshness, ContentFreshnessState::Unverified) {
                return true;
            }
            if self.proof_packet_refs.is_empty() {
                return true;
            }
            // A Stable claim on stale proof blocks unless a waiver discloses it.
            if matches!(self.proof_freshness, ContentFreshnessState::Stale)
                && !self.has_active_waiver()
            {
                return true;
            }
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        !self.is_stable_qualified()
            || matches!(
                self.proof_freshness,
                ContentFreshnessState::Cached
                    | ContentFreshnessState::Warming
                    | ContentFreshnessState::Stale
                    | ContentFreshnessState::Unverified
            )
            || matches!(self.copy_parity, CopyParityState::DisclosedDrift)
            || matches!(
                self.metadata_state,
                ContentOpsMetadataState::Partial | ContentOpsMetadataState::Missing
            )
    }

    /// Recomputes the derived status from the proof, parity, and metadata posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest
    /// narrowing forces `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> ContentRowStatus {
        if self.has_hard_blocker() {
            ContentRowStatus::Red
        } else if self.has_narrowing() {
            ContentRowStatus::Yellow
        } else {
            ContentRowStatus::Green
        }
    }

    /// Recomputes the exact stale-proof causes for the row, in deterministic
    /// order (qualification, freshness, parity, metadata).
    pub fn recompute_causes(&self) -> Vec<StaleProofCause> {
        let mut causes = Vec::new();
        if !self.is_stable_qualified() {
            causes.push(StaleProofCause {
                object_kind: self.object_kind,
                trigger: M5ContentDowngradeTrigger::UpstreamDependencyNarrowed,
                disclosed: true,
                detail: format!(
                    "Frozen content-wording matrix qualifies this object at `{}`, below a Stable wording claim.",
                    self.matrix_qualification.as_str()
                ),
            });
        }
        match self.proof_freshness {
            ContentFreshnessState::ProvenCurrent => {}
            ContentFreshnessState::Cached => causes.push(StaleProofCause {
                object_kind: self.object_kind,
                trigger: M5ContentDowngradeTrigger::FreshnessExpired,
                disclosed: true,
                detail: "Proof shown with a disclosed cache posture.".to_owned(),
            }),
            ContentFreshnessState::Warming => causes.push(StaleProofCause {
                object_kind: self.object_kind,
                trigger: M5ContentDowngradeTrigger::FreshnessExpired,
                disclosed: true,
                detail: "Proof is warming and not yet complete.".to_owned(),
            }),
            ContentFreshnessState::Stale => causes.push(StaleProofCause {
                object_kind: self.object_kind,
                trigger: M5ContentDowngradeTrigger::ProofStale,
                disclosed: self.has_active_waiver(),
                detail: "Proof packet has gone stale past its freshness floor.".to_owned(),
            }),
            ContentFreshnessState::Unverified => causes.push(StaleProofCause {
                object_kind: self.object_kind,
                trigger: M5ContentDowngradeTrigger::FreshnessExpired,
                disclosed: false,
                detail: "Proof freshness could not be verified.".to_owned(),
            }),
        }
        match self.copy_parity {
            CopyParityState::InParity => {}
            CopyParityState::DisclosedDrift => causes.push(StaleProofCause {
                object_kind: self.object_kind,
                trigger: self.parity_drift_trigger(),
                disclosed: true,
                detail: "Disclosed wording drift across consumer surfaces, held under a waiver."
                    .to_owned(),
            }),
            CopyParityState::UndisclosedDrift => causes.push(StaleProofCause {
                object_kind: self.object_kind,
                trigger: self.parity_drift_trigger(),
                disclosed: false,
                detail: "Undisclosed wording drift across consumer surfaces.".to_owned(),
            }),
        }
        match self.metadata_state {
            ContentOpsMetadataState::Complete => {}
            ContentOpsMetadataState::Partial => causes.push(StaleProofCause {
                object_kind: self.object_kind,
                trigger: M5ContentDowngradeTrigger::ContentOpsMetadataMissing,
                disclosed: true,
                detail: "Some version/source metadata is missing and disclosed.".to_owned(),
            }),
            ContentOpsMetadataState::Missing => causes.push(StaleProofCause {
                object_kind: self.object_kind,
                trigger: M5ContentDowngradeTrigger::ContentOpsMetadataMissing,
                disclosed: false,
                detail: "Required version/source metadata is absent.".to_owned(),
            }),
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed wording drift, or a Stable-qualified object running on stale
    /// proof, may only stay yellow (rather than red) when a waiver discloses it.
    pub fn requires_waiver(&self) -> bool {
        matches!(self.copy_parity, CopyParityState::DisclosedDrift)
            || (self.is_stable_qualified()
                && matches!(self.proof_freshness, ContentFreshnessState::Stale))
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<ContentCertificationFinding> {
        let mut findings = Vec::new();
        let object = self.object_kind.as_str().to_owned();

        if self.is_stable_qualified() && self.proof_packet_refs.is_empty() {
            findings.push(ContentCertificationFinding::RowMissingProof {
                object_kind: object.clone(),
            });
        }
        if matches!(self.copy_parity, CopyParityState::UndisclosedDrift) {
            findings.push(ContentCertificationFinding::UndisclosedCopyDrift {
                object_kind: object.clone(),
            });
        }
        if matches!(self.object_kind, M5ContentObjectKind::ContentOpsArtifact)
            && matches!(self.metadata_state, ContentOpsMetadataState::Missing)
        {
            findings.push(ContentCertificationFinding::MissingContentOpsMetadata {
                object_kind: object.clone(),
            });
        }
        if self.is_stable_qualified()
            && matches!(self.proof_freshness, ContentFreshnessState::Stale)
            && !self.has_active_waiver()
        {
            findings.push(ContentCertificationFinding::StaleProofWithoutWaiver {
                object_kind: object.clone(),
            });
        }
        if self.is_stable_qualified()
            && matches!(self.proof_freshness, ContentFreshnessState::Unverified)
        {
            findings.push(ContentCertificationFinding::UnverifiedProofOnStableRow {
                object_kind: object.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, ContentRowStatus::Green) && !self.has_reason() {
            findings.push(ContentCertificationFinding::NarrowedRowWithoutReason {
                object_kind: object.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must
        // carry an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(ContentCertificationFinding::NarrowedRowWithoutWaiver {
                object_kind: object.clone(),
            });
        }
        // An attached waiver must still be active and must point at this object.
        if let Some(waiver) = &self.active_waiver {
            if waiver.object_kind != self.object_kind {
                findings.push(ContentCertificationFinding::WaiverObjectMismatch {
                    object_kind: object.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(ContentCertificationFinding::WaiverExpired {
                    object_kind: object.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(ContentCertificationFinding::RowStatusStale {
                object_kind: object.clone(),
            });
        }
        if self.stale_proof_causes != self.recompute_causes() {
            findings.push(ContentCertificationFinding::RowCausesStale {
                object_kind: object,
            });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} fresh={} parity={} meta={} waiver={}",
            self.object_kind.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.proof_freshness.as_str(),
            self.copy_parity.as_str(),
            self.metadata_state.as_str(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the content-design certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum ContentCertificationFinding {
    /// A governed wording object has no certification row.
    ObjectKindMissing {
        /// The missing object-kind token.
        object_kind: String,
    },
    /// A Stable-qualified row carries no proof packet refs.
    RowMissingProof {
        /// The object-kind token.
        object_kind: String,
    },
    /// A row hides a wording drift across its consumer surfaces.
    UndisclosedCopyDrift {
        /// The object-kind token.
        object_kind: String,
    },
    /// The content-ops object lost its required version/source metadata.
    MissingContentOpsMetadata {
        /// The object-kind token.
        object_kind: String,
    },
    /// A Stable-qualified row claims current wording on stale proof with no waiver.
    StaleProofWithoutWaiver {
        /// The object-kind token.
        object_kind: String,
    },
    /// A Stable-qualified row claims current wording on unverified proof.
    UnverifiedProofOnStableRow {
        /// The object-kind token.
        object_kind: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The object-kind token.
        object_kind: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The object-kind token.
        object_kind: String,
    },
    /// An attached waiver does not point at the row's object.
    WaiverObjectMismatch {
        /// The object-kind token.
        object_kind: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The object-kind token.
        object_kind: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The object-kind token.
        object_kind: String,
    },
    /// The declared stale-proof causes do not match the recomputed causes.
    RowCausesStale {
        /// The object-kind token.
        object_kind: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered object kinds do not match the rows.
    CoverageStale,
}

impl ContentCertificationFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::ObjectKindMissing { .. } => "object_kind_missing",
            Self::RowMissingProof { .. } => "row_missing_proof",
            Self::UndisclosedCopyDrift { .. } => "undisclosed_copy_drift",
            Self::MissingContentOpsMetadata { .. } => "missing_content_ops_metadata",
            Self::StaleProofWithoutWaiver { .. } => "stale_proof_without_waiver",
            Self::UnverifiedProofOnStableRow { .. } => "unverified_proof_on_stable_row",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverObjectMismatch { .. } => "waiver_object_mismatch",
            Self::WaiverExpired { .. } => "waiver_expired",
            Self::RowStatusStale { .. } => "row_status_stale",
            Self::RowCausesStale { .. } => "row_causes_stale",
            Self::StatusCountsStale => "status_counts_stale",
            Self::CoverageStale => "coverage_stale",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::ObjectKindMissing { object_kind }
            | Self::RowMissingProof { object_kind }
            | Self::UndisclosedCopyDrift { object_kind }
            | Self::MissingContentOpsMetadata { object_kind }
            | Self::StaleProofWithoutWaiver { object_kind }
            | Self::UnverifiedProofOnStableRow { object_kind }
            | Self::NarrowedRowWithoutReason { object_kind }
            | Self::NarrowedRowWithoutWaiver { object_kind }
            | Self::WaiverObjectMismatch { object_kind, .. }
            | Self::WaiverExpired { object_kind, .. }
            | Self::RowStatusStale { object_kind }
            | Self::RowCausesStale { object_kind } => object_kind,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
        }
    }
}

/// The release certification packet shared by the release / public-truth
/// automation, the support-export wrapper, and the docs/help surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentDesignCertificationPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the packet.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable packet id used to pivot across surfaces.
    pub packet_id: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// The frozen content-wording matrix packet id this certification certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen content-wording matrix schema.
    pub matrix_schema_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// Per-object certification rows, in canonical order.
    pub rows: Vec<ContentCertificationRow>,
    /// Governed object kinds certified, in canonical (sorted) order.
    pub covered_object_kinds: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (fully proven) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<ContentCertificationWaiver>,
    /// Every exact stale-proof cause, in row then cause order.
    pub stale_proof_causes: Vec<StaleProofCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<ContentCertificationFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Release / public-truth automation refs that consume this packet to
    /// auto-narrow marketed wording rows.
    pub public_truth_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Claim-publication refs the row statuses feed.
    pub claim_publication_refs: Vec<String>,
    /// Docs/help refs the packet reopens from.
    pub docs_help_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published content-truth dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl ContentDesignCertificationPacket {
    /// Returns the certification row for `kind`, if present.
    pub fn row(&self, kind: M5ContentObjectKind) -> Option<&ContentCertificationRow> {
        self.rows.iter().find(|row| row.object_kind == kind)
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "packet: id={}, rows={}, green={}, yellow={}, red={}, clean={}",
                self.packet_id,
                self.row_count,
                self.green_row_count,
                self.yellow_row_count,
                self.red_row_count,
                self.report_clean,
            ),
            format!(
                "matrix={} build={} channel={} publishable={}",
                self.matrix_packet_ref,
                self.build_identity_ref,
                self.release_channel_class,
                self.all_rows_publishable,
            ),
        ];
        for row in &self.rows {
            lines.push(row.compact_line());
        }
        for waiver in &self.active_waivers {
            lines.push(format!(
                "  waiver {} -> {} (expires {})",
                waiver.waiver_id,
                waiver.object_kind.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.stale_proof_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.object_kind.as_str(),
                cause.cause_token(),
                cause.disclosed
            ));
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "  blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref()
            ));
        }
        lines
    }

    /// Projects the light content-truth dashboard the public-truth automation
    /// consumes.
    pub fn dashboard(&self) -> ContentTruthDashboard {
        ContentTruthDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 content-design certification packet serializes")
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 content-design certification\n\n");
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::content_design_certification`](../../../crates/aureline-shell/src/content_design_certification/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_content_design_certification -- markdown > \\\n  artifacts/release/m5-content-design-certification/m5_content_design_certification.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Certifies matrix packet: `{}`\n",
            self.matrix_packet_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!(
            "- Release channel: `{}`\n",
            self.release_channel_class
        ));
        out.push_str(&format!("- Rows certified: {}\n", self.row_count));
        out.push_str(&format!(
            "- Green (fully proven): {}\n",
            self.green_row_count
        ));
        out.push_str(&format!(
            "- Yellow (auto-narrowed): {}\n",
            self.yellow_row_count
        ));
        out.push_str(&format!("- Red (blocked): {}\n", self.red_row_count));
        out.push_str(&format!(
            "- All rows publishable: `{}`\n",
            self.all_rows_publishable
        ));
        out.push_str(&format!(
            "- Blocking findings: {}\n",
            self.blocking_findings.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Content-truth rows\n\n");
        out.push_str(
            "| Protected concept | Status | Qualification | Proof freshness | Copy parity | Metadata | Waiver |\n\
             | ----------------- | ------ | ------------- | --------------- | ----------- | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.protected_concept,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.proof_freshness.as_str(),
                row.copy_parity.as_str(),
                row.metadata_state.as_str(),
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&ContentCertificationRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, ContentRowStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str("None — every governed wording object certifies green.\n\n");
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.object_kind.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact stale-proof causes\n\n");
        if self.stale_proof_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.stale_proof_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.object_kind.as_str(),
                    cause.cause_token(),
                    cause.disclosed,
                    cause.detail,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Active waivers\n\n");
        if self.active_waivers.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for waiver in &self.active_waivers {
                out.push_str(&format!(
                    "- `{}` (`{}`, owner: {}, expires `{}`) — {}\n",
                    waiver.waiver_id,
                    waiver.object_kind.as_str(),
                    waiver.owner_role,
                    waiver.expires_at,
                    waiver.reason,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Findings\n\n");
        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_content_design_certification -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_content_design_certification_fixtures\n",
        );
        out.push_str("python3 tools/ci/m5/content_design_certification_check.py --repo-root .\n");
        out.push_str("```\n");
        out
    }
}

/// One row of the light content-truth dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentTruthDashboardRow {
    /// The governed object.
    pub object_kind: M5ContentObjectKind,
    /// Short protected-concept label.
    pub protected_concept: String,
    /// Derived green/yellow/red status.
    pub status: ContentRowStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5ContentQualificationClass,
    /// Proof freshness posture.
    pub proof_freshness: ContentFreshnessState,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// Copy-parity posture.
    pub copy_parity: CopyParityState,
    /// Content-ops metadata posture.
    pub metadata_state: ContentOpsMetadataState,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light content-truth dashboard the release / public-truth automation reads
/// to auto-narrow marketed wording rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentTruthDashboard {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the dashboard.
    pub schema_version: u32,
    /// Stable dashboard id.
    pub dashboard_id: String,
    /// The packet id this dashboard projects.
    pub source_packet_ref: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Dashboard rows, in canonical order.
    pub rows: Vec<ContentTruthDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Release / public-truth automation refs that consume the dashboard.
    pub public_truth_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl ContentTruthDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &ContentDesignCertificationPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| ContentTruthDashboardRow {
                object_kind: row.object_kind,
                protected_concept: row.protected_concept.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                proof_freshness: row.proof_freshness,
                last_proof_refresh: row.last_proof_refresh.clone(),
                copy_parity: row.copy_parity,
                metadata_state: row.metadata_state,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .stale_proof_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_CONTENT_CERT_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_CONTENT_CERT_SCHEMA_VERSION,
            dashboard_id: M5_CONTENT_CERT_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            public_truth_refs: packet.public_truth_refs.clone(),
            generated_at: packet.generated_at.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 content-truth dashboard serializes")
    }
}

/// Support-export wrapper for the content-design certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentDesignCertificationSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: ContentDesignCertificationPacket,
    /// Dashboard quoted in full.
    pub dashboard: ContentTruthDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl ContentDesignCertificationSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each object
    /// kind, each proof packet ref, and each active waiver id is quoted as a case
    /// id so a support reviewer — or the public-truth automation — can name the
    /// same object, proof, and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: ContentDesignCertificationPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.object_kind.as_str().to_owned());
            for proof_ref in &row.proof_packet_refs {
                case_ids.push(proof_ref.clone());
            }
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_CONTENT_CERT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_CONTENT_CERT_SCHEMA_VERSION,
            shared_contract_ref: M5_CONTENT_CERT_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_content_design_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentDesignCertificationInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen content-wording matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-object certification rows.
    pub rows: Vec<ContentCertificationRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Builds a [`ContentDesignCertificationPacket`] from the exact build identity,
/// the frozen matrix ref, and the per-object certification rows.
///
/// Each row's derived status and stale-proof causes, the aggregate counts, the
/// active waivers, and the blocking findings are recomputed here so the packet is
/// the single source of truth and the auto-narrowing cannot be asserted.
pub fn build_content_design_certification_packet(
    input: ContentDesignCertificationInput,
) -> ContentDesignCertificationPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is
    // self-consistent and the auto-narrowing is the single source of truth.
    let rows: Vec<ContentCertificationRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.stale_proof_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<ContentCertificationFinding> = Vec::new();

    // Every governed object must carry a certification row.
    let present: BTreeSet<M5ContentObjectKind> = rows.iter().map(|row| row.object_kind).collect();
    for kind in REQUIRED_OBJECT_KINDS {
        if !present.contains(&kind) {
            blocking_findings.push(ContentCertificationFinding::ObjectKindMissing {
                object_kind: kind.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_object_kinds: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|kind| kind.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ContentRowStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ContentRowStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ContentRowStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(ContentCertificationFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<ContentCertificationWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let stale_proof_causes: Vec<StaleProofCause> = rows
        .iter()
        .flat_map(|row| row.stale_proof_causes.clone())
        .collect();

    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    let report_clean = blocking_findings.is_empty();

    ContentDesignCertificationPacket {
        record_kind: M5_CONTENT_CERT_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_CONTENT_CERT_SCHEMA_VERSION,
        shared_contract_ref: M5_CONTENT_CERT_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_CONTENT_CERT_PACKET_ID.to_owned(),
        source_schema_ref: M5_CONTENT_CERT_SOURCE_SCHEMA_REF.to_owned(),
        headline: "One certifiable content-truth story for every governed M5 wording object: \
                   safety-critical strings, controlled glossary, action labels, error/recovery \
                   copy, AI copy guardrails, count/scope language, content-ops metadata, and \
                   commercial-boundary wording certified together, with each row's green/yellow/red \
                   claim auto-narrowed from its proof, copy-parity, and metadata posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_CONTENT_CERT_MATRIX_SCHEMA_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        rows,
        covered_object_kinds,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        stale_proof_causes,
        blocking_findings,
        report_clean,
        public_truth_refs: vec![
            "release_public_truth.content_design_certification".to_owned(),
            "release_public_truth.auto_narrow.content_truth_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.content_design_certification".to_owned(),
            "docs/release/m5-content-design-certification.md".to_owned(),
        ],
        claim_publication_refs: vec!["claim_publication.content_design_certification".to_owned()],
        docs_help_refs: vec![M5_CONTENT_CERT_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-content-design-certification".to_owned()],
        published_report_ref: M5_CONTENT_CERT_PUBLISHED_REPORT_REF.to_owned(),
        published_dashboard_ref: M5_CONTENT_CERT_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_CONTENT_CERT_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    }
}

/// Validation error produced by [`validate_content_design_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum ContentCertificationValidationError {
    /// The packet has no rows.
    NoRows,
    /// The packet's record kind is wrong.
    WrongRecordKind,
    /// The packet's schema version is wrong.
    WrongSchemaVersion,
    /// The packet's exact-build identity ref is empty.
    BuildIdentityRefMissing,
    /// The packet does not certify a frozen matrix packet.
    MatrixPacketRefMissing,
    /// The rows do not cover all eight governed object kinds.
    CoverageIncomplete,
    /// The declared covered object kinds do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared stale-proof causes do not match the recomputed causes.
    StaleProofCausesStale,
    /// The declared blocking findings do not match the recomputed findings.
    BlockingFindingsStale,
    /// A blocking finding remains in the packet.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning subject ref.
        subject_ref: String,
    },
    /// The published report ref is empty.
    PublishedReportRefMissing,
    /// The published dashboard ref is empty.
    PublishedDashboardRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a packet against the content-design certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed
/// wording object carries a current content-truth row; each row's status is the
/// derived auto-narrowed value, never asserted; a marketed (green) row cannot
/// keep a Stable wording claim while its proof is stale, unverified, or unbacked,
/// its wording drifted, or its content-ops metadata is missing; and a disclosed
/// narrowing is backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_content_design_certification_packet(
    packet: &ContentDesignCertificationPacket,
) -> Result<(), Vec<ContentCertificationValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(ContentCertificationValidationError::NoRows);
    }
    if packet.record_kind != M5_CONTENT_CERT_PACKET_RECORD_KIND {
        errors.push(ContentCertificationValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_CONTENT_CERT_SCHEMA_VERSION {
        errors.push(ContentCertificationValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(ContentCertificationValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(ContentCertificationValidationError::MatrixPacketRefMissing);
    }

    let present: BTreeSet<M5ContentObjectKind> =
        packet.rows.iter().map(|row| row.object_kind).collect();
    let coverage_complete = REQUIRED_OBJECT_KINDS
        .iter()
        .all(|kind| present.contains(kind));
    if !coverage_complete || packet.rows.len() != REQUIRED_OBJECT_KINDS.len() {
        errors.push(ContentCertificationValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|kind| kind.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_object_kinds {
        errors.push(ContentCertificationValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ContentRowStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ContentRowStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ContentRowStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(ContentCertificationValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<ContentCertificationWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(ContentCertificationValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<StaleProofCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.stale_proof_causes {
        errors.push(ContentCertificationValidationError::StaleProofCausesStale);
    }

    let mut recomputed: Vec<ContentCertificationFinding> = Vec::new();
    for kind in REQUIRED_OBJECT_KINDS {
        if !present.contains(&kind) {
            recomputed.push(ContentCertificationFinding::ObjectKindMissing {
                object_kind: kind.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(ContentCertificationFinding::StatusCountsStale);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(ContentCertificationValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(
            ContentCertificationValidationError::BlockingFindingPresent {
                class: finding.class_token().to_owned(),
                subject_ref: finding.subject_ref().to_owned(),
            },
        );
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(ContentCertificationValidationError::PublishedReportRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(ContentCertificationValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(ContentCertificationValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
