//! Make mirror-only, local-file-bundle, public-direct, blocked, and deferred
//! route handling explicit across the claimed M5 networked artifact families
//! while preserving local-core continuity.
//!
//! The sibling [`crate::networked_surface_transport_decision`] module emits one
//! inspectable transport decision per network-capable *action*, and
//! [`crate::networked_surface_transport_explainability`] projects that stream
//! into product-grade posture, ledger, and explain views. This module is the
//! **mirror/offline continuity layer**: it answers, per claimed M5 *artifact
//! family* (docs packs, registries, model packs, request workspaces, and
//! companion handoffs), the one question those layers leave open — when the
//! primary route is stale, deferred, or policy-blocked, how does the family's
//! content resolve, and does local work keep going regardless?
//!
//! Each [`ContinuityRecord`] resolves an artifact family to exactly one of five
//! route-handling behaviors, recorded with a stable [`ContinuityRouteClass`]
//! token so product UI and exported decision records agree:
//!
//! - [`ContinuityRouteClass::MirrorRoute`] — served from a declared signed
//!   mirror; deny rather than fall through to the public internet,
//! - [`ContinuityRouteClass::LocalFileBundle`] — served from a validated local
//!   bundle/cache with no live egress,
//! - [`ContinuityRouteClass::PublicDirect`] — an *explicitly declared* public
//!   egress (never a silent fall-through),
//! - [`ContinuityRouteClass::Blocked`] — denied by policy with a typed reason,
//! - [`ContinuityRouteClass::Deferred`] — queued for idempotent offline replay.
//!
//! Alongside the route, every record carries a typed [`StaleMirrorWarningClass`]
//! (so a stale mirror surfaces a warning instead of silently serving expired
//! content) and a typed [`PublicFallbackRuleClass`] (so the public-fallback
//! rule is declared, never folklore). A mirror-only or deny-all profile that
//! permits an undeclared public fall-through is a hard guardrail violation.
//!
//! These records aggregate into one stable proof packet
//! ([`MirrorOfflineContinuityPage`]) consumed by product UI, CLI/headless
//! output, diagnostics, support exports, and admin/audit surfaces, so the
//! mirror/offline route handling, stale-mirror warnings, and public-fallback
//! rules are inspectable rather than reconstructed from per-feature folklore.
//!
//! The stable claim holds when **all** of the following conditions are verified
//! simultaneously for every covered artifact family:
//!
//! 1. All required artifact families have a continuity record.
//! 2. No raw private material is present on any record.
//! 3. Every record resolved through the shared transport-governance layer
//!    (`no_bypass: true`).
//! 4. No record permits a silent fall-through to the public internet from a
//!    mirror-only or deny-all profile.
//! 5. Any deferred record queues only an explicitly idempotent action.
//! 6. Every record preserves local-core continuity.
//! 7. Every blocked record carries a typed denial reason.
//! 8. Every record carries a non-empty trust-proof ref.
//! 9. Every record's trust proof is fresh (or stale only within an accepted
//!    grace window).
//! 10. Every record's declared public-fallback rule is consistent with its
//!     route handling.
//! 11. No record serves a mirror whose stale-mirror warning is blocking
//!     (stale-beyond-grace or root-mismatch) instead of blocking the route.
//!
//! Four conditions force [`ContinuityQualificationClass::Withdrawn`] immediately
//! and cannot be overridden: raw private material exposed, a bypass of the
//! shared governance layer, a silent public fall-through from a confined
//! profile, or a non-idempotent action queued for offline replay. A missing
//! required artifact family narrows to [`ContinuityQualificationClass::Preview`];
//! the remaining gaps narrow to `Beta`, which lets release and support tooling
//! automatically narrow stale or under-qualified rows before publication.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language summary sentences, and
//! opaque refs only. Raw endpoint URLs, raw hostnames, raw ports, raw
//! credentials, raw bearer/session tokens, raw cookie jars, raw private
//! certificate bytes, raw SSH private material, and raw mirror bodies stay
//! outside the support boundary.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/network/networked-surface-mirror-offline-continuity.md`
//! - Artifact:
//!   `artifacts/network/networked-surface-mirror-offline-continuity.md`
//! - Schema:
//!   `schemas/network/networked_surface_mirror_offline_continuity.schema.json`
//! - Contract ref: [`MIRROR_OFFLINE_CONTINUITY_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::networked_surface_transport_matrix::{
    DenialReasonClass, EgressClass, MirrorOfflineBehaviorClass, OriginScopeClass,
    ProofFreshnessClass,
};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const MIRROR_OFFLINE_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const MIRROR_OFFLINE_CONTINUITY_SHARED_CONTRACT_REF: &str =
    "remote:networked_surface_mirror_offline_continuity:v1";

/// Record-kind tag for [`MirrorOfflineContinuityPage`] payloads.
pub const MIRROR_OFFLINE_CONTINUITY_PAGE_RECORD_KIND: &str =
    "remote_networked_surface_mirror_offline_continuity_page_record";

/// Record-kind tag for [`ContinuityRecord`] payloads.
pub const MIRROR_OFFLINE_CONTINUITY_RECORD_KIND: &str =
    "remote_networked_surface_mirror_offline_continuity_record";

/// Record-kind tag for [`MirrorOfflineContinuityRow`] payloads.
pub const MIRROR_OFFLINE_CONTINUITY_ROW_RECORD_KIND: &str =
    "remote_networked_surface_mirror_offline_continuity_row_record";

/// Record-kind tag for [`MirrorOfflineContinuityDefect`] payloads.
pub const MIRROR_OFFLINE_CONTINUITY_DEFECT_RECORD_KIND: &str =
    "remote_networked_surface_mirror_offline_continuity_defect_record";

/// Record-kind tag for [`MirrorOfflineContinuitySummary`] payloads.
pub const MIRROR_OFFLINE_CONTINUITY_SUMMARY_RECORD_KIND: &str =
    "remote_networked_surface_mirror_offline_continuity_summary_record";

/// Record-kind tag for [`MirrorOfflineContinuitySupportExport`] payloads.
pub const MIRROR_OFFLINE_CONTINUITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "remote_networked_surface_mirror_offline_continuity_support_export_record";

/// Repo-relative path of the stable doc for this continuity layer.
pub const MIRROR_OFFLINE_CONTINUITY_DOC_REF: &str =
    "docs/network/networked-surface-mirror-offline-continuity.md";

/// Repo-relative path of the artifact summary for this continuity layer.
pub const MIRROR_OFFLINE_CONTINUITY_ARTIFACT_REF: &str =
    "artifacts/network/networked-surface-mirror-offline-continuity.md";

/// Repo-relative ref to the canonical evidence index this layer binds into for
/// the closeout certification lane.
pub const MIRROR_OFFLINE_CONTINUITY_EVIDENCE_INDEX_REF: &str =
    "artifacts/release/m5/xt12-evidence-index.md";

