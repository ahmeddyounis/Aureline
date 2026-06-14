//! Extension/provider conformance for claimed M5 preview/browser-runtime rows,
//! stale-or-weaker-provider repair guidance, and mirror/offline or inspect-only
//! downgrade truth.
//!
//! Where [`crate::browser_runtime_inspectors`] materializes the *per-inspector*
//! truth a single DOM/CSS/console/network/storage inspector presents, and
//! [`crate::freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix`]
//! freezes the *qualification matrix* over claimed preview/runtime surfaces, this
//! module materializes the **per-provider conformance** truth packet that makes
//! those canonical vocabularies *enforceable* across first-party packs and
//! contributed extension providers: one packet that teaches every provider backing
//! a claimed M5 row to declare which runtime target kinds it supports, how good a
//! source mapping it can produce, how deep it can attach, whether it can hot reload,
//! and what client-scope limit it carries — *before* it can back the row.
//!
//! The packet is the one canonical answer to "for the claimed preview/runtime row
//! the user is looking at, which provider is backing it, did that provider declare
//! enough to honestly do so, and — if the provider became unavailable or weaker —
//! what bounded truth and repair guidance applies instead of a silent semantic
//! swap?" A [`ProviderConformancePacket`] binds first-party and contributed
//! providers onto the same governed vocabulary —
//! [`crate::BrowserRuntimeTargetKind`], [`crate::InspectorMappingQualityClass`],
//! [`crate::AttachDepthClass`], [`HotReloadDeclarationClass`],
//! [`ProviderStatusClass`], and [`OperatingProfileClass`] — instead of
//! provider-specific extension chrome.
//!
//! Source stays canonical and the conformance packet is derivative — never a second
//! writable truth model. A [`ProviderConformanceRow`] keeps the honesty rules the
//! spec freezes:
//!
//! - **Declare before you back.** A provider — first-party or contributed — must
//!   carry an explicit [`ProviderDeclaration`] naming its supported target kinds,
//!   supported mapping-quality classes, maximum attach depth, hot-reload
//!   declaration, and client-scope limit. A `live` row may only be backed by a
//!   provider whose declaration actually satisfies the
//!   [`ClaimedRowRequirement`]; a row never advertises capability its provider did
//!   not declare.
//! - **No silent weaker swap.** If a weaker provider would replace a stronger one,
//!   the row records [`ProviderStatusClass::WeakerReplacement`], preserves the prior
//!   declaration, proves the current declaration is actually weaker, and degrades to
//!   a bounded operating profile with repair guidance — never a silent semantic
//!   switch.
//! - **Stale/unavailable preserves history.** A stale or unavailable provider keeps
//!   its prior declaration and limitation notes as unresolved state with a precise
//!   downgrade label and repair guidance; it never silently presents as conformant.
//! - **Bounded profiles stay explicit and exportable.** Mirror/offline,
//!   inspect-only, and policy-limited profiles each carry an explicit downgrade
//!   trigger and a precise degraded label on every claimed row, so a degraded
//!   surface is bounded truth instead of a blank surface or a hidden provider
//!   dependency.
//! - **No inspect-to-write auto-upgrade.** A write-capable designer flow may appear
//!   only on a live, conformant row; an inspect-only, mirror/offline, or
//!   policy-limited row is never auto-upgraded into write capability.
//!
//! Raw provider payloads, credentials, raw URLs, hostnames, raw runtime handles,
//! and extension-private wording never cross this boundary; the packet carries only
//! typed class tokens, opaque provider/evidence refs, booleans, and precise
//! operator-facing labels, so support and diagnostics exports can reconstruct
//! exactly which provider backed each claimed row and what operating profile the
//! user saw.
//!
//! The boundary schema is
//! [`schemas/preview/extension_provider_conformance.schema.json`](../../../../schemas/preview/extension_provider_conformance.schema.json).
//! The contract doc is
//! [`docs/preview/m5/extension_provider_conformance.md`](../../../../docs/preview/m5/extension_provider_conformance.md).
//! The protected fixture directory is
//! [`fixtures/preview/m5/extension_provider_conformance/`](../../../../fixtures/preview/m5/extension_provider_conformance/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{AttachDepthClass, BrowserRuntimeTargetKind, InspectorMappingQualityClass};

/// Stable record-kind tag carried by [`ProviderConformancePacket`].
pub const EXTENSION_PROVIDER_CONFORMANCE_RECORD_KIND: &str = "extension_provider_conformance";

/// Schema version for the extension-provider conformance packet.
pub const EXTENSION_PROVIDER_CONFORMANCE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const EXTENSION_PROVIDER_CONFORMANCE_SCHEMA_REF: &str =
    "schemas/preview/extension_provider_conformance.schema.json";

