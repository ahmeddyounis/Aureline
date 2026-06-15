//! Typed About/Help/service-health provenance cards that converge the exact-build
//! identity and artifact provenance of every M5 artifact family in user-visible
//! product chrome.
//!
//! Where the M5 publication matrix
//! (`freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix`)
//! freezes the *release-control* truth that decides whether an artifact family
//! may publish, and the promotion-ledger register
//! (`implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs`)
//! records the *promotion history* every artifact graph accumulates, this
//! register speaks for the *user-visible provenance card* every M5 artifact
//! family exposes — the single inspectable object that About, Help, the release
//! center, service health, and support/export surfaces all render, so each agrees
//! on the same build identity and artifact provenance rather than telling
//! different stories. Each [`ProvenanceCard`] binds one M5 artifact family
//! ([`M5ArtifactFamilyKind`]) to:
//!
//! - its exact-build identity ([`ExactBuildIdentity`], reused from the
//!   publication matrix): the one-build identity and provenance refs, signature
//!   state, attestation availability, SBOM scope, symbol/source-map availability,
//!   mirror freshness, rollback target, and evidence completeness — the
//!   convergence anchor every surface must quote,
//! - its rollback/revocation posture ([`RollbackRevocationPosture`]) and its
//!   mirror/offline publication expectation ([`MirrorOfflineExpectation`]),
//! - a set of copy-safe, machine-readable provenance [`ProvenanceBadge`]s that
//!   project the exact-build state into the displayed facts — signature verified,
//!   attestation available, SPDX SBOM, CycloneDX export, mirrored, official,
//!   partial, and not-provided — each carrying a stable machine token so the
//!   badge is copy-safe and exportable,
//! - a set of [`SurfaceBinding`]s, one per user-visible [`ProvenanceSurfaceKind`]
//!   (About, Help, release center, service health, support, export), each
//!   carrying the build-identity and provenance refs the surface renders so the
//!   register can prove every surface converges on the *same* identity and that
//!   the provenance survives offline and mirror profiles,
//! - the required evidence and its freshness SLO ([`ProofPacket`]) and owner
//!   sign-off ([`OwnerSignoff`]),
//! - the public claim it backs, the active gap reasons ([`CardGapReason`])
//!   narrowing it, and the effective label it carries after narrowing
//!   ([`ProvenanceCard::published_label`]).
//!
//! The [`LaunchCutline`] fixes the boundary between a card that may converge a
//! Stable claim and one that must narrow below it. The
//! [`ProvenanceCardStopRule`] set names the closed conditions that gate
//! publication, one per [`CardGapReason`], and
//! [`M5ProvenanceCardRegister::publication`] records the proceed/hold verdict,
//! computed only from cards whose public claim is still at or above the cutline.
//!
//! Two guardrails are encoded directly in
//! [`validate`](M5ProvenanceCardRegister::validate):
//!
//! - A card may not become release-center-only truth that Help and About cannot
//!   explain: a card that renders a release-center surface but omits its About,
//!   Help, or service-health chrome is a hard violation, not a waivable
//!   narrowing.
//! - A badge may not imply a stronger trust posture than the actual
//!   signature/attestation/SBOM/symbol/mirror/rollback state available: a badge
//!   that ranks stronger than its underlying exact-build state is a hard
//!   violation.
//!
//! A card only converges its claimed label when its exact-build linkage is
//! intact, every surface renders the same identity, the provenance survives
//! offline and mirror profiles, every badge is honest, its proof packet is within
//! SLO, and it is owner-signed. Any card whose exact-build linkage thins, whose
//! surfaces diverge, whose provenance cannot be verified offline, or whose
//! evidence is missing or stale narrows below the cutline before promotion and
//! names every reason that forced it there.
//!
//! The register is checked in at
//! `artifacts/release/m5/add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts.json`
//! and embedded here, so this typed consumer and the CI gate agree on every card
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, signatures, SBOM bodies, attestation payloads, or
//! credential material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix::{
    AttestationAvailability, EvidenceCompleteness, ExactBuildIdentity, M5ArtifactFamilyKind,
    MirrorFreshness, MirrorOfflineExpectation, RollbackRevocationPosture, SbomScope,
    SymbolSourceMapAvailability,
};
use crate::release_center_model::SignatureStateClass;
use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

mod builder;
pub use builder::build_m5_provenance_cards;

/// Supported register schema version.
pub const M5_PROVENANCE_CARDS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_PROVENANCE_CARDS_RECORD_KIND: &str =
    "add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts";

/// Repo-relative path to the checked-in register.
pub const M5_PROVENANCE_CARDS_PATH: &str =
    "artifacts/release/m5/add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts.json";

/// Embedded checked-in register JSON.
pub const M5_PROVENANCE_CARDS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts.json"
));

/// A user-visible surface that renders the provenance card and must converge on
/// the same build identity and artifact provenance as every other surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceSurfaceKind {
    /// The About box / version panel.
    About,
    /// The Help surface and in-product diagnostics entry points.
    Help,
    /// The release-center artifact-graph view.
    ReleaseCenter,
    /// The service-health truth surface.
    ServiceHealth,
    /// The support packet surface.
    Support,
    /// The machine-readable export / copy-build-info surface.
    Export,
}

impl ProvenanceSurfaceKind {
    /// Every surface kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::About,
        Self::Help,
        Self::ReleaseCenter,
        Self::ServiceHealth,
        Self::Support,
        Self::Export,
    ];

    /// The user-visible product chrome that the release-center-only guardrail
    /// requires: a release-center surface may not stand alone without these.
    pub const HELP_ABOUT_CHROME: [Self; 3] = [Self::About, Self::Help, Self::ServiceHealth];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::About => "about",
            Self::Help => "help",
            Self::ReleaseCenter => "release_center",
            Self::ServiceHealth => "service_health",
            Self::Support => "support",
            Self::Export => "export",
        }
    }

    /// True when the surface is part of the user-visible Help/About chrome.
    pub const fn is_help_about_chrome(self) -> bool {
        matches!(self, Self::About | Self::Help | Self::ServiceHealth)
    }
}

/// The provenance facet a copy-safe badge speaks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceBadgeKind {
    /// The convergent exact-build identity ("official" build).
    BuildIdentity,
    /// The release signature.
    Signature,
    /// The build attestation.
    Attestation,
    /// The SPDX SBOM.
    SpdxSbom,
    /// The CycloneDX SBOM export.
    CycloneDxExport,
    /// The symbol / source-map availability.
    Symbols,
    /// The mirror / offline publication copy.
    Mirror,
    /// The rollback / revocation availability.
    Rollback,
}

impl ProvenanceBadgeKind {
    /// Every badge kind, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::BuildIdentity,
        Self::Signature,
        Self::Attestation,
        Self::SpdxSbom,
        Self::CycloneDxExport,
        Self::Symbols,
        Self::Mirror,
        Self::Rollback,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildIdentity => "build_identity",
            Self::Signature => "signature",
            Self::Attestation => "attestation",
            Self::SpdxSbom => "spdx_sbom",
            Self::CycloneDxExport => "cyclonedx_export",
            Self::Symbols => "symbols",
            Self::Mirror => "mirror",
            Self::Rollback => "rollback",
        }
    }
}

/// The copy-safe, machine-readable state a provenance badge displays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceBadgeState {
    /// Signature verified.
    Verified,
    /// Attestation / SBOM / export available and in scope.
    Available,
    /// An official, convergent build identity.
    Official,
    /// Served from a current mirror / offline copy.
    Mirrored,
    /// Present but only partially (component-scoped, retained-internal, lagging).
    Partial,
    /// Pending the release signature or attestation.
    Pending,
    /// Revoked.
    Revoked,
    /// Not provided for this family.
    NotProvided,
}

