//! Stricter trust-decision identity rendering for the new M5 install/update,
//! remote-attach, collaboration-invite, route-share, and policy-review surfaces.
//!
//! Install/update prompts, remote attach, collaboration invites, route shares,
//! and policy reviews are *trust decisions*, not ordinary browsing panes: the
//! user is about to grant trust, attach, install, or share based on a publisher
//! name, a remote-host label, a collaborator identity, a route share, or a
//! block of policy-review text. Those identity strings must render in a stronger
//! *strong-decision* mode than ordinary browsing surfaces, so a suspicious or
//! ambiguous identity is inspectable *before* trust is granted.
//!
//! This lane sits on the same shared content-integrity policy library as its
//! siblings: it runs the shared suspicious-content detector
//! ([`crate::detect_suspicious_content`]) over each identity and derives the
//! shared safe-inspection escape ([`crate::escape_for_safe_inspection`]) rather
//! than inventing a parallel detector. The frozen content-integrity matrix in
//! [`crate::freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix`]
//! locks the static qualification each surface may claim,
//! [`crate::m5_trust_class_ladder`] resolves the runtime trust class of active
//! content, and [`crate::m5_safe_preview_limited_mode`] guards expensive or
//! unsafe renders. This lane covers the orthogonal *identity-rendering* gap they
//! leave open: a trust-decision surface must render identity strictly and keep
//! raw identity inspection reachable, while an ordinary browsing pane need not.
//!
//! Every trust-decision surface resolves to
//! [`M5IdentityRenderMode::StrongDecision`] — strictly stronger than the
//! [`M5IdentityRenderMode::OrdinaryBrowsing`] baseline an everyday pane uses.
//! Strong-decision rendering shows the full identity without truncation,
//! highlights bidi/invisible/confusable cues from the shared detector, keeps an
//! open-raw and a copy-escaped affordance reachable, and keeps the same stricter
//! rendering in product, exported review packets, and support handoff artifacts.
//! Suspicious bytes are never normalized away and the escaped inspection form
//! never masquerades as the raw bytes.
//!
//! The packet is metadata only: it carries opaque refs to the raw identity and
//! the shared detector's escaped/safe inspection form, never the raw provider
//! identity bytes themselves, so no credential or raw payload crosses the export
//! boundary.
//!
//! The boundary schema is
//! [`schemas/security/m5-trust-decision-identity.schema.json`](../../../../schemas/security/m5-trust-decision-identity.schema.json).
//! The contract doc is
//! [`docs/security/m5/m5_trust_decision_identity.md`](../../../../docs/security/m5/m5_trust_decision_identity.md).
//! The protected fixture directory is
//! [`fixtures/security/m5/m5_trust_decision_identity/`](../../../../fixtures/security/m5/m5_trust_decision_identity/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::detector::{detect_suspicious_content, escape_for_safe_inspection};

/// Stable record-kind tag carried by [`M5TrustDecisionIdentityPacket`].
pub const M5_TRUST_DECISION_IDENTITY_RECORD_KIND: &str = "m5_trust_decision_identity_packet";

/// Integer schema version for the M5 trust-decision identity packet.
pub const M5_TRUST_DECISION_IDENTITY_SCHEMA_VERSION: u32 = 1;

/// Stable packet id minted by [`frozen_m5_trust_decision_identity_packet`].
pub const M5_TRUST_DECISION_IDENTITY_PACKET_ID: &str = "m5-trust-decision-identity:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_TRUST_DECISION_IDENTITY_SCHEMA_REF: &str =
    "schemas/security/m5-trust-decision-identity.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_TRUST_DECISION_IDENTITY_DOC_REF: &str =
    "docs/security/m5/m5_trust_decision_identity.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_TRUST_DECISION_IDENTITY_FIXTURE_DIR: &str =
    "fixtures/security/m5/m5_trust_decision_identity";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TRUST_DECISION_IDENTITY_ARTIFACT_REF: &str =
    "artifacts/security/m5/m5_trust_decision_identity/support_export.json";

/// Repo-relative path of the sibling suspicious-text detector-parity contract.
pub const M5_TRUST_DECISION_IDENTITY_SUSPICIOUS_TEXT_CONTRACT_REF: &str =
    "schemas/security/m5-suspicious-text-detector-parity.schema.json";

/// Repo-relative path of the sibling trust-class ladder contract.
pub const M5_TRUST_DECISION_IDENTITY_TRUST_CLASS_CONTRACT_REF: &str =
    "schemas/security/m5-trust-class-ladder.schema.json";

/// A new M5 surface that renders an identity as part of a *trust decision*
/// (install/update, attach, invite, share, or policy review) rather than as
/// ordinary browsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustDecisionSurface {
    /// A marketplace install/update prompt showing a publisher/package name.
    PublisherPackageName,
    /// A remote-attach prompt showing a remote-host label.
    RemoteHostLabel,
    /// A collaboration invite showing a collaborator identity.
    CollaboratorIdentity,
    /// A route-share prompt showing the share target.
    RouteShare,
    /// A policy review showing reviewer-authored policy text.
    PolicyReview,
}