/// Stable, ordered catalog of the field names a continuity record renders.
///
/// Product surfaces, CLI/headless output, and support exports MUST all render a
/// continuity record through this exact ordered field set, so the route-handling
/// tokens a user reads in the UI are identical to the ones CLI output and
/// support packets quote. [`ContinuityRecord::render_fields`] is the single
/// renderer; [`MirrorOfflineContinuityPage::all_records_at_field_parity`]
/// verifies the rendered field names match this catalog.
pub const CONTINUITY_FIELD_NAMES: [&str; 9] = [
    "artifact_family",
    "continuity_route",
    "origin_scope",
    "egress_class",
    "mirror_offline_behavior",
    "stale_mirror_warning",
    "public_fallback_rule",
    "local_core_workflow",
    "denial_reason",
];

// ---------------------------------------------------------------------------
// Artifact family vocabulary
// ---------------------------------------------------------------------------

/// Closed vocabulary for the claimed M5 networked artifact families whose
/// mirror/offline continuity this layer governs.
///
/// Each variant maps to a family of content a networked surface pulls in and
/// that must resolve through one declared mirror/offline behavior rather than a
/// per-feature public-fallback assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactFamilyClass {
    /// Documentation packs (bundled or fetched docs content).
    DocsPack,
    /// Extension / artifact registry reads.
    Registry,
    /// AI model packs (downloadable model weights and metadata).
    ModelPack,
    /// Request/API client workspaces (saved collections and environments).
    RequestWorkspace,
    /// Companion device handoff payloads.
    CompanionHandoff,
}

/// Required artifact families that must each carry a continuity record for the
/// stable claim.
pub const REQUIRED_ARTIFACT_FAMILIES: [ArtifactFamilyClass; 5] = [
    ArtifactFamilyClass::DocsPack,
    ArtifactFamilyClass::Registry,
    ArtifactFamilyClass::ModelPack,
    ArtifactFamilyClass::RequestWorkspace,
    ArtifactFamilyClass::CompanionHandoff,
];

impl ArtifactFamilyClass {
    /// Stable closed-vocabulary token recorded in records, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsPack => "docs_pack",
            Self::Registry => "registry",
            Self::ModelPack => "model_pack",
            Self::RequestWorkspace => "request_workspace",
            Self::CompanionHandoff => "companion_handoff",
        }
    }

    /// Human-readable family label safe for UI and exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DocsPack => "Docs pack",
            Self::Registry => "Registry",
            Self::ModelPack => "Model pack",
            Self::RequestWorkspace => "Request workspace",
            Self::CompanionHandoff => "Companion handoff",
        }
    }

    /// The local-core workflow that stays usable when this family's network
    /// route is stale, deferred, or policy-blocked.
    pub const fn local_core_workflow(self) -> &'static str {
        match self {
            Self::DocsPack => "local documentation reading and offline search",
            Self::Registry => "already-installed extensions and tools continue",
            Self::ModelPack => "already-installed local models continue",
            Self::RequestWorkspace => "local request-collection editing and replay",
            Self::CompanionHandoff => "the local workspace continues without the companion",
        }
    }
}

// ---------------------------------------------------------------------------
// Continuity route vocabulary
// ---------------------------------------------------------------------------

/// How an artifact family's content resolves when its primary route is stale,
/// deferred, or policy-blocked.
///
/// This is the canonical five-way distinction the row exists to make explicit:
/// product surfaces and exported decision records both quote this token rather
/// than guessing whether a family fell through to the public internet, served a
/// cached bundle, or was blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityRouteClass {
    /// Served from a declared signed mirror; deny rather than reach the public
    /// internet when the mirror is unavailable.
    MirrorRoute,
    /// Served from a validated local file bundle or cache; no live egress.
    LocalFileBundle,
    /// Served over an explicitly declared public egress. This is never a silent
    /// fall-through from a confined profile.
    PublicDirect,
    /// Blocked by policy; a typed denial reason is recorded and local-core work
    /// continues.
    Blocked,
    /// Queued for offline-deferred replay; only idempotent actions are allowed.
    Deferred,
}

impl ContinuityRouteClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MirrorRoute => "mirror_route",
            Self::LocalFileBundle => "local_file_bundle",
            Self::PublicDirect => "public_direct",
            Self::Blocked => "blocked",
            Self::Deferred => "deferred",
        }
    }

    /// Returns `true` when this route queues an action for offline-deferred
    /// replay and therefore admits only idempotent actions.
    pub const fn is_deferred(self) -> bool {
        matches!(self, Self::Deferred)
    }

    /// Returns `true` when this route is policy-blocked and must carry a typed
    /// denial reason.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::Blocked)
    }

    /// Returns `true` when this route sends traffic over the public internet.
    pub const fn reaches_public_internet(self) -> bool {
        matches!(self, Self::PublicDirect)
    }

    /// Returns `true` when this route serves content without any live egress.
    pub const fn serves_without_egress(self) -> bool {
        matches!(self, Self::MirrorRoute | Self::LocalFileBundle)
    }

    /// The public-fallback rule this route handling is consistent with.
    pub const fn consistent_public_fallback_rule(self) -> PublicFallbackRuleClass {
        match self {
            Self::MirrorRoute => PublicFallbackRuleClass::MirrorOnlyNoFallback,
            Self::LocalFileBundle => PublicFallbackRuleClass::NoPublicFallback,
            Self::PublicDirect => PublicFallbackRuleClass::ExplicitPublicDirectAllowed,
            Self::Blocked => PublicFallbackRuleClass::DenyAllNoFallback,
            Self::Deferred => PublicFallbackRuleClass::NoPublicFallback,
        }
    }
}

// ---------------------------------------------------------------------------
// Stale-mirror warning vocabulary
// ---------------------------------------------------------------------------

/// Typed stale-mirror warning surfaced alongside a continuity record.
///
/// A stale mirror surfaces an explicit, typed warning rather than silently
/// serving expired content. A blocking warning (stale beyond the grace window or
/// a root mismatch) requires the route to block instead of serve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaleMirrorWarningClass {
    /// No stale-mirror warning; the mirror or bundle is current.
    None,
    /// The mirror is stale but still within its accepted grace window.
    StaleWithinGrace,
    /// The mirror is stale beyond its grace window; the route must block.
    StaleBeyondGrace,
    /// The declared mirror root does not match the served mirror; the route must
    /// block.
    MirrorRootMismatch,
}

impl StaleMirrorWarningClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::StaleWithinGrace => "stale_within_grace",
            Self::StaleBeyondGrace => "stale_beyond_grace",
            Self::MirrorRootMismatch => "mirror_root_mismatch",
        }
    }

    /// Returns `true` when this warning must force the route to block rather
    /// than serve a stale or mismatched mirror.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::StaleBeyondGrace | Self::MirrorRootMismatch)
    }

    /// Returns `true` when this is a non-blocking advisory warning.
    pub const fn is_advisory(self) -> bool {
        matches!(self, Self::StaleWithinGrace)
    }
}

