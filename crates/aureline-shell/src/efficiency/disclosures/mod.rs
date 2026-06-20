//! Per-surface degraded disclosures for low-power and thermal posture.
//!
//! The parent [`crate::efficiency`] module owns the canonical efficiency-state
//! object model: the typed [`EfficiencyState`], the per-workload
//! [`WorkloadBudgetDecision`], the override and recovery vocabulary, and the
//! frozen governance matrix binding. The diagnostics and support-export
//! [`surfaces`][crate::efficiency::surfaces] then answer the operator questions
//! "what changed, why, and which subsystems were affected?".
//!
//! This module adds the missing *end-user* projection: a per-surface disclosure
//! that an affected product pane renders so a person can tell a surface is
//! intentionally **less fresh because of efficiency posture, not broken**. For
//! each of the surfaces the contract names — paused or slowed indexing,
//! assistant warmups, rich preview refresh, docs sync, marketplace refresh, and
//! optional uploads — a [`SurfaceDisclosure`] states three things the contract
//! requires:
//!
//! - **what still works now** — the protected edit/search/save/review path the
//!   adaptation never narrows, so the user is not left guessing;
//! - **what is delayed** — the specific freshness or assist that is reduced; and
//! - **how to inspect or override** — the open-details command and, only where
//!   policy allows, an explicit session override.
//!
//! Every disclosure derives its action, visible state, override posture, and
//! recovery state from the same canonical objects rather than minting local
//! low-power wording, so a disclosure, the status pill, the diagnostics row, and
//! the support export can never disagree about the active posture. Disclosures
//! are emitted only for surfaces whose behavior *materially changed*; a surface
//! still running within budget is listed in
//! [`EfficiencySurfaceDisclosures::unaffected_surface_tokens`] and shows no
//! banner, honoring the "no permanent banner for unchanged behavior" guardrail.
//!
//! The disclosures are inspectable truth packets, not toast text: each carries a
//! [`DisclosurePlacement`] that records the persistent inline anchor it lives on,
//! and asserts it is neither toast-only (which would lose long-lived low-power
//! truth) nor placed in the typing hot path.

use serde::{Deserialize, Serialize};

use super::governance::{
    EfficiencyGovernanceProjection, HiddenPaneBehavior, OverridePosture,
    M5_EFFICIENCY_GOVERNANCE_MATRIX_REF, M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF,
};
use super::surfaces::{EFFICIENCY_DETAILS_SURFACE_REF, EFFICIENCY_INSPECT_COMMAND_ID};
use super::{
    derive_override_posture, derive_recovery_state, protected_interactions,
    EfficiencyPressureSource, EfficiencyState, EfficiencyStateSnapshot, WorkloadBudgetDecision,
    WorkloadFamily,
};
use crate::notifications::envelope::SourceSubsystem;

#[cfg(test)]
mod tests;

/// Stable record kind for [`EfficiencySurfaceDisclosures`] payloads.
pub const EFFICIENCY_SURFACE_DISCLOSURES_RECORD_KIND: &str = "efficiency_surface_disclosures";

/// Stable record kind for an individual [`SurfaceDisclosure`].
pub const SURFACE_DISCLOSURE_RECORD_KIND: &str = "efficiency_surface_disclosure";

/// Schema version shared by the disclosure set and its rows.
pub const EFFICIENCY_SURFACE_DISCLOSURES_SCHEMA_VERSION: u32 = 1;

/// A product surface whose freshness or assist can be reduced by efficiency posture.
///
/// These are the user-facing panes the spec requires a degraded disclosure for.
/// Each maps to the canonical [`WorkloadFamily`] whose budget decision governs
/// the surface's freshness — see [`DisclosureSurface::governing_family`] — so the
/// disclosure's action, visible state, and override posture come from the frozen
/// policy rather than per-surface invention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisclosureSurface {
    /// Whole-workspace indexing and broad search freshness.
    PausedIndexing,
    /// Background assistant warmups that speed the first response.
    AiWarmups,
    /// Live re-rendering of a rich preview surface.
    RichPreviewRefresh,
    /// Background sync of docs and help content.
    DocsSync,
    /// Background refresh of the extension marketplace catalog.
    MarketplaceRefresh,
    /// Optional uploads, replication, and deferred transfer.
    OptionalUploads,
}

