//! Support-bundle manifest seed: bounded prototype not yet wired into the running shell.
//!
//! This module projects [`aureline_support::bundle`] into the shell crate so
//! the protected walk — open support export, preview bundle contents and
//! redactions locally, verify exact-build identity is captured before
//! export — has a durable Rust surface while native-shell wiring is still
//! pending. The
//! reviewer-facing landing page lives at
//! `/docs/support/support_bundle_seed.md`; this is the in-shell
//! projection that drives it.
//!
//! ## What this consumer owns
//!
//! - The [`SupportSeedSurface`] record: a thin projection over a built
//!   [`aureline_support::bundle::SupportBundlePreview`] that adds shell-
//!   facing copy (banner heading, honesty marker, action set) without
//!   forking the manifest.
//! - The seed default preview minted from
//!   [`SupportSeedSurface::default_local_preview`]: the smallest
//!   trustworthy bundle (exact-build identity row + policy/trust
//!   metadata row) that proves the protected walk on every dogfood
//!   build.
//! - The failure-drill preview minted from
//!   [`SupportSeedSurface::failure_drill_preview`]: queues a synthetic
//!   secret-bearing row so a reviewer can confirm the local-first
//!   defaults rewrite it to `prohibited` before export.
//!
//! ## What this consumer does NOT own
//!
//! - The manifest schema, the redaction defaults, or the exact-build
//!   capture. Those live in [`aureline_support::bundle`] and the
//!   boundary schemas under `/schemas/support/`.
//! - Live byte-level redaction or upload transport.
//! - The Help/About truth model — that surface owns its own truth in
//!   [`crate::help_about`]. The support seed simply quotes the same
//!   exact-build identity as a row inside the preview.

use std::collections::BTreeSet;

use aureline_commands::invocation::CommandInvocationSession;
use aureline_runtime::RuntimeEvidenceSupportExport;
use serde::{Deserialize, Serialize};

use aureline_support::bundle::{
    crash_incident_trail_preview as build_crash_incident_trail_preview, ActionPolicySourceContext,
    ActionReconstructionSeed, ActionabilityImpactClass, CrashIncidentTrail, DiagnosticDataClass,
    ExactBuildCapture, HighRiskContentClass, LocalFirstDefaults, PreviewItemSeed, RedactionState,
    SizeEstimate, SupportBundleManifest, SupportBundlePreview, SupportBundlePreviewBuilder,
    SupportBundlePreviewError,
};

use crate::activity_center::alpha::ActivityCenterSupportExport;
use crate::activity_center::git_review::GitReviewSupportExport;
use crate::admin_alpha::{
    AdminAlphaSupportExport, ADMIN_ALPHA_ARCHIVE_BOUNDARY_CONTRACT_REF,
    ADMIN_ALPHA_DESTRUCTION_RECEIPT_SCHEMA_REF, ADMIN_ALPHA_SUPPORT_EXPORT_RECORD_KIND,
};
use crate::commands::review_enforcement::AlphaReviewEnforcementRow;
use crate::drift_truth::{
    DriftTruthExportAudience, DriftTruthSnapshot, DRIFT_TRUTH_EXPORT_PACKET_RECORD_KIND,
};
use crate::inspectors::schema_registry::EndpointPolicySupportExport;
use crate::managed_truth::{ManagedTruthSnapshot, MANAGED_TRUTH_EXPORT_PACKET_RECORD_KIND};
use crate::restore::provenance::RestoreProvenanceRecord;
use crate::run_context::ExecutionEntryTruthSnapshot;

/// Stable record-kind tag carried in serialized support-seed surfaces.
pub const SUPPORT_SEED_SURFACE_RECORD_KIND: &str = "support_seed_surface_record";

/// Schema version for the [`SupportSeedSurface`] payload shape.
pub const SUPPORT_SEED_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Reviewer-facing notice quoted on every support-seed surface so the
/// lane's depth is not overstated.
pub const SUPPORT_SEED_SCOPE_NOTICE: &str =
    "Support-bundle seed surface: bounded prototype not yet wired into the running shell. \
     The local-first redaction defaults, exact-build identity capture, and \
     reopen-after-export path are real; byte-level redaction, upload transport, hosted intake, \
     and ticket routing are reserved for a later milestone.";

/// Stable command id the chrome routes to when the reviewer asks to
/// preview the local support bundle. The id is held verbatim so the
/// command palette, the support pane, and the Help/About row agree.
pub const COMMAND_ID_OPEN_LOCAL_PREVIEW: &str = "cmd:support.open_local_preview";

/// Stable command id the chrome routes to when the reviewer asks to copy
/// the manifest JSON for support hand-off. The seed never uploads on its
/// own.
pub const COMMAND_ID_COPY_MANIFEST_JSON: &str = "cmd:support.copy_manifest_json";

/// Seed action available on the support-seed surface. Held as a closed enum
/// so callers cannot fabricate enabled action paths the seed has not promised.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportSeedActionKind {
    /// Reopen the local preview snapshot from disk without contacting
    /// any support service.
    OpenLocalPreview,
    /// Copy the manifest JSON to the clipboard for support hand-off.
    CopyManifestJson,
    /// Reserved — share-or-upload action that lights up in a later
    /// milestone. The chrome must keep the row visible but disabled
    /// rather than silently dropping it.
    ReservedShareOrUpload,
    /// Reserved — open hosted support intake. Reserved for a later
    /// milestone.
    ReservedOpenSupportIntake,
}

impl SupportSeedActionKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenLocalPreview => "open_local_preview",
            Self::CopyManifestJson => "copy_manifest_json",
            Self::ReservedShareOrUpload => "reserved_share_or_upload",
            Self::ReservedOpenSupportIntake => "reserved_open_support_intake",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenLocalPreview => "Open local preview",
            Self::CopyManifestJson => "Copy manifest JSON",
            Self::ReservedShareOrUpload => "Share or upload (reserved)",
            Self::ReservedOpenSupportIntake => "Open support intake (reserved)",
        }
    }

    /// True when the action is wired to an enabled command in this seed.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::OpenLocalPreview | Self::CopyManifestJson)
    }

    /// Stable command id when the action is enabled; `None` for reserved
    /// rows so the chrome cannot accidentally route them.
    pub const fn command_id(self) -> Option<&'static str> {
        match self {
            Self::OpenLocalPreview => Some(COMMAND_ID_OPEN_LOCAL_PREVIEW),
            Self::CopyManifestJson => Some(COMMAND_ID_COPY_MANIFEST_JSON),
            Self::ReservedShareOrUpload | Self::ReservedOpenSupportIntake => None,
        }
    }
}

/// One actionable row on the support-seed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportSeedAction {
    pub kind: SupportSeedActionKind,
    pub action_token: String,
    pub label: String,
    pub is_live: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserved_reason: Option<String>,
}

impl SupportSeedAction {
    fn from_kind(kind: SupportSeedActionKind) -> Self {
        Self {
            kind,
            action_token: kind.as_str().to_owned(),
            label: kind.label().to_owned(),
            is_live: kind.is_live(),
            command_id: kind.command_id().map(|id| id.to_owned()),
            reserved_reason: if kind.is_live() {
                None
            } else {
                Some(
                    "Action is reserved for a later milestone; the seed never silently activates it."
                        .into(),
                )
            },
        }
    }
}

/// Support-seed surface record: bounded prototype not yet wired into the running shell.
/// Wraps a built [`SupportBundlePreview`]; callers read this
/// record without fabricating state the underlying preview does not already
/// carry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportSeedSurface {
    pub record_kind: String,
    pub schema_version: u32,
    pub seed_scope_notice: String,
    pub heading: String,
    pub preview: SupportBundlePreview,
    pub honesty_marker_present: bool,
    pub actions: Vec<SupportSeedAction>,
}

