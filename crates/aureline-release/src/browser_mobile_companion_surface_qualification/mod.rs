//! Qualification packet for browser and mobile companion surfaces.
//!
//! This module owns the release artifact that keeps companion clients narrow:
//! each promoted browser or mobile row must state its scope, freshness, authority
//! boundary, and exact desktop handoff posture before it can render as Stable.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{OwnerSignoff, StableClaimLevel};

/// Supported schema version for the checked-in companion qualification packet.
pub const BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_RECORD_KIND: &str =
    "browser_mobile_companion_surface_qualification";

/// Repo-relative path to the checked-in packet.
pub const BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_PATH: &str =
    "artifacts/release/m4/browser-mobile-companion-surface-qualification.json";

/// Embedded checked-in packet JSON.
pub const BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m4/browser-mobile-companion-surface-qualification.json"
));

/// Companion client family covered by a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionClientKind {
    /// Browser companion client.
    BrowserCompanion,
    /// Mobile companion client.
    MobileCompanion,
    /// OS or push notification surface.
    CompanionNotification,
    /// Cross-device desktop handoff surface.
    DesktopHandoff,
}

/// Narrow action scope a companion row may claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionScope {
    /// Review-only or review-triage surface.
    Review,
    /// Documentation browsing or context surface.
    Docs,
    /// Follow-only collaboration posture.
    Follow,
    /// Join-only session admission posture.
    Join,
    /// Light edit with bounded file/scope constraints.
    LightEdit,
    /// Notification triage and acknowledgement surface.
    NotificationTriage,
    /// Surface only creates or opens a desktop handoff.
    HandoffOnly,
}

/// Freshness posture visible on a companion row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionFreshness {
    /// Live state is currently connected.
    Live,
    /// Cached state with age or source visible.
    Cached,
    /// Stale snapshot with stale reason visible.
    Stale,
    /// Offline snapshot or metadata-only payload.
    Offline,
    /// Freshness is unknown and the row must narrow.
    Unknown,
}

impl CompanionFreshness {
    /// True when the row must show non-live state before action.
    pub const fn requires_visible_cue(self) -> bool {
        !matches!(self, Self::Live)
    }
}

/// Authority boundary enforced by a companion row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionAuthority {
    /// Observer-only, with no mutation authority.
    ObserverOnly,
    /// Follow-only, with no shared control.
    FollowOnly,
    /// Join admission only; control must be requested separately.
    JoinOnlyControlReRequest,
    /// Comment-only low-authority mutation.
    CommentOnly,
    /// Bounded light edit; privileged actions remain desktop-authoritative.
    LightEditBounded,
    /// The row may only mint a handoff packet.
    HandoffOnly,
    /// Desktop client remains authoritative for the target/action.
    DesktopAuthoritative,
}

impl CompanionAuthority {
    /// True when the row clearly avoids full-IDE or revived-control authority.
    pub const fn is_narrow(self) -> bool {
        matches!(
            self,
            Self::ObserverOnly
                | Self::FollowOnly
                | Self::JoinOnlyControlReRequest
                | Self::CommentOnly
                | Self::LightEditBounded
                | Self::HandoffOnly
                | Self::DesktopAuthoritative
        )
    }
}

/// User-visible lifecycle badge for companion rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionVisibleLabel {
    /// Claimed stable companion scope.
    StableCompanion,
    /// Beta on bounded profiles.
    BetaBounded,
    /// Preview companion scope.
    Preview,
    /// Labs surface outside the stable contract.
    Labs,
    /// Unsupported on this client.
    UnsupportedInClient,
}

impl CompanionVisibleLabel {
    /// True when the label visibly narrows the row below Stable.
    pub const fn is_narrow(self) -> bool {
        !matches!(self, Self::StableCompanion)
    }
}

/// Desktop handoff identity and expiry truth required on every companion row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesktopHandoffTruth {
    /// Exact target object identity is preserved.
    pub object_identity: bool,
    /// Route class is preserved.
    pub route_class: bool,
    /// Return anchor is preserved.
    pub return_anchor: bool,
    /// Tenant or organization identity is preserved.
    pub tenant_identity: bool,
    /// Policy posture is preserved.
    pub policy_posture: bool,
    /// Replay expiry or reauth truth is visible.
    pub replay_expiry_truth: bool,
    /// Prior collaboration role is preserved.
    pub prior_role: bool,
    /// Control authority is never silently revived across handoff.
    pub control_re_request: bool,
}

impl DesktopHandoffTruth {
    fn complete(&self) -> bool {
        self.object_identity
            && self.route_class
            && self.return_anchor
            && self.tenant_identity
            && self.policy_posture
            && self.replay_expiry_truth
            && self.prior_role
            && self.control_re_request
    }
}