impl M5TrustDecisionSurface {
    /// Every trust-decision surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PublisherPackageName,
        Self::RemoteHostLabel,
        Self::CollaboratorIdentity,
        Self::RouteShare,
        Self::PolicyReview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublisherPackageName => "publisher_package_name",
            Self::RemoteHostLabel => "remote_host_label",
            Self::CollaboratorIdentity => "collaborator_identity",
            Self::RouteShare => "route_share",
            Self::PolicyReview => "policy_review",
        }
    }

    /// The trust decision this surface gates.
    pub const fn decision_action(self) -> M5TrustDecisionAction {
        match self {
            Self::PublisherPackageName => M5TrustDecisionAction::InstallOrUpdate,
            Self::RemoteHostLabel => M5TrustDecisionAction::RemoteAttach,
            Self::CollaboratorIdentity => M5TrustDecisionAction::CollaborationInvite,
            Self::RouteShare => M5TrustDecisionAction::RouteShare,
            Self::PolicyReview => M5TrustDecisionAction::PolicyReview,
        }
    }

    /// Human-readable kind of identity this surface renders.
    pub const fn identity_kind(self) -> &'static str {
        match self {
            Self::PublisherPackageName => "publisher/package name",
            Self::RemoteHostLabel => "remote-host label",
            Self::CollaboratorIdentity => "collaborator identity",
            Self::RouteShare => "route-share target",
            Self::PolicyReview => "policy-review text",
        }
    }
}

/// The trust decision a [`M5TrustDecisionSurface`] gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustDecisionAction {
    /// Install or update a marketplace package.
    InstallOrUpdate,
    /// Attach to a remote host.
    RemoteAttach,
    /// Accept or extend a collaboration invite.
    CollaborationInvite,
    /// Share a route.
    RouteShare,
    /// Approve or reject a policy review.
    PolicyReview,
}

impl M5TrustDecisionAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallOrUpdate => "install_or_update",
            Self::RemoteAttach => "remote_attach",
            Self::CollaborationInvite => "collaboration_invite",
            Self::RouteShare => "route_share",
            Self::PolicyReview => "policy_review",
        }
    }

    /// The distinct decision verb the affordance carries (never a plain "Open").
    pub const fn verb(self) -> &'static str {
        match self {
            Self::InstallOrUpdate => "Install / Update",
            Self::RemoteAttach => "Attach",
            Self::CollaborationInvite => "Invite",
            Self::RouteShare => "Share",
            Self::PolicyReview => "Approve",
        }
    }
}

/// How strictly a surface renders an identity.
///
/// Trust-decision surfaces resolve to [`Self::StrongDecision`]; ordinary
/// browsing panes use the weaker [`Self::OrdinaryBrowsing`] baseline. The
/// strength rank lets [`M5TrustDecisionIdentityPacket::validate`] prove a
/// trust-decision surface renders *strictly stronger* than ordinary browsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IdentityRenderMode {
    /// The weaker baseline an everyday browsing pane uses.
    OrdinaryBrowsing,
    /// The stricter mode a trust-decision surface must use.
    StrongDecision,
}

impl M5IdentityRenderMode {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryBrowsing => "ordinary_browsing",
            Self::StrongDecision => "strong_decision",
        }
    }

    /// Ordinal strength rank; higher is stricter.
    pub const fn strength_rank(self) -> u32 {
        match self {
            Self::OrdinaryBrowsing => 1,
            Self::StrongDecision => 2,
        }
    }
}

/// Strength rank of the ordinary-browsing baseline, recorded for contrast.
pub const M5_ORDINARY_BROWSING_STRENGTH_RANK: u32 =
    M5IdentityRenderMode::OrdinaryBrowsing.strength_rank();

/// Strength rank of the strong-decision mode every trust surface must use.
pub const M5_STRONG_DECISION_STRENGTH_RANK: u32 =
    M5IdentityRenderMode::StrongDecision.strength_rank();

/// A suspicious-identity warning surfaced on a trust-decision surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IdentityWarningKind {
    /// The identity contains bidi-control or invisible-formatting codepoints.
    BidiOrInvisibleBytes,
    /// The identity mixes scripts so a glyph is not the letter it appears to be.
    MixedScriptConfusable,
}

impl M5IdentityWarningKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BidiOrInvisibleBytes => "bidi_or_invisible_bytes",
            Self::MixedScriptConfusable => "mixed_script_confusable",
        }
    }
}

/// A typed identity warning shown above the trust-decision affordance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IdentityWarning {
    /// Warning kind.
    pub kind: M5IdentityWarningKind,
    /// Warning-kind token, redundant with [`Self::kind`] for export readers.
    pub kind_token: String,
    /// Human-readable warning message.
    pub message: String,
}

/// The kind of raw-identity inspection affordance a strong-decision surface
/// offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IdentityInspectionActionKind {
    /// Open the raw identity bytes behind the rendered label.
    OpenRawIdentity,
    /// Copy the escaped, safe-inspection form of the identity.
    CopyEscapedIdentity,
    /// Inspect the suspicious codepoints the detector flagged.
    InspectCodepoints,
}

