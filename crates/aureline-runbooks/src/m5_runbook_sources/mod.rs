//! Governed runbook **source descriptors** — where a runbook came from, and how
//! much authority that provenance carries.
//!
//! The [governance matrix](crate::m5_runbook_governance) freezes *what* a runbook
//! object is. This module answers the question that comes first: *where did the
//! runbook come from, and is it allowed to speak with authority?* A rendered
//! runbook is not equally trustworthy regardless of origin. Aureline distributes
//! runbooks through four distinct source channels, and each carries a different
//! standing authority:
//!
//! - [`RepoLocal`](RunbookSourceProvenance::RepoLocal) — authored and signed
//!   in-repo as first-party governed guidance ⇒ [`Authoritative`](RunbookAuthorityPosture::Authoritative).
//! - [`MirroredDocsPack`](RunbookSourceProvenance::MirroredDocsPack) — a verified
//!   mirror of an upstream authoritative docs pack ⇒ [`Mirrored`](RunbookAuthorityPosture::Mirrored).
//! - [`ManagedCatalog`](RunbookSourceProvenance::ManagedCatalog) — published
//!   through a managed runbook catalog under a signed manifest ⇒
//!   [`Managed`](RunbookAuthorityPosture::Managed).
//! - [`BrowserReference`](RunbookSourceProvenance::BrowserReference) — captured
//!   from browser-only vendor documentation ⇒ [`ReferenceOnly`](RunbookAuthorityPosture::ReferenceOnly),
//!   and it stays reference-only **unless another governed source promotes its
//!   step set into an authority-bearing posture**.
//!
//! Each [`GovernedRunbookSource`] declares its provenance class, version, a
//! [signer/provenance](RunbookSourceSigner) block, a [freshness window](FreshnessWindow),
//! its owning scope, the authority posture its class defaults to, and its
//! [export rights](RunbookSourceExportRights). The descriptor *derives* an
//! effective authority posture from that declared truth: a browser reference
//! rises only via a valid governed [promotion](RunbookSourcePromotion), and any
//! source whose proof has gone stale or expired auto-narrows back to
//! reference-only. The derivation is recomputed and compared on validation, so a
//! reference-only browser doc can never silently masquerade as a first-party
//! executable runbook.
//!
//! The [`M5RunbookSourceRegister`] is the one inspectable, serde-serializable
//! truth packet the consuming surfaces read. Every source projects the *same*
//! [badge](RunbookSourceBadge) — provenance class, effective posture, freshness,
//! signer summary, and version — into the docs browser, the incident workspace,
//! operator dashboards, and support exports, so freshness, signer, and authority
//! stay visible wherever a runbook is rendered or exported. The packet carries no
//! credential bodies or raw vendor payloads.
//!
//! - Register schema:
//!   [`schemas/runbooks/m5-runbook-source-register.schema.json`](../../../../../schemas/runbooks/m5-runbook-source-register.schema.json)
//! - Contract doc:
//!   [`docs/runbooks/m5-runbook-sources.md`](../../../../../docs/runbooks/m5-runbook-sources.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_runbook_source_register, seeded_m5_runbook_source_register_stale_mirror_narrowed,
    seeded_runbook_sources, M5_RUNBOOK_SOURCE_REGISTER_ID,
};

use serde::{Deserialize, Serialize};

/// Record-kind tag carried by [`M5RunbookSourceRegister`].
pub const M5_RUNBOOK_SOURCE_REGISTER_RECORD_KIND: &str = "m5_runbook_source_register";

/// Record-kind tag carried by [`GovernedRunbookSource`].
pub const M5_RUNBOOK_SOURCE_RECORD_KIND: &str = "m5_governed_runbook_source";

/// Schema version shared by the register and its embedded source descriptors.
pub const M5_RUNBOOK_SOURCE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the source-register schema.
pub const M5_RUNBOOK_SOURCE_REGISTER_SCHEMA_REF: &str =
    "schemas/runbooks/m5-runbook-source-register.schema.json";

/// Repo-relative path of the published source-register inventory.
pub const M5_RUNBOOK_SOURCE_REGISTER_REF: &str =
    "artifacts/runbooks/m5-runbook-source-register.json";

/// Repo-relative path of the release-grade source-register export.
pub const M5_RUNBOOK_SOURCE_REGISTER_PROOF_REF: &str =
    "artifacts/release/m5-runbook-proof/runbook-source-register.json";

/// Repo-relative path of the source-register contract doc.
pub const M5_RUNBOOK_SOURCE_DOC_REF: &str = "docs/runbooks/m5-runbook-sources.md";

/// Repo-relative directory of the source-descriptor fixtures.
pub const M5_RUNBOOK_SOURCE_FIXTURE_DIR: &str = "fixtures/runbooks/m5-source-descriptors/";

/// Prefix every governed message id in this lane carries so consumers can route it.
pub const M5_RUNBOOK_SOURCE_MESSAGE_ID_PREFIX: &str = "runbooks_sources.";

