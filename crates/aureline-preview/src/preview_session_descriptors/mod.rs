//! Preview-session descriptors and source-sync chips for the first real M5
//! framework-pack, preview-route, and notebook-adjacent surfaces.
//!
//! Where [`crate::freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix`]
//! freezes the *qualification matrix* over claimed preview/runtime surfaces, and
//! [`crate::preview_origin`] materializes the per-view origin / target / mapping
//! truth objects, this module materializes the **per-session descriptor** every
//! claimed preview surface presents to the user: a single shared packet that
//! carries source revision, runtime identity, device/viewport target, data
//! posture (live / mock / captured), freshness, and source-sync state.
//!
//! The descriptor is the one canonical answer to "for the session the user is
//! looking at right now, which source revision is canonical, which runtime
//! produced the view, which device/viewport target is on screen, is the data
//! live / mock / captured, how fresh is the view, and is it in sync with the
//! canonical source?" A [`PreviewSessionDescriptorSet`] binds the first real
//! framework-pack, preview-route, and notebook-adjacent consumers onto the same
//! governed chips — [`PreviewSessionClass`], [`SourceSyncClass`],
//! [`PreviewDataPostureClass`], and [`PreviewFreshnessClass`] — instead of
//! provider-specific extension chrome.
//!
//! Each [`PreviewSessionDescriptor`] reuses the frozen
//! [`PreviewSessionClass`] and [`SourceSyncClass`] vocabularies and the
//! per-view [`PreviewOriginClass`], [`PreviewTargetClass`], and
//! [`DeviceCapabilityClass`] vocabularies rather than minting synonyms, and adds
//! the session-level dimensions this lane owns: [`PreviewConsumerSurface`],
//! [`PreviewDataPostureClass`], [`PreviewFreshnessClass`], and
//! [`SessionDowngradeTrigger`].
//!
//! The descriptor keeps source canonical. Switching between live, mock,
//! captured, or stale preview states changes governed chips and exports rather
//! than bespoke copy or silent icon changes; a runtime-only view never
//! masquerades as saved source state; a captured snapshot never claims a live
//! runtime; and a stale or downgraded session always exports a precise,
//! non-generic degraded label and a recorded trigger so downstream diagnostics,
//! support, and release packets can reuse the stale/downgraded truth directly.
//!
//! Raw URLs, hostnames, cookies, raw provider payloads, credentials, and raw
//! runtime handles never cross this boundary; the packet carries only typed
//! class tokens, opaque revision / evidence refs, booleans, and redacted
//! labels.
//!
//! The boundary schema is
//! [`schemas/preview/preview_session_descriptor_set.schema.json`](../../../../schemas/preview/preview_session_descriptor_set.schema.json).
//! The contract doc is
//! [`docs/preview/m5/preview_session_descriptors.md`](../../../../docs/preview/m5/preview_session_descriptors.md).
//! The protected fixture directory is
//! [`fixtures/preview/m5/preview_session_descriptors/`](../../../../fixtures/preview/m5/preview_session_descriptors/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix::{
    PreviewSessionClass, SourceSyncClass,
};
use crate::preview_origin::{DeviceCapabilityClass, PreviewOriginClass, PreviewTargetClass};

/// Stable record-kind tag carried by [`PreviewSessionDescriptorSet`].
pub const PREVIEW_SESSION_DESCRIPTOR_SET_RECORD_KIND: &str = "preview_session_descriptor_set";

/// Schema version for the preview-session descriptor set.
pub const PREVIEW_SESSION_DESCRIPTOR_SET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PREVIEW_SESSION_DESCRIPTOR_SET_SCHEMA_REF: &str =
    "schemas/preview/preview_session_descriptor_set.schema.json";

/// Repo-relative path of the contract doc.
pub const PREVIEW_SESSION_DESCRIPTOR_SET_DOC_REF: &str =
    "docs/preview/m5/preview_session_descriptors.md";

/// Repo-relative path of the protected fixture directory.
pub const PREVIEW_SESSION_DESCRIPTOR_SET_FIXTURE_DIR: &str =
    "fixtures/preview/m5/preview_session_descriptors";

