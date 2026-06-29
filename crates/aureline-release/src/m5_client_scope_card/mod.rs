//! Client-scope cards and deep-link / handoff disclosures that keep a narrowed client from
//! implying desktop parity.
//!
//! The [descriptor object](crate::m5_descriptor_object) lane freezes the typed client-scope
//! state a claimed M5 surface carries — its [client kind](crate::m5_descriptor_object::ClientScope),
//! its [authority class](crate::m5_descriptor_object::AuthorityClass), and its
//! [handoff requirement](crate::m5_descriptor_object::HandoffRequirement). The
//! [claim-narrowing](crate::m5_claim_narrowing) lane derives the one controlled claim state a
//! narrowed client implies, and the [descriptor-join](crate::m5_descriptor_join) lane proves that
//! truth survives copy/export. This lane is the *client-facing consumer*: it turns one client-scope
//! descriptor into the discovery, deep-link, handoff, and companion disclosures a surface actually
//! shows, so a browser companion, a headless client, or an unsupported surface states its scope and
//! authority *before* a user discovers a limit by failing into it — and can never imply capability
//! or authority parity it does not hold.
//!
//! A [`ClientScopeCard`] is built for one of the four [surface classes](SurfaceClass) — desktop,
//! browser companion, headless, and unsupported — and derives, never hand-authors, every visible
//! fact from the bound client-scope descriptor:
//!
//! - the [authority capabilities](AuthorityCapability) the scope *grants*, so the card can state
//!   what the client can do;
//! - the [blocked actions](BlockedAction) the desktop holds that this scope lacks, each with an
//!   attributable reason and the [handoff](crate::m5_descriptor_object::HandoffRequirement) that
//!   recovers it;
//! - the [parity caveats](ParityCaveat), one per weaker client-scope facet, so a narrowed client
//!   never reads at desktop parity by omission; and
//! - the controlled [claim state](crate::m5_claim_narrowing::NarrowedClaimState), derived from the
//!   same shared claim-narrowing runtime every other consumer reads.
//!
//! Each card projects onto every [disclosure surface](DisclosureSurface) — discovery, deep-link,
//! handoff, and companion — and each [`DisclosureProjection`] re-states the surface class, the
//! authority class, the handoff requirement, and the full blocked-action and parity-caveat counts.
//! The core guard is that **only** the desktop surface may carry full authority: a deep link or a
//! handoff summary can never claim broader authority than the current client holds, and a narrowed
//! card always carries at least one parity caveat and one blocked action, so companion / browser /
//! headless consumers cannot imply desktop parity by omission.
//!
//! The [`M5ClientScopeCardRegistry`] is the one inspectable, serde-serializable truth packet the
//! release center, Help/About, marketplace, docs/help, support exports, and companion handoffs read;
//! it carries metadata and refs only — no credential bodies or raw provider payloads.
//!
//! - Packet schema:
//!   [`schemas/provenance/m5-client-scope-card.schema.json`](../../../../../schemas/provenance/m5-client-scope-card.schema.json)
//! - Contract doc:
//!   [`docs/public-truth/m5-client-scope-card.md`](../../../../../docs/public-truth/m5-client-scope-card.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_browser_companion_card, seeded_browser_reference_card, seeded_desktop_full_card,
    seeded_headless_card, seeded_m5_client_scope_card_registry, seeded_unsupported_handoff_card,
    seeded_unsupported_not_provided_card, M5_CLIENT_SCOPE_CARD_REGISTRY_ID,
};

use serde::{Deserialize, Serialize};

use crate::m5_claim_narrowing::{ClaimNarrowingCase, NarrowedClaimState};
use crate::m5_descriptor_badge::{PublicTruthConsumer, M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX};
use crate::m5_descriptor_object::{
    ArtifactBinding, AuthorityClass, ClientScope, ClientScopeSubDescriptor, DescriptorFacet,
    DescriptorObject, DescriptorObjectInput, EvidenceState, FreshnessState, FreshnessSubDescriptor,
    HandoffRequirement, ProvenanceClass, ProvenanceSubDescriptor, QualificationClass,
    QualificationSubDescriptor, SignatureState,
};

/// Record-kind tag carried by a [`ClientScopeCard`].
pub const M5_CLIENT_SCOPE_CARD_RECORD_KIND: &str = "m5_client_scope_card";

/// Record-kind tag carried by [`M5ClientScopeCardRegistry`].
pub const M5_CLIENT_SCOPE_CARD_REGISTRY_RECORD_KIND: &str = "m5_client_scope_card_registry";

/// Schema version for the client-scope card and registry.
pub const M5_CLIENT_SCOPE_CARD_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the client-scope-card schema.
pub const M5_CLIENT_SCOPE_CARD_SCHEMA_REF: &str =
    "schemas/provenance/m5-client-scope-card.schema.json";

/// Repo-relative path of the published client-scope-card registry inventory.
pub const M5_CLIENT_SCOPE_CARD_REGISTRY_REF: &str =
    "artifacts/public-truth/m5-client-scope-card.json";

/// Repo-relative path of the release-grade client-scope-card parity proof.
pub const M5_CLIENT_SCOPE_CARD_PROOF_REF: &str =
    "artifacts/release/m5-descriptor-parity-proof/client-scope-card.json";

/// Repo-relative path of the client-scope-card contract doc.
pub const M5_CLIENT_SCOPE_CARD_DOC_REF: &str = "docs/public-truth/m5-client-scope-card.md";

/// Repo-relative directory of the client-scope-card consumer fixtures.
pub const M5_CLIENT_SCOPE_CARD_FIXTURE_DIR: &str = "fixtures/public-truth/m5-badge-consumers/";

/// One client-surface class a card is built for. The vocabulary covers the four surface classes
/// the source set names — the full desktop product, a browser / mobile companion, a headless /
/// automation client, and an unsupported surface — and only [`Desktop`](Self::Desktop) is permitted
/// to carry full authority. Declaration order is most→least capable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// The full desktop product surface — the only surface permitted full authority.
    Desktop,
    /// A browser or mobile companion surface with bounded, host-relayed scope.
    BrowserCompanion,
    /// A headless / automation client (CLI or scripted), never the desktop product.
    Headless,
    /// An unsupported surface class that cannot act in place at all.
    Unsupported,
}

impl SurfaceClass {
    /// Every surface class, in declaration order (most→least capable).
    pub const ALL: [Self; 4] = [
        Self::Desktop,
        Self::BrowserCompanion,
        Self::Headless,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::BrowserCompanion => "browser_companion",
            Self::Headless => "headless",
            Self::Unsupported => "unsupported",
        }
    }

    /// Reviewer-facing surface-class label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Desktop => "Desktop",
            Self::BrowserCompanion => "Browser companion",
            Self::Headless => "Headless client",
            Self::Unsupported => "Unsupported surface",
        }
    }

    /// True for the one surface class permitted to carry full authority / capability parity.
    pub const fn permits_full_authority(self) -> bool {
        matches!(self, Self::Desktop)
    }
}