impl M5IdentityInspectionActionKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenRawIdentity => "open_raw_identity",
            Self::CopyEscapedIdentity => "copy_escaped_identity",
            Self::InspectCodepoints => "inspect_codepoints",
        }
    }
}

/// A raw-identity inspection affordance with its target and label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IdentityInspectionAction {
    /// Action kind.
    pub kind: M5IdentityInspectionActionKind,
    /// Action-kind token, redundant with [`Self::kind`] for export readers.
    pub kind_token: String,
    /// Opaque ref the action targets (raw identity subject or escaped form).
    pub target_ref: String,
    /// Human-readable action label.
    pub label: String,
}

/// Inputs describing one trust-decision surface's identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5TrustDecisionIdentityInput<'a> {
    /// Surface this input describes.
    pub surface: M5TrustDecisionSurface,
    /// Opaque ref of the trust-decision subject (listing, host, invite, …).
    pub subject_ref: &'a str,
    /// Opaque ref the open-raw affordance targets for the raw identity bytes.
    pub raw_identity_ref: &'a str,
    /// The rendered identity text, fed to the shared suspicious-content
    /// detector. Only its escaped form and finding summary are exported.
    pub identity_text: &'a str,
}

/// Inputs needed to project the M5 trust-decision identity packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5TrustDecisionIdentitySeed<'a> {
    /// Stable case id shared by all surface projections.
    pub case_id: &'a str,
    /// Per-surface inputs, one per [`M5TrustDecisionSurface::ALL`].
    pub surface_inputs: [M5TrustDecisionIdentityInput<'a>; 5],
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: &'a str,
}

/// The resolved strong-decision identity rendering for one surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustDecisionIdentityProjection {
    /// Surface this projection describes.
    pub surface: M5TrustDecisionSurface,
    /// Surface token, redundant with [`Self::surface`] for export readers.
    pub surface_token: String,
    /// The trust decision this surface gates.
    pub decision_action: M5TrustDecisionAction,
    /// Decision-action token.
    pub decision_action_token: String,
    /// Distinct decision verb the affordance carries.
    pub decision_verb: String,
    /// Opaque trust-decision subject ref.
    pub subject_ref: String,
    /// Opaque ref the open-raw affordance targets.
    pub raw_identity_ref: String,
    /// Identity rendering mode (always [`M5IdentityRenderMode::StrongDecision`]).
    pub render_mode: M5IdentityRenderMode,
    /// Render-mode token.
    pub render_mode_token: String,
    /// Strength rank of the resolved mode; stricter than ordinary browsing.
    pub render_strength_rank: u32,
    /// The shared detector's escaped, safe-inspection form of the identity.
    pub identity_inspection_escaped: String,
    /// Character length of the rendered identity (no content, proves no
    /// truncation against [`Self::displayed_char_len`]).
    pub identity_char_len: usize,
    /// Character length actually displayed; equals [`Self::identity_char_len`].
    pub displayed_char_len: usize,
    /// Whether the full identity is shown without truncation.
    pub full_identity_shown: bool,
    /// Whether the shared detector flagged suspicious cues in the identity.
    pub has_suspicious_cues: bool,
    /// Number of suspicious findings the shared detector reported.
    pub suspicious_finding_count: usize,
    /// Distinct detector threat-class tokens present, sorted.
    pub threat_classes: Vec<String>,
    /// Shared-detector outcome token for this identity.
    pub detector_outcome_token: String,
    /// Whether the rendered glyphs differ materially from the raw bytes.
    pub rendered_differs_from_raw: bool,
    /// Warnings shown above the affordance, in declaration order.
    pub warnings: Vec<M5IdentityWarning>,
    /// Raw-identity inspection affordances offered.
    pub actions: Vec<M5IdentityInspectionAction>,
    /// Whether raw identity inspection stays reachable (always true).
    pub raw_inspection_reachable: bool,
    /// Whether an escaped-copy path stays reachable (always true).
    pub escaped_copy_reachable: bool,
    /// Whether the decision affordance is visually distinct from browsing.
    pub decision_affordance_distinct: bool,
    /// Whether the stricter rendering is preserved in product.
    pub preserved_in_product: bool,
    /// Whether the stricter rendering is preserved in exported review packets.
    pub preserved_in_exported_review_packet: bool,
    /// Whether the stricter rendering is preserved in support handoff artifacts.
    pub preserved_in_support_handoff: bool,
    /// Human-readable rationale for the resolved rendering.
    pub rationale: String,
}

impl M5TrustDecisionIdentityProjection {
    /// Whether an open-raw-identity affordance is offered.
    pub fn has_open_raw_identity(&self) -> bool {
        self.actions
            .iter()
            .any(|a| a.kind == M5IdentityInspectionActionKind::OpenRawIdentity)
    }

    /// Whether a copy-escaped-identity affordance is offered.
    pub fn has_copy_escaped_identity(&self) -> bool {
        self.actions
            .iter()
            .any(|a| a.kind == M5IdentityInspectionActionKind::CopyEscapedIdentity)
    }