/// Repo-relative path of the checked support-export artifact.
pub const PREVIEW_SESSION_DESCRIPTOR_SET_ARTIFACT_REF: &str =
    "artifacts/preview/m5/preview_session_descriptors/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const PREVIEW_SESSION_DESCRIPTOR_SET_SUMMARY_REF: &str =
    "artifacts/preview/m5/preview_session_descriptors.md";

/// One claimed first-real consumer surface a session descriptor normalizes onto
/// the shared session chips. Distinct from the matrix-level `PreviewSurface`:
/// this names the concrete product consumer that presents the session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewConsumerSurface {
    /// Framework-pack preview (e.g. a framework dev-server preview pane).
    FrameworkPackPreview,
    /// Preview-route surface (a governed preview route projected into the shell).
    PreviewRoute,
    /// Notebook-adjacent preview (a notebook output / rendered-cell preview).
    NotebookAdjacentPreview,
    /// Support / export projection of a session descriptor.
    SupportExportProjection,
}

impl PreviewConsumerSurface {
    /// The first-real consumer surfaces this lane must normalize, in
    /// declaration order. The support/export projection is reused downstream and
    /// is not a "first real" consumer, so it is excluded here.
    pub const FIRST_REAL: [Self; 3] = [
        Self::FrameworkPackPreview,
        Self::PreviewRoute,
        Self::NotebookAdjacentPreview,
    ];

    /// Stable token recorded in the descriptor.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkPackPreview => "framework_pack_preview",
            Self::PreviewRoute => "preview_route",
            Self::NotebookAdjacentPreview => "notebook_adjacent_preview",
            Self::SupportExportProjection => "support_export_projection",
        }
    }
}

/// Closed data-posture vocabulary. Names whether the data behind the preview is
/// live, mocked, or captured — the governed chip that must change (not bespoke
/// copy) when the user switches preview data modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewDataPostureClass {
    /// Live data from a running runtime.
    Live,
    /// Mock / fixture data standing in for live data.
    Mock,
    /// Captured / recorded data replayed into the view.
    Captured,
    /// The data posture could not be identified; forces a downgrade.
    Unidentified,
}

impl PreviewDataPostureClass {
    /// Stable token recorded in the descriptor.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Mock => "mock",
            Self::Captured => "captured",
            Self::Unidentified => "unidentified",
        }
    }

    /// True when the posture asserts a live, runtime-backed data feed.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::Live)
    }

    /// True when the posture is a static capture with no live runtime feed.
    pub const fn is_captured(self) -> bool {
        matches!(self, Self::Captured)
    }

    /// True when the posture could not be identified.
    pub const fn is_unidentified(self) -> bool {
        matches!(self, Self::Unidentified)
    }
}

/// Closed freshness vocabulary. Names how fresh the rendered view is relative to
/// its freshness SLO so a stale view can never advertise current state by
/// silence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewFreshnessClass {
    /// Within the freshness SLO.
    Fresh,
    /// Past half the SLO but not yet stale.
    Aging,
    /// Past the freshness SLO; forces a downgrade.
    Stale,
    /// Freshness could not be determined; forces a downgrade.
    Unknown,
}

impl PreviewFreshnessClass {
    /// Stable token recorded in the descriptor.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Aging => "aging",
            Self::Stale => "stale",
            Self::Unknown => "unknown",
        }
    }

    /// True when the freshness state forces a downgrade (stale or unknown).
    pub const fn forces_downgrade(self) -> bool {
        matches!(self, Self::Stale | Self::Unknown)
    }
}

/// Closed downgrade-trigger vocabulary. Names why a session was downgraded below
/// a current, in-sync, fresh, identified posture; the chrome quotes the trigger
/// verbatim instead of a generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionDowngradeTrigger {
    /// The view is past its freshness SLO.
    StaleFreshness,
    /// Freshness could not be determined.
    UnknownFreshness,
    /// The preview drifted from the canonical source.
    DriftedFromSource,
    /// The source-sync state could not be identified.
    UnidentifiedSourceSync,
    /// The data posture could not be identified.
    UnidentifiedDataPosture,
    /// Policy narrowed the session below its posture.
    PolicyNarrowed,
    /// An upstream dependency narrowed and dragged this session down with it.
    UpstreamDependencyNarrowed,
}