/// One controlled authority capability on the desktop's full-authority ladder. The desktop grants
/// every capability; each narrower scope grants a strict prefix, and the difference is the set of
/// [blocked actions](BlockedAction) a card must disclose. Declaration order is least→most
/// privileged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityCapability {
    /// Observe / read state — the least privileged capability.
    Observe,
    /// Mutate state in place (edit and persist) without a handoff.
    MutateInPlace,
    /// Approve / merge / sign — review authority.
    Approve,
    /// Administer / configure — the most privileged capability.
    Administer,
}

impl AuthorityCapability {
    /// Every capability, in declaration order (least→most privileged).
    pub const ALL: [Self; 4] = [
        Self::Observe,
        Self::MutateInPlace,
        Self::Approve,
        Self::Administer,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::MutateInPlace => "mutate_in_place",
            Self::Approve => "approve",
            Self::Administer => "administer",
        }
    }

    /// Reviewer-facing capability label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Observe => "Observe",
            Self::MutateInPlace => "Edit in place",
            Self::Approve => "Approve / merge",
            Self::Administer => "Administer",
        }
    }
}

/// One disclosure surface a card is projected onto. Discovery is where a user first meets the
/// surface; a [`DeepLink`](Self::DeepLink) and a [`Handoff`](Self::Handoff) summary are the two
/// places a narrowed client is most tempted to imply broader authority than it holds; the
/// [`Companion`](Self::Companion) surface is the companion handoff card. Declaration order is the
/// canonical render order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisclosureSurface {
    /// The discovery surface — where a user first encounters the client surface.
    Discovery,
    /// A deep link into the surface (a routed entry point that must not imply broader authority).
    DeepLink,
    /// A handoff summary that originates or opens a more capable client.
    Handoff,
    /// The companion handoff card.
    Companion,
}

impl DisclosureSurface {
    /// Every disclosure surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Discovery,
        Self::DeepLink,
        Self::Handoff,
        Self::Companion,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Discovery => "discovery",
            Self::DeepLink => "deep_link",
            Self::Handoff => "handoff",
            Self::Companion => "companion",
        }
    }

    /// Reviewer-facing disclosure-surface label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Discovery => "Discovery",
            Self::DeepLink => "Deep link",
            Self::Handoff => "Handoff summary",
            Self::Companion => "Companion handoff",
        }
    }
}

/// One desktop capability this scope lacks, with an attributable reason and the handoff that
/// recovers it. Naming the capability and its recovery is what lets a card disclose a blocked action
/// *before* a user discovers it by failing, rather than after.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockedAction {
    /// The desktop capability this scope cannot perform in place.
    pub capability: AuthorityCapability,
    /// Reviewer-facing capability label.
    pub capability_label: String,
    /// Stable reason id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub reason_message_id: String,
    /// The handoff that recovers the capability on a more capable client.
    pub recovery: HandoffRequirement,
    /// Stable recovery id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub recovery_message_id: String,
}

/// One parity caveat: a controlled statement that a single client-scope facet narrows the surface
/// below desktop parity. Keying the caveat to the descriptor facet and its weaker token is what
/// keeps a narrowed client from reading at desktop parity by omission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParityCaveat {
    /// The client-scope facet whose weaker value drives the caveat.
    pub facet: DescriptorFacet,
    /// The weaker value token.
    pub token: String,
    /// Stable caveat id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub caveat_message_id: String,
}

/// One disclosure surface's projection of a card. Every projection re-states the surface class, the
/// authority class, the handoff requirement, and the full blocked-action and parity-caveat counts —
/// that equality is the proof a deep link or a handoff summary can never imply broader authority
/// than the card it is projected from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisclosureProjection {
    /// The disclosure surface.
    pub surface: DisclosureSurface,
    /// Reviewer-facing disclosure-surface label.
    pub surface_label: String,
    /// The surface class this disclosure publishes.
    pub surface_class: SurfaceClass,
    /// The authority class this disclosure publishes.
    pub authority_class: AuthorityClass,
    /// The handoff requirement this disclosure publishes.
    pub handoff_requirement: HandoffRequirement,
    /// True only when the card carries full authority (desktop).
    pub claims_full_authority: bool,
    /// The number of blocked actions this disclosure preserves.
    pub blocked_action_count: u32,
    /// The number of parity caveats this disclosure preserves.
    pub parity_caveat_count: u32,
    /// True when this disclosure preserves every blocked action.
    pub preserves_blocked_actions: bool,
    /// True when this disclosure preserves every parity caveat.
    pub preserves_parity_caveats: bool,
    /// Always true: this disclosure implies no broader authority than the card holds.
    pub implies_no_broader_authority: bool,
    /// True when this disclosure must surface that a handoff is required to act.
    pub requires_handoff_disclosure: bool,
    /// Stable message id; prefixed [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub disclosure_message_id: String,
}

/// The per-card disclosure review. Every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientScopeCardGuard {
    /// The card states its scope and authority class on every disclosure (before any failure).
    pub scope_and_authority_stated: bool,
    /// Every blocked action is attributable to a capability with a reason and a recovery.
    pub blocked_actions_attributable: bool,
    /// A narrowed card carries at least one parity caveat and at least one blocked action.
    pub parity_caveats_present_when_narrowed: bool,
    /// Every disclosure surface is projected.
    pub all_disclosures_projected: bool,
    /// No disclosure implies broader authority than the card holds.
    pub no_disclosure_implies_broader_authority: bool,
    /// Deep-link and handoff disclosures preserve the same scope, authority, and caveats.
    pub deep_link_and_handoff_preserve_truth: bool,
    /// A required handoff is disclosed on every disclosure surface.
    pub handoff_disclosed_when_required: bool,
    /// The claim state matches the shared claim-narrowing runtime.
    pub claim_state_matches_narrowing_runtime: bool,
    /// Only the desktop surface class carries full authority.
    pub only_desktop_carries_full_authority: bool,
}

impl ClientScopeCardGuard {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.scope_and_authority_stated
            && self.blocked_actions_attributable
            && self.parity_caveats_present_when_narrowed
            && self.all_disclosures_projected
            && self.no_disclosure_implies_broader_authority
            && self.deep_link_and_handoff_preserve_truth
            && self.handoff_disclosed_when_required
            && self.claim_state_matches_narrowing_runtime
            && self.only_desktop_carries_full_authority
    }
}

