//! Control-plane-versus-data-plane outage taxonomy and degraded-state packets.
//!
//! This module is the in-product continuity lane that describes optional-service
//! impairment **by lane** — identity/policy, registry/updates/docs,
//! collaboration, remote control plane, AI gateway, and telemetry/support —
//! without ever conflating it with local editing failure. It sits alongside the
//! frozen continuity-claim matrix
//! ([`crate::m5_locality_tenant_keymode_and_drill_matrix`]) and reuses its
//! canonical control-plane/data-plane vocabulary ([`PlaneImpairmentClass`]) and
//! qualification ladder ([`ContinuityClaimQualificationClass`]) so the product
//! has exactly one outage-plane vocabulary.
//!
//! For each claimed optional-service family it produces one
//! [`ServiceOutageDescriptor`] a person can read directly in the product and in
//! support evidence. The descriptor answers the same questions everywhere:
//!
//! 1. Which optional-service lane is impaired, which plane does the impairment
//!    sit on (control plane or managed data plane), and how severe is it?
//! 2. What is the typed degraded state, and what narrower fallback takes over?
//! 3. What still works locally right now — editing, save, search, and version
//!    control — so local-first credibility is proven, not implied?
//!
//! The same descriptor is projected onto every claimed surface (desktop activity
//! center, CLI/headless explain, service-health, support-center export,
//! shiproom, and docs/public-truth) through an
//! [`ServiceOutageSurfaceProjection`], so the exact outage and local-core
//! vocabulary stays byte-identical everywhere instead of drifting per surface.
//!
//! Two guardrails are load-bearing:
//!
//! - An optional-service outage **may not conflate itself with local editing
//!   failure**: it may not flip a global "IDE down" state and may not mark local
//!   editing, save, search, or version control unavailable while a managed lane
//!   is the only thing impaired. A packet that does so **fails closed** — the
//!   claim is withdrawn and the typed degraded state is recorded as a misclaim —
//!   rather than being quietly published.
//! - Every surface renders the same canonical outage and local-core lines, so a
//!   support bundle, a shiproom card, and a docs page describe the same outage
//!   the same way.
//!
//! The packet is metadata-only. It carries closed-vocabulary tokens, export-safe
//! plain-language labels, UTC timestamps, and opaque refs. Raw provider payloads,
//! raw incident bodies, hostnames, and secret material never cross this boundary.

use serde::{Deserialize, Serialize};

use crate::m5_locality_tenant_keymode_and_drill_matrix::{
    ContinuityClaimQualificationClass, PlaneImpairmentClass,
};

#[cfg(test)]
mod tests;

/// Schema version carried on every record in this module.
pub const OUTAGE_TAXONOMY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const OUTAGE_TAXONOMY_SHARED_CONTRACT_REF: &str =
    "continuity:m5_control_plane_vs_data_plane_outage:v1";

/// Record-kind tag for [`ServiceOutageTaxonomyPage`] payloads.
pub const OUTAGE_TAXONOMY_PAGE_RECORD_KIND: &str = "outage_taxonomy_page_record";

/// Record-kind tag for [`ServiceOutageTaxonomySummary`] payloads.
pub const OUTAGE_TAXONOMY_SUMMARY_RECORD_KIND: &str = "outage_taxonomy_summary_record";

/// Record-kind tag for [`ServiceOutageDescriptor`] payloads.
pub const SERVICE_OUTAGE_DESCRIPTOR_RECORD_KIND: &str = "service_outage_descriptor_record";

/// Record-kind tag for [`ServiceOutageSurfaceProjection`] payloads.
pub const OUTAGE_SURFACE_PROJECTION_RECORD_KIND: &str = "outage_surface_projection_record";

/// Record-kind tag for [`ServiceOutageOutcome`] payloads.
pub const SERVICE_OUTAGE_OUTCOME_RECORD_KIND: &str = "service_outage_outcome_record";

/// Record-kind tag for [`OutageTaxonomyDefect`] payloads.
pub const OUTAGE_TAXONOMY_DEFECT_RECORD_KIND: &str = "outage_taxonomy_defect_record";

/// Record-kind tag for [`ServiceOutageTaxonomySupportExport`] payloads.
pub const OUTAGE_TAXONOMY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "outage_taxonomy_support_export_record";

/// Repo-relative path of the canonical reviewer doc for this lane.
pub const OUTAGE_TAXONOMY_DOC_REF: &str =
    "docs/m5/continuity/control-plane-vs-data-plane-degradation.md";

/// Repo-relative path of the checked-in artifact for this lane.
pub const OUTAGE_TAXONOMY_ARTIFACT_REF: &str =
    "artifacts/m5/continuity/control_plane_vs_data_plane_degradation.md";

/// Repo-relative path of the canonical JSON schema for this lane.
pub const OUTAGE_TAXONOMY_SCHEMA_REF: &str =
    "schemas/continuity/control_vs_data_plane_packet.schema.json";

/// One claimed optional-service family that can be impaired independently of the
/// local editing core.
///
/// These are exactly the lanes the outage taxonomy must describe distinctly so
/// that optional-service impairment is never reported as a local editing
/// failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OptionalServiceFamily {
    /// Identity, sign-in, and policy evaluation.
    IdentityPolicy,
    /// Extension/registry catalog, update channel, and docs packs.
    RegistryUpdatesDocs,
    /// Real-time collaboration, presence, and sharing.
    Collaboration,
    /// Remote attach, tunnel, and managed control-plane operations.
    RemoteControlPlane,
    /// Managed AI gateway and model inference.
    AiGateway,
    /// Telemetry, diagnostics upload, and support services.
    TelemetrySupport,
}

impl OptionalServiceFamily {
    /// Every optional-service family in canonical order.
    pub const ALL: [OptionalServiceFamily; 6] = [
        Self::IdentityPolicy,
        Self::RegistryUpdatesDocs,
        Self::Collaboration,
        Self::RemoteControlPlane,
        Self::AiGateway,
        Self::TelemetrySupport,
    ];

    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityPolicy => "identity_policy",
            Self::RegistryUpdatesDocs => "registry_updates_docs",
            Self::Collaboration => "collaboration",
            Self::RemoteControlPlane => "remote_control_plane",
            Self::AiGateway => "ai_gateway",
            Self::TelemetrySupport => "telemetry_support",
        }
    }

    /// Plain-language label naming the optional-service lane.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::IdentityPolicy => "identity and policy",
            Self::RegistryUpdatesDocs => "registry, updates, and docs",
            Self::Collaboration => "collaboration",
            Self::RemoteControlPlane => "remote control plane",
            Self::AiGateway => "AI gateway",
            Self::TelemetrySupport => "telemetry and support",
        }
    }
}