/// Repo-relative path of the contract doc.
pub const EXTENSION_PROVIDER_CONFORMANCE_DOC_REF: &str =
    "docs/preview/m5/extension_provider_conformance.md";

/// Repo-relative path of the protected fixture directory.
pub const EXTENSION_PROVIDER_CONFORMANCE_FIXTURE_DIR: &str =
    "fixtures/preview/m5/extension_provider_conformance";

/// Repo-relative path of the checked support-export artifact.
pub const EXTENSION_PROVIDER_CONFORMANCE_ARTIFACT_REF: &str =
    "artifacts/preview/m5/extension_provider_conformance/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const EXTENSION_PROVIDER_CONFORMANCE_SUMMARY_REF: &str =
    "artifacts/preview/m5/extension_provider_conformance.md";

/// Closed provider-origin vocabulary. Names whether a provider is shipped by
/// Aureline or contributed by a third party, so a contributed provider can never
/// quietly inherit first-party trust without declaring its own capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderOriginClass {
    /// A first-party preview/browser-runtime provider shipped with Aureline.
    FirstParty,
    /// A contributed / third-party extension provider.
    Contributed,
}

impl ProviderOriginClass {
    /// Every provider origin a claimed conformance set must demonstrate, in
    /// declaration order.
    pub const ALL: [Self; 2] = [Self::FirstParty, Self::Contributed];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::Contributed => "contributed",
        }
    }
}

/// Closed provider-status vocabulary. Names the conformance status of the provider
/// backing a claimed M5 row right now, so a degraded provider never silently
/// presents as conformant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderStatusClass {
    /// The provider's declaration satisfies the claimed row's requirement and the
    /// provider is available; only a conformant provider may back a live profile.
    Conformant,
    /// The provider's declaration went stale and must be re-verified; the row is
    /// unresolved and the prior declaration is preserved.
    StaleDeclaration,
    /// A weaker provider would replace a stronger one; the prior declaration is
    /// preserved and the swap is refused without an explicit, bounded downgrade.
    WeakerReplacement,
    /// The provider became unavailable; the prior declaration and limitation notes
    /// are preserved as unresolved state.
    Unavailable,
}

impl ProviderStatusClass {
    /// Every provider status a claimed conformance set must demonstrate, in
    /// declaration order.
    pub const ALL: [Self; 4] = [
        Self::Conformant,
        Self::StaleDeclaration,
        Self::WeakerReplacement,
        Self::Unavailable,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Conformant => "conformant",
            Self::StaleDeclaration => "stale_declaration",
            Self::WeakerReplacement => "weaker_replacement",
            Self::Unavailable => "unavailable",
        }
    }

    /// True when this status is a clean, conformant provider.
    pub const fn is_conformant(self) -> bool {
        matches!(self, Self::Conformant)
    }

    /// True when this status must preserve the prior provider declaration as
    /// history so a degraded provider does not erase what came before.
    pub const fn requires_prior_declaration(self) -> bool {
        matches!(
            self,
            Self::StaleDeclaration | Self::WeakerReplacement | Self::Unavailable
        )
    }
}

/// Closed operating-profile vocabulary. Names the bounded posture the claimed row
/// currently presents so a degraded surface stays explicit and exportable instead
/// of a blank surface or a hidden provider dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatingProfileClass {
    /// A full live runtime backing from a conformant provider.
    Live,
    /// A mirror / offline snapshot; bounded truth with no live runtime behind it.
    MirrorOffline,
    /// An inspect-only profile; no write-capable designer flow is offered.
    InspectOnly,
    /// A policy-limited profile; policy narrows what the row may show or do.
    PolicyLimited,
}

impl OperatingProfileClass {
    /// Every operating profile a claimed conformance set must demonstrate, in
    /// declaration order.
    pub const ALL: [Self; 4] = [
        Self::Live,
        Self::MirrorOffline,
        Self::InspectOnly,
        Self::PolicyLimited,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::MirrorOffline => "mirror_offline",
            Self::InspectOnly => "inspect_only",
            Self::PolicyLimited => "policy_limited",
        }
    }

    /// True when this profile is a full live runtime backing.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::Live)
    }

    /// True when this profile is a bounded degraded posture that must carry an
    /// explicit, exportable downgrade disclosure.
    pub const fn is_bounded(self) -> bool {
        matches!(
            self,
            Self::MirrorOffline | Self::InspectOnly | Self::PolicyLimited
        )
    }
}