    /// Whether an inspect-codepoints affordance is offered.
    pub fn has_inspect_codepoints(&self) -> bool {
        self.actions
            .iter()
            .any(|a| a.kind == M5IdentityInspectionActionKind::InspectCodepoints)
    }

    /// Whether this surface renders strictly stronger than ordinary browsing.
    pub fn stronger_than_ordinary_browsing(&self) -> bool {
        self.render_mode == M5IdentityRenderMode::StrongDecision
            && self.render_strength_rank > M5_ORDINARY_BROWSING_STRENGTH_RANK
    }

    /// Whether the stricter rendering is preserved across all carriers.
    pub fn preserved_everywhere(&self) -> bool {
        self.preserved_in_product
            && self.preserved_in_exported_review_packet
            && self.preserved_in_support_handoff
    }
}

/// Trust-decision identity review block; every field encodes a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustDecisionIdentityReview {
    /// One shared content-integrity policy library governs every surface.
    pub one_shared_policy_library_governs_all_surfaces: bool,
    /// Every trust-decision surface uses the strong-decision render mode.
    pub trust_decision_surfaces_use_strong_decision_mode: bool,
    /// Strong-decision rendering is strictly stronger than ordinary browsing.
    pub strong_decision_stricter_than_ordinary_browsing: bool,
    /// The full identity is shown without truncation on every surface.
    pub full_identity_shown_without_truncation: bool,
    /// Raw identity inspection stays reachable on every surface.
    pub raw_identity_inspection_reachable_everywhere: bool,
    /// Suspicious bytes are surfaced, never normalized away.
    pub suspicious_bytes_never_normalized_away: bool,
    /// The escaped inspection form never masquerades as the raw bytes.
    pub escaped_form_never_masquerades_as_raw: bool,
    /// Suspicious identities carry a visible warning and codepoint inspection.
    pub suspicious_identities_carry_warning_and_inspection: bool,
    /// The decision affordance is visually distinct from ordinary browsing.
    pub decision_affordance_distinct_from_browsing: bool,
    /// The stricter rendering is preserved in product, export, and handoff.
    pub stricter_rendering_preserved_in_product_export_and_handoff: bool,
}

impl M5TrustDecisionIdentityReview {
    /// The frozen, all-invariants-hold review block.
    pub const fn frozen() -> Self {
        Self {
            one_shared_policy_library_governs_all_surfaces: true,
            trust_decision_surfaces_use_strong_decision_mode: true,
            strong_decision_stricter_than_ordinary_browsing: true,
            full_identity_shown_without_truncation: true,
            raw_identity_inspection_reachable_everywhere: true,
            suspicious_bytes_never_normalized_away: true,
            escaped_form_never_masquerades_as_raw: true,
            suspicious_identities_carry_warning_and_inspection: true,
            decision_affordance_distinct_from_browsing: true,
            stricter_rendering_preserved_in_product_export_and_handoff: true,
        }
    }

    fn all_hold(&self) -> bool {
        self.one_shared_policy_library_governs_all_surfaces
            && self.trust_decision_surfaces_use_strong_decision_mode
            && self.strong_decision_stricter_than_ordinary_browsing
            && self.full_identity_shown_without_truncation
            && self.raw_identity_inspection_reachable_everywhere
            && self.suspicious_bytes_never_normalized_away
            && self.escaped_form_never_masquerades_as_raw
            && self.suspicious_identities_carry_warning_and_inspection
            && self.decision_affordance_distinct_from_browsing
            && self.stricter_rendering_preserved_in_product_export_and_handoff
    }
}

