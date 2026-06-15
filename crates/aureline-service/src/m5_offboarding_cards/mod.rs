//! Grace-period, seat-loss, cancellation, and org-switch offboarding cards.
//!
//! This module is the canonical offboarding-card object. Where the sibling
//! [`crate::m5_commercial_control_plane`] freezes the per-lane fail posture and
//! the managed-state vocabulary, and [`crate::m5_metering_degradation_rules`]
//! freezes the runtime metering-degradation behavior, this module freezes the
//! *humane offboarding surface* a user or admin sees when a managed entitlement
//! is winding down: what the event is, when it takes effect, which managed
//! features pause, what can still be exported, what keeps running locally, when
//! managed copies are deleted, and who owns the next step.
//!
//! The object freezes exactly one [`OffboardingCard`] per
//! [`LifecycleEvent`] — a grace period, a seat loss, a cancellation, and an org
//! switch — plus one [`CardSurfaceBinding`] per consumer surface. It reuses the
//! closed vocabularies the control-plane matrix already froze —
//! [`ServiceFamily`], [`ManagedStateClass`], [`MarketedClaim`], [`ScopeOwner`],
//! [`GracePeriodRight`], [`ExportGuarantee`], [`EntitlementState`],
//! [`PostureOrigin`], and [`ConsumerSurface`] — rather than minting a parallel
//! synonym set. The new tokens are only the offboarding vocabulary the matrix did
//! not carry: the lifecycle event, the card-action kind, the handoff owner and
//! contact channel, and the final-usage disclosure.
//!
//! Five invariants keep the cards humane and honest. First, **the local core is
//! never blocked**: every card carries a non-empty
//! [`OffboardingCard::local_safe_continuation`] and a deletion timeline whose
//! [`DeletionTimeline::local_artifacts_deleted`] is always false, so a wind-down
//! never deletes or pauses local editing, search, Git, or already-authorized
//! local automation. Second, **local and tenant-scoped state stay separated**:
//! every card carries an [`ArtifactSeparation`] that names what stays on device
//! and user-owned versus what is tenant-scoped and rebinds or is reclaimed, so a
//! seat loss or an org switch never blurs the two. Third, **export and local
//! continuation are never buried**: a card's actions are ranked, and no
//! upgrade/renew action may rank above an export or continue-local action.
//! Fourth, **the four events stay distinct**: every card lists the other three in
//! [`OffboardingCard::must_not_collapse_with`] and asserts it is not a generic
//! sign-in or account error, so a grace window, a seat loss, a cancellation, and
//! an org switch never collapse into one account error. Fifth, **no number
//! crosses the boundary bare**: any final usage figure is bound to its unit,
//! as-of time, and scope owner or suppressed entirely, via
//! [`FinalUsageDisclosure`], and every card carries an as-of time.
//!
//! A card's effective marketed claim is recomputed from its lifecycle event's
//! [`LifecycleEvent::claim_cap`], so a grace window and an org switch narrow the
//! marketed claim to [`MarketedClaim::ManagedNarrowed`] while a seat loss and a
//! cancellation narrow it to [`MarketedClaim::LocalSafeOnly`]; the stored value
//! must equal that recomputation or validation fails.
//! [`CardSet::cross_check_against_control_plane`] confirms each card that maps to
//! a frozen managed state agrees with the control-plane row on the entitlement
//! state, posture origin, and claim cap, so the cards project the matrix rather
//! than a parallel spreadsheet.
//!
//! [`canonical_offboarding_card_set`] builds the frozen set and
//! [`current_stable_offboarding_card_set`] reads and validates the checked-in
//! packet at
//! [`artifacts/service/m5-offboarding-cards.json`](../../../../artifacts/service/m5-offboarding-cards.json),
//! so account/offboarding surfaces, diagnostics, Help/About, the support/admin
//! export, and claim/public-truth automation all ingest one packet rather than
//! cloning status text. The boundary schema is
//! [`schemas/service/m5-offboarding-cards.schema.json`](../../../../schemas/service/m5-offboarding-cards.schema.json).

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_commercial_control_plane::{
    canonical_source_refs, canonical_stable_commercial_control_plane_matrix, ConsumerSurface,
    EntitlementState, ExportGuarantee, GracePeriodRight, ManagedStateClass, MarketedClaim,
    PostureOrigin, ServiceFamily,
};

#[cfg(test)]
mod tests;

/// Supported schema version for the offboarding-card set.
pub const OFFBOARDING_CARDS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the card-set packet.
pub const CARD_SET_RECORD_KIND: &str = "m5_offboarding_card_set";

/// Stable record-kind tag for a single offboarding card.
pub const CARD_RECORD_KIND: &str = "m5_offboarding_card";

/// Stable record-kind tag for a deletion timeline.
pub const DELETION_TIMELINE_RECORD_KIND: &str = "m5_offboarding_deletion_timeline";

/// Stable record-kind tag for an artifact-separation block.
pub const ARTIFACT_SEPARATION_RECORD_KIND: &str = "m5_offboarding_artifact_separation";

/// Stable record-kind tag for an owner-handoff block.
pub const OWNER_HANDOFF_RECORD_KIND: &str = "m5_offboarding_owner_handoff";

/// Stable record-kind tag for a card action.
pub const CARD_ACTION_RECORD_KIND: &str = "m5_offboarding_card_action";

/// Stable record-kind tag for a surface binding.
pub const SURFACE_BINDING_RECORD_KIND: &str = "m5_offboarding_card_surface_binding";

/// Stable record-kind tag for the card-set inspection block.
pub const INSPECTION_RECORD_KIND: &str = "m5_offboarding_card_inspection";

/// Repo-relative path to the boundary schema.
pub const OFFBOARDING_CARDS_SCHEMA_REF: &str = "schemas/service/m5-offboarding-cards.schema.json";

/// Repo-relative path to the reviewer contract.
pub const OFFBOARDING_CARDS_DOC_REF: &str = "docs/m5/add-grace-period-seat-loss-cancellation-and-org-switch-offboarding-cards-with-export-rights-local-safe-continuity-deletion-timeline-and-owner-handoff.md";

/// Repo-relative path to the checked-in card-set packet.
pub const OFFBOARDING_CARDS_ARTIFACT_PATH: &str = "artifacts/service/m5-offboarding-cards.json";

/// The managed-account lifecycle event an offboarding card explains.
///
/// A lifecycle event is an account/entitlement transition, and the four kinds
/// stay distinct: a grace window, a seat loss, a cancellation, and an org switch
/// never collapse into one generic account error, and none of them is a
/// sign-in/reauth failure (which stays in the [`ManagedStateClass`] vocabulary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleEvent {
    /// A typed grace window is open before suspension.
    GracePeriod,
    /// The user's seat was removed, reclaimed, or deprovisioned.
    SeatLoss,
    /// The plan or subscription was cancelled and the entitlement is ending.
    Cancellation,
    /// The account or org was switched or transferred; managed scope is rebinding.
    OrgSwitch,
}