impl SessionDowngradeTrigger {
    /// Stable token recorded in the descriptor.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaleFreshness => "stale_freshness",
            Self::UnknownFreshness => "unknown_freshness",
            Self::DriftedFromSource => "drifted_from_source",
            Self::UnidentifiedSourceSync => "unidentified_source_sync",
            Self::UnidentifiedDataPosture => "unidentified_data_posture",
            Self::PolicyNarrowed => "policy_narrowed",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// One preview-session descriptor: the shared truth packet a single claimed
/// preview surface presents for the session the user is looking at right now.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewSessionDescriptor {
    /// Stable session id.
    pub session_id: String,
    /// First-real consumer surface presenting the session.
    pub consumer_surface: PreviewConsumerSurface,
    /// Human-readable label summary safe to render on the chip strip.
    pub label_summary: String,
    /// ISO 8601 UTC timestamp the descriptor was observed.
    pub observed_at: String,

    /// Matrix-level preview-session class (reused frozen vocabulary).
    pub preview_session_class: PreviewSessionClass,
    /// Source-sync state of the derivative preview against canonical source
    /// (reused frozen vocabulary).
    pub source_sync_class: SourceSyncClass,
    /// Data posture chip: live / mock / captured / unidentified.
    pub data_posture: PreviewDataPostureClass,
    /// Freshness chip.
    pub freshness_class: PreviewFreshnessClass,
    /// Runtime identity (reused per-view origin vocabulary).
    pub runtime_origin_class: PreviewOriginClass,
    /// Device / viewport target kind (reused per-view target vocabulary).
    pub target_kind: PreviewTargetClass,
    /// Device capability profile of the target (reused per-view vocabulary).
    pub device_capability_class: DeviceCapabilityClass,

    /// Opaque ref to the canonical source revision this view derives from.
    /// Required for source-relative sync states; absent for a runtime-only view.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_revision_ref: Option<String>,
    /// Optional viewport width for layout-slice targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewport_pixel_width: Option<u32>,
    /// Optional viewport height for layout-slice targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewport_pixel_height: Option<u32>,

    /// Freshness SLO in seconds; a non-fresh view past this is stale.
    pub freshness_slo_seconds: u32,

    /// True when a live runtime backs the view.
    pub runtime_backed: bool,
    /// True when the session claims the view is saved source state. A
    /// runtime-only view must never set this true.
    pub claims_saved_source: bool,
    /// True when a visual edit against this session writes back to source.
    pub write_capable: bool,
    /// True when a write-capable session previews the real source diff before
    /// commit.
    pub previews_source_diff_before_commit: bool,

    /// Trigger that fired the downgrade; required when the session is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<SessionDowngradeTrigger>,
    /// Precise degraded label; required when the session is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,

    /// Evidence packet refs backing this session.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this session.
    pub source_contract_refs: Vec<String>,
}

impl PreviewSessionDescriptor {
    /// Whether the source-sync state is source-relative (it points at a saved
    /// canonical revision rather than being a runtime-only view).
    pub fn is_source_relative(&self) -> bool {
        matches!(
            self.source_sync_class,
            SourceSyncClass::InSyncFromSource
                | SourceSyncClass::PendingRebuild
                | SourceSyncClass::DriftedFromSource
        )
    }

    /// Whether the session must be downgraded below a current/in-sync/fresh
    /// posture because of stale or unidentified state.
    pub fn needs_downgrade(&self) -> bool {
        self.freshness_class.forces_downgrade()
            || self.source_sync_class.is_unidentified()
            || matches!(self.source_sync_class, SourceSyncClass::DriftedFromSource)
            || self.data_posture.is_unidentified()
    }

    /// Whether the downgrade evidence is consistent: a downgraded session
    /// carries both a recorded trigger and a precise, non-generic degraded
    /// label, and a non-downgraded session carries neither.
    pub fn downgrade_consistent(&self) -> bool {
        if self.needs_downgrade() {
            self.downgrade_trigger.is_some()
                && self
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.downgrade_trigger.is_none() && self.degraded_label.is_none()
        }
    }

