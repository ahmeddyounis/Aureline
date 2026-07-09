//! Incident-snapshot cards and desktop-handoff sheets carrying service/run identity,
//! severity, latest status, freshness, permitted bounded actions, an explicit
//! companion-versus-desktop capability boundary, an exact desktop target, and an auth or
//! tenant reminder where relevant.
//!
//! This module narrows the last two components frozen in
//! [`crate::freeze_the_m5_companion_component_matrix`] — the `incident_snapshot_card` and the
//! `desktop_handoff_sheet` — into one implemented, export-safe packet with two co-equal
//! control vectors. Together they preserve exact incident and escalation context when the
//! task exceeds companion scope: a user never has to infer which service or run an incident
//! belongs to, whether the companion can actually remediate, or what exactly a handoff will
//! open on desktop.
//!
//! An [`IncidentSnapshotCard`] always names its service/source class, its stable service and
//! run identity, the object it references, its client scope, its severity, its latest status,
//! and its freshness. Its awareness class is *derived* from the incident status rather than
//! asserted: a stale incident status can never read as a live incident, and the card stays
//! awareness-only — it never overpromises remediation depth by implying the companion can
//! resolve an incident inline. It always offers a keyboard-complete `Open` verb and a bounded
//! `Acknowledge` verb; any widening verb names one exact desktop-handoff target.
//!
//! A [`DesktopHandoffSheet`] always names its target object, its stable target identity, its
//! client scope, exactly what opens on desktop, and — where relevant — an auth or tenant
//! reminder. Its open class is *derived* from the frozen handoff target rather than asserted:
//! a sheet with no resolvable target degrades to an explicit not-openable state instead of
//! implying a desktop client will open the intended object, and it never offers an ambiguous
//! open-on-desktop into a target it cannot resolve. When the desktop session is in a different
//! tenant, needs re-auth, or is signed into a different account, the sheet carries an explicit
//! reminder so the desktop client opens the intended object without user archaeology.
//!
//! The object kinds ([`M5CompanionObjectKind`]), client scopes
//! ([`M5CompanionClientScope`]), freshness classes ([`M5CompanionFreshness`]), severities
//! ([`M5CompanionSeverity`]), handoff targets ([`M5CompanionHandoffTarget`]), degraded reasons
//! ([`M5CompanionDegradedReason`]), required labels ([`M5CompanionRequiredLabel`]), surface
//! families ([`M5CompanionSurfaceFamily`]), deployment lines
//! ([`M5CompanionDeploymentLine`]), consumer surfaces ([`M5CompanionConsumerSurface`]),
//! accessibility routes ([`M5CompanionAccessibilityRoute`]), and downgrade triggers
//! ([`M5CompanionDowngradeTrigger`]) are reused directly from the frozen matrix, so this lane
//! never invents a parallel companion vocabulary. It mints new vocabulary only for what that
//! matrix left implicit about these two controls: the incident service/source class, the
//! incident status, the derived incident awareness class, the keyboard-complete incident
//! verbs, the derived handoff open class, the handoff auth/tenant context, and the
//! keyboard-complete desktop-handoff verbs.
//!
//! Raw incident payloads, log bodies, secret values, and private endpoints stay outside the
//! support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-incident-snapshot-card-desktop-handoff-sheet-controls.schema.json`](../../../../schemas/ui/m5-incident-snapshot-card-desktop-handoff-sheet-controls.schema.json).
//! The contract doc is
//! [`docs/companion/implement_incident_snapshot_cards_and_desktop_handoff_sheets.md`](../../../../docs/companion/implement_incident_snapshot_cards_and_desktop_handoff_sheets.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_incident_snapshot_card_desktop_handoff_sheet_controls,
    seeded_incident_snapshot_card_desktop_handoff_sheet_controls_desktop_handoff_sheet_not_openable,
    seeded_incident_snapshot_card_desktop_handoff_sheet_controls_incident_snapshot_card_stale,
    INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The object kind, client scope, freshness, severity, handoff target, degraded reason,
// required labels, surface family, deployment line, consumer surface, accessibility route, and
// downgrade triggers are frozen once, in the companion component matrix. This lane reuses them
// verbatim so it never invents a parallel companion vocabulary.
use crate::freeze_the_m5_companion_component_matrix::{
    M5CompanionAccessibilityRoute, M5CompanionClientScope, M5CompanionComponentFamily,
    M5CompanionConsumerSurface, M5CompanionDegradedReason, M5CompanionDeploymentLine,
    M5CompanionDowngradeTrigger, M5CompanionFreshness, M5CompanionHandoffTarget,
    M5CompanionObjectKind, M5CompanionRequiredLabel, M5CompanionSeverity, M5CompanionSurfaceFamily,
    M5_COMPANION_COMPONENT_DOC_REF, M5_COMPANION_COMPONENT_FOUNDATION_MATRIX_REF,
    M5_COMPANION_COMPONENT_FOUNDATION_SESSION_FOLLOW_REF, M5_COMPANION_COMPONENT_SCHEMA_REF,
    M5_DESKTOP_HANDOFF_SHEET_SCHEMA_REF, M5_INCIDENT_SNAPSHOT_CARD_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`IncidentSnapshotCardDesktopHandoffSheetControlsPacket`].
pub const INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_RECORD_KIND: &str =
    "incident_snapshot_card_desktop_handoff_sheet_controls";

/// Schema version for incident-snapshot-card / desktop-handoff-sheet control records.
pub const INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_SCHEMA_REF: &str =
    "schemas/ui/m5-incident-snapshot-card-desktop-handoff-sheet-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_DOC_REF: &str =
    "docs/companion/implement_incident_snapshot_cards_and_desktop_handoff_sheets.md";

/// Repo-relative path of the protected fixture directory.
pub const INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_FIXTURE_DIR: &str =
    "fixtures/ui/m5-incident-snapshot-card-desktop-handoff-sheet-controls";

/// Repo-relative path of the checked support-export artifact.
pub const INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_ARTIFACT_REF: &str =
    "artifacts/release/m5-incident-snapshot-card-desktop-handoff-sheet-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_SUMMARY_REF: &str =
    "artifacts/release/m5-incident-snapshot-card-desktop-handoff-sheet-proof/summary.md";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_CSV_REF: &str =
    "artifacts/release/m5-incident-snapshot-card-desktop-handoff-sheet-proof/matrix.csv";

// ---- incident-snapshot-card vocabulary ----------------------------------

/// Controlled service / source class an incident-snapshot card binds, so a user always knows
/// which service an incident belongs to and whether it is a live provider read or a mirrored
/// snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentServiceClass {
    /// A service monitored by the local core.
    LocalCoreService,
    /// A service monitored by a hosted provider.
    HostedService,
    /// A service monitored by a self-hosted runner.
    SelfHostedService,
    /// A mirrored / offline snapshot of a service's incident.
    MirroredSnapshot,
    /// An aggregate of incident signals from more than one source.
    AggregatedSource,
    /// The service / source could not be determined.
    UnknownSource,
}