impl LifecycleEvent {
    /// Every lifecycle event, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::GracePeriod,
        Self::SeatLoss,
        Self::Cancellation,
        Self::OrgSwitch,
    ];

    /// The stable token used to build a card id.
    pub const fn slug(self) -> &'static str {
        match self {
            Self::GracePeriod => "grace_period",
            Self::SeatLoss => "seat_loss",
            Self::Cancellation => "cancellation",
            Self::OrgSwitch => "org_switch",
        }
    }

    /// The control-plane managed state this event maps to, when one exists.
    ///
    /// A grace period, a seat loss, and an org switch each have a frozen
    /// [`ManagedStateClass`] token; a cancellation is a distinct lifecycle event
    /// with no single managed-state token, so it never borrows one.
    pub const fn related_managed_state(self) -> Option<ManagedStateClass> {
        match self {
            Self::GracePeriod => Some(ManagedStateClass::GracePeriod),
            Self::SeatLoss => Some(ManagedStateClass::SeatRemoved),
            Self::OrgSwitch => Some(ManagedStateClass::OrgSwitched),
            Self::Cancellation => None,
        }
    }

    /// The marketed-claim cap this event imposes on every managed lane.
    ///
    /// A grace window and an org switch keep managed work in a narrowed form; a
    /// seat loss and a cancellation drop managed work to the local-safe baseline.
    pub const fn claim_cap(self) -> MarketedClaim {
        match self {
            Self::GracePeriod | Self::OrgSwitch => MarketedClaim::ManagedNarrowed,
            Self::SeatLoss | Self::Cancellation => MarketedClaim::LocalSafeOnly,
        }
    }

    /// The other lifecycle events this event must never collapse into.
    pub fn must_not_collapse_with(self) -> Vec<LifecycleEvent> {
        Self::ALL.into_iter().filter(|e| *e != self).collect()
    }
}

/// Who owns the next step after a lifecycle event.
///
/// The handoff names a contact role, never a raw person, account, or email; the
/// surface resolves the role to a real contact locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffOwner {
    /// The seat administrator who can restore or reassign the seat.
    SeatAdministrator,
    /// The organization owner who governs the org switch or transfer.
    OrganizationOwner,
    /// The billing contact who can renew or confirm the cancellation.
    BillingContact,
    /// Managed support.
    SupportContact,
}

/// How the next-step owner is reached.
///
/// The channel is a closed surface class, not a raw URL or address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContactChannel {
    /// The admin console.
    AdminConsole,
    /// The billing portal.
    BillingPortal,
    /// The support portal.
    SupportPortal,
    /// The in-app account panel.
    InAppAccountPanel,
}

/// The kind of action an offboarding card offers.
///
/// Export and local continuation always outrank upgrade or renewal; an
/// [`UpgradeOrRenew`](Self::UpgradeOrRenew) action may never rank above an
/// [`ExportNow`](Self::ExportNow) or [`ContinueLocal`](Self::ContinueLocal)
/// action on the same card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CardActionKind {
    /// Export bounded usage and offboarding artifacts now.
    ExportNow,
    /// Keep working locally; the local core is unaffected.
    ContinueLocal,
    /// Review the deletion timeline.
    ReviewDeletionTimeline,
    /// Contact the owner of the next step.
    ContactOwner,
    /// Renew or upgrade to keep managed actions.
    UpgradeOrRenew,
}

impl CardActionKind {
    /// True when this action is an export or local-continuation action that must
    /// never be outranked by an upgrade/renewal prompt.
    pub const fn is_protected_priority(self) -> bool {
        matches!(self, Self::ExportNow | Self::ContinueLocal)
    }

    /// True when this action is an upgrade or renewal prompt.
    pub const fn is_upgrade_prompt(self) -> bool {
        matches!(self, Self::UpgradeOrRenew)
    }
}

/// How a final usage figure is disclosed on a card.
///
/// A managed value is always bound to its unit, as-of time, and scope owner, or
/// suppressed entirely; it is never shown bare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalUsageDisclosure {
    /// The final usage figure is shown bound to its unit, as-of time, and scope owner.
    BoundToUnitAsOfScope,
    /// No final usage figure is shown; the managed scope is gone or unconfirmable.
    SuppressedNoManagedNumber,
}

impl FinalUsageDisclosure {
    /// True when a number is shown (bound), false when it is suppressed.
    pub const fn shows_number(self) -> bool {
        matches!(self, Self::BoundToUnitAsOfScope)
    }
}

/// One ranked user-visible action an offboarding card offers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardAction {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// The action kind.
    pub kind: CardActionKind,
    /// Render rank; lower is higher priority (rendered first).
    pub rank: u32,
    /// The reviewable label the surface renders.
    pub label: String,
}

impl CardAction {
    /// Builds a ranked action with the given kind and label.
    pub fn new(kind: CardActionKind, rank: u32, label: &str) -> Self {
        Self {
            record_kind: CARD_ACTION_RECORD_KIND.to_owned(),
            schema_version: OFFBOARDING_CARDS_SCHEMA_VERSION,
            kind,
            rank,
            label: label.to_owned(),
        }
    }
}

/// When the managed copies of an ending entitlement are deleted.
///
/// The timeline always names the export-before-suspend/delete deadline so export
/// is never silently lost, and [`Self::local_artifacts_deleted`] is always false
/// because a managed lifecycle event never deletes local data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeletionTimeline {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// When the lifecycle event takes effect.
    pub effective_at: String,
    /// The deadline to export bounded artifacts before suspension or deletion.
    pub export_admissible_until: String,
    /// When the grace window closes. Present for grace and cancellation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grace_window_closes_at: Option<String>,
    /// When managed copies are deleted. Absent when deletion is tenant-governed and unscheduled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_deletion_at: Option<String>,
    /// Always false: local artifacts are never deleted by a managed lifecycle event.
    pub local_artifacts_deleted: bool,
    /// Reviewable note describing what is removed when.
    pub timeline_note: String,
}

/// What stays local versus what is tenant-scoped after a lifecycle event.
///
/// Both lists are non-empty, so a seat loss or an org switch always separates
/// local artifacts (on device, user-owned) from tenant-scoped managed state
/// (rebinds or is reclaimed) rather than blurring the two.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactSeparation {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Local artifacts that stay on device and remain user-owned.
    pub local_artifacts: Vec<String>,
    /// Tenant-scoped managed state that rebinds, is reclaimed, or is retained by the tenant.
    pub tenant_scoped_managed_state: Vec<String>,
}

/// Who owns the next step and how to reach them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnerHandoff {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// The role that owns the next step.
    pub next_step_owner: HandoffOwner,
    /// How that owner is reached.
    pub contact_channel: ContactChannel,
    /// Reviewable instruction naming the next step.
    pub instruction: String,
}

/// One frozen offboarding card for a lifecycle event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingCard {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable card identifier.
    pub card_id: String,
    /// Reviewable card title.
    pub title: String,
    /// Reviewable card summary.
    pub summary: String,
    /// The lifecycle event this card explains.
    pub lifecycle_event: LifecycleEvent,
    /// The control-plane managed state this event maps to, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_managed_state: Option<ManagedStateClass>,
    /// The frozen entitlement state this event binds to.
    pub linked_entitlement_state: EntitlementState,
    /// The posture origin the narrowing is cited back to.
    pub posture_origin: PostureOrigin,
    /// When the event takes effect.
    pub effective_at: String,
    /// The managed features that pause; never empty.
    pub impacted_managed_features: Vec<String>,
    /// The managed service families affected; never empty.
    pub impacted_service_families: Vec<ServiceFamily>,
    /// Non-empty local-safe continuation that always keeps running.
    pub local_safe_continuation: Vec<String>,
    /// The grace-period export rights the event preserves; never empty.
    pub export_rights: Vec<GracePeriodRight>,
    /// The bounded export guarantee for the event's artifacts.
    pub export_guarantee: ExportGuarantee,
    /// When managed copies are deleted and the export deadline.
    pub deletion_timeline: DeletionTimeline,
    /// Who owns the next step and how to reach them.
    pub owner_handoff: OwnerHandoff,
    /// What stays local versus what is tenant-scoped.
    pub artifact_separation: ArtifactSeparation,
    /// The ranked actions; export and local continuation always outrank upgrade/renewal.
    pub actions: Vec<CardAction>,
    /// How any final usage figure is disclosed; bound or suppressed, never bare.
    pub final_usage_disclosure: FinalUsageDisclosure,
    /// Last measurement as-of time; present even when the figure is suppressed.
    pub as_of: String,
    /// The other lifecycle events this card must never collapse into.
    pub must_not_collapse_with: Vec<LifecycleEvent>,
    /// Always true: this card is distinct from a sign-in/reauth failure.
    pub distinct_from_sign_in_failure: bool,
    /// Always true: this card is not a generic account error.
    pub not_a_generic_account_error: bool,
    /// The marketed claim the event declares before narrowing.
    pub declared_marketed_claim: MarketedClaim,
    /// The marketed claim after the event's cap is applied.
    pub effective_marketed_claim: MarketedClaim,
    /// The next user-visible step; the local core always continues.
    pub recovery_cue: String,
}

