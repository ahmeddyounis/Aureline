//! Runtime-mutation action plan.
//!
//! Every mutation-capable preview / browser-runtime action — reload,
//! clear-storage, replay-request, live-style edit, restart-runtime,
//! navigate-tab, trigger-hot-reload, toggle-device-condition — publishes one
//! `RuntimeMutationActionPlan` *before* it is allowed to run.
//!
//! The plan names the action, its blast radius, the review requirement the
//! chrome must satisfy, and the support/export-safe side-effect summary the
//! surface must keep on screen. Inspect-only actions also publish a plan so
//! the chrome can keep the disclosure shape uniform — the difference is
//! whether `blast_class` is one of the mutation classes or
//! `no_mutation_inspect_only`.

use serde::{Deserialize, Serialize};

use super::{
    BrowserRuntimeSessionOrigin, PreviewOriginDescriptor, PreviewOriginFinding,
    PreviewTargetDescriptor,
};

/// Stable record-kind tag.
pub const RUNTIME_MUTATION_ACTION_PLAN_RECORD_KIND: &str = "runtime_mutation_action_plan_record";

/// Schema version mirrored by the boundary schemas this crate publishes.
pub const RUNTIME_MUTATION_ACTION_PLAN_SCHEMA_VERSION: u32 = 1;

/// Closed action-kind vocabulary. Names *what* the action does without
/// minting per-runtime synonyms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationActionKind {
    /// Inspect-only — read DOM / accessibility / console / network state.
    InspectOnly,
    /// Reload the current preview (refresh document, keep runtime).
    ReloadPreview,
    /// Clear browser storage (cookies, local / session storage,
    /// IndexedDB) on the target session.
    ClearBrowserStorage,
    /// Replay a previously-captured network request.
    ReplayNetworkRequest,
    /// Apply a live style edit through the source-mapped overlay.
    LiveStyleEdit,
    /// Restart the runtime process (full restart, in-memory state lost).
    RestartRuntime,
    /// Navigate the browser tab to a different in-app route.
    NavigateBrowserTab,
    /// Trigger a hot-reload event explicitly (most surfaces let the
    /// runtime drive this; explicit triggers are mutation-capable).
    TriggerHotReload,
    /// Toggle a device condition (offline, throttled network, locale).
    ToggleDeviceCondition,
    /// Export the inspector state. Mutation only in the sense that the
    /// export crosses the support / export boundary.
    ExportInspectorState,
}

impl MutationActionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOnly => "inspect_only",
            Self::ReloadPreview => "reload_preview",
            Self::ClearBrowserStorage => "clear_browser_storage",
            Self::ReplayNetworkRequest => "replay_network_request",
            Self::LiveStyleEdit => "live_style_edit",
            Self::RestartRuntime => "restart_runtime",
            Self::NavigateBrowserTab => "navigate_browser_tab",
            Self::TriggerHotReload => "trigger_hot_reload",
            Self::ToggleDeviceCondition => "toggle_device_condition",
            Self::ExportInspectorState => "export_inspector_state",
        }
    }

    /// True for actions that do not mutate any runtime / browser state.
    pub const fn is_inspect_only(self) -> bool {
        matches!(self, Self::InspectOnly | Self::ExportInspectorState)
    }
}

/// Closed blast-class vocabulary. Names *how far* the mutation reaches.
/// The chrome MUST quote the class verbatim — `local_runtime_only` MUST NOT
/// be inferred from a runtime that secretly mutates remote state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationBlastClass {
    /// No mutation — inspect-only.
    NoMutationInspectOnly,
    /// Mutation contained in the local dev-server / runtime process.
    LocalRuntimeOnly,
    /// Mutation contained in browser tab / webview state on the local
    /// machine (cookies, storage, navigation).
    LocalBrowserStateOnly,
    /// Mutation spans both the local runtime and local browser state.
    LocalRuntimeAndBrowserState,
    /// Mutation reaches a remote / container runtime.
    RemoteRuntimeReachable,
    /// Mutation reaches a managed preview service (governed boundary).
    ManagedPreviewServiceState,
    /// Mutation observable beyond the local workspace because the origin
    /// publishes a shared route audience (authenticated org, signed link,
    /// public).
    SharedRouteAudience,
    /// Not applicable — origin is imported / static evidence; mutation is
    /// never admissible.
    NotApplicableStaticEvidence,
}