/// Where a runbook came from: its source class / distribution channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookSourceProvenance {
    /// Authored and signed in-repo as first-party governed guidance.
    RepoLocal,
    /// A verified mirror of an upstream authoritative docs pack.
    MirroredDocsPack,
    /// Published through a managed runbook catalog under a signed manifest.
    ManagedCatalog,
    /// Captured from browser-only vendor documentation; reference unless promoted.
    BrowserReference,
}

impl RunbookSourceProvenance {
    /// Every provenance class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RepoLocal,
        Self::MirroredDocsPack,
        Self::ManagedCatalog,
        Self::BrowserReference,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepoLocal => "repo_local",
            Self::MirroredDocsPack => "mirrored_docs_pack",
            Self::ManagedCatalog => "managed_catalog",
            Self::BrowserReference => "browser_reference",
        }
    }

    /// The authority posture this class carries before promotion or freshness
    /// narrowing is applied.
    pub const fn default_posture(self) -> RunbookAuthorityPosture {
        match self {
            Self::RepoLocal => RunbookAuthorityPosture::Authoritative,
            Self::MirroredDocsPack => RunbookAuthorityPosture::Mirrored,
            Self::ManagedCatalog => RunbookAuthorityPosture::Managed,
            Self::BrowserReference => RunbookAuthorityPosture::ReferenceOnly,
        }
    }

    /// The provenance kind a source of this class must carry.
    pub const fn expected_provenance_kind(self) -> RunbookProvenanceKind {
        match self {
            Self::RepoLocal => RunbookProvenanceKind::SignedFirstParty,
            Self::MirroredDocsPack => RunbookProvenanceKind::MirrorDigest,
            Self::ManagedCatalog => RunbookProvenanceKind::CatalogManifest,
            Self::BrowserReference => RunbookProvenanceKind::BrowserCapture,
        }
    }

    /// True for the browser-reference class, which has no standing authority and
    /// may rise only via a governed promotion.
    pub const fn is_browser_reference(self) -> bool {
        matches!(self, Self::BrowserReference)
    }
}

/// How authoritative a runbook source is. This is the word a user or support
/// reader sees: authoritative, mirrored, managed, or reference-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookAuthorityPosture {
    /// First-party governed guidance; carries standing executable authority.
    Authoritative,
    /// A verified mirror of an authoritative pack; executable but labeled mirror.
    Mirrored,
    /// Published through a managed catalog; executable under catalog governance.
    Managed,
    /// Reference material only; never executes as a first-party runbook.
    ReferenceOnly,
}

impl RunbookAuthorityPosture {
    /// Every posture, in declaration order (most to least authoritative).
    pub const ALL: [Self; 4] = [
        Self::Authoritative,
        Self::Mirrored,
        Self::Managed,
        Self::ReferenceOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Mirrored => "mirrored",
            Self::Managed => "managed",
            Self::ReferenceOnly => "reference_only",
        }
    }

    /// Reviewer-facing label shown in every surface badge.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Authoritative => "Authoritative",
            Self::Mirrored => "Mirrored",
            Self::Managed => "Managed",
            Self::ReferenceOnly => "Reference only",
        }
    }

    /// True when this posture lets the source's steps execute as governed
    /// runbook steps (authoritative, mirrored, or managed). Reference-only does
    /// not.
    pub const fn is_authority_bearing(self) -> bool {
        !matches!(self, Self::ReferenceOnly)
    }
}

/// What kind of signer/provenance a source carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookProvenanceKind {
    /// A first-party signature over an in-repo runbook.
    SignedFirstParty,
    /// A content digest attesting a mirror matches its upstream pack.
    MirrorDigest,
    /// A signed manifest entry from a managed catalog.
    CatalogManifest,
    /// A browser capture; unsigned, never a first-party attestation.
    BrowserCapture,
}

impl RunbookProvenanceKind {
    /// Every provenance kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SignedFirstParty,
        Self::MirrorDigest,
        Self::CatalogManifest,
        Self::BrowserCapture,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedFirstParty => "signed_first_party",
            Self::MirrorDigest => "mirror_digest",
            Self::CatalogManifest => "catalog_manifest",
            Self::BrowserCapture => "browser_capture",
        }
    }

    /// True when this kind is a cryptographically attestable first-party
    /// provenance. A browser capture is not.
    pub const fn is_first_party_attestation(self) -> bool {
        !matches!(self, Self::BrowserCapture)
    }
}

/// Freshness of a source's provenance, derived from its [`FreshnessWindow`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookSourceFreshnessState {
    /// Verified within the fresh window.
    Fresh,
    /// Verified, but past the fresh window and due for re-verification.
    Aging,
    /// Verified too long ago; no longer authority-bearing until refreshed.
    Stale,
    /// Unverifiable provenance; the signer/digest could not be confirmed.
    Expired,
}