/// One client-scope card: a single surface class projected from one client-scope descriptor into
/// the discovery, deep-link, handoff, and companion disclosures it shows. It re-states the client
/// kind, authority class, and handoff requirement, derives the granted capabilities, blocked
/// actions, parity caveats, and claim state from that descriptor, and projects them onto every
/// disclosure surface so a narrowed client can never imply desktop parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientScopeCard {
    /// Record kind; must equal [`M5_CLIENT_SCOPE_CARD_RECORD_KIND`].
    pub record_kind: String,
    /// Stable card id.
    pub card_id: String,
    /// Reviewer-facing card label.
    pub card_label: String,
    /// The surface class this card is built for.
    pub surface_class: SurfaceClass,
    /// Reviewer-facing surface-class label.
    pub surface_class_label: String,
    /// The client-scope descriptor this card reads.
    pub client_scope: ClientScopeSubDescriptor,
    /// The client kind, re-stated from the descriptor.
    pub client_kind: ClientScope,
    /// The authority class, re-stated from the descriptor.
    pub authority_class: AuthorityClass,
    /// The handoff requirement, re-stated from the descriptor.
    pub handoff_requirement: HandoffRequirement,
    /// True only when this is the desktop surface at full authority — capability parity.
    pub claims_full_authority: bool,
    /// The authority capabilities this scope grants, in capability order.
    pub granted_capabilities: Vec<AuthorityCapability>,
    /// The desktop capabilities this scope lacks, each with reason and recovery.
    pub blocked_actions: Vec<BlockedAction>,
    /// The parity caveats, one per weaker client-scope facet, in facet order.
    pub parity_caveats: Vec<ParityCaveat>,
    /// The controlled claim state, derived from the shared claim-narrowing runtime.
    pub claim_state: NarrowedClaimState,
    /// Per-disclosure projections, in [`DisclosureSurface::ALL`] order.
    pub disclosures: Vec<DisclosureProjection>,
    /// A deterministic, copy-safe one-line summary of the card.
    pub copy_safe_summary: String,
    /// The per-card disclosure review.
    pub guard: ClientScopeCardGuard,
    /// Stable message id for the scope statement; prefixed
    /// [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub scope_message_id: String,
    /// Stable message id for the authority statement; prefixed
    /// [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub authority_message_id: String,
    /// Stable message id for the card explanation drawer; prefixed
    /// [`M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX`].
    pub explanation_drawer_message_id: String,
}

impl ClientScopeCard {
    /// Builds a card from a surface class and a client-scope descriptor, deriving the granted
    /// capabilities, blocked actions, parity caveats, claim state, disclosures, and review from the
    /// descriptor's own state so the disclosures are always generated from one source rather than
    /// hand-authored.
    pub fn new(
        card_id: &str,
        card_label: &str,
        surface_class: SurfaceClass,
        client_scope: ClientScopeSubDescriptor,
    ) -> Self {
        let claims_full_authority = derive_claims_full_authority(surface_class, &client_scope);
        let granted_capabilities = derive_granted_capabilities(client_scope.authority_class);
        let blocked_actions = derive_blocked_actions(&client_scope);
        let parity_caveats = derive_parity_caveats(&client_scope);
        let claim_state = derive_claim_state(&client_scope);
        let disclosures = derive_disclosures(
            surface_class,
            &client_scope,
            claims_full_authority,
            blocked_actions.len(),
            parity_caveats.len(),
        );
        let copy_safe_summary = derive_copy_safe_summary(
            card_id,
            surface_class,
            &client_scope,
            claim_state,
            blocked_actions.len(),
            parity_caveats.len(),
        );
        let guard = derive_card_guard(
            surface_class,
            &client_scope,
            claims_full_authority,
            &blocked_actions,
            &parity_caveats,
            claim_state,
            &disclosures,
        );
        Self {
            record_kind: M5_CLIENT_SCOPE_CARD_RECORD_KIND.to_owned(),
            card_id: card_id.to_owned(),
            card_label: card_label.to_owned(),
            surface_class,
            surface_class_label: surface_class.label().to_owned(),
            client_kind: client_scope.client_kind,
            authority_class: client_scope.authority_class,
            handoff_requirement: client_scope.handoff_requirement,
            claims_full_authority,
            granted_capabilities,
            blocked_actions,
            parity_caveats,
            claim_state,
            disclosures,
            copy_safe_summary,
            guard,
            scope_message_id: format!("{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}client_scope.scope"),
            authority_message_id: format!(
                "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}client_scope.authority"
            ),
            explanation_drawer_message_id: format!(
                "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}client_scope.drawer"
            ),
            client_scope,
        }
    }

    /// True when the card carries full desktop authority.
    pub fn is_full_authority(&self) -> bool {
        self.claims_full_authority
    }

    /// True when the card is narrowed below desktop parity.
    pub fn is_narrowed(&self) -> bool {
        !self.claims_full_authority
    }