impl OffboardingCard {
    /// Returns the card's export action, the action that must never be buried.
    pub fn export_action(&self) -> Option<&CardAction> {
        self.actions
            .iter()
            .find(|a| a.kind == CardActionKind::ExportNow)
    }

    /// True when no upgrade/renewal action ranks above an export or continue-local action.
    pub fn export_never_buried(&self) -> bool {
        let protected_max = self
            .actions
            .iter()
            .filter(|a| a.kind.is_protected_priority())
            .map(|a| a.rank)
            .max();
        let Some(protected_max) = protected_max else {
            return false;
        };
        self.actions
            .iter()
            .filter(|a| a.kind.is_upgrade_prompt())
            .all(|u| u.rank > protected_max)
    }
}

/// One surface bound to the offboarding cards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardSurfaceBinding {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable binding identifier.
    pub binding_id: String,
    /// The consumer surface that projects the cards.
    pub consumer_surface: ConsumerSurface,
    /// The card ids this surface resolves through.
    pub bound_card_ids: Vec<String>,
    /// Always true: the surface projects the effective claim, never a stronger one.
    pub projects_effective_claim: bool,
    /// Always true: the surface renders the local-safe continuation.
    pub renders_local_safe_continuation: bool,
    /// Always true: the surface names the owner handoff.
    pub names_owner_handoff: bool,
    /// Always true: the surface keeps export above any upgrade/renewal prompt.
    pub surfaces_export_before_upgrade: bool,
    /// Reviewable summary of what the surface renders.
    pub summary: String,
}

/// Compact inspection block recomputed from the card set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingCardInspection {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Number of cards.
    pub card_count: usize,
    /// Number of surface bindings.
    pub surface_binding_count: usize,
    /// Number of distinct lifecycle events covered.
    pub lifecycle_events_covered: usize,
    /// True when all four lifecycle events appear exactly once.
    pub lifecycle_vocab_complete: bool,
    /// True when every card keeps a non-empty local-safe continuation.
    pub all_cards_local_safe_backed: bool,
    /// True when no card's deletion timeline deletes local artifacts.
    pub never_deletes_local_artifacts: bool,
    /// True when every card separates local artifacts from tenant-scoped managed state.
    pub all_cards_separate_local_from_tenant: bool,
    /// True when every card states a deletion timeline with an export deadline.
    pub all_cards_state_deletion_timeline: bool,
    /// True when every card names an owner handoff.
    pub all_cards_name_owner_handoff: bool,
    /// True when no card buries export or local continuation beneath an upgrade prompt.
    pub export_never_buried: bool,
    /// True when no card shows a bare number: every figure is bound or suppressed, with an as-of time.
    pub value_never_bare: bool,
    /// True when every card stays distinct from the other three events and a sign-in failure.
    pub distinctness_complete: bool,
    /// Number of cards narrowed to a reduced managed claim.
    pub narrowed_card_count: usize,
    /// Number of cards narrowed to the local-safe-only claim.
    pub local_safe_only_card_count: usize,
}

/// The frozen offboarding-card set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardSet {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable set identifier.
    pub set_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Integer revision for the set content.
    pub set_revision: u32,
    /// Reviewable set title.
    pub title: String,
    /// Reviewable set summary.
    pub summary: String,
    /// Source schema and contract refs the set cites.
    pub source_refs: Vec<String>,
    /// The offboarding cards.
    pub cards: Vec<OffboardingCard>,
    /// The surface bindings.
    pub surface_bindings: Vec<CardSurfaceBinding>,
    /// The recomputed inspection block.
    pub inspection: OffboardingCardInspection,
}

impl CardSet {
    /// Returns the card for `event`, when one is frozen.
    pub fn card_for(&self, event: LifecycleEvent) -> Option<&OffboardingCard> {
        self.cards.iter().find(|c| c.lifecycle_event == event)
    }