impl DisclosureSurface {
    /// Every disclosure surface, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::PausedIndexing,
        Self::AiWarmups,
        Self::RichPreviewRefresh,
        Self::DocsSync,
        Self::MarketplaceRefresh,
        Self::OptionalUploads,
    ];

    /// Stable token recorded in disclosures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PausedIndexing => "paused_indexing",
            Self::AiWarmups => "ai_warmups",
            Self::RichPreviewRefresh => "rich_preview_refresh",
            Self::DocsSync => "docs_sync",
            Self::MarketplaceRefresh => "marketplace_refresh",
            Self::OptionalUploads => "optional_uploads",
        }
    }

    /// Title-cased label rendered as the disclosure subject.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PausedIndexing => "Indexing refresh",
            Self::AiWarmups => "Assistant warmups",
            Self::RichPreviewRefresh => "Rich preview refresh",
            Self::DocsSync => "Docs sync",
            Self::MarketplaceRefresh => "Marketplace refresh",
            Self::OptionalUploads => "Optional uploads",
        }
    }

    /// Lower-case noun phrase used inside disclosure sentences.
    pub const fn subject(self) -> &'static str {
        match self {
            Self::PausedIndexing => "whole-workspace indexing",
            Self::AiWarmups => "assistant warmups",
            Self::RichPreviewRefresh => "rich preview refresh",
            Self::DocsSync => "docs sync",
            Self::MarketplaceRefresh => "marketplace refresh",
            Self::OptionalUploads => "optional uploads",
        }
    }

    /// Canonical subsystem that owns the surface, for vocabulary traceability.
    pub const fn owner_subsystem(self) -> SourceSubsystem {
        match self {
            Self::PausedIndexing => SourceSubsystem::Indexer,
            Self::AiWarmups => SourceSubsystem::AiApply,
            Self::RichPreviewRefresh => SourceSubsystem::ReviewAndDiff,
            Self::DocsSync => SourceSubsystem::DocsHelpServiceHealth,
            Self::MarketplaceRefresh => SourceSubsystem::ExtensionHost,
            Self::OptionalUploads => SourceSubsystem::SyncMirror,
        }
    }

    /// Human-readable owner label rendered next to the disclosure.
    pub const fn owner_label(self) -> &'static str {
        match self {
            Self::PausedIndexing => "Indexer",
            Self::AiWarmups => "AI runtime",
            Self::RichPreviewRefresh => "Preview",
            Self::DocsSync => "Docs/help",
            Self::MarketplaceRefresh => "Extensions",
            Self::OptionalUploads => "Sync/mirror",
        }
    }

    /// The canonical workload family whose budget decision governs this surface's
    /// freshness. The disclosure reuses this family's action, visible state, and
    /// checkpoint policy so it can never disagree with the rest of the
    /// efficiency-state contract about what the adaptation did. Docs sync shares
    /// the rich-content-refresh budget with preview, and marketplace refresh
    /// shares the extension-background-refresh budget, so neither invents a
    /// parallel low-power policy.
    pub const fn governing_family(self) -> WorkloadFamily {
        match self {
            Self::PausedIndexing => WorkloadFamily::IndexingRefresh,
            Self::AiWarmups => WorkloadFamily::AiWarmup,
            Self::RichPreviewRefresh => WorkloadFamily::PreviewRefresh,
            Self::DocsSync => WorkloadFamily::PreviewRefresh,
            Self::MarketplaceRefresh => WorkloadFamily::ExtensionPolling,
            Self::OptionalUploads => WorkloadFamily::UploadTransfer,
        }
    }

    /// The protected path that stays fully responsive while this surface is
    /// reduced. Answers "what still works now?" so the user is not left assuming
    /// the surface is broken.
    pub const fn still_works_now(self) -> &'static str {
        match self {
            Self::PausedIndexing => {
                "Editing, saving, and search across open files stay fully responsive; only whole-workspace index refresh is reduced."
            }
            Self::AiWarmups => {
                "You can still invoke the assistant; only the background warmups that speed the first response are paused."
            }
            Self::RichPreviewRefresh => {
                "The last rendered preview stays visible and navigable; only live re-rendering is reduced."
            }
            Self::DocsSync => {
                "Already-open docs and help stay readable; only background content sync is reduced."
            }
            Self::MarketplaceRefresh => {
                "Installed extensions keep working and the marketplace stays browsable; only its background catalog refresh is reduced."
            }
            Self::OptionalUploads => {
                "Your work is saved locally and stays attributable; only optional uploads and replication are deferred."
            }
        }
    }

    /// The specific freshness or assist that is delayed. Answers "what is
    /// delayed?" without implying anything protected stopped.
    pub const fn what_is_delayed(self) -> &'static str {
        match self {
            Self::PausedIndexing => {
                "Whole-workspace indexing and broad search freshness may lag behind recent edits until it resumes."
            }
            Self::AiWarmups => {
                "The assistant may take longer to respond the first time until warmups resume."
            }
            Self::RichPreviewRefresh => {
                "Preview content may be a few edits stale until live refresh resumes."
            }
            Self::DocsSync => {
                "Docs and help may show a slightly older synced copy until sync resumes."
            }
            Self::MarketplaceRefresh => {
                "New or updated marketplace listings may not appear until the catalog refresh resumes."
            }
            Self::OptionalUploads => {
                "Queued uploads, replication, and sync stay pending until an allowed send or staged resume."
            }
        }
    }

    /// The persistent inline anchor this surface's disclosure lives on. It is the
    /// surface's own status affordance — never a toast and never the typing hot
    /// path — so a long-lived low-power effect stays inspectable in place.
    pub const fn placement_anchor(self) -> &'static str {
        match self {
            Self::PausedIndexing => "search_results_status_affordance",
            Self::AiWarmups => "assistant_panel_status_affordance",
            Self::RichPreviewRefresh => "preview_pane_status_affordance",
            Self::DocsSync => "docs_pane_status_affordance",
            Self::MarketplaceRefresh => "marketplace_pane_status_affordance",
            Self::OptionalUploads => "activity_center_deferred_row",
        }
    }

    /// Resolves a stable token back into its disclosure surface, if known.
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|surface| surface.as_str() == token)
    }
}