/// Closed hot-reload declaration vocabulary. Names what hot-reload posture a
/// provider declares for a claimed row, so a row never claims hot reload its
/// provider did not declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotReloadDeclarationClass {
    /// The provider declares stateful hot reload.
    Supported,
    /// The provider declares reload only via a full restart.
    RestartOnly,
    /// The provider declares no reload support.
    Unsupported,
    /// Hot reload does not apply to this provider's target kind.
    NotApplicable,
}

impl HotReloadDeclarationClass {
    /// Stable token recorded in the declaration.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::RestartOnly => "restart_only",
            Self::Unsupported => "unsupported",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Ordering rank used to compare two providers' hot-reload strength; higher is
    /// stronger.
    const fn rank(self) -> u8 {
        match self {
            Self::NotApplicable => 0,
            Self::Unsupported => 1,
            Self::RestartOnly => 2,
            Self::Supported => 3,
        }
    }
}

/// Closed repair-action vocabulary. Names the repair guidance a degraded or bounded
/// row offers the operator, so a degraded provider is never a dead end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairActionClass {
    /// Re-verify the provider's declaration against the claimed row's requirement.
    ReverifyDeclaration,
    /// Reinstall or update the provider to restore its declaration.
    ReinstallProvider,
    /// Restore the prior stronger provider rather than accept the weaker one.
    RestoreStrongerProvider,
    /// Continue against the mirror/offline snapshot as bounded truth.
    UseMirrorOffline,
    /// Acknowledge the inspect-only profile; no write flow will be offered.
    AcknowledgeInspectOnly,
    /// Request policy elevation to widen the policy-limited profile.
    RequestPolicyElevation,
}

impl RepairActionClass {
    /// Stable token recorded in the repair guidance.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReverifyDeclaration => "reverify_declaration",
            Self::ReinstallProvider => "reinstall_provider",
            Self::RestoreStrongerProvider => "restore_stronger_provider",
            Self::UseMirrorOffline => "use_mirror_offline",
            Self::AcknowledgeInspectOnly => "acknowledge_inspect_only",
            Self::RequestPolicyElevation => "request_policy_elevation",
        }
    }
}

/// Closed conformance-downgrade-trigger vocabulary. Names why a claimed row carries
/// a bounded or degraded disclosure; the chrome quotes the trigger verbatim instead
/// of a generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConformanceDowngradeTrigger {
    /// The backing provider became unavailable.
    ProviderUnavailable,
    /// The provider's declaration went stale and needs re-verification.
    ProviderDeclarationStale,
    /// A weaker provider was proposed in place of a stronger one.
    WeakerProviderProposed,
    /// The provider does not declare the claimed target kind.
    TargetKindUnsupported,
    /// The provider's declared mapping quality regressed below the requirement.
    MappingQualityRegressed,
    /// The provider's declared attach depth dropped below the requirement.
    AttachDepthReduced,
    /// The provider withdrew its hot-reload declaration.
    HotReloadWithdrawn,
    /// Policy narrowed the row below its prior posture.
    PolicyNarrowed,
    /// Only a mirror / offline snapshot is reachable.
    OfflineMirrorOnly,
}

impl ConformanceDowngradeTrigger {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderUnavailable => "provider_unavailable",
            Self::ProviderDeclarationStale => "provider_declaration_stale",
            Self::WeakerProviderProposed => "weaker_provider_proposed",
            Self::TargetKindUnsupported => "target_kind_unsupported",
            Self::MappingQualityRegressed => "mapping_quality_regressed",
            Self::AttachDepthReduced => "attach_depth_reduced",
            Self::HotReloadWithdrawn => "hot_reload_withdrawn",
            Self::PolicyNarrowed => "policy_narrowed",
            Self::OfflineMirrorOnly => "offline_mirror_only",
        }
    }
}

/// A provider's declared capabilities for a claimed M5 row. A provider — first-party
/// or contributed — must carry one of these before it can back a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderDeclaration {
    /// Runtime target kinds the provider declares it can back.
    pub supported_target_kinds: Vec<BrowserRuntimeTargetKind>,
    /// Source-mapping quality classes the provider declares it can produce.
    pub supported_mapping_qualities: Vec<InspectorMappingQualityClass>,
    /// Deepest attach the provider declares.
    pub max_attach_depth: AttachDepthClass,
    /// Hot-reload posture the provider declares.
    pub hot_reload: HotReloadDeclarationClass,
    /// Opaque token naming the provider's client-scope limit (e.g. single-client,
    /// shared-session-capped). Never a raw handle or credential.
    pub client_scope_limit_token: String,
}