// ---------------------------------------------------------------------------
// Public-fallback rule vocabulary
// ---------------------------------------------------------------------------

/// Declared rule governing whether and how a family may fall back to the public
/// internet.
///
/// The rule is always declared, never folklore: it makes explicit whether a
/// public egress is permitted at all and, if so, that it is an explicit choice
/// rather than a silent fall-through from a confined profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicFallbackRuleClass {
    /// No public fallback is permitted; the family serves locally or defers.
    NoPublicFallback,
    /// Public egress is explicitly declared and permitted for this family.
    ExplicitPublicDirectAllowed,
    /// Mirror-only profile: deny rather than fall through to the public
    /// internet.
    MirrorOnlyNoFallback,
    /// Deny-all profile: no egress and no public fallback.
    DenyAllNoFallback,
}

impl PublicFallbackRuleClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPublicFallback => "no_public_fallback",
            Self::ExplicitPublicDirectAllowed => "explicit_public_direct_allowed",
            Self::MirrorOnlyNoFallback => "mirror_only_no_fallback",
            Self::DenyAllNoFallback => "deny_all_no_fallback",
        }
    }

    /// Returns `true` when this rule permits any public egress at all.
    pub const fn permits_public_egress(self) -> bool {
        matches!(self, Self::ExplicitPublicDirectAllowed)
    }

    /// Returns `true` when this rule confines the family to a mirror-only or
    /// deny-all profile that must not fall through to the public internet.
    pub const fn is_confined_profile(self) -> bool {
        matches!(self, Self::MirrorOnlyNoFallback | Self::DenyAllNoFallback)
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall continuity page and for
/// individual artifact-family rows.
///
/// The tier is derived, not asserted: it is computed by comparing the audit
/// defect list against the stability conditions. A caller may never bump a row
/// to `Stable` without a clean audit and complete family coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityQualificationClass {
    /// All stability conditions hold and the audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required artifact family has no continuity record; the coverage gap
    /// prevents a beta claim for the missing family.
    Preview,
    /// A hard guardrail was violated; the packet is withdrawn immediately and
    /// cannot be overridden.
    Withdrawn,
}

impl ContinuityQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Returns `true` when this tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Returns `true` when this tier is claimable (stable or beta).
    pub const fn is_claimable(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }
}

/// Typed reason a packet or row was narrowed below
/// [`ContinuityQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityNarrowReasonClass {
    /// No narrowing — the row qualifies stable.
    NotNarrowed,
    /// A record carries `raw_private_material_excluded: false`; withdraws the
    /// packet immediately.
    RawPrivateMaterialExposed,
    /// A record resolved outside the shared transport-governance layer
    /// (`no_bypass: false`); withdraws the packet immediately.
    BypassedSharedGovernance,
    /// A mirror-only or deny-all profile permits a silent fall-through to the
    /// public internet; withdraws the packet immediately.
    SilentPublicFallbackResolved,
    /// A deferred record queued a non-idempotent action for offline replay;
    /// withdraws the packet immediately.
    NonIdempotentReplayQueued,
    /// A required artifact family has no continuity record; narrows to preview.
    RequiredArtifactFamilyMissing,
    /// A blocked record carries no typed denial reason.
    DenialReasonMissing,
    /// A record does not preserve local-core continuity.
    LocalCoreContinuityNotPreserved,
    /// A record carries no trust-proof ref.
    TrustProofMissing,
    /// A record's trust proof has expired beyond its freshness window.
    ProofStaleBeyondWindow,
    /// A record's declared public-fallback rule is inconsistent with its route
    /// handling.
    FallbackRuleInconsistent,
    /// A record serves a mirror whose stale-mirror warning is blocking instead
    /// of blocking the route.
    StaleMirrorServedBeyondGrace,
}

impl ContinuityNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::BypassedSharedGovernance => "bypassed_shared_governance",
            Self::SilentPublicFallbackResolved => "silent_public_fallback_resolved",
            Self::NonIdempotentReplayQueued => "non_idempotent_replay_queued",
            Self::RequiredArtifactFamilyMissing => "required_artifact_family_missing",
            Self::DenialReasonMissing => "denial_reason_missing",
            Self::LocalCoreContinuityNotPreserved => "local_core_continuity_not_preserved",
            Self::TrustProofMissing => "trust_proof_missing",
            Self::ProofStaleBeyondWindow => "proof_stale_beyond_window",
            Self::FallbackRuleInconsistent => "fallback_rule_inconsistent",
            Self::StaleMirrorServedBeyondGrace => "stale_mirror_served_beyond_grace",
        }
    }

    /// Returns `true` when this reason is a hard guardrail that withdraws the
    /// packet.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::RawPrivateMaterialExposed
                | Self::BypassedSharedGovernance
                | Self::SilentPublicFallbackResolved
                | Self::NonIdempotentReplayQueued
        )
    }

    /// Returns `true` when this reason narrows to preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(self, Self::RequiredArtifactFamilyMissing)
    }
}

// ---------------------------------------------------------------------------
// Continuity record (per artifact family)
// ---------------------------------------------------------------------------

/// Inspectable mirror/offline continuity record for one artifact family.
///
/// The record binds an artifact family to its declared route handling, the
/// stale-mirror warning state, the declared public-fallback rule, the
/// mirror/offline behavior, and the local-core workflow that stays usable
/// regardless. No raw mirror URL, raw credential, or raw payload appears; only
/// closed-vocabulary tokens and opaque refs cross the boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque id for this record.
    pub record_id: String,
    /// Artifact family this record governs.
    pub artifact_family: ArtifactFamilyClass,
    /// Stable token for [`Self::artifact_family`].
    pub artifact_family_token: String,
    /// Human-readable artifact-family label.
    pub artifact_family_label: String,
    /// The route handling resolved for this family.
    pub continuity_route: ContinuityRouteClass,
    /// Stable token for [`Self::continuity_route`].
    pub continuity_route_token: String,
    /// Origin ownership scope of the route.
    pub origin_scope: OriginScopeClass,
    /// Stable token for [`Self::origin_scope`].
    pub origin_scope_token: String,
    /// Egress class enforced for this family.
    pub egress_class: EgressClass,
    /// Stable token for [`Self::egress_class`].
    pub egress_class_token: String,
    /// Mirror/offline behavior when the primary route is unavailable.
    pub mirror_offline_behavior: MirrorOfflineBehaviorClass,
    /// Stable token for [`Self::mirror_offline_behavior`].
    pub mirror_offline_behavior_token: String,
    /// Stale-mirror warning state.
    pub stale_mirror_warning: StaleMirrorWarningClass,
    /// Stable token for [`Self::stale_mirror_warning`].
    pub stale_mirror_warning_token: String,
    /// Declared public-fallback rule.
    pub public_fallback_rule: PublicFallbackRuleClass,
    /// Stable token for [`Self::public_fallback_rule`].
    pub public_fallback_rule_token: String,
    /// Plain-language description of the local-core workflow that stays usable.
    pub local_core_workflow: String,
    /// Trust input proof ref anchoring host proof; required for the stable
    /// claim.
    pub trust_proof_ref: String,
    /// Freshness of the trust proof.
    pub trust_proof_freshness: ProofFreshnessClass,
    /// Stable token for [`Self::trust_proof_freshness`].
    pub trust_proof_freshness_token: String,
    /// Typed denial reason when the route is blocked; `None` otherwise.
    pub denial_reason: Option<DenialReasonClass>,
    /// Stable token for [`Self::denial_reason`]; empty when `None`.
    pub denial_reason_token: String,
    /// `true` when the action this record governs is idempotent.
    pub action_is_idempotent: bool,
    /// `true` when no silent fall-through to the public internet is permitted
    /// from a confined profile. Must be `true` for the stable claim.
    pub no_silent_public_fallback: bool,
    /// `true` when local-core work continues regardless of this family's route.
    pub local_core_continuity_preserved: bool,
    /// `true` when the record resolved through the shared transport-governance
    /// layer and did not ship a private proxy stack, direct CA override,
    /// undeclared public fallback, or hidden direct-connect retry.
    pub no_bypass: bool,
    /// Plain-language summary safe for UI, support export, and diagnostics.
    pub summary: String,
    /// `true` when no raw private material is present on this record.
    pub raw_private_material_excluded: bool,
}