    /// Serializes the set as pretty JSON safe for the checked-in artifact and exports.
    ///
    /// # Panics
    ///
    /// Panics only if the set cannot be serialized, which a validated set never is.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("offboarding card set serializes to JSON")
    }

    /// Validates the set and recomputes every derived value.
    ///
    /// Returns an empty vector when the set is internally consistent. Otherwise
    /// returns one [`OffboardingCardViolation`] per failed invariant: a wrong
    /// record kind or schema version, a missing identifier, a duplicate card, an
    /// incomplete lifecycle-event set, an empty local-safe continuation, a
    /// deletion timeline that deletes local artifacts or omits the export
    /// deadline, an unseparated artifact set, a missing owner handoff, a buried
    /// export action, a bare number, a missing distinctness, an effective claim
    /// that does not match the event cap, an unbound surface, or a stale
    /// inspection block.
    pub fn validate(&self) -> Vec<OffboardingCardViolation> {
        let mut violations = Vec::new();
        let mut push = |field: &str, message: &str| {
            violations.push(OffboardingCardViolation {
                field: field.to_owned(),
                message: message.to_owned(),
            });
        };

        if self.record_kind != CARD_SET_RECORD_KIND {
            push("record_kind", "set record_kind is wrong");
        }
        if self.schema_version != OFFBOARDING_CARDS_SCHEMA_VERSION {
            push("schema_version", "set schema_version is wrong");
        }
        if self.set_id.trim().is_empty() {
            push("set_id", "set_id must be non-empty");
        }
        if self.generated_at.trim().is_empty() {
            push("generated_at", "generated_at must be non-empty");
        }
        if self.title.trim().is_empty() {
            push("title", "title must be non-empty");
        }
        if self.summary.trim().is_empty() {
            push("summary", "summary must be non-empty");
        }
        if self.set_revision == 0 {
            push("set_revision", "set_revision must be at least 1");
        }
        if !self
            .source_refs
            .iter()
            .any(|entry| entry == OFFBOARDING_CARDS_SCHEMA_REF)
        {
            push("source_refs", "set must cite its boundary schema");
        }
        if self.cards.is_empty() {
            push("cards", "set must contain at least one card");
        }

        let mut card_ids = BTreeSet::new();
        let mut seen_events = BTreeSet::new();
        for card in &self.cards {
            self.validate_card(card, &mut push);
            if !card_ids.insert(card.card_id.as_str()) {
                push("cards", "card_id values must be unique");
            }
            // Exactly one card per lifecycle event: the set never doubles an event.
            if !seen_events.insert(card.lifecycle_event) {
                push("cards", "each lifecycle event must carry at most one card");
            }
        }

        // The vocabulary is exhaustive: every lifecycle event carries a card.
        for event in LifecycleEvent::ALL {
            if !self.cards.iter().any(|c| c.lifecycle_event == event) {
                push("cards", "every lifecycle event must carry a card");
            }
        }

        self.validate_surface_bindings(&mut push);

        let derived = OffboardingCardInspection::derive(&self.cards, &self.surface_bindings);
        if derived != self.inspection {
            push(
                "inspection",
                "stored inspection block does not match the recomputed set",
            );
        }

        violations
    }

    fn validate_card(&self, card: &OffboardingCard, push: &mut impl FnMut(&str, &str)) {
        if card.record_kind != CARD_RECORD_KIND {
            push("card.record_kind", "card record_kind is wrong");
        }
        if card.schema_version != OFFBOARDING_CARDS_SCHEMA_VERSION {
            push("card.schema_version", "card schema_version is wrong");
        }
        for (field, value) in [
            ("card.card_id", &card.card_id),
            ("card.title", &card.title),
            ("card.summary", &card.summary),
            ("card.effective_at", &card.effective_at),
            ("card.recovery_cue", &card.recovery_cue),
            ("card.as_of", &card.as_of),
        ] {
            if value.trim().is_empty() {
                push(field, "value must be non-empty");
            }
        }

        // The local core is never blocked: every card keeps a non-empty
        // continuation that always runs.
        if card.local_safe_continuation.is_empty()
            || card
                .local_safe_continuation
                .iter()
                .any(|s| s.trim().is_empty())
        {
            push(
                "card.local_safe_continuation",
                "every card must keep a non-empty local-safe continuation",
            );
        }
        if card.impacted_managed_features.is_empty()
            || card
                .impacted_managed_features
                .iter()
                .any(|s| s.trim().is_empty())
        {
            push(
                "card.impacted_managed_features",
                "every card must name at least one impacted managed feature",
            );
        }
        if card.impacted_service_families.is_empty() {
            push(
                "card.impacted_service_families",
                "every card must name at least one impacted service family",
            );
        }
        if card.export_rights.is_empty() {
            push(
                "card.export_rights",
                "every card must name at least one export right",
            );
        }

        self.validate_deletion_timeline(card, push);
        self.validate_artifact_separation(card, push);
        self.validate_owner_handoff(card, push);
        self.validate_actions(card, push);
        self.validate_disclosure(card, push);
        self.validate_distinctness(card, push);
        self.validate_claim(card, push);

        // The related managed state is recomputed from the event.
        if card.related_managed_state != card.lifecycle_event.related_managed_state() {
            push(
                "card.related_managed_state",
                "stored related managed state does not match the lifecycle event",
            );
        }
    }

    fn validate_deletion_timeline(
        &self,
        card: &OffboardingCard,
        push: &mut impl FnMut(&str, &str),
    ) {
        let timeline = &card.deletion_timeline;
        if timeline.record_kind != DELETION_TIMELINE_RECORD_KIND {
            push(
                "card.deletion_timeline.record_kind",
                "deletion-timeline record_kind is wrong",
            );
        }
        if timeline.schema_version != OFFBOARDING_CARDS_SCHEMA_VERSION {
            push(
                "card.deletion_timeline.schema_version",
                "deletion-timeline schema_version is wrong",
            );
        }
        // A managed lifecycle event never deletes local artifacts.
        if timeline.local_artifacts_deleted {
            push(
                "card.deletion_timeline.local_artifacts_deleted",
                "a lifecycle event must never delete local artifacts",
            );
        }
        // Export is never silently lost: the deadline is always stated.
        if timeline.export_admissible_until.trim().is_empty() {
            push(
                "card.deletion_timeline.export_admissible_until",
                "the deletion timeline must state the export deadline",
            );
        }
        if timeline.effective_at.trim().is_empty() {
            push(
                "card.deletion_timeline.effective_at",
                "the deletion timeline must state the effective date",
            );
        }
        if timeline.timeline_note.trim().is_empty() {
            push(
                "card.deletion_timeline.timeline_note",
                "the deletion timeline must carry a note",
            );
        }
        // The timeline's effective date must agree with the card's effective date.
        if timeline.effective_at != card.effective_at {
            push(
                "card.deletion_timeline.effective_at",
                "the deletion timeline effective date must match the card effective date",
            );
        }
        for (field, value) in [
            (
                "card.deletion_timeline.grace_window_closes_at",
                &timeline.grace_window_closes_at,
            ),
            (
                "card.deletion_timeline.managed_deletion_at",
                &timeline.managed_deletion_at,
            ),
        ] {
            if let Some(stamp) = value {
                if stamp.trim().is_empty() {
                    push(
                        field,
                        "an optional timestamp must be non-empty when present",
                    );
                }
            }
        }
    }

    fn validate_artifact_separation(
        &self,
        card: &OffboardingCard,
        push: &mut impl FnMut(&str, &str),
    ) {
        let separation = &card.artifact_separation;
        if separation.record_kind != ARTIFACT_SEPARATION_RECORD_KIND {
            push(
                "card.artifact_separation.record_kind",
                "artifact-separation record_kind is wrong",
            );
        }
        if separation.schema_version != OFFBOARDING_CARDS_SCHEMA_VERSION {
            push(
                "card.artifact_separation.schema_version",
                "artifact-separation schema_version is wrong",
            );
        }
        // Local and tenant-scoped state stay separated: both sides are non-empty.
        if separation.local_artifacts.is_empty()
            || separation
                .local_artifacts
                .iter()
                .any(|s| s.trim().is_empty())
        {
            push(
                "card.artifact_separation.local_artifacts",
                "every card must name the local artifacts that stay on device",
            );
        }
        if separation.tenant_scoped_managed_state.is_empty()
            || separation
                .tenant_scoped_managed_state
                .iter()
                .any(|s| s.trim().is_empty())
        {
            push(
                "card.artifact_separation.tenant_scoped_managed_state",
                "every card must name the tenant-scoped managed state separately from local artifacts",
            );
        }
    }

    fn validate_owner_handoff(&self, card: &OffboardingCard, push: &mut impl FnMut(&str, &str)) {
        let handoff = &card.owner_handoff;
        if handoff.record_kind != OWNER_HANDOFF_RECORD_KIND {
            push(
                "card.owner_handoff.record_kind",
                "owner-handoff record_kind is wrong",
            );
        }
        if handoff.schema_version != OFFBOARDING_CARDS_SCHEMA_VERSION {
            push(
                "card.owner_handoff.schema_version",
                "owner-handoff schema_version is wrong",
            );
        }
        if handoff.instruction.trim().is_empty() {
            push(
                "card.owner_handoff.instruction",
                "the owner handoff must name the next step",
            );
        }
    }

    fn validate_actions(&self, card: &OffboardingCard, push: &mut impl FnMut(&str, &str)) {
        let mut ranks = BTreeSet::new();
        let mut kinds = BTreeSet::new();
        for action in &card.actions {
            if action.record_kind != CARD_ACTION_RECORD_KIND {
                push("card.action.record_kind", "action record_kind is wrong");
            }
            if action.schema_version != OFFBOARDING_CARDS_SCHEMA_VERSION {
                push(
                    "card.action.schema_version",
                    "action schema_version is wrong",
                );
            }
            if action.label.trim().is_empty() {
                push("card.action.label", "action label must be non-empty");
            }
            if !ranks.insert(action.rank) {
                push("card.actions", "action ranks must be unique within a card");
            }
            kinds.insert(action.kind);
        }
        // Export and local continuation are always offered.
        if !kinds.contains(&CardActionKind::ExportNow) {
            push("card.actions", "every card must offer an export action");
        }
        if !kinds.contains(&CardActionKind::ContinueLocal) {
            push(
                "card.actions",
                "every card must offer a local-continuation action",
            );
        }
        // Export and local continuation are never buried beneath an upgrade prompt.
        if !card.export_never_buried() {
            push(
                "card.actions",
                "export and local continuation must outrank any upgrade or renewal prompt",
            );
        }
    }

    fn validate_disclosure(&self, card: &OffboardingCard, push: &mut impl FnMut(&str, &str)) {
        // No number crosses the boundary bare: every card carries an as-of time
        // regardless of whether a figure is shown.
        if card.as_of.trim().is_empty() {
            push(
                "card.as_of",
                "every card must carry an as-of time so a number is never shown without one",
            );
        }
    }

    fn validate_distinctness(&self, card: &OffboardingCard, push: &mut impl FnMut(&str, &str)) {
        // The four events stay distinct: every card lists the other three.
        let expected: BTreeSet<LifecycleEvent> = card
            .lifecycle_event
            .must_not_collapse_with()
            .into_iter()
            .collect();
        let stored: BTreeSet<LifecycleEvent> =
            card.must_not_collapse_with.iter().copied().collect();
        if stored != expected {
            push(
                "card.must_not_collapse_with",
                "a card must stay distinct from the other three lifecycle events",
            );
        }
        if card.must_not_collapse_with.contains(&card.lifecycle_event) {
            push(
                "card.must_not_collapse_with",
                "a card cannot be distinct from itself",
            );
        }
        if !card.distinct_from_sign_in_failure {
            push(
                "card.distinct_from_sign_in_failure",
                "a card must stay distinct from a sign-in or reauth failure",
            );
        }
        if !card.not_a_generic_account_error {
            push(
                "card.not_a_generic_account_error",
                "a card must declare it is not a generic account error",
            );
        }
    }

    fn validate_claim(&self, card: &OffboardingCard, push: &mut impl FnMut(&str, &str)) {
        if card.declared_marketed_claim != MarketedClaim::ManagedFull {
            push(
                "card.declared_marketed_claim",
                "a card declares the full managed claim before narrowing",
            );
        }
        // The marketed claim narrows automatically from the event cap.
        if card.effective_marketed_claim != card.lifecycle_event.claim_cap() {
            push(
                "card.effective_marketed_claim",
                "the effective claim must equal the lifecycle event's cap",
            );
        }
    }

    fn validate_surface_bindings(&self, push: &mut impl FnMut(&str, &str)) {
        let card_ids: BTreeSet<&str> = self.cards.iter().map(|c| c.card_id.as_str()).collect();
        let mut binding_ids = BTreeSet::new();
        for binding in &self.surface_bindings {
            if binding.record_kind != SURFACE_BINDING_RECORD_KIND {
                push(
                    "surface_binding.record_kind",
                    "binding record_kind is wrong",
                );
            }
            if binding.schema_version != OFFBOARDING_CARDS_SCHEMA_VERSION {
                push(
                    "surface_binding.schema_version",
                    "binding schema_version is wrong",
                );
            }
            if binding.binding_id.trim().is_empty() {
                push("surface_binding.binding_id", "binding_id must be non-empty");
            }
            if !binding_ids.insert(binding.binding_id.as_str()) {
                push("surface_bindings", "binding_id values must be unique");
            }
            if binding.summary.trim().is_empty() {
                push(
                    "surface_binding.summary",
                    "binding summary must be non-empty",
                );
            }
            if !binding.projects_effective_claim {
                push(
                    "surface_binding.projects_effective_claim",
                    "a surface must project the effective claim, never a stronger one",
                );
            }
            if !binding.renders_local_safe_continuation {
                push(
                    "surface_binding.renders_local_safe_continuation",
                    "a surface must render the local-safe continuation",
                );
            }
            if !binding.names_owner_handoff {
                push(
                    "surface_binding.names_owner_handoff",
                    "a surface must name the owner handoff",
                );
            }
            if !binding.surfaces_export_before_upgrade {
                push(
                    "surface_binding.surfaces_export_before_upgrade",
                    "a surface must keep export above any upgrade or renewal prompt",
                );
            }
            if binding.bound_card_ids.is_empty() {
                push(
                    "surface_binding.bound_card_ids",
                    "a binding must resolve through at least one card",
                );
            }
            for card_ref in &binding.bound_card_ids {
                if !card_ids.contains(card_ref.as_str()) {
                    push(
                        "surface_binding.bound_card_ids",
                        "binding card ref must resolve to a card",
                    );
                }
            }
        }
        // Every consumer surface must be bound.
        for surface in ConsumerSurface::ALL {
            if !self
                .surface_bindings
                .iter()
                .any(|b| b.consumer_surface == surface)
            {
                push(
                    "surface_bindings",
                    "account, diagnostics, Help/About, support/admin, and claim automation must all bind",
                );
                break;
            }
        }
    }

    /// Cross-checks every card against its control-plane managed-state row.
    ///
    /// Confirms each card that maps to a frozen [`ManagedStateClass`] — a grace
    /// period, a seat loss, and an org switch — agrees with the
    /// commercial-control-plane row on the linked entitlement state, the posture
    /// origin, and the claim cap, so a card narrows the marketed claim the same
    /// way the matrix does rather than inventing a parallel narrowing. Returns an
    /// empty vector when every mapped card matches its row.
    pub fn cross_check_against_control_plane(&self) -> Vec<OffboardingCardViolation> {
        let matrix = canonical_stable_commercial_control_plane_matrix();
        let mut violations = Vec::new();
        for card in &self.cards {
            let Some(state) = card.related_managed_state else {
                continue;
            };
            let Some(row) = matrix
                .managed_states
                .iter()
                .find(|r| r.managed_state == state)
            else {
                violations.push(OffboardingCardViolation {
                    field: "card.related_managed_state".to_owned(),
                    message: format!(
                        "card {} maps to managed state {state:?} with no control-plane row",
                        card.card_id
                    ),
                });
                continue;
            };
            let mut mismatch = |field: &str| {
                violations.push(OffboardingCardViolation {
                    field: field.to_owned(),
                    message: format!(
                        "card {} drifted from control-plane managed state {state:?}",
                        card.card_id
                    ),
                });
            };
            if card.linked_entitlement_state != row.linked_entitlement_state {
                mismatch("card.linked_entitlement_state");
            }
            if card.posture_origin != row.posture_origin {
                mismatch("card.posture_origin");
            }
            // The card's narrowed claim is the matrix state's cap.
            if card.effective_marketed_claim != row.claim_cap {
                mismatch("card.effective_marketed_claim");
            }
        }
        violations
    }
}