/// Operational severity of an optional-service lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpairmentSeverityClass {
    /// The lane is fully operational.
    Operational,
    /// The lane is partially impaired and running on its narrower fallback.
    Degraded,
    /// The lane is fully unavailable; the fallback is the only path.
    Unavailable,
    /// The lane is reconnecting and reconciling after an impairment.
    Recovering,
}

impl ImpairmentSeverityClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Operational => "operational",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
            Self::Recovering => "recovering",
        }
    }

    /// Plain-language summary of the severity.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::Operational => "operational",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
            Self::Recovering => "recovering",
        }
    }

    /// True when the lane is impaired in any way (not fully operational).
    pub const fn is_impaired(self) -> bool {
        !matches!(self, Self::Operational)
    }
}

/// Typed narrower fallback an impaired optional-service lane runs on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedFallbackClass {
    /// No fallback is needed; the lane is operational.
    NoneNeeded,
    /// Actions queue locally and reconcile on reconnect.
    QueueAndReconcile,
    /// The last-known cached catalog/docs results are served read-only.
    ServeFromCache,
    /// AI falls back to a local model or to manual editing.
    LocalModelOrManualFallback,
    /// A cached policy decision drives a read-only session.
    CachedPolicyReadOnly,
    /// Telemetry/support data is buffered locally and shipped later.
    BufferLocallyAndShipLater,
    /// The managed control plane is unreachable; only local-core work continues.
    FailClosedLocalCoreOnly,
    /// An impaired lane has not declared a fallback; the claim narrows.
    NotDeclared,
}

impl DegradedFallbackClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneNeeded => "none_needed",
            Self::QueueAndReconcile => "queue_and_reconcile",
            Self::ServeFromCache => "serve_from_cache",
            Self::LocalModelOrManualFallback => "local_model_or_manual_fallback",
            Self::CachedPolicyReadOnly => "cached_policy_read_only",
            Self::BufferLocallyAndShipLater => "buffer_locally_and_ship_later",
            Self::FailClosedLocalCoreOnly => "fail_closed_local_core_only",
            Self::NotDeclared => "not_declared",
        }
    }

    /// Plain-language summary of the fallback.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::NoneNeeded => "no fallback needed",
            Self::QueueAndReconcile => "queue locally and reconcile on reconnect",
            Self::ServeFromCache => "serve the last cached results read-only",
            Self::LocalModelOrManualFallback => "fall back to a local model or manual editing",
            Self::CachedPolicyReadOnly => "run a read-only session from cached policy",
            Self::BufferLocallyAndShipLater => "buffer locally and ship when the lane returns",
            Self::FailClosedLocalCoreOnly => "fail closed and keep local-core work only",
            Self::NotDeclared => "not declared",
        }
    }

    /// True when this fallback is an actual narrower path (not absent or empty).
    pub const fn is_active(self) -> bool {
        !matches!(self, Self::NoneNeeded | Self::NotDeclared)
    }
}

/// Freshness state of an outage packet's evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutageEvidenceStateClass {
    /// Evidence is current.
    Current,
    /// Evidence is stale but within an approved grace window.
    StaleWithinGrace,
    /// Evidence is stale enough that a fresh refresh is required.
    StaleNeedsRefresh,
    /// No outage evidence is present.
    Missing,
}

impl OutageEvidenceStateClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::StaleWithinGrace => "stale_within_grace",
            Self::StaleNeedsRefresh => "stale_needs_refresh",
            Self::Missing => "missing",
        }
    }

    /// Plain-language summary of the evidence state.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::StaleWithinGrace => "stale within grace",
            Self::StaleNeedsRefresh => "stale, needs refresh",
            Self::Missing => "missing",
        }
    }

    /// True when the evidence is fresh enough to leave the claim stable.
    pub const fn is_acceptable(self) -> bool {
        matches!(self, Self::Current | Self::StaleWithinGrace)
    }
}

/// Typed degraded state computed for an outage packet.
///
/// The whole point of this enum is to keep optional-service impairment with
/// preserved local-core work distinct from a genuine local editing failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutageDegradedStateClass {
    /// The lane is operational; no degraded state.
    Operational,
    /// The control plane is impaired while local-core work is preserved.
    ControlPlaneImpairedLocalCorePreserved,
    /// A managed data plane is impaired while local-core work is preserved.
    ManagedDataPlaneImpairedLocalCorePreserved,
    /// Both managed planes are impaired while local-core work is preserved.
    BothManagedPlanesImpairedLocalCorePreserved,
    /// The packet wrongly conflates an optional-service outage with local-core
    /// failure; surfaced honestly so reviewers can see the misclaim.
    LocalCoreConflatedMisclaim,
}

impl OutageDegradedStateClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Operational => "operational",
            Self::ControlPlaneImpairedLocalCorePreserved => {
                "control_plane_impaired_local_core_preserved"
            }
            Self::ManagedDataPlaneImpairedLocalCorePreserved => {
                "managed_data_plane_impaired_local_core_preserved"
            }
            Self::BothManagedPlanesImpairedLocalCorePreserved => {
                "both_managed_planes_impaired_local_core_preserved"
            }
            Self::LocalCoreConflatedMisclaim => "local_core_conflated_misclaim",
        }
    }

    /// Plain-language summary of the degraded state.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::Operational => "operational",
            Self::ControlPlaneImpairedLocalCorePreserved => {
                "control plane impaired; local-core work preserved"
            }
            Self::ManagedDataPlaneImpairedLocalCorePreserved => {
                "managed data plane impaired; local-core work preserved"
            }
            Self::BothManagedPlanesImpairedLocalCorePreserved => {
                "both managed planes impaired; local-core work preserved"
            }
            Self::LocalCoreConflatedMisclaim => {
                "misclaim: optional-service outage conflated with local editing failure"
            }
        }
    }
}

/// Surface an outage descriptor is projected onto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutageSurfaceClass {
    /// The desktop activity center / in-product banner.
    Desktop,
    /// The CLI / headless explain surface.
    CliHeadlessExplain,
    /// The service-health surface.
    ServiceHealth,
    /// A support-center export packet.
    SupportExport,
    /// The shiproom readiness dashboard.
    Shiproom,
    /// Docs and public-truth pages.
    DocsPublicTruth,
}