impl SupportSeedSurface {
    /// Mint the seed surface from a built preview. The chrome reads the
    /// preview's manifest verbatim; this projection only adds copy and
    /// the closed action set.
    pub fn from_preview(preview: SupportBundlePreview, heading: impl Into<String>) -> Self {
        let honesty_marker_present = preview.honesty_marker_present();
        Self {
            record_kind: SUPPORT_SEED_SURFACE_RECORD_KIND.to_owned(),
            schema_version: SUPPORT_SEED_SURFACE_SCHEMA_VERSION,
            seed_scope_notice: SUPPORT_SEED_SCOPE_NOTICE.to_owned(),
            heading: heading.into(),
            preview,
            honesty_marker_present,
            actions: vec![
                SupportSeedAction::from_kind(SupportSeedActionKind::OpenLocalPreview),
                SupportSeedAction::from_kind(SupportSeedActionKind::CopyManifestJson),
                SupportSeedAction::from_kind(SupportSeedActionKind::ReservedShareOrUpload),
                SupportSeedAction::from_kind(SupportSeedActionKind::ReservedOpenSupportIntake),
            ],
        }
    }

    /// The default local preview every dogfood build can mint without
    /// extra inputs: the exact-build identity row and the policy/trust
    /// metadata row. This is the seed projection the protected walk
    /// exercises.
    ///
    /// `generated_at` is held as a parameter so callers can pin a
    /// deterministic timestamp in tests.
    pub fn default_local_preview(
        exact_build: ExactBuildCapture,
        generated_at: impl Into<String>,
    ) -> Result<Self, SupportBundlePreviewError> {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:local-default:0001",
            "Local-first support bundle preview",
            generated_at,
            exact_build,
        );
        builder
            .add_item(default_build_identity_seed())
            .add_item(default_policy_trust_seed());
        let preview = builder.build()?;
        Ok(Self::from_preview(preview, "Support — local-first preview"))
    }

    /// The failure-drill preview that proves the local-first defaults
    /// rewrite a queued secret-bearing row to `prohibited` and emit an
    /// excluded-class entry naming the row's support-pack item id.
    pub fn failure_drill_preview(
        exact_build: ExactBuildCapture,
        generated_at: impl Into<String>,
    ) -> Result<Self, SupportBundlePreviewError> {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:failure-drill:0001",
            "Failure drill: queued secret material is held back",
            generated_at,
            exact_build,
        );
        builder
            .add_item(default_build_identity_seed())
            .add_item(failure_drill_secret_seed());
        let preview = builder.build()?;
        Ok(Self::from_preview(
            preview,
            "Support — failure drill preview",
        ))
    }

    /// Mint a local support preview that cites the same restore provenance
    /// record shown by startup recovery, restore summaries, and diagnostics.
    pub fn restore_provenance_preview(
        exact_build: ExactBuildCapture,
        generated_at: impl Into<String>,
        provenance: &RestoreProvenanceRecord,
    ) -> Result<Self, SupportBundlePreviewError> {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:restore-provenance:0001",
            "Restore provenance support preview",
            generated_at,
            exact_build,
        );
        builder
            .add_item(default_build_identity_seed())
            .add_item(default_policy_trust_seed())
            .add_item(restore_provenance_seed(provenance));
        let preview = builder.build()?;
        Ok(Self::from_preview(
            preview,
            "Support — restore provenance preview",
        ))
    }

    /// Mint a local support preview that exports durable activity rows as
    /// structured records rather than scraped UI text.
    pub fn activity_center_preview(
        exact_build: ExactBuildCapture,
        generated_at: impl Into<String>,
        activity_export: &ActivityCenterSupportExport,
    ) -> Result<Self, SupportBundlePreviewError> {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:activity-center:0001",
            "Activity-center support preview",
            generated_at,
            exact_build,
        );
        builder
            .add_item(default_build_identity_seed())
            .add_item(default_policy_trust_seed())
            .add_item(activity_center_seed(activity_export));
        let preview = builder.build()?;
        Ok(Self::from_preview(
            preview,
            "Support — activity-center preview",
        ))
    }

    /// Mint a local support preview that exports Git/review activity events as
    /// structured branch, target, action, and exact-reopen records.
    pub fn git_review_event_preview(
        exact_build: ExactBuildCapture,
        generated_at: impl Into<String>,
        git_review_export: &GitReviewSupportExport,
    ) -> Result<Self, SupportBundlePreviewError> {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:git-review-activity:0001",
            "Git/review activity support preview",
            generated_at,
            exact_build,
        );
        builder
            .add_item(default_build_identity_seed())
            .add_item(default_policy_trust_seed())
            .add_item(git_review_event_seed(git_review_export));
        let preview = builder.build()?;
        Ok(Self::from_preview(
            preview,
            "Support — Git/review activity preview",
        ))
    }

    /// Mint a local support preview for a reviewed command invocation.
    /// The manifest records the command id, invocation id, target,
    /// route, exposure, policy source, redaction class, and exact-build
    /// refs without scraping command-palette or activity-center text.
    pub fn reviewed_command_route_preview(
        exact_build: ExactBuildCapture,
        generated_at: impl Into<String>,
        review_row: &AlphaReviewEnforcementRow,
        invocation_session: &CommandInvocationSession,
    ) -> Result<Self, SupportBundlePreviewError> {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:reviewed-command-route:0001",
            "Reviewed command route support preview",
            generated_at,
            exact_build,
        );
        builder
            .add_item(default_build_identity_seed())
            .add_item(default_policy_trust_seed())
            .add_item(reviewed_command_route_seed(review_row, invocation_session));
        builder.add_action_reconstruction_context(reviewed_command_action_reconstruction_seed(
            review_row,
            invocation_session,
        ));
        let preview = builder.build()?;
        Ok(Self::from_preview(
            preview,
            "Support - reviewed command route preview",
        ))
    }

    /// Mint a local support preview that links a crash envelope,
    /// exact-build symbolication status, trace IDs, and support-bundle
    /// manifest ref into one incident trail row.
    pub fn crash_incident_trail_preview(
        exact_build: ExactBuildCapture,
        generated_at: impl Into<String>,
        trail: &CrashIncidentTrail,
    ) -> Result<Self, SupportBundlePreviewError> {
        let preview = build_crash_incident_trail_preview(exact_build, generated_at, trail)?;
        Ok(Self::from_preview(
            preview,
            "Support - crash incident trail preview",
        ))
    }

    /// Mint a local support preview that exports the schema registry,
    /// endpoint-policy, and operational-signal inspection as structured
    /// metadata instead of screenshots or copied prose.
    pub fn schema_registry_endpoint_policy_preview(
        exact_build: ExactBuildCapture,
        generated_at: impl Into<String>,
        endpoint_policy_export: &EndpointPolicySupportExport,
    ) -> Result<Self, SupportBundlePreviewError> {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:schema-endpoint-policy:0001",
            "Schema registry endpoint-policy support preview",
            generated_at,
            exact_build,
        );
        builder
            .add_item(default_build_identity_seed())
            .add_item(default_policy_trust_seed())
            .add_item(schema_registry_endpoint_policy_seed(endpoint_policy_export));
        let preview = builder.build()?;
        Ok(Self::from_preview(
            preview,
            "Support — schema and endpoint policy preview",
        ))
    }

    /// Mint a local support preview that exports admin delete, legal-hold,
    /// chronology, and policy-diff truth as structured metadata.
    pub fn admin_delete_hold_policy_preview(
        exact_build: ExactBuildCapture,
        generated_at: impl Into<String>,
        admin_export: &AdminAlphaSupportExport,
    ) -> Result<Self, SupportBundlePreviewError> {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:admin-delete-hold-policy:0001",
            "Admin delete, hold, and policy diff support preview",
            generated_at,
            exact_build,
        );
        builder
            .add_item(default_build_identity_seed())
            .add_item(default_policy_trust_seed())
            .add_item(admin_delete_hold_policy_seed(admin_export));
        let preview = builder.build()?;
        Ok(Self::from_preview(
            preview,
            "Support - admin delete and policy truth preview",
        ))
    }

    /// Mint a local support preview that exports version-skew and drift truth
    /// for helper, provider snapshot, and saved-artifact surfaces as
    /// structured metadata.
    pub fn drift_truth_preview(
        exact_build: ExactBuildCapture,
        generated_at: impl Into<String>,
        drift_snapshot: &DriftTruthSnapshot,
    ) -> Result<Self, SupportBundlePreviewError> {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:version-skew-drift-truth:0001",
            "Version-skew and drift truth support preview",
            generated_at,
            exact_build,
        );
        builder
            .add_item(default_build_identity_seed())
            .add_item(default_policy_trust_seed())
            .add_item(drift_truth_seed(drift_snapshot));
        let preview = builder.build()?;
        Ok(Self::from_preview(
            preview,
            "Support - version-skew and drift truth preview",
        ))
    }

    /// Mint a local support preview that exports managed/provider-linked
    /// region, residency, tenant, storage/copy, key, and plane-state truth as
    /// structured metadata.
    pub fn managed_truth_preview(
        exact_build: ExactBuildCapture,
        generated_at: impl Into<String>,
        managed_truth: &ManagedTruthSnapshot,
    ) -> Result<Self, SupportBundlePreviewError> {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:managed-truth:0001",
            "Managed region, residency, tenant, and key truth support preview",
            generated_at,
            exact_build,
        );
        builder
            .add_item(default_build_identity_seed())
            .add_item(default_policy_trust_seed())
            .add_item(managed_truth_seed(managed_truth));
        let preview = builder.build()?;
        Ok(Self::from_preview(
            preview,
            "Support - managed region and key truth preview",
        ))
    }

    /// Mint a local support preview that exports profile-session,
    /// trace-bundle, replay-capability, and comparison-class truth as one
    /// structured runtime-evidence row.
    pub fn runtime_evidence_preview(
        exact_build: ExactBuildCapture,
        generated_at: impl Into<String>,
        runtime_evidence: &RuntimeEvidenceSupportExport,
    ) -> Result<Self, SupportBundlePreviewError> {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:runtime-evidence:0001",
            "Runtime evidence support preview",
            generated_at,
            exact_build,
        );
        builder
            .add_item(default_build_identity_seed())
            .add_item(default_policy_trust_seed())
            .add_item(runtime_evidence_seed(runtime_evidence));
        let preview = builder.build()?;
        Ok(Self::from_preview(
            preview,
            "Support - runtime evidence preview",
        ))
    }

    /// Mint a local support preview for execution-entry toolchain detection.
    pub fn execution_entry_toolchains_preview(
        exact_build: ExactBuildCapture,
        generated_at: impl Into<String>,
        snapshot: &ExecutionEntryTruthSnapshot,
    ) -> Result<Self, SupportBundlePreviewError> {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:execution-entry-toolchains:0001",
            "Execution-entry toolchain support preview",
            generated_at,
            exact_build,
        );
        builder
            .add_item(default_build_identity_seed())
            .add_item(default_policy_trust_seed())
            .add_item(execution_entry_toolchains_seed(snapshot));
        let preview = builder.build()?;
        Ok(Self::from_preview(
            preview,
            "Support - execution entry toolchains preview",
        ))
    }

    /// Convenience: the manifest the export writer would emit. Held as a
    /// borrowed accessor so the chrome never needs to clone the manifest
    /// just to render a row count.
    pub fn manifest(&self) -> &SupportBundleManifest {
        &self.preview.manifest
    }

    /// True when at least one preview row is in a state that prevented
    /// the seed from minting an `included` row (prohibited, omitted,
    /// retained-local-only, ...). Drives the chrome's honesty banner.
    pub fn has_prohibited_row(&self) -> bool {
        self.preview.manifest.has_prohibited_row()
    }

    /// True when the manifest carries at least one exact-build ref.
    /// The protected-walk acceptance condition.
    pub fn has_exact_build_identity(&self) -> bool {
        self.preview.manifest.has_exact_build_identity()
    }

    /// Number of rows the chrome renders in the preview pane.
    pub fn preview_row_count(&self) -> usize {
        self.preview.manifest.preview_items.len()
    }

    /// Find the first action with the given kind.
    pub fn find_action(&self, kind: SupportSeedActionKind) -> Option<&SupportSeedAction> {
        self.actions.iter().find(|a| a.kind == kind)
    }

    /// True when at least one preview row carries a redaction state
    /// other than `not_required_metadata`. Used by reviewers who want
    /// to confirm the seed never silently widens beyond metadata.
    pub fn carries_non_metadata_row(&self) -> bool {
        self.preview.manifest.preview_items.iter().any(|item| {
            !matches!(
                item.redaction.redaction_state,
                RedactionState::NotRequiredMetadata
            )
        })
    }
}

