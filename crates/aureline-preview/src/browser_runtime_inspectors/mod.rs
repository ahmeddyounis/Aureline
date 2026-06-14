//! DOM / CSS / console / network / storage browser-runtime inspectors with
//! target-kind, attach-depth, mapping-quality, and redaction-safe session
//! continuity.
//!
//! Where
//! [`crate::freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix`]
//! freezes the *qualification matrix* over claimed preview/runtime surfaces,
//! [`crate::preview_session_descriptors`] materializes the *per-session*
//! descriptor each surface presents, and [`crate::inspect_to_source_tree`]
//! materializes the *per-node* source-mapping truth, this module materializes
//! the **per-inspector** truth packet behind every claimed browser-runtime
//! inspection surface: one shared packet that teaches each DOM, CSS, console,
//! network, or storage inspector to say which runtime target it is attached to,
//! how deep the attach reaches, how good its source mapping is, how fresh the
//! session is, and how its values are redacted — before any value body, jump, or
//! mutation affordance appears.
//!
//! The packet is the one canonical answer to "for the inspector the user is
//! looking at, which runtime or device am I seeing, how deep is the attach, how
//! good is the source mapping, what data posture applies, and is a mutation
//! against it safe?" A [`BrowserRuntimeInspectorPacket`] binds the five inspector
//! lanes onto the same governed vocabulary — [`InspectorKind`],
//! [`BrowserRuntimeTargetKind`], [`crate::AttachDepthClass`],
//! [`InspectorMappingQualityClass`], [`SessionFreshnessClass`],
//! [`SessionContinuityClass`], and [`RedactionPostureClass`] — instead of
//! provider-specific extension chrome.
//!
//! Source stays canonical and the inspector packet is derivative — never a
//! second writable truth model. An [`InspectorRow`] keeps the honesty rules the
//! spec freezes:
//!
//! - **One target-kind vocabulary.** Every claimed browser-runtime surface names
//!   its target through [`BrowserRuntimeTargetKind`] — embedded preview, external
//!   browser, simulator/emulator, device browser, remote preview session, or
//!   captured snapshot — so the chrome can never blur an embedded preview into a
//!   product-native browser or a captured snapshot into a live runtime.
//! - **Honest attach depth.** The recorded [`crate::AttachDepthClass`] must
//!   actually reach the inspector lane it backs; a shallow attach never
//!   advertises storage or network inspection by silence.
//! - **Redaction-safe by default.** Console, network, and storage inspectors
//!   carry sensitive value bodies; their redaction posture must hide those values
//!   by default so cookies, tokens, storage entries, and request bodies never
//!   leak into generic diagnostics or support exports.
//! - **No runtime masquerade.** A runtime-only or captured-snapshot inspector
//!   never claims to be saved source state, and a stale or imported session never
//!   silently presents as a live one.
//! - **No inspect-to-write auto-upgrade.** A mutation-capable browser-runtime
//!   action cannot appear without an explicit [`SideEffectClass`], a target
//!   identity, and a review or confirmation [`MutationReviewPosture`]; it may
//!   never target a captured snapshot.
//! - **Attributable continuity.** Session identity is threaded into reconnect,
//!   imported-snapshot, and stale-session continuity so browser-runtime history
//!   stays attributable across attach, reconnect, and export.
//!
//! Raw URLs, hostnames, cookies, tokens, storage entries, request/response
//! bodies, raw provider payloads, credentials, and raw runtime handles never
//! cross this boundary; the packet carries only typed class tokens, opaque
//! target/session/evidence refs, booleans, and redacted labels, so support and
//! diagnostics exports can reconstruct exactly what data posture the user saw for
//! each inspector.
//!
//! The boundary schema is
//! [`schemas/preview/browser_runtime_inspectors.schema.json`](../../../../schemas/preview/browser_runtime_inspectors.schema.json).
//! The contract doc is
//! [`docs/preview/m5/browser_runtime_inspectors.md`](../../../../docs/preview/m5/browser_runtime_inspectors.md).
//! The protected fixture directory is
//! [`fixtures/preview/m5/browser_runtime_inspectors/`](../../../../fixtures/preview/m5/browser_runtime_inspectors/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::AttachDepthClass;

/// Stable record-kind tag carried by [`BrowserRuntimeInspectorPacket`].
pub const BROWSER_RUNTIME_INSPECTORS_RECORD_KIND: &str = "browser_runtime_inspectors";

/// Schema version for the browser-runtime inspector packet.
pub const BROWSER_RUNTIME_INSPECTORS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const BROWSER_RUNTIME_INSPECTORS_SCHEMA_REF: &str =
    "schemas/preview/browser_runtime_inspectors.schema.json";

/// Repo-relative path of the contract doc.
pub const BROWSER_RUNTIME_INSPECTORS_DOC_REF: &str =
    "docs/preview/m5/browser_runtime_inspectors.md";