impl IncidentServiceClass {
    /// Every service class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalCoreService,
        Self::HostedService,
        Self::SelfHostedService,
        Self::MirroredSnapshot,
        Self::AggregatedSource,
        Self::UnknownSource,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCoreService => "local_core_service",
            Self::HostedService => "hosted_service",
            Self::SelfHostedService => "self_hosted_service",
            Self::MirroredSnapshot => "mirrored_snapshot",
            Self::AggregatedSource => "aggregated_source",
            Self::UnknownSource => "unknown_source",
        }
    }
}

/// Controlled incident status — the latest lifecycle state of the incident shown on the card.
///
/// The frozen matrix leaves the incident lifecycle implicit (it freezes only severity for the
/// incident family), so this lane mints the status vocabulary. The derived awareness class is
/// resolved from it, never asserted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentStatus {
    /// The incident is firing and not yet acknowledged.
    Firing,
    /// The incident has been acknowledged.
    Acknowledged,
    /// The incident is being investigated.
    Investigating,
    /// The incident is being mitigated.
    Mitigating,
    /// The incident is resolved.
    Resolved,
    /// The status is stale and could not be refreshed.
    Stale,
}

impl IncidentStatus {
    /// Every incident status, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Firing,
        Self::Acknowledged,
        Self::Investigating,
        Self::Mitigating,
        Self::Resolved,
        Self::Stale,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Firing => "firing",
            Self::Acknowledged => "acknowledged",
            Self::Investigating => "investigating",
            Self::Mitigating => "mitigating",
            Self::Resolved => "resolved",
            Self::Stale => "stale",
        }
    }
}

/// Derived awareness class an incident-snapshot card may present.
///
/// This is the incident honesty axis: the class is derived from the incident status, never
/// asserted, so a stale incident status can never read as a live incident.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentAwarenessClass {
    /// The incident is active and not yet acknowledged.
    ActiveUnacknowledged,
    /// The incident is active and acknowledged or under investigation.
    ActiveAcknowledged,
    /// The incident is being mitigated.
    Mitigating,
    /// The incident is resolved.
    Resolved,
    /// The status is stale and cannot be read as a live incident.
    StaleUnknown,
}

impl IncidentAwarenessClass {
    /// Every awareness class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ActiveUnacknowledged,
        Self::ActiveAcknowledged,
        Self::Mitigating,
        Self::Resolved,
        Self::StaleUnknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveUnacknowledged => "active_unacknowledged",
            Self::ActiveAcknowledged => "active_acknowledged",
            Self::Mitigating => "mitigating",
            Self::Resolved => "resolved",
            Self::StaleUnknown => "stale_unknown",
        }
    }
}

/// One keyboard-complete default action an incident-snapshot card offers, so a card never
/// hides its action affordance behind a pointer-only gesture and every widening action is
/// traceable to one exact desktop target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentSnapshotCardVerb {
    /// Open the exact incident record this card references.
    Open,
    /// Acknowledge the incident — a bounded companion-safe awareness action.
    Acknowledge,
    /// View the incident timeline (read-only from the companion).
    ViewTimeline,
    /// Follow the incident for further updates.
    Follow,
    /// Hand off to the exact desktop target.
    HandoffToDesktop,
    /// Dismiss the card.
    Dismiss,
}

impl IncidentSnapshotCardVerb {
    /// Every incident verb, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Open,
        Self::Acknowledge,
        Self::ViewTimeline,
        Self::Follow,
        Self::HandoffToDesktop,
        Self::Dismiss,
    ];

    /// The default verbs every keyboard-complete card must offer.
    pub const MANDATORY: [Self; 1] = [Self::Open];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Acknowledge => "acknowledge",
            Self::ViewTimeline => "view_timeline",
            Self::Follow => "follow",
            Self::HandoffToDesktop => "handoff_to_desktop",
            Self::Dismiss => "dismiss",
        }
    }
}

/// Disclosures an incident-snapshot card must carry, derived from the incident status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IncidentCardDisclosure {
    /// The derived awareness class this card may present.
    pub awareness_class: IncidentAwarenessClass,
    /// Whether the card may present a live, current incident.
    pub is_live_status: bool,
    /// Whether the card must carry an explicit stale note.
    pub needs_stale_note: bool,
    /// Whether the incident is still open.
    pub is_open: bool,
    /// Whether the card must carry an explicit awareness-only note (remediation is off-companion).
    pub needs_awareness_note: bool,
    /// Whether the incident is resolved.
    pub is_resolved: bool,
}

/// Resolves the awareness truth an incident-snapshot card may present.
///
/// A firing incident is active and unacknowledged. An acknowledged or investigating incident is
/// active and acknowledged. A mitigating incident is mitigating. A resolved incident is
/// resolved. A stale status is stale-unknown — never a live incident — so a card whose status
/// could not be refreshed never reads as a current incident.
pub fn resolve_incident_awareness(status: IncidentStatus) -> IncidentCardDisclosure {
    use IncidentAwarenessClass as Awareness;
    use IncidentStatus as Status;

    let awareness_class = match status {
        Status::Firing => Awareness::ActiveUnacknowledged,
        Status::Acknowledged | Status::Investigating => Awareness::ActiveAcknowledged,
        Status::Mitigating => Awareness::Mitigating,
        Status::Resolved => Awareness::Resolved,
        Status::Stale => Awareness::StaleUnknown,
    };

    let is_open = matches!(
        awareness_class,
        Awareness::ActiveUnacknowledged | Awareness::ActiveAcknowledged | Awareness::Mitigating
    );

    IncidentCardDisclosure {
        awareness_class,
        is_live_status: !matches!(awareness_class, Awareness::StaleUnknown),
        needs_stale_note: matches!(awareness_class, Awareness::StaleUnknown),
        is_open,
        needs_awareness_note: is_open,
        is_resolved: matches!(awareness_class, Awareness::Resolved),
    }
}

