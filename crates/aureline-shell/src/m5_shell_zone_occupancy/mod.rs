//! Canonical shell-zone occupancy and declared-slot routing for every claimed M5
//! surface family.
//!
//! The [frozen shell-zone matrix][matrix] already binds each claimed M5 surface
//! family — notebook, data grid, profiler, pipeline, docs, preview, review,
//! incident, companion, and operator — to its canonical shell slot, its declared
//! fallback slot, its dependency-missing placeholder behavior, and its responsive,
//! multi-window, and owning-window routing truth. This lane is the occupancy
//! capstone on top of that matrix: for every governed family it certifies that the
//! live surface **actually occupies a declared shell slot**, that command,
//! keyboard, docs, and onboarding routes all resolve to the same slot and occupant,
//! and that a dependency-missing or policy-blocked occupant degrades into an
//! explicit in-slot placeholder card that preserves spatial continuity rather than
//! collapsing the surrounding layout or inventing a private chrome island.
//!
//! Three records carry the truth:
//!
//! - the per-family **occupancy row** ([`ShellOccupancyRow`]): one row per
//!   [`M5ShellSurfaceFamily`] naming its declared canonical/fallback slot, the
//!   registered slot set it may attach to, the slot it currently occupies, its
//!   slot-attachment / occupant-availability / route-resolution posture, the route
//!   channels that resolve to it, any active waiver, and a derived green/yellow/red
//!   [`ShellOccupancyStatus`].
//! - the release **occupancy packet** ([`ShellOccupancyPacket`]): the full set of
//!   rows with derived per-row status, aggregate green/yellow/red counts, the active
//!   waivers, the exact occupancy causes ([`ShellOccupancyCause`]), and the blocking
//!   findings the lane refuses to ship with.
//! - the **occupancy dashboard** ([`ShellOccupancyDashboard`]): a light projection
//!   the shell / windowing / layout / release automation reads to auto-narrow a
//!   claimed surface when its occupancy proof falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to
//! `yellow` the moment its frozen qualification is below Stable, it degrades into a
//! disclosed dependency-missing/policy-blocked placeholder, or one of its routes
//! falls back to a disclosed, waivered alternative; it drops to `red` if it attaches
//! outside any declared shell slot (a private chrome island), its placeholder
//! collapses the surrounding layout or loses the surface identity, a route resolves
//! to a different slot or occupant than the declared owner, or its occupied slot is
//! not in the family's registered slot set. That derivation is the auto-narrowing
//! the acceptance criteria require, and the registered-slot check is the lint that
//! prevents a later unregistered shell attachment from shipping as stable.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw
//! URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials —
//! only stable ids, closed vocabulary, counts, refs, and short labels. The surface
//! family, shell-zone slot, qualification, downgrade-trigger, consumer-surface, and
//! placeholder-behavior vocabulary is re-exported by reference from the already
//! frozen [matrix]; the certified rows are pulled straight from that matrix's seeded
//! packet, so this lane mints no parallel shell vocabulary and cannot certify a
//! family the matrix does not freeze. Only the occupancy-specific vocabulary
//! ([`ShellOccupancyStatus`], [`SlotAttachmentState`], [`OccupantAvailabilityState`],
//! [`RouteResolutionState`], [`RouteChannel`], [`ShellOccupancyWaiver`],
//! [`ShellOccupancyCause`], [`ShellOccupancyFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix as matrix;