impl MutationBlastClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoMutationInspectOnly => "no_mutation_inspect_only",
            Self::LocalRuntimeOnly => "local_runtime_only",
            Self::LocalBrowserStateOnly => "local_browser_state_only",
            Self::LocalRuntimeAndBrowserState => "local_runtime_and_browser_state",
            Self::RemoteRuntimeReachable => "remote_runtime_reachable",
            Self::ManagedPreviewServiceState => "managed_preview_service_state",
            Self::SharedRouteAudience => "shared_route_audience",
            Self::NotApplicableStaticEvidence => "not_applicable_static_evidence",
        }
    }

    /// True when this blast class implies mutation reaches remote /
    /// governed state. The chrome MUST NOT claim local-only safety in
    /// these cases.
    pub const fn implies_remote_or_governed(self) -> bool {
        matches!(
            self,
            Self::RemoteRuntimeReachable
                | Self::ManagedPreviewServiceState
                | Self::SharedRouteAudience
        )
    }

    /// True when this blast class is *some* mutation (i.e. not inspect /
    /// not applicable).
    pub const fn implies_any_mutation(self) -> bool {
        !matches!(
            self,
            Self::NoMutationInspectOnly | Self::NotApplicableStaticEvidence
        )
    }
}

/// Closed review-requirement vocabulary. The chrome MUST satisfy the
/// requirement before a mutation-capable action is admissible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationReviewRequirement {
    /// Inspect-only — no review required.
    NoReviewRequiredInspectOnly,
    /// Local mutation — the chrome must show an explicit confirm step
    /// before applying.
    ExplicitConfirmBeforeApply,
    /// Remote / governed mutation — the chrome must obtain an approval
    /// ticket from the managed-workspace flow before applying.
    ManagedApprovalRequiredBeforeApply,
    /// Not admissible — the action is blocked by policy / posture; the
    /// chrome must render the block reason rather than a confirm.
    BlockedNotAdmissible,
}

impl MutationReviewRequirement {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoReviewRequiredInspectOnly => "no_review_required_inspect_only",
            Self::ExplicitConfirmBeforeApply => "explicit_confirm_before_apply",
            Self::ManagedApprovalRequiredBeforeApply => "managed_approval_required_before_apply",
            Self::BlockedNotAdmissible => "blocked_not_admissible",
        }
    }
}

/// Canonical runtime-mutation action plan record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeMutationActionPlan {
    pub record_kind: String,
    pub runtime_mutation_action_plan_schema_version: u32,
    pub runtime_mutation_action_plan_id: String,
    /// ISO 8601 UTC monotonic timestamp.
    pub observed_at: String,

    /// Opaque ref to the preview-origin descriptor.
    pub preview_origin_descriptor_ref: String,
    /// Opaque ref to the preview-target descriptor.
    pub preview_target_descriptor_ref: String,
    /// Opaque ref to the browser-runtime session-origin record when the
    /// action runs against a browser session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_runtime_session_origin_ref: Option<String>,
    /// Opaque ref to the managed-workspace approval ticket — required
    /// when `review_requirement = managed_approval_required_before_apply`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_workspace_approval_ref: Option<String>,

    pub action_kind: MutationActionKind,
    pub blast_class: MutationBlastClass,
    pub review_requirement: MutationReviewRequirement,

    /// Reviewer-facing one-sentence side-effect summary. Always rendered
    /// on the action confirm chrome. Never contains raw URLs, raw
    /// selectors, raw cookies, raw bodies.
    pub side_effect_summary: String,
    /// One-sentence summary safe to embed in a support export or issue
    /// intake. Closed-vocabulary tokens only; never contains raw private
    /// app state.
    pub support_export_summary: String,
    /// Block reason — required when `review_requirement =
    /// blocked_not_admissible`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason_summary: Option<String>,
}

impl RuntimeMutationActionPlan {
    /// Run the cross-record honesty rules over the plan. Empty vec means
    /// clean; populated vec is what the chrome / audit must publish.
    pub fn validate(&self) -> Vec<PreviewOriginFinding> {
        let mut findings = Vec::new();
        let subject = self.runtime_mutation_action_plan_id.as_str();

        if self.record_kind != RUNTIME_MUTATION_ACTION_PLAN_RECORD_KIND {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    RUNTIME_MUTATION_ACTION_PLAN_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.runtime_mutation_action_plan_schema_version
            != RUNTIME_MUTATION_ACTION_PLAN_SCHEMA_VERSION
        {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.schema_version",
                subject,
                format!(
                    "schema_version must be {}, found {}",
                    RUNTIME_MUTATION_ACTION_PLAN_SCHEMA_VERSION,
                    self.runtime_mutation_action_plan_schema_version
                ),
            ));
        }