impl RunbookSourceFreshnessState {
    /// Every freshness state, in declaration order (freshest to most stale).
    pub const ALL: [Self; 4] = [Self::Fresh, Self::Aging, Self::Stale, Self::Expired];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Aging => "aging",
            Self::Stale => "stale",
            Self::Expired => "expired",
        }
    }

    /// True when the source is recent enough to retain its declared authority.
    pub const fn is_authority_bearing(self) -> bool {
        matches!(self, Self::Fresh | Self::Aging)
    }

    /// True when freshness must narrow the source back to reference-only.
    pub const fn narrows_to_reference_only(self) -> bool {
        matches!(self, Self::Stale | Self::Expired)
    }
}

/// The inputs that derive a source's [freshness state](RunbookSourceFreshnessState).
///
/// Freshness is computed, not asserted: a source records how many days ago its
/// signer/provenance was last verified, the day-thresholds that bound its fresh
/// and stale windows, and whether that last verification actually succeeded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessWindow {
    /// Inclusive upper bound (days since verification) for [`Fresh`](RunbookSourceFreshnessState::Fresh).
    pub fresh_within_days: u32,
    /// Inclusive upper bound (days since verification) for [`Aging`](RunbookSourceFreshnessState::Aging);
    /// beyond it the source is [`Stale`](RunbookSourceFreshnessState::Stale).
    pub stale_after_days: u32,
    /// Days since the source's signer/provenance was last verified.
    pub days_since_verification: u32,
    /// Whether that last verification confirmed the signer/digest. `false` ⇒
    /// [`Expired`](RunbookSourceFreshnessState::Expired).
    pub provenance_verified: bool,
}

impl FreshnessWindow {
    /// Derives the freshness state from the window inputs.
    pub fn state(&self) -> RunbookSourceFreshnessState {
        if !self.provenance_verified {
            RunbookSourceFreshnessState::Expired
        } else if self.days_since_verification <= self.fresh_within_days {
            RunbookSourceFreshnessState::Fresh
        } else if self.days_since_verification <= self.stale_after_days {
            RunbookSourceFreshnessState::Aging
        } else {
            RunbookSourceFreshnessState::Stale
        }
    }

    /// True when the window thresholds are internally consistent.
    pub fn is_valid(&self) -> bool {
        self.fresh_within_days > 0 && self.fresh_within_days <= self.stale_after_days
    }
}

/// A source's signer/provenance block: who attests the runbook, with what kind of
/// attestation, over what version, and whether that attestation was verified.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookSourceSigner {
    /// Opaque signer identity ref (key id, publisher id, or capture origin).
    pub signer_ref: String,
    /// What kind of attestation backs the source.
    pub provenance_kind: RunbookProvenanceKind,
    /// Whether the signature/digest was verified at last sync.
    pub signature_verified: bool,
    /// The version/digest the attestation covers; must match the source version.
    pub attested_version: String,
}

impl RunbookSourceSigner {
    /// A redaction-safe one-line summary every surface badge renders.
    pub fn summary(&self) -> String {
        let verified = if self.signature_verified {
            "verified"
        } else {
            "unverified"
        };
        format!(
            "{} via {} ({verified})",
            self.signer_ref,
            self.provenance_kind.as_str()
        )
    }
}

/// A source's export rights: whether and how it appears in support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookSourceExportRights {
    /// Whether the descriptor may appear in a support export at all.
    pub exportable: bool,
    /// Redaction class applied on export.
    pub redaction_class: String,
    /// Whether the signer ref is included (vs. summarized) in an export.
    pub include_signer_in_export: bool,
    /// Always `false`: a source descriptor never carries a raw vendor body.
    pub raw_body_exportable: bool,
}

/// A governed promotion that raises a browser reference into an authority-bearing
/// posture, vouched for by another governed source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookSourcePromotion {
    /// Stable promotion id.
    pub promotion_id: String,
    /// The governed source id that vouches for this promotion. It must itself be
    /// authority-bearing and not a browser reference.
    pub promoted_by_source_id: String,
    /// The posture the browser reference is promoted to (authoritative or managed).
    pub promotes_to: RunbookAuthorityPosture,
    /// Role accountable for approving the promotion.
    pub approver_role: String,
    /// Stable message id naming the rationale; prefixed [`M5_RUNBOOK_SOURCE_MESSAGE_ID_PREFIX`].
    pub rationale_message_id: String,
}

impl RunbookSourcePromotion {
    /// True when the promotion target is an authority-bearing posture a browser
    /// reference may legitimately be promoted to.
    pub fn promotes_to_authority(&self) -> bool {
        matches!(
            self.promotes_to,
            RunbookAuthorityPosture::Authoritative | RunbookAuthorityPosture::Managed
        )
    }
}

/// A surface that renders runbook source descriptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookSourceSurface {
    /// The docs/help runbook browser.
    DocsBrowser,
    /// The incident workspace.
    IncidentWorkspace,
    /// Operator dashboards.
    OperatorDashboard,
    /// Support exports / bundles.
    SupportExport,
}