    /// Whether a runtime-only view is honestly labeled rather than passed off as
    /// saved source state. A runtime-only view is runtime-backed, never claims to
    /// be saved source state, and carries no canonical source revision.
    pub fn runtime_masquerade_ok(&self) -> bool {
        if self.source_sync_class.is_runtime_only() {
            self.runtime_backed && !self.claims_saved_source && self.source_revision_ref.is_none()
        } else {
            true
        }
    }

    /// Whether a live data posture is backed by a live runtime that can actually
    /// produce a live feed.
    pub fn live_posture_ok(&self) -> bool {
        if self.data_posture.is_live() {
            self.runtime_backed && self.runtime_origin_class.admits_live_runtime_events()
        } else {
            true
        }
    }

    /// Whether a captured posture stays honest: a captured snapshot has no live
    /// runtime feed and cannot be written back to source.
    pub fn captured_posture_ok(&self) -> bool {
        if self.data_posture.is_captured() {
            !self.runtime_backed && !self.write_capable
        } else {
            true
        }
    }

    /// Whether the canonical source revision presence matches the source-sync
    /// state: source-relative states reference a revision; a runtime-only view
    /// carries none.
    pub fn source_revision_presence_ok(&self) -> bool {
        if self.is_source_relative() {
            self.source_revision_ref.is_some()
        } else if self.source_sync_class.is_runtime_only() {
            self.source_revision_ref.is_none()
        } else {
            true
        }
    }

    /// Whether a write-capable session is backed by a writable canonical source
    /// and previews the real source diff before commit. Captured and
    /// runtime-only sessions are never write-capable (no source to write back).
    pub fn write_capability_ok(&self) -> bool {
        if self.write_capable {
            self.source_revision_ref.is_some()
                && !self.data_posture.is_captured()
                && !self.source_sync_class.is_runtime_only()
                && self.previews_source_diff_before_commit
        } else {
            true
        }
    }

    /// Whether the viewport dimensions are paired (both present or both absent).
    pub fn viewport_pairing_ok(&self) -> bool {
        self.viewport_pixel_width.is_some() == self.viewport_pixel_height.is_some()
    }

    /// Deterministic governed chip line for this session: the closed-vocabulary
    /// chips downstream consumers render verbatim instead of bespoke copy.
    pub fn chip_tokens(&self) -> String {
        format!(
            "session={session} source_sync={sync} data={data} freshness={fresh} origin={origin} target={target}",
            session = self.preview_session_class.as_str(),
            sync = self.source_sync_class.as_str(),
            data = self.data_posture.as_str(),
            fresh = self.freshness_class.as_str(),
            origin = self.runtime_origin_class.as_str(),
            target = self.target_kind.as_str(),
        )
    }

    /// Whether every dimension required to record this session is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.session_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.observed_at.trim().is_empty()
            && self.freshness_slo_seconds > 0
            && self.downgrade_consistent()
            && self.runtime_masquerade_ok()
            && self.live_posture_ok()
            && self.captured_posture_ok()
            && self.source_revision_presence_ok()
            && self.write_capability_ok()
            && self.viewport_pairing_ok()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.source_contract_refs.is_empty()
            && self
                .source_contract_refs
                .iter()
                .all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block for the session descriptor set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionGuardrails {
    /// Source remains canonical; the session descriptor is derivative, never a
    /// second writable truth model.
    pub source_canonical_no_second_writable_model: bool,
    /// Runtime state never hides source-mapping uncertainty behind a live chip.
    pub runtime_state_never_hides_source_mapping_uncertainty: bool,
    /// Inspect-only sessions are never auto-upgraded into write-capable flows.
    pub inspect_only_never_auto_upgraded_to_write: bool,
    /// Embedded preview/browser boundaries are not blurred into product authority.
    pub embedded_boundaries_not_blurred_into_product: bool,
    /// Switching posture changes governed chips and exports, not bespoke copy.
    pub posture_switch_changes_governed_chips_not_bespoke_copy: bool,
    /// Stale or downgraded sessions export precise truth downstream.
    pub stale_or_downgraded_sessions_export_truth: bool,
}

impl SessionGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.source_canonical_no_second_writable_model
            && self.runtime_state_never_hides_source_mapping_uncertainty
            && self.inspect_only_never_auto_upgraded_to_write
            && self.embedded_boundaries_not_blurred_into_product
            && self.posture_switch_changes_governed_chips_not_bespoke_copy
            && self.stale_or_downgraded_sessions_export_truth
    }
}