impl OutageSurfaceClass {
    /// Every surface in canonical projection order.
    pub const ALL: [OutageSurfaceClass; 6] = [
        Self::Desktop,
        Self::CliHeadlessExplain,
        Self::ServiceHealth,
        Self::SupportExport,
        Self::Shiproom,
        Self::DocsPublicTruth,
    ];

    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::CliHeadlessExplain => "cli_headless_explain",
            Self::ServiceHealth => "service_health",
            Self::SupportExport => "support_export",
            Self::Shiproom => "shiproom",
            Self::DocsPublicTruth => "docs_public_truth",
        }
    }
}

/// Typed reason an outage packet narrowed below stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutageNarrowReasonClass {
    /// No narrowing is active.
    NotNarrowed,
    /// An optional-service outage conflates itself with local-core failure.
    LocalCoreConflated,
    /// An impaired lane does not name a narrower fallback.
    FallbackUndeclared,
    /// An operational lane claims an active fallback or an impaired local-core.
    OperationalStateInconsistent,
    /// An outage packet has no evidence reference.
    OutageEvidenceMissing,
    /// An outage packet's evidence is stale and needs a refresh.
    OutageEvidenceStale,
    /// A surface renders different outage or local-core wording than the descriptor.
    OutageVocabularyDrift,
    /// A packet is not projected onto every required surface.
    SurfaceReuseIncomplete,
    /// The taxonomy does not classify both a control-plane and a data-plane outage.
    PlaneDistinctionMissing,
    /// The taxonomy does not cover every optional-service family.
    FamilyCoverageIncomplete,
}

impl OutageNarrowReasonClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::LocalCoreConflated => "local_core_conflated",
            Self::FallbackUndeclared => "fallback_undeclared",
            Self::OperationalStateInconsistent => "operational_state_inconsistent",
            Self::OutageEvidenceMissing => "outage_evidence_missing",
            Self::OutageEvidenceStale => "outage_evidence_stale",
            Self::OutageVocabularyDrift => "outage_vocabulary_drift",
            Self::SurfaceReuseIncomplete => "surface_reuse_incomplete",
            Self::PlaneDistinctionMissing => "plane_distinction_missing",
            Self::FamilyCoverageIncomplete => "family_coverage_incomplete",
        }
    }

    /// True when this reason withdraws the claim immediately (fails closed).
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(self, Self::LocalCoreConflated)
    }

    /// True when this reason holds the claim at preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(
            self,
            Self::OperationalStateInconsistent
                | Self::OutageEvidenceStale
                | Self::OutageVocabularyDrift
        )
    }
}

/// Derives a qualification from the outage narrow reasons present.
fn qualification_from_reasons<'a>(
    reasons: impl IntoIterator<Item = &'a OutageNarrowReasonClass>,
) -> ContinuityClaimQualificationClass {
    let mut saw_any = false;
    let mut saw_preview = false;
    for reason in reasons {
        saw_any = true;
        if reason.is_withdrawal_reason() {
            return ContinuityClaimQualificationClass::Withdrawn;
        }
        if reason.is_preview_reason() {
            saw_preview = true;
        }
    }
    if saw_preview {
        ContinuityClaimQualificationClass::Preview
    } else if saw_any {
        ContinuityClaimQualificationClass::Beta
    } else {
        ContinuityClaimQualificationClass::Stable
    }
}

/// What still works locally while an optional-service lane is impaired.
///
/// These four capabilities are the local editing core. An optional-service
/// outage must keep all four available; a packet that marks any of them
/// unavailable is conflating a managed-lane outage with a local editing failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalCoreContinuity {
    /// True when local editing still works.
    pub editing_available: bool,
    /// True when local save / autosave still works.
    pub save_available: bool,
    /// True when local search still works.
    pub search_available: bool,
    /// True when local version control (Git) still works.
    pub version_control_available: bool,
}

impl LocalCoreContinuity {
    /// A fully preserved local-core (every capability available).
    pub const fn fully_preserved() -> Self {
        Self {
            editing_available: true,
            save_available: true,
            search_available: true,
            version_control_available: true,
        }
    }

    /// True when every local-core capability is still available.
    pub const fn all_preserved(&self) -> bool {
        self.editing_available
            && self.save_available
            && self.search_available
            && self.version_control_available
    }

    /// Canonical one-line summary of what still works locally.
    pub fn summary_line(&self) -> String {
        if self.all_preserved() {
            "Local editing, save, search, and version control all keep working.".to_owned()
        } else {
            format!(
                "Local continuity impaired: editing {}, save {}, search {}, version control {}.",
                yes_no(self.editing_available),
                yes_no(self.save_available),
                yes_no(self.search_available),
                yes_no(self.version_control_available),
            )
        }
    }
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "available"
    } else {
        "unavailable"
    }
}

/// One claimed optional-service outage packet decorated with the facts needed to
/// build its degraded-state descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceOutageEntry {
    /// Opaque packet identifier.
    pub packet_id: String,
    /// Optional-service family this packet describes.
    pub family: OptionalServiceFamily,
    /// Stable token for [`Self::family`].
    pub family_token: String,
    /// Reviewable label naming the lane.
    pub lane_label: String,
    /// Plane(s) the impairment sits on.
    pub impaired_plane: PlaneImpairmentClass,
    /// Stable token for [`Self::impaired_plane`].
    pub impaired_plane_token: String,
    /// Operational severity of the lane.
    pub severity: ImpairmentSeverityClass,
    /// Stable token for [`Self::severity`].
    pub severity_token: String,
    /// Narrower fallback the lane runs on while impaired.
    pub fallback: DegradedFallbackClass,
    /// Stable token for [`Self::fallback`].
    pub fallback_token: String,
    /// What still works locally during this outage.
    pub local_core: LocalCoreContinuity,
    /// True when this packet flips a global "IDE down" state. Must stay false for
    /// an optional-service outage while local-core work is safe.
    pub sets_global_ide_down: bool,
    /// Freshness of the outage evidence.
    pub evidence_state: OutageEvidenceStateClass,
    /// Stable token for [`Self::evidence_state`].
    pub evidence_state_token: String,
    /// Opaque ref to the outage evidence; never a raw incident body.
    pub outage_evidence_ref: String,
    /// Export-safe explanation of the outage and its fallback.
    pub impairment_note: String,
    /// Surfaces this packet is projected onto.
    pub projected_surfaces: Vec<OutageSurfaceClass>,
}