/// Where and how a disclosure is shown, proving it is durable inline truth rather
/// than a toast and that it never sits in the typing hot path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisclosurePlacement {
    /// Persistent inline anchor the disclosure renders on.
    pub anchor: String,
    /// True when the disclosure persists for as long as the effect is active, so
    /// long-lived low-power truth is never lost to a dismissed toast.
    pub persistent_while_active: bool,
    /// Always false: a long-lived low-power effect is never toast-only.
    pub toast_only: bool,
    /// Always false: the disclosure is kept out of the typing hot path.
    pub in_typing_hot_path: bool,
    /// True when the disclosure respects reduced-motion settings.
    pub reduced_motion_safe: bool,
}

impl DisclosurePlacement {
    fn for_surface(surface: DisclosureSurface) -> Self {
        Self {
            anchor: surface.placement_anchor().to_owned(),
            persistent_while_active: true,
            toast_only: false,
            in_typing_hot_path: false,
            reduced_motion_safe: true,
        }
    }

    /// True when the placement honors the long-lived-truth and hot-path rules.
    pub const fn is_durable_inline_truth(&self) -> bool {
        self.persistent_while_active && !self.toast_only && !self.in_typing_hot_path
    }
}

/// The open-details affordance a disclosure exposes so the user can inspect the
/// full efficiency state behind the change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisclosureInspectHint {
    /// Command id that opens the full efficiency-state details.
    pub command_id: String,
    /// Surface ref the open-details command opens.
    pub opens_surface_ref: String,
    /// Label rendered on the open-details affordance.
    pub label: String,
}

impl Default for DisclosureInspectHint {
    fn default() -> Self {
        Self {
            command_id: EFFICIENCY_INSPECT_COMMAND_ID.to_owned(),
            opens_surface_ref: EFFICIENCY_DETAILS_SURFACE_REF.to_owned(),
            label: "Open efficiency details".to_owned(),
        }
    }
}

/// Whether and how the user may override this surface's reduction. Overrides are
/// explicit and policy-aware: the affordance is offered only when the active
/// [`OverridePosture`] allows it, and names the blocking policy when it does not.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisclosureOverrideAffordance {
    /// Override-posture token the affordance derives from.
    pub posture: String,
    /// True when the user may override the reduction.
    pub override_allowed: bool,
    /// Label for the override affordance when one is offered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_label: Option<String>,
    /// Policy reference that blocks the override, when one applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_blocked_ref: Option<String>,
    /// One-sentence explanation of the override posture.
    pub explanation: String,
}