impl RunbookSourceSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DocsBrowser,
        Self::IncidentWorkspace,
        Self::OperatorDashboard,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowser => "docs_browser",
            Self::IncidentWorkspace => "incident_workspace",
            Self::OperatorDashboard => "operator_dashboard",
            Self::SupportExport => "support_export",
        }
    }
}

/// One governed runbook source descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernedRunbookSource {
    /// Record kind; must equal [`M5_RUNBOOK_SOURCE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNBOOK_SOURCE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable source id, unique within the register.
    pub source_id: String,
    /// Reviewer-facing label.
    pub source_label: String,
    /// Where the runbook came from: its source class.
    pub provenance_class: RunbookSourceProvenance,
    /// Stable version ref for the source content.
    pub version_ref: String,
    /// Signer/provenance block.
    pub signer: RunbookSourceSigner,
    /// Freshness window inputs.
    pub freshness: FreshnessWindow,
    /// Owning scope (org / team / workspace ref) accountable for the source.
    pub owning_scope: String,
    /// Owner role accountable for the source.
    pub owner_role: String,
    /// The authority posture this source's class defaults to (before derivation).
    pub declared_authority_posture: RunbookAuthorityPosture,
    /// The derived, effective authority posture after promotion and freshness.
    pub effective_authority_posture: RunbookAuthorityPosture,
    /// A governed promotion, present only for a promoted browser reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promotion: Option<RunbookSourcePromotion>,
    /// Export rights.
    pub export_rights: RunbookSourceExportRights,
    /// Stable message id; prefixed [`M5_RUNBOOK_SOURCE_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

impl GovernedRunbookSource {
    /// The derived freshness state.
    pub fn freshness_state(&self) -> RunbookSourceFreshnessState {
        self.freshness.state()
    }

    /// Derives the effective authority posture from the declared class posture,
    /// any governed promotion, and freshness. A browser reference rises only via
    /// a valid promotion; a stale or expired source narrows back to
    /// reference-only.
    pub fn derive_effective_posture(&self) -> RunbookAuthorityPosture {
        let promoted = if self.provenance_class.is_browser_reference() {
            match &self.promotion {
                Some(p) if p.promotes_to_authority() => p.promotes_to,
                _ => RunbookAuthorityPosture::ReferenceOnly,
            }
        } else {
            self.provenance_class.default_posture()
        };
        if self.freshness_state().narrows_to_reference_only() {
            RunbookAuthorityPosture::ReferenceOnly
        } else {
            promoted
        }
    }

    /// Recomputes the stored effective posture from the declared truth.
    pub fn recompute(&mut self) {
        self.effective_authority_posture = self.derive_effective_posture();
    }

    /// True when the source may execute as a governed runbook (authority-bearing
    /// effective posture *and* not stale). Reference-only sources are never
    /// executable, which is what stops a browser doc masquerading as first-party.
    pub fn is_executable(&self) -> bool {
        self.effective_authority_posture.is_authority_bearing()
            && self.freshness_state().is_authority_bearing()
    }

    /// True when the effective posture is reference-only.
    pub fn is_reference_only(&self) -> bool {
        matches!(
            self.effective_authority_posture,
            RunbookAuthorityPosture::ReferenceOnly
        )
    }

    /// The surface-independent badge every consuming surface renders for this
    /// source. The same truth — provenance, posture, freshness, signer, version —
    /// shows in the docs browser, the incident workspace, operator dashboards, and
    /// support exports.
    pub fn badge(&self) -> RunbookSourceBadge {
        RunbookSourceBadge {
            source_id: self.source_id.clone(),
            source_label: self.source_label.clone(),
            provenance_class: self.provenance_class.as_str().to_owned(),
            authority_posture: self.effective_authority_posture.as_str().to_owned(),
            authority_posture_label: self.effective_authority_posture.label().to_owned(),
            freshness_state: self.freshness_state().as_str().to_owned(),
            signer_summary: self.signer.summary(),
            version_ref: self.version_ref.clone(),
            executable: self.is_executable(),
            reference_only: self.is_reference_only(),
            promoted: self.promotion.is_some() && !self.is_reference_only(),
            detail_message_id: self.detail_message_id.clone(),
        }
    }

