//! Capstone certification that ties the M5 post-install notice/provenance,
//! community-handoff, reproduction-packet, and device-permission/auth-boundary
//! surfaces into one certifiable boundary-truth story per governed object.
//!
//! The product treats post-install provenance disclosure, official-versus-community
//! handoff, redaction-safe reproduction packets, offline-capture continuity, and
//! device/mic/webview/auth boundary honesty as governed contracts, not help copy.
//! Each of those concepts is already frozen as one governed object kind by the
//! [public-handoff / capture-boundary matrix][matrix]. This lane is the final row:
//! it certifies, for every governed object, that its boundary truth is **currently
//! proven** — its disclosure/notice freshness is fresh, its boundary is honestly
//! disclosed across every consumer surface (never impersonating native trusted
//! product chrome), and its redaction/offline-continuity posture keeps raw
//! sensitive material from leaving — and it auto-narrows any object whose proof is
//! stale, whose boundary drifted, or whose redaction is unsafe before the object
//! can keep a Stable public claim.
//!
//! Three records carry the truth:
//!
//! - the per-object **certification row** ([`HandoffCertificationRow`]): one row per
//!   [`M5HandoffObjectKind`] naming the object's certified surface, the proof
//!   packets that keep it current, its disclosure freshness, boundary-honesty, and
//!   redaction-readiness posture, the consumer surfaces it must stay aligned across,
//!   any active waiver, and a derived green/yellow/red [`HandoffCertStatus`].
//! - the release **certification packet**
//!   ([`PublicHandoffCertificationPacket`]): the full set of rows with derived
//!   per-row status, aggregated green/yellow/red counts, the active waivers, the
//!   exact stale-proof causes ([`HandoffStaleProofCause`]), and the blocking
//!   findings the lane refuses to ship with.
//! - the **boundary-truth dashboard** ([`HandoffTruthDashboard`]): a light
//!   projection release / help / support / public-truth automation reads to
//!   auto-narrow claimed surfaces when disclosure, boundary, or redaction proof
//!   falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to
//! `yellow` the moment its frozen qualification is below Stable, its proof is a
//! disclosed cache/warming/waivered-stale/unverified posture, its boundary carries a
//! disclosed gap, or its redaction is partial; it drops to `red` if it hides a
//! native-chrome impersonation, would let raw sensitive material leave, claims
//! Stable on stale or unverified proof with no waiver, or claims Stable with no
//! backing proof at all. That derivation is the auto-narrowing the acceptance
//! criteria require: a claimed surface cannot keep a green public claim once its
//! underlying proof goes stale, missing, drifted, or unsafe.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw
//! URLs, raw local paths, raw usernames, raw hostnames, tokens, raw diagnostics,
//! credentials, or user text bodies — only stable ids, closed vocabulary, counts,
//! refs, and short labels. The object-kind, qualification, consumer-surface,
//! downgrade-trigger, and freshness vocabulary is re-exported by reference from the
//! already-frozen [matrix]; the certified object rows are pulled straight from that
//! matrix's seeded packet, so this lane mints no parallel handoff vocabulary and
//! cannot drift from the contracts it certifies. Only the certification-specific
//! vocabulary ([`HandoffCertStatus`], [`BoundaryHonestyState`],
//! [`RedactionReadinessState`], [`HandoffCertificationWaiver`],
//! [`HandoffStaleProofCause`], [`HandoffCertificationFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_public_handoff_and_capture_boundary_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_public_handoff_and_capture_boundary_matrix as matrix;