impl DisclosureOverrideAffordance {
    fn for_surface(surface: DisclosureSurface, posture: OverridePosture) -> Self {
        let (override_allowed, override_label, policy_blocked_ref, explanation) = match posture {
            OverridePosture::UserOverrideSessionOnly => (
                true,
                Some(format!("Keep {} running this session", surface.subject())),
                None,
                "On battery you can override this for the current session.".to_owned(),
            ),
            OverridePosture::UserOverridePersistent => (
                true,
                Some(format!("Keep {} running", surface.subject())),
                None,
                "You can override this reduction and keep it off.".to_owned(),
            ),
            OverridePosture::PolicyBlocked => (
                false,
                None,
                Some("policy:efficiency.override_blocked".to_owned()),
                "Admin or local policy capped this work, so it cannot be overridden here."
                    .to_owned(),
            ),
            OverridePosture::AdminControlled => (
                false,
                None,
                Some("policy:efficiency.admin_controlled".to_owned()),
                "Admin policy controls whether this work runs.".to_owned(),
            ),
            OverridePosture::NotOverridable => (
                false,
                None,
                None,
                "This protects core interaction, so it cannot be overridden until pressure clears."
                    .to_owned(),
            ),
        };
        Self {
            posture: posture.as_str().to_owned(),
            override_allowed,
            override_label,
            policy_blocked_ref,
            explanation,
        }
    }
}

/// One per-surface degraded disclosure.
///
/// It names what still works, what is delayed, and how to inspect or override,
/// and binds those to the canonical action, visible state, and override posture
/// so it can never contradict the status, diagnostics, or support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceDisclosure {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Surface token this disclosure describes.
    pub surface_token: String,
    /// Title-cased surface label.
    pub surface_label: String,
    /// Owner label rendered next to the disclosure.
    pub owner_label: String,
    /// Canonical owner-subsystem token, for vocabulary traceability.
    pub owner_subsystem_token: String,
    /// Workload-family token whose budget governs this surface's freshness.
    pub governing_subsystem_token: String,
    /// Active efficiency-state token.
    pub active_state: String,
    /// Source-of-change tokens that drove the active state.
    pub source_of_change: Vec<String>,
    /// Budget-action token applied to the governing work.
    pub action: String,
    /// Visible-capability-state token after the action.
    pub visible_state: String,
    /// Freshness class for rendering a stale/slow badge instead of an error.
    pub freshness_class: String,
    /// Always true: behavior materially changed, so a disclosure is warranted.
    pub behavior_changed: bool,
    /// Always true: a degraded disclosure is never an error state.
    pub is_degraded_not_error: bool,
    /// Headline sentence naming the surface, its state, and the cause.
    pub headline: String,
    /// What still works now (a protected path the adaptation never narrows).
    pub still_works_now: String,
    /// What is delayed (the specific reduced freshness or assist).
    pub what_is_delayed: String,
    /// How to inspect the full efficiency state.
    pub inspect: DisclosureInspectHint,
    /// Whether and how the user may override the reduction.
    pub override_affordance: DisclosureOverrideAffordance,
    /// Where and how the disclosure is shown.
    pub placement: DisclosurePlacement,
}

impl SurfaceDisclosure {
    /// Builds a disclosure for one surface under the active state, or `None` when
    /// the surface's behavior did not materially change (so no banner is shown).
    fn for_surface(
        surface: DisclosureSurface,
        state: EfficiencyState,
        sources: &[EfficiencyPressureSource],
        posture: OverridePosture,
        observed_at: &str,
    ) -> Option<Self> {
        let source = *sources
            .first()
            .unwrap_or(&EfficiencyPressureSource::AcPower);
        let decision = WorkloadBudgetDecision::for_state(
            surface.governing_family(),
            state,
            source,
            observed_at,
        );
        if !decision.changed_behavior() {
            return None;
        }
        Some(Self {
            record_kind: SURFACE_DISCLOSURE_RECORD_KIND.to_owned(),
            surface_token: surface.as_str().to_owned(),
            surface_label: surface.label().to_owned(),
            owner_label: surface.owner_label().to_owned(),
            owner_subsystem_token: source_subsystem_token(surface.owner_subsystem()).to_owned(),
            governing_subsystem_token: surface.governing_family().as_str().to_owned(),
            active_state: state.as_str().to_owned(),
            source_of_change: sources.iter().map(|s| s.as_str().to_owned()).collect(),
            action: decision.action.clone(),
            visible_state: decision.capability_row.visible_state.clone(),
            freshness_class: freshness_class_for(&decision.action).to_owned(),
            behavior_changed: true,
            is_degraded_not_error: true,
            headline: headline_for(surface, &decision.action, source),
            still_works_now: surface.still_works_now().to_owned(),
            what_is_delayed: surface.what_is_delayed().to_owned(),
            inspect: DisclosureInspectHint::default(),
            override_affordance: DisclosureOverrideAffordance::for_surface(surface, posture),
            placement: DisclosurePlacement::for_surface(surface),
        })
    }
}