/// Consumer-projection block for the session descriptor set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionConsumerProjection {
    /// Product surfaces ingest these descriptors instead of cloning chip text.
    pub product_ingests_sessions: bool,
    /// Docs/help ingests the same descriptors.
    pub docs_help_ingests_sessions: bool,
    /// Diagnostics ingests the same descriptors.
    pub diagnostics_ingests_sessions: bool,
    /// Support export ingests the same descriptors.
    pub support_export_ingests_sessions: bool,
    /// Release-control surfaces ingest the same descriptors.
    pub release_control_ingests_sessions: bool,
    /// Downgraded sessions are visibly labeled below current in every surface.
    pub downgraded_sessions_labeled_below_current: bool,
}

impl SessionConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_sessions
            && self.docs_help_ingests_sessions
            && self.diagnostics_ingests_sessions
            && self.support_export_ingests_sessions
            && self.release_control_ingests_sessions
            && self.downgraded_sessions_labeled_below_current
    }
}

/// Constructor input for [`PreviewSessionDescriptorSet::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewSessionDescriptorSetInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Per-session descriptors.
    pub sessions: Vec<PreviewSessionDescriptor>,
    /// Guardrail invariants block.
    pub guardrails: SessionGuardrails,
    /// Consumer projection block.
    pub consumer_projection: SessionConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe preview-session descriptor set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewSessionDescriptorSet {
    /// Record kind; must equal [`PREVIEW_SESSION_DESCRIPTOR_SET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PREVIEW_SESSION_DESCRIPTOR_SET_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Per-session descriptors.
    pub sessions: Vec<PreviewSessionDescriptor>,
    /// Guardrail invariants block.
    pub guardrails: SessionGuardrails,
    /// Consumer projection block.
    pub consumer_projection: SessionConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PreviewSessionDescriptorSet {
    /// Builds a preview-session descriptor set packet.
    pub fn new(input: PreviewSessionDescriptorSetInput) -> Self {
        Self {
            record_kind: PREVIEW_SESSION_DESCRIPTOR_SET_RECORD_KIND.to_owned(),
            schema_version: PREVIEW_SESSION_DESCRIPTOR_SET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            set_label: input.set_label,
            sessions: input.sessions,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Consumer surfaces represented by some session in this set.
    pub fn represented_surfaces(&self) -> BTreeSet<PreviewConsumerSurface> {
        self.sessions.iter().map(|s| s.consumer_surface).collect()
    }

    /// Data postures represented by some session in this set.
    pub fn represented_postures(&self) -> BTreeSet<PreviewDataPostureClass> {
        self.sessions.iter().map(|s| s.data_posture).collect()
    }

    /// Count of sessions downgraded below a current posture.
    pub fn downgraded_session_count(&self) -> usize {
        self.sessions.iter().filter(|s| s.needs_downgrade()).count()
    }

    /// Whether the set demonstrates a posture switch: it carries a live posture
    /// plus at least one of mock or captured.
    pub fn demonstrates_posture_switch(&self) -> bool {
        let postures = self.represented_postures();
        postures.contains(&PreviewDataPostureClass::Live)
            && (postures.contains(&PreviewDataPostureClass::Mock)
                || postures.contains(&PreviewDataPostureClass::Captured))
    }

    /// Validates the session descriptor set invariants.
    pub fn validate(&self) -> Vec<PreviewSessionDescriptorSetViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PREVIEW_SESSION_DESCRIPTOR_SET_RECORD_KIND {
            violations.push(PreviewSessionDescriptorSetViolation::WrongRecordKind);
        }
        if self.schema_version != PREVIEW_SESSION_DESCRIPTOR_SET_SCHEMA_VERSION {
            violations.push(PreviewSessionDescriptorSetViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.set_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PreviewSessionDescriptorSetViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_sessions(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("preview session descriptor set serializes"),
        ) {
            violations.push(PreviewSessionDescriptorSetViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("preview session descriptor set serializes")
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Preview-Session Descriptors\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.set_label));
        out.push_str(&format!(
            "- Sessions: {} ({} downgraded)\n",
            self.sessions.len(),
            self.downgraded_session_count()
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {}\n",
            self.represented_surfaces().len(),
            PreviewConsumerSurface::FIRST_REAL.len() + 1
        ));
        out.push_str("\n## Sessions\n\n");
        for session in &self.sessions {
            out.push_str(&format!(
                "- **{}** ({})\n",
                session.session_id,
                session.consumer_surface.as_str()
            ));
            out.push_str(&format!("  - {}\n", session.label_summary));
            out.push_str(&format!("  - {}\n", session.chip_tokens()));
            out.push_str(&format!(
                "  - source_revision=`{}` runtime_backed={} write_capable={}\n",
                session.source_revision_ref.as_deref().unwrap_or("none"),
                session.runtime_backed,
                session.write_capable,
            ));
            if let Some(label) = &session.degraded_label {
                out.push_str(&format!("  - Downgraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in session descriptor set export.
#[derive(Debug)]
pub enum PreviewSessionDescriptorSetArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PreviewSessionDescriptorSetViolation>),
}

impl fmt::Display for PreviewSessionDescriptorSetArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "preview session descriptor set export parse failed: {error}"
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
                    "preview session descriptor set export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PreviewSessionDescriptorSetArtifactError {}

/// Validation failures emitted by [`PreviewSessionDescriptorSet::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreviewSessionDescriptorSetViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required first-real consumer surface is represented by no session.
    RequiredConsumerSurfaceMissing,
    /// The set demonstrates no live-vs-mock-or-captured posture switch.
    PostureSwitchCaseMissing,
    /// The set demonstrates no downgraded / stale session.
    DowngradedSessionCaseMissing,
    /// A session is incomplete.
    SessionIncomplete,
    /// A downgraded session lacks a precise degraded label or trigger.
    DowngradedSessionMissingLabelOrTrigger,
    /// A non-downgraded session carries a downgrade trigger or label.
    NonDowngradedSessionCarriesDowngrade,
    /// A runtime-only view masquerades as saved source state.
    RuntimeOnlyMasqueradesAsSource,
    /// A live data posture is not backed by a live runtime.
    LiveDataPostureWithoutLiveRuntime,
    /// A captured posture claims a live runtime or write capability.
    CapturedPostureClaimsRuntimeOrWrite,
    /// The canonical source revision presence is inconsistent with the sync state.
    SourceRevisionPresenceInconsistent,
    /// A write-capable session is unbacked or skips the source-diff preview.
    WriteCapableSessionUnbackedOrSkipsDiffPreview,
    /// Viewport dimensions are not paired.
    ViewportDimensionMismatch,
    /// A session lacks evidence refs.
    SessionEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl PreviewSessionDescriptorSetViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredConsumerSurfaceMissing => "required_consumer_surface_missing",
            Self::PostureSwitchCaseMissing => "posture_switch_case_missing",
            Self::DowngradedSessionCaseMissing => "downgraded_session_case_missing",
            Self::SessionIncomplete => "session_incomplete",
            Self::DowngradedSessionMissingLabelOrTrigger => {
                "downgraded_session_missing_label_or_trigger"
            }
            Self::NonDowngradedSessionCarriesDowngrade => {
                "non_downgraded_session_carries_downgrade"
            }
            Self::RuntimeOnlyMasqueradesAsSource => "runtime_only_masquerades_as_source",
            Self::LiveDataPostureWithoutLiveRuntime => "live_data_posture_without_live_runtime",
            Self::CapturedPostureClaimsRuntimeOrWrite => "captured_posture_claims_runtime_or_write",
            Self::SourceRevisionPresenceInconsistent => "source_revision_presence_inconsistent",
            Self::WriteCapableSessionUnbackedOrSkipsDiffPreview => {
                "write_capable_session_unbacked_or_skips_diff_preview"
            }
            Self::ViewportDimensionMismatch => "viewport_dimension_mismatch",
            Self::SessionEvidenceMissing => "session_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in session descriptor set export.
pub fn current_m5_preview_session_descriptor_set_export(
) -> Result<PreviewSessionDescriptorSet, PreviewSessionDescriptorSetArtifactError> {
    let packet: PreviewSessionDescriptorSet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/preview/m5/preview_session_descriptors/support_export.json"
    )))
    .map_err(PreviewSessionDescriptorSetArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PreviewSessionDescriptorSetArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &PreviewSessionDescriptorSet,
    violations: &mut Vec<PreviewSessionDescriptorSetViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PREVIEW_SESSION_DESCRIPTOR_SET_SCHEMA_REF,
        PREVIEW_SESSION_DESCRIPTOR_SET_DOC_REF,
        PREVIEW_SESSION_DESCRIPTOR_SET_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(PreviewSessionDescriptorSetViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &PreviewSessionDescriptorSet,
    violations: &mut Vec<PreviewSessionDescriptorSetViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in PreviewConsumerSurface::FIRST_REAL {
        if !surfaces.contains(&required) {
            violations.push(PreviewSessionDescriptorSetViolation::RequiredConsumerSurfaceMissing);
            break;
        }
    }

    if !packet.demonstrates_posture_switch() {
        violations.push(PreviewSessionDescriptorSetViolation::PostureSwitchCaseMissing);
    }

    if !packet
        .sessions
        .iter()
        .any(|s| s.needs_downgrade() && s.downgrade_consistent())
    {
        violations.push(PreviewSessionDescriptorSetViolation::DowngradedSessionCaseMissing);
    }
}

fn validate_sessions(
    packet: &PreviewSessionDescriptorSet,
    violations: &mut Vec<PreviewSessionDescriptorSetViolation>,
) {
    for session in &packet.sessions {
        if !session.is_complete() {
            violations.push(PreviewSessionDescriptorSetViolation::SessionIncomplete);
        }
        if session.needs_downgrade()
            && (session.downgrade_trigger.is_none()
                || !session
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations
                .push(PreviewSessionDescriptorSetViolation::DowngradedSessionMissingLabelOrTrigger);
        }
        if !session.needs_downgrade()
            && (session.downgrade_trigger.is_some() || session.degraded_label.is_some())
        {
            violations
                .push(PreviewSessionDescriptorSetViolation::NonDowngradedSessionCarriesDowngrade);
        }
        if !session.runtime_masquerade_ok() {
            violations.push(PreviewSessionDescriptorSetViolation::RuntimeOnlyMasqueradesAsSource);
        }
        if !session.live_posture_ok() {
            violations
                .push(PreviewSessionDescriptorSetViolation::LiveDataPostureWithoutLiveRuntime);
        }
        if !session.captured_posture_ok() {
            violations
                .push(PreviewSessionDescriptorSetViolation::CapturedPostureClaimsRuntimeOrWrite);
        }
        if !session.source_revision_presence_ok() {
            violations
                .push(PreviewSessionDescriptorSetViolation::SourceRevisionPresenceInconsistent);
        }
        if !session.write_capability_ok() {
            violations.push(
                PreviewSessionDescriptorSetViolation::WriteCapableSessionUnbackedOrSkipsDiffPreview,
            );
        }
        if !session.viewport_pairing_ok() {
            violations.push(PreviewSessionDescriptorSetViolation::ViewportDimensionMismatch);
        }
        if session.evidence_refs.is_empty()
            || session.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(PreviewSessionDescriptorSetViolation::SessionEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &PreviewSessionDescriptorSet,
    violations: &mut Vec<PreviewSessionDescriptorSetViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(PreviewSessionDescriptorSetViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &PreviewSessionDescriptorSet,
    violations: &mut Vec<PreviewSessionDescriptorSetViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(PreviewSessionDescriptorSetViolation::ConsumerProjectionIncomplete);
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
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