/// Repo-relative path of the protected fixture directory.
pub const BROWSER_RUNTIME_INSPECTORS_FIXTURE_DIR: &str =
    "fixtures/preview/m5/browser_runtime_inspectors";

/// Repo-relative path of the checked support-export artifact.
pub const BROWSER_RUNTIME_INSPECTORS_ARTIFACT_REF: &str =
    "artifacts/preview/m5/browser_runtime_inspectors/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const BROWSER_RUNTIME_INSPECTORS_SUMMARY_REF: &str =
    "artifacts/preview/m5/browser_runtime_inspectors.md";

/// Closed inspector-lane vocabulary. Names which browser-runtime inspector a row
/// belongs to so DOM, CSS, console, network, and storage inspection all normalize
/// onto the same packet instead of bespoke per-provider chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorKind {
    /// Live DOM element tree inspection.
    Dom,
    /// Computed and authored style (CSS) inspection.
    Css,
    /// Runtime console message inspection.
    Console,
    /// Network request/response activity inspection.
    Network,
    /// Storage inspection (cookies, local/session storage, IndexedDB, etc.).
    Storage,
}

impl InspectorKind {
    /// Every inspector lane a claimed M5 browser-runtime surface must cover, in
    /// declaration order.
    pub const ALL: [Self; 5] = [
        Self::Dom,
        Self::Css,
        Self::Console,
        Self::Network,
        Self::Storage,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dom => "dom",
            Self::Css => "css",
            Self::Console => "console",
            Self::Network => "network",
            Self::Storage => "storage",
        }
    }

    /// True when the inspector lane surfaces sensitive value bodies — console
    /// messages, network request/response bodies, or storage entries — that must
    /// be redacted by default. DOM structure and computed CSS metadata are not
    /// sensitive value surfaces.
    pub const fn is_sensitive_value_surface(self) -> bool {
        matches!(self, Self::Console | Self::Network | Self::Storage)
    }

    /// Minimum attach-depth rank the inspector lane needs before it can honestly
    /// claim to be backed. Higher means a deeper attach is required.
    const fn required_attach_rank(self) -> u8 {
        match self {
            // Console needs a live runtime attach but not a DOM walk per se; the
            // shallowest live attach (`dom_only`) is the honest floor.
            Self::Dom | Self::Console => 1,
            Self::Css => 2,
            Self::Network => 3,
            Self::Storage => 4,
        }
    }

    /// Whether the recorded attach depth actually reaches this inspector lane, so
    /// a shallow attach never advertises a deeper inspector by silence.
    pub const fn attach_depth_supports(self, depth: AttachDepthClass) -> bool {
        attach_depth_rank(depth) >= self.required_attach_rank()
    }
}

/// Closed browser-runtime target-kind vocabulary. The single vocabulary through
/// which a claimed M5 browser-runtime surface distinguishes which runtime or
/// device the user is inspecting. Adding a value is additive-minor; repurposing
/// is breaking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserRuntimeTargetKind {
    /// An embedded preview rendered inside the shell or extension host.
    EmbeddedPreview,
    /// An external/system browser process Aureline drives over a transport.
    ExternalBrowser,
    /// An OS-vendor simulator or emulator runtime.
    SimulatorOrEmulator,
    /// A browser running on a tethered physical device.
    DeviceBrowser,
    /// A preview session running on a remote / container / managed runtime.
    RemotePreviewSession,
    /// A captured snapshot with no live runtime behind it.
    CapturedSnapshot,
}

impl BrowserRuntimeTargetKind {
    /// Every target kind a claimed surface must be able to name, in declaration
    /// order.
    pub const ALL: [Self; 6] = [
        Self::EmbeddedPreview,
        Self::ExternalBrowser,
        Self::SimulatorOrEmulator,
        Self::DeviceBrowser,
        Self::RemotePreviewSession,
        Self::CapturedSnapshot,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmbeddedPreview => "embedded_preview",
            Self::ExternalBrowser => "external_browser",
            Self::SimulatorOrEmulator => "simulator_or_emulator",
            Self::DeviceBrowser => "device_browser",
            Self::RemotePreviewSession => "remote_preview_session",
            Self::CapturedSnapshot => "captured_snapshot",
        }
    }

    /// True when the target is a live runtime whose state can diverge from saved
    /// source. A captured snapshot is not a live runtime.
    pub const fn is_live_runtime(self) -> bool {
        !matches!(self, Self::CapturedSnapshot)
    }
}

/// Closed inspector mapping-quality vocabulary. Names how good the source mapping
/// behind an inspected element is, mirroring the inspect-to-source labels so a
/// browser-runtime inspector never advertises a stronger mapping than it holds.
///
/// `exact`          — maps unambiguously to a canonical-source span.
/// `approximate`    — maps to source heuristically; a jump lands near, not exactly.
/// `generated_only` — corresponds to generated output (e.g. compiled CSS) with no
///                    hand-authored span.
/// `runtime_only`   — exists only in the live runtime with no source backing; it
///                    must never claim saved source state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorMappingQualityClass {
    Exact,
    Approximate,
    GeneratedOnly,
    RuntimeOnly,
}