impl ProvenanceBadgeState {
    /// Every badge state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Verified,
        Self::Available,
        Self::Official,
        Self::Mirrored,
        Self::Partial,
        Self::Pending,
        Self::Revoked,
        Self::NotProvided,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Available => "available",
            Self::Official => "official",
            Self::Mirrored => "mirrored",
            Self::Partial => "partial",
            Self::Pending => "pending",
            Self::Revoked => "revoked",
            Self::NotProvided => "not_provided",
        }
    }

    /// Trust rank; a stronger posture ranks higher. A badge that ranks stronger
    /// than its underlying exact-build state implies a posture the artifact does
    /// not actually hold.
    pub const fn trust_rank(self) -> u8 {
        match self {
            Self::Verified => 5,
            Self::Available | Self::Official => 4,
            Self::Mirrored => 3,
            Self::Partial | Self::Pending => 2,
            Self::Revoked => 1,
            Self::NotProvided => 0,
        }
    }
}

/// The state a card earned for its claimed label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardState {
    /// Full, current, owner-signed, surface-converged backing; holds the label.
    Converged,
    /// Holds the label only because an active, unexpired waiver covers a gap.
    OnWaiver,
    /// Narrowed: surfaces disagree on the build identity or a required surface is
    /// missing.
    SurfaceDivergent,
    /// Narrowed: the exact-build provenance (signature/attestation/SBOM/symbol/
    /// mirror/rollback) is thin.
    ProvenanceThin,
    /// Narrowed: the proof packet is missing or stale.
    Stale,
    /// Narrowed: a relied-on waiver expired.
    WaiverExpired,
    /// Narrowed: owner sign-off is missing.
    OwnerUnsigned,
}

impl CardState {
    /// Every card state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Converged,
        Self::OnWaiver,
        Self::SurfaceDivergent,
        Self::ProvenanceThin,
        Self::Stale,
        Self::WaiverExpired,
        Self::OwnerUnsigned,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Converged => "converged",
            Self::OnWaiver => "on_waiver",
            Self::SurfaceDivergent => "surface_divergent",
            Self::ProvenanceThin => "provenance_thin",
            Self::Stale => "stale",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerUnsigned => "owner_unsigned",
        }
    }

    /// True when the state lets a card hold a label at or above the cutline.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Converged | Self::OnWaiver)
    }
}

/// Closed reason a provenance card narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardGapReason {
    /// The release signature is missing, unverified, or pending.
    SignatureMissing,
    /// The release signature was revoked.
    SignatureRevoked,
    /// A build attestation is missing or pending.
    AttestationMissing,
    /// The SBOM is partial or missing.
    SbomIncomplete,
    /// Symbols/source maps were stripped or are missing.
    SymbolsMissing,
    /// The mirror copy is stale or unpublished.
    MirrorStale,
    /// The rollback target (last-known-good) is missing.
    RollbackTargetMissing,
    /// The one-build identity or provenance ref is missing.
    ExactBuildLinkageBroken,
    /// The provenance cannot be verified offline or under a mirror profile.
    OfflineUnverifiable,
    /// A surface renders a different build identity than the card.
    SurfaceDivergent,
    /// A required user-visible surface does not render the card.
    SurfaceMissing,
    /// The release evidence set is incomplete.
    EvidenceIncomplete,
    /// The proof packet aged out of its freshness SLO.
    ProofPacketStale,
    /// No proof packet has been captured.
    ProofPacketMissing,
    /// A waiver the card relied on expired.
    WaiverExpired,
    /// Owner sign-off is missing.
    OwnerSignoffMissing,
}

impl CardGapReason {
    /// Every gap reason, in declaration order.
    pub const ALL: [Self; 16] = [
        Self::SignatureMissing,
        Self::SignatureRevoked,
        Self::AttestationMissing,
        Self::SbomIncomplete,
        Self::SymbolsMissing,
        Self::MirrorStale,
        Self::RollbackTargetMissing,
        Self::ExactBuildLinkageBroken,
        Self::OfflineUnverifiable,
        Self::SurfaceDivergent,
        Self::SurfaceMissing,
        Self::EvidenceIncomplete,
        Self::ProofPacketStale,
        Self::ProofPacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignatureMissing => "signature_missing",
            Self::SignatureRevoked => "signature_revoked",
            Self::AttestationMissing => "attestation_missing",
            Self::SbomIncomplete => "sbom_incomplete",
            Self::SymbolsMissing => "symbols_missing",
            Self::MirrorStale => "mirror_stale",
            Self::RollbackTargetMissing => "rollback_target_missing",
            Self::ExactBuildLinkageBroken => "exact_build_linkage_broken",
            Self::OfflineUnverifiable => "offline_unverifiable",
            Self::SurfaceDivergent => "surface_divergent",
            Self::SurfaceMissing => "surface_missing",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }

    /// True when the reason names an exact-build provenance gap.
    pub const fn is_provenance_gap(self) -> bool {
        matches!(
            self,
            Self::SignatureMissing
                | Self::SignatureRevoked
                | Self::AttestationMissing
                | Self::SbomIncomplete
                | Self::SymbolsMissing
                | Self::MirrorStale
                | Self::RollbackTargetMissing
                | Self::ExactBuildLinkageBroken
                | Self::EvidenceIncomplete
        )
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the public claim below the cutline.
    NarrowLabel,
    /// Re-sign the release artifact.
    ReSignArtifact,
    /// Re-attest the build.
    ReAttest,
    /// Regenerate the SBOM.
    RegenerateSbom,
    /// Publish the symbols / source maps.
    PublishSymbols,
    /// Refresh the mirror copy.
    RefreshMirror,
    /// Record the rollback target (last-known-good).
    RecordRollbackTarget,
    /// Rebuild to restore exact-build linkage.
    RebuildExactBuild,
    /// Restore offline / mirror provenance parity.
    RestoreOfflineParity,
    /// Reconcile the diverging surface to the canonical build identity.
    ReconcileSurfaces,
    /// Render the missing user-visible surface.
    RenderMissingSurface,
    /// Recapture the evidence set.
    RecaptureEvidence,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Renew the expired waiver.
    RenewWaiver,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl CardAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 16] = [
        Self::HoldPublication,
        Self::NarrowLabel,
        Self::ReSignArtifact,
        Self::ReAttest,
        Self::RegenerateSbom,
        Self::PublishSymbols,
        Self::RefreshMirror,
        Self::RecordRollbackTarget,
        Self::RebuildExactBuild,
        Self::RestoreOfflineParity,
        Self::ReconcileSurfaces,
        Self::RenderMissingSurface,
        Self::RecaptureEvidence,
        Self::RefreshProofPacket,
        Self::RenewWaiver,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowLabel => "narrow_label",
            Self::ReSignArtifact => "re_sign_artifact",
            Self::ReAttest => "re_attest",
            Self::RegenerateSbom => "regenerate_sbom",
            Self::PublishSymbols => "publish_symbols",
            Self::RefreshMirror => "refresh_mirror",
            Self::RecordRollbackTarget => "record_rollback_target",
            Self::RebuildExactBuild => "rebuild_exact_build",
            Self::RestoreOfflineParity => "restore_offline_parity",
            Self::ReconcileSurfaces => "reconcile_surfaces",
            Self::RenderMissingSurface => "render_missing_surface",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RenewWaiver => "renew_waiver",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One copy-safe, machine-readable provenance badge on a card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceBadge {
    /// The provenance facet the badge speaks for.
    pub kind: ProvenanceBadgeKind,
    /// The displayed, copy-safe state.
    pub state: ProvenanceBadgeState,
    /// A stable `kind:state` machine token a surface can copy and export.
    pub machine_token: String,
    /// Reviewable one-line human label.
    pub label: String,
    /// Whether the badge is copy-safe (carries no credential body). Always true.
    pub copyable: bool,
}

impl ProvenanceBadge {
    /// The canonical machine token for this badge: `kind:state`.
    pub fn canonical_token(&self) -> String {
        format!("{}:{}", self.kind.as_str(), self.state.as_str())
    }
}

/// One user-visible surface binding: proof that a surface renders the card's
/// build identity and provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceBinding {
    /// The user-visible surface.
    pub surface: ProvenanceSurfaceKind,
    /// Ref to the surface that renders the card.
    pub surface_ref: String,
    /// The one-build identity ref the surface renders. Must equal the card's.
    pub build_identity_ref: String,
    /// The provenance ref the surface renders. Must equal the card's.
    pub provenance_ref: String,
    /// Whether the surface renders the copy-safe provenance badges.
    pub renders_badges: bool,
    /// Whether the surface exposes copy-safe build info / provenance.
    pub copyable: bool,
    /// Whether the surface renders the provenance without live vendor
    /// connectivity (survives offline and mirror profiles).
    pub offline_available: bool,
    /// Reviewable one-line statement of the binding.
    pub summary: String,
}

impl SurfaceBinding {
    /// True when the surface renders the same build identity and provenance as
    /// `exact_build`.
    pub fn converges_with(&self, exact_build: &ExactBuildIdentity) -> bool {
        self.build_identity_ref == exact_build.build_identity_ref
            && self.provenance_ref == exact_build.provenance_ref
    }
}

/// One stop rule: a closed condition that gates publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceCardStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched card fires this rule.
    pub trigger_reason: CardGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: CardAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One provenance card: an M5 artifact family's exact-build identity converged
/// across every user-visible surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceCard {
    /// Stable entry id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The M5 artifact family the card speaks for.
    pub family_kind: M5ArtifactFamilyKind,
    /// Ref to the artifact the card describes.
    pub artifact_ref: String,
    /// Reviewable one-line summary of the artifact.
    pub artifact_summary: String,
    /// Whether the card backs a release-blocking artifact.
    pub release_blocking: bool,
    /// The public claim the card backs.
    pub claim_ref: String,
    /// The claim's canonical lifecycle label. Always a hard ceiling.
    pub claim_label: StableClaimLevel,
    /// The state the card earned for its claimed label.
    pub card_state: CardState,
    /// The exact-build identity every surface must converge on.
    pub exact_build: ExactBuildIdentity,
    /// The rollback/revocation posture.
    pub rollback_revocation: RollbackRevocationPosture,
    /// The mirror/offline publication expectation.
    pub mirror_offline: MirrorOfflineExpectation,
    /// The copy-safe, machine-readable provenance badges, one per badge kind.
    pub badges: Vec<ProvenanceBadge>,
    /// The user-visible surface bindings that render the card.
    pub surface_bindings: Vec<SurfaceBinding>,
    /// The required evidence and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional convergence, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the card.
    #[serde(default)]
    pub active_gap_reasons: Vec<CardGapReason>,
    /// The label the card effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this card.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the card carries this posture.
    pub rationale: String,
}

impl ProvenanceCard {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the card's public claim is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the card's state lets it hold its claimed label.
    pub fn holds_label(&self) -> bool {
        self.card_state.holds_label()
    }