impl OffboardingCardInspection {
    fn derive(cards: &[OffboardingCard], surface_bindings: &[CardSurfaceBinding]) -> Self {
        let events: BTreeSet<LifecycleEvent> = cards.iter().map(|c| c.lifecycle_event).collect();

        let narrowed_card_count = cards
            .iter()
            .filter(|c| c.effective_marketed_claim != c.declared_marketed_claim)
            .count();
        let local_safe_only_card_count = cards
            .iter()
            .filter(|c| c.effective_marketed_claim == MarketedClaim::LocalSafeOnly)
            .count();

        Self {
            record_kind: INSPECTION_RECORD_KIND.to_owned(),
            schema_version: OFFBOARDING_CARDS_SCHEMA_VERSION,
            card_count: cards.len(),
            surface_binding_count: surface_bindings.len(),
            lifecycle_events_covered: events.len(),
            lifecycle_vocab_complete: events.len() == LifecycleEvent::ALL.len(),
            all_cards_local_safe_backed: cards
                .iter()
                .all(|c| !c.local_safe_continuation.is_empty()),
            never_deletes_local_artifacts: cards
                .iter()
                .all(|c| !c.deletion_timeline.local_artifacts_deleted),
            all_cards_separate_local_from_tenant: cards.iter().all(|c| {
                !c.artifact_separation.local_artifacts.is_empty()
                    && !c.artifact_separation.tenant_scoped_managed_state.is_empty()
            }),
            all_cards_state_deletion_timeline: cards.iter().all(|c| {
                !c.deletion_timeline
                    .export_admissible_until
                    .trim()
                    .is_empty()
            }),
            all_cards_name_owner_handoff: cards
                .iter()
                .all(|c| !c.owner_handoff.instruction.trim().is_empty()),
            export_never_buried: cards.iter().all(|c| c.export_never_buried()),
            value_never_bare: cards.iter().all(|c| !c.as_of.trim().is_empty()),
            distinctness_complete: cards.iter().all(|c| {
                let expected: BTreeSet<LifecycleEvent> = c
                    .lifecycle_event
                    .must_not_collapse_with()
                    .into_iter()
                    .collect();
                let stored: BTreeSet<LifecycleEvent> =
                    c.must_not_collapse_with.iter().copied().collect();
                stored == expected
                    && c.distinct_from_sign_in_failure
                    && c.not_a_generic_account_error
            }),
            narrowed_card_count,
            local_safe_only_card_count,
        }
    }
}