impl InspectorMappingQualityClass {
    /// Every mapping-quality class a claimed surface must demonstrate, in
    /// declaration order.
    pub const ALL: [Self; 4] = [
        Self::Exact,
        Self::Approximate,
        Self::GeneratedOnly,
        Self::RuntimeOnly,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::GeneratedOnly => "generated_only",
            Self::RuntimeOnly => "runtime_only",
        }
    }

    /// True when the inspected element maps back to a canonical-source span
    /// (exact or approximate) and so may claim saved-source backing.
    pub const fn is_source_backed(self) -> bool {
        matches!(self, Self::Exact | Self::Approximate)
    }

    /// True when the inspected element has no source backing at all.
    pub const fn is_runtime_only(self) -> bool {
        matches!(self, Self::RuntimeOnly)
    }
}

/// Closed session-freshness vocabulary. Names how fresh the inspector session is
/// right now so a stale or captured view never silently presents as live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionFreshnessClass {
    /// Attached to a live runtime; the view reflects current runtime state.
    Live,
    /// Re-attached after a transport drop; the prior session is re-pinned.
    Reconnected,
    /// The session went stale; the view may not reflect current runtime state.
    Stale,
    /// A captured snapshot with no live runtime behind it.
    CapturedSnapshot,
}

impl SessionFreshnessClass {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Reconnected => "reconnected",
            Self::Stale => "stale",
            Self::CapturedSnapshot => "captured_snapshot",
        }
    }

    /// True when the session reflects current live runtime state.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::Live)
    }
}

/// Closed session-continuity vocabulary. Names how this inspector's session was
/// obtained so reconnect, imported-snapshot, and stale-session history stays
/// attributable and a downgraded session never silently re-upgrades.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionContinuityClass {
    /// A fresh attach to a live runtime; there is no prior session to carry.
    FreshAttach,
    /// A reconnect that re-pins a prior session; the prior session ref is carried.
    Reconnected,
    /// An imported captured snapshot; the originating session ref is carried.
    ImportedSnapshot,
    /// A session whose runtime went away; the prior session ref is carried.
    StaleSession,
}

impl SessionContinuityClass {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshAttach => "fresh_attach",
            Self::Reconnected => "reconnected",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::StaleSession => "stale_session",
        }
    }

    /// True when this continuity carries a prior session ref for attributability.
    pub const fn carries_prior_session(self) -> bool {
        !matches!(self, Self::FreshAttach)
    }

    /// True when this continuity is a degraded state that must record a trigger
    /// and a precise degraded label.
    pub const fn requires_downgrade(self) -> bool {
        matches!(self, Self::ImportedSnapshot | Self::StaleSession)
    }

    /// Whether the recorded freshness is consistent with this continuity, so a
    /// session can never claim a continuity its freshness contradicts.
    pub const fn consistent_with_freshness(self, freshness: SessionFreshnessClass) -> bool {
        matches!(
            (self, freshness),
            (Self::FreshAttach, SessionFreshnessClass::Live)
                | (Self::Reconnected, SessionFreshnessClass::Reconnected)
                | (
                    Self::ImportedSnapshot,
                    SessionFreshnessClass::CapturedSnapshot
                )
                | (Self::StaleSession, SessionFreshnessClass::Stale)
        )
    }
}

/// Closed redaction-posture vocabulary. Names how an inspector handles its value
/// bodies so cookies, tokens, storage entries, and request/response bodies never
/// leak into generic diagnostics or support exports by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionPostureClass {
    /// Sensitive values are redacted; only typed metadata crosses the boundary.
    RedactedByDefault,
    /// Only counts / keys / types cross the boundary; no values at all.
    MetadataOnly,
    /// Values are replaced by opaque hashes or refs, never the raw body.
    HashedReference,
    /// The inspector exposes no sensitive value class (e.g. DOM structure or
    /// computed-style metadata); values may pass without redaction.
    NonSensitivePassthrough,
}

impl RedactionPostureClass {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RedactedByDefault => "redacted_by_default",
            Self::MetadataOnly => "metadata_only",
            Self::HashedReference => "hashed_reference",
            Self::NonSensitivePassthrough => "non_sensitive_passthrough",
        }
    }

    /// True when this posture hides sensitive value bodies by default and so is
    /// safe for a sensitive inspector lane.
    pub const fn hides_sensitive_values(self) -> bool {
        matches!(
            self,
            Self::RedactedByDefault | Self::MetadataOnly | Self::HashedReference
        )
    }

    /// True when this posture passes values through unredacted and is therefore
    /// only valid for a non-sensitive inspector lane.
    pub const fn is_passthrough(self) -> bool {
        matches!(self, Self::NonSensitivePassthrough)
    }
}