/// Cross-surface strong-decision identity rendering packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustDecisionIdentityPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this packet.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Case id shared by all projections.
    pub case_id: String,
    /// Number of trust-decision surfaces projected.
    pub surface_count: usize,
    /// Number of surfaces whose identity carried suspicious cues.
    pub suspicious_surface_count: usize,
    /// Strength rank of the ordinary-browsing baseline (for contrast).
    pub ordinary_browsing_strength_rank: u32,
    /// Strength rank of the strong-decision mode every surface uses.
    pub strong_decision_strength_rank: u32,
    /// Distinct render-mode tokens across surfaces.
    pub render_modes: Vec<String>,
    /// Whether projection normalized or stripped any source (always false).
    pub normalization_applied: bool,
    /// Per-surface resolved projections.
    pub surfaces: Vec<M5TrustDecisionIdentityProjection>,
    /// Trust-decision identity review block.
    pub review: M5TrustDecisionIdentityReview,
    /// Source contract refs consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TrustDecisionIdentityPacket {
    /// Returns true when every required surface is present exactly once.
    pub fn covers_all_surfaces(&self) -> bool {
        let present: BTreeSet<_> = self.surfaces.iter().map(|s| s.surface).collect();
        M5TrustDecisionSurface::ALL
            .iter()
            .all(|s| present.contains(s))
            && present.len() == M5TrustDecisionSurface::ALL.len()
    }

    /// Returns true when every surface renders strictly stronger than ordinary
    /// browsing.
    pub fn all_stronger_than_ordinary_browsing(&self) -> bool {
        self.surfaces
            .iter()
            .all(M5TrustDecisionIdentityProjection::stronger_than_ordinary_browsing)
    }

    /// Returns true when raw identity inspection stays reachable everywhere.
    pub fn raw_inspection_reachable_everywhere(&self) -> bool {
        self.surfaces.iter().all(|s| {
            s.raw_inspection_reachable
                && s.escaped_copy_reachable
                && s.has_open_raw_identity()
                && s.has_copy_escaped_identity()
        })
    }

    /// Returns true when the full identity is shown without truncation
    /// everywhere.
    pub fn full_identity_shown_everywhere(&self) -> bool {
        self.surfaces
            .iter()
            .all(|s| s.full_identity_shown && s.displayed_char_len == s.identity_char_len)
    }

    /// Returns true when every suspicious surface carries a warning and a
    /// codepoint-inspection affordance.
    pub fn suspicious_surfaces_warn_and_inspect(&self) -> bool {
        self.surfaces.iter().all(|s| {
            if !s.has_suspicious_cues {
                return true;
            }
            !s.warnings.is_empty() && !s.threat_classes.is_empty() && s.has_inspect_codepoints()
        })
    }

    /// Returns true when the stricter rendering is preserved across product,
    /// export, and handoff for every surface.
    pub fn preserved_everywhere(&self) -> bool {
        self.surfaces
            .iter()
            .all(M5TrustDecisionIdentityProjection::preserved_everywhere)
    }

    /// Validates the trust-decision identity invariants.
    pub fn validate(&self) -> Vec<M5TrustDecisionIdentityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TRUST_DECISION_IDENTITY_RECORD_KIND {
            violations.push(M5TrustDecisionIdentityViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TRUST_DECISION_IDENTITY_SCHEMA_VERSION {
            violations.push(M5TrustDecisionIdentityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.case_id.trim().is_empty()
            || self.minted_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            violations.push(M5TrustDecisionIdentityViolation::MissingIdentity);
        }
        if self.source_contract_refs.is_empty() {
            violations.push(M5TrustDecisionIdentityViolation::MissingSourceContracts);
        }
        if self.normalization_applied {
            violations.push(M5TrustDecisionIdentityViolation::NormalizationApplied);
        }
        if self.ordinary_browsing_strength_rank != M5_ORDINARY_BROWSING_STRENGTH_RANK
            || self.strong_decision_strength_rank != M5_STRONG_DECISION_STRENGTH_RANK
            || self.strong_decision_strength_rank <= self.ordinary_browsing_strength_rank
        {
            violations.push(M5TrustDecisionIdentityViolation::StrengthRankMismatch);
        }
        if !self.covers_all_surfaces() {
            violations.push(M5TrustDecisionIdentityViolation::SurfaceMissing);
        }
        if !self.all_stronger_than_ordinary_browsing() {
            violations.push(M5TrustDecisionIdentityViolation::NotStrongerThanBrowsing);
        }
        if !self.full_identity_shown_everywhere() {
            violations.push(M5TrustDecisionIdentityViolation::IdentityTruncated);
        }
        if !self.raw_inspection_reachable_everywhere() {
            violations.push(M5TrustDecisionIdentityViolation::RawInspectionUnreachable);
        }
        if !self.suspicious_surfaces_warn_and_inspect() {
            violations.push(M5TrustDecisionIdentityViolation::SuspiciousSurfaceMissingWarning);
        }
        if !self.preserved_everywhere() {
            violations.push(M5TrustDecisionIdentityViolation::RenderingNotPreserved);
        }
        if self.surface_count != self.surfaces.len() {
            violations.push(M5TrustDecisionIdentityViolation::SurfaceCountMismatch);
        }
        if self.suspicious_surface_count != self.declared_suspicious_count() {
            violations.push(M5TrustDecisionIdentityViolation::SuspiciousCountMismatch);
        }
        for surface in &self.surfaces {
            if surface.actions.is_empty() {
                violations.push(M5TrustDecisionIdentityViolation::ActionsMissing);
                break;
            }
        }
        for surface in &self.surfaces {
            // The escaped form may equal the rendered identity only when the
            // detector flagged nothing escapable; otherwise it must differ so
            // it cannot masquerade as the raw bytes.
            if surface.rendered_differs_from_raw && !surface.has_suspicious_cues {
                violations.push(M5TrustDecisionIdentityViolation::DivergenceWithoutCue);
                break;
            }
        }
        for surface in &self.surfaces {
            let mut malformed = surface
                .warnings
                .iter()
                .any(|w| w.kind_token != w.kind.as_str() || w.message.trim().is_empty());
            malformed |= surface
                .actions
                .iter()
                .any(|a| a.kind_token != a.kind.as_str() || a.label.trim().is_empty());
            if malformed {
                violations.push(M5TrustDecisionIdentityViolation::AffordanceMalformed);
                break;
            }
        }
        if !self.review.all_hold() {
            violations.push(M5TrustDecisionIdentityViolation::ReviewIncomplete);
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 trust-decision identity packet serializes"),
        ) {
            violations.push(M5TrustDecisionIdentityViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    fn declared_suspicious_count(&self) -> usize {
        self.surfaces
            .iter()
            .filter(|s| s.has_suspicious_cues)
            .count()
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 trust-decision identity packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Trust-Decision Identity Rendering\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Case: `{}`\n", self.case_id));
        out.push_str(&format!(
            "- Surfaces: {} · suspicious: {}\n",
            self.surface_count, self.suspicious_surface_count
        ));
        out.push_str(&format!(
            "- Strength: strong-decision {} > ordinary-browsing {}\n",
            self.strong_decision_strength_rank, self.ordinary_browsing_strength_rank
        ));
        out.push_str("\n## Surfaces\n\n");
        for surface in &self.surfaces {
            out.push_str(&format!(
                "- **{}** → decision `{}` ({}), render `{}`\n",
                surface.surface.as_str(),
                surface.decision_action.as_str(),
                surface.decision_verb,
                surface.render_mode.as_str(),
            ));
            out.push_str(&format!(
                "  - Identity: `{}` ({} chars) · suspicious: {} · raw ref: `{}`\n",
                surface.identity_inspection_escaped,
                surface.identity_char_len,
                surface.has_suspicious_cues,
                surface.raw_identity_ref,
            ));
            if !surface.threat_classes.is_empty() {
                out.push_str(&format!(
                    "  - Threat classes: {}\n",
                    surface.threat_classes.join(", ")
                ));
            }
            let actions = surface
                .actions
                .iter()
                .map(|a| a.kind.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("  - Inspection: {actions}\n"));
        }
        out
    }
}

/// Validation failures emitted by [`M5TrustDecisionIdentityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TrustDecisionIdentityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are missing.
    MissingSourceContracts,
    /// Projection normalized or stripped the source bytes.
    NormalizationApplied,
    /// The recorded strength ranks are inconsistent or not strictly ordered.
    StrengthRankMismatch,
    /// A required surface is missing.
    SurfaceMissing,
    /// A surface did not render strictly stronger than ordinary browsing.
    NotStrongerThanBrowsing,
    /// A surface truncated the identity instead of showing it in full.
    IdentityTruncated,
    /// Open-raw or escaped-copy is unreachable on some surface.
    RawInspectionUnreachable,
    /// A suspicious surface carried no warning or codepoint-inspection.
    SuspiciousSurfaceMissingWarning,
    /// A surface did not preserve the stricter rendering across carriers.
    RenderingNotPreserved,
    /// The declared surface count does not match the projections.
    SurfaceCountMismatch,
    /// The declared suspicious-surface count does not match the projections.
    SuspiciousCountMismatch,
    /// A surface offers no inspection affordances.
    ActionsMissing,
    /// A surface claims rendered/raw divergence without a detector cue.
    DivergenceWithoutCue,
    /// A warning or action is malformed (token mismatch or empty label).
    AffordanceMalformed,
    /// Review block does not satisfy required invariants.
    ReviewIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5TrustDecisionIdentityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NormalizationApplied => "normalization_applied",
            Self::StrengthRankMismatch => "strength_rank_mismatch",
            Self::SurfaceMissing => "surface_missing",
            Self::NotStrongerThanBrowsing => "not_stronger_than_browsing",
            Self::IdentityTruncated => "identity_truncated",
            Self::RawInspectionUnreachable => "raw_inspection_unreachable",
            Self::SuspiciousSurfaceMissingWarning => "suspicious_surface_missing_warning",
            Self::RenderingNotPreserved => "rendering_not_preserved",
            Self::SurfaceCountMismatch => "surface_count_mismatch",
            Self::SuspiciousCountMismatch => "suspicious_count_mismatch",
            Self::ActionsMissing => "actions_missing",
            Self::DivergenceWithoutCue => "divergence_without_cue",
            Self::AffordanceMalformed => "affordance_malformed",
            Self::ReviewIncomplete => "review_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Errors emitted when reading the checked-in trust-decision identity export.
#[derive(Debug)]
pub enum M5TrustDecisionIdentityExportError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TrustDecisionIdentityViolation>),
}

impl std::fmt::Display for M5TrustDecisionIdentityExportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 trust-decision identity export parse failed: {error}"
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
                    "m5 trust-decision identity export failed validation: {tokens}"
                )
            }
        }
    }
}