    /// Deterministic export-safe JSON for the card.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only card fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("client-scope card serializes")
    }

    /// Validates the card's invariants: the authority / handoff / capabilities / blocked actions /
    /// parity caveats / claim state / disclosures all agree with the bound descriptor, only the
    /// desktop surface carries full authority, no disclosure implies broader authority, a narrowed
    /// card always carries a caveat and a blocked action, message ids carry the lane prefix, and the
    /// export carries no raw material.
    pub fn validate(&self) -> Vec<M5ClientScopeCardViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_CLIENT_SCOPE_CARD_RECORD_KIND {
            out.push(M5ClientScopeCardViolation::WrongRecordKind);
        }
        if self.card_id.trim().is_empty()
            || self.card_label.trim().is_empty()
            || self.copy_safe_summary.trim().is_empty()
        {
            out.push(M5ClientScopeCardViolation::MissingIdentity);
        }

        // The re-stated client-scope fields must mirror the bound descriptor.
        if self.client_kind != self.client_scope.client_kind
            || self.authority_class != self.client_scope.authority_class
            || self.handoff_requirement != self.client_scope.handoff_requirement
        {
            out.push(M5ClientScopeCardViolation::DescriptorMismatch);
        }
        if self.surface_class_label != self.surface_class.label() {
            out.push(M5ClientScopeCardViolation::DescriptorMismatch);
        }

        // The derived facts must agree with a fresh derivation from the descriptor.
        if self.claims_full_authority
            != derive_claims_full_authority(self.surface_class, &self.client_scope)
        {
            out.push(M5ClientScopeCardViolation::AuthorityParityDrift);
        }
        if self.granted_capabilities
            != derive_granted_capabilities(self.client_scope.authority_class)
        {
            out.push(M5ClientScopeCardViolation::CapabilityDrift);
        }
        if self.blocked_actions != derive_blocked_actions(&self.client_scope) {
            out.push(M5ClientScopeCardViolation::BlockedActionDrift);
        }
        if self.parity_caveats != derive_parity_caveats(&self.client_scope) {
            out.push(M5ClientScopeCardViolation::ParityCaveatDrift);
        }
        if self.claim_state != derive_claim_state(&self.client_scope) {
            out.push(M5ClientScopeCardViolation::ClaimStateDrift);
        }
        let expected_disclosures = derive_disclosures(
            self.surface_class,
            &self.client_scope,
            self.claims_full_authority,
            self.blocked_actions.len(),
            self.parity_caveats.len(),
        );
        if self.disclosures != expected_disclosures {
            out.push(M5ClientScopeCardViolation::DisclosureDrift);
        }
        if self.copy_safe_summary
            != derive_copy_safe_summary(
                &self.card_id,
                self.surface_class,
                &self.client_scope,
                self.claim_state,
                self.blocked_actions.len(),
                self.parity_caveats.len(),
            )
        {
            out.push(M5ClientScopeCardViolation::CopySafeSummaryDrift);
        }

        // The core authority-parity guard: only the desktop surface may carry full authority.
        let authority_is_full = self.authority_class.is_full_authority();
        let kind_is_full = self.client_kind.is_full_authority();
        if self.surface_class.permits_full_authority() {
            // A desktop card must be a genuine desktop-full scope with no required handoff.
            if !authority_is_full || !kind_is_full || !self.handoff_requirement.is_in_product() {
                out.push(M5ClientScopeCardViolation::DesktopScopeNotFull);
            }
        } else if authority_is_full || kind_is_full || self.claims_full_authority {
            // A non-desktop card must never carry full authority or claim parity.
            out.push(M5ClientScopeCardViolation::NarrowedClaimsFullAuthority);
        }

        // A narrowed card must visibly carry at least one parity caveat and one blocked action.
        if self.is_narrowed() && (self.parity_caveats.is_empty() || self.blocked_actions.is_empty())
        {
            out.push(M5ClientScopeCardViolation::NarrowedHidesLimits);
        }
        // A full-authority card must carry no caveat or blocked action.
        if self.claims_full_authority
            && (!self.parity_caveats.is_empty() || !self.blocked_actions.is_empty())
        {
            out.push(M5ClientScopeCardViolation::FullAuthorityHasLimits);
        }

        // Every disclosure is projected, in canonical order, and preserves the card's truth.
        let projected: Vec<DisclosureSurface> =
            self.disclosures.iter().map(|d| d.surface).collect();
        if projected != DisclosureSurface::ALL.to_vec() {
            out.push(M5ClientScopeCardViolation::DisclosureSetMismatch);
        }
        for disclosure in &self.disclosures {
            if disclosure.surface_class != self.surface_class
                || disclosure.authority_class != self.authority_class
                || disclosure.handoff_requirement != self.handoff_requirement
                || disclosure.claims_full_authority != self.claims_full_authority
                || disclosure.blocked_action_count != self.blocked_actions.len() as u32
                || disclosure.parity_caveat_count != self.parity_caveats.len() as u32
            {
                out.push(M5ClientScopeCardViolation::DisclosureDiverged);
            }
            if !disclosure.preserves_blocked_actions
                || !disclosure.preserves_parity_caveats
                || !disclosure.implies_no_broader_authority
            {
                out.push(M5ClientScopeCardViolation::DisclosureImpliesBroaderAuthority);
            }
            // The core guard: a narrowed card may never read as full authority on a disclosure.
            if self.is_narrowed() && disclosure.claims_full_authority {
                out.push(M5ClientScopeCardViolation::DisclosureImpliesBroaderAuthority);
            }
            // A required handoff must be disclosed on every surface.
            if !self.handoff_requirement.is_in_product() && !disclosure.requires_handoff_disclosure
            {
                out.push(M5ClientScopeCardViolation::HandoffNotDisclosed);
            }
        }

        if !self.guard.all_hold()
            || self.guard
                != derive_card_guard(
                    self.surface_class,
                    &self.client_scope,
                    self.claims_full_authority,
                    &self.blocked_actions,
                    &self.parity_caveats,
                    self.claim_state,
                    &self.disclosures,
                )
        {
            out.push(M5ClientScopeCardViolation::GuardReviewFailed);
        }

        if !message_ids_prefixed(self) {
            out.push(M5ClientScopeCardViolation::UnprefixedMessageId);
        }
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("client-scope card serializes"),
        ) {
            out.push(M5ClientScopeCardViolation::RawMaterialInExport);
        }
        out
    }
}

/// Derives whether a card carries full authority: only a desktop surface bound to a genuine
/// desktop-full scope with no required handoff.
fn derive_claims_full_authority(
    surface_class: SurfaceClass,
    client_scope: &ClientScopeSubDescriptor,
) -> bool {
    surface_class.permits_full_authority()
        && client_scope.client_kind.is_full_authority()
        && client_scope.authority_class.is_full_authority()
        && client_scope.handoff_requirement.is_in_product()
}

/// Derives the capabilities an authority class grants — a strict prefix of the desktop's full
/// ladder, so the difference is exactly the blocked-action set.
fn derive_granted_capabilities(authority: AuthorityClass) -> Vec<AuthorityCapability> {
    let granted: &[AuthorityCapability] = match authority {
        AuthorityClass::FullAuthority => &[
            AuthorityCapability::Observe,
            AuthorityCapability::MutateInPlace,
            AuthorityCapability::Approve,
            AuthorityCapability::Administer,
        ],
        AuthorityClass::ScopedAuthority => &[
            AuthorityCapability::Observe,
            AuthorityCapability::MutateInPlace,
        ],
        AuthorityClass::ReferenceOnly => &[AuthorityCapability::Observe],
        AuthorityClass::HandoffOnly | AuthorityClass::NotProvided => &[],
    };
    granted.to_vec()
}

/// The handoff that recovers a blocked capability: a console pivot keeps its console handoff;
/// every other scope recovers on the desktop.
fn recovery_handoff(handoff: HandoffRequirement) -> HandoffRequirement {
    match handoff {
        HandoffRequirement::ConsoleHandoffRequired => HandoffRequirement::ConsoleHandoffRequired,
        _ => HandoffRequirement::DesktopHandoffRequired,
    }
}

/// Derives the blocked actions: every desktop capability this scope does not grant, in capability
/// order, each bound to the handoff that recovers it.
fn derive_blocked_actions(client_scope: &ClientScopeSubDescriptor) -> Vec<BlockedAction> {
    let granted = derive_granted_capabilities(client_scope.authority_class);
    let recovery = recovery_handoff(client_scope.handoff_requirement);
    AuthorityCapability::ALL
        .iter()
        .filter(|cap| !granted.contains(cap))
        .map(|&capability| BlockedAction {
            capability,
            capability_label: capability.label().to_owned(),
            reason_message_id: format!(
                "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}client_scope.blocked.{}",
                capability.as_str()
            ),
            recovery,
            recovery_message_id: format!(
                "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}client_scope.recovery.{}",
                recovery.as_str()
            ),
        })
        .collect()
}

/// Builds a parity caveat for one client-scope facet whose value narrows the surface.
fn parity_caveat_for(facet: DescriptorFacet, token: &str) -> ParityCaveat {
    ParityCaveat {
        facet,
        token: token.to_owned(),
        caveat_message_id: format!(
            "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}client_scope.caveat.{}.{token}",
            facet.as_str()
        ),
    }
}

/// Derives the parity caveats: one per weaker client-scope facet — a narrowed client kind, a
/// narrowed authority, or a required handoff — in facet order, so a narrowed client never reads at
/// desktop parity by omission.
fn derive_parity_caveats(client_scope: &ClientScopeSubDescriptor) -> Vec<ParityCaveat> {
    let mut out = Vec::new();
    if !client_scope.client_kind.is_full_authority() {
        out.push(parity_caveat_for(
            DescriptorFacet::ClientKind,
            client_scope.client_kind.as_str(),
        ));
    }
    if !client_scope.authority_class.is_full_authority() {
        out.push(parity_caveat_for(
            DescriptorFacet::AuthorityClass,
            client_scope.authority_class.as_str(),
        ));
    }
    if !client_scope.handoff_requirement.is_in_product() {
        out.push(parity_caveat_for(
            DescriptorFacet::HandoffRequirement,
            client_scope.handoff_requirement.as_str(),
        ));
    }
    out
}