fn default_build_identity_seed() -> PreviewItemSeed {
    PreviewItemSeed {
        support_pack_item_id: "support.item.build_identity".into(),
        title: "Exact build and install identity".into(),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "build_and_install_truth".into(),
        artifact_kind_class: "exact_build_identity_manifest".into(),
        manifest_path_ref: "preview_items[0]".into(),
        bundle_member_path_ref: Some("manifest/build_identity.json".into()),
        source_refs: vec![
            "docs/build/exact_build_identity_model.md".into(),
            "artifacts/support/support_evidence_pack_matrix.yaml#support.item.build_identity"
                .into(),
        ],
        size_estimate: SizeEstimate {
            estimated_bytes: Some(4096),
            confidence_class: "estimated".into(),
            display_label: "4 KB".into(),
            size_source_class: "collector_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::BlocksFirstActionableDiagnosis,
        impact_summary:
            "Removing this row would prevent support from matching crash, symbol, docs, and \
             release evidence to one build."
                .into(),
        notes: "Metadata-only; embedded by default under the local-first defaults.".into(),
    }
}

fn default_policy_trust_seed() -> PreviewItemSeed {
    PreviewItemSeed {
        support_pack_item_id: "support.item.policy_trust_state".into(),
        title: "Policy fingerprint and trust state".into(),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "governance_and_export_controls".into(),
        artifact_kind_class: "policy_fingerprint_summary".into(),
        manifest_path_ref: "preview_items[1]".into(),
        bundle_member_path_ref: Some("manifest/policy_trust.json".into()),
        source_refs: vec![
            "docs/policy/admin_policy_and_bundle_cache_contract.md".into(),
            "artifacts/support/support_evidence_pack_matrix.yaml#support.item.policy_trust_state"
                .into(),
        ],
        size_estimate: SizeEstimate {
            estimated_bytes: Some(2048),
            confidence_class: "estimated".into(),
            display_label: "2 KB".into(),
            size_source_class: "collector_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::High,
        impact_summary:
            "Without this row, support cannot tell which policy allowed or excluded bundle classes."
                .into(),
        notes: "Metadata-only; the manifest names the policy and trust state that governed \
                collection."
            .into(),
    }
}

fn failure_drill_secret_seed() -> PreviewItemSeed {
    PreviewItemSeed {
        support_pack_item_id: "support.item.raw_secrets".into(),
        title: "Captured secret material (failure drill)".into(),
        data_class: DiagnosticDataClass::HighRisk,
        high_risk_content_class: HighRiskContentClass::SecretBearing,
        bundle_section_class: "logs_traces_and_manifests".into(),
        artifact_kind_class: "raw_secret_capture".into(),
        manifest_path_ref: "preview_items[1]".into(),
        bundle_member_path_ref: None,
        source_refs: vec!["docs/security/safe_preview_trust_classes.md".into()],
        size_estimate: SizeEstimate {
            estimated_bytes: Some(2048),
            confidence_class: "upper_bound".into(),
            display_label: "<= 2 KB".into(),
            size_source_class: "upper_bound_policy".into(),
        },
        impact_class: ActionabilityImpactClass::None,
        impact_summary:
            "Removing this row has no diagnostic cost because raw secret bytes never travel.".into(),
        notes: "Failure drill: the local-first defaults rewrite this row to 'prohibited' before \
                export."
            .into(),
    }
}

fn runtime_evidence_seed(runtime_evidence: &RuntimeEvidenceSupportExport) -> PreviewItemSeed {
    PreviewItemSeed {
        support_pack_item_id: runtime_evidence.support_pack_item_id.clone(),
        title: "Profile, trace-bundle, replay capability, and comparison truth".into(),
        data_class: DiagnosticDataClass::HighRisk,
        high_risk_content_class: HighRiskContentClass::RawTraceOrTranscript,
        bundle_section_class: "logs_traces_and_manifests".into(),
        artifact_kind_class: "runtime_evidence_trace_bundle_manifest".into(),
        manifest_path_ref: "preview_items[2]".into(),
        bundle_member_path_ref: Some("manifest/runtime_evidence/support_export.json".into()),
        source_refs: vec![
            "schemas/runtime/profile_session_alpha.schema.json".into(),
            "schemas/runtime/trace_bundle_alpha.schema.json".into(),
            "schemas/runtime/replay_capability_alpha.schema.json".into(),
            "schemas/runtime/runtime_evidence_comparison_alpha.schema.json".into(),
            "docs/runtime/trace_replay_alpha.md".into(),
            "artifacts/support/support_evidence_pack_matrix.yaml#support.item.runtime_traces"
                .into(),
        ],
        size_estimate: SizeEstimate {
            estimated_bytes: Some(8192),
            confidence_class: "estimated".into(),
            display_label: "8 KB manifest".into(),
            size_source_class: "manifest_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::High,
        impact_summary:
            "Removing this row would drop mapping quality, comparison class, replay capability, \
             redaction, and retention truth from runtime-evidence support review."
                .into(),
        notes: format!(
            "Runtime evidence support export {}; profile {}; trace {}; replay {}; comparison {}; \
             context {}; exact build {}; mapping {}; comparison class {}; replay lane {}; \
             redaction {}; retention {}; raw payload exported: {}; import/view-only: {}.",
            runtime_evidence.export_id,
            runtime_evidence.profile_session_id,
            runtime_evidence.trace_bundle_id,
            runtime_evidence.replay_descriptor_id,
            runtime_evidence.comparison_packet_id,
            runtime_evidence.execution_context_id,
            runtime_evidence.exact_build_identity_ref,
            runtime_evidence.mapping_quality_state.as_str(),
            runtime_evidence.comparison_class.as_str(),
            runtime_evidence.replay_lane_state.as_str(),
            runtime_evidence.redaction_mode.as_str(),
            runtime_evidence.retention_class.as_str(),
            runtime_evidence.raw_payload_exported,
            runtime_evidence.import_view_only
        ),
    }
}

fn execution_entry_toolchains_seed(snapshot: &ExecutionEntryTruthSnapshot) -> PreviewItemSeed {
    let per_entry = snapshot
        .entries
        .iter()
        .map(|entry| {
            format!(
                "{}={}",
                entry.entry_point_token,
                if entry.context_summary.detected_toolchain_tokens.is_empty() {
                    "none".to_owned()
                } else {
                    entry.context_summary.detected_toolchain_tokens.join("|")
                }
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    PreviewItemSeed {
        support_pack_item_id: "support.item.execution_entry_toolchains".into(),
        title: "Execution-entry toolchain detection".into(),
        data_class: DiagnosticDataClass::EnvironmentAdjacent,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "runtime_and_toolchain_truth".into(),
        artifact_kind_class: "execution_entry_toolchain_summary".into(),
        manifest_path_ref: "preview_items[2]".into(),
        bundle_member_path_ref: Some("manifest/runtime/execution_entry_toolchains.json".into()),
        source_refs: vec![
            "schemas/runtime/execution_context.schema.json".into(),
            "docs/runtime/execution_context_seed.md".into(),
        ],
        size_estimate: SizeEstimate {
            estimated_bytes: Some(4096),
            confidence_class: "estimated".into(),
            display_label: "4 KB manifest".into(),
            size_source_class: "manifest_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::High,
        impact_summary:
            "Removing this row would prevent support from comparing toolchain detection across \
             terminal, task, test, debug, and AI entry points."
                .into(),
        notes: format!(
            "Execution-entry snapshot {}; entries {}; shared shape {}; toolchains: {}.",
            snapshot.workspace_id,
            snapshot.entries.len(),
            snapshot.all_entries_share_summary_shape,
            per_entry
        ),
    }
}

fn restore_provenance_seed(provenance: &RestoreProvenanceRecord) -> PreviewItemSeed {
    let summary = provenance.summary_line();
    PreviewItemSeed {
        support_pack_item_id: provenance.support_pack_item_id(),
        title: "Restore provenance and fidelity".into(),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "restore_and_recovery_truth".into(),
        artifact_kind_class: "state_restore_provenance_and_placeholder_record".into(),
        manifest_path_ref: "preview_items[2]".into(),
        bundle_member_path_ref: Some(format!(
            "manifest/restore_provenance/{}.json",
            provenance.restore_provenance_id
        )),
        source_refs: vec![
            "docs/state/restore_provenance_and_placeholder_contract.md".into(),
            "docs/ux/restore_fidelity_classes.md".into(),
            provenance.restore_provenance_id.clone(),
        ],
        size_estimate: SizeEstimate {
            estimated_bytes: Some(8192),
            confidence_class: "estimated".into(),
            display_label: "8 KB".into(),
            size_source_class: "collector_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::High,
        impact_summary:
            "Without this row, support cannot explain why restore fidelity was downgraded or \
             why live surfaces were not rerun."
                .into(),
        notes: summary,
    }
}

fn activity_center_seed(activity_export: &ActivityCenterSupportExport) -> PreviewItemSeed {
    PreviewItemSeed {
        support_pack_item_id: "support.item.activity_center_alpha_rows".into(),
        title: "Activity-center durable rows".into(),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "activity_and_attention_truth".into(),
        artifact_kind_class: "activity_center_support_export_record".into(),
        manifest_path_ref: "preview_items[2]".into(),
        bundle_member_path_ref: Some(format!(
            "manifest/activity_center/{}.json",
            activity_export.export_id
        )),
        source_refs: vec![
            "schemas/events/activity_row.schema.json".into(),
            "docs/ux/activity_center_alpha.md".into(),
            activity_export.export_id.clone(),
        ],
        size_estimate: SizeEstimate {
            estimated_bytes: Some(8192),
            confidence_class: "estimated".into(),
            display_label: "8 KB".into(),
            size_source_class: "collector_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::High,
        impact_summary:
            "Without this row, support cannot explain which durable activity jobs existed, \
             which exact row reopens them, or which task/test and restore failures survived \
             look-away."
                .into(),
        notes: format!(
            "Structured activity export includes {} row(s) from {} source row(s); raw private \
             material excluded: {}.",
            activity_export.row_count(),
            activity_export.source_snapshot_row_count,
            activity_export.raw_private_material_excluded,
        ),
    }
}

fn git_review_event_seed(git_review_export: &GitReviewSupportExport) -> PreviewItemSeed {
    PreviewItemSeed {
        support_pack_item_id: "support.item.git_review_activity_events".into(),
        title: "Git and review activity events".into(),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "activity_and_attention_truth".into(),
        artifact_kind_class: "git_review_event_support_export".into(),
        manifest_path_ref: "preview_items[2]".into(),
        bundle_member_path_ref: Some(format!(
            "manifest/git_review_activity/{}.json",
            git_review_export.export_id
        )),
        source_refs: vec![
            "schemas/support/git_review_event_alpha.schema.json".into(),
            "crates/aureline-shell/src/activity_center/git_review.rs".into(),
            git_review_export.export_id.clone(),
        ],
        size_estimate: SizeEstimate {
            estimated_bytes: Some(12288),
            confidence_class: "estimated".into(),
            display_label: "12 KB".into(),
            size_source_class: "collector_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::High,
        impact_summary:
            "Without this row, support cannot reconstruct which branch, target, action, and \
             exact reopen link were attached to Git publish or review failures."
                .into(),
        notes: format!(
            "Structured Git/review export includes {} row(s) from {} source event(s); branch, \
             target, and action identity preserved on {} row(s); raw private material excluded: {}.",
            git_review_export.row_count(),
            git_review_export.source_event_count,
            git_review_export.branch_target_action_complete_count,
            git_review_export.raw_private_material_excluded,
        ),
    }
}

fn reviewed_command_route_seed(
    review_row: &AlphaReviewEnforcementRow,
    invocation_session: &CommandInvocationSession,
) -> PreviewItemSeed {
    PreviewItemSeed {
        support_pack_item_id: "support.item.execution_context_summary".into(),
        title: "Reviewed command, target, route, and policy source".into(),
        data_class: DiagnosticDataClass::EnvironmentAdjacent,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "route_and_execution_truth".into(),
        artifact_kind_class: "action_route_truth_packet".into(),
        manifest_path_ref: "preview_items[2]".into(),
        bundle_member_path_ref: Some(format!(
            "manifest/route_and_execution/{}.json",
            invocation_session
                .invocation_session_id
                .replace([':', '/'], "_")
        )),
        source_refs: vec![
            "docs/commands/alpha_preview_apply_revert.md".into(),
            "docs/support/reconstruction_drill.md".into(),
            review_row.command_revision_ref.clone(),
            invocation_session.invocation_session_id.clone(),
        ],
        size_estimate: SizeEstimate {
            estimated_bytes: Some(8192),
            confidence_class: "estimated".into(),
            display_label: "8 KB".into(),
            size_source_class: "collector_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::High,
        impact_summary:
            "Without this row, support cannot reconstruct the command, target, route, exposure, \
             policy source, or reviewed invocation posture."
                .into(),
        notes: format!(
            "Reviewed command {} uses {} through {}; raw arguments and payload bodies excluded.",
            review_row.command_id,
            action_target_class_for_review(review_row),
            action_route_class_for_review(review_row),
        ),
    }
}

fn reviewed_command_action_reconstruction_seed(
    review_row: &AlphaReviewEnforcementRow,
    invocation_session: &CommandInvocationSession,
) -> ActionReconstructionSeed {
    ActionReconstructionSeed {
        support_pack_item_id: "support.item.execution_context_summary".into(),
        command_id: review_row.command_id.clone(),
        command_descriptor_ref: review_row.command_revision_ref.clone(),
        invocation_session_id: invocation_session.invocation_session_id.clone(),
        target_identity_ref: target_identity_ref_for_invocation(invocation_session),
        action_route_packet_ref: invocation_session
            .preview_posture
            .preview_record_ref
            .as_ref()
            .map(|preview_ref| format!("route-packet:{preview_ref}")),
        transport_decision_ref: None,
        action_origin_class: action_origin_class_for_invocation(invocation_session).into(),
        action_target_class: action_target_class_for_review(review_row).into(),
        action_route_class: action_route_class_for_review(review_row).into(),
        action_exposure_class: action_exposure_class_for_review(review_row).into(),
        origin_scope: None,
        traffic_origin: None,
        endpoint_class: None,
        transport_target_class: None,
        route_choice: None,
        egress_class: None,
        decision_outcome: None,
        route_truth_state: None,
        fallback_posture: None,
        actor_ref: None,
        occurred_at: None,
        policy_source: ActionPolicySourceContext {
            policy_source_ref: format!(
                "policy-source:{}",
                invocation_session.policy_context.policy_epoch
            ),
            policy_epoch: invocation_session.policy_context.policy_epoch.clone(),
            trust_state: invocation_session.policy_context.trust_state.clone(),
            policy_bundle_ref: invocation_session
                .policy_context
                .execution_context_id
                .clone(),
            source_class: "invocation_policy_context".into(),
        },
        route_summary:
            "Support can reconstruct command, invocation, target, route, exposure, policy source, \
             and exact build from this manifest row."
                .into(),
        reviewed_enforcement_ref: Some(format!(
            "alpha-review-enforcement:{}",
            review_row.command_id
        )),
        redaction_class: invocation_session.redaction_class.clone(),
    }
}

fn target_identity_ref_for_invocation(invocation_session: &CommandInvocationSession) -> String {
    invocation_session
        .context_refs
        .context_object_refs
        .first()
        .cloned()
        .or_else(|| {
            invocation_session
                .context_snapshot
                .focused_entity_ref
                .clone()
        })
        .or_else(|| {
            invocation_session
                .context_snapshot
                .execution_context_id
                .clone()
        })
        .unwrap_or_else(|| "target:unknown_target_class".into())
}

fn action_origin_class_for_invocation(
    invocation_session: &CommandInvocationSession,
) -> &'static str {
    match invocation_session.issuing_surface.as_str() {
        "headless_cli" | "cli" | "cli_headless" => "cli_invocation_local",
        "ai_tool_surface" | "ai_composer" => "ai_tool_call_local",
        "extension_host" => "extension_host_local",
        _ => "user_keystroke_local",
    }
}

fn action_target_class_for_review(review_row: &AlphaReviewEnforcementRow) -> &'static str {
    match review_row.lane_class.as_str() {
        "ai_mutation" => "ai_sandbox_target",
        "provider" | "git" => "connected_provider_target",
        "install" | "workspace" => "local_host_target",
        _ if review_row.effect_class == "external_effect" => "connected_provider_target",
        _ => "unknown_target_class",
    }
}

fn action_route_class_for_review(review_row: &AlphaReviewEnforcementRow) -> &'static str {
    match review_row.lane_class.as_str() {
        "provider" => "browser_handoff_route",
        "git" | "ai_mutation" => "approval_gated_route",
        "install" | "workspace" => "in_process_route",
        _ if review_row.approval_posture_class != "no_approval_required" => "approval_gated_route",
        _ => "heuristic_unknown_route",
    }
}

fn action_exposure_class_for_review(review_row: &AlphaReviewEnforcementRow) -> &'static str {
    match review_row.dominant_side_effect_class.as_str() {
        "provider_visible_mutation" | "remote_mutation" => "provider_visible_mutation",
        "install_or_update" | "writes_files" => "workspace_visible_mutation",
        "remote_publish" | "public_publish" => "publicly_visible_publish",
        _ if review_row.effect_class == "external_effect" => "exposure_unknown_requires_review",
        _ => "local_only_mutation",
    }
}

fn schema_registry_endpoint_policy_seed(
    endpoint_policy_export: &EndpointPolicySupportExport,
) -> PreviewItemSeed {
    PreviewItemSeed {
        support_pack_item_id: "support.item.schema_registry_endpoint_policy".into(),
        title: "Schema registry and endpoint-policy inspection".into(),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "governance_and_export_controls".into(),
        artifact_kind_class: "schema_registry_endpoint_policy_support_export".into(),
        manifest_path_ref: "preview_items[2]".into(),
        bundle_member_path_ref: Some(format!(
            "manifest/schema_registry_endpoint_policy/{}.json",
            endpoint_policy_export.export_id
        )),
        source_refs: vec![
            "crates/aureline-shell/src/inspectors/schema_registry/mod.rs".into(),
            "docs/privacy/endpoint_policy_alpha.md".into(),
            endpoint_policy_export.export_id.clone(),
        ],
        size_estimate: SizeEstimate {
            estimated_bytes: Some(12288),
            confidence_class: "estimated".into(),
            display_label: "12 KB".into(),
            size_source_class: "collector_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::High,
        impact_summary:
            "Without this row, support cannot explain which telemetry, support export, \
             endpoint-policy, and operational-signal rows were active or local-only."
                .into(),
        notes: format!(
            "Structured schema/endpoint-policy export includes {} schema row(s), {} endpoint \
             policy row(s), and {} operational signal slice(s); raw payload material excluded: {}.",
            endpoint_policy_export.schema_row_count(),
            endpoint_policy_export.endpoint_policy_row_count(),
            endpoint_policy_export.operational_signal_slice_count(),
            endpoint_policy_export.raw_payloads_excluded,
        ),
    }
}

fn admin_delete_hold_policy_seed(admin_export: &AdminAlphaSupportExport) -> PreviewItemSeed {
    PreviewItemSeed {
        support_pack_item_id: "support.item.admin_delete_hold_policy_truth".into(),
        title: "Admin delete, hold, chronology, and policy diff truth".into(),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "governance_and_export_controls".into(),
        artifact_kind_class: ADMIN_ALPHA_SUPPORT_EXPORT_RECORD_KIND.into(),
        manifest_path_ref: "preview_items[2]".into(),
        bundle_member_path_ref: Some(format!(
            "manifest/admin_delete_hold_policy/{}.json",
            admin_export.export_id.replace(':', "_")
        )),
        source_refs: vec![
            "crates/aureline-shell/src/admin_alpha/mod.rs".into(),
            "docs/admin/policy_diff_alpha.md".into(),
            "docs/governance/archive_search_destruction_alpha.md".into(),
            ADMIN_ALPHA_ARCHIVE_BOUNDARY_CONTRACT_REF.into(),
            ADMIN_ALPHA_DESTRUCTION_RECEIPT_SCHEMA_REF.into(),
            admin_export.export_id.clone(),
        ],
        size_estimate: SizeEstimate {
            estimated_bytes: Some((admin_export.row_count() as u64).saturating_mul(1536)),
            confidence_class: "estimated".into(),
            display_label: format!("{} admin delete/export row(s)", admin_export.row_count()),
            size_source_class: "row_count_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::High,
        impact_summary:
            "Without this row, support cannot tell whether delete/export results were completed, \
             partial, blocked by hold, policy-retained, outside platform scope, manually local, \
             or omitted by redaction."
                .into(),
        notes: format!(
            "Structured admin export includes {} delete/export row(s), {} result token(s), and \
             policy diff {}; raw payload material excluded: {}.",
            admin_export.row_count(),
            admin_export.result_vocabulary.len(),
            admin_export.policy_diff_preview.diff_id,
            admin_export.raw_payloads_excluded,
        ),
    }
}

fn drift_truth_seed(drift_snapshot: &DriftTruthSnapshot) -> PreviewItemSeed {
    let packet = drift_snapshot.export_packet(DriftTruthExportAudience::Support);
    let mut source_refs = BTreeSet::new();
    source_refs.insert("crates/aureline-shell/src/drift_truth/mod.rs".to_owned());
    source_refs.insert("docs/compat/version_skew_alpha.md".to_owned());
    source_refs.insert(packet.packet_id.clone());
    for row in &packet.rows {
        source_refs.extend(row.source_refs.iter().cloned());
    }

    PreviewItemSeed {
        support_pack_item_id: "support.item.version_skew_drift_truth".into(),
        title: "Version-skew and drift truth".into(),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "compatibility_and_drift_truth".into(),
        artifact_kind_class: DRIFT_TRUTH_EXPORT_PACKET_RECORD_KIND.into(),
        manifest_path_ref: "preview_items[2]".into(),
        bundle_member_path_ref: Some(format!(
            "manifest/compatibility/{}.json",
            packet.packet_id.replace(':', "_")
        )),
        source_refs: source_refs.into_iter().collect(),
        size_estimate: SizeEstimate {
            estimated_bytes: Some((packet.rows.len() as u64).saturating_mul(1024)),
            confidence_class: "estimated".into(),
            display_label: format!("{} drift row(s)", packet.rows.len()),
            size_source_class: "row_count_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::High,
        impact_summary:
            "Without this row, support cannot tell whether helper, provider, or saved-artifact \
             surfaces were blocked, stale, retry-only, or waiting for migration review."
                .into(),
        notes: format!(
            "Structured drift export includes {} row(s); raw payload material excluded: {}.",
            packet.rows.len(),
            packet.raw_payloads_excluded,
        ),
    }
}

fn managed_truth_seed(managed_truth: &ManagedTruthSnapshot) -> PreviewItemSeed {
    let packet = managed_truth.export_packet();
    let mut source_refs = BTreeSet::new();
    source_refs.insert("crates/aureline-shell/src/managed_truth/mod.rs".to_owned());
    source_refs.insert("docs/managed/region_residency_alpha.md".to_owned());
    source_refs.insert(packet.packet_id.clone());
    for row in &packet.rows {
        source_refs.extend(row.source_refs.iter().cloned());
    }

    PreviewItemSeed {
        support_pack_item_id: "support.item.managed_region_residency_truth".into(),
        title: "Managed and provider-linked boundary truth".into(),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "managed_boundary_truth".into(),
        artifact_kind_class: MANAGED_TRUTH_EXPORT_PACKET_RECORD_KIND.into(),
        manifest_path_ref: "preview_items[2]".into(),
        bundle_member_path_ref: Some(format!(
            "manifest/managed_truth/{}.json",
            packet.packet_id.replace(':', "_")
        )),
        source_refs: source_refs.into_iter().collect(),
        size_estimate: SizeEstimate {
            estimated_bytes: Some((packet.rows.len() as u64).saturating_mul(1280)),
            confidence_class: "estimated".into(),
            display_label: format!("{} managed truth row(s)", packet.rows.len()),
            size_source_class: "row_count_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::High,
        impact_summary:
            "Without this row, support cannot tell where claimed managed/provider-linked work \
             runs, where data or copies live, which tenant/key boundary applies, or whether a \
             control-plane versus data-plane impairment narrowed the action."
                .into(),
        notes: format!(
            "Structured managed-truth export includes {} row(s); raw payload material excluded: {}.",
            packet.rows.len(),
            packet.raw_payloads_excluded,
        ),
    }
}

/// Reuse the redaction-profile ref token so the chrome and the manifest
/// share one source of truth. Re-exported for test locality.
pub const ACTIVE_REDACTION_PROFILE_REF: &str = LocalFirstDefaults::PROFILE_REF;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    use aureline_runtime::{
        CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest,
        ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode,
        NodeToolchainDetectorConfig, PythonEnvironmentDetectorConfig, ScopeClass, TargetClass,
        TrustState, WorkspaceToolchainDetector, WorkspaceToolchainDetectorConfig,
    };
    use aureline_support::bundle::ReleaseChannelClass;

    use crate::run_context::{ExecutionEntryPoint, ExecutionEntrySurface};

    fn fixture_capture() -> ExactBuildCapture {
        ExactBuildCapture::for_fixture(
            "build-id:aureline:dev:0.0.0:x86_64-unknown-linux-gnu:debug:abcdef123456",
            "0.0.0",
            ReleaseChannelClass::DevLocal,
        )
    }

    fn baseline_resolver() -> ExecutionContextResolver {
        ExecutionContextResolver::new(ExecutionContextResolverConfig {
            workspace_id: "workspace:toolchains".to_owned(),
            profile_id: Some("profile:default".to_owned()),
            identity_mode: IdentityMode::AccountFreeLocal,
            policy_epoch: 1,
            workspace_default_target_class: TargetClass::LocalHost,
            workspace_default_working_directory: Some("/workspace/toolchains".to_owned()),
            workspace_default_scope_class: ScopeClass::CurrentRoot,
            local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
            environment_capsule_ref: EnvironmentCapsuleRef {
                capsule_id: "caps:workspace:toolchains".to_owned(),
                capsule_hash: "sha256:toolchains".to_owned(),
                resolved_schema_version: "1".to_owned(),
                drift_state: CapsuleDriftState::InSync,
            },
            resolver_version: "test-resolver".to_owned(),
        })
    }

    fn toolchain_fixture_root() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/runtime/toolchain_detection_entry_points/sample_workspace")
    }

    fn workspace_toolchain_detector() -> WorkspaceToolchainDetector {
        WorkspaceToolchainDetector::new(WorkspaceToolchainDetectorConfig {
            node_detector: NodeToolchainDetectorConfig {
                ambient_node_version: Some("22.11.0".to_owned()),
                ambient_npm_version: Some("10.9.0".to_owned()),
                ambient_yarn_version: Some("1.22.22".to_owned()),
                ambient_pnpm_version: Some("9.15.4".to_owned()),
                ..NodeToolchainDetectorConfig::default()
            },
            python_detector: PythonEnvironmentDetectorConfig {
                ambient_python_version: Some("3.12.7".to_owned()),
                ambient_interpreter_ref: Some("/usr/bin/python3".to_owned()),
                ambient_uv_version: Some("0.5.7".to_owned()),
                ambient_poetry_version: Some("1.8.4".to_owned()),
                ..PythonEnvironmentDetectorConfig::default()
            },
            ambient_tsc_version: Some("5.7.2".to_owned()),
            ambient_pytest_version: Some("8.3.4".to_owned()),
            ambient_ruff_version: Some("0.8.4".to_owned()),
            ambient_eslint_version: Some("9.16.0".to_owned()),
        })
    }

    #[test]
    fn protected_walk_default_preview_carries_exact_build_identity() {
        let surface =
            SupportSeedSurface::default_local_preview(fixture_capture(), "2026-05-10T07:00:00Z")
                .expect("build default preview");

        assert!(surface.has_exact_build_identity());
        assert!(!surface.has_prohibited_row());
        assert!(!surface.honesty_marker_present);
        assert_eq!(surface.preview_row_count(), 2);
        assert!(!surface.carries_non_metadata_row());
        assert_eq!(
            surface
                .preview
                .manifest
                .collection_context
                .active_redaction_profile_ref,
            ACTIVE_REDACTION_PROFILE_REF
        );

        let open = surface
            .find_action(SupportSeedActionKind::OpenLocalPreview)
            .expect("open-preview action present");
        assert!(open.is_live);
        assert_eq!(
            open.command_id.as_deref(),
            Some(COMMAND_ID_OPEN_LOCAL_PREVIEW)
        );

        let reserved = surface
            .find_action(SupportSeedActionKind::ReservedShareOrUpload)
            .expect("reserved share/upload row present");
        assert!(!reserved.is_live);
        assert!(reserved.command_id.is_none());
    }

    #[test]
    fn failure_drill_preview_holds_secret_back_and_lights_honesty_marker() {
        let surface =
            SupportSeedSurface::failure_drill_preview(fixture_capture(), "2026-05-10T07:01:00Z")
                .expect("build failure-drill preview");

        assert!(surface.has_prohibited_row());
        assert!(surface.honesty_marker_present);
        assert!(surface.has_exact_build_identity());

        // The reviewer can still find the secret row inside the preview;
        // it has not been silently dropped.
        let secret_row = surface
            .preview
            .manifest
            .preview_items
            .iter()
            .find(|i| i.parity_binding.support_pack_item_id == "support.item.raw_secrets")
            .expect("secret row preserved in preview");
        assert_eq!(
            secret_row.redaction.redaction_state,
            RedactionState::Prohibited
        );
        assert!(secret_row.redaction.visible_high_risk_label.is_some());

        // The redaction report names the prohibited row.
        assert!(surface
            .preview
            .manifest
            .redaction_report
            .prohibited_items_confirmed_absent
            .iter()
            .any(|id| id == "support.item.raw_secrets"));
        assert!(
            !surface
                .preview
                .manifest
                .redaction_report
                .secret_scan_summary
                .raw_secret_values_exported
        );
    }

    #[test]
    fn reserved_actions_are_not_silently_promoted() {
        let surface =
            SupportSeedSurface::default_local_preview(fixture_capture(), "2026-05-10T07:02:00Z")
                .expect("build default preview");

        for action in &surface.actions {
            match action.kind {
                SupportSeedActionKind::OpenLocalPreview
                | SupportSeedActionKind::CopyManifestJson => {
                    assert!(action.is_live, "{:?} must be live", action.kind);
                    assert!(action.command_id.is_some());
                }
                SupportSeedActionKind::ReservedShareOrUpload
                | SupportSeedActionKind::ReservedOpenSupportIntake => {
                    assert!(!action.is_live, "{:?} must stay reserved", action.kind);
                    assert!(action.command_id.is_none());
                    assert!(action.reserved_reason.is_some());
                }
            }
        }
    }

    #[test]
    fn surface_round_trips_through_serde() {
        let surface =
            SupportSeedSurface::default_local_preview(fixture_capture(), "2026-05-10T07:03:00Z")
                .expect("build default preview");
        let json = serde_json::to_string(&surface).expect("ser");
        let parsed: SupportSeedSurface = serde_json::from_str(&json).expect("de");
        assert_eq!(parsed, surface);
    }

    #[test]
    fn support_preview_lists_same_toolchain_detection_for_all_execution_entries() {
        let discovery = workspace_toolchain_detector()
            .detect_workspace(&toolchain_fixture_root(), "2026-05-15T12:00:00Z");
        let mut resolver = baseline_resolver();
        let inputs = [
            (
                ExecutionEntryPoint::Terminal,
                ExecutionContextRequest::local_terminal_seed(
                    "terminal.open",
                    TrustState::Trusted,
                    "mono:0",
                ),
            ),
            (
                ExecutionEntryPoint::Task,
                ExecutionContextRequest::package_script_task_seed(
                    "task.run.package_script",
                    TrustState::Trusted,
                    "mono:1",
                ),
            ),
            (
                ExecutionEntryPoint::Test,
                ExecutionContextRequest::test_seed(
                    "test.run.pytest",
                    TrustState::Trusted,
                    "mono:2",
                ),
            ),
            (
                ExecutionEntryPoint::DebugPrep,
                ExecutionContextRequest::debug_prep_seed(
                    "debug.prep.attach",
                    TrustState::Trusted,
                    "mono:3",
                ),
            ),
            (
                ExecutionEntryPoint::AiTool,
                ExecutionContextRequest::ai_tool_call_seed(
                    "ai.route.preview",
                    TrustState::Trusted,
                    "mono:4",
                ),
            ),
        ];

        let entries = inputs
            .into_iter()
            .map(|(entry_point, request)| {
                let context = resolver
                    .resolve(request)
                    .with_workspace_toolchain_discovery(discovery.clone());
                ExecutionEntrySurface::project(entry_point, &context)
            })
            .collect::<Vec<_>>();
        let expected_tokens = entries[0].context_summary.detected_toolchain_tokens.clone();
        assert_eq!(entries.len(), 5);
        for entry in &entries {
            assert_eq!(
                entry.context_summary.detected_toolchain_tokens,
                expected_tokens
            );
        }
        for required in [
            "node@22.11.0",
            "npm@10.9.0",
            "yarn@1.22.22",
            "pnpm@9.15.4",
            "python@3.12.7",
            "tsc@5.7.2",
            "pytest@8.3.4",
            "ruff@0.8.4",
            "eslint@9.16.0",
        ] {
            assert!(expected_tokens.contains(&required.to_owned()));
        }

        let snapshot = ExecutionEntryTruthSnapshot::from_entries("workspace:toolchains", entries);
        let surface = SupportSeedSurface::execution_entry_toolchains_preview(
            fixture_capture(),
            "2026-05-15T12:00:01Z",
            &snapshot,
        )
        .expect("support preview builds");
        let row = surface
            .manifest()
            .preview_items
            .iter()
            .find(|item| {
                item.parity_binding.support_pack_item_id
                    == "support.item.execution_entry_toolchains"
            })
            .expect("toolchain support row exists");

        for entry_token in ["terminal", "task", "test", "debug_prep", "ai_tool"] {
            assert!(row.notes.contains(entry_token), "missing {entry_token}");
        }
        assert!(row.notes.contains("node@22.11.0"));
        assert!(row.notes.contains("eslint@9.16.0"));
    }

    #[test]
    fn activity_center_preview_exports_structured_rows() {
        use crate::activity_center::alpha::{
            ActivityCancellabilityClass, ActivityCenterAlphaSnapshot, ActivityCenterSupportExport,
            ActivityJobFamily, ActivityProgressForm, ActivityRow, ActivityRowAction,
            ActivityRowActionAvailability, ActivityRowActionKind, ActivityRowCollapseState,
            ActivityRowDisplayState, ActivityRowImpact, ActivityRowInput, ActivityRowProgress,
            ActivityRowStateClass, ActivityRowSupportLink, ActivityRowTimeline,
        };
        use crate::notifications::{PrivacyClass, RedactionClass, SeverityClass, SourceSubsystem};

        let row = ActivityRow::from_input(ActivityRowInput {
            activity_row_id: "ux:activity-row:test:failed".into(),
            durable_job_id: "ux:durable-job:test:failed".into(),
            canonical_event_id: "ux:event:test:failed".into(),
            canonical_object_target_ref: "ux:durable-job:test:failed".into(),
            exact_reopen_identity_ref: "ux:activity-row:test:failed".into(),
            job_family: ActivityJobFamily::TestRun,
            source_subsystem: SourceSubsystem::TestRunner,
            actor_identity_ref: "id:actor:system:test-runner".into(),
            actor_or_subsystem_label: "Test runner".into(),
            execution_origin_class: "user_initiated".into(),
            severity_class: SeverityClass::Error,
            privacy_class: PrivacyClass::WorkspaceSensitive,
            summary_label: "Test run failed".into(),
            target_label: "Pytest".into(),
            target_scope_label: "Active workspace".into(),
            state_class: ActivityRowStateClass::Failed,
            progress: ActivityRowProgress {
                forms: vec![ActivityProgressForm::FailureOrPartialSummary],
                phase_label: "Review failed tests".into(),
                progress_bar: None,
                queue_reason_label: None,
                approval_source_label: None,
                actor_or_subsystem_label: "Test runner".into(),
                age_label: "Finished now".into(),
                indeterminate_reason_label: None,
                expected_boundary_class: "local".into(),
                cancellability_class: ActivityCancellabilityClass::AlreadyTerminal,
                detail_or_evidence_ref: Some("evidence:test:failed".into()),
            },
            timeline: ActivityRowTimeline {
                minted_at: "2026-05-13T03:12:00Z".into(),
                queued_at: Some("2026-05-13T03:12:00Z".into()),
                started_at: Some("2026-05-13T03:12:02Z".into()),
                last_observed_at: "2026-05-13T03:13:00Z".into(),
                finished_at: Some("2026-05-13T03:13:00Z".into()),
                archived_at: None,
                superseded_by_row_id_ref: None,
                retention_label: "Retained until resolved or archived".into(),
            },
            impact: ActivityRowImpact::routine("Local test output only."),
            actions: vec![ActivityRowAction {
                action_id: "action:activity:test:open".into(),
                action_kind: ActivityRowActionKind::OpenDetails,
                label: "Open test details".into(),
                command_id: Some("cmd:activity.open_job_details".into()),
                availability_class: ActivityRowActionAvailability::Enabled,
                disabled_reason_label: None,
                target_identity_ref: "ux:activity-row:test:failed".into(),
                preserves_durable_history: true,
                reissues_original_side_effect: false,
            }],
            display: ActivityRowDisplayState {
                collapse_state: ActivityRowCollapseState::CollapsedSummary,
                can_expand_inline: true,
                expand_reveals_label: "failure, evidence, and retry posture".into(),
            },
            support_link: ActivityRowSupportLink {
                exportable: true,
                support_pack_item_id: Some("support.item.activity.test_failed".into()),
                bundle_member_path_ref: Some("manifest/activity/test_failed.json".into()),
                redaction_class: RedactionClass::MetadataSafeDefault,
                raw_private_material_excluded: true,
                export_field_refs: vec!["export.activity.identity".into()],
            },
            git_review_context: None,
            occurrence_count: 1,
        });
        let snapshot = ActivityCenterAlphaSnapshot::from_rows(vec![row]);
        let export = ActivityCenterSupportExport::from_snapshot(
            "support-export:activity:test",
            "2026-05-13T03:14:00Z",
            &snapshot,
        );

        let surface = SupportSeedSurface::activity_center_preview(
            fixture_capture(),
            "2026-05-13T03:15:00Z",
            &export,
        )
        .expect("build activity preview");

        assert_eq!(surface.preview_row_count(), 3);
        assert!(surface.preview.manifest.preview_items.iter().any(|item| {
            item.parity_binding.support_pack_item_id == "support.item.activity_center_alpha_rows"
        }));
        assert!(!surface.has_prohibited_row());
    }

    #[test]
    fn git_review_preview_exports_structured_event_family() {
        use crate::activity_center::alpha::ActivityRowStateClass;
        use crate::activity_center::git_review::{
            GitReviewActionClass, GitReviewActionIdentity, GitReviewBranchContext,
            GitReviewEventFamily, GitReviewEventInput, GitReviewEventPhase, GitReviewEventRecord,
            GitReviewExactReopenLink, GitReviewReopenKind, GitReviewSupportExport,
            GitReviewSupportProjection, GitReviewTargetIdentity, GitReviewTargetKind,
        };
        use crate::notifications::SeverityClass;

        let event = GitReviewEventRecord::from_input(GitReviewEventInput {
            event_id: "event.git_review.publish.support_preview".into(),
            occurred_at: "2026-05-13T22:34:00Z".into(),
            event_family: GitReviewEventFamily::GitPublish,
            phase: GitReviewEventPhase::Failed,
            state_class: ActivityRowStateClass::Failed,
            severity_class: SeverityClass::Error,
            actor_identity_ref: "id:actor:local-user".into(),
            actor_label: "Local user".into(),
            workspace_ref: "workspace.repo.aureline".into(),
            summary_label: "Publish failed".into(),
            detail_label: "Publish review can reopen exactly.".into(),
            branch: GitReviewBranchContext::new(
                Some("feature/activity".into()),
                Some("refs/heads/feature/activity".into()),
                Some("refs/remotes/origin/feature/activity".into()),
                Some("git.rev.abc1234".into()),
            ),
            target: GitReviewTargetIdentity {
                canonical_target_ref: "git.publish.target.origin-feature".into(),
                target_kind: GitReviewTargetKind::RemoteRef,
                target_label: "origin/feature/activity".into(),
                scope_ref: None,
                target_refs: vec!["refs/heads/feature/activity".into()],
                review_workspace_ref: None,
                route_ref: Some("git.publish.route.origin".into()),
            },
            action: GitReviewActionIdentity {
                action_id: "action.git.publish.review".into(),
                action_class: GitReviewActionClass::PublishReview,
                command_id: "cmd:git.publish.review.reopen".into(),
                source_record_ref: "git.publish.preview.support".into(),
                preview_ref: Some("git.publish.preview.support".into()),
                result_ref: Some("git.publish.result.failed.support".into()),
                journal_ref: None,
                recovery_ref: Some("git.publish.recovery.support".into()),
                side_effect_class: "push_to_upstream".into(),
                reissues_original_side_effect: false,
            },
            exact_reopen_links: vec![GitReviewExactReopenLink::new(
                GitReviewReopenKind::GitPublishReview,
                "cmd:git.publish.review.reopen",
                "git.publish.preview.support",
                "Reopen publish review",
            )],
            support_projection: GitReviewSupportProjection::metadata_safe(
                "support.item.git_review.publish",
                "support.export.git_review.publish.support",
                "manifest/git_review_activity/publish_support.json",
            ),
        });
        let export = GitReviewSupportExport::from_events(
            "support.export.git_review.support_preview",
            "2026-05-13T22:35:00Z",
            &[event],
        );

        let surface = SupportSeedSurface::git_review_event_preview(
            fixture_capture(),
            "2026-05-13T22:36:00Z",
            &export,
        )
        .expect("build Git/review preview");

        assert_eq!(surface.preview_row_count(), 3);
        assert!(surface.preview.manifest.preview_items.iter().any(|item| {
            item.parity_binding.support_pack_item_id == "support.item.git_review_activity_events"
        }));
        assert!(!surface.has_prohibited_row());
    }

    #[test]
    fn schema_registry_endpoint_policy_preview_exports_structured_inspection() {
        use crate::inspectors::schema_registry::{
            DestinationClass, EndpointPolicyInspectionInput, OperationalSignalKind,
            OperationalSignalSlice, SchemaRegistryInspector, SignalFreshnessClass,
            SignalRedactionClass, SignalTimeWindow, SignalTruncationState,
        };

        let inspector = SchemaRegistryInspector::from_default_artifact_registers()
            .expect("load schema registers");
        let snapshot = inspector
            .inspect(EndpointPolicyInspectionInput {
                inspection_id: "inspection.schema_endpoint_policy.support_seed".into(),
                generated_at: "2026-05-14T01:00:00Z".into(),
                claimed_schema_refs: vec![
                    "telemetry.ux_product_event".into(),
                    "support.bundle_manifest".into(),
                    "schema_alpha:support_export.bundle_manifest".into(),
                ],
                operational_signal_slices: vec![OperationalSignalSlice {
                    signal_slice_id: "signal.slice.support_seed.log".into(),
                    signal_kind: OperationalSignalKind::Logs,
                    source_backend: "local structured log stream".into(),
                    source_backend_ref: "source.ref.support_seed.log".into(),
                    time_window: SignalTimeWindow {
                        window_start_utc: "2026-05-14T00:55:00Z".into(),
                        window_end_utc: "2026-05-14T01:00:00Z".into(),
                        display_time_zone_iana: "UTC".into(),
                        display_utc_offset: "+00:00".into(),
                    },
                    freshness: SignalFreshnessClass::Live,
                    truncation_state: SignalTruncationState::Clipped,
                    redaction_class: SignalRedactionClass::RedactedPayload,
                    retention_export_posture: "support export keeps metadata and omission notes"
                        .into(),
                    destination_class: DestinationClass::SupportBundle,
                }],
                support_export_id: "support.export.schema_endpoint_policy.support_seed".into(),
                runbook_handoff_id: "runbook.handoff.schema_endpoint_policy.support_seed".into(),
            })
            .expect("inspect endpoint policy");

        let surface = SupportSeedSurface::schema_registry_endpoint_policy_preview(
            fixture_capture(),
            "2026-05-14T01:01:00Z",
            &snapshot.support_export,
        )
        .expect("build endpoint-policy preview");

        assert_eq!(surface.preview_row_count(), 3);
        assert!(surface.preview.manifest.preview_items.iter().any(|item| {
            item.parity_binding.support_pack_item_id
                == "support.item.schema_registry_endpoint_policy"
        }));
        assert!(!surface.has_prohibited_row());
    }
}