    /// Validates this source descriptor's invariants.
    pub fn validate(&self) -> Vec<M5RunbookSourceViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_RUNBOOK_SOURCE_RECORD_KIND
            || self.schema_version != M5_RUNBOOK_SOURCE_SCHEMA_VERSION
        {
            out.push(M5RunbookSourceViolation::WrongSourceRecordKind);
        }
        if self.source_id.trim().is_empty()
            || self.source_label.trim().is_empty()
            || self.version_ref.trim().is_empty()
            || self.owning_scope.trim().is_empty()
            || self.owner_role.trim().is_empty()
            || self.export_rights.redaction_class.trim().is_empty()
            || self.signer.signer_ref.trim().is_empty()
            || self.signer.attested_version.trim().is_empty()
        {
            out.push(M5RunbookSourceViolation::MissingIdentity);
        }
        if !self
            .detail_message_id
            .starts_with(M5_RUNBOOK_SOURCE_MESSAGE_ID_PREFIX)
        {
            out.push(M5RunbookSourceViolation::UnprefixedMessageId);
        }
        // The descriptor cannot lie about the posture its class defaults to.
        if self.declared_authority_posture != self.provenance_class.default_posture() {
            out.push(M5RunbookSourceViolation::DeclaredPostureMismatch);
        }
        // The signer's attestation must cover the declared version.
        if self.signer.attested_version != self.version_ref {
            out.push(M5RunbookSourceViolation::VersionAttestationMismatch);
        }
        // The provenance kind must match the class, and a browser capture can
        // never claim a verified first-party attestation — that would let a
        // browser doc masquerade as first-party.
        if self.signer.provenance_kind != self.provenance_class.expected_provenance_kind() {
            out.push(M5RunbookSourceViolation::ProvenanceKindMismatch);
        }
        if self.provenance_class.is_browser_reference()
            && self.signer.provenance_kind.is_first_party_attestation()
        {
            out.push(M5RunbookSourceViolation::FirstPartyMasquerade);
        }
        if matches!(
            self.signer.provenance_kind,
            RunbookProvenanceKind::BrowserCapture
        ) && self.signer.signature_verified
        {
            out.push(M5RunbookSourceViolation::FirstPartyMasquerade);
        }
        if !self.freshness.is_valid() {
            out.push(M5RunbookSourceViolation::FreshnessWindowInvalid);
        }
        // A verified signature implies verified provenance: you cannot hold a
        // checked first-party signature over a source whose provenance the
        // freshness window says was never confirmed. (The converse need not hold —
        // a browser capture can be re-confirmed current without a signature.)
        if self.signer.signature_verified && !self.freshness.provenance_verified {
            out.push(M5RunbookSourceViolation::ProvenanceVerificationMismatch);
        }
        // Only a browser reference may carry a promotion; others are already
        // authority-bearing by class.
        if let Some(promotion) = &self.promotion {
            if !self.provenance_class.is_browser_reference() {
                out.push(M5RunbookSourceViolation::PromotionOnNonBrowserSource);
            }
            if promotion.promotion_id.trim().is_empty()
                || promotion.promoted_by_source_id.trim().is_empty()
                || promotion.approver_role.trim().is_empty()
                || !promotion
                    .rationale_message_id
                    .starts_with(M5_RUNBOOK_SOURCE_MESSAGE_ID_PREFIX)
            {
                out.push(M5RunbookSourceViolation::PromotionIncomplete);
            }
            if !promotion.promotes_to_authority() {
                out.push(M5RunbookSourceViolation::PromotionTargetNotAuthoritative);
            }
        }
        // A source descriptor never carries a raw vendor body.
        if self.export_rights.raw_body_exportable {
            out.push(M5RunbookSourceViolation::RawBoundaryMaterialInExport);
        }
        // The stored effective posture must match a fresh derivation.
        if self.effective_authority_posture != self.derive_effective_posture() {
            out.push(M5RunbookSourceViolation::EffectivePostureDrift);
        }
        out
    }
}

/// The surface-independent rendered truth for one source. Every consuming surface
/// shows the same badge, so freshness, signer, and authority posture stay visible
/// wherever a runbook is rendered or exported.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookSourceBadge {
    /// Stable source id.
    pub source_id: String,
    /// Reviewer-facing label.
    pub source_label: String,
    /// Provenance class token.
    pub provenance_class: String,
    /// Effective authority posture token.
    pub authority_posture: String,
    /// Effective authority posture label (the word the reader sees).
    pub authority_posture_label: String,
    /// Freshness state token.
    pub freshness_state: String,
    /// Redaction-safe signer summary.
    pub signer_summary: String,
    /// Source version ref.
    pub version_ref: String,
    /// Whether the source may execute as a governed runbook.
    pub executable: bool,
    /// Whether the source is reference-only.
    pub reference_only: bool,
    /// Whether a governed promotion raised this source's authority.
    pub promoted: bool,
    /// Stable message id; prefixed [`M5_RUNBOOK_SOURCE_MESSAGE_ID_PREFIX`].
    pub detail_message_id: String,
}

/// Which surfaces expose the source register. Every flag must hold so the
/// descriptor is visible wherever a runbook is rendered or exported.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookSourceSurfaceExposure {
    /// The docs/help runbook browser exposes the source register.
    pub docs_browser_exposes_sources: bool,
    /// The incident workspace exposes the source register.
    pub incident_workspace_exposes_sources: bool,
    /// Operator dashboards expose the source register.
    pub operator_dashboard_exposes_sources: bool,
    /// Support exports expose the source register.
    pub support_export_exposes_sources: bool,
}