/// Derives the controlled claim state from the shared claim-narrowing runtime, reading a clean
/// baseline descriptor whose only narrowing is this client scope — so a client-scope card can never
/// derive a different state than the interactive consumers do.
fn derive_claim_state(client_scope: &ClientScopeSubDescriptor) -> NarrowedClaimState {
    let descriptor = DescriptorObject::new(DescriptorObjectInput {
        descriptor_id: "client-scope-card:claim-derivation".to_owned(),
        descriptor_label: "Client-scope card claim derivation".to_owned(),
        artifact_ref: ArtifactBinding {
            artifact_id: "client-scope-card-derivation".to_owned(),
            artifact_family: "client_scope_card".to_owned(),
            artifact_kind: "claim_derivation".to_owned(),
            schema_ref: M5_CLIENT_SCOPE_CARD_SCHEMA_REF.to_owned(),
            content_digest_ref: "digest-ref:client-scope-card-derivation".to_owned(),
        },
        provenance: ProvenanceSubDescriptor {
            source_class: ProvenanceClass::FirstPartySigned,
            signature_state: SignatureState::SignedAttested,
        },
        freshness: FreshnessSubDescriptor {
            freshness_state: FreshnessState::Current,
            evidence_state: EvidenceState::Complete,
        },
        qualification: QualificationSubDescriptor {
            support_class: QualificationClass::Stable,
            evidence_state: EvidenceState::Complete,
        },
        client_scope: *client_scope,
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "1970-01-01T00:00:00Z".to_owned(),
    });
    ClaimNarrowingCase::from_descriptor(
        "client-scope-card:derivation",
        "Client-scope card derivation",
        descriptor,
    )
    .canonical_claim_state
}

/// Derives the per-disclosure projections, one per [`DisclosureSurface`], each preserving the
/// surface class, authority, handoff, blocked-action count, and parity-caveat count, and each
/// stating it implies no broader authority than the card holds.
fn derive_disclosures(
    surface_class: SurfaceClass,
    client_scope: &ClientScopeSubDescriptor,
    claims_full_authority: bool,
    blocked_action_count: usize,
    parity_caveat_count: usize,
) -> Vec<DisclosureProjection> {
    let requires_handoff_disclosure = !client_scope.handoff_requirement.is_in_product();
    DisclosureSurface::ALL
        .iter()
        .map(|&surface| DisclosureProjection {
            surface,
            surface_label: surface.label().to_owned(),
            surface_class,
            authority_class: client_scope.authority_class,
            handoff_requirement: client_scope.handoff_requirement,
            claims_full_authority,
            blocked_action_count: blocked_action_count as u32,
            parity_caveat_count: parity_caveat_count as u32,
            preserves_blocked_actions: true,
            preserves_parity_caveats: true,
            implies_no_broader_authority: true,
            requires_handoff_disclosure,
            disclosure_message_id: format!(
                "{M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX}client_scope.disclosure.{}.render",
                surface.as_str()
            ),
        })
        .collect()
}

/// Derives the deterministic, copy-safe one-line summary for a client-scope card.
fn derive_copy_safe_summary(
    card_id: &str,
    surface_class: SurfaceClass,
    client_scope: &ClientScopeSubDescriptor,
    claim_state: NarrowedClaimState,
    blocked_action_count: usize,
    parity_caveat_count: usize,
) -> String {
    format!(
        "{} · surface {} · client {} · authority {} · handoff {} · claim {} · {} blocked · {} caveat(s)",
        card_id,
        surface_class.as_str(),
        client_scope.client_kind.as_str(),
        client_scope.authority_class.as_str(),
        client_scope.handoff_requirement.as_str(),
        claim_state.as_str(),
        blocked_action_count,
        parity_caveat_count
    )
}

/// Derives the per-card disclosure review from its derived state.
fn derive_card_guard(
    surface_class: SurfaceClass,
    client_scope: &ClientScopeSubDescriptor,
    claims_full_authority: bool,
    blocked_actions: &[BlockedAction],
    parity_caveats: &[ParityCaveat],
    claim_state: NarrowedClaimState,
    disclosures: &[DisclosureProjection],
) -> ClientScopeCardGuard {
    let narrowed = !claims_full_authority;
    let requires_handoff = !client_scope.handoff_requirement.is_in_product();

    let scope_and_authority_stated = disclosures.iter().all(|d| {
        d.surface_class == surface_class
            && d.authority_class == client_scope.authority_class
            && d.handoff_requirement == client_scope.handoff_requirement
    });

    let blocked_actions_attributable = blocked_actions.iter().all(|a| {
        !a.reason_message_id.trim().is_empty() && !a.recovery_message_id.trim().is_empty()
    });

    let parity_caveats_present_when_narrowed =
        !narrowed || (!parity_caveats.is_empty() && !blocked_actions.is_empty());

    let all_disclosures_projected = disclosures.iter().map(|d| d.surface).collect::<Vec<_>>()
        == DisclosureSurface::ALL.to_vec();

    let no_disclosure_implies_broader_authority = disclosures.iter().all(|d| {
        d.implies_no_broader_authority
            && d.claims_full_authority == claims_full_authority
            && (!narrowed || !d.claims_full_authority)
    });

    // The deep-link and handoff disclosures are the two most likely to over-claim; both must carry
    // the card's exact authority, handoff, and full caveat / blocked counts.
    let deep_link_and_handoff_preserve_truth = disclosures
        .iter()
        .filter(|d| {
            matches!(
                d.surface,
                DisclosureSurface::DeepLink | DisclosureSurface::Handoff
            )
        })
        .all(|d| {
            d.authority_class == client_scope.authority_class
                && d.handoff_requirement == client_scope.handoff_requirement
                && d.claims_full_authority == claims_full_authority
                && d.blocked_action_count == blocked_actions.len() as u32
                && d.parity_caveat_count == parity_caveats.len() as u32
                && d.preserves_blocked_actions
                && d.preserves_parity_caveats
        });

    let handoff_disclosed_when_required =
        !requires_handoff || disclosures.iter().all(|d| d.requires_handoff_disclosure);

    let claim_state_matches_narrowing_runtime = claim_state == derive_claim_state(client_scope);

    let only_desktop_carries_full_authority = if surface_class.permits_full_authority() {
        true
    } else {
        !client_scope.authority_class.is_full_authority()
            && !client_scope.client_kind.is_full_authority()
            && !claims_full_authority
    };

    ClientScopeCardGuard {
        scope_and_authority_stated,
        blocked_actions_attributable,
        parity_caveats_present_when_narrowed,
        all_disclosures_projected,
        no_disclosure_implies_broader_authority,
        deep_link_and_handoff_preserve_truth,
        handoff_disclosed_when_required,
        claim_state_matches_narrowing_runtime,
        only_desktop_carries_full_authority,
    }
}