impl ServiceOutageEntry {
    /// Builds an outage entry for an optional-service family.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        packet_id: impl Into<String>,
        family: OptionalServiceFamily,
        lane_label: impl Into<String>,
        impaired_plane: PlaneImpairmentClass,
        severity: ImpairmentSeverityClass,
        fallback: DegradedFallbackClass,
        local_core: LocalCoreContinuity,
        sets_global_ide_down: bool,
        evidence_state: OutageEvidenceStateClass,
        outage_evidence_ref: impl Into<String>,
        impairment_note: impl Into<String>,
        projected_surfaces: Vec<OutageSurfaceClass>,
    ) -> Self {
        Self {
            packet_id: packet_id.into(),
            family,
            family_token: family.as_str().to_owned(),
            lane_label: lane_label.into(),
            impaired_plane,
            impaired_plane_token: impaired_plane.as_str().to_owned(),
            severity,
            severity_token: severity.as_str().to_owned(),
            fallback,
            fallback_token: fallback.as_str().to_owned(),
            local_core,
            sets_global_ide_down,
            evidence_state,
            evidence_state_token: evidence_state.as_str().to_owned(),
            outage_evidence_ref: outage_evidence_ref.into(),
            impairment_note: impairment_note.into(),
            projected_surfaces,
        }
    }

    /// Surfaces this packet is required to reach (every surface).
    pub fn required_surfaces(&self) -> &'static [OutageSurfaceClass] {
        &OutageSurfaceClass::ALL
    }

    /// True when this packet conflates an optional-service outage with a local
    /// editing failure.
    ///
    /// This is the load-bearing guardrail: an optional-service outage may neither
    /// flip a global "IDE down" state nor mark any local-core capability
    /// unavailable. Either is a misclaim that fails the packet closed.
    pub fn conflates_local_core(&self) -> bool {
        self.sets_global_ide_down || !self.local_core.all_preserved()
    }

    /// True when local-core continuity survives this outage.
    pub const fn local_core_preserved(&self) -> bool {
        self.local_core.all_preserved()
    }

    /// The typed degraded state this packet is in.
    pub fn degraded_state(&self) -> OutageDegradedStateClass {
        if self.conflates_local_core() {
            OutageDegradedStateClass::LocalCoreConflatedMisclaim
        } else if !self.severity.is_impaired() {
            OutageDegradedStateClass::Operational
        } else if self.impaired_plane == PlaneImpairmentClass::BothPlanes {
            OutageDegradedStateClass::BothManagedPlanesImpairedLocalCorePreserved
        } else if self.impaired_plane.covers_control_plane() {
            OutageDegradedStateClass::ControlPlaneImpairedLocalCorePreserved
        } else {
            OutageDegradedStateClass::ManagedDataPlaneImpairedLocalCorePreserved
        }
    }

    /// True when an operational lane carries an inconsistent impaired posture.
    fn operational_inconsistent(&self) -> bool {
        !self.severity.is_impaired()
            && (self.fallback.is_active() || !self.local_core.all_preserved())
    }
}

/// Plain-language degraded-state descriptor for one optional-service outage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceOutageDescriptor {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque descriptor identifier.
    pub descriptor_id: String,
    /// Packet this descriptor describes.
    pub packet_id: String,
    /// Stable token for the optional-service family.
    pub family_token: String,
    /// Plain-language optional-service family.
    pub family_plain: String,
    /// Reviewable label naming the lane.
    pub lane_label: String,
    /// Stable token for the impaired plane.
    pub impaired_plane_token: String,
    /// Plain-language impaired plane.
    pub impaired_plane_plain: String,
    /// Stable token for the severity.
    pub severity_token: String,
    /// Plain-language severity.
    pub severity_plain: String,
    /// Stable token for the narrower fallback.
    pub fallback_token: String,
    /// Plain-language narrower fallback.
    pub fallback_plain: String,
    /// Stable token for the typed degraded state.
    pub degraded_state_token: String,
    /// Plain-language typed degraded state.
    pub degraded_state_plain: String,
    /// True when local editing still works.
    pub local_editing_available: bool,
    /// True when local save still works.
    pub local_save_available: bool,
    /// True when local search still works.
    pub local_search_available: bool,
    /// True when local version control still works.
    pub local_version_control_available: bool,
    /// True when every local-core capability survives the outage.
    pub local_core_preserved: bool,
    /// True when this packet flips a global "IDE down" state.
    pub sets_global_ide_down: bool,
    /// True when this packet conflates the outage with a local editing failure.
    pub conflates_local_core: bool,
    /// Canonical one-line outage summary reused by every surface projection.
    pub outage_summary_line: String,
    /// Canonical one-line local-core summary reused by every surface projection.
    pub local_core_line: String,
    /// Export-safe explanation of the outage and its fallback.
    pub impairment_note: String,
}

impl ServiceOutageDescriptor {
    /// Builds an outage descriptor from a decorated entry.
    pub fn from_entry(entry: &ServiceOutageEntry) -> Self {
        Self {
            record_kind: SERVICE_OUTAGE_DESCRIPTOR_RECORD_KIND.to_owned(),
            schema_version: OUTAGE_TAXONOMY_SCHEMA_VERSION,
            shared_contract_ref: OUTAGE_TAXONOMY_SHARED_CONTRACT_REF.to_owned(),
            descriptor_id: format!("continuity:service-outage-descriptor:{}", entry.packet_id),
            packet_id: entry.packet_id.clone(),
            family_token: entry.family_token.clone(),
            family_plain: entry.family.plain().to_owned(),
            lane_label: entry.lane_label.clone(),
            impaired_plane_token: entry.impaired_plane_token.clone(),
            impaired_plane_plain: plane_plain(entry.impaired_plane).to_owned(),
            severity_token: entry.severity_token.clone(),
            severity_plain: entry.severity.plain().to_owned(),
            fallback_token: entry.fallback_token.clone(),
            fallback_plain: entry.fallback.plain().to_owned(),
            degraded_state_token: entry.degraded_state().as_str().to_owned(),
            degraded_state_plain: entry.degraded_state().plain().to_owned(),
            local_editing_available: entry.local_core.editing_available,
            local_save_available: entry.local_core.save_available,
            local_search_available: entry.local_core.search_available,
            local_version_control_available: entry.local_core.version_control_available,
            local_core_preserved: entry.local_core_preserved(),
            sets_global_ide_down: entry.sets_global_ide_down,
            conflates_local_core: entry.conflates_local_core(),
            outage_summary_line: outage_summary_line(entry),
            local_core_line: entry.local_core.summary_line(),
            impairment_note: entry.impairment_note.clone(),
        }
    }
}

/// One surface rendering of an outage descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceOutageSurfaceProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Surface this projection renders on.
    pub surface: OutageSurfaceClass,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Packet this projection describes.
    pub packet_id: String,
    /// Descriptor id rendered on this surface.
    pub descriptor_id: String,
    /// Outage summary line rendered on this surface.
    pub outage_summary_line: String,
    /// Local-core summary line rendered on this surface.
    pub local_core_line: String,
}