impl ContinuityRecord {
    /// Construct a continuity record, filling in token fields from the typed
    /// enum values.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        record_id: impl Into<String>,
        artifact_family: ArtifactFamilyClass,
        continuity_route: ContinuityRouteClass,
        origin_scope: OriginScopeClass,
        egress_class: EgressClass,
        mirror_offline_behavior: MirrorOfflineBehaviorClass,
        stale_mirror_warning: StaleMirrorWarningClass,
        public_fallback_rule: PublicFallbackRuleClass,
        trust_proof_ref: impl Into<String>,
        trust_proof_freshness: ProofFreshnessClass,
        denial_reason: Option<DenialReasonClass>,
        action_is_idempotent: bool,
        summary: impl Into<String>,
    ) -> Self {
        let denial_reason_token = denial_reason
            .map(|d| d.as_str().to_owned())
            .unwrap_or_default();
        Self {
            record_kind: MIRROR_OFFLINE_CONTINUITY_RECORD_KIND.to_owned(),
            schema_version: MIRROR_OFFLINE_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: MIRROR_OFFLINE_CONTINUITY_SHARED_CONTRACT_REF.to_owned(),
            record_id: record_id.into(),
            artifact_family,
            artifact_family_token: artifact_family.as_str().to_owned(),
            artifact_family_label: artifact_family.label().to_owned(),
            continuity_route,
            continuity_route_token: continuity_route.as_str().to_owned(),
            origin_scope,
            origin_scope_token: origin_scope.as_str().to_owned(),
            egress_class,
            egress_class_token: egress_class.as_str().to_owned(),
            mirror_offline_behavior,
            mirror_offline_behavior_token: mirror_offline_behavior.as_str().to_owned(),
            stale_mirror_warning,
            stale_mirror_warning_token: stale_mirror_warning.as_str().to_owned(),
            public_fallback_rule,
            public_fallback_rule_token: public_fallback_rule.as_str().to_owned(),
            local_core_workflow: artifact_family.local_core_workflow().to_owned(),
            trust_proof_ref: trust_proof_ref.into(),
            trust_proof_freshness,
            trust_proof_freshness_token: trust_proof_freshness.as_str().to_owned(),
            denial_reason,
            denial_reason_token,
            action_is_idempotent,
            no_silent_public_fallback: true,
            local_core_continuity_preserved: true,
            no_bypass: true,
            summary: summary.into(),
            raw_private_material_excluded: true,
        }
    }

    /// Returns `true` when the declared public-fallback rule matches the route
    /// handling.
    pub fn fallback_rule_is_consistent(&self) -> bool {
        self.continuity_route.consistent_public_fallback_rule() == self.public_fallback_rule
    }

    /// Returns `true` when a mirror-only or deny-all profile would silently
    /// reach the public internet — a hard guardrail violation.
    ///
    /// This catches both the directly-declared violation (`no_silent_public_fallback:
    /// false`) and the derived inconsistency where a confined mirror/offline
    /// profile is paired with a public-direct route or a public-permitting rule.
    pub fn derives_silent_public_fallback(&self) -> bool {
        if !self.no_silent_public_fallback {
            return true;
        }
        let confined_profile = matches!(
            self.mirror_offline_behavior,
            MirrorOfflineBehaviorClass::MirrorFirstThenDeny | MirrorOfflineBehaviorClass::DenyAll
        );
        confined_profile
            && (self.continuity_route.reaches_public_internet()
                || self.public_fallback_rule.permits_public_egress())
    }

    /// Returns `true` when this record serves a mirror whose stale-mirror
    /// warning is blocking instead of blocking the route.
    pub fn serves_stale_mirror_beyond_grace(&self) -> bool {
        self.stale_mirror_warning.is_blocking() && !self.continuity_route.is_blocked()
    }

    /// Render the record's fields as ordered `(name, value)` pairs through the
    /// shared [`CONTINUITY_FIELD_NAMES`] catalog.
    ///
    /// This is the single renderer product, CLI/headless, and support views all
    /// call, so the field names and route-handling tokens stay identical across
    /// surfaces.
    pub fn render_fields(&self) -> Vec<(String, String)> {
        let values: [&str; CONTINUITY_FIELD_NAMES.len()] = [
            self.artifact_family_token.as_str(),
            self.continuity_route_token.as_str(),
            self.origin_scope_token.as_str(),
            self.egress_class_token.as_str(),
            self.mirror_offline_behavior_token.as_str(),
            self.stale_mirror_warning_token.as_str(),
            self.public_fallback_rule_token.as_str(),
            self.local_core_workflow.as_str(),
            self.denial_reason_token.as_str(),
        ];
        CONTINUITY_FIELD_NAMES
            .iter()
            .zip(values.iter())
            .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
            .collect()
    }

    /// Render the record as CLI/headless `key=value` lines.
    pub fn render_cli_lines(&self) -> Vec<String> {
        self.render_fields()
            .into_iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect()
    }

    /// Render the record as support-export `key: value` lines.
    pub fn render_support_lines(&self) -> Vec<String> {
        self.render_fields()
            .into_iter()
            .map(|(name, value)| format!("{name}: {value}"))
            .collect()
    }

    /// Returns `true` when the rendered field names match
    /// [`CONTINUITY_FIELD_NAMES`] in order, proving product/CLI/support parity by
    /// construction.
    pub fn fields_at_parity(&self) -> bool {
        let rendered = self.render_fields();
        rendered.len() == CONTINUITY_FIELD_NAMES.len()
            && rendered
                .iter()
                .zip(CONTINUITY_FIELD_NAMES.iter())
                .all(|((name, _), expected)| name == *expected)
    }
}