/// True when every message id the card carries is prefixed with the lane prefix.
fn message_ids_prefixed(card: &ClientScopeCard) -> bool {
    let prefixed = |s: &str| s.starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX);
    prefixed(&card.scope_message_id)
        && prefixed(&card.authority_message_id)
        && prefixed(&card.explanation_drawer_message_id)
        && card
            .blocked_actions
            .iter()
            .all(|a| prefixed(&a.reason_message_id) && prefixed(&a.recovery_message_id))
        && card
            .parity_caveats
            .iter()
            .all(|c| prefixed(&c.caveat_message_id))
        && card
            .disclosures
            .iter()
            .all(|d| prefixed(&d.disclosure_message_id))
}

/// Self-describing controlled-vocabulary set so the registry resolves every token a card can carry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientScopeCardVocabulary {
    /// Surface-class tokens.
    pub surface_classes: Vec<String>,
    /// Client-kind tokens.
    pub client_kinds: Vec<String>,
    /// Authority-class tokens.
    pub authority_classes: Vec<String>,
    /// Handoff-requirement tokens.
    pub handoff_requirements: Vec<String>,
    /// Authority-capability tokens.
    pub capabilities: Vec<String>,
    /// Disclosure-surface tokens.
    pub disclosure_surfaces: Vec<String>,
    /// Client-scope facet tokens (the facets a parity caveat can key off).
    pub facets: Vec<String>,
    /// Claim-state tokens.
    pub claim_states: Vec<String>,
    /// Consumer tokens.
    pub consumers: Vec<String>,
}

/// The three descriptor facets a client-scope card reasons over, in facet order.
const CLIENT_SCOPE_FACETS: [DescriptorFacet; 3] = [
    DescriptorFacet::ClientKind,
    DescriptorFacet::AuthorityClass,
    DescriptorFacet::HandoffRequirement,
];

impl ClientScopeCardVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_classes: SurfaceClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            client_kinds: ClientScope::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            authority_classes: AuthorityClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            handoff_requirements: HandoffRequirement::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            capabilities: AuthorityCapability::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            disclosure_surfaces: DisclosureSurface::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            facets: CLIENT_SCOPE_FACETS
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            claim_states: NarrowedClaimState::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            consumers: PublicTruthConsumer::ALL
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

/// Conformance review for the client-scope-card registry. Every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientScopeCardConformance {
    /// Every card is self-consistent.
    pub cards_validate: bool,
    /// The four surface classes are all covered by at least one card.
    pub all_surface_classes_covered: bool,
    /// Every card's authority and handoff are read from the bound descriptor.
    pub authority_derived_from_descriptor: bool,
    /// Every blocked action is attributable to a capability, reason, and recovery.
    pub blocked_actions_attributable: bool,
    /// A narrowed card always carries at least one parity caveat and one blocked action.
    pub parity_caveats_present_when_narrowed: bool,
    /// Every card states its scope and authority on the discovery surface — before any failure.
    pub scope_stated_before_failure: bool,
    /// Every disclosure surface is projected for every card.
    pub all_disclosures_projected: bool,
    /// No deep-link or handoff disclosure implies broader authority than its card.
    pub deep_link_and_handoff_preserve_truth: bool,
    /// A narrowed companion / browser / headless card can never imply desktop parity.
    pub narrowed_never_implies_desktop_parity: bool,
    /// The claim state matches the shared claim-narrowing runtime for every card.
    pub claim_state_matches_narrowing_runtime: bool,
    /// Only the desktop surface class carries full authority.
    pub only_desktop_carries_full_authority: bool,
    /// The controlled vocabularies match the canonical frozen tokens.
    pub controlled_enums_frozen: bool,
    /// Every public-truth consumer reads this one card runtime.
    pub shared_across_consumers: bool,
    /// The export carries no raw provider material.
    pub export_carries_no_raw_material: bool,
}

impl ClientScopeCardConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.cards_validate
            && self.all_surface_classes_covered
            && self.authority_derived_from_descriptor
            && self.blocked_actions_attributable
            && self.parity_caveats_present_when_narrowed
            && self.scope_stated_before_failure
            && self.all_disclosures_projected
            && self.deep_link_and_handoff_preserve_truth
            && self.narrowed_never_implies_desktop_parity
            && self.claim_state_matches_narrowing_runtime
            && self.only_desktop_carries_full_authority
            && self.controlled_enums_frozen
            && self.shared_across_consumers
            && self.export_carries_no_raw_material
    }
}

/// Roll-up counts over the registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientScopeCardSummary {
    /// Total cards.
    pub total_cards: u32,
    /// Cards that carry full desktop authority.
    pub full_authority_cards: u32,
    /// Cards narrowed below desktop parity.
    pub narrowed_cards: u32,
    /// Total disclosure projections across every card.
    pub total_disclosure_projections: u32,
    /// Total blocked actions across every card.
    pub total_blocked_actions: u32,
    /// Total parity caveats across every card.
    pub total_parity_caveats: u32,
}

/// Constructor input for [`M5ClientScopeCardRegistry::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ClientScopeCardRegistryInput {
    /// Stable registry id.
    pub registry_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The client-scope cards this registry publishes.
    pub cards: Vec<ClientScopeCard>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// The one inspectable, serde-serializable client-scope-card truth packet every public-truth