/// An incident-snapshot card naming service/source class, service/run identity, severity,
/// latest status, derived awareness, permitted bounded actions, and an exact handoff target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentSnapshotCard {
    /// Frozen component this control implements; must be `incident_snapshot_card`.
    pub component: M5CompanionComponentFamily,
    /// Stable card id.
    pub card_id: String,
    /// Human-readable incident label; required and non-empty.
    pub incident_label: String,
    /// Object kind this card references, reused from the frozen matrix.
    pub object_kind: M5CompanionObjectKind,
    /// Human-readable object label; required and non-empty.
    pub object_label: String,
    /// Exact object landing reference — the one stable object `Open` lands on, never a
    /// generic activity page. Required and non-empty.
    pub object_landing_ref: String,
    /// Stable service identity (e.g. an affected service id); required and non-empty.
    pub service_ref: String,
    /// Stable run identity (e.g. an incident run/occurrence id); required and non-empty.
    pub run_ref: String,
    /// Client scope this card is scoped to, reused from the frozen matrix.
    pub client_scope: M5CompanionClientScope,
    /// Human-readable client-scope label; required and non-empty.
    pub scope_label: String,
    /// Service / source class behind this card.
    pub service_class: IncidentServiceClass,
    /// Human-readable service / source label; required and non-empty.
    pub service_label: String,
    /// Severity, reused from the frozen matrix.
    pub severity: M5CompanionSeverity,
    /// Human-readable severity label; always required so severity stays explicit.
    pub severity_label: String,
    /// Latest incident status.
    pub incident_status: IncidentStatus,
    /// Derived awareness class (must equal the resolved class).
    pub awareness_class: IncidentAwarenessClass,
    /// Whether the card claims a live, current incident (must equal the derived truth).
    pub claims_live_status: bool,
    /// Freshness class, reused from the frozen matrix.
    pub freshness: M5CompanionFreshness,
    /// Stale note; required when the incident status is stale-unknown.
    pub stale_note: String,
    /// Awareness-only note; required while the incident is open, so the card never overpromises
    /// remediation depth (remediation happens on desktop, not the companion).
    pub awareness_note: String,
    /// Scope / freshness note; always required so scope and freshness stay explicit.
    pub scope_and_freshness_note: String,
    /// Exact desktop-handoff target, reused from the frozen matrix.
    pub handoff_target: M5CompanionHandoffTarget,
    /// Human-readable handoff label; always required so the handoff target is explicit.
    pub handoff_label: String,
    /// Keyboard-complete default actions (must include the mandatory `Open`).
    pub status_verbs: Vec<IncidentSnapshotCardVerb>,
    /// Degraded reasons this card can name (required, matching the frozen matrix).
    pub degraded_reasons: Vec<M5CompanionDegradedReason>,
    /// Mandatory labels this card can show (must include the mandatory labels).
    pub required_labels: Vec<M5CompanionRequiredLabel>,
    /// Claimed M5 surface families that render this card.
    pub surface_families: Vec<M5CompanionSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5CompanionDeploymentLine>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5CompanionAccessibilityRoute>,
    /// Companion subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks client scope or freshness. MUST be `false`.
    pub masks_scope_or_freshness: bool,
    /// Hard invariant: never hides its companion-versus-desktop capability boundary.
    /// MUST be `false`.
    pub hides_capability_boundary: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be
    /// `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: never implies a desktop-required action is companion-safe. MUST be
    /// `false`.
    pub implies_desktop_action_is_companion_safe: bool,
    /// Hard invariant: `Open` never routes to a generic activity page. MUST be `false`.
    pub routes_to_generic_activity_page: bool,
    /// Hard invariant: never overpromises remediation depth by implying the companion can
    /// resolve the incident inline. MUST be `false`.
    pub implies_companion_remediation: bool,
}

impl IncidentSnapshotCard {
    /// Awareness disclosures this card must carry, derived from the incident status.
    pub fn awareness_disclosure(&self) -> IncidentCardDisclosure {
        resolve_incident_awareness(self.incident_status)
    }

    /// Whether the card offers every mandatory keyboard-complete action.
    fn declares_mandatory_verbs(&self) -> bool {
        let present: BTreeSet<IncidentSnapshotCardVerb> =
            self.status_verbs.iter().copied().collect();
        IncidentSnapshotCardVerb::MANDATORY
            .iter()
            .all(|verb| present.contains(verb))
    }

    /// Whether the card declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CompanionRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CompanionRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the card offers a desktop-handoff verb.
    fn offers_handoff(&self) -> bool {
        self.status_verbs
            .contains(&IncidentSnapshotCardVerb::HandoffToDesktop)
    }
}

// ---- desktop-handoff-sheet vocabulary -----------------------------------

/// Derived open class a desktop-handoff sheet may present.
///
/// This is the handoff honesty axis: the class is derived from the frozen handoff target,
/// never asserted, so a sheet with no resolvable target degrades to an explicit not-openable
/// state instead of implying a desktop client will open the intended object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffOpenClass {
    /// Opens an exact file location (path plus position).
    OpensExactLocation,
    /// Opens an exact panel (a review panel or CI pipeline run view).
    OpensExactPanel,
    /// Opens an exact workspace or session (an incident workspace or agent session).
    OpensExactWorkspace,
    /// Not openable — no desktop target resolves for this sheet.
    NotOpenable,
}

impl HandoffOpenClass {
    /// Every open class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OpensExactLocation,
        Self::OpensExactPanel,
        Self::OpensExactWorkspace,
        Self::NotOpenable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpensExactLocation => "opens_exact_location",
            Self::OpensExactPanel => "opens_exact_panel",
            Self::OpensExactWorkspace => "opens_exact_workspace",
            Self::NotOpenable => "not_openable",
        }
    }
}

/// Controlled auth / tenant context a desktop-handoff sheet binds, so a handoff that lands in a
/// different tenant, needs re-auth, or targets a different account always carries a reminder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffAuthContext {
    /// Same auth and tenant — no reminder is needed.
    SameAuthNoReminder,
    /// The desktop session may need re-authentication first.
    ReauthRequired,
    /// The desktop client must switch tenant / organization first.
    TenantSwitchRequired,
    /// The desktop client is signed into a different account.
    AccountMismatchWarning,
    /// The desktop client needs an elevated scope first.
    ScopeElevationRequired,
}

impl HandoffAuthContext {
    /// Every auth context, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SameAuthNoReminder,
        Self::ReauthRequired,
        Self::TenantSwitchRequired,
        Self::AccountMismatchWarning,
        Self::ScopeElevationRequired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameAuthNoReminder => "same_auth_no_reminder",
            Self::ReauthRequired => "reauth_required",
            Self::TenantSwitchRequired => "tenant_switch_required",
            Self::AccountMismatchWarning => "account_mismatch_warning",
            Self::ScopeElevationRequired => "scope_elevation_required",
        }
    }

    /// Whether this context requires an explicit auth / tenant reminder note.
    pub const fn needs_reminder(self) -> bool {
        !matches!(self, Self::SameAuthNoReminder)
    }
}

/// One keyboard-complete default action a desktop-handoff sheet offers, so a sheet never hides
/// its open affordance behind a pointer-only gesture and never offers an ambiguous open into a
/// target it cannot resolve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DesktopHandoffSheetVerb {
    /// Open the exact object this sheet references (the object landing reference).
    Open,
    /// Open the exact target on desktop — the widening handoff action.
    OpenOnDesktop,
    /// Copy the exact target reference.
    CopyReference,
    /// Share the handoff.
    Share,
    /// Preview exactly what will open on desktop.
    PreviewTarget,
    /// Dismiss the sheet.
    Dismiss,
}