impl ProviderDeclaration {
    /// Whether the declaration carries the minimum a provider must declare: at least
    /// one supported target kind, at least one supported mapping quality, and a
    /// non-empty client-scope limit token.
    pub fn is_complete(&self) -> bool {
        !self.supported_target_kinds.is_empty()
            && !self.supported_mapping_qualities.is_empty()
            && !self.client_scope_limit_token.trim().is_empty()
    }

    /// Strongest source-mapping rank the declaration can produce; `0` if it declares
    /// none.
    fn best_mapping_rank(&self) -> u8 {
        self.supported_mapping_qualities
            .iter()
            .map(|q| mapping_quality_rank(*q))
            .max()
            .unwrap_or(0)
    }

    /// Whether this declaration satisfies a claimed row's requirement: it supports
    /// the required target kind, can produce a mapping quality at least as strong as
    /// required, attaches at least as deep as required, and declares hot reload when
    /// the row requires it.
    pub fn satisfies(&self, requirement: &ClaimedRowRequirement) -> bool {
        self.supported_target_kinds
            .contains(&requirement.required_target_kind)
            && self.supported_mapping_qualities.iter().any(|q| {
                mapping_quality_rank(*q)
                    >= mapping_quality_rank(requirement.required_mapping_quality)
            })
            && attach_depth_rank(self.max_attach_depth)
                >= attach_depth_rank(requirement.required_attach_depth)
            && (!requirement.requires_hot_reload
                || self.hot_reload == HotReloadDeclarationClass::Supported)
    }

    /// Whether this declaration is at least as strong as `other` across every
    /// declared dimension: it covers every target kind `other` covers, produces a
    /// mapping quality at least as strong, attaches at least as deep, and declares a
    /// hot-reload posture at least as strong.
    pub fn at_least_as_strong_as(&self, other: &ProviderDeclaration) -> bool {
        other
            .supported_target_kinds
            .iter()
            .all(|k| self.supported_target_kinds.contains(k))
            && self.best_mapping_rank() >= other.best_mapping_rank()
            && attach_depth_rank(self.max_attach_depth) >= attach_depth_rank(other.max_attach_depth)
            && self.hot_reload.rank() >= other.hot_reload.rank()
    }

    /// Whether this declaration is strictly weaker than `other` on some declared
    /// dimension.
    pub fn is_weaker_than(&self, other: &ProviderDeclaration) -> bool {
        !self.at_least_as_strong_as(other)
    }
}

/// What a claimed M5 row needs the backing provider to declare before it can be
/// backed live.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimedRowRequirement {
    /// Runtime target kind the claimed row requires.
    pub required_target_kind: BrowserRuntimeTargetKind,
    /// Minimum source-mapping quality the claimed row requires.
    pub required_mapping_quality: InspectorMappingQualityClass,
    /// Minimum attach depth the claimed row requires.
    pub required_attach_depth: AttachDepthClass,
    /// Whether the claimed row requires hot reload.
    pub requires_hot_reload: bool,
}

/// Repair guidance attached to a degraded or bounded row so a degraded provider is
/// never a dead end.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairGuidance {
    /// The repair action offered.
    pub action: RepairActionClass,
    /// Precise, operator-facing repair summary; never a generic non-answer or
    /// extension-private wording.
    pub guidance_summary: String,
}

/// One provider-conformance row: the truth packet for a single provider backing a
/// single claimed M5 preview/browser-runtime row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderConformanceRow {
    /// Stable row id.
    pub row_id: String,
    /// Human-readable label of the claimed M5 row this provider backs.
    pub claimed_surface_label: String,
    /// Opaque provider id. Never a raw handle, URL, or credential.
    pub provider_id: String,
    /// Whether the provider is first-party or contributed.
    pub provider_origin: ProviderOriginClass,
    /// The provider's currently-backing declaration.
    pub declaration: ProviderDeclaration,
    /// What the claimed row requires of its backing provider.
    pub requirement: ClaimedRowRequirement,
    /// The prior provider declaration preserved as history; required whenever the
    /// status preserves history (stale, weaker, or unavailable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_declaration: Option<ProviderDeclaration>,
    /// The conformance status of the backing provider right now.
    pub status: ProviderStatusClass,
    /// The bounded operating posture the claimed row currently presents.
    pub operating_profile: OperatingProfileClass,
    /// Whether the row offers a write-capable designer flow; only valid on a live,
    /// conformant row.
    pub offers_write_capable_flow: bool,
    /// Repair guidance; required whenever the row is non-conformant or bounded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair: Option<RepairGuidance>,
    /// Trigger that fired the bounded/degraded disclosure; required whenever the row
    /// is non-conformant or bounded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<ConformanceDowngradeTrigger>,
    /// Precise degraded label; required when the row carries a downgrade trigger.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,
    /// Human-readable label summary safe to render on the row.
    pub label_summary: String,
    /// ISO 8601 UTC timestamp the conformance state was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
}