/// Per-packet verdict joining a packet to its computed qualification and reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceOutageOutcome {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Packet this outcome describes.
    pub packet_id: String,
    /// Stable token for the optional-service family.
    pub family_token: String,
    /// Computed qualification token for the packet.
    pub qualification_token: String,
    /// True when the packet narrowed below stable.
    pub narrowed: bool,
    /// True when the packet's claim is withheld entirely.
    pub claim_withheld: bool,
    /// True when the packet conflates the outage with a local editing failure.
    pub conflates_local_core: bool,
    /// True when local-core continuity survives this outage.
    pub local_core_preserved: bool,
    /// Stable token for the typed degraded state.
    pub degraded_state_token: String,
    /// Stable narrow-reason tokens that applied to the packet.
    pub narrow_reason_tokens: Vec<String>,
}

/// Typed defect emitted by the outage-taxonomy audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutageTaxonomyDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque defect identifier.
    pub defect_id: String,
    /// Typed narrow reason.
    pub narrow_reason: OutageNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Opaque source packet id or taxonomy concern that triggered the defect.
    pub source: String,
    /// Export-safe explanation of the defect.
    pub note: String,
}

impl OutageTaxonomyDefect {
    fn new(
        narrow_reason: OutageNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source = source.into();
        Self {
            record_kind: OUTAGE_TAXONOMY_DEFECT_RECORD_KIND.to_owned(),
            schema_version: OUTAGE_TAXONOMY_SCHEMA_VERSION,
            shared_contract_ref: OUTAGE_TAXONOMY_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "continuity:defect:outage-taxonomy:{}:{}",
                narrow_reason.as_str(),
                source
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source,
            note: note.into(),
        }
    }
}

/// Aggregate summary for an outage-taxonomy page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceOutageTaxonomySummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Overall qualification for the page.
    pub overall_qualification_token: String,
    /// Number of outage packets.
    pub packet_count: usize,
    /// Number of distinct optional-service families covered.
    pub family_count: usize,
    /// Number of packets whose impairment covers the control plane.
    pub control_plane_impaired_count: usize,
    /// Number of packets whose impairment covers the data plane.
    pub data_plane_impaired_count: usize,
    /// Number of operational (unimpaired) packets.
    pub operational_count: usize,
    /// Number of degraded packets.
    pub degraded_count: usize,
    /// Number of unavailable packets.
    pub unavailable_count: usize,
    /// Number of recovering packets.
    pub recovering_count: usize,
    /// Number of packets that preserve local-core continuity.
    pub local_core_preserved_count: usize,
    /// Number of packets that narrowed below stable.
    pub narrowed_count: usize,
    /// Number of packets whose claim is withheld.
    pub withdrawn_count: usize,
    /// Number of surface projections emitted.
    pub surface_projection_count: usize,
    /// True when every surface renders the same outage/local-core vocabulary.
    pub vocabulary_consistent: bool,
    /// True when every packet preserves local-core continuity.
    pub all_local_core_preserved: bool,
    /// True when no packet flips a misleading global "IDE down" state.
    pub no_global_ide_down_misclaim: bool,
    /// True when the taxonomy covers every optional-service family.
    pub all_families_covered: bool,
    /// True when both a control-plane and a data-plane outage are classified.
    pub plane_distinction_present: bool,
    /// True when no raw provider payload is carried anywhere in the packet.
    pub raw_payloads_excluded: bool,
    /// Number of defects recorded for the page.
    pub defect_count: usize,
}

/// Full auditable input for the outage-taxonomy page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceOutageTaxonomyInput {
    /// Reviewable label for the page.
    pub input_label: String,
    /// Claimed outage packets.
    pub entries: Vec<ServiceOutageEntry>,
}

/// Canonical proof packet for the control-plane-versus-data-plane outage taxonomy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceOutageTaxonomyPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page identifier.
    pub page_id: String,
    /// Reviewable page label.
    pub page_label: String,
    /// UTC timestamp when the packet was generated.
    pub generated_at: String,
    /// Aggregate summary derived from the embedded input and defects.
    pub summary: ServiceOutageTaxonomySummary,
    /// Typed defects for the packet.
    pub defects: Vec<OutageTaxonomyDefect>,
    /// Degraded-state descriptors, one per packet.
    pub descriptors: Vec<ServiceOutageDescriptor>,
    /// Per-surface projections proving identical vocabulary across surfaces.
    pub surface_projections: Vec<ServiceOutageSurfaceProjection>,
    /// Per-packet verdicts joining each packet to its computed qualification.
    pub outcomes: Vec<ServiceOutageOutcome>,
    /// The audited input embedded as evidence.
    pub input: ServiceOutageTaxonomyInput,
}

impl ServiceOutageTaxonomyPage {
    /// Builds an outage-taxonomy page from the supplied input.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        input: ServiceOutageTaxonomyInput,
    ) -> Self {
        let descriptors: Vec<ServiceOutageDescriptor> = input
            .entries
            .iter()
            .map(ServiceOutageDescriptor::from_entry)
            .collect();
        let surface_projections = build_surface_projections(&input.entries);
        let defects = audit(&input, &surface_projections);
        let outcomes = build_outcomes(&input, &defects);
        let summary = build_summary(&input, &surface_projections, &outcomes, &defects);
        Self {
            record_kind: OUTAGE_TAXONOMY_PAGE_RECORD_KIND.to_owned(),
            schema_version: OUTAGE_TAXONOMY_SCHEMA_VERSION,
            shared_contract_ref: OUTAGE_TAXONOMY_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            defects,
            descriptors,
            surface_projections,
            outcomes,
            input,
        }
    }

    /// True when the page qualifies stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == ContinuityClaimQualificationClass::Stable.as_str()
    }

    /// True when every surface renders identical outage/local-core vocabulary.
    pub fn surfaces_share_vocabulary(&self) -> bool {
        self.summary.vocabulary_consistent
    }

    /// True when the taxonomy covers every optional-service family.
    pub fn covers_all_families(&self) -> bool {
        self.summary.all_families_covered
    }

    /// True when the taxonomy classifies both a control-plane and a data-plane outage.
    pub fn distinguishes_control_and_data_plane(&self) -> bool {
        self.summary.plane_distinction_present
    }

    /// Returns the descriptor for a packet id, if present.
    pub fn descriptor(&self, packet_id: &str) -> Option<&ServiceOutageDescriptor> {
        self.descriptors.iter().find(|d| d.packet_id == packet_id)
    }

    /// Returns the descriptor for an optional-service family, if present.
    pub fn descriptor_for_family(
        &self,
        family: OptionalServiceFamily,
    ) -> Option<&ServiceOutageDescriptor> {
        self.descriptors
            .iter()
            .find(|d| d.family_token == family.as_str())
    }

    /// Returns the computed outcome for a packet id, if present.
    pub fn outcome(&self, packet_id: &str) -> Option<&ServiceOutageOutcome> {
        self.outcomes.iter().find(|o| o.packet_id == packet_id)
    }

    /// Returns the descriptors that conflate an outage with local-core failure.
    pub fn conflating_descriptors(&self) -> Vec<&ServiceOutageDescriptor> {
        self.descriptors
            .iter()
            .filter(|d| d.conflates_local_core)
            .collect()
    }
}