/// Closed side-effect vocabulary. Names what a mutation-capable browser-runtime
/// action does so a write can never appear without an explicit, typed side-effect
/// class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectClass {
    /// Mutates the live DOM tree.
    DomMutation,
    /// Overrides a computed/authored style.
    StyleOverride,
    /// Writes a storage entry (cookie / local / session / IndexedDB).
    StorageWrite,
    /// Replays / refires a network request.
    NetworkReplay,
    /// Evaluates an expression in the runtime console.
    ConsoleEval,
}

impl SideEffectClass {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DomMutation => "dom_mutation",
            Self::StyleOverride => "style_override",
            Self::StorageWrite => "storage_write",
            Self::NetworkReplay => "network_replay",
            Self::ConsoleEval => "console_eval",
        }
    }

    /// Whether this side-effect class is the one an inspector lane can produce, so
    /// a storage write never appears on a DOM inspector by mislabelling.
    pub const fn matches_inspector(self, inspector: InspectorKind) -> bool {
        matches!(
            (self, inspector),
            (Self::DomMutation, InspectorKind::Dom)
                | (Self::StyleOverride, InspectorKind::Css)
                | (Self::ConsoleEval, InspectorKind::Console)
                | (Self::NetworkReplay, InspectorKind::Network)
                | (Self::StorageWrite, InspectorKind::Storage)
        )
    }
}

/// Closed mutation review-posture vocabulary. Names the review or confirmation a
/// mutation-capable action requires; a browser-runtime mutation can never appear
/// without one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationReviewPosture {
    /// The user must confirm the action before it fires.
    ConfirmationRequired,
    /// The action requires explicit review before it fires.
    ReviewRequired,
    /// The action is blocked pending elevation; it cannot fire yet.
    BlockedNeedsElevation,
}

impl MutationReviewPosture {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConfirmationRequired => "confirmation_required",
            Self::ReviewRequired => "review_required",
            Self::BlockedNeedsElevation => "blocked_needs_elevation",
        }
    }
}

/// Closed inspector-downgrade-trigger vocabulary. Names why a session's continuity
/// was preserved through a downgrade event; the chrome quotes the trigger verbatim
/// instead of a generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorDowngradeTrigger {
    /// The runtime reconnected and the prior session could not stay live.
    RuntimeReconnect,
    /// The inspection provider was lost (e.g. the devtools bridge went away).
    ProviderLoss,
    /// The session expired.
    SessionExpired,
    /// A captured snapshot was imported in place of a live session.
    SnapshotImported,
    /// The attach depth was reduced below what the inspector previously claimed.
    AttachDepthReduced,
    /// The redaction policy tightened and narrowed what values are visible.
    RedactionPolicyTightened,
    /// Policy narrowed the session below its prior posture.
    PolicyNarrowed,
}

impl InspectorDowngradeTrigger {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeReconnect => "runtime_reconnect",
            Self::ProviderLoss => "provider_loss",
            Self::SessionExpired => "session_expired",
            Self::SnapshotImported => "snapshot_imported",
            Self::AttachDepthReduced => "attach_depth_reduced",
            Self::RedactionPolicyTightened => "redaction_policy_tightened",
            Self::PolicyNarrowed => "policy_narrowed",
        }
    }
}

/// A mutation-capable browser-runtime action plan attached to an inspector row.
/// Its presence is the write affordance; the spec requires that it can never
/// appear without an explicit side-effect class and a review/confirmation posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationDescriptor {
    /// What the mutation does.
    pub side_effect_class: SideEffectClass,
    /// The review or confirmation the mutation requires.
    pub review_posture: MutationReviewPosture,
}

/// One browser-runtime inspector: the shared truth packet a single DOM, CSS,
/// console, network, or storage inspector presents before any value body, jump,
/// or mutation affordance appears.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorRow {
    /// Stable inspector id.
    pub inspector_id: String,
    /// Which inspector lane this row backs.
    pub inspector_kind: InspectorKind,
    /// Which runtime target this inspector is attached to.
    pub target_kind: BrowserRuntimeTargetKind,
    /// Opaque ref naming the target identity; required so a mutation can name what
    /// it acts on. Never a raw URL, hostname, or runtime handle.
    pub target_identity_ref: String,
    /// Stable id of the inspector session.
    pub session_id: String,
    /// Opaque ref to the prior session this one continues; required for any
    /// continuity other than a fresh attach so history stays attributable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_session_ref: Option<String>,
    /// How this inspector's session was obtained.
    pub continuity: SessionContinuityClass,
    /// How deep the runtime attach reaches.
    pub attach_depth: AttachDepthClass,
    /// How good the source mapping behind the inspected element is.
    pub mapping_quality: InspectorMappingQualityClass,
    /// How fresh the inspector session is right now.
    pub freshness: SessionFreshnessClass,
    /// How this inspector handles its value bodies.
    pub redaction_posture: RedactionPostureClass,
    /// Human-readable label summary safe to render on the inspector row.
    pub label_summary: String,
    /// ISO 8601 UTC timestamp the inspector state was observed.
    pub observed_at: String,
    /// True when the inspector claims to reflect saved source state. Only an
    /// exact/approximate (source-backed) row may set this true.
    pub claims_saved_source: bool,
    /// A mutation-capable action plan, when this inspector offers a write.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation: Option<MutationDescriptor>,
    /// Trigger that fired a continuity-preserving downgrade; required when a row
    /// carries a degraded continuity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<InspectorDowngradeTrigger>,
    /// Precise degraded label; required when the row carries a downgrade trigger.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
}