/// Publication and support destinations that must ingest the row label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompanionProjection {
    /// Docs and Help ingest the scope vocabulary.
    pub docs_help: bool,
    /// About/service-health surfaces ingest the lifecycle label.
    pub about_service_health: bool,
    /// Public claims avoid broad everywhere wording.
    pub public_claims: bool,
    /// Support export carries scope, freshness, authority, and downgrade reason.
    pub support_export: bool,
}

impl CompanionProjection {
    fn complete(&self) -> bool {
        self.docs_help && self.about_service_health && self.public_claims && self.support_export
    }
}

/// One browser/mobile companion surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompanionSurfaceRow {
    /// Stable row id.
    pub surface_id: String,
    /// Human-readable title.
    pub title: String,
    /// Client family.
    pub client_kind: CompanionClientKind,
    /// Whether the promoted build exposes this surface.
    pub promoted_build_surface: bool,
    /// Claimed lifecycle label before companion qualification.
    pub claim_label: StableClaimLevel,
    /// Label rendered after qualification or narrowing.
    pub displayed_label: StableClaimLevel,
    /// User-visible companion label.
    pub visible_label: CompanionVisibleLabel,
    /// Stable proof packet, absent for preview/labs rows.
    #[serde(default)]
    pub qualification_packet: Option<ProofPacket>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Claimed companion scopes.
    #[serde(default)]
    pub scopes: Vec<CompanionScope>,
    /// Unsupported actions shown before reliance.
    #[serde(default)]
    pub unsupported_actions: Vec<String>,
    /// Required desktop continuation wording.
    pub desktop_continuation: String,
    /// Freshness posture.
    pub freshness: CompanionFreshness,
    /// Stale/offline/cache cues visible to users.
    #[serde(default)]
    pub freshness_cues: Vec<String>,
    /// Authority boundary enforced on the row.
    pub authority: CompanionAuthority,
    /// Desktop handoff truth.
    pub desktop_handoff: DesktopHandoffTruth,
    /// Projection destinations.
    pub projection: CompanionProjection,
    /// Accessibility evidence refs.
    #[serde(default)]
    pub accessibility_refs: Vec<String>,
    /// Privacy/security review refs.
    #[serde(default)]
    pub privacy_refs: Vec<String>,
    /// Support/export packet refs.
    #[serde(default)]
    pub support_export_refs: Vec<String>,
    /// Reviewable reason this row carries its posture.
    pub rationale: String,
}

impl CompanionSurfaceRow {
    /// True when this row renders at or above the Stable cutline.
    pub fn renders_stable(&self) -> bool {
        self.displayed_label.is_at_or_above_cutline()
    }

    /// True when the row carries a captured, current proof packet.
    pub fn has_green_packet(&self) -> bool {
        self.qualification_packet.as_ref().is_some_and(|packet| {
            packet.has_capture() && packet.slo_state == FreshnessSloState::Current
        })
    }

    /// True when every claimed row has explicit narrow scope and unsupported operations.
    pub fn has_scope_truth(&self) -> bool {
        !self.scopes.is_empty()
            && !self.unsupported_actions.is_empty()
            && !self.desktop_continuation.trim().is_empty()
    }

    /// True when stale/offline/cache state has visible cues before action.
    pub fn has_freshness_truth(&self) -> bool {
        !self.freshness.requires_visible_cue() || !self.freshness_cues.is_empty()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompanionQualificationSummary {
    /// Total promoted-build rows.
    pub promoted_surface_count: usize,
    /// Rows rendering at Stable.
    pub stable_surface_count: usize,
    /// Rows narrowed below Stable.
    pub narrowed_surface_count: usize,
    /// Stable rows with green packets.
    pub green_packet_count: usize,
    /// Rows carrying preview/labs/unsupported labels.
    pub preview_or_labs_count: usize,
}

/// Canonical browser/mobile companion qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BrowserMobileCompanionSurfaceQualification {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Human-readable release document.
    pub release_doc_ref: String,
    /// User-facing help projection.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<CompanionSurfaceRow>,
    /// Summary counts.
    pub summary: CompanionQualificationSummary,
}

impl BrowserMobileCompanionSurfaceQualification {
    /// Returns rows rendered at Stable.
    pub fn stable_surfaces(&self) -> Vec<&CompanionSurfaceRow> {
        self.surfaces
            .iter()
            .filter(|surface| surface.renders_stable())
            .collect()
    }

    /// Returns rows narrowed below Stable.
    pub fn narrowed_surfaces(&self) -> Vec<&CompanionSurfaceRow> {
        self.surfaces
            .iter()
            .filter(|surface| !surface.renders_stable())
            .collect()
    }