pub use matrix::{
    HandoffNoticeFreshnessState, M5HandoffConsumerSurface, M5HandoffDowngradeTrigger,
    M5HandoffObjectKind, M5HandoffQualificationClass,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_public_handoff_certification_packet,
    seeded_public_handoff_certification_packet_embedded_impersonation_blocked,
    seeded_public_handoff_certification_packet_repro_redaction_unsafe,
    seeded_public_handoff_certification_packet_service_health_stale, SEED_BUILD_IDENTITY_REF,
    SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_HANDOFF_CERT_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_HANDOFF_CERT_SHARED_CONTRACT_REF: &str = "shell:m5_public_handoff_certification:v1";

/// Stable record kind for [`PublicHandoffCertificationPacket`] payloads.
pub const M5_HANDOFF_CERT_PACKET_RECORD_KIND: &str =
    "shell_m5_public_handoff_certification_packet_record";

/// Stable record kind for [`HandoffTruthDashboard`] payloads.
pub const M5_HANDOFF_CERT_DASHBOARD_RECORD_KIND: &str = "shell_m5_public_handoff_truth_dashboard_record";

/// Stable record kind for [`PublicHandoffCertificationSupportExport`] payloads.
pub const M5_HANDOFF_CERT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_public_handoff_certification_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_HANDOFF_CERT_PACKET_ID: &str = "m5-public-handoff-certification:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_HANDOFF_CERT_DASHBOARD_ID: &str = "m5-public-handoff-truth-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_HANDOFF_CERT_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-public-handoff-certification:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_HANDOFF_CERT_SOURCE_SCHEMA_REF: &str =
    "schemas/help/m5-public-handoff-certification.schema.json";

/// Published markdown report ref reviewers reopen the certification from.
pub const M5_HANDOFF_CERT_PUBLISHED_REPORT_REF: &str =
    "artifacts/help/m5-public-handoff-certification.md";

/// Published certification-packet artifact ref.
pub const M5_HANDOFF_CERT_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-public-handoff-proof/packet.json";

/// Published boundary-truth dashboard ref the public-truth automation consumes.
pub const M5_HANDOFF_CERT_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-public-handoff-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_HANDOFF_CERT_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-public-handoff-proof/support_export.json";

/// Published companion doc ref.
pub const M5_HANDOFF_CERT_PUBLISHED_DOC_REF: &str =
    "docs/help/m5_public_handoff_certification_contract.md";

/// Repo-relative ref to the frozen public-handoff matrix schema.
pub const M5_HANDOFF_CERT_MATRIX_SCHEMA_REF: &str = matrix::M5_PUBLIC_HANDOFF_MATRIX_SCHEMA_REF;

/// Every governed handoff / capture-boundary object the certification must cover,
/// in canonical order.
///
/// These are exactly the object kinds the frozen public-handoff matrix freezes;
/// the lane certifies none beyond them and refuses to ship if any is missing.
pub const REQUIRED_OBJECT_KINDS: [M5HandoffObjectKind; 8] = M5HandoffObjectKind::ALL;

/// The derived boundary-truth light a governed handoff object carries.
///
/// `green` means the object's boundary truth is currently proven at full standing.
/// `yellow` is a disclosed narrowing (the object is honestly narrowed below Stable,
/// runs on a disclosed cache/warming/waivered-stale/unverified proof, discloses a
/// boundary gap, or carries a partial redaction posture). `red` is blocked: the
/// object hides a native-chrome impersonation, would let raw sensitive material
/// leave, claims Stable on stale/unverified/unbacked proof, and may not keep a
/// public claim until it is repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffCertStatus {
    /// The boundary truth is currently proven at full standing.
    Green,
    /// The public claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The public claim is blocked and may not be published until repaired.
    Red,
}

impl HandoffCertStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// `true` when the row keeps a publishable (green or yellow) claim.
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Green | Self::Yellow)
    }
}

/// The boundary-honesty posture of a governed object across its consumer surfaces.
///
/// `honestly_disclosed` means every surface presents the same honest boundary — a
/// community route labeled community, a provenance labeled by source class, an
/// embedded/auth surface clearly labeled rather than posing as native chrome.
/// `disclosed_gap` means a known boundary-labeling difference exists and is disclosed
/// (and must be waivered). `undisclosed_impersonation` is a hidden boundary that
/// would impersonate native trusted product chrome or imply an unearned authority —
/// always a blocker the surface cannot publish past.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryHonestyState {
    /// Every consumer surface presents the same honest boundary.
    HonestlyDisclosed,
    /// A known boundary-labeling difference exists and is disclosed (and waivered).
    DisclosedGap,
    /// A hidden boundary would impersonate native chrome or imply unearned
    /// authority — a blocker.
    UndisclosedImpersonation,
}

impl BoundaryHonestyState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HonestlyDisclosed => "honestly_disclosed",
            Self::DisclosedGap => "disclosed_gap",
            Self::UndisclosedImpersonation => "undisclosed_impersonation",
        }
    }

    /// `true` when the boundary is honest at full standing.
    pub const fn is_honest(self) -> bool {
        matches!(self, Self::HonestlyDisclosed)
    }
}