impl DesktopHandoffSheetVerb {
    /// Every desktop-handoff verb, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Open,
        Self::OpenOnDesktop,
        Self::CopyReference,
        Self::Share,
        Self::PreviewTarget,
        Self::Dismiss,
    ];

    /// The default verbs every keyboard-complete sheet must offer.
    pub const MANDATORY: [Self; 1] = [Self::Open];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::OpenOnDesktop => "open_on_desktop",
            Self::CopyReference => "copy_reference",
            Self::Share => "share",
            Self::PreviewTarget => "preview_target",
            Self::Dismiss => "dismiss",
        }
    }

    /// Whether this verb opens the target on desktop.
    fn is_open_on_desktop_verb(self) -> bool {
        matches!(self, Self::OpenOnDesktop)
    }
}

/// Disclosures a desktop-handoff sheet must carry, derived from the handoff target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandoffSheetDisclosure {
    /// The derived open class this sheet may present.
    pub open_class: HandoffOpenClass,
    /// Whether the sheet may present the target as openable on desktop.
    pub is_openable: bool,
    /// Whether the sheet must carry an explicit not-openable note.
    pub needs_not_openable_note: bool,
}

/// Resolves the open truth a desktop-handoff sheet may present.
///
/// A file location opens an exact location. A review panel or CI pipeline run opens an exact
/// panel. An incident workspace or agent session opens an exact workspace. No handoff target
/// floors to not-openable, so a sheet with nothing to open never implies a desktop client will
/// open the intended object.
pub fn resolve_handoff_open(target: M5CompanionHandoffTarget) -> HandoffSheetDisclosure {
    use HandoffOpenClass as Open;
    use M5CompanionHandoffTarget as Target;

    let open_class = match target {
        Target::FileLocation => Open::OpensExactLocation,
        Target::ReviewPanel | Target::CiPipelineRun => Open::OpensExactPanel,
        Target::IncidentWorkspace | Target::AgentSession => Open::OpensExactWorkspace,
        Target::NoHandoff => Open::NotOpenable,
    };

    HandoffSheetDisclosure {
        open_class,
        is_openable: !matches!(open_class, Open::NotOpenable),
        needs_not_openable_note: matches!(open_class, Open::NotOpenable),
    }
}

/// A desktop-handoff sheet naming target object, stable target identity, client scope, exactly
/// what opens on desktop, an auth or tenant reminder where relevant, derived open class, and
/// permitted verbs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopHandoffSheet {
    /// Frozen component this control implements; must be `desktop_handoff_sheet`.
    pub component: M5CompanionComponentFamily,
    /// Stable sheet id.
    pub sheet_id: String,
    /// Human-readable handoff title; required and non-empty.
    pub handoff_title: String,
    /// Object kind this sheet references, reused from the frozen matrix.
    pub object_kind: M5CompanionObjectKind,
    /// Human-readable object label; required and non-empty.
    pub object_label: String,
    /// Exact object landing reference — the one stable object `Open` lands on, never a
    /// generic activity page. Required and non-empty.
    pub object_landing_ref: String,
    /// Stable target identity — the exact desktop target `OpenOnDesktop` resolves to; required
    /// and non-empty.
    pub target_ref: String,
    /// Human-readable target-object label — what opens on desktop; required and non-empty.
    pub target_object_label: String,
    /// Client scope this sheet is scoped to, reused from the frozen matrix.
    pub client_scope: M5CompanionClientScope,
    /// Human-readable client-scope label; required and non-empty.
    pub scope_label: String,
    /// Exact desktop-handoff target, reused from the frozen matrix.
    pub handoff_target: M5CompanionHandoffTarget,
    /// Derived open class (must equal the resolved class).
    pub open_class: HandoffOpenClass,
    /// Whether the sheet claims the target is openable on desktop (must equal the derived truth).
    pub claims_openable: bool,
    /// Freshness class, reused from the frozen matrix.
    pub freshness: M5CompanionFreshness,
    /// What-opens-on-desktop note; always required so the desktop target stays explicit.
    pub opens_on_desktop_note: String,
    /// Not-openable note; required when the sheet has no resolvable desktop target.
    pub not_openable_note: String,
    /// Auth / tenant context behind this handoff.
    pub auth_context: HandoffAuthContext,
    /// Auth / tenant reminder note; required when the auth context needs a reminder.
    pub auth_tenant_reminder_note: String,
    /// Scope / freshness note; always required so scope and freshness stay explicit.
    pub scope_and_freshness_note: String,
    /// Human-readable handoff label; always required so the handoff action is explicit.
    pub handoff_label: String,
    /// Keyboard-complete default verbs (must include the mandatory `Open`).
    pub handoff_verbs: Vec<DesktopHandoffSheetVerb>,
    /// Degraded reasons this sheet can name (required, matching the frozen matrix).
    pub degraded_reasons: Vec<M5CompanionDegradedReason>,
    /// Mandatory labels this sheet can show (must include the mandatory labels).
    pub required_labels: Vec<M5CompanionRequiredLabel>,
    /// Claimed M5 surface families that render this sheet.
    pub surface_families: Vec<M5CompanionSurfaceFamily>,
    /// Deployment lines this sheet keeps the same truth across.
    pub deployment_lines: Vec<M5CompanionDeploymentLine>,
    /// Non-visual accessibility routes this sheet offers.
    pub accessibility_routes: Vec<M5CompanionAccessibilityRoute>,
    /// Companion subsystems that consume this sheet's projection.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this sheet.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks client scope or freshness. MUST be `false`.
    pub masks_scope_or_freshness: bool,
    /// Hard invariant: never hides its companion-versus-desktop capability boundary.
    /// MUST be `false`.
    pub hides_capability_boundary: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be
    /// `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: never implies a desktop-required action is companion-safe. MUST be
    /// `false`.
    pub implies_desktop_action_is_companion_safe: bool,
    /// Hard invariant: `Open` never routes to a generic activity page. MUST be `false`.
    pub routes_to_generic_activity_page: bool,
}

impl DesktopHandoffSheet {
    /// Open disclosures this sheet must carry, derived from the handoff target.
    pub fn open_disclosure(&self) -> HandoffSheetDisclosure {
        resolve_handoff_open(self.handoff_target)
    }

    /// Whether the sheet offers every mandatory keyboard-complete verb.
    fn declares_mandatory_verbs(&self) -> bool {
        let present: BTreeSet<DesktopHandoffSheetVerb> =
            self.handoff_verbs.iter().copied().collect();
        DesktopHandoffSheetVerb::MANDATORY
            .iter()
            .all(|verb| present.contains(verb))
    }