        // Action / blast consistency.
        if self.action_kind.is_inspect_only()
            && self.blast_class != MutationBlastClass::NoMutationInspectOnly
            && self.blast_class != MutationBlastClass::NotApplicableStaticEvidence
        {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.inspect_blast_mismatch",
                subject,
                "inspect_only / export_inspector_state actions cannot declare a mutation blast class",
            ));
        }
        if !self.action_kind.is_inspect_only()
            && matches!(self.blast_class, MutationBlastClass::NoMutationInspectOnly)
        {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.mutation_action_must_declare_blast",
                subject,
                "mutation-capable actions must declare a mutation blast class",
            ));
        }

        // Review-requirement / blast consistency.
        if matches!(
            self.review_requirement,
            MutationReviewRequirement::NoReviewRequiredInspectOnly
        ) && !matches!(
            self.blast_class,
            MutationBlastClass::NoMutationInspectOnly
                | MutationBlastClass::NotApplicableStaticEvidence
        ) {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.no_review_requires_inspect_blast",
                subject,
                "no_review_required_inspect_only requires blast_class = no_mutation_inspect_only or not_applicable_static_evidence",
            ));
        }
        if matches!(
            self.review_requirement,
            MutationReviewRequirement::ExplicitConfirmBeforeApply
        ) && self.blast_class.implies_remote_or_governed()
        {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.local_confirm_for_remote_blast",
                subject,
                "remote / governed blast class requires managed_approval_required_before_apply",
            ));
        }
        if matches!(
            self.review_requirement,
            MutationReviewRequirement::ManagedApprovalRequiredBeforeApply
        ) && self.managed_workspace_approval_ref.is_none()
        {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.managed_review_requires_approval_ref",
                subject,
                "managed_approval_required_before_apply requires a non-null managed_workspace_approval_ref",
            ));
        }
        if matches!(
            self.review_requirement,
            MutationReviewRequirement::BlockedNotAdmissible
        ) && self.block_reason_summary.is_none()
        {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.blocked_requires_block_reason",
                subject,
                "blocked_not_admissible requires a non-null block_reason_summary",
            ));
        }
        if !matches!(
            self.review_requirement,
            MutationReviewRequirement::BlockedNotAdmissible
        ) && self.block_reason_summary.is_some()
        {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.block_reason_only_when_blocked",
                subject,
                "block_reason_summary is only admissible when review_requirement = blocked_not_admissible",
            ));
        }

        // Static-evidence safety: only inspect / not-applicable / blocked.
        if matches!(
            self.blast_class,
            MutationBlastClass::NotApplicableStaticEvidence
        ) && !matches!(
            self.review_requirement,
            MutationReviewRequirement::NoReviewRequiredInspectOnly
                | MutationReviewRequirement::BlockedNotAdmissible
        ) {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.static_evidence_review_floor",
                subject,
                "not_applicable_static_evidence blast class requires no_review_required_inspect_only or blocked_not_admissible",
            ));
        }

        // Browser-runtime actions that mutate browser state must reference
        // a session-origin record.
        let touches_browser_state = matches!(
            self.action_kind,
            MutationActionKind::ClearBrowserStorage
                | MutationActionKind::NavigateBrowserTab
                | MutationActionKind::ReplayNetworkRequest
                | MutationActionKind::LiveStyleEdit
        );
        if touches_browser_state && self.browser_runtime_session_origin_ref.is_none() {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.browser_action_requires_session_origin",
                subject,
                "browser-state-mutating actions must reference a browser_runtime_session_origin record",
            ));
        }

        // Side-effect summary and support-export summary must be non-empty
        // for any mutation-capable action; the chrome cannot leave the
        // confirm row blank.
        if self.action_kind.is_inspect_only() {
            // Inspect-only summaries can still be non-empty; no rule.
        } else {
            if self.side_effect_summary.trim().is_empty() {
                findings.push(PreviewOriginFinding::new(
                    "runtime_mutation_action_plan.side_effect_summary_required",
                    subject,
                    "mutation-capable action must publish a non-empty side_effect_summary",
                ));
            }
            if self.support_export_summary.trim().is_empty() {
                findings.push(PreviewOriginFinding::new(
                    "runtime_mutation_action_plan.support_export_summary_required",
                    subject,
                    "mutation-capable action must publish a non-empty support_export_summary",
                ));
            }
        }

        findings
    }

    /// True when this plan declares an action the surface may actually run
    /// (review requirement satisfied / not blocked).
    pub fn is_admissible(&self) -> bool {
        !matches!(
            self.review_requirement,
            MutationReviewRequirement::BlockedNotAdmissible
        )
    }

    /// Cross-check the plan against the origin / target / session
    /// descriptors it references. The plan carries opaque refs but the
    /// chrome typically holds the full descriptors; this helper makes the
    /// "remote origin cannot publish local_runtime_only" rule explicit.
    pub fn cross_validate(
        &self,
        origin: &PreviewOriginDescriptor,
        target: &PreviewTargetDescriptor,
        session: Option<&BrowserRuntimeSessionOrigin>,
    ) -> Vec<PreviewOriginFinding> {
        let mut findings = self.validate();
        let subject = self.runtime_mutation_action_plan_id.as_str();

        if self.preview_origin_descriptor_ref != origin.preview_origin_descriptor_id {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.origin_ref_mismatch",
                subject,
                "preview_origin_descriptor_ref does not match supplied descriptor id",
            ));
        }
        if self.preview_target_descriptor_ref != target.preview_target_descriptor_id {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.target_ref_mismatch",
                subject,
                "preview_target_descriptor_ref does not match supplied descriptor id",
            ));
        }
        if let Some(session) = session {
            if self.browser_runtime_session_origin_ref.as_deref()
                != Some(session.browser_runtime_session_origin_id.as_str())
            {
                findings.push(PreviewOriginFinding::new(
                    "runtime_mutation_action_plan.session_ref_mismatch",
                    subject,
                    "browser_runtime_session_origin_ref does not match supplied session id",
                ));
            }
        }

        // Static-evidence origin must never carry a mutation blast class.
        if matches!(
            origin.origin_class,
            super::PreviewOriginClass::ImportedOrStaticEvidence
        ) && self.blast_class.implies_any_mutation()
        {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.static_evidence_forbids_mutation",
                subject,
                "imported_or_static_evidence origin cannot host a mutation-capable plan",
            ));
        }

        // Remote / managed origin cannot host a local-only blast claim
        // (the spec's local-only-safety overclaim rule).
        if origin.origin_class.touches_remote_or_governed()
            && matches!(
                self.blast_class,
                MutationBlastClass::LocalRuntimeOnly
                    | MutationBlastClass::LocalBrowserStateOnly
                    | MutationBlastClass::LocalRuntimeAndBrowserState
            )
        {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.local_only_safety_overclaim",
                subject,
                "remote / managed origin cannot host a local-only mutation blast class",
            ));
        }

        // Remote-audience sharing posture forces shared-route blast class
        // when the action mutates.
        if origin.sharing_posture.implies_remote_audience()
            && self.blast_class.implies_any_mutation()
            && !matches!(self.blast_class, MutationBlastClass::SharedRouteAudience)
        {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.remote_audience_requires_shared_route_blast",
                subject,
                "remote-audience sharing posture requires shared_route_audience blast class for mutations",
            ));
        }

        // Managed origin must escalate to managed-approval review.
        if matches!(
            origin.origin_class,
            super::PreviewOriginClass::ManagedPreviewService
        ) && self.blast_class.implies_any_mutation()
            && !matches!(
                self.review_requirement,
                MutationReviewRequirement::ManagedApprovalRequiredBeforeApply
                    | MutationReviewRequirement::BlockedNotAdmissible
            )
        {
            findings.push(PreviewOriginFinding::new(
                "runtime_mutation_action_plan.managed_origin_requires_managed_review",
                subject,
                "managed_preview_service origin requires managed_approval_required_before_apply for mutations",
            ));
        }

        if let Some(session) = session {
            if !session.admits_mutation() && self.blast_class.implies_any_mutation() {
                findings.push(PreviewOriginFinding::new(
                    "runtime_mutation_action_plan.session_does_not_admit_mutation",
                    subject,
                    "browser-runtime session does not admit mutation; plan must be inspect_only / blocked",
                ));
            }
        }

        findings
    }

    /// Render a deterministic plaintext summary safe to embed in support
    /// exports.
    pub fn render_plaintext(&self) -> String {
        format!(
            "runtime_mutation_plan {id} action={action} blast={blast} review={review}: {summary}",
            id = self.runtime_mutation_action_plan_id,
            action = self.action_kind.as_str(),
            blast = self.blast_class.as_str(),
            review = self.review_requirement.as_str(),
            summary = self.support_export_summary,
        )
    }
}