impl std::error::Error for M5TrustDecisionIdentityExportError {}

/// Resolves a single surface's strong-decision identity rendering by running
/// the shared suspicious-content detector over the identity text.
fn resolve_surface(input: &M5TrustDecisionIdentityInput<'_>) -> M5TrustDecisionIdentityProjection {
    let surface = input.surface;
    let decision_action = surface.decision_action();
    let render_mode = M5IdentityRenderMode::StrongDecision;

    let detection = detect_suspicious_content(input.identity_text);
    let escaped = escape_for_safe_inspection(input.identity_text);
    let char_len = input.identity_text.chars().count();

    let threat_classes = distinct_tokens(detection.findings.iter().map(|f| f.class.as_str()));
    let has_suspicious_cues = detection.has_findings();
    // The rendered glyphs differ materially from the raw bytes when the detector
    // flagged anything: invisible/bidi bytes that the escaped form makes visible,
    // or confusable glyphs that are not the letters they appear to be.
    let rendered_differs_from_raw = has_suspicious_cues;

    let mut warnings = Vec::new();
    if has_suspicious_cues {
        let has_bidi_or_invisible = detection
            .findings
            .iter()
            .any(|f| matches!(f.class.as_str(), "bidi_control" | "invisible_formatting"));
        let has_confusable = detection.findings.iter().any(|f| {
            matches!(
                f.class.as_str(),
                "mixed_script_confusable" | "whole_script_confusable"
            )
        });
        if has_bidi_or_invisible {
            warnings.push(warning(
                M5IdentityWarningKind::BidiOrInvisibleBytes,
                format!(
                    "This {} hides bidi or invisible codepoints; inspect the raw identity before you {}.",
                    surface.identity_kind(),
                    decision_action.verb().to_lowercase()
                ),
            ));
        }
        if has_confusable {
            warnings.push(warning(
                M5IdentityWarningKind::MixedScriptConfusable,
                format!(
                    "This {} mixes scripts so a glyph may not be the letter it appears to be; inspect the raw identity before you {}.",
                    surface.identity_kind(),
                    decision_action.verb().to_lowercase()
                ),
            ));
        }
    }

    let mut actions = vec![
        M5IdentityInspectionAction {
            kind: M5IdentityInspectionActionKind::OpenRawIdentity,
            kind_token: M5IdentityInspectionActionKind::OpenRawIdentity
                .as_str()
                .to_owned(),
            target_ref: input.raw_identity_ref.to_owned(),
            label: "Open raw identity".to_owned(),
        },
        M5IdentityInspectionAction {
            kind: M5IdentityInspectionActionKind::CopyEscapedIdentity,
            kind_token: M5IdentityInspectionActionKind::CopyEscapedIdentity
                .as_str()
                .to_owned(),
            target_ref: input.raw_identity_ref.to_owned(),
            label: "Copy escaped identity".to_owned(),
        },
    ];
    if has_suspicious_cues {
        actions.push(M5IdentityInspectionAction {
            kind: M5IdentityInspectionActionKind::InspectCodepoints,
            kind_token: M5IdentityInspectionActionKind::InspectCodepoints
                .as_str()
                .to_owned(),
            target_ref: input.raw_identity_ref.to_owned(),
            label: "Inspect codepoints".to_owned(),
        });
    }

    let rationale = build_rationale(surface, has_suspicious_cues, &threat_classes);

    M5TrustDecisionIdentityProjection {
        surface,
        surface_token: surface.as_str().to_owned(),
        decision_action,
        decision_action_token: decision_action.as_str().to_owned(),
        decision_verb: decision_action.verb().to_owned(),
        subject_ref: input.subject_ref.to_owned(),
        raw_identity_ref: input.raw_identity_ref.to_owned(),
        render_mode,
        render_mode_token: render_mode.as_str().to_owned(),
        render_strength_rank: render_mode.strength_rank(),
        identity_inspection_escaped: escaped,
        identity_char_len: char_len,
        displayed_char_len: char_len,
        full_identity_shown: true,
        has_suspicious_cues,
        suspicious_finding_count: detection.findings.len(),
        threat_classes,
        detector_outcome_token: detection.outcome.as_str().to_owned(),
        rendered_differs_from_raw,
        warnings,
        actions,
        raw_inspection_reachable: true,
        escaped_copy_reachable: true,
        decision_affordance_distinct: true,
        preserved_in_product: true,
        preserved_in_exported_review_packet: true,
        preserved_in_support_handoff: true,
        rationale,
    }
}