impl RunbookSourceSurfaceExposure {
    /// The canonical exposure: every surface renders the descriptor.
    pub const fn all_surfaces() -> Self {
        Self {
            docs_browser_exposes_sources: true,
            incident_workspace_exposes_sources: true,
            operator_dashboard_exposes_sources: true,
            support_export_exposes_sources: true,
        }
    }

    /// True when every surface exposes the register.
    pub const fn all_expose(&self) -> bool {
        self.docs_browser_exposes_sources
            && self.incident_workspace_exposes_sources
            && self.operator_dashboard_exposes_sources
            && self.support_export_exposes_sources
    }
}

/// Self-describing controlled-vocabulary set so the packet resolves every token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookSourceVocabulary {
    /// Provenance-class tokens.
    pub provenance_classes: Vec<String>,
    /// Authority-posture tokens.
    pub authority_postures: Vec<String>,
    /// Provenance-kind tokens.
    pub provenance_kinds: Vec<String>,
    /// Freshness-state tokens.
    pub freshness_states: Vec<String>,
    /// Surface tokens.
    pub surfaces: Vec<String>,
}

impl RunbookSourceVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            provenance_classes: RunbookSourceProvenance::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            authority_postures: RunbookAuthorityPosture::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            provenance_kinds: RunbookProvenanceKind::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            freshness_states: RunbookSourceFreshnessState::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            surfaces: RunbookSourceSurface::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Conformance review for the source register. Every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookSourceConformance {
    /// Every source declares class, version, signer, freshness, scope, and export.
    pub every_source_declares_class_version_signer_freshness_scope_and_export: bool,
    /// The register distinguishes authoritative, mirrored, managed, and reference-only.
    pub posture_distinguishes_authoritative_mirrored_managed_reference_only: bool,
    /// A browser reference is reference-only unless a governed promotion raises it.
    pub browser_reference_is_reference_only_unless_governed_promotion: bool,
    /// Freshness, signer, and posture stay visible on every surface.
    pub freshness_signer_and_posture_visible_on_every_surface: bool,
    /// A reference-only source can never present as a first-party executable runbook.
    pub reference_only_cannot_masquerade_as_first_party_executable: bool,
    /// A stale or expired source auto-narrows back to reference-only.
    pub stale_or_expired_sources_auto_narrow_to_reference_only: bool,
    /// The export carries no raw boundary material.
    pub export_carries_no_raw_boundary_material: bool,
    /// The register is generated from the same checked-in source descriptors.
    pub generated_from_checked_in_sources: bool,
}

impl RunbookSourceConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.every_source_declares_class_version_signer_freshness_scope_and_export
            && self.posture_distinguishes_authoritative_mirrored_managed_reference_only
            && self.browser_reference_is_reference_only_unless_governed_promotion
            && self.freshness_signer_and_posture_visible_on_every_surface
            && self.reference_only_cannot_masquerade_as_first_party_executable
            && self.stale_or_expired_sources_auto_narrow_to_reference_only
            && self.export_carries_no_raw_boundary_material
            && self.generated_from_checked_in_sources
    }
}

/// Constructor input for [`M5RunbookSourceRegister::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RunbookSourceRegisterInput {
    /// Stable register id.
    pub register_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the register was computed as-of.
    pub evaluated_at: String,
    /// The governed source descriptors.
    pub sources: Vec<GovernedRunbookSource>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 runbook source register: the inventory of governed runbook