impl InspectorRow {
    /// Whether the recorded attach depth actually reaches this inspector lane.
    pub fn attach_depth_ok(&self) -> bool {
        self.inspector_kind.attach_depth_supports(self.attach_depth)
    }

    /// Whether the recorded freshness is consistent with the recorded continuity.
    pub fn freshness_continuity_ok(&self) -> bool {
        self.continuity.consistent_with_freshness(self.freshness)
    }

    /// Whether the target kind and freshness agree on liveness: a captured
    /// snapshot is the one and only target whose freshness is captured-snapshot.
    pub fn target_freshness_ok(&self) -> bool {
        (self.target_kind == BrowserRuntimeTargetKind::CapturedSnapshot)
            == (self.freshness == SessionFreshnessClass::CapturedSnapshot)
    }

    /// Whether the prior-session ref presence matches the continuity: a fresh
    /// attach carries none; every other continuity carries one for attributability.
    pub fn prior_session_ok(&self) -> bool {
        if self.continuity.carries_prior_session() {
            self.prior_session_ref
                .as_ref()
                .is_some_and(|r| !r.trim().is_empty())
        } else {
            self.prior_session_ref.is_none()
        }
    }

    /// Whether the redaction posture is safe for this inspector lane: a sensitive
    /// lane (console / network / storage) must hide its values by default; a
    /// non-sensitive lane may pass values through but may also redact.
    pub fn redaction_posture_ok(&self) -> bool {
        if self.inspector_kind.is_sensitive_value_surface() {
            self.redaction_posture.hides_sensitive_values()
        } else {
            true
        }
    }

    /// Whether only a source-backed row claims saved source state. Runtime-only
    /// and generated-only rows are derivative and may never claim saved source.
    pub fn saved_source_claim_ok(&self) -> bool {
        if self.claims_saved_source {
            self.mapping_quality.is_source_backed()
        } else {
            true
        }
    }

    /// Whether a runtime-only row stays honest: it never claims to be saved source
    /// state.
    pub fn runtime_masquerade_ok(&self) -> bool {
        if self.mapping_quality.is_runtime_only() {
            !self.claims_saved_source
        } else {
            true
        }
    }

    /// Whether the mutation affordance stays honest: a mutation may appear only
    /// with an explicit side-effect class matching the inspector lane, a non-empty
    /// target identity, a review/confirmation posture, and only against a live
    /// runtime target — never a captured snapshot.
    pub fn mutation_affordance_ok(&self) -> bool {
        match &self.mutation {
            None => true,
            Some(mutation) => {
                mutation
                    .side_effect_class
                    .matches_inspector(self.inspector_kind)
                    && !self.target_identity_ref.trim().is_empty()
                    && self.target_kind.is_live_runtime()
            }
        }
    }

    /// Whether this row demonstrates a continuity-preserving downgrade.
    pub fn has_downgrade(&self) -> bool {
        self.downgrade_trigger.is_some()
    }