/// consumer reads: the cards, the controlled vocabulary they share, the consumers that read the
/// runtime, a conformance review, and a roll-up summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ClientScopeCardRegistry {
    /// Record kind; must equal [`M5_CLIENT_SCOPE_CARD_REGISTRY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CLIENT_SCOPE_CARD_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable registry id.
    pub registry_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The client-scope cards this registry publishes.
    pub cards: Vec<ClientScopeCard>,
    /// The controlled vocabulary every card shares.
    pub vocabulary: ClientScopeCardVocabulary,
    /// The public-truth consumers that read this card runtime.
    pub consumers: Vec<String>,
    /// Conformance review block.
    pub conformance: ClientScopeCardConformance,
    /// Roll-up counts.
    pub summary: ClientScopeCardSummary,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ClientScopeCardRegistry {
    /// Builds a registry from its cards, deriving the vocabulary, consumer list, conformance review,
    /// and summary.
    pub fn new(input: M5ClientScopeCardRegistryInput) -> Self {
        let cards = input.cards;
        let consumers: Vec<String> = PublicTruthConsumer::ALL
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect();
        let conformance = derive_registry_conformance(&cards);
        let summary = derive_summary(&cards);
        Self {
            record_kind: M5_CLIENT_SCOPE_CARD_REGISTRY_RECORD_KIND.to_owned(),
            schema_version: M5_CLIENT_SCOPE_CARD_SCHEMA_VERSION,
            registry_id: input.registry_id,
            report_label: input.report_label,
            cards,
            vocabulary: ClientScopeCardVocabulary::canonical(),
            consumers,
            conformance,
            summary,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Finds a card by id.
    pub fn card(&self, card_id: &str) -> Option<&ClientScopeCard> {
        self.cards.iter().find(|c| c.card_id == card_id)
    }

    /// Deterministic export-safe JSON for the registry.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("client-scope card registry serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 client-scope card parity\n\n");
        out.push_str(&format!("- Registry: `{}`\n", self.registry_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Cards: {}\n", self.cards.len()));
        out.push_str(&format!("- Minted: `{}`\n", self.minted_at));
        out.push_str("- Surface classes: desktop, browser companion, headless, unsupported\n");
        out.push_str("- Disclosures: discovery, deep link, handoff, companion\n");
        out.push_str(
            "- Consumed by: release center, Help/About, marketplace, docs/help, certification, evaluation packs, support, companion\n",
        );

        out.push_str("\n## Cards\n\n");
        out.push_str(
            "| Card | Surface | Client | Authority | Handoff | Claim | Blocked | Caveats |\n",
        );
        out.push_str(
            "|------|---------|--------|-----------|---------|-------|---------|--------|\n",
        );
        for card in &self.cards {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} | {} |\n",
                card.card_id,
                card.surface_class.as_str(),
                card.client_kind.as_str(),
                card.authority_class.as_str(),
                card.handoff_requirement.as_str(),
                card.claim_state.as_str(),
                card.blocked_actions.len(),
                card.parity_caveats.len()
            ));
        }

        out.push_str("\n## Disclosure parity\n\n");
        for card in &self.cards {
            out.push_str(&format!(
                "### `{}` → surface `{}` ({})\n\n",
                card.card_id,
                card.surface_class.as_str(),
                if card.claims_full_authority {
                    "full authority"
                } else {
                    "narrowed"
                }
            ));
            out.push_str(&format!(
                "Copy-safe summary: `{}`\n\n",
                card.copy_safe_summary
            ));
            if card.blocked_actions.is_empty() {
                out.push_str("_No blocked actions — full desktop parity._\n\n");
            } else {
                out.push_str("**Blocked actions (recover via handoff):**\n\n");
                for action in &card.blocked_actions {
                    out.push_str(&format!(
                        "- `{}` → recover via `{}`\n",
                        action.capability.as_str(),
                        action.recovery.as_str()
                    ));
                }
                out.push('\n');
            }
            if !card.parity_caveats.is_empty() {
                out.push_str("**Parity caveats:**\n\n");
                for caveat in &card.parity_caveats {
                    out.push_str(&format!(
                        "- `{}` (`{}`)\n",
                        caveat.facet.as_str(),
                        caveat.token
                    ));
                }
                out.push('\n');
            }
            out.push_str("| Disclosure | Authority | Handoff disclosed | No broader authority |\n");
            out.push_str("|------------|-----------|-------------------|----------------------|\n");
            for disclosure in &card.disclosures {
                out.push_str(&format!(
                    "| `{}` | `{}` | {} | {} |\n",
                    disclosure.surface.as_str(),
                    disclosure.authority_class.as_str(),
                    if disclosure.requires_handoff_disclosure {
                        "yes"
                    } else {
                        "n/a"
                    },
                    if disclosure.implies_no_broader_authority {
                        "yes"
                    } else {
                        "NO"
                    }
                ));
            }
            out.push('\n');
        }
        out
    }

    /// Validates the registry's invariants.
    pub fn validate(&self) -> Vec<M5ClientScopeCardViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_CLIENT_SCOPE_CARD_REGISTRY_RECORD_KIND {
            out.push(M5ClientScopeCardViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CLIENT_SCOPE_CARD_SCHEMA_VERSION {
            out.push(M5ClientScopeCardViolation::WrongSchemaVersion);
        }
        if self.registry_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5ClientScopeCardViolation::MissingIdentity);
        }
        if self.cards.is_empty() {
            out.push(M5ClientScopeCardViolation::RegistryHasNoCards);
        }
        let mut seen = std::collections::BTreeSet::new();
        for card in &self.cards {
            if !seen.insert(card.card_id.clone()) {
                out.push(M5ClientScopeCardViolation::DuplicateCardId);
            }
            out.extend(card.validate());
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5ClientScopeCardViolation::VocabularyMismatch);
        }
        let expected_consumers: Vec<String> = PublicTruthConsumer::ALL
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect();
        if self.consumers != expected_consumers {
            out.push(M5ClientScopeCardViolation::ConsumerSetMismatch);
        }
        if self.conformance != derive_registry_conformance(&self.cards)
            || !self.conformance.all_hold()
        {
            out.push(M5ClientScopeCardViolation::ConformanceReviewFailed);
        }
        if self.summary != derive_summary(&self.cards) {
            out.push(M5ClientScopeCardViolation::SummaryMismatch);
        }
        out
    }
}

/// Derives the roll-up summary from the cards.
fn derive_summary(cards: &[ClientScopeCard]) -> ClientScopeCardSummary {
    ClientScopeCardSummary {
        total_cards: cards.len() as u32,
        full_authority_cards: cards.iter().filter(|c| c.is_full_authority()).count() as u32,
        narrowed_cards: cards.iter().filter(|c| c.is_narrowed()).count() as u32,
        total_disclosure_projections: cards.iter().map(|c| c.disclosures.len() as u32).sum(),
        total_blocked_actions: cards.iter().map(|c| c.blocked_actions.len() as u32).sum(),
        total_parity_caveats: cards.iter().map(|c| c.parity_caveats.len() as u32).sum(),
    }
}