impl ProviderConformanceRow {
    /// Whether this row needs a bounded/degraded disclosure: a non-conformant
    /// provider or a bounded operating profile must always disclose why.
    pub fn needs_disclosure(&self) -> bool {
        !self.status.is_conformant() || self.operating_profile.is_bounded()
    }

    /// Whether a live profile is honestly backed: a live row must be conformant, its
    /// declaration must actually satisfy the requirement, and it must carry no
    /// downgrade disclosure.
    pub fn live_conformance_ok(&self) -> bool {
        if self.operating_profile.is_live() {
            self.status.is_conformant()
                && self.declaration.satisfies(&self.requirement)
                && self.downgrade_trigger.is_none()
        } else {
            true
        }
    }

    /// Whether a non-conformant provider never stays on a live profile.
    pub fn nonconformant_not_live_ok(&self) -> bool {
        self.status.is_conformant() || !self.operating_profile.is_live()
    }

    /// Whether the prior-declaration presence matches the status: a history-preserving
    /// status carries one; a conformant status carries none.
    pub fn prior_declaration_ok(&self) -> bool {
        if self.status.requires_prior_declaration() {
            self.prior_declaration
                .as_ref()
                .is_some_and(ProviderDeclaration::is_complete)
        } else {
            self.prior_declaration.is_none()
        }
    }

    /// Whether a weaker-replacement row is honest: the prior declaration is present
    /// and the current declaration is actually strictly weaker than it.
    pub fn weaker_replacement_ok(&self) -> bool {
        if self.status == ProviderStatusClass::WeakerReplacement {
            self.prior_declaration
                .as_ref()
                .is_some_and(|prior| self.declaration.is_weaker_than(prior))
        } else {
            true
        }
    }

    /// Whether the repair-guidance presence matches the row: a non-conformant or
    /// bounded row carries precise repair guidance; a clean live row carries none.
    pub fn repair_ok(&self) -> bool {
        match &self.repair {
            Some(repair) => self.needs_disclosure() && !label_is_generic(&repair.guidance_summary),
            None => !self.needs_disclosure(),
        }
    }

    /// Whether the downgrade disclosure is consistent: a row that needs disclosure
    /// carries a trigger and a precise non-generic degraded label; a clean live row
    /// carries neither.
    pub fn downgrade_ok(&self) -> bool {
        let trigger_ok = self.downgrade_trigger.is_some() == self.needs_disclosure();
        let label_ok = match &self.degraded_label {
            Some(label) => self.downgrade_trigger.is_some() && !label_is_generic(label),
            None => self.downgrade_trigger.is_none(),
        };
        trigger_ok && label_ok
    }

    /// Whether the write-capability stays honest: a write-capable designer flow may
    /// appear only on a live, conformant row, never on an inspect-only, mirror/offline,
    /// or policy-limited row.
    pub fn write_capability_ok(&self) -> bool {
        if self.offers_write_capable_flow {
            self.operating_profile.is_live() && self.status.is_conformant()
        } else {
            true
        }
    }

    /// Whether this row demonstrates a bounded/degraded disclosure.
    pub fn has_disclosure(&self) -> bool {
        self.downgrade_trigger.is_some()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "origin={origin} status={status} profile={profile} target={target} \
mapping={mapping} attach={attach} hot_reload={hot_reload}",
            origin = self.provider_origin.as_str(),
            status = self.status.as_str(),
            profile = self.operating_profile.as_str(),
            target = self.requirement.required_target_kind.as_str(),
            mapping = self.requirement.required_mapping_quality.as_str(),
            attach = self.requirement.required_attach_depth.as_str(),
            hot_reload = self.declaration.hot_reload.as_str(),
        )
    }

    /// Whether every dimension required to record this row is present and internally
    /// consistent.
    pub fn is_complete(&self) -> bool {
        !self.row_id.trim().is_empty()
            && !self.claimed_surface_label.trim().is_empty()
            && !self.provider_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.observed_at.trim().is_empty()
            && self.declaration.is_complete()
            && self.live_conformance_ok()
            && self.nonconformant_not_live_ok()
            && self.prior_declaration_ok()
            && self.weaker_replacement_ok()
            && self.repair_ok()
            && self.downgrade_ok()
            && self.write_capability_ok()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block for the extension-provider conformance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceGuardrails {
    /// Source remains canonical; the conformance packet is derivative, never a
    /// second writable truth model.
    pub source_canonical_no_second_writable_model: bool,
    /// Runtime state and extension-private wording never hide source-mapping
    /// uncertainty behind a conformance label.
    pub runtime_state_never_hides_source_mapping_uncertainty: bool,
    /// Inspect-only rows are never auto-upgraded into write-capable designer flows.
    pub inspect_only_never_auto_upgraded_to_write: bool,
    /// Embedded preview / browser boundaries are not blurred into product authority.
    pub embedded_boundaries_not_blurred_into_product: bool,
    /// A weaker provider never silently swaps in stronger-looking semantics.
    pub weaker_provider_never_silently_swaps_semantics: bool,
    /// A stale or unavailable provider preserves prior history and limitation notes.
    pub stale_or_unavailable_provider_preserves_history: bool,
    /// Mirror/offline, inspect-only, and policy-limited profiles stay explicit and
    /// exportable on every claimed row.
    pub bounded_profiles_explicit_and_exportable: bool,
}

impl ConformanceGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.source_canonical_no_second_writable_model
            && self.runtime_state_never_hides_source_mapping_uncertainty
            && self.inspect_only_never_auto_upgraded_to_write
            && self.embedded_boundaries_not_blurred_into_product
            && self.weaker_provider_never_silently_swaps_semantics
            && self.stale_or_unavailable_provider_preserves_history
            && self.bounded_profiles_explicit_and_exportable
    }
}