pub use matrix::{
    M5PlaceholderBehavior, M5ShellConsumerSurface, M5ShellDowngradeTrigger,
    M5ShellQualificationClass, M5ShellSurfaceFamily, M5ShellZoneSlot,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_shell_occupancy_packet, seeded_m5_shell_occupancy_packet_notebook_undeclared_blocked,
    seeded_m5_shell_occupancy_packet_review_route_conflict_blocked,
    seeded_m5_shell_occupancy_packet_data_grid_placeholder_collapsed_blocked, SEED_BUILD_IDENTITY_REF,
    SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_SHELL_OCCUPANCY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_SHELL_OCCUPANCY_SHARED_CONTRACT_REF: &str = "shell:m5_shell_zone_occupancy:v1";

/// Stable record kind for [`ShellOccupancyPacket`] payloads.
pub const M5_SHELL_OCCUPANCY_PACKET_RECORD_KIND: &str = "shell_m5_shell_zone_occupancy_packet_record";

/// Stable record kind for [`ShellOccupancyDashboard`] payloads.
pub const M5_SHELL_OCCUPANCY_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_shell_zone_occupancy_dashboard_record";

/// Stable record kind for [`ShellOccupancySupportExport`] payloads.
pub const M5_SHELL_OCCUPANCY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_shell_zone_occupancy_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_SHELL_OCCUPANCY_PACKET_ID: &str = "m5-shell-zone-occupancy:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_SHELL_OCCUPANCY_DASHBOARD_ID: &str = "m5-shell-zone-occupancy-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_SHELL_OCCUPANCY_SUPPORT_EXPORT_ID: &str = "support-export:m5-shell-zone-occupancy:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_SHELL_OCCUPANCY_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-shell-zone-occupancy.schema.json";

/// Published markdown report ref reviewers reopen the occupancy proof from.
pub const M5_SHELL_OCCUPANCY_PUBLISHED_REPORT_REF: &str = "artifacts/shell/m5-shell-zone-occupancy.md";

/// Published occupancy-packet artifact ref.
pub const M5_SHELL_OCCUPANCY_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-shell-occupancy-proof/packet.json";

/// Published occupancy-dashboard artifact ref.
pub const M5_SHELL_OCCUPANCY_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-shell-occupancy-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_SHELL_OCCUPANCY_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-shell-occupancy-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_SHELL_OCCUPANCY_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-shell-occupancy-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_SHELL_OCCUPANCY_PUBLISHED_DOC_REF: &str =
    "docs/shell/m5_shell_zone_occupancy_contract.md";

/// Repo-relative ref to the frozen shell-zone matrix schema.
pub const M5_SHELL_OCCUPANCY_MATRIX_SCHEMA_REF: &str = matrix::M5_SHELL_ZONE_MATRIX_SCHEMA_REF;

/// Every governed surface family the occupancy proof must cover, in canonical
/// order. These are exactly the families the frozen shell-zone matrix freezes; the
/// lane certifies none beyond them and refuses to ship if any is missing.
pub const REQUIRED_FAMILIES: [M5ShellSurfaceFamily; 10] = M5ShellSurfaceFamily::ALL;

/// The derived occupancy light a governed surface family carries.
///
/// `green` means the family occupies a declared slot, its occupant is available,
/// and every route resolves to that slot and occupant. `yellow` is a disclosed
/// narrowing (the family is honestly narrowed below Stable, degrades into a
/// disclosed dependency-missing/policy-blocked placeholder, or a route falls back to
/// a disclosed, waivered alternative). `red` is blocked: the family attaches outside
/// any declared slot, its placeholder collapses the surrounding layout or loses its
/// identity, a route resolves to a different slot/occupant, or its occupied slot is
/// not registered — and it may not keep a shell-maturity claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellOccupancyStatus {
    /// Full standing: declared slot, available occupant, all routes resolve.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl ShellOccupancyStatus {
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

/// How the live occupant attaches to the shell.
///
/// `attached_to_declared_slot` means the occupant docks into a slot in the family's
/// registered slot set (its canonical or declared fallback slot). `undisclosed_slot`
/// means the surface attached outside any declared slot — a private chrome island —
/// always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotAttachmentState {
    /// The occupant docks into a declared (registered) shell slot.
    AttachedToDeclaredSlot,
    /// The surface attached outside any declared slot — a private chrome island.
    UndeclaredSlotAttachment,
}

impl SlotAttachmentState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AttachedToDeclaredSlot => "attached_to_declared_slot",
            Self::UndeclaredSlotAttachment => "undeclared_slot_attachment",
        }
    }

    /// `true` when the occupant attaches to a declared slot.
    pub const fn is_declared(self) -> bool {
        matches!(self, Self::AttachedToDeclaredSlot)
    }
}

/// The availability posture of the occupant in its declared slot.
///
/// `occupant_available` means the surface renders its live content. The two
/// placeholder states are disclosed degradations that keep the slot occupied by an
/// explicit placeholder card preserving spatial continuity. `placeholder_collapsed`
/// means the placeholder collapsed the surrounding layout or lost the surface
/// identity/reopen path — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OccupantAvailabilityState {
    /// The surface renders its live content in the declared slot.
    OccupantAvailable,
    /// A missing dependency degrades to a disclosed in-slot placeholder card.
    DependencyMissingPlaceholder,
    /// A policy block degrades to a disclosed in-slot placeholder card.
    PolicyBlockedPlaceholder,
    /// The placeholder collapsed the surrounding layout or lost the identity.
    PlaceholderCollapsedLayout,
}