    /// True when a gap reason is active on the card.
    pub fn has_active_reason(&self, reason: CardGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// Returns the surface binding for `surface`, if present.
    pub fn surface(&self, surface: ProvenanceSurfaceKind) -> Option<&SurfaceBinding> {
        self.surface_bindings.iter().find(|b| b.surface == surface)
    }

    /// Returns the badge for `kind`, if present.
    pub fn badge(&self, kind: ProvenanceBadgeKind) -> Option<&ProvenanceBadge> {
        self.badges.iter().find(|b| b.kind == kind)
    }

    /// True when every surface binding renders the card's build identity.
    pub fn surfaces_converge(&self) -> bool {
        !self.surface_bindings.is_empty()
            && self
                .surface_bindings
                .iter()
                .all(|b| b.converges_with(&self.exact_build))
    }

    /// True when the card carries its About, Help, and service-health chrome.
    pub fn has_help_about_chrome(&self) -> bool {
        ProvenanceSurfaceKind::HELP_ABOUT_CHROME
            .iter()
            .all(|surface| self.surface(*surface).is_some())
    }

    /// True when the card carries every user-visible surface binding.
    pub fn has_all_surfaces(&self) -> bool {
        ProvenanceSurfaceKind::ALL
            .iter()
            .all(|surface| self.surface(*surface).is_some())
    }

    /// True when the user-visible provenance survives offline and mirror
    /// profiles: the artifact is offline-verifiable and the Help/About chrome
    /// renders without live vendor connectivity.
    pub fn offline_provenance_survives(&self) -> bool {
        self.mirror_offline.offline_verifiable
            && ProvenanceSurfaceKind::HELP_ABOUT_CHROME
                .iter()
                .all(|surface| {
                    self.surface(*surface)
                        .map(|b| b.offline_available)
                        .unwrap_or(false)
                })
    }

    /// The canonical badge state for a facet given the exact-build identity, so a
    /// badge can be checked for honesty.
    pub fn canonical_badge_state(&self, kind: ProvenanceBadgeKind) -> ProvenanceBadgeState {
        canonical_badge_state(kind, &self.exact_build, &self.rollback_revocation)
    }

    /// True when a badge implies a stronger trust posture than the underlying
    /// exact-build state allows.
    pub fn badge_overclaims(&self) -> bool {
        self.overclaiming_badge_kind().is_some()
    }

    /// The first badge kind whose declared state overclaims its underlying state.
    pub fn overclaiming_badge_kind(&self) -> Option<ProvenanceBadgeKind> {
        self.badges
            .iter()
            .find(|badge| {
                badge.state.trust_rank() > self.canonical_badge_state(badge.kind).trust_rank()
            })
            .map(|badge| badge.kind)
    }
}

/// The canonical, honest badge state for `kind` given an exact-build identity.
pub(crate) fn canonical_badge_state(
    kind: ProvenanceBadgeKind,
    exact_build: &ExactBuildIdentity,
    rollback: &RollbackRevocationPosture,
) -> ProvenanceBadgeState {
    use ProvenanceBadgeState as S;
    match kind {
        ProvenanceBadgeKind::BuildIdentity => {
            if exact_build.linkage_intact() {
                S::Official
            } else if exact_build.build_identity_ref.trim().is_empty()
                || exact_build.provenance_ref.trim().is_empty()
            {
                S::NotProvided
            } else {
                S::Partial
            }
        }
        ProvenanceBadgeKind::Signature => match exact_build.signature_state {
            SignatureStateClass::Verified => S::Verified,
            SignatureStateClass::PendingReleaseSignature => S::Pending,
            SignatureStateClass::Revoked => S::Revoked,
            SignatureStateClass::PresentUnverified => S::Partial,
            SignatureStateClass::Missing => S::NotProvided,
        },
        ProvenanceBadgeKind::Attestation => match exact_build.attestation_availability {
            AttestationAvailability::Attested => S::Available,
            AttestationAvailability::PendingAttestation => S::Pending,
            AttestationAvailability::Unattested | AttestationAvailability::NotApplicable => {
                S::NotProvided
            }
        },
        ProvenanceBadgeKind::SpdxSbom | ProvenanceBadgeKind::CycloneDxExport => {
            match exact_build.sbom_scope {
                SbomScope::FullGraph | SbomScope::ComponentScoped => S::Available,
                SbomScope::Partial => S::Partial,
                SbomScope::Missing | SbomScope::NotApplicable => S::NotProvided,
            }
        }
        ProvenanceBadgeKind::Symbols => match exact_build.symbol_availability {
            SymbolSourceMapAvailability::Published => S::Available,
            SymbolSourceMapAvailability::RetainedInternal => S::Partial,
            SymbolSourceMapAvailability::Stripped
            | SymbolSourceMapAvailability::Missing
            | SymbolSourceMapAvailability::NotApplicable => S::NotProvided,
        },
        ProvenanceBadgeKind::Mirror => match exact_build.mirror_freshness {
            MirrorFreshness::Current => S::Mirrored,
            MirrorFreshness::Stale => S::Partial,
            MirrorFreshness::Unpublished | MirrorFreshness::NotApplicable => S::NotProvided,
        },
        ProvenanceBadgeKind::Rollback => {
            if exact_build.rollback_target_ref.trim().is_empty() {
                S::NotProvided
            } else if rollback.revocable {
                S::Available
            } else {
                S::Partial
            }
        }
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ProvenanceCardSummary {
    /// Total cards.
    pub total_entries: usize,
    /// Cards converging a label at or above the cutline.
    pub entries_converged: usize,
    /// Cards narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Cards holding via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Release-blocking cards.
    pub release_blocking_total: usize,
    /// Release-blocking cards that converge.
    pub release_blocking_converged: usize,
    /// Release-blocking cards narrowed.
    pub release_blocking_narrowed: usize,
    /// Notebook-pack cards.
    pub notebook_pack_entries: usize,
    /// Request/data-asset cards.
    pub request_data_asset_entries: usize,
    /// Profiler/replay cards.
    pub profiler_replay_entries: usize,
    /// Framework/template-pack cards.
    pub framework_template_entries: usize,
    /// Docs-pack cards.
    pub docs_pack_entries: usize,
    /// Model-pack cards.
    pub model_pack_entries: usize,
    /// Companion/offboarding cards.
    pub companion_offboarding_entries: usize,
    /// Managed-output cards.
    pub managed_output_entries: usize,
    /// Cards whose signature is verified.
    pub signatures_verified: usize,
    /// Cards whose attestation is available.
    pub attestations_available: usize,
    /// Cards with an in-scope SPDX SBOM.
    pub spdx_sbom_in_scope: usize,
    /// Cards with an exportable CycloneDX SBOM.
    pub cyclonedx_exportable: usize,
    /// Cards whose symbols are available.
    pub symbols_available: usize,
    /// Cards whose mirror copy is current.
    pub mirror_current: usize,
    /// Cards with a recorded rollback target.
    pub rollback_targets_recorded: usize,
    /// Cards whose provenance is offline-verifiable.
    pub offline_verifiable: usize,
    /// Cards whose every surface converges on the build identity.
    pub surfaces_converged: usize,
    /// Cards with a within-SLO captured proof packet.
    pub packets_current: usize,
    /// Cards whose proof packet is due for refresh.
    pub packets_due_for_refresh: usize,
    /// Cards whose proof packet breached its SLO.
    pub packets_breached: usize,
    /// Cards without a captured proof packet.
    pub packets_missing: usize,
    /// Total surface bindings across all cards.
    pub total_surface_bindings: usize,
    /// Total badges across all cards.
    pub total_badges: usize,
    /// Total active gap reasons across all cards.
    pub total_active_gap_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One copy-safe badge in the export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceCardBadgeExport {
    /// Badge facet.
    pub kind: ProvenanceBadgeKind,
    /// Displayed state.
    pub state: ProvenanceBadgeState,
    /// Copy-safe machine token.
    pub machine_token: String,
    /// Human label.
    pub label: String,
}

/// One surface binding in the export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceCardSurfaceExport {
    /// Surface.
    pub surface: ProvenanceSurfaceKind,
    /// Surface ref.
    pub surface_ref: String,
    /// Whether the surface renders the card's build identity.
    pub converges: bool,
    /// Whether the surface renders the badges.
    pub renders_badges: bool,
    /// Whether the surface exposes copy-safe build info.
    pub copyable: bool,
    /// Whether the surface renders offline.
    pub offline_available: bool,
}

/// One row in the export/audit-safe projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProvenanceCardExportRow {
    /// Card id.
    pub entry_id: String,
    /// Artifact family.
    pub family_kind: M5ArtifactFamilyKind,
    /// Artifact ref.
    pub artifact_ref: String,
    /// Whether the card backs a release-blocking artifact.
    pub release_blocking: bool,
    /// Public claim ref.
    pub claim_ref: String,
    /// Canonical claim label.
    pub claim_label: StableClaimLevel,
    /// Effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the card publishes a label at or above the cutline.
    pub publishes_stable: bool,
    /// Card state.
    pub card_state: CardState,
    /// One-build identity ref every surface converges on.
    pub build_identity_ref: String,
    /// Provenance ref every surface converges on.
    pub provenance_ref: String,
    /// Signature state.
    pub signature_state: SignatureStateClass,
    /// Attestation availability.
    pub attestation_availability: AttestationAvailability,
    /// SBOM scope.
    pub sbom_scope: SbomScope,
    /// Symbol/source-map availability.
    pub symbol_availability: SymbolSourceMapAvailability,
    /// Mirror freshness.
    pub mirror_freshness: MirrorFreshness,
    /// Whether the rollback/revocation posture is revocable.
    pub rollback_revocable: bool,
    /// Whether the provenance is offline-verifiable.
    pub offline_verifiable: bool,
    /// Whether every surface converges on the build identity.
    pub surfaces_converge: bool,
    /// Proof-packet freshness state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<CardGapReason>,
    /// Copy-safe badges.
    pub badges: Vec<ProvenanceCardBadgeExport>,
    /// Surface bindings.
    pub surfaces: Vec<ProvenanceCardSurfaceExport>,
}

/// The export/audit-safe projection downstream surfaces render.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProvenanceCardExportProjection {
    /// Register id.
    pub manifest_id: String,
    /// As-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Per-card rows.
    pub rows: Vec<M5ProvenanceCardExportRow>,
}

/// The typed provenance-card register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ProvenanceCardRegister {
    /// Register schema version.
    pub schema_version: u32,
    /// Stable record kind.
    pub record_kind: String,
    /// Register id.
    pub manifest_id: String,
    /// Lifecycle status.
    pub status: String,
    /// Human-readable overview page.
    pub overview_page: String,
    /// As-of date.
    pub as_of: String,
    /// Ref to the stable claim manifest.
    pub claim_manifest_ref: String,
    /// Ref to the M5 exact-build publication matrix.
    pub publication_matrix_ref: String,
    /// Ref to the M5 artifact-graph promotion register.
    pub artifact_graph_ref: String,
    /// Ref to the docs/Help/About/service-health truth register.
    pub docs_help_about_truth_ref: String,
    /// Ref to the shared service-health feed.
    pub service_health_feed_ref: String,
    /// Ref to the release-center object model.
    pub release_center_model_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed family-kind vocabulary.
    pub family_kinds: Vec<M5ArtifactFamilyKind>,
    /// Closed surface-kind vocabulary.
    pub surface_kinds: Vec<ProvenanceSurfaceKind>,
    /// Closed badge-kind vocabulary.
    pub badge_kinds: Vec<ProvenanceBadgeKind>,
    /// Closed badge-state vocabulary.
    pub badge_states: Vec<ProvenanceBadgeState>,
    /// Closed card-state vocabulary.
    pub card_states: Vec<CardState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<CardGapReason>,
    /// Closed action vocabulary.
    pub card_actions: Vec<CardAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// Declared release-blocking artifact refs.
    pub release_blocking_artifact_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<ProvenanceCardStopRule>,
    /// Provenance cards.
    pub rows: Vec<ProvenanceCard>,
    /// The publication verdict.
    pub publication: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5ProvenanceCardSummary,
}