    /// Recomputes summary counts from row state.
    pub fn computed_summary(&self) -> CompanionQualificationSummary {
        let promoted: Vec<&CompanionSurfaceRow> = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .collect();
        CompanionQualificationSummary {
            promoted_surface_count: promoted.len(),
            stable_surface_count: promoted
                .iter()
                .filter(|surface| surface.renders_stable())
                .count(),
            narrowed_surface_count: promoted
                .iter()
                .filter(|surface| !surface.renders_stable())
                .count(),
            green_packet_count: promoted
                .iter()
                .filter(|surface| surface.renders_stable() && surface.has_green_packet())
                .count(),
            preview_or_labs_count: promoted
                .iter()
                .filter(|surface| surface.visible_label.is_narrow())
                .count(),
        }
    }

    /// Validates structural invariants that do not depend on wall-clock arithmetic.
    pub fn validate(&self) -> Vec<CompanionQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_SCHEMA_VERSION {
            violations.push(CompanionQualificationViolation::SchemaVersion {
                expected: BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_RECORD_KIND {
            violations.push(CompanionQualificationViolation::RecordKind {
                expected: BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_RECORD_KIND.to_string(),
                actual: self.record_kind.clone(),
            });
        }

        let mut ids = BTreeSet::new();
        for surface in &self.surfaces {
            if !ids.insert(surface.surface_id.clone()) {
                violations.push(CompanionQualificationViolation::DuplicateSurfaceId {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.displayed_label.rank() > surface.claim_label.rank() {
                violations.push(CompanionQualificationViolation::DisplayedWiderThanClaim {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.promoted_build_surface
                && surface.renders_stable()
                && !surface.has_green_packet()
            {
                violations.push(
                    CompanionQualificationViolation::StableSurfaceWithoutGreenPacket {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable() && !surface.owner_signoff.signed_off {
                violations.push(
                    CompanionQualificationViolation::StableSurfaceMissingOwnerSignoff {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable()
                && surface.visible_label != CompanionVisibleLabel::StableCompanion
            {
                violations.push(
                    CompanionQualificationViolation::StableSurfaceLacksStableLabel {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if !surface.has_scope_truth() {
                violations.push(CompanionQualificationViolation::MissingScopeTruth {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if !surface.has_freshness_truth() {
                violations.push(CompanionQualificationViolation::MissingFreshnessCue {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if !surface.authority.is_narrow() {
                violations.push(CompanionQualificationViolation::AuthorityOverclaim {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if !surface.desktop_handoff.complete() {
                violations.push(CompanionQualificationViolation::IncompleteDesktopHandoff {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.renders_stable() && !surface.projection.complete() {
                violations.push(CompanionQualificationViolation::IncompleteProjection {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.renders_stable()
                && (surface.accessibility_refs.is_empty()
                    || surface.privacy_refs.is_empty()
                    || surface.support_export_refs.is_empty())
            {
                violations.push(CompanionQualificationViolation::MissingValidationEvidence {
                    surface_id: surface.surface_id.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(CompanionQualificationViolation::SummaryMismatch {
                expected: self.computed_summary(),
                actual: self.summary.clone(),
            });
        }

        violations
    }
}

/// Validation error for the browser/mobile companion qualification packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompanionQualificationViolation {
    /// Packet schema version differs from the supported version.
    SchemaVersion { expected: u32, actual: u32 },
    /// Packet record kind is not the companion qualification record kind.
    RecordKind { expected: String, actual: String },
    /// A surface id appears more than once.
    DuplicateSurfaceId { surface_id: String },
    /// The displayed label is wider than the claim label.
    DisplayedWiderThanClaim { surface_id: String },
    /// Stable row lacks a current captured proof packet.
    StableSurfaceWithoutGreenPacket { surface_id: String },
    /// Stable row lacks owner signoff.
    StableSurfaceMissingOwnerSignoff { surface_id: String },
    /// Stable row does not render the stable companion label.
    StableSurfaceLacksStableLabel { surface_id: String },
    /// Row lacks explicit scopes, unsupported actions, or desktop continuation.
    MissingScopeTruth { surface_id: String },
    /// Row lacks stale/offline/cache cueing for a non-live state.
    MissingFreshnessCue { surface_id: String },
    /// Row implies authority wider than a companion client may hold.
    AuthorityOverclaim { surface_id: String },
    /// Row lacks exact target, route, return-anchor, tenant, policy, or expiry truth.
    IncompleteDesktopHandoff { surface_id: String },
    /// Stable row is not projected into docs/help/About/support/public claims.
    IncompleteProjection { surface_id: String },
    /// Stable row lacks accessibility, privacy, or support-export validation refs.
    MissingValidationEvidence { surface_id: String },
    /// Summary block has drifted from row state.
    SummaryMismatch {
        expected: CompanionQualificationSummary,
        actual: CompanionQualificationSummary,
    },
}

impl fmt::Display for CompanionQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Parse the checked-in browser/mobile companion qualification packet.
pub fn current_browser_mobile_companion_surface_qualification(
) -> Result<BrowserMobileCompanionSurfaceQualification, Box<dyn Error + Send + Sync>> {
    Ok(serde_json::from_str(
        BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_JSON,
    )?)
}