impl OccupantAvailabilityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OccupantAvailable => "occupant_available",
            Self::DependencyMissingPlaceholder => "dependency_missing_placeholder",
            Self::PolicyBlockedPlaceholder => "policy_blocked_placeholder",
            Self::PlaceholderCollapsedLayout => "placeholder_collapsed_layout",
        }
    }

    /// `true` when the occupant renders live content.
    pub const fn is_available(self) -> bool {
        matches!(self, Self::OccupantAvailable)
    }

    /// `true` when the occupant shows a continuity-preserving placeholder card.
    pub const fn is_disclosed_placeholder(self) -> bool {
        matches!(
            self,
            Self::DependencyMissingPlaceholder | Self::PolicyBlockedPlaceholder
        )
    }
}

/// How command, keyboard, docs, and onboarding routes resolve to the family.
///
/// `all_routes_resolve` means every route lands on the same declared slot and
/// occupant. `disclosed_route_fallback` means one route resolves to a disclosed,
/// waivered alternative. `conflicting_route_resolution` means a route resolves to a
/// different slot or occupant than the declared owner — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteResolutionState {
    /// Every route resolves to the same declared slot and occupant.
    AllRoutesResolveToSlotOccupant,
    /// One route resolves to a disclosed, waivered alternative.
    DisclosedRouteFallback,
    /// A route resolves to a different slot or occupant than the declared owner.
    ConflictingRouteResolution,
}

impl RouteResolutionState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllRoutesResolveToSlotOccupant => "all_routes_resolve_to_slot_occupant",
            Self::DisclosedRouteFallback => "disclosed_route_fallback",
            Self::ConflictingRouteResolution => "conflicting_route_resolution",
        }
    }

    /// `true` when every route resolves to the declared slot and occupant.
    pub const fn is_fully_resolved(self) -> bool {
        matches!(self, Self::AllRoutesResolveToSlotOccupant)
    }
}

/// One of the four route channels that must resolve to the same slot and occupant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteChannel {
    /// Command palette / command registry route.
    Command,
    /// Keyboard / keybinding route.
    Keyboard,
    /// Docs / help route.
    Docs,
    /// Onboarding / first-run route.
    Onboarding,
}

impl RouteChannel {
    /// Every route channel, in declaration order.
    pub const ALL: [Self; 4] = [Self::Command, Self::Keyboard, Self::Docs, Self::Onboarding];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Command => "command",
            Self::Keyboard => "keyboard",
            Self::Docs => "docs",
            Self::Onboarding => "onboarding",
        }
    }
}

/// Short, reviewer-facing label for a governed family's occupant surface.
pub const fn occupant_label(family: M5ShellSurfaceFamily) -> &'static str {
    match family {
        M5ShellSurfaceFamily::Notebook => "Notebook editor / cell surface",
        M5ShellSurfaceFamily::DataGrid => "Tabular data grid surface",
        M5ShellSurfaceFamily::Profiler => "Profiler / performance surface",
        M5ShellSurfaceFamily::Pipeline => "Pipeline / workflow graph surface",
        M5ShellSurfaceFamily::Docs => "Documentation reader surface",
        M5ShellSurfaceFamily::Preview => "Preview surface (render, diff, media)",
        M5ShellSurfaceFamily::Review => "Review / change-request surface",
        M5ShellSurfaceFamily::Incident => "Incident / operations-response surface",
        M5ShellSurfaceFamily::Companion => "Companion assistant surface",
        M5ShellSurfaceFamily::Operator => "Operator / control-plane surface",
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red route posture stay
/// narrowed (yellow) rather than blocked — never lets an undeclared attachment, a
/// collapsed placeholder, or a conflicting route hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellOccupancyWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The governed family the waiver applies to.
    pub family: M5ShellSurfaceFamily,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row
    /// blocks.
    pub expires_at: String,
}

impl ShellOccupancyWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed family's occupancy claim.
///
/// The trigger token mirrors the frozen [`M5ShellDowngradeTrigger`] vocabulary so a
/// cause never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellOccupancyCause {
    /// The governed family the cause applies to.
    pub family: M5ShellSurfaceFamily,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5ShellDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a
    /// non-disclosed cause is a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl ShellOccupancyCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed surface family, certified across its slot attachment, occupant