fn warning(kind: M5IdentityWarningKind, message: String) -> M5IdentityWarning {
    M5IdentityWarning {
        kind,
        kind_token: kind.as_str().to_owned(),
        message,
    }
}

fn build_rationale(
    surface: M5TrustDecisionSurface,
    has_suspicious_cues: bool,
    threat_classes: &[String],
) -> String {
    let decision = surface.decision_action();
    if has_suspicious_cues {
        format!(
            "{} is a trust decision ({}), so its {} renders in strong-decision mode; the detector flagged {} so a warning and raw inspection are shown before the user can {}.",
            surface.as_str(),
            decision.as_str(),
            surface.identity_kind(),
            threat_classes.join(", "),
            decision.verb().to_lowercase()
        )
    } else {
        format!(
            "{} is a trust decision ({}), so its {} renders in strong-decision mode with the full identity shown and raw inspection reachable, even though the detector flagged nothing.",
            surface.as_str(),
            decision.as_str(),
            surface.identity_kind()
        )
    }
}

fn distinct_tokens<'a>(tokens: impl Iterator<Item = &'a str>) -> Vec<String> {
    let set: BTreeSet<&str> = tokens.collect();
    set.into_iter().map(str::to_owned).collect()
}

/// Projects strong-decision identity rendering, suspicious-cue warnings, and
/// raw-identity inspection affordances across every new M5 trust-decision
/// surface.
pub fn project_m5_trust_decision_identity(
    seed: &M5TrustDecisionIdentitySeed<'_>,
) -> M5TrustDecisionIdentityPacket {
    let surfaces: Vec<_> = seed.surface_inputs.iter().map(resolve_surface).collect();

    let suspicious_surface_count = surfaces.iter().filter(|s| s.has_suspicious_cues).count();
    let render_modes = distinct_tokens(surfaces.iter().map(|s| s.render_mode.as_str()));

    M5TrustDecisionIdentityPacket {
        record_kind: M5_TRUST_DECISION_IDENTITY_RECORD_KIND.to_owned(),
        schema_version: M5_TRUST_DECISION_IDENTITY_SCHEMA_VERSION,
        packet_id: M5_TRUST_DECISION_IDENTITY_PACKET_ID.to_owned(),
        case_id: seed.case_id.to_owned(),
        surface_count: surfaces.len(),
        suspicious_surface_count,
        ordinary_browsing_strength_rank: M5_ORDINARY_BROWSING_STRENGTH_RANK,
        strong_decision_strength_rank: M5_STRONG_DECISION_STRENGTH_RANK,
        render_modes,
        normalization_applied: false,
        surfaces,
        review: M5TrustDecisionIdentityReview::frozen(),
        source_contract_refs: vec![
            M5_TRUST_DECISION_IDENTITY_SCHEMA_REF.to_owned(),
            M5_TRUST_DECISION_IDENTITY_DOC_REF.to_owned(),
            M5_TRUST_DECISION_IDENTITY_SUSPICIOUS_TEXT_CONTRACT_REF.to_owned(),
            M5_TRUST_DECISION_IDENTITY_TRUST_CLASS_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: seed.minted_at.to_owned(),
    }
}

/// Builds the canonical frozen trust-decision identity packet.
///
/// This is the single in-code source of truth for the checked-in support export
/// at [`M5_TRUST_DECISION_IDENTITY_ARTIFACT_REF`]; the bin emits this packet and
/// a test asserts the checked-in artifact deserializes back to it unchanged. The
/// scenario exercises clean identities, an invisible-byte host, a confusable
/// publisher, and policy-review text carrying a bidi control.
pub fn frozen_m5_trust_decision_identity_packet() -> M5TrustDecisionIdentityPacket {
    use M5TrustDecisionSurface as Surface;

    let surface_inputs = [
        // Publisher name with a Cyrillic confusable ('а' U+0430 in "Aureline").
        M5TrustDecisionIdentityInput {
            surface: Surface::PublisherPackageName,
            subject_ref: "marketplace:listing:formatter@3.1.0",
            raw_identity_ref: "marketplace:publisher:formatter@3.1.0:identity",
            identity_text: "Aurel\u{0456}ne Labs",
        },
        // Remote host with a zero-width space hiding inside the label.
        M5TrustDecisionIdentityInput {
            surface: Surface::RemoteHostLabel,
            subject_ref: "remote:attach:build-01",
            raw_identity_ref: "remote:host:build-01:identity",
            identity_text: "build\u{200B}-01.internal",
        },
        // Clean collaborator identity, no detector findings.
        M5TrustDecisionIdentityInput {
            surface: Surface::CollaboratorIdentity,
            subject_ref: "collab:invite:9f2",
            raw_identity_ref: "collab:invite:9f2:identity",
            identity_text: "Dana Okonomou",
        },
        // Clean route-share target.
        M5TrustDecisionIdentityInput {
            surface: Surface::RouteShare,
            subject_ref: "route:share:dashboard",
            raw_identity_ref: "route:share:dashboard:target",
            identity_text: "team-observability",
        },
        // Policy-review text carrying a right-to-left override (U+202E).
        M5TrustDecisionIdentityInput {
            surface: Surface::PolicyReview,
            subject_ref: "policy:review:egress-allow-list",
            raw_identity_ref: "policy:review:egress-allow-list:text",
            identity_text: "Allow egress to \u{202E}gpc.example\u{202C}",
        },
    ];

    project_m5_trust_decision_identity(&M5TrustDecisionIdentitySeed {
        case_id: "case:m5-trust-decision-identity:stable",
        surface_inputs,
        minted_at: "2026-06-10T00:00:00Z",
    })
}

/// Reads and validates the checked-in trust-decision identity support export.
pub fn current_m5_trust_decision_identity_export(
) -> Result<M5TrustDecisionIdentityPacket, M5TrustDecisionIdentityExportError> {
    let packet: M5TrustDecisionIdentityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/security/m5/m5_trust_decision_identity/support_export.json"
    )))
    .map_err(M5TrustDecisionIdentityExportError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TrustDecisionIdentityExportError::Validation(violations))
    }
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