/// Derives the registry conformance review from its cards.
fn derive_registry_conformance(cards: &[ClientScopeCard]) -> ClientScopeCardConformance {
    let cards_validate = !cards.is_empty() && cards.iter().all(|c| c.validate().is_empty());

    let all_surface_classes_covered = SurfaceClass::ALL
        .iter()
        .all(|sc| cards.iter().any(|c| c.surface_class == *sc));

    let authority_derived = cards.iter().all(|c| {
        c.authority_class == c.client_scope.authority_class
            && c.handoff_requirement == c.client_scope.handoff_requirement
            && c.client_kind == c.client_scope.client_kind
    });

    let blocked_attributable = cards.iter().all(|c| {
        c.blocked_actions == derive_blocked_actions(&c.client_scope)
            && c.blocked_actions
                .iter()
                .all(|a| !a.reason_message_id.trim().is_empty())
    });

    let caveats_when_narrowed = cards.iter().all(|c| {
        !c.is_narrowed() || (!c.parity_caveats.is_empty() && !c.blocked_actions.is_empty())
    });

    // Every card states its scope and authority on the discovery surface specifically.
    let scope_stated_before_failure = cards.iter().all(|c| {
        c.disclosures
            .iter()
            .find(|d| matches!(d.surface, DisclosureSurface::Discovery))
            .is_some_and(|d| {
                d.surface_class == c.surface_class
                    && d.authority_class == c.authority_class
                    && d.blocked_action_count == c.blocked_actions.len() as u32
                    && d.parity_caveat_count == c.parity_caveats.len() as u32
            })
    });

    let all_disclosures_projected = cards.iter().all(|c| {
        c.disclosures.iter().map(|d| d.surface).collect::<Vec<_>>()
            == DisclosureSurface::ALL.to_vec()
    });

    let deep_link_and_handoff_preserve_truth = cards.iter().all(|c| {
        c.disclosures
            .iter()
            .filter(|d| {
                matches!(
                    d.surface,
                    DisclosureSurface::DeepLink | DisclosureSurface::Handoff
                )
            })
            .all(|d| {
                d.authority_class == c.authority_class
                    && d.handoff_requirement == c.handoff_requirement
                    && d.claims_full_authority == c.claims_full_authority
                    && d.blocked_action_count == c.blocked_actions.len() as u32
                    && d.parity_caveat_count == c.parity_caveats.len() as u32
            })
    });

    // The core guard: every narrowed card carries a caveat and a blocked action on every
    // disclosure and reads as not-full-authority — it can never imply desktop parity by omission.
    let narrowed_never_implies_desktop_parity = cards.iter().all(|c| {
        !c.is_narrowed()
            || (!c.parity_caveats.is_empty()
                && !c.blocked_actions.is_empty()
                && c.disclosures.iter().all(|d| !d.claims_full_authority))
    });
    // The guard is not vacuous: at least one narrowed card exists.
    let narrowed_never_implies_desktop_parity =
        narrowed_never_implies_desktop_parity && cards.iter().any(|c| c.is_narrowed());

    let claim_state_matches = cards
        .iter()
        .all(|c| c.claim_state == derive_claim_state(&c.client_scope));

    let only_desktop_full = cards.iter().all(|c| {
        if c.surface_class.permits_full_authority() {
            true
        } else {
            !c.authority_class.is_full_authority() && !c.claims_full_authority
        }
    });

    let export_clean = cards.iter().all(|c| {
        !json_contains_forbidden_material(
            &serde_json::to_value(c).expect("client-scope card serializes"),
        )
    });

    ClientScopeCardConformance {
        cards_validate,
        all_surface_classes_covered,
        authority_derived_from_descriptor: authority_derived,
        blocked_actions_attributable: blocked_attributable,
        parity_caveats_present_when_narrowed: caveats_when_narrowed,
        scope_stated_before_failure,
        all_disclosures_projected,
        deep_link_and_handoff_preserve_truth,
        narrowed_never_implies_desktop_parity,
        claim_state_matches_narrowing_runtime: claim_state_matches,
        only_desktop_carries_full_authority: only_desktop_full,
        controlled_enums_frozen: ClientScopeCardVocabulary::canonical().matches_canonical(),
        shared_across_consumers: true,
        export_carries_no_raw_material: export_clean,
    }
}

/// Validation failures for the client-scope-card lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ClientScopeCardViolation {
    /// The record kind is wrong.
    WrongRecordKind,
    /// The schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// The re-stated client-scope fields disagree with the bound descriptor.
    DescriptorMismatch,
    /// The full-authority parity flag drifted from a fresh derivation.
    AuthorityParityDrift,
    /// The granted capabilities drifted from a fresh derivation.
    CapabilityDrift,
    /// The blocked actions drifted from a fresh derivation.
    BlockedActionDrift,
    /// The parity caveats drifted from a fresh derivation.
    ParityCaveatDrift,
    /// The claim state drifted from the shared claim-narrowing runtime.
    ClaimStateDrift,
    /// The disclosure projections drifted from a fresh derivation.
    DisclosureDrift,
    /// The copy-safe summary drifted from a fresh derivation.
    CopySafeSummaryDrift,
    /// A desktop card is not bound to a genuine desktop-full scope.
    DesktopScopeNotFull,
    /// A non-desktop card carries full authority or claims parity it lacks.
    NarrowedClaimsFullAuthority,
    /// A narrowed card hides its limits — no parity caveat or no blocked action.
    NarrowedHidesLimits,
    /// A full-authority card carries a parity caveat or blocked action it should not.
    FullAuthorityHasLimits,
    /// The disclosure set does not match the canonical disclosure surfaces.
    DisclosureSetMismatch,
    /// A disclosure diverged from the card's canonical state.
    DisclosureDiverged,
    /// A disclosure implies broader authority than the card holds.
    DisclosureImpliesBroaderAuthority,
    /// A required handoff is not disclosed on a disclosure surface.
    HandoffNotDisclosed,
    /// A guard-review flag does not hold or drifted.
    GuardReviewFailed,
    /// A message id is missing the lane prefix.
    UnprefixedMessageId,
    /// The registry publishes no cards.
    RegistryHasNoCards,
    /// Two cards share a card id.
    DuplicateCardId,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// The consumer set does not match the canonical consumers.
    ConsumerSetMismatch,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// The summary did not match the computed roll-up.
    SummaryMismatch,
    /// The export contains raw provider material.
    RawMaterialInExport,
}

impl M5ClientScopeCardViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::DescriptorMismatch => "descriptor_mismatch",
            Self::AuthorityParityDrift => "authority_parity_drift",
            Self::CapabilityDrift => "capability_drift",
            Self::BlockedActionDrift => "blocked_action_drift",
            Self::ParityCaveatDrift => "parity_caveat_drift",
            Self::ClaimStateDrift => "claim_state_drift",
            Self::DisclosureDrift => "disclosure_drift",
            Self::CopySafeSummaryDrift => "copy_safe_summary_drift",
            Self::DesktopScopeNotFull => "desktop_scope_not_full",
            Self::NarrowedClaimsFullAuthority => "narrowed_claims_full_authority",
            Self::NarrowedHidesLimits => "narrowed_hides_limits",
            Self::FullAuthorityHasLimits => "full_authority_has_limits",
            Self::DisclosureSetMismatch => "disclosure_set_mismatch",
            Self::DisclosureDiverged => "disclosure_diverged",
            Self::DisclosureImpliesBroaderAuthority => "disclosure_implies_broader_authority",
            Self::HandoffNotDisclosed => "handoff_not_disclosed",
            Self::GuardReviewFailed => "guard_review_failed",
            Self::UnprefixedMessageId => "unprefixed_message_id",
            Self::RegistryHasNoCards => "registry_has_no_cards",
            Self::DuplicateCardId => "duplicate_card_id",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ConsumerSetMismatch => "consumer_set_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked raw material. Mirrors the redaction posture of
/// the upstream descriptor lanes.
const FORBIDDEN_KEY_SUBSTRINGS: [&str; 6] = [
    "credential",
    "secret",
    "password",
    "api_key",
    "raw_payload",
    "bearer_token",
];

/// Scans a serialized value for forbidden material. Returns true when a key (case-insensitive)
/// contains a forbidden substring.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => map.iter().any(|(key, child)| {
            let lower = key.to_ascii_lowercase();
            FORBIDDEN_KEY_SUBSTRINGS
                .iter()
                .any(|needle| lower.contains(needle))
                || json_contains_forbidden_material(child)
        }),
        serde_json::Value::Array(items) => items.iter().any(json_contains_forbidden_material),
        _ => false,
    }
}