/// The full set of per-surface disclosures for one workspace's active posture.
///
/// Project it from the canonical [`EfficiencyStateSnapshot`] with
/// [`from_snapshot`](Self::from_snapshot) so the disclosures, the status pill,
/// the diagnostics row, and the support export all derive from one object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencySurfaceDisclosures {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Active workspace id.
    pub workspace_id: String,
    /// Active efficiency-state token.
    pub active_state: String,
    /// Source-of-change tokens that drove the active state.
    pub source_of_change: Vec<String>,
    /// True when any surface materially changed behavior.
    pub behavior_changed: bool,
    /// Aggregate override-posture token for the adaptation.
    pub override_posture: String,
    /// Recovery-state token for the adaptation.
    pub recovery_state: String,
    /// One disclosure per materially-changed surface.
    pub disclosures: Vec<SurfaceDisclosure>,
    /// Tokens of surfaces still running within budget, which show no banner.
    pub unaffected_surface_tokens: Vec<String>,
    /// Protected interactions the adaptation may not narrow.
    pub protected_interactions_preserved: Vec<String>,
    /// True when save durability and user-owned artifacts stay preserved.
    pub durability_preserved: bool,
    /// Matrix-bound governance projection for vocabulary traceability.
    pub governance: EfficiencyGovernanceProjection,
    /// Open-details command id shared by every disclosure.
    pub inspect_command_id: String,
    /// Surface ref the open-details command opens.
    pub opens_surface_ref: String,
    /// Support-export packet id that quotes the same posture.
    pub support_export_ref: String,
    /// Observation timestamp.
    pub observed_at: String,
}

impl EfficiencySurfaceDisclosures {
    /// Builds the disclosure set for a typed posture.
    pub fn for_state(
        workspace_id: &str,
        state: EfficiencyState,
        sources: &[EfficiencyPressureSource],
        hidden_surface_count: usize,
        observed_at: &str,
    ) -> Self {
        let posture = derive_override_posture(state, sources);
        let recovery = derive_recovery_state(state);
        let mut disclosures = Vec::new();
        let mut unaffected_surface_tokens = Vec::new();
        for surface in DisclosureSurface::ALL {
            match SurfaceDisclosure::for_surface(surface, state, sources, posture, observed_at) {
                Some(disclosure) => disclosures.push(disclosure),
                None => unaffected_surface_tokens.push(surface.as_str().to_owned()),
            }
        }
        let source_tokens = sources
            .iter()
            .map(|source| source.as_str().to_owned())
            .collect::<Vec<_>>();
        let governance = EfficiencyGovernanceProjection {
            matrix_ref: M5_EFFICIENCY_GOVERNANCE_MATRIX_REF.to_owned(),
            schema_ref: M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF.to_owned(),
            active_state: state.as_str().to_owned(),
            source_of_change: source_tokens.clone(),
            hidden_pane_behaviors: hidden_pane_behaviors_for(hidden_surface_count)
                .iter()
                .map(|behavior| behavior.as_str().to_owned())
                .collect(),
            override_posture: posture.as_str().to_owned(),
            recovery_state: recovery.as_str().to_owned(),
        };
        Self {
            record_kind: EFFICIENCY_SURFACE_DISCLOSURES_RECORD_KIND.to_owned(),
            schema_version: EFFICIENCY_SURFACE_DISCLOSURES_SCHEMA_VERSION,
            workspace_id: workspace_id.to_owned(),
            active_state: state.as_str().to_owned(),
            source_of_change: source_tokens,
            behavior_changed: !disclosures.is_empty(),
            override_posture: posture.as_str().to_owned(),
            recovery_state: recovery.as_str().to_owned(),
            disclosures,
            unaffected_surface_tokens,
            protected_interactions_preserved: protected_interactions(),
            durability_preserved: true,
            governance,
            inspect_command_id: EFFICIENCY_INSPECT_COMMAND_ID.to_owned(),
            opens_surface_ref: EFFICIENCY_DETAILS_SURFACE_REF.to_owned(),
            support_export_ref: support_export_id(workspace_id, state),
            observed_at: observed_at.to_owned(),
        }
    }