    /// Whether the sheet declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CompanionRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CompanionRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the sheet offers an open-on-desktop verb.
    fn offers_open_on_desktop(&self) -> bool {
        self.handoff_verbs
            .iter()
            .any(|verb| verb.is_open_on_desktop_verb())
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance trust review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentSnapshotCardDesktopHandoffSheetGlanceReview {
    /// The incident card names its service and run identity.
    pub incident_card_shows_service_and_run_identity: bool,
    /// The incident card names its severity.
    pub incident_card_shows_severity: bool,
    /// The incident card states its latest status.
    pub incident_card_states_latest_status: bool,
    /// The incident card stays awareness-only and never overpromises remediation depth.
    pub incident_card_stays_awareness_only: bool,
    /// The handoff sheet names its target object and stable target identity.
    pub handoff_sheet_shows_target_object_and_identity: bool,
    /// The handoff sheet states exactly what opens on desktop.
    pub handoff_sheet_states_what_opens_on_desktop: bool,
    /// The handoff sheet shows an auth or tenant reminder where relevant.
    pub handoff_sheet_shows_auth_or_tenant_reminder: bool,
    /// The object identity is always explicit.
    pub object_identity_always_explicit: bool,
    /// The client scope is always explicit.
    pub client_scope_always_explicit: bool,
    /// The freshness is always explicit.
    pub freshness_always_explicit: bool,
    /// Awareness / openability is derived from status / target, never asserted.
    pub awareness_and_openability_derived_never_asserted: bool,
    /// A stale card is never shown as live.
    pub stale_never_shown_as_live: bool,
    /// Every verb traces to one stable object.
    pub every_verb_traces_to_one_object: bool,
    /// Every widening verb names one exact desktop-handoff target.
    pub every_handoff_names_exact_target: bool,
    /// A desktop-only action is never implied companion-safe.
    pub desktop_only_action_never_implied_companion_safe: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl IncidentSnapshotCardDesktopHandoffSheetGlanceReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.incident_card_shows_service_and_run_identity
            && self.incident_card_shows_severity
            && self.incident_card_states_latest_status
            && self.incident_card_stays_awareness_only
            && self.handoff_sheet_shows_target_object_and_identity
            && self.handoff_sheet_states_what_opens_on_desktop
            && self.handoff_sheet_shows_auth_or_tenant_reminder
            && self.object_identity_always_explicit
            && self.client_scope_always_explicit
            && self.freshness_always_explicit
            && self.awareness_and_openability_derived_never_asserted
            && self.stale_never_shown_as_live
            && self.every_verb_traces_to_one_object
            && self.every_handoff_names_exact_target
            && self.desktop_only_action_never_implied_companion_safe
            && self.no_surface_invents_alternate_state_label
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentSnapshotCardDesktopHandoffSheetConsumerProjection {
    /// The incident-awareness UI reads a single canonical source.
    pub incident_awareness_ui_reads_single_source: bool,
    /// The desktop-handoff UI reads a single canonical source.
    pub desktop_handoff_ui_reads_single_source: bool,
    /// The first glance names object, scope, and freshness without drilling in.
    pub first_glance_names_object_scope_and_freshness: bool,
    /// The remediation / open posture is visible before a tap.
    pub remediation_and_open_posture_visible_before_tap: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl IncidentSnapshotCardDesktopHandoffSheetConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.incident_awareness_ui_reads_single_source
            && self.desktop_handoff_ui_reads_single_source
            && self.first_glance_names_object_scope_and_freshness
            && self.remediation_and_open_posture_visible_before_tap
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentSnapshotCardDesktopHandoffSheetProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`IncidentSnapshotCardDesktopHandoffSheetControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncidentSnapshotCardDesktopHandoffSheetControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Incident-snapshot cards.
    pub incident_snapshot_cards: Vec<IncidentSnapshotCard>,
    /// Desktop-handoff sheets.
    pub desktop_handoff_sheets: Vec<DesktopHandoffSheet>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Glance review block.
    pub glance_review: IncidentSnapshotCardDesktopHandoffSheetGlanceReview,
    /// Consumer projection block.
    pub consumer_projection: IncidentSnapshotCardDesktopHandoffSheetConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: IncidentSnapshotCardDesktopHandoffSheetProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe incident-snapshot-card / desktop-handoff-sheet controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentSnapshotCardDesktopHandoffSheetControlsPacket {
    /// Record kind; must equal [`INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Incident-snapshot cards.
    pub incident_snapshot_cards: Vec<IncidentSnapshotCard>,
    /// Desktop-handoff sheets.
    pub desktop_handoff_sheets: Vec<DesktopHandoffSheet>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Glance review block.
    pub glance_review: IncidentSnapshotCardDesktopHandoffSheetGlanceReview,
    /// Consumer projection block.
    pub consumer_projection: IncidentSnapshotCardDesktopHandoffSheetConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: IncidentSnapshotCardDesktopHandoffSheetProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl IncidentSnapshotCardDesktopHandoffSheetControlsPacket {
    /// Builds an incident-snapshot-card / desktop-handoff-sheet controls packet from stable-lane
    /// input.
    pub fn new(input: IncidentSnapshotCardDesktopHandoffSheetControlsPacketInput) -> Self {
        Self {
            record_kind: INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_RECORD_KIND.to_owned(),
            schema_version: INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            incident_snapshot_cards: input.incident_snapshot_cards,
            desktop_handoff_sheets: input.desktop_handoff_sheets,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            glance_review: input.glance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the incident-snapshot-card / desktop-handoff-sheet control invariants.
    pub fn validate(&self) -> Vec<IncidentSnapshotCardDesktopHandoffSheetViolation> {
        let mut violations = Vec::new();

        if self.record_kind != INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_RECORD_KIND {
            violations.push(IncidentSnapshotCardDesktopHandoffSheetViolation::WrongRecordKind);
        }
        if self.schema_version != INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_SCHEMA_VERSION {
            violations.push(IncidentSnapshotCardDesktopHandoffSheetViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(IncidentSnapshotCardDesktopHandoffSheetViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_incident_snapshot_cards(self, &mut violations);
        validate_desktop_handoff_sheets(self, &mut violations);

        if !self.glance_review.all_hold() {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::GlanceReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::ConsumerProjectionIncomplete,
            );
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("incident snapshot card desktop handoff sheet packet serializes"),
        ) {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::RawBoundaryMaterialInExport,
            );
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("incident snapshot card desktop handoff sheet packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per control.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "control,id,object_kind,client_scope,freshness,state_or_target,derived,live_or_openable\n",
        );
        for card in &self.incident_snapshot_cards {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                "incident_snapshot_card",
                csv_field(&card.card_id),
                card.object_kind.as_str(),
                card.client_scope.as_str(),
                card.freshness.as_str(),
                card.incident_status.as_str(),
                card.awareness_disclosure().awareness_class.as_str(),
                card.awareness_disclosure().is_live_status,
            ));
        }
        for sheet in &self.desktop_handoff_sheets {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                "desktop_handoff_sheet",
                csv_field(&sheet.sheet_id),
                sheet.object_kind.as_str(),
                sheet.client_scope.as_str(),
                sheet.freshness.as_str(),
                sheet.handoff_target.as_str(),
                sheet.open_disclosure().open_class.as_str(),
                sheet.open_disclosure().is_openable,
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let not_live = self
            .incident_snapshot_cards
            .iter()
            .filter(|card| !card.awareness_disclosure().is_live_status)
            .count();
        let not_openable = self
            .desktop_handoff_sheets
            .iter()
            .filter(|sheet| !sheet.open_disclosure().is_openable)
            .count();

        let mut out = String::new();
        out.push_str("# Incident-snapshot cards and desktop-handoff sheets\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Incident-snapshot cards: {} ({} not a live incident)\n",
            self.incident_snapshot_cards.len(),
            not_live
        ));
        out.push_str(&format!(
            "- Desktop-handoff sheets: {} ({} not openable)\n",
            self.desktop_handoff_sheets.len(),
            not_openable
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Incident-snapshot cards\n\n");
        for card in &self.incident_snapshot_cards {
            out.push_str(&format!(
                "- **{}** ({}) — scope `{}`, service `{}`, severity `{}`, status `{}`, freshness `{}` → `{}`, handoff `{}`\n",
                card.incident_label,
                card.object_kind.as_str(),
                card.client_scope.as_str(),
                card.service_class.as_str(),
                card.severity.as_str(),
                card.incident_status.as_str(),
                card.freshness.as_str(),
                card.awareness_disclosure().awareness_class.as_str(),
                card.handoff_target.as_str(),
            ));
        }

        out.push_str("\n## Desktop-handoff sheets\n\n");
        for sheet in &self.desktop_handoff_sheets {
            out.push_str(&format!(
                "- **{}** ({}) — scope `{}`, target `{}`, auth `{}`, freshness `{}` → `{}`\n",
                sheet.handoff_title,
                sheet.object_kind.as_str(),
                sheet.client_scope.as_str(),
                sheet.handoff_target.as_str(),
                sheet.auth_context.as_str(),
                sheet.freshness.as_str(),
                sheet.open_disclosure().open_class.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in incident-snapshot-card / desktop-handoff-sheet
/// export.
#[derive(Debug)]
pub enum IncidentSnapshotCardDesktopHandoffSheetArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<IncidentSnapshotCardDesktopHandoffSheetViolation>),
}

impl fmt::Display for IncidentSnapshotCardDesktopHandoffSheetArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "incident snapshot card desktop handoff sheet export parse failed: {error}"
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
                    "incident snapshot card desktop handoff sheet export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for IncidentSnapshotCardDesktopHandoffSheetArtifactError {}

/// Validation failures emitted by
/// [`IncidentSnapshotCardDesktopHandoffSheetControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IncidentSnapshotCardDesktopHandoffSheetViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No incident-snapshot cards are present.
    IncidentSnapshotCardsMissing,
    /// An incident-snapshot card is incomplete.
    IncidentSnapshotCardIncomplete,
    /// An incident-snapshot card carries the wrong frozen component class.
    IncidentSnapshotCardWrongComponentClass,
    /// A control does not name its exact object landing reference.
    ObjectLandingRefMissing,
    /// An incident-snapshot card does not name its stable service / run identity.
    ServiceOrRunIdentityMissing,
    /// An incident-snapshot card does not name its service / source label.
    ServiceLabelMissing,
    /// An incident-snapshot card does not name its severity label.
    SeverityLabelMissing,
    /// An incident-snapshot card misrepresents its derived awareness state.
    AwarenessStateMisrepresented,
    /// A stale incident-snapshot card does not name its stale state.
    StaleNoteMissing,
    /// An open incident-snapshot card does not name its awareness-only state.
    AwarenessNoteMissing,
    /// An incident-snapshot card overpromises remediation depth.
    RemediationDepthOverpromised,
    /// An incident-snapshot card omits the mandatory `Open` verb.
    IncidentVerbsIncomplete,
    /// The incident-snapshot cards do not cover every derived awareness class.
    AwarenessClassCoverageMissing,
    /// The incident-snapshot cards do not cover every incident status.
    IncidentStatusCoverageMissing,
    /// No desktop-handoff sheets are present.
    DesktopHandoffSheetsMissing,
    /// A desktop-handoff sheet is incomplete.
    DesktopHandoffSheetIncomplete,
    /// A desktop-handoff sheet carries the wrong frozen component class.
    DesktopHandoffSheetWrongComponentClass,
    /// A desktop-handoff sheet does not name its stable target identity.
    TargetIdentityMissing,
    /// A desktop-handoff sheet does not name its target-object label.
    TargetObjectLabelMissing,
    /// A desktop-handoff sheet misrepresents its derived open state.
    HandoffOpenMisrepresented,
    /// A desktop-handoff sheet does not name exactly what opens on desktop.
    OpensOnDesktopNoteMissing,
    /// A not-openable desktop-handoff sheet does not name its not-openable state.
    NotOpenableNoteMissing,
    /// A desktop-handoff sheet needs an auth / tenant reminder but does not name it.
    AuthTenantReminderMissing,
    /// A desktop-handoff sheet omits the mandatory `Open` verb.
    DesktopHandoffVerbsIncomplete,
    /// A desktop-handoff sheet offers an ambiguous open into a not-openable target.
    AmbiguousHandoffOffered,
    /// The desktop-handoff sheets do not cover every open class.
    HandoffOpenClassCoverageMissing,
    /// The desktop-handoff sheets do not cover every handoff target.
    HandoffTargetCoverageMissing,
    /// A control does not name its scope / freshness.
    ScopeAndFreshnessNoteMissing,
    /// A control does not name its scope label.
    ScopeLabelMissing,
    /// A control offers a handoff verb but its handoff target does not resolve exactly.
    HandoffTargetUnresolved,
    /// A control does not name its handoff label.
    HandoffLabelMissing,
    /// A control does not declare its degraded reasons.
    DegradedReasonsMissing,
    /// A control does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A control does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A control masks its client scope or freshness.
    ScopeOrFreshnessMasked,
    /// A control hides its companion-versus-desktop capability boundary.
    CapabilityBoundaryHidden,
    /// A control invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// A control implies a desktop-required action is companion-safe.
    DesktopActionImpliedCompanionSafe,
    /// A control routes to a generic activity page instead of one stable object.
    RoutesToGenericActivityPage,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Glance review does not satisfy required invariants.
    GlanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl IncidentSnapshotCardDesktopHandoffSheetViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::IncidentSnapshotCardsMissing => "incident_snapshot_cards_missing",
            Self::IncidentSnapshotCardIncomplete => "incident_snapshot_card_incomplete",
            Self::IncidentSnapshotCardWrongComponentClass => {
                "incident_snapshot_card_wrong_component_class"
            }
            Self::ObjectLandingRefMissing => "object_landing_ref_missing",
            Self::ServiceOrRunIdentityMissing => "service_or_run_identity_missing",
            Self::ServiceLabelMissing => "service_label_missing",
            Self::SeverityLabelMissing => "severity_label_missing",
            Self::AwarenessStateMisrepresented => "awareness_state_misrepresented",
            Self::StaleNoteMissing => "stale_note_missing",
            Self::AwarenessNoteMissing => "awareness_note_missing",
            Self::RemediationDepthOverpromised => "remediation_depth_overpromised",
            Self::IncidentVerbsIncomplete => "incident_verbs_incomplete",
            Self::AwarenessClassCoverageMissing => "awareness_class_coverage_missing",
            Self::IncidentStatusCoverageMissing => "incident_status_coverage_missing",
            Self::DesktopHandoffSheetsMissing => "desktop_handoff_sheets_missing",
            Self::DesktopHandoffSheetIncomplete => "desktop_handoff_sheet_incomplete",
            Self::DesktopHandoffSheetWrongComponentClass => {
                "desktop_handoff_sheet_wrong_component_class"
            }
            Self::TargetIdentityMissing => "target_identity_missing",
            Self::TargetObjectLabelMissing => "target_object_label_missing",
            Self::HandoffOpenMisrepresented => "handoff_open_misrepresented",
            Self::OpensOnDesktopNoteMissing => "opens_on_desktop_note_missing",
            Self::NotOpenableNoteMissing => "not_openable_note_missing",
            Self::AuthTenantReminderMissing => "auth_tenant_reminder_missing",
            Self::DesktopHandoffVerbsIncomplete => "desktop_handoff_verbs_incomplete",
            Self::AmbiguousHandoffOffered => "ambiguous_handoff_offered",
            Self::HandoffOpenClassCoverageMissing => "handoff_open_class_coverage_missing",
            Self::HandoffTargetCoverageMissing => "handoff_target_coverage_missing",
            Self::ScopeAndFreshnessNoteMissing => "scope_and_freshness_note_missing",
            Self::ScopeLabelMissing => "scope_label_missing",
            Self::HandoffTargetUnresolved => "handoff_target_unresolved",
            Self::HandoffLabelMissing => "handoff_label_missing",
            Self::DegradedReasonsMissing => "degraded_reasons_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ScopeOrFreshnessMasked => "scope_or_freshness_masked",
            Self::CapabilityBoundaryHidden => "capability_boundary_hidden",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::DesktopActionImpliedCompanionSafe => "desktop_action_implied_companion_safe",
            Self::RoutesToGenericActivityPage => "routes_to_generic_activity_page",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::GlanceReviewIncomplete => "glance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable incident-snapshot-card / desktop-handoff-sheet
/// export.
pub fn current_incident_snapshot_card_desktop_handoff_sheet_export() -> Result<
    IncidentSnapshotCardDesktopHandoffSheetControlsPacket,
    IncidentSnapshotCardDesktopHandoffSheetArtifactError,
> {
    let packet: IncidentSnapshotCardDesktopHandoffSheetControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-incident-snapshot-card-desktop-handoff-sheet-proof/support_export.json"
        )))
        .map_err(IncidentSnapshotCardDesktopHandoffSheetArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(IncidentSnapshotCardDesktopHandoffSheetArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &IncidentSnapshotCardDesktopHandoffSheetControlsPacket,
    violations: &mut Vec<IncidentSnapshotCardDesktopHandoffSheetViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_SCHEMA_REF,
        INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_DOC_REF,
        M5_COMPANION_COMPONENT_SCHEMA_REF,
        M5_COMPANION_COMPONENT_DOC_REF,
        M5_INCIDENT_SNAPSHOT_CARD_SCHEMA_REF,
        M5_DESKTOP_HANDOFF_SHEET_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_incident_snapshot_cards(
    packet: &IncidentSnapshotCardDesktopHandoffSheetControlsPacket,
    violations: &mut Vec<IncidentSnapshotCardDesktopHandoffSheetViolation>,
) {
    if packet.incident_snapshot_cards.is_empty() {
        violations
            .push(IncidentSnapshotCardDesktopHandoffSheetViolation::IncidentSnapshotCardsMissing);
        return;
    }

    let mut awareness_classes: BTreeSet<IncidentAwarenessClass> = BTreeSet::new();
    let mut incident_statuses: BTreeSet<IncidentStatus> = BTreeSet::new();

    for card in &packet.incident_snapshot_cards {
        let disclosure = card.awareness_disclosure();
        awareness_classes.insert(disclosure.awareness_class);
        incident_statuses.insert(card.incident_status);

        if card.card_id.trim().is_empty()
            || card.incident_label.trim().is_empty()
            || card.object_label.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.surface_families.is_empty()
            || card.deployment_lines.is_empty()
            || card.consumer_surfaces.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::IncidentSnapshotCardIncomplete,
            );
        }
        if card.component != M5CompanionComponentFamily::IncidentSnapshotCard {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::IncidentSnapshotCardWrongComponentClass,
            );
        }
        if card.object_landing_ref.trim().is_empty() {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::ObjectLandingRefMissing);
        }
        if card.service_ref.trim().is_empty() || card.run_ref.trim().is_empty() {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::ServiceOrRunIdentityMissing,
            );
        }
        if card.service_label.trim().is_empty() {
            violations.push(IncidentSnapshotCardDesktopHandoffSheetViolation::ServiceLabelMissing);
        }
        if card.severity_label.trim().is_empty() {
            violations.push(IncidentSnapshotCardDesktopHandoffSheetViolation::SeverityLabelMissing);
        }
        if card.awareness_class != disclosure.awareness_class
            || card.claims_live_status != disclosure.is_live_status
        {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::AwarenessStateMisrepresented,
            );
        }
        if disclosure.needs_stale_note && card.stale_note.trim().is_empty() {
            violations.push(IncidentSnapshotCardDesktopHandoffSheetViolation::StaleNoteMissing);
        }
        if disclosure.needs_awareness_note && card.awareness_note.trim().is_empty() {
            violations.push(IncidentSnapshotCardDesktopHandoffSheetViolation::AwarenessNoteMissing);
        }
        if card.implies_companion_remediation {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::RemediationDepthOverpromised,
            );
        }
        if card.scope_and_freshness_note.trim().is_empty() {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::ScopeAndFreshnessNoteMissing,
            );
        }
        if card.scope_label.trim().is_empty() {
            violations.push(IncidentSnapshotCardDesktopHandoffSheetViolation::ScopeLabelMissing);
        }
        if !card.declares_mandatory_verbs() {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::IncidentVerbsIncomplete);
        }
        if card.offers_handoff() && card.handoff_target == M5CompanionHandoffTarget::NoHandoff {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::HandoffTargetUnresolved);
        }
        if card.handoff_label.trim().is_empty() {
            violations.push(IncidentSnapshotCardDesktopHandoffSheetViolation::HandoffLabelMissing);
        }
        validate_common_control(
            &card.degraded_reasons,
            card.declares_mandatory_labels(),
            &card.accessibility_routes,
            ControlInvariants {
                masks_scope_or_freshness: card.masks_scope_or_freshness,
                hides_capability_boundary: card.hides_capability_boundary,
                invents_alternate_state_label: card.invents_alternate_state_label,
                implies_desktop_action_is_companion_safe: card
                    .implies_desktop_action_is_companion_safe,
                routes_to_generic_activity_page: card.routes_to_generic_activity_page,
            },
            violations,
        );
    }

    for required in IncidentAwarenessClass::ALL {
        if !awareness_classes.contains(&required) {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::AwarenessClassCoverageMissing,
            );
            break;
        }
    }
    for required in IncidentStatus::ALL {
        if !incident_statuses.contains(&required) {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::IncidentStatusCoverageMissing,
            );
            break;
        }
    }
}