/// One failed card-set invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffboardingCardViolation {
    /// The field path that failed.
    pub field: String,
    /// A short reviewable message.
    pub message: String,
}

impl fmt::Display for OffboardingCardViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

/// Error returned when the checked-in set cannot be read or validated.
#[derive(Debug)]
pub enum OffboardingCardError {
    /// The checked-in JSON failed to parse.
    Parse(serde_json::Error),
    /// The checked-in set failed validation.
    Validation(Vec<OffboardingCardViolation>),
}

impl fmt::Display for OffboardingCardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(f, "offboarding card set parse error: {err}"),
            Self::Validation(violations) => write!(
                f,
                "offboarding card set failed validation: {} violation(s)",
                violations.len()
            ),
        }
    }
}

impl std::error::Error for OffboardingCardError {}

/// Reads and validates the checked-in stable offboarding-card set.
///
/// This is the canonical reader: account/offboarding surfaces, diagnostics,
/// Help/About, the support/admin export, and claim/public-truth automation call
/// it to ingest the cards rather than cloning status text.
///
/// # Errors
///
/// Returns [`OffboardingCardError`] when the checked-in packet fails to parse or
/// fails validation.
pub fn current_stable_offboarding_card_set() -> Result<CardSet, OffboardingCardError> {
    let set: CardSet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/service/m5-offboarding-cards.json"
    )))
    .map_err(OffboardingCardError::Parse)?;
    let violations = set.validate();
    if violations.is_empty() {
        Ok(set)
    } else {
        Err(OffboardingCardError::Validation(violations))
    }
}

/// Source refs every offboarding-card export carries.
fn card_source_refs() -> Vec<String> {
    let mut refs = vec![
        OFFBOARDING_CARDS_SCHEMA_REF.to_owned(),
        OFFBOARDING_CARDS_DOC_REF.to_owned(),
    ];
    // Reuse the control-plane refs so the cards cite the same frozen vocabulary.
    refs.extend(canonical_source_refs());
    refs
}

/// Deterministic event-effective date for the checked-in cards.
pub const STABLE_EFFECTIVE_AT: &str = "2026-06-15T00:00:00Z";

/// Deterministic export deadline for the checked-in cards.
pub const STABLE_EXPORT_UNTIL: &str = "2026-06-29T00:00:00Z";

/// Deterministic managed-deletion date for the checked-in cards.
pub const STABLE_MANAGED_DELETION_AT: &str = "2026-07-29T00:00:00Z";

/// Deterministic as-of time for the checked-in cards.
pub const STABLE_AS_OF: &str = "2026-06-15T00:00:00Z";

/// The fixed, per-event data a card is built from.
struct CardDef {
    lifecycle_event: LifecycleEvent,
    title: &'static str,
    summary: &'static str,
    linked_entitlement_state: EntitlementState,
    posture_origin: PostureOrigin,
    impacted_managed_features: &'static [&'static str],
    impacted_service_families: &'static [ServiceFamily],
    local_safe_continuation: &'static [&'static str],
    export_rights: &'static [GracePeriodRight],
    export_guarantee: ExportGuarantee,
    local_artifacts: &'static [&'static str],
    tenant_scoped_managed_state: &'static [&'static str],
    handoff_owner: HandoffOwner,
    contact_channel: ContactChannel,
    handoff_instruction: &'static str,
    final_usage_disclosure: FinalUsageDisclosure,
    grace_window_closes_at: Option<&'static str>,
    managed_deletion_at: Option<&'static str>,
    timeline_note: &'static str,
    recovery_cue: &'static str,
    has_upgrade_action: bool,
}

fn build_actions(has_upgrade_action: bool) -> Vec<CardAction> {
    let mut actions = vec![
        CardAction::new(
            CardActionKind::ExportNow,
            1,
            "Export your bounded usage and offboarding artifacts now (CSV and JSON where offered).",
        ),
        CardAction::new(
            CardActionKind::ContinueLocal,
            2,
            "Keep editing, searching, and using Git locally; local work is unaffected.",
        ),
        CardAction::new(
            CardActionKind::ReviewDeletionTimeline,
            3,
            "Review the deletion timeline to see what is removed and when.",
        ),
        CardAction::new(
            CardActionKind::ContactOwner,
            4,
            "Contact the owner of the next step.",
        ),
    ];
    if has_upgrade_action {
        actions.push(CardAction::new(
            CardActionKind::UpgradeOrRenew,
            5,
            "Renew or upgrade to keep managed actions; this never outranks export or local continuation.",
        ));
    }
    actions
}

fn build_card(def: &CardDef) -> OffboardingCard {
    let timeline = DeletionTimeline {
        record_kind: DELETION_TIMELINE_RECORD_KIND.to_owned(),
        schema_version: OFFBOARDING_CARDS_SCHEMA_VERSION,
        effective_at: STABLE_EFFECTIVE_AT.to_owned(),
        export_admissible_until: STABLE_EXPORT_UNTIL.to_owned(),
        grace_window_closes_at: def.grace_window_closes_at.map(|s| s.to_owned()),
        managed_deletion_at: def.managed_deletion_at.map(|s| s.to_owned()),
        local_artifacts_deleted: false,
        timeline_note: def.timeline_note.to_owned(),
    };

    let separation = ArtifactSeparation {
        record_kind: ARTIFACT_SEPARATION_RECORD_KIND.to_owned(),
        schema_version: OFFBOARDING_CARDS_SCHEMA_VERSION,
        local_artifacts: def
            .local_artifacts
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        tenant_scoped_managed_state: def
            .tenant_scoped_managed_state
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
    };

    let handoff = OwnerHandoff {
        record_kind: OWNER_HANDOFF_RECORD_KIND.to_owned(),
        schema_version: OFFBOARDING_CARDS_SCHEMA_VERSION,
        next_step_owner: def.handoff_owner,
        contact_channel: def.contact_channel,
        instruction: def.handoff_instruction.to_owned(),
    };

    OffboardingCard {
        record_kind: CARD_RECORD_KIND.to_owned(),
        schema_version: OFFBOARDING_CARDS_SCHEMA_VERSION,
        card_id: format!("offboarding_card.{}", def.lifecycle_event.slug()),
        title: def.title.to_owned(),
        summary: def.summary.to_owned(),
        lifecycle_event: def.lifecycle_event,
        related_managed_state: def.lifecycle_event.related_managed_state(),
        linked_entitlement_state: def.linked_entitlement_state,
        posture_origin: def.posture_origin,
        effective_at: STABLE_EFFECTIVE_AT.to_owned(),
        impacted_managed_features: def
            .impacted_managed_features
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        impacted_service_families: def.impacted_service_families.to_vec(),
        local_safe_continuation: def
            .local_safe_continuation
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        export_rights: def.export_rights.to_vec(),
        export_guarantee: def.export_guarantee,
        deletion_timeline: timeline,
        owner_handoff: handoff,
        artifact_separation: separation,
        actions: build_actions(def.has_upgrade_action),
        final_usage_disclosure: def.final_usage_disclosure,
        as_of: STABLE_AS_OF.to_owned(),
        must_not_collapse_with: def.lifecycle_event.must_not_collapse_with(),
        distinct_from_sign_in_failure: true,
        not_a_generic_account_error: true,
        declared_marketed_claim: MarketedClaim::ManagedFull,
        effective_marketed_claim: def.lifecycle_event.claim_cap(),
        recovery_cue: def.recovery_cue.to_owned(),
    }
}