/// sources and the badges every consuming surface reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunbookSourceRegister {
    /// Record kind; must equal [`M5_RUNBOOK_SOURCE_REGISTER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNBOOK_SOURCE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable register id.
    pub register_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the register was computed as-of.
    pub evaluated_at: String,
    /// The governed source descriptors.
    pub sources: Vec<GovernedRunbookSource>,
    /// One surface-independent badge per source, in source order.
    pub badges: Vec<RunbookSourceBadge>,
    /// Which surfaces expose the register.
    pub surface_exposure: RunbookSourceSurfaceExposure,
    /// Controlled-vocabulary set.
    pub vocabulary: RunbookSourceVocabulary,
    /// Conformance review block.
    pub conformance: RunbookSourceConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RunbookSourceRegister {
    /// Builds a register from seed input, deriving each source's badge and the
    /// conformance review from the source descriptors.
    pub fn new(input: M5RunbookSourceRegisterInput) -> Self {
        let badges: Vec<RunbookSourceBadge> = input.sources.iter().map(|s| s.badge()).collect();
        let conformance = derive_conformance(&input.sources);
        Self {
            record_kind: M5_RUNBOOK_SOURCE_REGISTER_RECORD_KIND.to_owned(),
            schema_version: M5_RUNBOOK_SOURCE_SCHEMA_VERSION,
            register_id: input.register_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            sources: input.sources,
            badges,
            surface_exposure: RunbookSourceSurfaceExposure::all_surfaces(),
            vocabulary: RunbookSourceVocabulary::canonical(),
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Finds a source by id.
    pub fn source(&self, source_id: &str) -> Option<&GovernedRunbookSource> {
        self.sources.iter().find(|s| s.source_id == source_id)
    }

    /// The badges a given surface renders. Every surface shows the same truth;
    /// support exports omit any source whose export rights forbid it.
    pub fn badges_for_surface(&self, surface: RunbookSourceSurface) -> Vec<RunbookSourceBadge> {
        self.sources
            .iter()
            .filter(|s| {
                surface != RunbookSourceSurface::SupportExport || s.export_rights.exportable
            })
            .map(|s| s.badge())
            .collect()
    }

    /// Validates the register's invariants.
    pub fn validate(&self) -> Vec<M5RunbookSourceViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_RUNBOOK_SOURCE_REGISTER_RECORD_KIND {
            out.push(M5RunbookSourceViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RUNBOOK_SOURCE_SCHEMA_VERSION {
            out.push(M5RunbookSourceViolation::WrongSchemaVersion);
        }
        if self.register_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5RunbookSourceViolation::MissingIdentity);
        }
        if self.sources.is_empty() {
            out.push(M5RunbookSourceViolation::RegisterHasNoSources);
        }

        // Unique source ids.
        let mut seen = std::collections::BTreeSet::new();
        for source in &self.sources {
            if !seen.insert(source.source_id.as_str()) {
                out.push(M5RunbookSourceViolation::DuplicateSourceId);
            }
            out.extend(source.validate());
        }

        // Every promotion must be vouched for by a governed source that is itself
        // authority-bearing and not a browser reference.
        for source in &self.sources {
            if let Some(promotion) = &source.promotion {
                match self.source(&promotion.promoted_by_source_id) {
                    Some(voucher)
                        if voucher.source_id != source.source_id
                            && !voucher.provenance_class.is_browser_reference()
                            && voucher.effective_authority_posture.is_authority_bearing() => {}
                    _ => out.push(M5RunbookSourceViolation::PromotionReferenceInvalid),
                }
            }
        }

        // The badges must recompute exactly from the sources.
        let expected: Vec<RunbookSourceBadge> = self.sources.iter().map(|s| s.badge()).collect();
        if expected != self.badges {
            out.push(M5RunbookSourceViolation::BadgeDrift);
        }

        if !self.surface_exposure.all_expose() {
            out.push(M5RunbookSourceViolation::SurfaceExposureIncomplete);
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5RunbookSourceViolation::VocabularyMismatch);
        }
        if self.conformance != derive_conformance(&self.sources) || !self.conformance.all_hold() {
            out.push(M5RunbookSourceViolation::ConformanceReviewFailed);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 runbook source register serializes"),
        ) {
            out.push(M5RunbookSourceViolation::RawBoundaryMaterialInExport);
        }

        out
    }

    /// Deterministic export-safe JSON for the register.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 runbook source register serializes")
    }

    /// Deterministic Markdown proof for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Runbook Source Register\n\n");
        out.push_str(&format!("- Register: `{}`\n", self.register_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!("- Sources: {}\n", self.sources.len()));
        let executable = self.badges.iter().filter(|b| b.executable).count();
        let reference = self.badges.iter().filter(|b| b.reference_only).count();
        out.push_str(&format!(
            "- Executable: {executable} · Reference-only: {reference}\n"
        ));
        out.push_str(
            "- Exposed on: docs browser, incident workspace, operator dashboards, support exports\n",
        );

        out.push_str("\n## Governed runbook sources\n\n");
        out.push_str("| Source | Provenance | Authority | Freshness | Executable | Signer |\n");
        out.push_str("|--------|------------|-----------|-----------|------------|--------|\n");
        for source in &self.sources {
            let badge = source.badge();
            out.push_str(&format!(
                "| `{}` | `{}` | {} | `{}` | {} | {} |\n",
                badge.source_id,
                badge.provenance_class,
                badge.authority_posture_label,
                badge.freshness_state,
                if badge.executable { "yes" } else { "no" },
                badge.signer_summary,
            ));
        }
        out
    }
}