fn validate_desktop_handoff_sheets(
    packet: &IncidentSnapshotCardDesktopHandoffSheetControlsPacket,
    violations: &mut Vec<IncidentSnapshotCardDesktopHandoffSheetViolation>,
) {
    if packet.desktop_handoff_sheets.is_empty() {
        violations
            .push(IncidentSnapshotCardDesktopHandoffSheetViolation::DesktopHandoffSheetsMissing);
        return;
    }

    let mut open_classes: BTreeSet<HandoffOpenClass> = BTreeSet::new();
    let mut handoff_targets: BTreeSet<M5CompanionHandoffTarget> = BTreeSet::new();

    for sheet in &packet.desktop_handoff_sheets {
        let disclosure = sheet.open_disclosure();
        open_classes.insert(disclosure.open_class);
        handoff_targets.insert(sheet.handoff_target);

        if sheet.sheet_id.trim().is_empty()
            || sheet.handoff_title.trim().is_empty()
            || sheet.object_label.trim().is_empty()
            || sheet.fields_shown.is_empty()
            || sheet.surface_families.is_empty()
            || sheet.deployment_lines.is_empty()
            || sheet.consumer_surfaces.is_empty()
            || sheet.source_contract_refs.is_empty()
        {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::DesktopHandoffSheetIncomplete,
            );
        }
        if sheet.component != M5CompanionComponentFamily::DesktopHandoffSheet {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::DesktopHandoffSheetWrongComponentClass,
            );
        }
        if sheet.object_landing_ref.trim().is_empty() {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::ObjectLandingRefMissing);
        }
        if sheet.target_ref.trim().is_empty() {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::TargetIdentityMissing);
        }
        if sheet.target_object_label.trim().is_empty() {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::TargetObjectLabelMissing);
        }
        if sheet.open_class != disclosure.open_class
            || sheet.claims_openable != disclosure.is_openable
        {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::HandoffOpenMisrepresented);
        }
        if sheet.opens_on_desktop_note.trim().is_empty() {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::OpensOnDesktopNoteMissing);
        }
        if disclosure.needs_not_openable_note && sheet.not_openable_note.trim().is_empty() {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::NotOpenableNoteMissing);
        }
        if sheet.auth_context.needs_reminder() && sheet.auth_tenant_reminder_note.trim().is_empty()
        {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::AuthTenantReminderMissing);
        }
        if sheet.scope_and_freshness_note.trim().is_empty() {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::ScopeAndFreshnessNoteMissing,
            );
        }
        if sheet.scope_label.trim().is_empty() {
            violations.push(IncidentSnapshotCardDesktopHandoffSheetViolation::ScopeLabelMissing);
        }
        if !sheet.declares_mandatory_verbs() {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::DesktopHandoffVerbsIncomplete,
            );
        }
        if sheet.offers_open_on_desktop() && !disclosure.is_openable {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::AmbiguousHandoffOffered);
        }
        if sheet.offers_open_on_desktop()
            && sheet.handoff_target == M5CompanionHandoffTarget::NoHandoff
        {
            violations
                .push(IncidentSnapshotCardDesktopHandoffSheetViolation::HandoffTargetUnresolved);
        }
        if sheet.handoff_label.trim().is_empty() {
            violations.push(IncidentSnapshotCardDesktopHandoffSheetViolation::HandoffLabelMissing);
        }
        validate_common_control(
            &sheet.degraded_reasons,
            sheet.declares_mandatory_labels(),
            &sheet.accessibility_routes,
            ControlInvariants {
                masks_scope_or_freshness: sheet.masks_scope_or_freshness,
                hides_capability_boundary: sheet.hides_capability_boundary,
                invents_alternate_state_label: sheet.invents_alternate_state_label,
                implies_desktop_action_is_companion_safe: sheet
                    .implies_desktop_action_is_companion_safe,
                routes_to_generic_activity_page: sheet.routes_to_generic_activity_page,
            },
            violations,
        );
    }

    for required in HandoffOpenClass::ALL {
        if !open_classes.contains(&required) {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::HandoffOpenClassCoverageMissing,
            );
            break;
        }
    }
    for required in M5CompanionHandoffTarget::ALL {
        if !handoff_targets.contains(&required) {
            violations.push(
                IncidentSnapshotCardDesktopHandoffSheetViolation::HandoffTargetCoverageMissing,
            );
            break;
        }
    }
}