    /// Whether the downgrade evidence is consistent: a row carrying a trigger also
    /// carries a precise non-generic degraded label, and a row with no trigger
    /// carries no degraded label; a continuity that requires a downgrade carries
    /// one.
    pub fn downgrade_consistent(&self) -> bool {
        let trigger_label_ok = if self.downgrade_trigger.is_some() {
            self.degraded_label
                .as_ref()
                .is_some_and(|label| !label_is_generic(label))
        } else {
            self.degraded_label.is_none()
        };
        let required_ok = !self.continuity.requires_downgrade() || self.downgrade_trigger.is_some();
        trigger_label_ok && required_ok
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "inspector={inspector} target={target} attach={attach} mapping={mapping} \
freshness={freshness} continuity={continuity} redaction={redaction}",
            inspector = self.inspector_kind.as_str(),
            target = self.target_kind.as_str(),
            attach = self.attach_depth.as_str(),
            mapping = self.mapping_quality.as_str(),
            freshness = self.freshness.as_str(),
            continuity = self.continuity.as_str(),
            redaction = self.redaction_posture.as_str(),
        )
    }

    /// Whether every dimension required to record this row is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.inspector_id.trim().is_empty()
            && !self.target_identity_ref.trim().is_empty()
            && !self.session_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.observed_at.trim().is_empty()
            && self.attach_depth_ok()
            && self.freshness_continuity_ok()
            && self.target_freshness_ok()
            && self.prior_session_ok()
            && self.redaction_posture_ok()
            && self.saved_source_claim_ok()
            && self.runtime_masquerade_ok()
            && self.mutation_affordance_ok()
            && self.downgrade_consistent()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block for the browser-runtime inspector packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorGuardrails {
    /// Source remains canonical; the inspector packet is derivative, never a
    /// second writable truth model.
    pub source_canonical_no_second_writable_model: bool,
    /// Runtime state never hides source-mapping uncertainty behind an inspector
    /// label.
    pub runtime_state_never_hides_source_mapping_uncertainty: bool,
    /// Inspect-only rows are never auto-upgraded into write-capable designer flows.
    pub inspect_only_never_auto_upgraded_to_write: bool,
    /// Embedded preview / browser boundaries are not blurred into product authority.
    pub embedded_boundaries_not_blurred_into_product: bool,
    /// Sensitive console / network / storage values are redacted by default.
    pub sensitive_values_redacted_by_default: bool,
    /// A mutation-capable action requires a side-effect class, target identity,
    /// and a review/confirmation posture.
    pub mutation_requires_side_effect_class_and_review: bool,
    /// Session identity stays attributable across reconnect, imported snapshot,
    /// and stale-session continuity.
    pub session_identity_attributable_across_reconnect: bool,
}

impl InspectorGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.source_canonical_no_second_writable_model
            && self.runtime_state_never_hides_source_mapping_uncertainty
            && self.inspect_only_never_auto_upgraded_to_write
            && self.embedded_boundaries_not_blurred_into_product
            && self.sensitive_values_redacted_by_default
            && self.mutation_requires_side_effect_class_and_review
            && self.session_identity_attributable_across_reconnect
    }
}

/// Consumer-projection block for the browser-runtime inspector packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorConsumerProjection {
    /// Product surfaces ingest these inspector rows instead of cloning chip text.
    pub product_ingests_inspectors: bool,
    /// Docs/help ingests the same inspector rows.
    pub docs_help_ingests_inspectors: bool,
    /// Diagnostics ingests the same inspector rows.
    pub diagnostics_ingests_inspectors: bool,
    /// Support export ingests the same inspector rows.
    pub support_export_ingests_inspectors: bool,
    /// Release-control surfaces ingest the same inspector rows.
    pub release_control_ingests_inspectors: bool,
    /// Support / diagnostics exports can reconstruct the redaction posture the user
    /// saw for each inspector.
    pub support_export_reconstructs_redaction_posture: bool,
}

impl InspectorConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_inspectors
            && self.docs_help_ingests_inspectors
            && self.diagnostics_ingests_inspectors
            && self.support_export_ingests_inspectors
            && self.release_control_ingests_inspectors
            && self.support_export_reconstructs_redaction_posture
    }
}