/// Support-export wrapper for the outage-taxonomy page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceOutageTaxonomySupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export identifier.
    pub export_id: String,
    /// UTC timestamp when the export was produced.
    pub generated_at: String,
    /// The outage-taxonomy page embedded as evidence.
    pub page: ServiceOutageTaxonomyPage,
    /// Typed narrow reasons present in the embedded packet.
    pub narrow_reasons_present: Vec<OutageNarrowReasonClass>,
    /// True when raw provider payloads are excluded from this export.
    pub raw_payloads_excluded: bool,
}

impl ServiceOutageTaxonomySupportExport {
    /// Wraps an outage-taxonomy page inside a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: ServiceOutageTaxonomyPage,
    ) -> Self {
        let mut reasons: Vec<OutageNarrowReasonClass> = Vec::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
        }
        reasons.sort();
        Self {
            record_kind: OUTAGE_TAXONOMY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: OUTAGE_TAXONOMY_SCHEMA_VERSION,
            shared_contract_ref: OUTAGE_TAXONOMY_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            raw_payloads_excluded: true,
        }
    }
}

/// Re-runs the outage-taxonomy audit over a page, including its stored projections.
///
/// Unlike [`ServiceOutageTaxonomyPage::new`], this validates the page's stored
/// surface projections against freshly derived canonical lines, so a tampered
/// projection (one that renders different vocabulary than its descriptor) is
/// caught on re-validation.
pub fn audit_service_outage_taxonomy_page(
    page: &ServiceOutageTaxonomyPage,
) -> Vec<OutageTaxonomyDefect> {
    audit(&page.input, &page.surface_projections)
}