// ---------------------------------------------------------------------------
// Continuity snapshot (aggregate of all records)
// ---------------------------------------------------------------------------

/// Aggregate of all continuity records for the covered artifact families.
///
/// The snapshot carries one [`ContinuityRecord`] per claimed M5 artifact family.
/// A snapshot missing any required family causes the page to narrow to
/// `Preview`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuitySnapshot {
    /// All continuity records in the snapshot.
    pub records: Vec<ContinuityRecord>,
}

impl ContinuitySnapshot {
    /// Returns the record for the given family, if present.
    pub fn record_for_family(&self, family: ArtifactFamilyClass) -> Option<&ContinuityRecord> {
        self.records.iter().find(|r| r.artifact_family == family)
    }

    /// Returns the set of artifact-family tokens covered by this snapshot.
    pub fn covered_family_tokens(&self) -> BTreeSet<&str> {
        self.records
            .iter()
            .map(|r| r.artifact_family_token.as_str())
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Continuity row (per-family stability row)
// ---------------------------------------------------------------------------

/// Stability qualification for one artifact family in the continuity page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorOfflineContinuityRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Record id for this row.
    pub record_id: String,
    /// Artifact family token for this row.
    pub artifact_family_token: String,
    /// Continuity route token.
    pub continuity_route_token: String,
    /// Origin scope token.
    pub origin_scope_token: String,
    /// Egress class token.
    pub egress_class_token: String,
    /// Mirror/offline behavior token.
    pub mirror_offline_behavior_token: String,
    /// Stale-mirror warning token.
    pub stale_mirror_warning_token: String,
    /// Public-fallback rule token.
    pub public_fallback_rule_token: String,
    /// Denial reason token; empty when not blocked.
    pub denial_reason_token: String,
    /// `true` when the record resolved through the shared governance layer.
    pub no_bypass: bool,
    /// `true` when no silent public fall-through is permitted.
    pub no_silent_public_fallback: bool,
    /// `true` when a deferred action is idempotent (always `true` when not
    /// deferred).
    pub replay_idempotent_only: bool,
    /// `true` when local-core continuity is preserved.
    pub local_core_continuity_preserved: bool,
    /// `true` when the declared public-fallback rule matches the route handling.
    pub fallback_rule_consistent: bool,
    /// Trust proof freshness token.
    pub proof_freshness_token: String,
    /// `true` when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Derived qualification tier token.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// Aggregate banner emitted with the continuity page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MirrorOfflineContinuitySummary {
    /// Total row count (one row per covered family).
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Artifact-family tokens covered by the page.
    pub families_covered: Vec<String>,
    /// Number of records that resolved through the shared governance layer.
    pub no_bypass_count: usize,
    /// Number of records that preserve local-core continuity.
    pub local_core_continuity_preserved_count: usize,
    /// Number of records with a fresh (or grace-window) trust proof.
    pub usable_proof_count: usize,
    /// Number of records whose declared public-fallback rule is consistent.
    pub fallback_rule_consistent_count: usize,
    /// Record counts by continuity-route token.
    pub route_counts: BTreeMap<String, usize>,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl MirrorOfflineContinuitySummary {
    fn build(
        rows: &[MirrorOfflineContinuityRow],
        snapshot: &ContinuitySnapshot,
        defects: &[MirrorOfflineContinuityDefect],
    ) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
        }
        // The overall tier is derived from the full defect list, not just the
        // per-row qualifications, so a missing required family (which has no
        // row) still narrows the page to preview.
        let has_withdrawal = defects
            .iter()
            .any(|d| d.narrow_reason.is_withdrawal_reason());
        let has_preview = defects.iter().any(|d| d.narrow_reason.is_preview_reason());
        let overall = if has_withdrawal || withdrawn > 0 {
            ContinuityQualificationClass::Withdrawn
        } else if has_preview || preview > 0 {
            ContinuityQualificationClass::Preview
        } else if !defects.is_empty() || beta > 0 {
            ContinuityQualificationClass::Beta
        } else {
            ContinuityQualificationClass::Stable
        };
        let mut route_counts: BTreeMap<String, usize> = BTreeMap::new();
        for record in &snapshot.records {
            *route_counts
                .entry(record.continuity_route_token.clone())
                .or_insert(0) += 1;
        }
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            families_covered: snapshot
                .records
                .iter()
                .map(|r| r.artifact_family_token.clone())
                .collect(),
            no_bypass_count: snapshot.records.iter().filter(|r| r.no_bypass).count(),
            local_core_continuity_preserved_count: snapshot
                .records
                .iter()
                .filter(|r| r.local_core_continuity_preserved)
                .count(),
            usable_proof_count: snapshot
                .records
                .iter()
                .filter(|r| r.trust_proof_freshness.is_usable())
                .count(),
            fallback_rule_consistent_count: snapshot
                .records
                .iter()
                .filter(|r| r.fallback_rule_is_consistent())
                .count(),
            route_counts,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defect
// ---------------------------------------------------------------------------

/// Typed defect emitted by the continuity page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorOfflineContinuityDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: ContinuityNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (artifact-family token or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl MirrorOfflineContinuityDefect {
    fn new(
        narrow_reason: ContinuityNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: MIRROR_OFFLINE_CONTINUITY_DEFECT_RECORD_KIND.to_owned(),
            schema_version: MIRROR_OFFLINE_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: MIRROR_OFFLINE_CONTINUITY_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "remote:defect:networked-surface-mirror-offline-continuity:{}:{}",
                narrow_reason.as_str(),
                &source_str
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source: source_str,
            note: note.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Continuity page (proof packet)
// ---------------------------------------------------------------------------

/// Stable mirror/offline continuity proof packet for the claimed M5 artifact
/// families.
///
/// The packet is the single inspectable record that proves every claimed M5
/// artifact family distinguishes mirror-route, local-file-bundle, public-direct,
/// blocked, and deferred behavior, surfaces stale-mirror warnings, declares its
/// public-fallback rule, and preserves local-core continuity. Dashboards, docs,
/// Help/About surfaces, CLI/headless output, support exports, release tooling,
/// and diagnostics should ingest this packet rather than reconstructing
/// mirror/offline behavior from per-feature folklore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorOfflineContinuityPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// UTC instant when the packet was generated.
    pub generated_at: String,
    /// Repo-relative ref to the canonical evidence index this page binds into.
    pub evidence_index_ref: String,
    /// Aggregate summary derived from all rows.
    pub summary: MirrorOfflineContinuitySummary,
    /// Per-family stability rows.
    pub rows: Vec<MirrorOfflineContinuityRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<MirrorOfflineContinuityDefect>,
    /// The continuity snapshot embedded as evidence.
    pub continuity_snapshot: ContinuitySnapshot,
}

impl MirrorOfflineContinuityPage {
    /// Build the continuity page from a continuity snapshot.
    ///
    /// Rows are derived per family, and the qualification for each is computed
    /// from the combined audit of the whole snapshot.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        continuity_snapshot: ContinuitySnapshot,
    ) -> Self {
        let defects = audit_snapshot(&continuity_snapshot);
        let rows = derive_rows(&continuity_snapshot, &defects);
        let summary = MirrorOfflineContinuitySummary::build(&rows, &continuity_snapshot, &defects);
        Self {
            record_kind: MIRROR_OFFLINE_CONTINUITY_PAGE_RECORD_KIND.to_owned(),
            schema_version: MIRROR_OFFLINE_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: MIRROR_OFFLINE_CONTINUITY_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            evidence_index_ref: MIRROR_OFFLINE_CONTINUITY_EVIDENCE_INDEX_REF.to_owned(),
            summary,
            rows,
            defects,
            continuity_snapshot,
        }
    }

    /// Returns `true` when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token == ContinuityQualificationClass::Stable.as_str()
    }