/// Constructor input for [`BrowserRuntimeInspectorPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserRuntimeInspectorPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Per-inspector descriptors.
    pub inspectors: Vec<InspectorRow>,
    /// Guardrail invariants block.
    pub guardrails: InspectorGuardrails,
    /// Consumer projection block.
    pub consumer_projection: InspectorConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe browser-runtime inspector packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserRuntimeInspectorPacket {
    /// Record kind; must equal [`BROWSER_RUNTIME_INSPECTORS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`BROWSER_RUNTIME_INSPECTORS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Per-inspector descriptors.
    pub inspectors: Vec<InspectorRow>,
    /// Guardrail invariants block.
    pub guardrails: InspectorGuardrails,
    /// Consumer projection block.
    pub consumer_projection: InspectorConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl BrowserRuntimeInspectorPacket {
    /// Builds a browser-runtime inspector packet.
    pub fn new(input: BrowserRuntimeInspectorPacketInput) -> Self {
        Self {
            record_kind: BROWSER_RUNTIME_INSPECTORS_RECORD_KIND.to_owned(),
            schema_version: BROWSER_RUNTIME_INSPECTORS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            set_label: input.set_label,
            inspectors: input.inspectors,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Inspector lanes represented by some row in this packet.
    pub fn represented_inspector_kinds(&self) -> BTreeSet<InspectorKind> {
        self.inspectors.iter().map(|r| r.inspector_kind).collect()
    }

    /// Target kinds represented by some row in this packet.
    pub fn represented_target_kinds(&self) -> BTreeSet<BrowserRuntimeTargetKind> {
        self.inspectors.iter().map(|r| r.target_kind).collect()
    }

    /// Mapping-quality classes represented by some row in this packet.
    pub fn represented_mapping_qualities(&self) -> BTreeSet<InspectorMappingQualityClass> {
        self.inspectors.iter().map(|r| r.mapping_quality).collect()
    }

    /// Count of rows that demonstrate a continuity-preserving downgrade.
    pub fn downgraded_row_count(&self) -> usize {
        self.inspectors.iter().filter(|r| r.has_downgrade()).count()
    }

    /// Count of rows that offer a mutation-capable action.
    pub fn mutation_row_count(&self) -> usize {
        self.inspectors
            .iter()
            .filter(|r| r.mutation.is_some())
            .count()
    }

    /// Validates the browser-runtime inspector packet invariants.
    pub fn validate(&self) -> Vec<BrowserRuntimeInspectorViolation> {
        let mut violations = Vec::new();

        if self.record_kind != BROWSER_RUNTIME_INSPECTORS_RECORD_KIND {
            violations.push(BrowserRuntimeInspectorViolation::WrongRecordKind);
        }
        if self.schema_version != BROWSER_RUNTIME_INSPECTORS_SCHEMA_VERSION {
            violations.push(BrowserRuntimeInspectorViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.set_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(BrowserRuntimeInspectorViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("browser-runtime inspector packet serializes"),
        ) {
            violations.push(BrowserRuntimeInspectorViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("browser-runtime inspector packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Browser-Runtime Inspectors\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.set_label));
        out.push_str(&format!(
            "- Inspectors: {} ({} mutation-capable, {} downgraded)\n",
            self.inspectors.len(),
            self.mutation_row_count(),
            self.downgraded_row_count()
        ));
        out.push_str(&format!(
            "- Inspector kinds: {} / {}\n",
            self.represented_inspector_kinds().len(),
            InspectorKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Target kinds: {} / {}\n",
            self.represented_target_kinds().len(),
            BrowserRuntimeTargetKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Mapping qualities: {} / {}\n",
            self.represented_mapping_qualities().len(),
            InspectorMappingQualityClass::ALL.len()
        ));
        out.push_str("\n## Inspectors\n\n");
        for row in &self.inspectors {
            out.push_str(&format!(
                "- **{}** ({})\n",
                row.inspector_id,
                row.inspector_kind.as_str()
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!("  - {}\n", row.chip_tokens()));
            if let Some(mutation) = &row.mutation {
                out.push_str(&format!(
                    "  - Mutation: side_effect={} review={} target=`{}`\n",
                    mutation.side_effect_class.as_str(),
                    mutation.review_posture.as_str(),
                    row.target_identity_ref,
                ));
            }
            if let Some(label) = &row.degraded_label {
                out.push_str(&format!("  - Downgraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in browser-runtime inspector export.
#[derive(Debug)]
pub enum BrowserRuntimeInspectorArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<BrowserRuntimeInspectorViolation>),
}

impl fmt::Display for BrowserRuntimeInspectorArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "browser-runtime inspector export parse failed: {error}"
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
                    "browser-runtime inspector export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for BrowserRuntimeInspectorArtifactError {}

/// Validation failures emitted by [`BrowserRuntimeInspectorPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrowserRuntimeInspectorViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required inspector lane is represented by no row.
    RequiredInspectorKindMissing,
    /// A required target kind is represented by no row.
    RequiredTargetKindMissing,
    /// A required mapping-quality class is represented by no row.
    RequiredMappingQualityMissing,
    /// The packet demonstrates no mutation-capable inspector.
    MutationCaseMissing,
    /// The packet demonstrates no continuity-preserving downgrade row.
    DowngradedRowCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A row's attach depth does not reach its inspector lane.
    AttachDepthInsufficient,
    /// A row's freshness disagrees with its continuity.
    FreshnessContinuityMismatch,
    /// A row's target kind and freshness disagree on liveness.
    TargetFreshnessMismatch,
    /// A row's prior-session ref presence is inconsistent with its continuity.
    PriorSessionInconsistent,
    /// A sensitive inspector lane exposes values without a redaction-safe posture.
    SensitiveValuesUnredacted,
    /// A non-source-backed row claims saved source state.
    NonSourceBackedClaimsSavedSource,
    /// A runtime-only row masquerades as saved source state.
    RuntimeOnlyMasqueradesAsSource,
    /// A row offers a mutation it cannot honestly back.
    MutationAffordanceUnbacked,
    /// A row carries a downgrade trigger without a precise label, or vice versa.
    DowngradeInconsistent,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl BrowserRuntimeInspectorViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredInspectorKindMissing => "required_inspector_kind_missing",
            Self::RequiredTargetKindMissing => "required_target_kind_missing",
            Self::RequiredMappingQualityMissing => "required_mapping_quality_missing",
            Self::MutationCaseMissing => "mutation_case_missing",
            Self::DowngradedRowCaseMissing => "downgraded_row_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::AttachDepthInsufficient => "attach_depth_insufficient",
            Self::FreshnessContinuityMismatch => "freshness_continuity_mismatch",
            Self::TargetFreshnessMismatch => "target_freshness_mismatch",
            Self::PriorSessionInconsistent => "prior_session_inconsistent",
            Self::SensitiveValuesUnredacted => "sensitive_values_unredacted",
            Self::NonSourceBackedClaimsSavedSource => "non_source_backed_claims_saved_source",
            Self::RuntimeOnlyMasqueradesAsSource => "runtime_only_masquerades_as_source",
            Self::MutationAffordanceUnbacked => "mutation_affordance_unbacked",
            Self::DowngradeInconsistent => "downgrade_inconsistent",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in browser-runtime inspector export.
pub fn current_m5_browser_runtime_inspectors_export(
) -> Result<BrowserRuntimeInspectorPacket, BrowserRuntimeInspectorArtifactError> {
    let packet: BrowserRuntimeInspectorPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/preview/m5/browser_runtime_inspectors/support_export.json"
    )))
    .map_err(BrowserRuntimeInspectorArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(BrowserRuntimeInspectorArtifactError::Validation(violations))
    }
}

/// Numeric rank of an attach depth on the DOM → styles → network → storage
/// ladder; deeper attaches rank higher.
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
    packet: &BrowserRuntimeInspectorPacket,
    violations: &mut Vec<BrowserRuntimeInspectorViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        BROWSER_RUNTIME_INSPECTORS_SCHEMA_REF,
        BROWSER_RUNTIME_INSPECTORS_DOC_REF,
        BROWSER_RUNTIME_INSPECTORS_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(BrowserRuntimeInspectorViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &BrowserRuntimeInspectorPacket,
    violations: &mut Vec<BrowserRuntimeInspectorViolation>,
) {
    let inspector_kinds = packet.represented_inspector_kinds();
    for required in InspectorKind::ALL {
        if !inspector_kinds.contains(&required) {
            violations.push(BrowserRuntimeInspectorViolation::RequiredInspectorKindMissing);
            break;
        }
    }

    let target_kinds = packet.represented_target_kinds();
    for required in BrowserRuntimeTargetKind::ALL {
        if !target_kinds.contains(&required) {
            violations.push(BrowserRuntimeInspectorViolation::RequiredTargetKindMissing);
            break;
        }
    }

    let qualities = packet.represented_mapping_qualities();
    for required in InspectorMappingQualityClass::ALL {
        if !qualities.contains(&required) {
            violations.push(BrowserRuntimeInspectorViolation::RequiredMappingQualityMissing);
            break;
        }
    }

    if !packet
        .inspectors
        .iter()
        .any(|r| r.mutation.is_some() && r.mutation_affordance_ok() && r.is_complete())
    {
        violations.push(BrowserRuntimeInspectorViolation::MutationCaseMissing);
    }

    if !packet
        .inspectors
        .iter()
        .any(|r| r.has_downgrade() && r.downgrade_consistent())
    {
        violations.push(BrowserRuntimeInspectorViolation::DowngradedRowCaseMissing);
    }
}

fn validate_rows(
    packet: &BrowserRuntimeInspectorPacket,
    violations: &mut Vec<BrowserRuntimeInspectorViolation>,
) {
    for row in &packet.inspectors {
        if !row.is_complete() {
            violations.push(BrowserRuntimeInspectorViolation::RowIncomplete);
        }
        if !row.attach_depth_ok() {
            violations.push(BrowserRuntimeInspectorViolation::AttachDepthInsufficient);
        }
        if !row.freshness_continuity_ok() {
            violations.push(BrowserRuntimeInspectorViolation::FreshnessContinuityMismatch);
        }
        if !row.target_freshness_ok() {
            violations.push(BrowserRuntimeInspectorViolation::TargetFreshnessMismatch);
        }
        if !row.prior_session_ok() {
            violations.push(BrowserRuntimeInspectorViolation::PriorSessionInconsistent);
        }
        if !row.redaction_posture_ok() {
            violations.push(BrowserRuntimeInspectorViolation::SensitiveValuesUnredacted);
        }
        if !row.saved_source_claim_ok() {
            violations.push(BrowserRuntimeInspectorViolation::NonSourceBackedClaimsSavedSource);
        }
        if !row.runtime_masquerade_ok() {
            violations.push(BrowserRuntimeInspectorViolation::RuntimeOnlyMasqueradesAsSource);
        }
        if !row.mutation_affordance_ok() {
            violations.push(BrowserRuntimeInspectorViolation::MutationAffordanceUnbacked);
        }
        if !row.downgrade_consistent() {
            violations.push(BrowserRuntimeInspectorViolation::DowngradeInconsistent);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(BrowserRuntimeInspectorViolation::RowEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &BrowserRuntimeInspectorPacket,
    violations: &mut Vec<BrowserRuntimeInspectorViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(BrowserRuntimeInspectorViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &BrowserRuntimeInspectorPacket,
    violations: &mut Vec<BrowserRuntimeInspectorViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(BrowserRuntimeInspectorViolation::ConsumerProjectionIncomplete);
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
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
            | "no session"
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