/// The redaction / offline-continuity readiness posture of a governed object.
///
/// `proven` means share is previewed-and-redacted and offline capture survives a
/// failed handoff (or the object carries no shareable capture). `partial` discloses
/// that some redaction/continuity evidence is incomplete. `unsafe_material` means
/// raw sensitive material would leave, or offline capture would be lost — always a
/// blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionReadinessState {
    /// Share is previewed/redacted and offline capture survives a failed handoff.
    Proven,
    /// Some redaction/continuity evidence is incomplete and disclosed.
    Partial,
    /// Raw sensitive material would leave, or offline capture would be lost.
    UnsafeMaterial,
}

impl RedactionReadinessState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proven => "proven",
            Self::Partial => "partial",
            Self::UnsafeMaterial => "unsafe_material",
        }
    }

    /// `true` when redaction/continuity is proven.
    pub const fn is_proven(self) -> bool {
        matches!(self, Self::Proven)
    }
}

/// Short, reviewer-facing label for a governed object's certified surface.
pub const fn certified_surface_label(kind: M5HandoffObjectKind) -> &'static str {
    match kind {
        M5HandoffObjectKind::PostInstallNotice => "Post-install notice / provenance card",
        M5HandoffObjectKind::ProvenanceDisclosure => "Provenance / source-authenticity disclosure",
        M5HandoffObjectKind::CommunityHandoffRoute => "Official-vs-community handoff route",
        M5HandoffObjectKind::ReproductionPacket => "Redaction-safe reproduction packet",
        M5HandoffObjectKind::OfflineCaptureContinuity => "Offline-capture continuity",
        M5HandoffObjectKind::DevicePermissionBoundary => "Device / mic permission boundary",
        M5HandoffObjectKind::EmbeddedAuthBoundary => "Embedded webview / auth boundary",
        M5HandoffObjectKind::ServiceHealthNotice => "Release / service-health notice",
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red proof or boundary
/// posture stay narrowed (yellow) rather than blocked — never lets a hidden
/// impersonation, unsafe redaction, or unbacked claim hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffCertificationWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The governed object the waiver applies to.
    pub object_kind: M5HandoffObjectKind,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row
    /// blocks.
    pub expires_at: String,
}

impl HandoffCertificationWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed object's public claim.
///
/// The trigger token mirrors the frozen [`M5HandoffDowngradeTrigger`] vocabulary so
/// a cause never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffStaleProofCause {
    /// The governed object the cause applies to.
    pub object_kind: M5HandoffObjectKind,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5HandoffDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a
    /// non-disclosed cause is a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl HandoffStaleProofCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed handoff object, certified across its proof, boundary, and redaction
/// posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffCertificationRow {
    /// The governed object being certified.
    pub object_kind: M5HandoffObjectKind,
    /// The object's frozen qualification class from the public-handoff matrix.
    pub matrix_qualification: M5HandoffQualificationClass,
    /// Owner role accountable for keeping this object governed.
    pub owner_role: String,
    /// Short certified-surface label.
    pub certified_surface: String,
    /// Proof packet refs that keep this object current. Pulled from the matrix.
    pub proof_packet_refs: Vec<String>,
    /// RFC 3339 timestamp of the last proof refresh for this object.
    pub last_proof_refresh: String,
    /// Disclosure / notice freshness posture.
    pub disclosure_freshness: HandoffNoticeFreshnessState,
    /// Boundary-honesty posture across the object's consumer surfaces.
    pub boundary_honesty: BoundaryHonestyState,
    /// Redaction / offline-continuity readiness posture.
    pub redaction_readiness: RedactionReadinessState,
    /// Consumer surfaces this object must stay aligned across. Pulled from the
    /// matrix.
    pub consumer_surfaces: Vec<M5HandoffConsumerSurface>,
    /// Downgrade triggers that apply to this object. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5HandoffDowngradeTrigger>,
    /// Active waiver, when a disclosed narrowing is in force.
    pub active_waiver: Option<HandoffCertificationWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: HandoffCertStatus,
    /// The exact stale-proof causes that narrowed or blocked this row.
    pub stale_proof_causes: Vec<HandoffStaleProofCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl HandoffCertificationRow {
    /// `true` when this object's frozen qualification keeps a Stable claim.
    pub fn is_stable_qualified(&self) -> bool {
        self.matrix_qualification.is_stable()
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// The downgrade trigger this object reports for a boundary-honesty gap.
    fn boundary_trigger(&self) -> M5HandoffDowngradeTrigger {
        match self.object_kind {
            M5HandoffObjectKind::DevicePermissionBoundary
            | M5HandoffObjectKind::EmbeddedAuthBoundary => {
                M5HandoffDowngradeTrigger::NativeChromeImpersonation
            }
            M5HandoffObjectKind::PostInstallNotice
            | M5HandoffObjectKind::ProvenanceDisclosure => {
                M5HandoffDowngradeTrigger::ProvenanceUnverified
            }
            _ => M5HandoffDowngradeTrigger::RouteVisibilityUndeclared,
        }
    }

    /// The downgrade trigger this object reports for a redaction-readiness gap.
    fn redaction_trigger(&self) -> M5HandoffDowngradeTrigger {
        match self.object_kind {
            M5HandoffObjectKind::OfflineCaptureContinuity => {
                M5HandoffDowngradeTrigger::OfflineContinuityLost
            }
            _ => M5HandoffDowngradeTrigger::RedactionPreviewMissing,
        }
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        // A hidden native-chrome impersonation always blocks.
        if matches!(
            self.boundary_honesty,
            BoundaryHonestyState::UndisclosedImpersonation
        ) {
            return true;
        }
        // Raw sensitive material leaving (or lost offline capture) always blocks.
        if matches!(
            self.redaction_readiness,
            RedactionReadinessState::UnsafeMaterial
        ) {
            return true;
        }
        if self.is_stable_qualified() {
            // A Stable claim on unverified proof, or with no backing proof at all,
            // always blocks.
            if matches!(
                self.disclosure_freshness,
                HandoffNoticeFreshnessState::Unverified
            ) {
                return true;
            }
            if self.proof_packet_refs.is_empty() {
                return true;
            }
            // A Stable claim on stale proof blocks unless a waiver discloses it.
            if matches!(self.disclosure_freshness, HandoffNoticeFreshnessState::Stale)
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
                self.disclosure_freshness,
                HandoffNoticeFreshnessState::Cached
                    | HandoffNoticeFreshnessState::Warming
                    | HandoffNoticeFreshnessState::Stale
                    | HandoffNoticeFreshnessState::Unverified
            )
            || matches!(self.boundary_honesty, BoundaryHonestyState::DisclosedGap)
            || matches!(
                self.redaction_readiness,
                RedactionReadinessState::Partial | RedactionReadinessState::UnsafeMaterial
            )
    }

    /// Recomputes the derived status from the proof, boundary, and redaction
    /// posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest
    /// narrowing forces `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> HandoffCertStatus {
        if self.has_hard_blocker() {
            HandoffCertStatus::Red
        } else if self.has_narrowing() {
            HandoffCertStatus::Yellow
        } else {
            HandoffCertStatus::Green
        }
    }

    /// Recomputes the exact stale-proof causes for the row, in deterministic order
    /// (qualification, freshness, boundary, redaction).
    pub fn recompute_causes(&self) -> Vec<HandoffStaleProofCause> {
        let mut causes = Vec::new();
        if !self.is_stable_qualified() {
            causes.push(HandoffStaleProofCause {
                object_kind: self.object_kind,
                trigger: M5HandoffDowngradeTrigger::UpstreamDependencyNarrowed,
                disclosed: true,
                detail: format!(
                    "Frozen public-handoff matrix qualifies this object at `{}`, below a Stable public claim.",
                    self.matrix_qualification.as_str()
                ),
            });
        }
        match self.disclosure_freshness {
            HandoffNoticeFreshnessState::ProvenCurrent => {}
            HandoffNoticeFreshnessState::Cached => causes.push(HandoffStaleProofCause {
                object_kind: self.object_kind,
                trigger: M5HandoffDowngradeTrigger::NoticeStale,
                disclosed: true,
                detail: "Notice shown with a disclosed cache posture.".to_owned(),
            }),
            HandoffNoticeFreshnessState::Warming => causes.push(HandoffStaleProofCause {
                object_kind: self.object_kind,
                trigger: M5HandoffDowngradeTrigger::NoticeStale,
                disclosed: true,
                detail: "Notice is warming and not yet complete.".to_owned(),
            }),
            HandoffNoticeFreshnessState::Stale => causes.push(HandoffStaleProofCause {
                object_kind: self.object_kind,
                trigger: M5HandoffDowngradeTrigger::ProofStale,
                disclosed: self.has_active_waiver(),
                detail: "Disclosure/notice proof has gone stale past its freshness floor.".to_owned(),
            }),
            HandoffNoticeFreshnessState::Unverified => causes.push(HandoffStaleProofCause {
                object_kind: self.object_kind,
                trigger: M5HandoffDowngradeTrigger::ProvenanceUnverified,
                disclosed: false,
                detail: "Disclosure/notice freshness could not be verified.".to_owned(),
            }),
        }
        match self.boundary_honesty {
            BoundaryHonestyState::HonestlyDisclosed => {}
            BoundaryHonestyState::DisclosedGap => causes.push(HandoffStaleProofCause {
                object_kind: self.object_kind,
                trigger: self.boundary_trigger(),
                disclosed: true,
                detail: "Disclosed boundary-labeling gap across consumer surfaces, held under a waiver."
                    .to_owned(),
            }),
            BoundaryHonestyState::UndisclosedImpersonation => causes.push(HandoffStaleProofCause {
                object_kind: self.object_kind,
                trigger: self.boundary_trigger(),
                disclosed: false,
                detail: "Undisclosed boundary that would impersonate native chrome or imply unearned authority.".to_owned(),
            }),
        }
        match self.redaction_readiness {
            RedactionReadinessState::Proven => {}
            RedactionReadinessState::Partial => causes.push(HandoffStaleProofCause {
                object_kind: self.object_kind,
                trigger: self.redaction_trigger(),
                disclosed: true,
                detail: "Some redaction/offline-continuity evidence is incomplete and disclosed."
                    .to_owned(),
            }),
            RedactionReadinessState::UnsafeMaterial => causes.push(HandoffStaleProofCause {
                object_kind: self.object_kind,
                trigger: self.redaction_trigger(),
                disclosed: false,
                detail: "Raw sensitive material would leave, or offline capture would be lost."
                    .to_owned(),
            }),
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay
    /// publishable.
    ///
    /// A disclosed boundary gap, or a Stable-qualified object running on stale
    /// proof, may only stay yellow (rather than red) when a waiver discloses it.
    pub fn requires_waiver(&self) -> bool {
        matches!(self.boundary_honesty, BoundaryHonestyState::DisclosedGap)
            || (self.is_stable_qualified()
                && matches!(self.disclosure_freshness, HandoffNoticeFreshnessState::Stale))
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<HandoffCertificationFinding> {
        let mut findings = Vec::new();
        let object = self.object_kind.as_str().to_owned();

        if self.is_stable_qualified() && self.proof_packet_refs.is_empty() {
            findings.push(HandoffCertificationFinding::RowMissingProof {
                object_kind: object.clone(),
            });
        }
        if matches!(
            self.boundary_honesty,
            BoundaryHonestyState::UndisclosedImpersonation
        ) {
            findings.push(HandoffCertificationFinding::UndisclosedImpersonation {
                object_kind: object.clone(),
            });
        }
        if matches!(
            self.redaction_readiness,
            RedactionReadinessState::UnsafeMaterial
        ) {
            findings.push(HandoffCertificationFinding::UnsafeRedaction {
                object_kind: object.clone(),
            });
        }
        if self.is_stable_qualified()
            && matches!(self.disclosure_freshness, HandoffNoticeFreshnessState::Stale)
            && !self.has_active_waiver()
        {
            findings.push(HandoffCertificationFinding::StaleProofWithoutWaiver {
                object_kind: object.clone(),
            });
        }
        if self.is_stable_qualified()
            && matches!(
                self.disclosure_freshness,
                HandoffNoticeFreshnessState::Unverified
            )
        {
            findings.push(HandoffCertificationFinding::UnverifiedProofOnStableRow {
                object_kind: object.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, HandoffCertStatus::Green) && !self.has_reason() {
            findings.push(HandoffCertificationFinding::NarrowedRowWithoutReason {
                object_kind: object.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must
        // carry an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(HandoffCertificationFinding::NarrowedRowWithoutWaiver {
                object_kind: object.clone(),
            });
        }
        // An attached waiver must still be active and must point at this object.
        if let Some(waiver) = &self.active_waiver {
            if waiver.object_kind != self.object_kind {
                findings.push(HandoffCertificationFinding::WaiverObjectMismatch {
                    object_kind: object.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(HandoffCertificationFinding::WaiverExpired {
                    object_kind: object.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(HandoffCertificationFinding::RowStatusStale {
                object_kind: object.clone(),
            });
        }
        if self.stale_proof_causes != self.recompute_causes() {
            findings.push(HandoffCertificationFinding::RowCausesStale {
                object_kind: object,
            });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} fresh={} boundary={} redaction={} waiver={}",
            self.object_kind.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.disclosure_freshness.as_str(),
            self.boundary_honesty.as_str(),
            self.redaction_readiness.as_str(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the public-handoff certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum HandoffCertificationFinding {
    /// A governed handoff object has no certification row.
    ObjectKindMissing {
        /// The missing object-kind token.
        object_kind: String,
    },
    /// A Stable-qualified row carries no proof packet refs.
    RowMissingProof {
        /// The object-kind token.
        object_kind: String,
    },
    /// A row hides a native-chrome impersonation across its consumer surfaces.
    UndisclosedImpersonation {
        /// The object-kind token.
        object_kind: String,
    },
    /// A row would let raw sensitive material leave, or lose offline capture.
    UnsafeRedaction {
        /// The object-kind token.
        object_kind: String,
    },
    /// A Stable-qualified row claims current truth on stale proof with no waiver.
    StaleProofWithoutWaiver {
        /// The object-kind token.
        object_kind: String,
    },
    /// A Stable-qualified row claims current truth on unverified proof.
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
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl HandoffCertificationFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::ObjectKindMissing { .. } => "object_kind_missing",
            Self::RowMissingProof { .. } => "row_missing_proof",
            Self::UndisclosedImpersonation { .. } => "undisclosed_impersonation",
            Self::UnsafeRedaction { .. } => "unsafe_redaction",
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
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::ObjectKindMissing { object_kind }
            | Self::RowMissingProof { object_kind }
            | Self::UndisclosedImpersonation { object_kind }
            | Self::UnsafeRedaction { object_kind }
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
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release certification packet shared by the release / help / support /
/// public-truth automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicHandoffCertificationPacket {
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
    /// The frozen public-handoff matrix packet id this certification certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen public-handoff matrix schema.
    pub matrix_schema_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// Per-object certification rows, in canonical order.
    pub rows: Vec<HandoffCertificationRow>,
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
    pub active_waivers: Vec<HandoffCertificationWaiver>,
    /// Every exact stale-proof cause, in row then cause order.
    pub stale_proof_causes: Vec<HandoffStaleProofCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<HandoffCertificationFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Release / public-truth automation refs that consume this packet to
    /// auto-narrow claimed surfaces.
    pub public_truth_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Claim-publication refs the row statuses feed.
    pub claim_publication_refs: Vec<String>,
    /// Help / docs refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published certification-packet ref.
    pub published_packet_ref: String,
    /// Published boundary-truth dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl PublicHandoffCertificationPacket {
    /// Returns the certification row for `kind`, if present.
    pub fn row(&self, kind: M5HandoffObjectKind) -> Option<&HandoffCertificationRow> {
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

    /// Projects the light boundary-truth dashboard the public-truth automation
    /// consumes.
    pub fn dashboard(&self) -> HandoffTruthDashboard {
        HandoffTruthDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 public-handoff certification packet serializes")
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 public-handoff & capture-boundary certification\n\n");
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_public_handoff_certification`](../../crates/aureline-shell/src/m5_public_handoff_certification/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_certification -- markdown > \\\n  artifacts/help/m5-public-handoff-certification.md\n",
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

        out.push_str("## Boundary-truth rows\n\n");
        out.push_str(
            "| Certified surface | Status | Qualification | Disclosure freshness | Boundary honesty | Redaction readiness | Waiver |\n\
             | ----------------- | ------ | ------------- | -------------------- | ---------------- | ------------------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.certified_surface,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.disclosure_freshness.as_str(),
                row.boundary_honesty.as_str(),
                row.redaction_readiness.as_str(),
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&HandoffCertificationRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, HandoffCertStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str("None — every governed handoff object certifies green.\n\n");
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_certification -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_public_handoff_certification_fixtures\n");
        out.push_str("```\n");
        out
    }
}

/// One row of the light boundary-truth dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffTruthDashboardRow {
    /// The governed object.
    pub object_kind: M5HandoffObjectKind,
    /// Short certified-surface label.
    pub certified_surface: String,
    /// Derived green/yellow/red status.
    pub status: HandoffCertStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5HandoffQualificationClass,
    /// Disclosure / notice freshness posture.
    pub disclosure_freshness: HandoffNoticeFreshnessState,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// Boundary-honesty posture.
    pub boundary_honesty: BoundaryHonestyState,
    /// Redaction-readiness posture.
    pub redaction_readiness: RedactionReadinessState,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light boundary-truth dashboard the release / help / support / public-truth
/// automation reads to auto-narrow claimed surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffTruthDashboard {
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
    pub rows: Vec<HandoffTruthDashboardRow>,
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

impl HandoffTruthDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &PublicHandoffCertificationPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| HandoffTruthDashboardRow {
                object_kind: row.object_kind,
                certified_surface: row.certified_surface.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                disclosure_freshness: row.disclosure_freshness,
                last_proof_refresh: row.last_proof_refresh.clone(),
                boundary_honesty: row.boundary_honesty,
                redaction_readiness: row.redaction_readiness,
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
            record_kind: M5_HANDOFF_CERT_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_HANDOFF_CERT_SCHEMA_VERSION,
            dashboard_id: M5_HANDOFF_CERT_DASHBOARD_ID.to_owned(),
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
        serde_json::to_string_pretty(self).expect("m5 boundary-truth dashboard serializes")
    }
}

/// Support-export wrapper for the public-handoff certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicHandoffCertificationSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: PublicHandoffCertificationPacket,
    /// Dashboard quoted in full.
    pub dashboard: HandoffTruthDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl PublicHandoffCertificationSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each object kind,
    /// each proof packet ref, and each active waiver id is quoted as a case id so a
    /// support reviewer — or the public-truth automation — can name the same object,
    /// proof, and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: PublicHandoffCertificationPacket,
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
            record_kind: M5_HANDOFF_CERT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_HANDOFF_CERT_SCHEMA_VERSION,
            shared_contract_ref: M5_HANDOFF_CERT_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_public_handoff_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicHandoffCertificationInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen public-handoff matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-object certification rows.
    pub rows: Vec<HandoffCertificationRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The certification packet carries only closed vocabulary, refs, and short labels,
/// so raw URLs, credentials, or tokens must never appear.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds a [`PublicHandoffCertificationPacket`] from the exact build identity, the
/// frozen matrix ref, and the per-object certification rows.
///
/// Each row's derived status and stale-proof causes, the aggregate counts, the
/// active waivers, and the blocking findings are recomputed here so the packet is
/// the single source of truth and the auto-narrowing cannot be asserted.
pub fn build_public_handoff_certification_packet(
    input: PublicHandoffCertificationInput,
) -> PublicHandoffCertificationPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is
    // self-consistent and the auto-narrowing is the single source of truth.
    let rows: Vec<HandoffCertificationRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.stale_proof_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<HandoffCertificationFinding> = Vec::new();

    // Every governed object must carry a certification row.
    let present: BTreeSet<M5HandoffObjectKind> = rows.iter().map(|row| row.object_kind).collect();
    for kind in REQUIRED_OBJECT_KINDS {
        if !present.contains(&kind) {
            blocking_findings.push(HandoffCertificationFinding::ObjectKindMissing {
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
        .filter(|row| matches!(row.derived_status, HandoffCertStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, HandoffCertStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, HandoffCertStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(HandoffCertificationFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<HandoffCertificationWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let stale_proof_causes: Vec<HandoffStaleProofCause> = rows
        .iter()
        .flat_map(|row| row.stale_proof_causes.clone())
        .collect();

    let mut packet = PublicHandoffCertificationPacket {
        record_kind: M5_HANDOFF_CERT_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_HANDOFF_CERT_SCHEMA_VERSION,
        shared_contract_ref: M5_HANDOFF_CERT_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_HANDOFF_CERT_PACKET_ID.to_owned(),
        source_schema_ref: M5_HANDOFF_CERT_SOURCE_SCHEMA_REF.to_owned(),
        headline: "One certifiable boundary-truth story for every governed M5 public-handoff \
                   object: post-install notice/provenance, provenance disclosure, official-vs-\
                   community handoff, redaction-safe reproduction packet, offline-capture \
                   continuity, device/mic permission boundary, embedded webview/auth boundary, \
                   and release/service-health notice certified together, with each row's \
                   green/yellow/red claim auto-narrowed from its disclosure freshness, \
                   boundary-honesty, and redaction posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_HANDOFF_CERT_MATRIX_SCHEMA_REF.to_owned(),
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
        blocking_findings: Vec::new(),
        report_clean: false,
        public_truth_refs: vec![
            "release_public_truth.public_handoff_certification".to_owned(),
            "release_public_truth.auto_narrow.boundary_truth_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.public_handoff_certification".to_owned(),
            "artifacts/release/m5-public-handoff-proof/packet.json".to_owned(),
        ],
        claim_publication_refs: vec!["claim_publication.public_handoff_certification".to_owned()],
        help_docs_refs: vec![M5_HANDOFF_CERT_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-public-handoff-certification".to_owned()],
        published_report_ref: M5_HANDOFF_CERT_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_HANDOFF_CERT_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_HANDOFF_CERT_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_HANDOFF_CERT_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(HandoffCertificationFinding::RawBoundaryMaterialInExport);
    }

    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    packet.report_clean = blocking_findings.is_empty();
    packet.blocking_findings = blocking_findings;

    packet
}

/// Validation error produced by [`validate_public_handoff_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum HandoffCertificationValidationError {
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
    /// The published packet ref is empty.
    PublishedPacketRefMissing,
    /// The published dashboard ref is empty.
    PublishedDashboardRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a packet against the public-handoff certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed
/// handoff object carries a current boundary-truth row; each row's status is the
/// derived auto-narrowed value, never asserted; a published (green) row cannot keep
/// a Stable claim while its proof is stale, unverified, or unbacked, its boundary
/// impersonates native chrome, or its redaction is unsafe; and a disclosed
/// narrowing is backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_public_handoff_certification_packet(
    packet: &PublicHandoffCertificationPacket,
) -> Result<(), Vec<HandoffCertificationValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(HandoffCertificationValidationError::NoRows);
    }
    if packet.record_kind != M5_HANDOFF_CERT_PACKET_RECORD_KIND {
        errors.push(HandoffCertificationValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_HANDOFF_CERT_SCHEMA_VERSION {
        errors.push(HandoffCertificationValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(HandoffCertificationValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(HandoffCertificationValidationError::MatrixPacketRefMissing);
    }

    let present: BTreeSet<M5HandoffObjectKind> =
        packet.rows.iter().map(|row| row.object_kind).collect();
    let coverage_complete = REQUIRED_OBJECT_KINDS
        .iter()
        .all(|kind| present.contains(kind));
    if !coverage_complete || packet.rows.len() != REQUIRED_OBJECT_KINDS.len() {
        errors.push(HandoffCertificationValidationError::CoverageIncomplete);
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
        errors.push(HandoffCertificationValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), HandoffCertStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), HandoffCertStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), HandoffCertStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(HandoffCertificationValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<HandoffCertificationWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(HandoffCertificationValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<HandoffStaleProofCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.stale_proof_causes {
        errors.push(HandoffCertificationValidationError::StaleProofCausesStale);
    }

    let mut recomputed: Vec<HandoffCertificationFinding> = Vec::new();
    for kind in REQUIRED_OBJECT_KINDS {
        if !present.contains(&kind) {
            recomputed.push(HandoffCertificationFinding::ObjectKindMissing {
                object_kind: kind.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(HandoffCertificationFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(HandoffCertificationFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(HandoffCertificationValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(
            HandoffCertificationValidationError::BlockingFindingPresent {
                class: finding.class_token().to_owned(),
                subject_ref: finding.subject_ref().to_owned(),
            },
        );
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(HandoffCertificationValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(HandoffCertificationValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(HandoffCertificationValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(HandoffCertificationValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