    /// Returns `true` when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// Returns `true` when all required artifact families have a record.
    pub fn covers_all_required_families(&self) -> bool {
        let covered = self.continuity_snapshot.covered_family_tokens();
        REQUIRED_ARTIFACT_FAMILIES
            .iter()
            .all(|family| covered.contains(family.as_str()))
    }

    /// Returns `true` when the page distinguishes all five route-handling
    /// behaviors across its covered families.
    pub fn distinguishes_all_route_classes(&self) -> bool {
        let present: BTreeSet<&str> = self
            .continuity_snapshot
            .records
            .iter()
            .map(|r| r.continuity_route_token.as_str())
            .collect();
        [
            ContinuityRouteClass::MirrorRoute,
            ContinuityRouteClass::LocalFileBundle,
            ContinuityRouteClass::PublicDirect,
            ContinuityRouteClass::Blocked,
            ContinuityRouteClass::Deferred,
        ]
        .iter()
        .all(|route| present.contains(route.as_str()))
    }

    /// Returns `true` when no record permits a silent fall-through to the public
    /// internet from a confined profile.
    pub fn no_record_allows_silent_public_fallback(&self) -> bool {
        self.continuity_snapshot
            .records
            .iter()
            .all(|r| !r.derives_silent_public_fallback())
    }

    /// Returns `true` when every deferred record queues only an idempotent
    /// action.
    pub fn replay_queues_are_idempotent_only(&self) -> bool {
        self.continuity_snapshot
            .records
            .iter()
            .all(|r| !r.continuity_route.is_deferred() || r.action_is_idempotent)
    }

    /// Returns `true` when every record preserves local-core continuity.
    pub fn all_records_preserve_local_core(&self) -> bool {
        self.continuity_snapshot
            .records
            .iter()
            .all(|r| r.local_core_continuity_preserved)
    }

    /// Returns `true` when every blocked record carries a typed denial reason.
    pub fn blocked_records_carry_reasons(&self) -> bool {
        self.continuity_snapshot
            .records
            .iter()
            .all(|r| !r.continuity_route.is_blocked() || r.denial_reason.is_some())
    }

    /// Returns `true` when every record's declared public-fallback rule matches
    /// its route handling.
    pub fn all_fallback_rules_consistent(&self) -> bool {
        self.continuity_snapshot
            .records
            .iter()
            .all(|r| r.fallback_rule_is_consistent())
    }