    /// Projects the canonical snapshot into the per-surface disclosure set.
    ///
    /// The snapshot already tokenizes the active state and its causes; this
    /// re-derives the typed inputs and reuses [`for_state`](Self::for_state) so
    /// the disclosures share the snapshot's workspace, state, source, override
    /// posture, recovery state, and governance binding.
    pub fn from_snapshot(snapshot: &EfficiencyStateSnapshot) -> Self {
        let state = EfficiencyState::from_token(&snapshot.active_state).unwrap_or_default();
        let sources = snapshot
            .pressure_sources
            .iter()
            .filter_map(|token| EfficiencyPressureSource::from_token(token))
            .collect::<Vec<_>>();
        Self::for_state(
            &snapshot.workspace_id,
            state,
            &sources,
            snapshot.hidden_pane_audit.hidden_surface_count,
            &snapshot.observed_at,
        )
    }

    /// True when at least one surface materially changed behavior.
    pub fn has_disclosures(&self) -> bool {
        !self.disclosures.is_empty()
    }

    /// Returns the disclosure for a surface token, if one was emitted.
    pub fn disclosure_for(&self, surface_token: &str) -> Option<&SurfaceDisclosure> {
        self.disclosures
            .iter()
            .find(|disclosure| disclosure.surface_token == surface_token)
    }

    /// True when every disclosure keeps a protected path explicitly working,
    /// stays a degraded (not error) state, and is durable inline truth. This is
    /// the invariant the protected-path and toast-only acceptance criteria need.
    pub fn preserves_protected_path_truth(&self) -> bool {
        self.disclosures.iter().all(|disclosure| {
            disclosure.is_degraded_not_error
                && !disclosure.still_works_now.is_empty()
                && disclosure.placement.is_durable_inline_truth()
        })
    }
}

/// Builds the canonical support-export id for a workspace's active state, so the
/// disclosure set points at the same packet the diagnostics and support surfaces
/// quote. Kept in lockstep with the support-export id minted by
/// [`crate::efficiency::surfaces`].
fn support_export_id(workspace_id: &str, state: EfficiencyState) -> String {
    format!(
        "support.export.efficiency.{}.{}",
        workspace_id,
        state.as_str()
    )
}

/// The hidden-pane behaviours a hidden surface adopted, derived from how many
/// hidden surfaces the snapshot audited. Mirrors the parent surfaces' mapping so
/// the governance projection agrees across surfaces.
fn hidden_pane_behaviors_for(hidden_surface_count: usize) -> Vec<HiddenPaneBehavior> {
    if hidden_surface_count == 0 {
        return Vec::new();
    }
    vec![
        HiddenPaneBehavior::RenderSuppressed,
        HiddenPaneBehavior::AnimationSuppressed,
        HiddenPaneBehavior::PollingPaused,
    ]
}

/// Maps a budget-action token to the freshness class a surface renders, so a
/// reduced surface shows a stale/slow badge instead of looking broken.
fn freshness_class_for(action: &str) -> &'static str {
    match action {
        "throttle" => "reduced_cadence",
        "defer" => "deferred",
        "pause" => "paused",
        "deny" => "paused",
        "staged_resume" => "resuming",
        _ => "current",
    }
}

/// Builds the disclosure headline from the surface, its action, and the cause,
/// reusing the canonical action vocabulary so it agrees with the capability row.
fn headline_for(
    surface: DisclosureSurface,
    action: &str,
    source: EfficiencyPressureSource,
) -> String {
    match action {
        "throttle" => format!(
            "{}: reduced rate while {} is active.",
            surface.label(),
            source.label()
        ),
        "defer" => format!(
            "{}: deferred while {} is active.",
            surface.label(),
            source.label()
        ),
        "pause" => format!(
            "{}: paused while {} is active.",
            surface.label(),
            source.label()
        ),
        "deny" => format!(
            "{}: paused to protect core work while {} is active.",
            surface.label(),
            source.label()
        ),
        "staged_resume" => format!(
            "{}: resuming in stages after pressure cleared.",
            surface.label()
        ),
        _ => format!("{}: running within the current budget.", surface.label()),
    }
}