fn binding(
    binding_id: &str,
    consumer_surface: ConsumerSurface,
    bound_card_ids: &[&str],
    summary: &str,
) -> CardSurfaceBinding {
    CardSurfaceBinding {
        record_kind: SURFACE_BINDING_RECORD_KIND.to_owned(),
        schema_version: OFFBOARDING_CARDS_SCHEMA_VERSION,
        binding_id: binding_id.to_owned(),
        consumer_surface,
        bound_card_ids: bound_card_ids.iter().map(|s| (*s).to_owned()).collect(),
        projects_effective_claim: true,
        renders_local_safe_continuation: true,
        names_owner_handoff: true,
        surfaces_export_before_upgrade: true,
        summary: summary.to_owned(),
    }
}

/// Stable identifier for the checked-in set.
pub const STABLE_SET_ID: &str = "offboarding-cards:stable:0001";

/// Stable title for the checked-in set.
pub const STABLE_SET_TITLE: &str =
    "Grace-period, seat-loss, cancellation, and org-switch offboarding cards with export rights, local-safe continuity, deletion timeline, and owner handoff";

/// Deterministic timestamp for the checked-in set.
pub const STABLE_SET_GENERATED_AT: &str = "2026-06-15T00:00:00Z";

/// Revision for the checked-in set.
pub const STABLE_SET_REVISION: u32 = 1;

/// Builds the checked-in set with the stable identity constants.
///
/// The checked-in artifact, the conformance dump, and the round-trip test all
/// build through this function so they agree on every field.
pub fn canonical_stable_offboarding_card_set() -> CardSet {
    canonical_offboarding_card_set(
        STABLE_SET_ID.to_owned(),
        STABLE_SET_TITLE.to_owned(),
        STABLE_SET_GENERATED_AT.to_owned(),
        STABLE_SET_REVISION,
    )
}