/// The five hard-invariant bools every control must keep `false`.
struct ControlInvariants {
    masks_scope_or_freshness: bool,
    hides_capability_boundary: bool,
    invents_alternate_state_label: bool,
    implies_desktop_action_is_companion_safe: bool,
    routes_to_generic_activity_page: bool,
}

/// Validates the axes shared by both control vectors.
fn validate_common_control(
    degraded_reasons: &[M5CompanionDegradedReason],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5CompanionAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<IncidentSnapshotCardDesktopHandoffSheetViolation>,
) {
    if degraded_reasons.is_empty() {
        violations.push(IncidentSnapshotCardDesktopHandoffSheetViolation::DegradedReasonsMissing);
    }
    if !declares_mandatory_labels {
        violations.push(IncidentSnapshotCardDesktopHandoffSheetViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5CompanionAccessibilityRoute::KeyboardFocusable)
    {
        violations
            .push(IncidentSnapshotCardDesktopHandoffSheetViolation::AccessibilityRouteMissing);
    }
    if invariants.masks_scope_or_freshness {
        violations.push(IncidentSnapshotCardDesktopHandoffSheetViolation::ScopeOrFreshnessMasked);
    }
    if invariants.hides_capability_boundary {
        violations.push(IncidentSnapshotCardDesktopHandoffSheetViolation::CapabilityBoundaryHidden);
    }
    if invariants.invents_alternate_state_label {
        violations
            .push(IncidentSnapshotCardDesktopHandoffSheetViolation::AlternateStateLabelInvented);
    }
    if invariants.implies_desktop_action_is_companion_safe {
        violations.push(
            IncidentSnapshotCardDesktopHandoffSheetViolation::DesktopActionImpliedCompanionSafe,
        );
    }
    if invariants.routes_to_generic_activity_page {
        violations
            .push(IncidentSnapshotCardDesktopHandoffSheetViolation::RoutesToGenericActivityPage);
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
///
/// The companion vocabulary carries no secret-value words, so this check flags only
/// raw-*value* shapes that must never cross the boundary: a password / passphrase
/// literal, a bearer literal, a URL scheme, or a PEM header.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