    /// Returns `true` when every record renders at field-catalog parity.
    pub fn all_records_at_field_parity(&self) -> bool {
        self.continuity_snapshot
            .records
            .iter()
            .all(|r| r.fields_at_parity())
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that carries the continuity page plus a metadata-safe
/// defect roll-up.
///
/// No raw mirror URLs, raw hostnames, raw credentials, raw cookies, or raw
/// private key material may appear in this export. Only closed-vocabulary
/// tokens, opaque refs, counts, and plain-language summary sentences cross the
/// boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorOfflineContinuitySupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// UTC export timestamp.
    pub generated_at: String,
    /// The continuity page embedded as evidence.
    pub page: MirrorOfflineContinuityPage,
    /// Narrow-reason class values present in the page's defect list.
    pub narrow_reasons_present: Vec<ContinuityNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// `true` when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl MirrorOfflineContinuitySupportExport {
    /// Wrap a continuity page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: MirrorOfflineContinuityPage,
    ) -> Self {
        let mut reasons: Vec<ContinuityNarrowReasonClass> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }
        reasons.sort();
        Self {
            record_kind: MIRROR_OFFLINE_CONTINUITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: MIRROR_OFFLINE_CONTINUITY_SCHEMA_VERSION,
            shared_contract_ref: MIRROR_OFFLINE_CONTINUITY_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_private_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Audit and validate functions (public API)
// ---------------------------------------------------------------------------

/// Re-run the continuity audit over the snapshot embedded in a page.
pub fn audit_mirror_offline_continuity_page(
    page: &MirrorOfflineContinuityPage,
) -> Vec<MirrorOfflineContinuityDefect> {
    audit_snapshot(&page.continuity_snapshot)
}

/// Validate a continuity page; returns `Ok` when the audit is clean.
pub fn validate_mirror_offline_continuity_page(
    page: &MirrorOfflineContinuityPage,
) -> Result<(), Vec<MirrorOfflineContinuityDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal audit logic
// ---------------------------------------------------------------------------

fn audit_snapshot(snapshot: &ContinuitySnapshot) -> Vec<MirrorOfflineContinuityDefect> {
    let mut defects: Vec<MirrorOfflineContinuityDefect> = Vec::new();

    // Hard guardrails first — any one of these withdraws the packet and makes
    // no further check meaningful.
    for record in &snapshot.records {
        if !record.raw_private_material_excluded {
            defects.push(MirrorOfflineContinuityDefect::new(
                ContinuityNarrowReasonClass::RawPrivateMaterialExposed,
                record.artifact_family_token.clone(),
                format!(
                    "record '{}' for family '{}' exposes raw private material; packet is withdrawn",
                    record.record_id, record.artifact_family_token
                ),
            ));
            return defects;
        }
        if !record.no_bypass {
            defects.push(MirrorOfflineContinuityDefect::new(
                ContinuityNarrowReasonClass::BypassedSharedGovernance,
                record.artifact_family_token.clone(),
                format!(
                    "record '{}' for family '{}' resolved outside the shared transport-governance layer; packet is withdrawn",
                    record.record_id, record.artifact_family_token
                ),
            ));
            return defects;
        }
        if record.derives_silent_public_fallback() {
            defects.push(MirrorOfflineContinuityDefect::new(
                ContinuityNarrowReasonClass::SilentPublicFallbackResolved,
                record.artifact_family_token.clone(),
                format!(
                    "record '{}' for family '{}' permits a silent fall-through to the public internet from a mirror-only or deny-all profile; packet is withdrawn",
                    record.record_id, record.artifact_family_token
                ),
            ));
            return defects;
        }
        if record.continuity_route.is_deferred() && !record.action_is_idempotent {
            defects.push(MirrorOfflineContinuityDefect::new(
                ContinuityNarrowReasonClass::NonIdempotentReplayQueued,
                record.artifact_family_token.clone(),
                format!(
                    "record '{}' for family '{}' queues a non-idempotent action for offline replay; packet is withdrawn",
                    record.record_id, record.artifact_family_token
                ),
            ));
            return defects;
        }
    }

    let covered: BTreeSet<&str> = snapshot
        .records
        .iter()
        .map(|r| r.artifact_family_token.as_str())
        .collect();

    // Coverage check: all required artifact families must have a record.
    for required_family in &REQUIRED_ARTIFACT_FAMILIES {
        if !covered.contains(required_family.as_str()) {
            defects.push(MirrorOfflineContinuityDefect::new(
                ContinuityNarrowReasonClass::RequiredArtifactFamilyMissing,
                required_family.as_str(),
                format!(
                    "required artifact family '{}' has no continuity record; packet is narrowed to preview",
                    required_family.as_str()
                ),
            ));
        }
    }

    // Per-record checks.
    for record in &snapshot.records {
        if record.continuity_route.is_blocked() && record.denial_reason.is_none() {
            defects.push(MirrorOfflineContinuityDefect::new(
                ContinuityNarrowReasonClass::DenialReasonMissing,
                record.artifact_family_token.clone(),
                format!(
                    "record '{}' for family '{}' is blocked but carries no typed denial reason",
                    record.record_id, record.artifact_family_token
                ),
            ));
        }

        if !record.local_core_continuity_preserved {
            defects.push(MirrorOfflineContinuityDefect::new(
                ContinuityNarrowReasonClass::LocalCoreContinuityNotPreserved,
                record.artifact_family_token.clone(),
                format!(
                    "record '{}' for family '{}' does not preserve local-core continuity; local work may be blocked",
                    record.record_id, record.artifact_family_token
                ),
            ));
        }

        if record.trust_proof_ref.is_empty() {
            defects.push(MirrorOfflineContinuityDefect::new(
                ContinuityNarrowReasonClass::TrustProofMissing,
                record.artifact_family_token.clone(),
                format!(
                    "record '{}' for family '{}' carries no trust-proof ref; host proof is unverifiable",
                    record.record_id, record.artifact_family_token
                ),
            ));
        }

        if !record.trust_proof_freshness.is_usable() {
            defects.push(MirrorOfflineContinuityDefect::new(
                ContinuityNarrowReasonClass::ProofStaleBeyondWindow,
                record.artifact_family_token.clone(),
                format!(
                    "record '{}' for family '{}' trust proof is {}; stable claim is narrowed to beta",
                    record.record_id, record.artifact_family_token, record.trust_proof_freshness_token
                ),
            ));
        }

        if !record.fallback_rule_is_consistent() {
            defects.push(MirrorOfflineContinuityDefect::new(
                ContinuityNarrowReasonClass::FallbackRuleInconsistent,
                record.artifact_family_token.clone(),
                format!(
                    "record '{}' for family '{}' declares public-fallback rule '{}' but its route handling '{}' implies '{}'",
                    record.record_id,
                    record.artifact_family_token,
                    record.public_fallback_rule_token,
                    record.continuity_route_token,
                    record
                        .continuity_route
                        .consistent_public_fallback_rule()
                        .as_str()
                ),
            ));
        }

        if record.serves_stale_mirror_beyond_grace() {
            defects.push(MirrorOfflineContinuityDefect::new(
                ContinuityNarrowReasonClass::StaleMirrorServedBeyondGrace,
                record.artifact_family_token.clone(),
                format!(
                    "record '{}' for family '{}' serves a mirror with a blocking stale-mirror warning '{}' instead of blocking the route",
                    record.record_id, record.artifact_family_token, record.stale_mirror_warning_token
                ),
            ));
        }
    }

    defects
}

fn derive_rows(
    snapshot: &ContinuitySnapshot,
    page_defects: &[MirrorOfflineContinuityDefect],
) -> Vec<MirrorOfflineContinuityRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_preview_reason());

    let overall_narrow_reason = if has_withdrawal {
        page_defects
            .iter()
            .find(|d| d.narrow_reason.is_withdrawal_reason())
            .map(|d| d.narrow_reason)
            .unwrap_or(ContinuityNarrowReasonClass::RawPrivateMaterialExposed)
    } else if has_preview {
        ContinuityNarrowReasonClass::RequiredArtifactFamilyMissing
    } else if !page_defects.is_empty() {
        page_defects[0].narrow_reason
    } else {
        ContinuityNarrowReasonClass::NotNarrowed
    };