/// Validates an outage-taxonomy page and returns `Ok(())` when the audit is clean.
pub fn validate_service_outage_taxonomy_page(
    page: &ServiceOutageTaxonomyPage,
) -> Result<(), Vec<OutageTaxonomyDefect>> {
    let defects = audit_service_outage_taxonomy_page(page);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Returns the seeded stable outage-taxonomy page.
pub fn seeded_service_outage_taxonomy_page() -> ServiceOutageTaxonomyPage {
    ServiceOutageTaxonomyPage::new(
        "continuity:outage-taxonomy:seeded",
        "Control-plane-versus-data-plane outage taxonomy",
        "2026-06-01T00:00:00Z",
        seeded_service_outage_taxonomy_input(),
    )
}

/// Returns the seeded input used by the canonical outage-taxonomy page.
///
/// The seeded page exercises one simulated impairment for every optional-service
/// family — identity/policy, registry/updates/docs, collaboration, remote
/// control plane, AI gateway, and telemetry/support — across a mix of control
/// and data planes and a mix of degraded, unavailable, and recovering
/// severities. Every packet preserves full local-core continuity and none flips
/// a global "IDE down" state, so the page qualifies stable.
pub fn seeded_service_outage_taxonomy_input() -> ServiceOutageTaxonomyInput {
    let all = OutageSurfaceClass::ALL.to_vec();
    let entries = vec![
        ServiceOutageEntry::new(
            "continuity-outage:identity-policy",
            OptionalServiceFamily::IdentityPolicy,
            "Identity and policy",
            PlaneImpairmentClass::ControlPlaneImpairment,
            ImpairmentSeverityClass::Degraded,
            DegradedFallbackClass::CachedPolicyReadOnly,
            LocalCoreContinuity::fully_preserved(),
            false,
            OutageEvidenceStateClass::Current,
            "outage-evidence:identity-policy:2026-06-01",
            "Sign-in and policy refresh are paused; a cached policy decision keeps the session read-only. Local editing is unaffected.",
            all.clone(),
        ),
        ServiceOutageEntry::new(
            "continuity-outage:registry-updates-docs",
            OptionalServiceFamily::RegistryUpdatesDocs,
            "Registry, updates, and docs",
            PlaneImpairmentClass::ControlPlaneImpairment,
            ImpairmentSeverityClass::Degraded,
            DegradedFallbackClass::ServeFromCache,
            LocalCoreContinuity::fully_preserved(),
            false,
            OutageEvidenceStateClass::Current,
            "outage-evidence:registry-updates-docs:2026-06-01",
            "The extension catalog and docs packs serve the last cached results; new installs and updates wait until the lane returns.",
            all.clone(),
        ),
        ServiceOutageEntry::new(
            "continuity-outage:collaboration",
            OptionalServiceFamily::Collaboration,
            "Collaboration",
            PlaneImpairmentClass::DataPlaneImpairment,
            ImpairmentSeverityClass::Unavailable,
            DegradedFallbackClass::QueueAndReconcile,
            LocalCoreContinuity::fully_preserved(),
            false,
            OutageEvidenceStateClass::Current,
            "outage-evidence:collaboration:2026-05-28",
            "Real-time presence and sharing are unavailable; local edits queue and reconcile when the relay returns.",
            all.clone(),
        ),
        ServiceOutageEntry::new(
            "continuity-outage:remote-control-plane",
            OptionalServiceFamily::RemoteControlPlane,
            "Remote control plane",
            PlaneImpairmentClass::ControlPlaneImpairment,
            ImpairmentSeverityClass::Unavailable,
            DegradedFallbackClass::FailClosedLocalCoreOnly,
            LocalCoreContinuity::fully_preserved(),
            false,
            OutageEvidenceStateClass::Current,
            "outage-evidence:remote-control-plane:2026-05-30",
            "Remote attach and managed control-plane operations fail closed; local-core editing, save, search, and Git continue on the device.",
            all.clone(),
        ),
        ServiceOutageEntry::new(
            "continuity-outage:ai-gateway",
            OptionalServiceFamily::AiGateway,
            "AI gateway",
            PlaneImpairmentClass::DataPlaneImpairment,
            ImpairmentSeverityClass::Degraded,
            DegradedFallbackClass::LocalModelOrManualFallback,
            LocalCoreContinuity::fully_preserved(),
            false,
            OutageEvidenceStateClass::Current,
            "outage-evidence:ai-gateway:2026-06-01",
            "Managed model inference is degraded; AI features fall back to a local model or to manual editing. Editing is never blocked.",
            all.clone(),
        ),
        ServiceOutageEntry::new(
            "continuity-outage:telemetry-support",
            OptionalServiceFamily::TelemetrySupport,
            "Telemetry and support",
            PlaneImpairmentClass::ControlPlaneImpairment,
            ImpairmentSeverityClass::Recovering,
            DegradedFallbackClass::BufferLocallyAndShipLater,
            LocalCoreContinuity::fully_preserved(),
            false,
            OutageEvidenceStateClass::StaleWithinGrace,
            "outage-evidence:telemetry-support:2026-05-25",
            "Telemetry and diagnostics upload is reconnecting; reports buffer locally and ship once the lane recovers.",
            all,
        ),
    ];
    ServiceOutageTaxonomyInput {
        input_label: "Optional-service outage taxonomy across identity/policy, registry/updates/docs, collaboration, remote control plane, AI gateway, and telemetry/support".to_owned(),
        entries,
    }
}

fn audit(
    input: &ServiceOutageTaxonomyInput,
    projections: &[ServiceOutageSurfaceProjection],
) -> Vec<OutageTaxonomyDefect> {
    let mut defects = Vec::new();
    for entry in &input.entries {
        audit_entry(entry, &mut defects);
    }
    audit_vocabulary(input, projections, &mut defects);
    audit_taxonomy(input, &mut defects);
    defects
}

fn audit_entry(entry: &ServiceOutageEntry, defects: &mut Vec<OutageTaxonomyDefect>) {
    // Headline guardrail: an optional-service outage may not conflate itself with
    // a local editing failure, by flipping a global "IDE down" state or marking
    // any local-core capability unavailable.
    if entry.conflates_local_core() {
        defects.push(OutageTaxonomyDefect::new(
            OutageNarrowReasonClass::LocalCoreConflated,
            entry.packet_id.clone(),
            "an optional-service outage may not flip a global IDE-down state or mark local editing, save, search, or version control unavailable; local-core work stays safe",
        ));
    }

    if entry.severity.is_impaired() {
        // An impaired lane must name the narrower fallback that takes over.
        if !entry.fallback.is_active() {
            defects.push(OutageTaxonomyDefect::new(
                OutageNarrowReasonClass::FallbackUndeclared,
                entry.packet_id.clone(),
                "an impaired optional-service lane must name the narrower fallback that takes over",
            ));
        }
    } else if entry.operational_inconsistent() {
        // An operational lane must not claim an active fallback or impaired core.
        defects.push(OutageTaxonomyDefect::new(
            OutageNarrowReasonClass::OperationalStateInconsistent,
            entry.packet_id.clone(),
            "an operational lane must not claim an active fallback or an impaired local-core capability",
        ));
    }

    // Surface projection completeness.
    let missing: Vec<&OutageSurfaceClass> = entry
        .required_surfaces()
        .iter()
        .filter(|surface| !entry.projected_surfaces.contains(surface))
        .collect();
    if !missing.is_empty() {
        defects.push(OutageTaxonomyDefect::new(
            OutageNarrowReasonClass::SurfaceReuseIncomplete,
            entry.packet_id.clone(),
            "every outage packet must reach the desktop, CLI/headless explain, service-health, support-export, shiproom, and docs/public-truth surfaces",
        ));
    }

    // Outage evidence freshness.
    if entry.evidence_state == OutageEvidenceStateClass::Missing
        || entry.outage_evidence_ref.is_empty()
    {
        defects.push(OutageTaxonomyDefect::new(
            OutageNarrowReasonClass::OutageEvidenceMissing,
            entry.packet_id.clone(),
            "every outage packet must reference current outage evidence",
        ));
    } else if !entry.evidence_state.is_acceptable() {
        defects.push(OutageTaxonomyDefect::new(
            OutageNarrowReasonClass::OutageEvidenceStale,
            entry.packet_id.clone(),
            "outage evidence is stale and a refresh is required before the claim stays stable",
        ));
    }
}

fn audit_vocabulary(
    input: &ServiceOutageTaxonomyInput,
    projections: &[ServiceOutageSurfaceProjection],
    defects: &mut Vec<OutageTaxonomyDefect>,
) {
    for entry in &input.entries {
        let canonical_outage = outage_summary_line(entry);
        let canonical_local = entry.local_core.summary_line();
        let drifted = projections
            .iter()
            .filter(|projection| projection.packet_id == entry.packet_id)
            .any(|projection| {
                projection.outage_summary_line != canonical_outage
                    || projection.local_core_line != canonical_local
            });
        if drifted {
            defects.push(OutageTaxonomyDefect::new(
                OutageNarrowReasonClass::OutageVocabularyDrift,
                entry.packet_id.clone(),
                "a surface renders different outage or local-core vocabulary than the descriptor",
            ));
        }
    }
}

fn audit_taxonomy(input: &ServiceOutageTaxonomyInput, defects: &mut Vec<OutageTaxonomyDefect>) {
    if input.entries.is_empty() {
        return;
    }
    if !plane_distinction_is_complete(&input.entries) {
        defects.push(OutageTaxonomyDefect::new(
            OutageNarrowReasonClass::PlaneDistinctionMissing,
            "taxonomy:plane_distinction",
            "the taxonomy must classify at least one control-plane outage and one data-plane outage",
        ));
    }
    if !all_families_covered(&input.entries) {
        defects.push(OutageTaxonomyDefect::new(
            OutageNarrowReasonClass::FamilyCoverageIncomplete,
            "taxonomy:family_coverage",
            "the taxonomy must cover every optional-service family with at least one outage packet",
        ));
    }
}

fn plane_distinction_is_complete(entries: &[ServiceOutageEntry]) -> bool {
    let has_control = entries
        .iter()
        .any(|entry| entry.impaired_plane.covers_control_plane());
    let has_data = entries
        .iter()
        .any(|entry| entry.impaired_plane.covers_data_plane());
    has_control && has_data
}

fn all_families_covered(entries: &[ServiceOutageEntry]) -> bool {
    OptionalServiceFamily::ALL
        .iter()
        .all(|family| entries.iter().any(|entry| entry.family == *family))
}

fn build_surface_projections(
    entries: &[ServiceOutageEntry],
) -> Vec<ServiceOutageSurfaceProjection> {
    let mut projections = Vec::new();
    for entry in entries {
        let outage_summary_line = outage_summary_line(entry);
        let local_core_line = entry.local_core.summary_line();
        let descriptor_id = format!("continuity:service-outage-descriptor:{}", entry.packet_id);
        for surface in OutageSurfaceClass::ALL {
            if !entry.projected_surfaces.contains(&surface) {
                continue;
            }
            projections.push(ServiceOutageSurfaceProjection {
                record_kind: OUTAGE_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
                schema_version: OUTAGE_TAXONOMY_SCHEMA_VERSION,
                shared_contract_ref: OUTAGE_TAXONOMY_SHARED_CONTRACT_REF.to_owned(),
                surface,
                surface_token: surface.as_str().to_owned(),
                packet_id: entry.packet_id.clone(),
                descriptor_id: descriptor_id.clone(),
                outage_summary_line: outage_summary_line.clone(),
                local_core_line: local_core_line.clone(),
            });
        }
    }
    projections
}

fn build_outcomes(
    input: &ServiceOutageTaxonomyInput,
    defects: &[OutageTaxonomyDefect],
) -> Vec<ServiceOutageOutcome> {
    input
        .entries
        .iter()
        .map(|entry| {
            let reasons: Vec<OutageNarrowReasonClass> = defects
                .iter()
                .filter(|defect| defect.source == entry.packet_id)
                .map(|defect| defect.narrow_reason)
                .collect();
            let qualification = qualification_from_reasons(reasons.iter());
            let mut reason_tokens: Vec<String> = reasons
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect();
            reason_tokens.sort();
            reason_tokens.dedup();
            ServiceOutageOutcome {
                record_kind: SERVICE_OUTAGE_OUTCOME_RECORD_KIND.to_owned(),
                schema_version: OUTAGE_TAXONOMY_SCHEMA_VERSION,
                shared_contract_ref: OUTAGE_TAXONOMY_SHARED_CONTRACT_REF.to_owned(),
                packet_id: entry.packet_id.clone(),
                family_token: entry.family_token.clone(),
                qualification_token: qualification.as_str().to_owned(),
                narrowed: qualification != ContinuityClaimQualificationClass::Stable,
                claim_withheld: qualification == ContinuityClaimQualificationClass::Withdrawn,
                conflates_local_core: entry.conflates_local_core(),
                local_core_preserved: entry.local_core_preserved(),
                degraded_state_token: entry.degraded_state().as_str().to_owned(),
                narrow_reason_tokens: reason_tokens,
            }
        })
        .collect()
}

fn build_summary(
    input: &ServiceOutageTaxonomyInput,
    projections: &[ServiceOutageSurfaceProjection],
    outcomes: &[ServiceOutageOutcome],
    defects: &[OutageTaxonomyDefect],
) -> ServiceOutageTaxonomySummary {
    let overall = if defects
        .iter()
        .any(|defect| defect.narrow_reason.is_withdrawal_reason())
    {
        ContinuityClaimQualificationClass::Withdrawn
    } else if defects
        .iter()
        .any(|defect| defect.narrow_reason.is_preview_reason())
    {
        ContinuityClaimQualificationClass::Preview
    } else if defects.is_empty() {
        ContinuityClaimQualificationClass::Stable
    } else {
        ContinuityClaimQualificationClass::Beta
    };

    let vocabulary_consistent = !defects
        .iter()
        .any(|defect| defect.narrow_reason == OutageNarrowReasonClass::OutageVocabularyDrift);

    let mut families: Vec<OptionalServiceFamily> =
        input.entries.iter().map(|entry| entry.family).collect();
    families.sort();
    families.dedup();

    ServiceOutageTaxonomySummary {
        record_kind: OUTAGE_TAXONOMY_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: OUTAGE_TAXONOMY_SCHEMA_VERSION,
        shared_contract_ref: OUTAGE_TAXONOMY_SHARED_CONTRACT_REF.to_owned(),
        overall_qualification_token: overall.as_str().to_owned(),
        packet_count: input.entries.len(),
        family_count: families.len(),
        control_plane_impaired_count: input
            .entries
            .iter()
            .filter(|entry| {
                entry.severity.is_impaired() && entry.impaired_plane.covers_control_plane()
            })
            .count(),
        data_plane_impaired_count: input
            .entries
            .iter()
            .filter(|entry| {
                entry.severity.is_impaired() && entry.impaired_plane.covers_data_plane()
            })
            .count(),
        operational_count: severity_count(input, ImpairmentSeverityClass::Operational),
        degraded_count: severity_count(input, ImpairmentSeverityClass::Degraded),
        unavailable_count: severity_count(input, ImpairmentSeverityClass::Unavailable),
        recovering_count: severity_count(input, ImpairmentSeverityClass::Recovering),
        local_core_preserved_count: input
            .entries
            .iter()
            .filter(|entry| entry.local_core_preserved())
            .count(),
        narrowed_count: outcomes.iter().filter(|outcome| outcome.narrowed).count(),
        withdrawn_count: outcomes
            .iter()
            .filter(|outcome| outcome.claim_withheld)
            .count(),
        surface_projection_count: projections.len(),
        vocabulary_consistent,
        all_local_core_preserved: outcomes.iter().all(|outcome| outcome.local_core_preserved),
        no_global_ide_down_misclaim: !input.entries.iter().any(|entry| entry.sets_global_ide_down),
        all_families_covered: all_families_covered(&input.entries),
        plane_distinction_present: plane_distinction_is_complete(&input.entries),
        raw_payloads_excluded: true,
        defect_count: defects.len(),
    }
}

fn severity_count(input: &ServiceOutageTaxonomyInput, severity: ImpairmentSeverityClass) -> usize {
    input
        .entries
        .iter()
        .filter(|entry| entry.severity == severity)
        .count()
}

fn outage_summary_line(entry: &ServiceOutageEntry) -> String {
    format!(
        "{}: {} on the {}; fallback {}; degraded state {}.",
        entry.family.plain(),
        entry.severity.plain(),
        plane_plain(entry.impaired_plane),
        entry.fallback.plain(),
        entry.degraded_state().plain(),
    )
}

fn plane_plain(class: PlaneImpairmentClass) -> &'static str {
    match class {
        PlaneImpairmentClass::ControlPlaneImpairment => "control plane",
        PlaneImpairmentClass::DataPlaneImpairment => "managed data plane",
        PlaneImpairmentClass::BothPlanes => "control and managed data planes",
    }
}