/// Consumer-projection block for the extension-provider conformance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConformanceConsumerProjection {
    /// Product surfaces ingest these conformance rows instead of cloning chip text.
    pub product_ingests_conformance: bool,
    /// Docs/help ingests the same conformance rows.
    pub docs_help_ingests_conformance: bool,
    /// Diagnostics ingests the same conformance rows.
    pub diagnostics_ingests_conformance: bool,
    /// Support export ingests the same conformance rows.
    pub support_export_ingests_conformance: bool,
    /// Release-control surfaces ingest the same conformance rows.
    pub release_control_ingests_conformance: bool,
    /// Support / diagnostics exports can reconstruct the operating profile the user
    /// saw for each claimed row.
    pub support_export_reconstructs_operating_profile: bool,
}

impl ConformanceConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_conformance
            && self.docs_help_ingests_conformance
            && self.diagnostics_ingests_conformance
            && self.support_export_ingests_conformance
            && self.release_control_ingests_conformance
            && self.support_export_reconstructs_operating_profile
    }
}

/// Constructor input for [`ProviderConformancePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderConformancePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Per-provider conformance rows.
    pub rows: Vec<ProviderConformanceRow>,
    /// Guardrail invariants block.
    pub guardrails: ConformanceGuardrails,
    /// Consumer projection block.
    pub consumer_projection: ConformanceConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe extension-provider conformance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderConformancePacket {
    /// Record kind; must equal [`EXTENSION_PROVIDER_CONFORMANCE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`EXTENSION_PROVIDER_CONFORMANCE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Per-provider conformance rows.
    pub rows: Vec<ProviderConformanceRow>,
    /// Guardrail invariants block.
    pub guardrails: ConformanceGuardrails,
    /// Consumer projection block.
    pub consumer_projection: ConformanceConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ProviderConformancePacket {
    /// Builds an extension-provider conformance packet.
    pub fn new(input: ProviderConformancePacketInput) -> Self {
        Self {
            record_kind: EXTENSION_PROVIDER_CONFORMANCE_RECORD_KIND.to_owned(),
            schema_version: EXTENSION_PROVIDER_CONFORMANCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            set_label: input.set_label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Provider origins represented by some row in this packet.
    pub fn represented_provider_origins(&self) -> BTreeSet<ProviderOriginClass> {
        self.rows.iter().map(|r| r.provider_origin).collect()
    }

    /// Provider statuses represented by some row in this packet.
    pub fn represented_statuses(&self) -> BTreeSet<ProviderStatusClass> {
        self.rows.iter().map(|r| r.status).collect()
    }

    /// Operating profiles represented by some row in this packet.
    pub fn represented_operating_profiles(&self) -> BTreeSet<OperatingProfileClass> {
        self.rows.iter().map(|r| r.operating_profile).collect()
    }

    /// Count of rows that carry a bounded/degraded disclosure.
    pub fn disclosed_row_count(&self) -> usize {
        self.rows.iter().filter(|r| r.has_disclosure()).count()
    }

    /// Count of rows that offer a write-capable designer flow.
    pub fn write_capable_row_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|r| r.offers_write_capable_flow)
            .count()
    }

    /// Validates the extension-provider conformance packet invariants.
    pub fn validate(&self) -> Vec<ProviderConformanceViolation> {
        let mut violations = Vec::new();

        if self.record_kind != EXTENSION_PROVIDER_CONFORMANCE_RECORD_KIND {
            violations.push(ProviderConformanceViolation::WrongRecordKind);
        }
        if self.schema_version != EXTENSION_PROVIDER_CONFORMANCE_SCHEMA_VERSION {
            violations.push(ProviderConformanceViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.set_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ProviderConformanceViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("provider conformance packet serializes"),
        ) {
            violations.push(ProviderConformanceViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("provider conformance packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Extension-Provider Conformance\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.set_label));
        out.push_str(&format!(
            "- Rows: {} ({} write-capable, {} disclosed)\n",
            self.rows.len(),
            self.write_capable_row_count(),
            self.disclosed_row_count()
        ));
        out.push_str(&format!(
            "- Provider origins: {} / {}\n",
            self.represented_provider_origins().len(),
            ProviderOriginClass::ALL.len()
        ));
        out.push_str(&format!(
            "- Provider statuses: {} / {}\n",
            self.represented_statuses().len(),
            ProviderStatusClass::ALL.len()
        ));
        out.push_str(&format!(
            "- Operating profiles: {} / {}\n",
            self.represented_operating_profiles().len(),
            OperatingProfileClass::ALL.len()
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({})\n",
                row.row_id, row.claimed_surface_label
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!("  - {}\n", row.chip_tokens()));
            if let Some(repair) = &row.repair {
                out.push_str(&format!(
                    "  - Repair: action={} — {}\n",
                    repair.action.as_str(),
                    repair.guidance_summary,
                ));
            }
            if let Some(label) = &row.degraded_label {
                out.push_str(&format!("  - Disclosed: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in extension-provider conformance export.
#[derive(Debug)]
pub enum ProviderConformanceArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ProviderConformanceViolation>),
}

impl fmt::Display for ProviderConformanceArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "provider conformance export parse failed: {error}"
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
                    "provider conformance export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ProviderConformanceArtifactError {}

/// Validation failures emitted by [`ProviderConformancePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderConformanceViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required operating profile is represented by no row.
    RequiredOperatingProfileMissing,
    /// A required provider status is represented by no row.
    RequiredProviderStatusMissing,
    /// Both first-party and contributed provider origins are not represented.
    ProviderOriginCoverageMissing,
    /// The packet demonstrates no consistent weaker-replacement repair row.
    WeakerReplacementCaseMissing,
    /// The packet demonstrates no bounded-profile disclosure row.
    BoundedProfileCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A row's provider declaration is incomplete.
    DeclarationIncomplete,
    /// A live row is not backed by a conformant, satisfying provider.
    LiveRequiresConformance,
    /// A non-conformant provider is still presented on a live profile.
    NonconformantRowStillLive,
    /// A row's prior-declaration presence is inconsistent with its status.
    PriorDeclarationInconsistent,
    /// A weaker-replacement row's current declaration is not actually weaker.
    WeakerReplacementNotWeaker,
    /// A row's repair guidance presence or precision is inconsistent.
    RepairGuidanceInconsistent,
    /// A row's downgrade trigger / label is inconsistent.
    DowngradeInconsistent,
    /// A row offers a write-capable flow on a non-live or non-conformant row.
    WriteCapabilityUnbacked,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ProviderConformanceViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredOperatingProfileMissing => "required_operating_profile_missing",
            Self::RequiredProviderStatusMissing => "required_provider_status_missing",
            Self::ProviderOriginCoverageMissing => "provider_origin_coverage_missing",
            Self::WeakerReplacementCaseMissing => "weaker_replacement_case_missing",
            Self::BoundedProfileCaseMissing => "bounded_profile_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::DeclarationIncomplete => "declaration_incomplete",
            Self::LiveRequiresConformance => "live_requires_conformance",
            Self::NonconformantRowStillLive => "nonconformant_row_still_live",
            Self::PriorDeclarationInconsistent => "prior_declaration_inconsistent",
            Self::WeakerReplacementNotWeaker => "weaker_replacement_not_weaker",
            Self::RepairGuidanceInconsistent => "repair_guidance_inconsistent",
            Self::DowngradeInconsistent => "downgrade_inconsistent",
            Self::WriteCapabilityUnbacked => "write_capability_unbacked",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in extension-provider conformance export.
pub fn current_m5_extension_provider_conformance_export(
) -> Result<ProviderConformancePacket, ProviderConformanceArtifactError> {
    let packet: ProviderConformancePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/preview/m5/extension_provider_conformance/support_export.json"
    )))
    .map_err(ProviderConformanceArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ProviderConformanceArtifactError::Validation(violations))
    }
}

/// Numeric rank of a source-mapping quality; stronger (more source-faithful)
/// mappings rank higher.
const fn mapping_quality_rank(quality: InspectorMappingQualityClass) -> u8 {
    match quality {
        InspectorMappingQualityClass::RuntimeOnly => 1,
        InspectorMappingQualityClass::GeneratedOnly => 2,
        InspectorMappingQualityClass::Approximate => 3,
        InspectorMappingQualityClass::Exact => 4,
    }
}

/// Numeric rank of an attach depth on the DOM → styles → network → storage ladder;
/// deeper attaches rank higher.
const fn attach_depth_rank(depth: AttachDepthClass) -> u8 {
    match depth {
        AttachDepthClass::NoAttach | AttachDepthClass::NotApplicableNonBrowser => 0,
        AttachDepthClass::DomOnly => 1,
        AttachDepthClass::DomAndStyles => 2,
        AttachDepthClass::DomStylesNetwork => 3,
        AttachDepthClass::DomStylesNetworkStorage => 4,
    }
}

fn validate_source_contracts(
    packet: &ProviderConformancePacket,
    violations: &mut Vec<ProviderConformanceViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        EXTENSION_PROVIDER_CONFORMANCE_SCHEMA_REF,
        EXTENSION_PROVIDER_CONFORMANCE_DOC_REF,
        EXTENSION_PROVIDER_CONFORMANCE_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ProviderConformanceViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &ProviderConformancePacket,
    violations: &mut Vec<ProviderConformanceViolation>,
) {
    let profiles = packet.represented_operating_profiles();
    for required in OperatingProfileClass::ALL {
        if !profiles.contains(&required) {
            violations.push(ProviderConformanceViolation::RequiredOperatingProfileMissing);
            break;
        }
    }

    let statuses = packet.represented_statuses();
    for required in ProviderStatusClass::ALL {
        if !statuses.contains(&required) {
            violations.push(ProviderConformanceViolation::RequiredProviderStatusMissing);
            break;
        }
    }

    let origins = packet.represented_provider_origins();
    for required in ProviderOriginClass::ALL {
        if !origins.contains(&required) {
            violations.push(ProviderConformanceViolation::ProviderOriginCoverageMissing);
            break;
        }
    }

    if !packet.rows.iter().any(|r| {
        r.status == ProviderStatusClass::WeakerReplacement
            && r.weaker_replacement_ok()
            && r.is_complete()
    }) {
        violations.push(ProviderConformanceViolation::WeakerReplacementCaseMissing);
    }

    if !packet
        .rows
        .iter()
        .any(|r| r.operating_profile.is_bounded() && r.has_disclosure() && r.downgrade_ok())
    {
        violations.push(ProviderConformanceViolation::BoundedProfileCaseMissing);
    }
}

fn validate_rows(
    packet: &ProviderConformancePacket,
    violations: &mut Vec<ProviderConformanceViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(ProviderConformanceViolation::RowIncomplete);
        }
        if !row.declaration.is_complete() {
            violations.push(ProviderConformanceViolation::DeclarationIncomplete);
        }
        if !row.live_conformance_ok() {
            violations.push(ProviderConformanceViolation::LiveRequiresConformance);
        }
        if !row.nonconformant_not_live_ok() {
            violations.push(ProviderConformanceViolation::NonconformantRowStillLive);
        }
        if !row.prior_declaration_ok() {
            violations.push(ProviderConformanceViolation::PriorDeclarationInconsistent);
        }
        if !row.weaker_replacement_ok() {
            violations.push(ProviderConformanceViolation::WeakerReplacementNotWeaker);
        }
        if !row.repair_ok() {
            violations.push(ProviderConformanceViolation::RepairGuidanceInconsistent);
        }
        if !row.downgrade_ok() {
            violations.push(ProviderConformanceViolation::DowngradeInconsistent);
        }
        if !row.write_capability_ok() {
            violations.push(ProviderConformanceViolation::WriteCapabilityUnbacked);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(ProviderConformanceViolation::RowEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &ProviderConformancePacket,
    violations: &mut Vec<ProviderConformanceViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(ProviderConformanceViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &ProviderConformancePacket,
    violations: &mut Vec<ProviderConformanceViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(ProviderConformanceViolation::ConsumerProjectionIncomplete);
    }
}

/// Whether a degraded or repair label is a generic non-answer rather than a precise
/// operator-facing label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "stale"
            | "downgraded"
            | "disconnected"
            | "no provider"
            | "try again"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("set-cookie")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