    snapshot
        .records
        .iter()
        .map(|record| {
            let row_narrow = find_row_narrow_reason(record, page_defects, overall_narrow_reason);
            let row_qual = qualification_for_reason(row_narrow);
            let summary = build_row_summary(&record.artifact_family_token, &row_qual, row_narrow);
            MirrorOfflineContinuityRow {
                record_kind: MIRROR_OFFLINE_CONTINUITY_ROW_RECORD_KIND.to_owned(),
                schema_version: MIRROR_OFFLINE_CONTINUITY_SCHEMA_VERSION,
                shared_contract_ref: MIRROR_OFFLINE_CONTINUITY_SHARED_CONTRACT_REF.to_owned(),
                record_id: record.record_id.clone(),
                artifact_family_token: record.artifact_family_token.clone(),
                continuity_route_token: record.continuity_route_token.clone(),
                origin_scope_token: record.origin_scope_token.clone(),
                egress_class_token: record.egress_class_token.clone(),
                mirror_offline_behavior_token: record.mirror_offline_behavior_token.clone(),
                stale_mirror_warning_token: record.stale_mirror_warning_token.clone(),
                public_fallback_rule_token: record.public_fallback_rule_token.clone(),
                denial_reason_token: record.denial_reason_token.clone(),
                no_bypass: record.no_bypass,
                no_silent_public_fallback: record.no_silent_public_fallback,
                replay_idempotent_only: !record.continuity_route.is_deferred()
                    || record.action_is_idempotent,
                local_core_continuity_preserved: record.local_core_continuity_preserved,
                fallback_rule_consistent: record.fallback_rule_is_consistent(),
                proof_freshness_token: record.trust_proof_freshness_token.clone(),
                raw_private_material_excluded: record.raw_private_material_excluded,
                qualification_token: row_qual.as_str().to_owned(),
                narrow_reason_token: row_narrow.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn qualification_for_reason(reason: ContinuityNarrowReasonClass) -> ContinuityQualificationClass {
    if reason.is_withdrawal_reason() {
        ContinuityQualificationClass::Withdrawn
    } else if reason.is_preview_reason() {
        ContinuityQualificationClass::Preview
    } else if reason != ContinuityNarrowReasonClass::NotNarrowed {
        ContinuityQualificationClass::Beta
    } else {
        ContinuityQualificationClass::Stable
    }
}

fn find_row_narrow_reason(
    record: &ContinuityRecord,
    page_defects: &[MirrorOfflineContinuityDefect],
    overall_narrow_reason: ContinuityNarrowReasonClass,
) -> ContinuityNarrowReasonClass {
    // A withdrawal reason taints the whole packet; every row is withdrawn.
    if overall_narrow_reason.is_withdrawal_reason() {
        return overall_narrow_reason;
    }
    // Otherwise a family-specific defect governs the row.
    if let Some(defect) = page_defects
        .iter()
        .find(|d| d.source == record.artifact_family_token)
    {
        return defect.narrow_reason;
    }
    ContinuityNarrowReasonClass::NotNarrowed
}

fn build_row_summary(
    family_token: &str,
    qual: &ContinuityQualificationClass,
    narrow_reason: ContinuityNarrowReasonClass,
) -> String {
    match qual {
        ContinuityQualificationClass::Stable => format!(
            "Artifact family '{}' continuity qualifies stable: its route handling, stale-mirror \
             warning, and public-fallback rule are typed and consistent; it resolved through the \
             shared governance layer with no silent public fall-through; and local-core work \
             continues regardless.",
            family_token
        ),
        _ => format!(
            "Artifact family '{}' continuity narrowed to {} ({}): see defect list for details.",
            family_token,
            qual.as_str(),
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded stable continuity page consumed by the headless example,
/// the integration tests, and the fixture generator.
///
/// The seeded page produces zero defects: all required artifact families have a
/// record, each demonstrates a distinct one of the five route-handling
/// behaviors, no raw private material is present, every record resolved through
/// the shared governance layer, no mirror-only or deny-all profile allows a
/// silent public fall-through, every deferred action is idempotent-only, every
/// record preserves local-core continuity, every blocked record carries a typed
/// reason, carries a trust-proof ref, has a fresh trust proof, and declares a
/// public-fallback rule consistent with its route.
pub fn seeded_mirror_offline_continuity_page() -> MirrorOfflineContinuityPage {
    MirrorOfflineContinuityPage::new(
        "remote:networked_surface_mirror_offline_continuity:default",
        "Networked-surface mirror/offline continuity — stable packet",
        "2026-06-01T00:00:00Z",
        seeded_mirror_offline_continuity_snapshot(),
    )
}

/// Build the seeded continuity snapshot used by the seeded page.
///
/// Each required artifact family is represented with a fully-typed, clean record
/// that passes all stability conditions, and the five records together
/// demonstrate all five route-handling behaviors:
/// `local_file_bundle` (docs pack), `mirror_route` (registry), `blocked`
/// (model pack), `public_direct` (request workspace), and `deferred`
/// (companion handoff).
pub fn seeded_mirror_offline_continuity_snapshot() -> ContinuitySnapshot {
    ContinuitySnapshot {
        records: vec![
            // Docs pack — served from a validated local file bundle; no egress.
            ContinuityRecord::new(
                "remote:mirror_offline_continuity:docs_pack:0001",
                ArtifactFamilyClass::DocsPack,
                ContinuityRouteClass::LocalFileBundle,
                OriginScopeClass::FirstParty,
                EgressClass::ManagedEndpoint,
                MirrorOfflineBehaviorClass::CachedOffline,
                StaleMirrorWarningClass::None,
                PublicFallbackRuleClass::NoPublicFallback,
                "trust:docs_pack:proof:2026-06-01",
                ProofFreshnessClass::Fresh,
                None,
                true,
                "Docs pack resolves from a validated local bundle with no live egress; when the \
                 bundle is stale it surfaces a warning rather than reaching the public internet; \
                 local documentation reading and offline search continue.",
            ),
            // Registry — served from a declared signed mirror; deny, no public fallback.
            ContinuityRecord::new(
                "remote:mirror_offline_continuity:registry:0001",
                ArtifactFamilyClass::Registry,
                ContinuityRouteClass::MirrorRoute,
                OriginScopeClass::FirstParty,
                EgressClass::MirrorOnly,
                MirrorOfflineBehaviorClass::MirrorFirstThenDeny,
                StaleMirrorWarningClass::StaleWithinGrace,
                PublicFallbackRuleClass::MirrorOnlyNoFallback,
                "trust:registry:proof:2026-06-01",
                ProofFreshnessClass::Fresh,
                None,
                true,
                "Registry reads resolve to the declared signed mirror and deny rather than fall \
                 through to the public internet; a within-grace stale-mirror warning is surfaced; \
                 already-installed extensions and tools continue.",
            ),
            // Model pack — blocked by deny-all profile with a typed reason.
            ContinuityRecord::new(
                "remote:mirror_offline_continuity:model_pack:0001",
                ArtifactFamilyClass::ModelPack,
                ContinuityRouteClass::Blocked,
                OriginScopeClass::ManagedTenant,
                EgressClass::AirGapped,
                MirrorOfflineBehaviorClass::DenyAll,
                StaleMirrorWarningClass::None,
                PublicFallbackRuleClass::DenyAllNoFallback,
                "trust:model_pack:proof:2026-06-01",
                ProofFreshnessClass::Fresh,
                Some(DenialReasonClass::PolicyBlocked),
                true,
                "Model pack download is blocked by the deny-all profile with a typed \
                 policy-blocked reason and no public fallback; already-installed local models \
                 continue.",
            ),
            // Request workspace — explicit public-direct egress.
            ContinuityRecord::new(
                "remote:mirror_offline_continuity:request_workspace:0001",
                ArtifactFamilyClass::RequestWorkspace,
                ContinuityRouteClass::PublicDirect,
                OriginScopeClass::UserConfigured,
                EgressClass::PublicInternet,
                MirrorOfflineBehaviorClass::LocalCoreOnly,
                StaleMirrorWarningClass::None,
                PublicFallbackRuleClass::ExplicitPublicDirectAllowed,
                "trust:request_workspace:proof:2026-06-01",
                ProofFreshnessClass::Fresh,
                None,
                true,
                "Request workspace sends to a user-configured public endpoint over an explicitly \
                 declared public-direct route — never a silent fall-through; when offline, local \
                 request-collection editing and replay continue.",
            ),
            // Companion handoff — deferred for idempotent offline replay.
            ContinuityRecord::new(
                "remote:mirror_offline_continuity:companion_handoff:0001",
                ArtifactFamilyClass::CompanionHandoff,
                ContinuityRouteClass::Deferred,
                OriginScopeClass::LoopbackLocal,
                EgressClass::LoopbackOnly,
                MirrorOfflineBehaviorClass::OfflineGrace,
                StaleMirrorWarningClass::None,
                PublicFallbackRuleClass::NoPublicFallback,
                "trust:companion_handoff:proof:2026-06-01",
                ProofFreshnessClass::Fresh,
                None,
                true,
                "Companion handoff is queued for offline-deferred replay within the offline-grace \
                 window; only the idempotent handoff action is queued; the local workspace \
                 continues without the companion.",
            ),
        ],
    }
}