/// availability, and route-resolution posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellOccupancyRow {
    /// The governed family being certified.
    pub family: M5ShellSurfaceFamily,
    /// The family's frozen qualification class from the shell-zone matrix.
    pub matrix_qualification: M5ShellQualificationClass,
    /// Owner role accountable for keeping this family governed.
    pub owner_role: String,
    /// Short occupant-surface label.
    pub occupant_surface: String,
    /// Canonical shell slot from the matrix.
    pub canonical_slot: M5ShellZoneSlot,
    /// Declared fallback slot from the matrix.
    pub fallback_slot: M5ShellZoneSlot,
    /// The registered slot set this family may attach to (canonical + fallback).
    pub registered_slots: Vec<M5ShellZoneSlot>,
    /// The shell slot the live occupant currently docks into.
    pub occupied_slot: M5ShellZoneSlot,
    /// Dependency-missing placeholder behavior from the matrix.
    pub placeholder_behavior: M5PlaceholderBehavior,
    /// Slot-attachment posture.
    pub slot_attachment: SlotAttachmentState,
    /// Occupant-availability posture.
    pub occupant_availability: OccupantAvailabilityState,
    /// Route-resolution posture.
    pub route_resolution: RouteResolutionState,
    /// Route channels that resolve to the declared slot and occupant.
    pub resolved_route_channels: Vec<RouteChannel>,
    /// Consumer surfaces this family must stay aligned across. Pulled from the
    /// matrix.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this family. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5ShellDowngradeTrigger>,
    /// Active waiver, when a disclosed route narrowing is in force.
    pub active_waiver: Option<ShellOccupancyWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: ShellOccupancyStatus,
    /// The exact occupancy causes that narrowed or blocked this row.
    pub occupancy_causes: Vec<ShellOccupancyCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl ShellOccupancyRow {
    /// `true` when this family's frozen qualification keeps a Stable claim.
    pub fn is_stable_qualified(&self) -> bool {
        self.matrix_qualification.is_stable()
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the occupied slot is in the registered slot set — the lint that
    /// prevents an unregistered shell attachment from shipping as stable.
    pub fn occupied_slot_is_registered(&self) -> bool {
        self.registered_slots.contains(&self.occupied_slot)
    }

    /// `true` when the family's canonical slot is in its registered slot set.
    pub fn registered_declares_canonical(&self) -> bool {
        self.registered_slots.contains(&self.canonical_slot)
    }

    /// `true` when every route channel resolves to the declared slot and occupant.
    fn resolves_every_route_channel(&self) -> bool {
        RouteChannel::ALL
            .iter()
            .all(|channel| self.resolved_route_channels.contains(channel))
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        // Attaching outside a declared slot is a private chrome island.
        if matches!(
            self.slot_attachment,
            SlotAttachmentState::UndeclaredSlotAttachment
        ) {
            return true;
        }
        // The occupied slot must be one the family registered.
        if !self.occupied_slot_is_registered() {
            return true;
        }
        // The registry must declare the canonical slot.
        if !self.registered_declares_canonical() {
            return true;
        }
        // A placeholder that collapses layout or loses identity always blocks.
        if matches!(
            self.occupant_availability,
            OccupantAvailabilityState::PlaceholderCollapsedLayout
        ) {
            return true;
        }
        // A route that resolves to a different slot/occupant always blocks.
        if matches!(
            self.route_resolution,
            RouteResolutionState::ConflictingRouteResolution
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        !self.is_stable_qualified()
            || self.occupant_availability.is_disclosed_placeholder()
            || matches!(
                self.route_resolution,
                RouteResolutionState::DisclosedRouteFallback
            )
    }

    /// Recomputes the derived status from the attachment, occupant, and route
    /// posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest
    /// narrowing forces `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> ShellOccupancyStatus {
        if self.has_hard_blocker() {
            ShellOccupancyStatus::Red
        } else if self.has_narrowing() {
            ShellOccupancyStatus::Yellow
        } else {
            ShellOccupancyStatus::Green
        }
    }

    /// Recomputes the exact occupancy causes for the row, in deterministic order
    /// (qualification, slot attachment, occupant availability, route resolution).
    pub fn recompute_causes(&self) -> Vec<ShellOccupancyCause> {
        let mut causes = Vec::new();
        if !self.is_stable_qualified() {
            causes.push(ShellOccupancyCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                disclosed: true,
                detail: format!(
                    "Frozen shell-zone matrix qualifies this family at `{}`, below a Stable shell claim.",
                    self.matrix_qualification.as_str()
                ),
            });
        }
        match self.slot_attachment {
            SlotAttachmentState::AttachedToDeclaredSlot => {
                if !self.occupied_slot_is_registered() {
                    causes.push(ShellOccupancyCause {
                        family: self.family,
                        trigger: M5ShellDowngradeTrigger::SlotUndeclared,
                        disclosed: false,
                        detail: format!(
                            "Occupied slot `{}` is not in the family's registered slot set.",
                            self.occupied_slot.as_str()
                        ),
                    });
                }
            }
            SlotAttachmentState::UndeclaredSlotAttachment => causes.push(ShellOccupancyCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::SlotUndeclared,
                disclosed: false,
                detail: "Surface attached outside any declared shell slot (a private chrome island)."
                    .to_owned(),
            }),
        }
        match self.occupant_availability {
            OccupantAvailabilityState::OccupantAvailable => {}
            OccupantAvailabilityState::DependencyMissingPlaceholder => {
                causes.push(ShellOccupancyCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "A missing dependency degrades the occupant to a disclosed in-slot \
                             placeholder card that preserves spatial continuity."
                        .to_owned(),
                });
            }
            OccupantAvailabilityState::PolicyBlockedPlaceholder => causes.push(ShellOccupancyCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::PolicyBlocked,
                disclosed: true,
                detail: "A policy block degrades the occupant to a disclosed in-slot placeholder \
                         card that preserves spatial continuity."
                    .to_owned(),
            }),
            OccupantAvailabilityState::PlaceholderCollapsedLayout => {
                causes.push(ShellOccupancyCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::PlaceholderLostIdentityOrReopen,
                    disclosed: false,
                    detail: "The placeholder collapsed the surrounding layout or lost the surface \
                             identity and reopen path."
                        .to_owned(),
                });
            }
        }
        match self.route_resolution {
            RouteResolutionState::AllRoutesResolveToSlotOccupant => {}
            RouteResolutionState::DisclosedRouteFallback => causes.push(ShellOccupancyCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::OwningWindowRoutingLost,
                disclosed: true,
                detail: "A command/keyboard/docs/onboarding route resolves to a disclosed, \
                         waivered alternative slot for the same occupant."
                    .to_owned(),
            }),
            RouteResolutionState::ConflictingRouteResolution => causes.push(ShellOccupancyCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::OwningWindowRoutingLost,
                disclosed: false,
                detail: "A route resolves to a different slot or occupant than the declared owner."
                    .to_owned(),
            }),
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay
    /// publishable.
    ///
    /// A disclosed route fallback may only stay yellow (rather than red) when a
    /// waiver discloses it.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.route_resolution,
            RouteResolutionState::DisclosedRouteFallback
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<ShellOccupancyFinding> {
        let mut findings = Vec::new();
        let family = self.family.as_str().to_owned();

        if matches!(
            self.slot_attachment,
            SlotAttachmentState::UndeclaredSlotAttachment
        ) {
            findings.push(ShellOccupancyFinding::UndeclaredSlotAttachment {
                family: family.clone(),
            });
        }
        if !self.occupied_slot_is_registered() {
            findings.push(ShellOccupancyFinding::OccupiedSlotNotRegistered {
                family: family.clone(),
                slot: self.occupied_slot.as_str().to_owned(),
            });
        }
        if !self.registered_declares_canonical() {
            findings.push(ShellOccupancyFinding::CanonicalSlotNotRegistered {
                family: family.clone(),
                slot: self.canonical_slot.as_str().to_owned(),
            });
        }
        if matches!(
            self.occupant_availability,
            OccupantAvailabilityState::PlaceholderCollapsedLayout
        ) {
            findings.push(ShellOccupancyFinding::PlaceholderCollapsedLayout {
                family: family.clone(),
            });
        }
        if matches!(
            self.route_resolution,
            RouteResolutionState::ConflictingRouteResolution
        ) {
            findings.push(ShellOccupancyFinding::ConflictingRouteResolution {
                family: family.clone(),
            });
        }
        // A row that asserts every route resolves must list every channel.
        if matches!(
            self.route_resolution,
            RouteResolutionState::AllRoutesResolveToSlotOccupant
        ) && !self.resolves_every_route_channel()
        {
            findings.push(ShellOccupancyFinding::RouteChannelsIncomplete {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, ShellOccupancyStatus::Green) && !self.has_reason() {
            findings.push(ShellOccupancyFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry
        // an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(ShellOccupancyFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.family != self.family {
                findings.push(ShellOccupancyFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(ShellOccupancyFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(ShellOccupancyFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.occupancy_causes != self.recompute_causes() {
            findings.push(ShellOccupancyFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} slot={} attach={} occupant={} route={} waiver={}",
            self.family.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.occupied_slot.as_str(),
            self.slot_attachment.as_str(),
            self.occupant_availability.as_str(),
            self.route_resolution.as_str(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the shell-zone occupancy proof refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum ShellOccupancyFinding {
    /// A governed surface family has no occupancy row.
    FamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A row attached outside any declared shell slot.
    UndeclaredSlotAttachment {
        /// The family token.
        family: String,
    },
    /// A row's occupied slot is not in the family's registered slot set.
    OccupiedSlotNotRegistered {
        /// The family token.
        family: String,
        /// The unregistered slot token.
        slot: String,
    },
    /// A family's canonical slot is not in its registered slot set.
    CanonicalSlotNotRegistered {
        /// The family token.
        family: String,
        /// The missing canonical slot token.
        slot: String,
    },
    /// A row's placeholder collapsed the surrounding layout or lost identity.
    PlaceholderCollapsedLayout {
        /// The family token.
        family: String,
    },
    /// A route resolves to a different slot or occupant than the declared owner.
    ConflictingRouteResolution {
        /// The family token.
        family: String,
    },
    /// A row asserting every route resolves does not list every route channel.
    RouteChannelsIncomplete {
        /// The family token.
        family: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The family token.
        family: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The family token.
        family: String,
    },
    /// An attached waiver does not point at the row's family.
    WaiverFamilyMismatch {
        /// The family token.
        family: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The family token.
        family: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The family token.
        family: String,
    },
    /// The declared occupancy causes do not match the recomputed causes.
    RowCausesStale {
        /// The family token.
        family: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl ShellOccupancyFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::FamilyMissing { .. } => "family_missing",
            Self::UndeclaredSlotAttachment { .. } => "undeclared_slot_attachment",
            Self::OccupiedSlotNotRegistered { .. } => "occupied_slot_not_registered",
            Self::CanonicalSlotNotRegistered { .. } => "canonical_slot_not_registered",
            Self::PlaceholderCollapsedLayout { .. } => "placeholder_collapsed_layout",
            Self::ConflictingRouteResolution { .. } => "conflicting_route_resolution",
            Self::RouteChannelsIncomplete { .. } => "route_channels_incomplete",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverFamilyMismatch { .. } => "waiver_family_mismatch",
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
            Self::FamilyMissing { family }
            | Self::UndeclaredSlotAttachment { family }
            | Self::OccupiedSlotNotRegistered { family, .. }
            | Self::CanonicalSlotNotRegistered { family, .. }
            | Self::PlaceholderCollapsedLayout { family }
            | Self::ConflictingRouteResolution { family }
            | Self::RouteChannelsIncomplete { family }
            | Self::NarrowedRowWithoutReason { family }
            | Self::NarrowedRowWithoutWaiver { family }
            | Self::WaiverFamilyMismatch { family, .. }
            | Self::WaiverExpired { family, .. }
            | Self::RowStatusStale { family }
            | Self::RowCausesStale { family } => family,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release occupancy packet shared by the shell / windowing / layout / release
/// automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellOccupancyPacket {
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
    /// The frozen shell-zone matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen shell-zone matrix schema.
    pub matrix_schema_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// Per-family occupancy rows, in canonical order.
    pub rows: Vec<ShellOccupancyRow>,
    /// Governed families certified, in canonical (sorted) order.
    pub covered_families: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (fully occupied) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<ShellOccupancyWaiver>,
    /// Every exact occupancy cause, in row then cause order.
    pub occupancy_causes: Vec<ShellOccupancyCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<ShellOccupancyFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Shell / release automation refs that consume this packet to auto-narrow
    /// claimed surfaces.
    pub shell_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published occupancy-packet ref.
    pub published_packet_ref: String,
    /// Published occupancy-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl ShellOccupancyPacket {
    /// Returns the occupancy row for `family`, if present.
    pub fn row(&self, family: M5ShellSurfaceFamily) -> Option<&ShellOccupancyRow> {
        self.rows.iter().find(|row| row.family == family)
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
                waiver.family.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.occupancy_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.family.as_str(),
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

    /// Projects the light occupancy dashboard the shell automation consumes.
    pub fn dashboard(&self) -> ShellOccupancyDashboard {
        ShellOccupancyDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 shell-zone occupancy packet serializes")
    }

    /// Deterministic, machine-readable occupancy CSV: one row per family naming its
    /// status, qualification, slots, attachment, occupant, route, and waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "family,status,qualification,canonical_slot,fallback_slot,occupied_slot,registered_slots,slot_attachment,occupant_availability,route_resolution,resolved_route_channels,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.family.as_str(),
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.canonical_slot.as_str(),
                row.fallback_slot.as_str(),
                row.occupied_slot.as_str(),
                join_tokens(&row.registered_slots, |s| s.as_str()),
                row.slot_attachment.as_str(),
                row.occupant_availability.as_str(),
                row.route_resolution.as_str(),
                join_tokens(&row.resolved_route_channels, |c| c.as_str()),
                row.active_waiver
                    .as_ref()
                    .map(|w| w.waiver_id.as_str())
                    .unwrap_or("none"),
            ));
        }
        out
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 shell-zone occupancy & declared-slot routing\n\n");
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_shell_zone_occupancy`](../../crates/aureline-shell/src/m5_shell_zone_occupancy/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_occupancy -- markdown > \\\n  artifacts/shell/m5-shell-zone-occupancy.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!("- Source schema ref: `{}`\n", self.source_schema_ref));
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
        out.push_str(&format!("- Green (fully occupied): {}\n", self.green_row_count));
        out.push_str(&format!("- Yellow (auto-narrowed): {}\n", self.yellow_row_count));
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
            if self.report_clean { "clean" } else { "blocked" }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Occupancy rows\n\n");
        out.push_str(
            "| Occupant surface | Status | Qualification | Occupied slot | Attachment | Occupant | Route | Waiver |\n\
             | ---------------- | ------ | ------------- | ------------- | ---------- | -------- | ----- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.occupant_surface,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.occupied_slot.as_str(),
                row.slot_attachment.as_str(),
                row.occupant_availability.as_str(),
                row.route_resolution.as_str(),
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&ShellOccupancyRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, ShellOccupancyStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str("None — every governed family occupies a declared slot at full standing.\n\n");
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.family.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact occupancy causes\n\n");
        if self.occupancy_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.occupancy_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.family.as_str(),
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
                    waiver.family.as_str(),
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_occupancy -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_shell_zone_occupancy_fixtures\n");
        out.push_str("```\n");
        out
    }
}

/// One row of the light occupancy dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellOccupancyDashboardRow {
    /// The governed family.
    pub family: M5ShellSurfaceFamily,
    /// Short occupant-surface label.
    pub occupant_surface: String,
    /// Derived green/yellow/red status.
    pub status: ShellOccupancyStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5ShellQualificationClass,
    /// The slot the occupant currently docks into.
    pub occupied_slot: M5ShellZoneSlot,
    /// Slot-attachment posture.
    pub slot_attachment: SlotAttachmentState,
    /// Occupant-availability posture.
    pub occupant_availability: OccupantAvailabilityState,
    /// Route-resolution posture.
    pub route_resolution: RouteResolutionState,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light occupancy dashboard the shell / windowing / layout / release
/// automation reads to auto-narrow claimed surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellOccupancyDashboard {
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
    pub rows: Vec<ShellOccupancyDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Shell / release automation refs that consume the dashboard.
    pub shell_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl ShellOccupancyDashboard {
    /// Projects the dashboard from an occupancy packet.
    pub fn from_packet(packet: &ShellOccupancyPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| ShellOccupancyDashboardRow {
                family: row.family,
                occupant_surface: row.occupant_surface.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                occupied_slot: row.occupied_slot,
                slot_attachment: row.slot_attachment,
                occupant_availability: row.occupant_availability,
                route_resolution: row.route_resolution,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .occupancy_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_SHELL_OCCUPANCY_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_SHELL_OCCUPANCY_SCHEMA_VERSION,
            dashboard_id: M5_SHELL_OCCUPANCY_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            shell_automation_refs: packet.shell_automation_refs.clone(),
            generated_at: packet.generated_at.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 shell-zone occupancy dashboard serializes")
    }
}

/// Support-export wrapper for the shell-zone occupancy packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellOccupancySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: ShellOccupancyPacket,
    /// Dashboard quoted in full.
    pub dashboard: ShellOccupancyDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl ShellOccupancySupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each family, each
    /// occupied slot, and each active waiver id is quoted as a case id so a support
    /// reviewer — or the shell automation — can name the same family, slot, and
    /// waiver the runtime certified.
    pub fn from_packet(support_export_id: impl Into<String>, packet: ShellOccupancyPacket) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.family.as_str().to_owned());
            case_ids.push(row.occupied_slot.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_SHELL_OCCUPANCY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_SHELL_OCCUPANCY_SCHEMA_VERSION,
            shared_contract_ref: M5_SHELL_OCCUPANCY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_shell_occupancy_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellOccupancyInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen shell-zone matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family occupancy rows.
    pub rows: Vec<ShellOccupancyRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The occupancy packet carries only closed vocabulary, refs, and short labels, so
/// raw URLs, credentials, or tokens must never appear.
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

/// Builds a [`ShellOccupancyPacket`] from the exact build identity, the frozen
/// matrix ref, and the per-family occupancy rows.
///
/// Each row's derived status and occupancy causes, the aggregate counts, the active
/// waivers, and the blocking findings are recomputed here so the packet is the
/// single source of truth and the auto-narrowing cannot be asserted.
pub fn build_m5_shell_occupancy_packet(input: ShellOccupancyInput) -> ShellOccupancyPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is
    // self-consistent and the auto-narrowing is the single source of truth.
    let rows: Vec<ShellOccupancyRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.occupancy_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<ShellOccupancyFinding> = Vec::new();

    // Every governed family must carry an occupancy row.
    let present: BTreeSet<M5ShellSurfaceFamily> = rows.iter().map(|row| row.family).collect();
    for family in REQUIRED_FAMILIES {
        if !present.contains(&family) {
            blocking_findings.push(ShellOccupancyFinding::FamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_families: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ShellOccupancyStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ShellOccupancyStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ShellOccupancyStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(ShellOccupancyFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<ShellOccupancyWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let occupancy_causes: Vec<ShellOccupancyCause> = rows
        .iter()
        .flat_map(|row| row.occupancy_causes.clone())
        .collect();

    let mut packet = ShellOccupancyPacket {
        record_kind: M5_SHELL_OCCUPANCY_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_SHELL_OCCUPANCY_SCHEMA_VERSION,
        shared_contract_ref: M5_SHELL_OCCUPANCY_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_SHELL_OCCUPANCY_PACKET_ID.to_owned(),
        source_schema_ref: M5_SHELL_OCCUPANCY_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Canonical shell-zone occupancy and declared-slot routing for every claimed M5 \
                   surface family: notebook, data grid, profiler, pipeline, docs, preview, review, \
                   incident, companion, and operator each certified to occupy a declared shell slot \
                   with command/keyboard/docs/onboarding routes resolving to the same slot and \
                   occupant, with each row's green/yellow/red claim auto-narrowed from its \
                   slot-attachment, occupant-availability, and route-resolution posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_SHELL_OCCUPANCY_MATRIX_SCHEMA_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        rows,
        covered_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        occupancy_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        shell_automation_refs: vec![
            "shell_frame.occupancy_registry".to_owned(),
            "release_automation.auto_narrow.shell_occupancy_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.shell_zone_occupancy".to_owned(),
            "artifacts/release/m5-shell-occupancy-proof/packet.json".to_owned(),
        ],
        help_docs_refs: vec![M5_SHELL_OCCUPANCY_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-shell-zone-occupancy".to_owned()],
        published_report_ref: M5_SHELL_OCCUPANCY_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_SHELL_OCCUPANCY_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_SHELL_OCCUPANCY_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_SHELL_OCCUPANCY_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("occupancy packet serializes"),
    ) {
        blocking_findings.push(ShellOccupancyFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_shell_occupancy_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum ShellOccupancyValidationError {
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
    /// The rows do not cover all ten governed families.
    CoverageIncomplete,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared occupancy causes do not match the recomputed causes.
    OccupancyCausesStale,
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

/// Validates a packet against the shell-zone occupancy invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed
/// family carries a current occupancy row; each row's status is the derived
/// auto-narrowed value, never asserted; a green row cannot keep a claim while it
/// attaches outside a declared slot, its occupied slot is unregistered, its
/// placeholder collapses layout, or a route conflicts; and a disclosed narrowing is
/// backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_shell_occupancy_packet(
    packet: &ShellOccupancyPacket,
) -> Result<(), Vec<ShellOccupancyValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(ShellOccupancyValidationError::NoRows);
    }
    if packet.record_kind != M5_SHELL_OCCUPANCY_PACKET_RECORD_KIND {
        errors.push(ShellOccupancyValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_SHELL_OCCUPANCY_SCHEMA_VERSION {
        errors.push(ShellOccupancyValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(ShellOccupancyValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(ShellOccupancyValidationError::MatrixPacketRefMissing);
    }

    let present: BTreeSet<M5ShellSurfaceFamily> = packet.rows.iter().map(|row| row.family).collect();
    let coverage_complete = REQUIRED_FAMILIES.iter().all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != REQUIRED_FAMILIES.len() {
        errors.push(ShellOccupancyValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_families {
        errors.push(ShellOccupancyValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ShellOccupancyStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ShellOccupancyStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ShellOccupancyStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(ShellOccupancyValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<ShellOccupancyWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(ShellOccupancyValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<ShellOccupancyCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.occupancy_causes {
        errors.push(ShellOccupancyValidationError::OccupancyCausesStale);
    }

    let mut recomputed: Vec<ShellOccupancyFinding> = Vec::new();
    for family in REQUIRED_FAMILIES {
        if !present.contains(&family) {
            recomputed.push(ShellOccupancyFinding::FamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(ShellOccupancyFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("occupancy packet serializes"),
    ) {
        recomputed.push(ShellOccupancyFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(ShellOccupancyValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(ShellOccupancyValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(ShellOccupancyValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(ShellOccupancyValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(ShellOccupancyValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(ShellOccupancyValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