/// Stable owner-subsystem token for a [`SourceSubsystem`]. Kept private to the
/// disclosures so the surface owner is recorded with the same token vocabulary
/// the rest of the efficiency contract uses.
fn source_subsystem_token(source: SourceSubsystem) -> &'static str {
    match source {
        SourceSubsystem::Indexer => "indexer",
        SourceSubsystem::AiApply => "ai_apply",
        SourceSubsystem::ReviewAndDiff => "review_and_diff",
        SourceSubsystem::DocsHelpServiceHealth => "docs_help_service_health",
        SourceSubsystem::ExtensionHost => "extension_host",
        SourceSubsystem::SyncMirror => "sync_mirror",
        _ => "shell",
    }
}

/// One seeded disclosure scenario: the typed inputs that drive it together with
/// the disclosure set they produce. Backs the dump example, the checked-in
/// fixtures, and the round-trip test so the disclosures never drift from code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyDisclosureCase {
    /// Stable scenario id.
    pub case_id: String,
    /// Active workspace id.
    pub workspace_id: String,
    /// Active efficiency state.
    pub active_state: EfficiencyState,
    /// Source-of-change pressure sources.
    pub source_of_change: Vec<EfficiencyPressureSource>,
    /// Number of hidden surfaces the posture audited.
    pub hidden_surface_count: usize,
    /// Observation timestamp.
    pub observed_at: String,
    /// The disclosure set the inputs produce.
    pub disclosures: EfficiencySurfaceDisclosures,
}

/// Builds a deterministic disclosure case for a seeded posture.
pub fn seed_efficiency_disclosure_case(
    case_id: &str,
    workspace_id: &str,
    state: EfficiencyState,
    sources: &[EfficiencyPressureSource],
    hidden_surface_count: usize,
    observed_at: &str,
) -> EfficiencyDisclosureCase {
    let disclosures = EfficiencySurfaceDisclosures::for_state(
        workspace_id,
        state,
        sources,
        hidden_surface_count,
        observed_at,
    );
    EfficiencyDisclosureCase {
        case_id: case_id.to_owned(),
        workspace_id: workspace_id.to_owned(),
        active_state: state,
        source_of_change: sources.to_vec(),
        hidden_surface_count,
        observed_at: observed_at.to_owned(),
        disclosures,
    }
}

/// The representative disclosure scenarios. They mirror the postures the parent
/// efficiency-state surfaces seed — OS battery saver, thermal pressure, a
/// policy-imposed cap, a critical-battery protect-core posture, and staged
/// recovery — so the disclosures align with the canonical snapshots that the
/// status, diagnostics, and support surfaces project.
pub fn seeded_efficiency_disclosure_cases() -> Vec<EfficiencyDisclosureCase> {
    use EfficiencyPressureSource as Source;
    use EfficiencyState as State;
    vec![
        seed_efficiency_disclosure_case(
            "battery-saver",
            "ws:battery-saver",
            State::EfficiencyAware,
            &[Source::OsBatterySaver],
            1,
            "2026-06-20T14:01:00Z",
        ),
        seed_efficiency_disclosure_case(
            "thermal",
            "ws:efficiency-demo",
            State::ThermalConstrained,
            &[Source::ThermalPressure],
            2,
            "2026-06-20T14:00:00Z",
        ),
        seed_efficiency_disclosure_case(
            "policy-cap",
            "ws:policy-cap",
            State::EfficiencyAware,
            &[Source::PolicyCap],
            0,
            "2026-06-20T14:02:00Z",
        ),
        seed_efficiency_disclosure_case(
            "critical-battery",
            "ws:critical-battery",
            State::ProtectCore,
            &[Source::CriticalBattery],
            1,
            "2026-06-20T14:03:00Z",
        ),
        seed_efficiency_disclosure_case(
            "recovery",
            "ws:recovery",
            State::Recovery,
            &[Source::PressureCleared],
            0,
            "2026-06-20T14:04:00Z",
        ),
    ]
}