/// Builds the canonical, frozen offboarding-card set.
///
/// The set freezes one card per [`LifecycleEvent`] — a grace period, a seat
/// loss, a cancellation, and an org switch — plus one binding per consumer
/// surface. Each card states the event type, effective date, impacted managed
/// features, export rights, local-safe continuation, deletion timeline, and owner
/// handoff, separates local artifacts from tenant-scoped managed state, keeps
/// export above any upgrade prompt, stays distinct from the other three events
/// and a sign-in failure, and narrows the marketed claim from the event's cap.
pub fn canonical_offboarding_card_set(
    set_id: String,
    title: String,
    generated_at: String,
    set_revision: u32,
) -> CardSet {
    let defs = [
        CardDef {
            lifecycle_event: LifecycleEvent::GracePeriod,
            title: "Grace period open",
            summary: "A typed grace window is open before suspension. Managed actions stay admissible per the window and bounded artifacts can still be exported; local work is unaffected.",
            linked_entitlement_state: EntitlementState::EntitlementInGrace,
            posture_origin: PostureOrigin::Account,
            impacted_managed_features: &[
                "Managed AI gateway inference narrows to the grace allowance.",
                "Managed sync, relay, registry, and workspace lanes stay admissible per the grace window.",
            ],
            impacted_service_families: &[
                ServiceFamily::AiGatewayFamily,
                ServiceFamily::SyncFamily,
                ServiceFamily::CollaborationRelayFamily,
                ServiceFamily::RegistryOrMirrorMetadataFamily,
                ServiceFamily::TelemetryOrSupportIngestFamily,
                ServiceFamily::RemoteWorkspaceControlPlaneFamily,
            ],
            local_safe_continuation: &[
                "Local editing, search, and Git continue without interruption.",
                "Already-authorized local automation keeps running.",
            ],
            export_rights: &[
                GracePeriodRight::ManagedAdmissiblePerGrace,
                GracePeriodRight::ExportBeforeSuspend,
                GracePeriodRight::OffboardingExportAdmissible,
            ],
            export_guarantee: ExportGuarantee::ParityWithCsvAndJson,
            local_artifacts: &[
                "Local files, settings, and Git history stay on device and remain yours.",
                "Locally cached usage snapshots stay available, labeled with their as-of time.",
            ],
            tenant_scoped_managed_state: &[
                "Managed quota and usage counters stay tenant-scoped and reset per the grace window.",
                "Managed-side copies are suspended, not deleted, until the grace window closes.",
            ],
            handoff_owner: HandoffOwner::BillingContact,
            contact_channel: ContactChannel::BillingPortal,
            handoff_instruction: "The billing contact can renew before the grace window closes to keep managed actions; export first either way.",
            final_usage_disclosure: FinalUsageDisclosure::BoundToUnitAsOfScope,
            grace_window_closes_at: Some(STABLE_EXPORT_UNTIL),
            managed_deletion_at: Some(STABLE_MANAGED_DELETION_AT),
            timeline_note: "Export until the grace window closes; managed copies are suspended at close and deleted after the retention window. Local data is never deleted.",
            recovery_cue: "Export bounded artifacts before the grace window closes, then renew or continue locally; local work continues now.",
            has_upgrade_action: true,
        },
        CardDef {
            lifecycle_event: LifecycleEvent::SeatLoss,
            title: "Seat removed",
            summary: "Your seat was removed, reclaimed, or deprovisioned. Managed actions for this seat stop; your local work and local artifacts are unaffected.",
            linked_entitlement_state: EntitlementState::EntitlementSuspendedAdmin,
            posture_origin: PostureOrigin::Seat,
            impacted_managed_features: &[
                "Managed AI gateway, sync, relay, registry, support ingest, and workspace actions for this seat stop.",
                "Tenant-scoped managed state for this seat is reclaimed by the organization.",
            ],
            impacted_service_families: &[
                ServiceFamily::AiGatewayFamily,
                ServiceFamily::SyncFamily,
                ServiceFamily::CollaborationRelayFamily,
                ServiceFamily::RegistryOrMirrorMetadataFamily,
                ServiceFamily::TelemetryOrSupportIngestFamily,
                ServiceFamily::RemoteWorkspaceControlPlaneFamily,
            ],
            local_safe_continuation: &[
                "Local editing, search, save, and Git continue; nothing local depends on the seat.",
                "Already-authorized local automation keeps running.",
            ],
            export_rights: &[
                GracePeriodRight::ExportBeforeSuspend,
                GracePeriodRight::OffboardingExportAdmissible,
            ],
            export_guarantee: ExportGuarantee::ParityWithCsvAndJson,
            local_artifacts: &[
                "Local files, settings, and Git history stay on device and remain yours.",
                "Local support bundles you generated stay on device.",
            ],
            tenant_scoped_managed_state: &[
                "The seat's managed quota, usage, and synced settings are tenant-scoped and reclaimed by the organization.",
                "Managed-side copies are governed by the organization's retention policy.",
            ],
            handoff_owner: HandoffOwner::SeatAdministrator,
            contact_channel: ContactChannel::AdminConsole,
            handoff_instruction: "A seat administrator can restore or reassign the seat to resume managed actions; this is a seat change, not a sign-in failure.",
            final_usage_disclosure: FinalUsageDisclosure::SuppressedNoManagedNumber,
            grace_window_closes_at: None,
            managed_deletion_at: None,
            timeline_note: "Export bounded artifacts before the seat's managed access ends; managed-side deletion follows the organization's retention policy. Local data is never deleted.",
            recovery_cue: "Ask an admin to restore the seat to resume managed actions; local work continues now.",
            has_upgrade_action: false,
        },
        CardDef {
            lifecycle_event: LifecycleEvent::Cancellation,
            title: "Plan cancelled",
            summary: "The plan or subscription was cancelled and the managed entitlement is ending. Export your bounded artifacts before suspension; local work continues with no managed dependency.",
            linked_entitlement_state: EntitlementState::EntitlementExpired,
            posture_origin: PostureOrigin::Plan,
            impacted_managed_features: &[
                "All managed lanes — AI gateway, sync, relay, registry, support ingest, and workspace — wind down to the local-safe baseline.",
                "Managed copies are scheduled for deletion after the export window and retention period.",
            ],
            impacted_service_families: &[
                ServiceFamily::AiGatewayFamily,
                ServiceFamily::SyncFamily,
                ServiceFamily::CollaborationRelayFamily,
                ServiceFamily::RegistryOrMirrorMetadataFamily,
                ServiceFamily::TelemetryOrSupportIngestFamily,
                ServiceFamily::RemoteWorkspaceControlPlaneFamily,
            ],
            local_safe_continuation: &[
                "Local editing, search, save, and Git continue with no managed dependency.",
                "Already-authorized local automation keeps running after the plan ends.",
            ],
            export_rights: &[
                GracePeriodRight::ExportBeforeSuspend,
                GracePeriodRight::OffboardingExportAdmissible,
            ],
            export_guarantee: ExportGuarantee::ParityWithCsvAndJson,
            local_artifacts: &[
                "Local files, settings, and Git history stay on device and remain yours after cancellation.",
                "Exported offboarding bundles you save stay on device.",
            ],
            tenant_scoped_managed_state: &[
                "Tenant-scoped managed quota, usage, and synced copies are suspended at the deadline and deleted after retention.",
                "Managed deletion is scheduled and inspectable, not silent.",
            ],
            handoff_owner: HandoffOwner::BillingContact,
            contact_channel: ContactChannel::BillingPortal,
            handoff_instruction: "The billing contact can confirm or reverse the cancellation before the deadline; export first either way.",
            final_usage_disclosure: FinalUsageDisclosure::BoundToUnitAsOfScope,
            grace_window_closes_at: Some(STABLE_EXPORT_UNTIL),
            managed_deletion_at: Some(STABLE_MANAGED_DELETION_AT),
            timeline_note: "Export until the deadline; managed copies are suspended at the deadline and deleted after the retention window. Local data is never deleted.",
            recovery_cue: "Export bounded artifacts before the deadline; local work continues now whether or not the plan is renewed.",
            has_upgrade_action: true,
        },
        CardDef {
            lifecycle_event: LifecycleEvent::OrgSwitch,
            title: "Organization switched",
            summary: "The account or org was switched or transferred. Managed scope is rebinding to the new org; your local artifacts stay local and the prior org's tenant-scoped state stays with that tenant.",
            linked_entitlement_state: EntitlementState::EntitlementPendingRecheck,
            posture_origin: PostureOrigin::Org,
            impacted_managed_features: &[
                "Managed AI gateway, sync, relay, registry, support ingest, and workspace scope rebinds to the new org's entitlement.",
                "The prior org's tenant-scoped managed state stays with that tenant and does not migrate.",
            ],
            impacted_service_families: &[
                ServiceFamily::AiGatewayFamily,
                ServiceFamily::SyncFamily,
                ServiceFamily::CollaborationRelayFamily,
                ServiceFamily::RegistryOrMirrorMetadataFamily,
                ServiceFamily::TelemetryOrSupportIngestFamily,
                ServiceFamily::RemoteWorkspaceControlPlaneFamily,
            ],
            local_safe_continuation: &[
                "Local editing, search, save, and Git continue while managed scope rebinds.",
                "Already-authorized local automation keeps running across the switch.",
            ],
            export_rights: &[
                GracePeriodRight::ExportBeforeSuspend,
                GracePeriodRight::OffboardingExportAdmissible,
            ],
            export_guarantee: ExportGuarantee::ParityWithJsonOnly,
            local_artifacts: &[
                "Local files, settings, and Git history stay on device and remain yours across the switch.",
                "Locally cached snapshots stay available, labeled with their as-of time.",
            ],
            tenant_scoped_managed_state: &[
                "The prior org's tenant-scoped quota, usage, and synced copies stay with that tenant and do not migrate.",
                "The new org's managed scope rebinds on recheck; it does not inherit the prior org's counters.",
            ],
            handoff_owner: HandoffOwner::OrganizationOwner,
            contact_channel: ContactChannel::AdminConsole,
            handoff_instruction: "The organization owner governs the switch and what rebinds; this is an org change, not a seat loss or a sign-in failure.",
            final_usage_disclosure: FinalUsageDisclosure::SuppressedNoManagedNumber,
            grace_window_closes_at: None,
            managed_deletion_at: None,
            timeline_note: "Export the prior org's bounded artifacts before the switch completes; nothing is deleted by the switch, and local data is never deleted.",
            recovery_cue: "Managed scope is rebinding to the new org; export the prior org's artifacts and continue locally now.",
            has_upgrade_action: false,
        },
    ];

    let cards: Vec<OffboardingCard> = defs.iter().map(build_card).collect();

    let all_card_ids: Vec<&str> = cards.iter().map(|c| c.card_id.as_str()).collect();
    let separation_card_ids: Vec<&str> = cards
        .iter()
        .filter(|c| {
            matches!(
                c.lifecycle_event,
                LifecycleEvent::SeatLoss | LifecycleEvent::OrgSwitch
            )
        })
        .map(|c| c.card_id.as_str())
        .collect();

    let surface_bindings = vec![
        binding(
            "offboarding_surface.account",
            ConsumerSurface::AccountSurface,
            &all_card_ids,
            "The account and offboarding surface renders each card's event type, effective date, impacted managed features, export rights, local-safe continuation, deletion timeline, and owner handoff, with export above any renewal prompt.",
        ),
        binding(
            "offboarding_surface.diagnostics",
            ConsumerSurface::Diagnostics,
            &all_card_ids,
            "Diagnostics and service-health surfaces project each card's effective claim and the local-safe continuation without inventing a stronger claim.",
        ),
        binding(
            "offboarding_surface.help_about",
            ConsumerSurface::HelpAbout,
            &all_card_ids,
            "The Help/About truth surface states that a grace window, a seat loss, a cancellation, and an org switch each keep local editing, search, and Git running and never delete local data.",
        ),
        binding(
            "offboarding_surface.support_admin",
            ConsumerSurface::SupportAdminPacket,
            &separation_card_ids,
            "Support and admin export packets carry the seat-loss and org-switch separation of local artifacts from tenant-scoped managed state, the deletion timeline, and the owner handoff.",
        ),
        binding(
            "offboarding_surface.claim_public_truth",
            ConsumerSurface::ClaimPublicTruthAutomation,
            &all_card_ids,
            "Claim and public-truth automation narrows a marketed managed claim to each card's effective claim — managed-narrowed during grace or an org switch, local-safe-only after a seat loss or cancellation.",
        ),
    ];

    let inspection = OffboardingCardInspection::derive(&cards, &surface_bindings);

    let summary = "Frozen grace-period, seat-loss, cancellation, and org-switch offboarding cards \
        for the managed lanes. Each card states the event type, effective date, impacted managed \
        features, export rights, local-safe continuation, deletion timeline, and owner handoff, \
        separates local artifacts from tenant-scoped managed state, keeps export and local \
        continuation above any upgrade or renewal prompt, stays distinct from the other three \
        events and a sign-in failure, and narrows the marketed claim from the lifecycle event's \
        cap without ever deleting or blocking the local core."
        .to_owned();

    CardSet {
        record_kind: CARD_SET_RECORD_KIND.to_owned(),
        schema_version: OFFBOARDING_CARDS_SCHEMA_VERSION,
        set_id,
        generated_at,
        set_revision,
        title,
        summary,
        source_refs: card_source_refs(),
        cards,
        surface_bindings,
        inspection,
    }
}