impl M5ProvenanceCardRegister {
    /// Returns the card registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&ProvenanceCard> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the cards converging a label at or above the cutline.
    pub fn rows_converged(&self) -> Vec<&ProvenanceCard> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the cards narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&ProvenanceCard> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking cards.
    pub fn release_blocking_rows(&self) -> Vec<&ProvenanceCard> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the cards for one artifact-family kind.
    pub fn rows_for_kind(&self, kind: M5ArtifactFamilyKind) -> Vec<&ProvenanceCard> {
        self.rows
            .iter()
            .filter(|row| row.family_kind == kind)
            .collect()
    }

    /// Returns the cards that render a binding for `surface`.
    pub fn rows_for_surface(&self, surface: ProvenanceSurfaceKind) -> Vec<&ProvenanceCard> {
        self.rows
            .iter()
            .filter(|row| row.surface(surface).is_some())
            .collect()
    }

    /// True when `rule` fires: a watched card carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &ProvenanceCardStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the cards and stop rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .stop_rules
            .iter()
            .any(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Stop-rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Card ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only cards whose claim is at or above the cutline count: a card whose
    /// claim is already canonically narrowed is not a *publication* blocker, it
    /// merely inherits the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<CardGapReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row.claim_holds_stable()
                && row
                    .active_gap_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the cards and stop rules.
    pub fn computed_summary(&self) -> M5ProvenanceCardSummary {
        let kind = |kind: M5ArtifactFamilyKind| self.rows_for_kind(kind).len();
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let release_blocking: Vec<&ProvenanceCard> = self.release_blocking_rows();
        M5ProvenanceCardSummary {
            total_entries: self.rows.len(),
            entries_converged: self
                .rows
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            entries_narrowed: self
                .rows
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            entries_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.card_state == CardState::OnWaiver)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_converged: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            notebook_pack_entries: kind(M5ArtifactFamilyKind::NotebookPack),
            request_data_asset_entries: kind(M5ArtifactFamilyKind::RequestDataAsset),
            profiler_replay_entries: kind(M5ArtifactFamilyKind::ProfilerReplayArtifact),
            framework_template_entries: kind(M5ArtifactFamilyKind::FrameworkTemplatePack),
            docs_pack_entries: kind(M5ArtifactFamilyKind::DocsPack),
            model_pack_entries: kind(M5ArtifactFamilyKind::ModelPack),
            companion_offboarding_entries: kind(M5ArtifactFamilyKind::CompanionOffboardingPacket),
            managed_output_entries: kind(M5ArtifactFamilyKind::ManagedOutput),
            signatures_verified: self
                .rows
                .iter()
                .filter(|row| row.exact_build.signature_state == SignatureStateClass::Verified)
                .count(),
            attestations_available: self
                .rows
                .iter()
                .filter(|row| {
                    row.exact_build.attestation_availability == AttestationAvailability::Attested
                })
                .count(),
            spdx_sbom_in_scope: self
                .rows
                .iter()
                .filter(|row| {
                    matches!(
                        row.exact_build.sbom_scope,
                        SbomScope::FullGraph | SbomScope::ComponentScoped
                    )
                })
                .count(),
            cyclonedx_exportable: self
                .rows
                .iter()
                .filter(|row| {
                    matches!(
                        row.exact_build.sbom_scope,
                        SbomScope::FullGraph | SbomScope::ComponentScoped
                    )
                })
                .count(),
            symbols_available: self
                .rows
                .iter()
                .filter(|row| {
                    matches!(
                        row.exact_build.symbol_availability,
                        SymbolSourceMapAvailability::Published
                            | SymbolSourceMapAvailability::RetainedInternal
                    )
                })
                .count(),
            mirror_current: self
                .rows
                .iter()
                .filter(|row| row.exact_build.mirror_freshness == MirrorFreshness::Current)
                .count(),
            rollback_targets_recorded: self
                .rows
                .iter()
                .filter(|row| !row.exact_build.rollback_target_ref.trim().is_empty())
                .count(),
            offline_verifiable: self
                .rows
                .iter()
                .filter(|row| row.offline_provenance_survives())
                .count(),
            surfaces_converged: self
                .rows
                .iter()
                .filter(|row| row.surfaces_converge() && row.has_all_surfaces())
                .count(),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_surface_bindings: self.rows.iter().map(|row| row.surface_bindings.len()).sum(),
            total_badges: self.rows.iter().map(|row| row.badges.len()).sum(),
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/audit-safe projection downstream surfaces render
    /// instead of cloning status text. Each row carries its copy-safe badges and
    /// per-surface bindings.
    pub fn support_export_projection(&self) -> M5ProvenanceCardExportProjection {
        M5ProvenanceCardExportProjection {
            manifest_id: self.manifest_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5ProvenanceCardExportRow {
                    entry_id: row.entry_id.clone(),
                    family_kind: row.family_kind,
                    artifact_ref: row.artifact_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    card_state: row.card_state,
                    build_identity_ref: row.exact_build.build_identity_ref.clone(),
                    provenance_ref: row.exact_build.provenance_ref.clone(),
                    signature_state: row.exact_build.signature_state,
                    attestation_availability: row.exact_build.attestation_availability,
                    sbom_scope: row.exact_build.sbom_scope,
                    symbol_availability: row.exact_build.symbol_availability,
                    mirror_freshness: row.exact_build.mirror_freshness,
                    rollback_revocable: row.rollback_revocation.revocable,
                    offline_verifiable: row.offline_provenance_survives(),
                    surfaces_converge: row.surfaces_converge(),
                    slo_state: row.proof_packet.slo_state,
                    active_gap_reasons: row.active_gap_reasons.clone(),
                    badges: row
                        .badges
                        .iter()
                        .map(|badge| ProvenanceCardBadgeExport {
                            kind: badge.kind,
                            state: badge.state,
                            machine_token: badge.machine_token.clone(),
                            label: badge.label.clone(),
                        })
                        .collect(),
                    surfaces: row
                        .surface_bindings
                        .iter()
                        .map(|binding| ProvenanceCardSurfaceExport {
                            surface: binding.surface,
                            surface_ref: binding.surface_ref.clone(),
                            converges: binding.converges_with(&row.exact_build),
                            renders_badges: binding.renders_badges,
                            copyable: binding.copyable,
                            offline_available: binding.offline_available,
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<M5ProvenanceCardViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(M5ProvenanceCardViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(M5ProvenanceCardViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5ProvenanceCardViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5ProvenanceCardViolation>) {
        if self.schema_version != M5_PROVENANCE_CARDS_SCHEMA_VERSION {
            violations.push(M5ProvenanceCardViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_PROVENANCE_CARDS_RECORD_KIND {
            violations.push(M5ProvenanceCardViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("manifest_id", &self.manifest_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("publication_matrix_ref", &self.publication_matrix_ref),
            ("artifact_graph_ref", &self.artifact_graph_ref),
            ("docs_help_about_truth_ref", &self.docs_help_about_truth_ref),
            ("service_health_feed_ref", &self.service_health_feed_ref),
            ("release_center_model_ref", &self.release_center_model_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ProvenanceCardViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        let closed =
            |violations: &mut Vec<M5ProvenanceCardViolation>, ok: bool, field: &'static str| {
                if !ok {
                    violations.push(M5ProvenanceCardViolation::ClosedVocabularyMismatch { field });
                }
            };
        closed(
            violations,
            self.lifecycle_labels == StableClaimLevel::ALL.to_vec(),
            "lifecycle_labels",
        );
        closed(
            violations,
            self.family_kinds == M5ArtifactFamilyKind::ALL.to_vec(),
            "family_kinds",
        );
        closed(
            violations,
            self.surface_kinds == ProvenanceSurfaceKind::ALL.to_vec(),
            "surface_kinds",
        );
        closed(
            violations,
            self.badge_kinds == ProvenanceBadgeKind::ALL.to_vec(),
            "badge_kinds",
        );
        closed(
            violations,
            self.badge_states == ProvenanceBadgeState::ALL.to_vec(),
            "badge_states",
        );
        closed(
            violations,
            self.card_states == CardState::ALL.to_vec(),
            "card_states",
        );
        closed(
            violations,
            self.gap_reasons == CardGapReason::ALL.to_vec(),
            "gap_reasons",
        );
        closed(
            violations,
            self.card_actions == CardAction::ALL.to_vec(),
            "card_actions",
        );

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(M5ProvenanceCardViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5ProvenanceCardViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5ProvenanceCardViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5ProvenanceCardViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5ProvenanceCardViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5ProvenanceCardViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5ProvenanceCardViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5ProvenanceCardViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5ProvenanceCardViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in CardGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(M5ProvenanceCardViolation::GapReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(&self, row: &ProvenanceCard, violations: &mut Vec<M5ProvenanceCardViolation>) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("artifact_ref", &row.artifact_ref),
            ("artifact_summary", &row.artifact_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            (
                "exact_build.build_identity_ref",
                &row.exact_build.build_identity_ref,
            ),
            (
                "exact_build.provenance_ref",
                &row.exact_build.provenance_ref,
            ),
            ("proof_packet.packet_id", &row.proof_packet.packet_id),
            ("proof_packet.packet_ref", &row.proof_packet.packet_ref),
            (
                "proof_packet.proof_index_ref",
                &row.proof_packet.proof_index_ref,
            ),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &row.proof_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ProvenanceCardViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_badges(row, violations);
        self.validate_surfaces(row, violations);
        self.validate_convergence(row, violations);

        // The ceiling: no card may publish a label wider than its claim.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(M5ProvenanceCardViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be positive and the warn window consistent.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(M5ProvenanceCardViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(M5ProvenanceCardViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A claim whose canonical label is below the cutline forces the card to
        // inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(M5ProvenanceCardViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(M5ProvenanceCardViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.card_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A converged/on-waiver card publishes exactly the claim's canonical
            // label, carries no active reason, rides a captured within-SLO packet,
            // is owner-signed, and converges every surface on an intact identity.
            if row.published_label != row.claim_label {
                violations.push(M5ProvenanceCardViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(M5ProvenanceCardViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(M5ProvenanceCardViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(M5ProvenanceCardViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(M5ProvenanceCardViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.exact_build.linkage_intact() {
                violations.push(M5ProvenanceCardViolation::HeldWithBrokenExactBuild {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.surfaces_converge() {
                violations.push(M5ProvenanceCardViolation::HeldWithoutSurfaceConvergence {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.has_all_surfaces() {
                violations.push(M5ProvenanceCardViolation::HeldWithoutSurface {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.offline_provenance_survives() {
                violations.push(M5ProvenanceCardViolation::HeldWithoutOfflineProvenance {
                    entry_id: row.entry_id.clone(),
                });
            }
            // A converged card carries no waiver; an on-waiver card a valid one.
            match row.card_state {
                CardState::Converged => {
                    if row.waiver.is_some() {
                        violations.push(M5ProvenanceCardViolation::ClearedWithWaiver {
                            entry_id: row.entry_id.clone(),
                        });
                    }
                }
                CardState::OnWaiver => {
                    if row
                        .waiver
                        .as_ref()
                        .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                        .unwrap_or(true)
                    {
                        violations.push(M5ProvenanceCardViolation::WaiverStateWithoutWaiver {
                            entry_id: row.entry_id.clone(),
                            state: row.card_state,
                        });
                    }
                }
                _ => {}
            }
        } else {
            // A narrowing state must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(M5ProvenanceCardViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.card_state,
                    published: row.published_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(M5ProvenanceCardViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.card_state,
                });
            }
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(CardGapReason::ProofPacketStale)
            {
                violations.push(M5ProvenanceCardViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(CardGapReason::ProofPacketMissing)
            {
                violations.push(M5ProvenanceCardViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    /// Validates a card's badge set: every badge kind present once, machine
    /// tokens canonical, and — the trust guardrail — no badge overclaiming.
    fn validate_badges(
        &self,
        row: &ProvenanceCard,
        violations: &mut Vec<M5ProvenanceCardViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for badge in &row.badges {
            if !seen.insert(badge.kind) {
                violations.push(M5ProvenanceCardViolation::DuplicateBadgeKind {
                    entry_id: row.entry_id.clone(),
                    kind: badge.kind,
                });
            }
            if badge.machine_token != badge.canonical_token() {
                violations.push(M5ProvenanceCardViolation::BadgeTokenMismatch {
                    entry_id: row.entry_id.clone(),
                    kind: badge.kind,
                });
            }
            if !badge.copyable {
                violations.push(M5ProvenanceCardViolation::BadgeNotCopySafe {
                    entry_id: row.entry_id.clone(),
                    kind: badge.kind,
                });
            }
            if badge.label.trim().is_empty() {
                violations.push(M5ProvenanceCardViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: "badge.label",
                });
            }
            // Guardrail: a badge may not imply a stronger trust posture than the
            // underlying exact-build state.
            if badge.state.trust_rank() > row.canonical_badge_state(badge.kind).trust_rank() {
                violations.push(M5ProvenanceCardViolation::BadgeOverclaimsTrust {
                    entry_id: row.entry_id.clone(),
                    kind: badge.kind,
                });
            }
        }
        for kind in ProvenanceBadgeKind::ALL {
            if row.badge(kind).is_none() {
                violations.push(M5ProvenanceCardViolation::BadgeKindMissing {
                    entry_id: row.entry_id.clone(),
                    kind,
                });
            }
        }
    }

    /// Validates a card's surface bindings and the release-center-only guardrail.
    fn validate_surfaces(
        &self,
        row: &ProvenanceCard,
        violations: &mut Vec<M5ProvenanceCardViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for binding in &row.surface_bindings {
            if !seen.insert(binding.surface) {
                violations.push(M5ProvenanceCardViolation::DuplicateSurface {
                    entry_id: row.entry_id.clone(),
                    surface: binding.surface,
                });
            }
            for (field, value) in [
                ("surface_ref", &binding.surface_ref),
                ("build_identity_ref", &binding.build_identity_ref),
                ("provenance_ref", &binding.provenance_ref),
                ("summary", &binding.summary),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5ProvenanceCardViolation::EmptyField {
                        entry_id: row.entry_id.clone(),
                        field_name: field,
                    });
                }
            }
        }

        // Guardrail: a card that renders a release-center surface but omits its
        // user-visible Help/About/service-health chrome is release-center-only
        // truth Help and About cannot explain.
        if row.surface(ProvenanceSurfaceKind::ReleaseCenter).is_some()
            && !row.has_help_about_chrome()
        {
            violations.push(M5ProvenanceCardViolation::ReleaseCenterOnlyTruth {
                entry_id: row.entry_id.clone(),
            });
        }
    }

    /// Every "if this provenance aspect is thin, the matching reason must be
    /// active" rule, applied to every card regardless of held/narrowing state.
    fn validate_convergence(
        &self,
        row: &ProvenanceCard,
        violations: &mut Vec<M5ProvenanceCardViolation>,
    ) {
        let require =
            |violations: &mut Vec<M5ProvenanceCardViolation>, bad: bool, reason: CardGapReason| {
                if bad && !row.has_active_reason(reason) {
                    violations.push(M5ProvenanceCardViolation::ConvergenceGapWithoutReason {
                        entry_id: row.entry_id.clone(),
                        reason,
                    });
                }
            };
        let eb = &row.exact_build;

        require(
            violations,
            eb.signature_state == SignatureStateClass::Revoked,
            CardGapReason::SignatureRevoked,
        );
        require(
            violations,
            !matches!(
                eb.signature_state,
                SignatureStateClass::Verified | SignatureStateClass::Revoked
            ),
            CardGapReason::SignatureMissing,
        );
        require(
            violations,
            !eb.attestation_availability.holds_label(),
            CardGapReason::AttestationMissing,
        );
        require(
            violations,
            !eb.sbom_scope.holds_label(),
            CardGapReason::SbomIncomplete,
        );
        require(
            violations,
            !eb.symbol_availability.holds_label(),
            CardGapReason::SymbolsMissing,
        );
        require(
            violations,
            !eb.mirror_freshness.holds_label(),
            CardGapReason::MirrorStale,
        );
        require(
            violations,
            eb.rollback_target_ref.trim().is_empty(),
            CardGapReason::RollbackTargetMissing,
        );
        require(
            violations,
            eb.evidence_completeness != EvidenceCompleteness::Complete,
            CardGapReason::EvidenceIncomplete,
        );
        require(
            violations,
            eb.build_identity_ref.trim().is_empty() || eb.provenance_ref.trim().is_empty(),
            CardGapReason::ExactBuildLinkageBroken,
        );
        require(
            violations,
            !row.offline_provenance_survives(),
            CardGapReason::OfflineUnverifiable,
        );
        require(
            violations,
            !row.surfaces_converge(),
            CardGapReason::SurfaceDivergent,
        );
        require(
            violations,
            !row.has_all_surfaces(),
            CardGapReason::SurfaceMissing,
        );
    }

    fn validate_state_reason_coherence(
        &self,
        row: &ProvenanceCard,
        violations: &mut Vec<M5ProvenanceCardViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<M5ProvenanceCardViolation>,
                               expected: CardGapReason| {
            violations.push(M5ProvenanceCardViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.card_state,
                expected_reason: expected,
            });
        };

        match row.card_state {
            CardState::SurfaceDivergent => {
                if !row.has_active_reason(CardGapReason::SurfaceDivergent)
                    && !row.has_active_reason(CardGapReason::SurfaceMissing)
                {
                    push_incoherent(violations, CardGapReason::SurfaceDivergent);
                }
            }
            CardState::ProvenanceThin => {
                if !row
                    .active_gap_reasons
                    .iter()
                    .any(|reason| reason.is_provenance_gap())
                {
                    push_incoherent(violations, CardGapReason::SignatureMissing);
                }
            }
            CardState::Stale => {
                if !row.has_active_reason(CardGapReason::ProofPacketStale)
                    && !row.has_active_reason(CardGapReason::ProofPacketMissing)
                {
                    push_incoherent(violations, CardGapReason::ProofPacketStale);
                }
            }
            CardState::WaiverExpired => {
                if !row.has_active_reason(CardGapReason::WaiverExpired) {
                    push_incoherent(violations, CardGapReason::WaiverExpired);
                }
            }
            CardState::OwnerUnsigned => {
                if !row.has_active_reason(CardGapReason::OwnerSignoffMissing) {
                    push_incoherent(violations, CardGapReason::OwnerSignoffMissing);
                }
            }
            CardState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(M5ProvenanceCardViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.card_state,
                    });
                }
            }
            CardState::Converged => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5ProvenanceCardViolation>) {
        let covered: BTreeSet<String> = self
            .rows
            .iter()
            .map(|row| row.artifact_ref.clone())
            .collect();
        for declared in &self.release_blocking_artifact_refs {
            if !covered.contains(declared) {
                violations.push(
                    M5ProvenanceCardViolation::ReleaseBlockingArtifactUncovered {
                        artifact_ref: declared.clone(),
                    },
                );
            }
        }
        for row in &self.rows {
            if row.release_blocking
                && !self
                    .release_blocking_artifact_refs
                    .contains(&row.artifact_ref)
            {
                violations.push(M5ProvenanceCardViolation::ReleaseBlockingRowNotDeclared {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<M5ProvenanceCardViolation>) {
        if self.publication.promotion_gate.trim().is_empty() {
            violations.push(M5ProvenanceCardViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(M5ProvenanceCardViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(M5ProvenanceCardViolation::PublicationDecisionInconsistent {
                declared: self.publication.decision,
                computed,
            });
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(M5ProvenanceCardViolation::PublicationBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.publication.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(M5ProvenanceCardViolation::PublicationBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the provenance-card register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ProvenanceCardViolation {
    /// The register carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the register.
        actual: u32,
    },
    /// The register carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the register.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The register has no cards.
    EmptyRegister,
    /// The register has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Owning entry id (or a `<...>` sentinel).
        entry_id: String,
        /// Offending field name.
        field_name: &'static str,
    },
    /// Two cards share an id.
    DuplicateEntryId {
        /// Duplicated id.
        entry_id: String,
    },
    /// Two stop rules share an id.
    DuplicateStopRuleId {
        /// Duplicated rule id.
        rule_id: String,
    },
    /// A stop rule watches no labels.
    StopRuleWithoutLabels {
        /// Offending rule id.
        rule_id: String,
    },
    /// A gap reason has no stop rule watching for it.
    GapReasonWithoutStopRule {
        /// Uncovered reason.
        reason: CardGapReason,
    },
    /// A card carries the same badge kind twice.
    DuplicateBadgeKind {
        /// Owning card id.
        entry_id: String,
        /// Duplicated badge kind.
        kind: ProvenanceBadgeKind,
    },
    /// A card is missing a badge kind.
    BadgeKindMissing {
        /// Owning card id.
        entry_id: String,
        /// Missing badge kind.
        kind: ProvenanceBadgeKind,
    },
    /// A badge's machine token is not its canonical `kind:state` form.
    BadgeTokenMismatch {
        /// Owning card id.
        entry_id: String,
        /// Offending badge kind.
        kind: ProvenanceBadgeKind,
    },
    /// A badge is not marked copy-safe.
    BadgeNotCopySafe {
        /// Owning card id.
        entry_id: String,
        /// Offending badge kind.
        kind: ProvenanceBadgeKind,
    },
    /// A badge implies a stronger trust posture than the exact-build state allows.
    BadgeOverclaimsTrust {
        /// Owning card id.
        entry_id: String,
        /// Overclaiming badge kind.
        kind: ProvenanceBadgeKind,
    },
    /// A card carries the same surface binding twice.
    DuplicateSurface {
        /// Owning card id.
        entry_id: String,
        /// Duplicated surface.
        surface: ProvenanceSurfaceKind,
    },
    /// A card renders a release-center surface without its Help/About chrome.
    ReleaseCenterOnlyTruth {
        /// Owning card id.
        entry_id: String,
    },
    /// A card publishes a label wider than its public claim.
    PublishedWiderThanClaim {
        /// Owning card id.
        entry_id: String,
        /// Claim label.
        claim: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A card holds a label while its public claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Owning card id.
        entry_id: String,
        /// Claim label.
        claim: StableClaimLevel,
    },
    /// A narrowing state names no active reason.
    NarrowingWithoutReason {
        /// Owning card id.
        entry_id: String,
        /// Card state.
        state: CardState,
    },
    /// A narrowing state still publishes at or above the cutline.
    PublishedLabelNotNarrowed {
        /// Owning card id.
        entry_id: String,
        /// Card state.
        state: CardState,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A held card's published label differs from its claim.
    HeldLabelNotEqualClaim {
        /// Owning card id.
        entry_id: String,
        /// Claim label.
        claim: StableClaimLevel,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A held card carries an active gap reason.
    HeldWithActiveGap {
        /// Owning card id.
        entry_id: String,
    },
    /// A held card has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Owning card id.
        entry_id: String,
    },
    /// A held card rides a stale proof packet.
    HeldOnStalePacket {
        /// Owning card id.
        entry_id: String,
        /// Packet freshness state.
        slo_state: FreshnessSloState,
    },
    /// A held card has no owner sign-off.
    HeldWithoutSignoff {
        /// Owning card id.
        entry_id: String,
    },
    /// A held card's exact-build linkage is not intact.
    HeldWithBrokenExactBuild {
        /// Owning card id.
        entry_id: String,
    },
    /// A held card's surfaces do not all converge on the build identity.
    HeldWithoutSurfaceConvergence {
        /// Owning card id.
        entry_id: String,
    },
    /// A held card is missing a required user-visible surface.
    HeldWithoutSurface {
        /// Owning card id.
        entry_id: String,
    },
    /// A held card's provenance does not survive offline / mirror profiles.
    HeldWithoutOfflineProvenance {
        /// Owning card id.
        entry_id: String,
    },
    /// A converged card carries a waiver.
    ClearedWithWaiver {
        /// Owning card id.
        entry_id: String,
    },
    /// A thin provenance aspect is present without its narrowing reason.
    ConvergenceGapWithoutReason {
        /// Owning card id.
        entry_id: String,
        /// Required reason.
        reason: CardGapReason,
    },
    /// A card state requires a reason it does not name.
    StateReasonIncoherent {
        /// Owning card id.
        entry_id: String,
        /// Card state.
        state: CardState,
        /// Required reason.
        expected_reason: CardGapReason,
    },
    /// A waiver state names no waiver.
    WaiverStateWithoutWaiver {
        /// Owning card id.
        entry_id: String,
        /// Card state.
        state: CardState,
    },
    /// A breached packet does not name the stale-packet reason.
    BreachedPacketWithoutReason {
        /// Owning card id.
        entry_id: String,
    },
    /// A missing packet does not name the missing-packet reason.
    MissingPacketWithoutReason {
        /// Owning card id.
        entry_id: String,
    },
    /// A declared release-blocking artifact has no covering card.
    ReleaseBlockingArtifactUncovered {
        /// Uncovered artifact ref.
        artifact_ref: String,
    },
    /// A release-blocking card is not declared in the artifact-ref list.
    ReleaseBlockingRowNotDeclared {
        /// Owning card id.
        entry_id: String,
    },
    /// The declared publication decision disagrees with the computed one.
    PublicationDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// A publication blocking set disagrees with the firing stop rules.
    PublicationBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The summary counts disagree with the cards.
    SummaryMismatch,
    /// A proof packet's freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Owning card id.
        entry_id: String,
    },
}

impl fmt::Display for M5ProvenanceCardViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported register schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported register record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "register {field} is not the canonical value")
            }
            Self::EmptyRegister => write!(f, "register has no cards"),
            Self::NoStopRules => write!(f, "register has no stop rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => write!(f, "duplicate entry id {entry_id}"),
            Self::DuplicateStopRuleId { rule_id } => write!(f, "duplicate stop rule id {rule_id}"),
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutStopRule { reason } => write!(
                f,
                "gap reason {} has no stop rule watching for it",
                reason.as_str()
            ),
            Self::DuplicateBadgeKind { entry_id, kind } => {
                write!(f, "card {entry_id} carries badge {} twice", kind.as_str())
            }
            Self::BadgeKindMissing { entry_id, kind } => {
                write!(f, "card {entry_id} is missing badge {}", kind.as_str())
            }
            Self::BadgeTokenMismatch { entry_id, kind } => write!(
                f,
                "card {entry_id} badge {} machine token is not canonical",
                kind.as_str()
            ),
            Self::BadgeNotCopySafe { entry_id, kind } => write!(
                f,
                "card {entry_id} badge {} is not marked copy-safe",
                kind.as_str()
            ),
            Self::BadgeOverclaimsTrust { entry_id, kind } => write!(
                f,
                "card {entry_id} badge {} implies a stronger trust posture than the build state",
                kind.as_str()
            ),
            Self::DuplicateSurface { entry_id, surface } => write!(
                f,
                "card {entry_id} carries surface {} twice",
                surface.as_str()
            ),
            Self::ReleaseCenterOnlyTruth { entry_id } => write!(
                f,
                "card {entry_id} renders release-center truth without Help/About chrome"
            ),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "card {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "card {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "card {entry_id} state {state:?} narrows without active reason"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "card {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "card {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "card {entry_id} converges with active gap")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "card {entry_id} converges without fresh packet")
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => write!(f, "card {entry_id} converges on stale packet {slo_state:?}"),
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "card {entry_id} converges without owner signoff")
            }
            Self::HeldWithBrokenExactBuild { entry_id } => {
                write!(
                    f,
                    "card {entry_id} converges on a broken exact-build linkage"
                )
            }
            Self::HeldWithoutSurfaceConvergence { entry_id } => write!(
                f,
                "card {entry_id} converges while a surface renders a different build identity"
            ),
            Self::HeldWithoutSurface { entry_id } => {
                write!(
                    f,
                    "card {entry_id} converges without every user-visible surface"
                )
            }
            Self::HeldWithoutOfflineProvenance { entry_id } => write!(
                f,
                "card {entry_id} converges without offline / mirror provenance"
            ),
            Self::ClearedWithWaiver { entry_id } => {
                write!(f, "converged card {entry_id} carries a waiver")
            }
            Self::ConvergenceGapWithoutReason { entry_id, reason } => write!(
                f,
                "card {entry_id} provenance gap requires active reason {}",
                reason.as_str()
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "card {entry_id} state {state:?} requires reason {expected_reason:?}"
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(f, "card {entry_id} state {state:?} names no waiver")
            }
            Self::BreachedPacketWithoutReason { entry_id } => write!(
                f,
                "card {entry_id} breached packet without proof_packet_stale reason"
            ),
            Self::MissingPacketWithoutReason { entry_id } => write!(
                f,
                "card {entry_id} missing packet without proof_packet_missing reason"
            ),
            Self::ReleaseBlockingArtifactUncovered { artifact_ref } => write!(
                f,
                "release-blocking artifact {artifact_ref} has no covering card"
            ),
            Self::ReleaseBlockingRowNotDeclared { entry_id } => write!(
                f,
                "release-blocking card {entry_id} is not declared in release_blocking_artifact_refs"
            ),
            Self::PublicationDecisionInconsistent { declared, computed } => write!(
                f,
                "publication {declared:?} disagrees with computed {computed:?}"
            ),
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with cards"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "card {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for M5ProvenanceCardViolation {}

/// Loads the embedded provenance-card register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`M5ProvenanceCardRegister`].
pub fn current_m5_provenance_cards() -> Result<M5ProvenanceCardRegister, serde_json::Error> {
    serde_json::from_str(M5_PROVENANCE_CARDS_JSON)
}

#[cfg(test)]
mod tests;