/// Derives the conformance review from the source descriptors so the stored block
/// reflects the actual register rather than an assertion.
fn derive_conformance(sources: &[GovernedRunbookSource]) -> RunbookSourceConformance {
    let every_declares = !sources.is_empty()
        && sources.iter().all(|s| {
            s.validate()
                .iter()
                .all(|v| !matches!(v, M5RunbookSourceViolation::MissingIdentity))
                && s.freshness.is_valid()
        });

    // The register distinguishes the four authority kinds when it carries all
    // four provenance classes — a true claim independent of any single source's
    // current freshness narrowing.
    let classes: std::collections::BTreeSet<RunbookSourceProvenance> =
        sources.iter().map(|s| s.provenance_class).collect();
    let distinguishes = RunbookSourceProvenance::ALL
        .iter()
        .all(|c| classes.contains(c));

    let browser_reference_governed = sources
        .iter()
        .filter(|s| s.provenance_class.is_browser_reference())
        .all(|s| match &s.promotion {
            Some(p) if p.promotes_to_authority() && s.freshness_state().is_authority_bearing() => {
                s.effective_authority_posture.is_authority_bearing()
            }
            _ => s.is_reference_only(),
        });

    let posture_visible = sources.iter().all(|s| {
        let badge = s.badge();
        !badge.authority_posture.is_empty()
            && !badge.freshness_state.is_empty()
            && !badge.signer_summary.is_empty()
    });

    let no_masquerade = sources
        .iter()
        .all(|s| !(s.is_reference_only() && s.is_executable()));

    let stale_narrows = sources
        .iter()
        .all(|s| !s.freshness_state().narrows_to_reference_only() || s.is_reference_only());

    let export_clean = sources.iter().all(|s| !s.export_rights.raw_body_exportable);

    let generated = sources
        .iter()
        .all(|s| s.effective_authority_posture == s.derive_effective_posture());

    RunbookSourceConformance {
        every_source_declares_class_version_signer_freshness_scope_and_export: every_declares,
        posture_distinguishes_authoritative_mirrored_managed_reference_only: distinguishes,
        browser_reference_is_reference_only_unless_governed_promotion: browser_reference_governed,
        freshness_signer_and_posture_visible_on_every_surface: posture_visible,
        reference_only_cannot_masquerade_as_first_party_executable: no_masquerade,
        stale_or_expired_sources_auto_narrow_to_reference_only: stale_narrows,
        export_carries_no_raw_boundary_material: export_clean,
        generated_from_checked_in_sources: generated,
    }
}

/// Validation failures for the runbook source-descriptor lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunbookSourceViolation {
    /// The register record kind is wrong.
    WrongRecordKind,
    /// The register schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// The register declares no sources.
    RegisterHasNoSources,
    /// Two sources share a source id.
    DuplicateSourceId,
    /// An embedded source record carries the wrong record kind or schema version.
    WrongSourceRecordKind,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// A source's declared posture does not match its provenance class default.
    DeclaredPostureMismatch,
    /// A source's signer attests a version other than the declared one.
    VersionAttestationMismatch,
    /// A source's provenance kind does not match its provenance class.
    ProvenanceKindMismatch,
    /// A browser capture claims a verified first-party attestation.
    FirstPartyMasquerade,
    /// A source's freshness window thresholds are inconsistent.
    FreshnessWindowInvalid,
    /// A source claims a verified signature over unconfirmed provenance.
    ProvenanceVerificationMismatch,
    /// A promotion sits on a non-browser source.
    PromotionOnNonBrowserSource,
    /// A promotion is missing its id, voucher, approver, or rationale.
    PromotionIncomplete,
    /// A promotion targets a non-authority-bearing posture.
    PromotionTargetNotAuthoritative,
    /// A promotion's voucher source is missing, self-referential, or not governed.
    PromotionReferenceInvalid,
    /// A source's stored effective posture drifted from a fresh derivation.
    EffectivePostureDrift,
    /// The stored badges drifted from a fresh recompute.
    BadgeDrift,
    /// A surface does not expose the source register.
    SurfaceExposureIncomplete,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// The export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5RunbookSourceViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::RegisterHasNoSources => "register_has_no_sources",
            Self::DuplicateSourceId => "duplicate_source_id",
            Self::WrongSourceRecordKind => "wrong_source_record_kind",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::DeclaredPostureMismatch => "declared_posture_mismatch",
            Self::VersionAttestationMismatch => "version_attestation_mismatch",
            Self::ProvenanceKindMismatch => "provenance_kind_mismatch",
            Self::FirstPartyMasquerade => "first_party_masquerade",
            Self::FreshnessWindowInvalid => "freshness_window_invalid",
            Self::ProvenanceVerificationMismatch => "provenance_verification_mismatch",
            Self::PromotionOnNonBrowserSource => "promotion_on_non_browser_source",
            Self::PromotionIncomplete => "promotion_incomplete",
            Self::PromotionTargetNotAuthoritative => "promotion_target_not_authoritative",
            Self::PromotionReferenceInvalid => "promotion_reference_invalid",
            Self::EffectivePostureDrift => "effective_posture_drift",
            Self::BadgeDrift => "badge_drift",
            Self::SurfaceExposureIncomplete => "surface_exposure_incomplete",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked boundary material. Mirrors the
/// redaction posture of the governance lane.
const FORBIDDEN_KEY_SUBSTRINGS: [&str; 6] = [
    "credential",
    "secret",
    "password",
    "api_key",
    "raw_payload",
    "bearer_token",
];

/// Scans a serialized packet for forbidden boundary material. Returns true when a
/// key (case-insensitive) contains a forbidden substring.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => map.iter().any(|(key, child)| {
            let lower = key.to_ascii_lowercase();
            FORBIDDEN_KEY_SUBSTRINGS
                .iter()
                .any(|needle| lower.contains(needle))
                || json_contains_forbidden_boundary_material(child)
        }),
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
