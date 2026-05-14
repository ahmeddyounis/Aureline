//! Status-bar state items for target, profile, trust, encoding, and
//! background work.
//!
//! The status bar is the durable, persistent surface for ambient operational
//! truth. Toasts, banners, and OS notifications are transient. Logs are not
//! visible without effort. The status bar is the row a user can glance at to
//! confirm "what target am I executing on, which profile is bound, is this
//! workspace trusted, what encoding/line endings does this file have, and is
//! there background work in flight?"
//!
//! ## Truth sources
//!
//! Every item is a thin projection over upstream truth — never a private
//! cache:
//!
//! - **Target** — projected from
//!   [`aureline_runtime::ExecutionContext::target_identity`] or, when the
//!   active surface is the bottom-panel terminal, from the canonical PTY
//!   session header.
//! - **Profile** — projected from
//!   [`crate::chrome::title_context_bar::ProfileIdentity`] which already
//!   resolves deployment profile, identity mode, and profile mode.
//! - **Trust** — projected from [`aureline_workspace::TrustState`] which the
//!   workspace lifecycle machine settles before partial readiness.
//! - **Encoding** — projected from
//!   [`aureline_workspace::save::SourceFidelityRecord`] (encoding, BOM, and
//!   newline mode together — line endings are part of the same fidelity
//!   tuple).
//! - **Background state** — projected from a background-work counter and an
//!   optional degraded token, so a single ambient row collapses many job
//!   tickers without drowning the bar in flickering counters.
//!
//! ## Failure-drill posture
//!
//! Putting one upstream truth source into a degraded state — for example,
//! a workspace flipping to `Restricted`, an encoding that decoded as
//! `unknown_binary_like`, or background work whose owner went `Offline` —
//! must surface a degraded label on the matching item rather than a stale
//! success state. The fixture suite under
//! `/fixtures/ux/status_bar_cases/*.json` exercises the drill against the
//! same projection the live shell renders.

use serde::{Deserialize, Serialize};

use aureline_workspace::save::{
    DetectedEncoding, FinalNewlineDetected, NewlineModeDetected, SourceFidelityRecord,
};
use aureline_workspace::TrustState as WorkspaceTrustState;

use crate::efficiency::EfficiencyStatusSnapshot;
use crate::state_cards::DegradedStateToken;

/// Stable record-kind tag carried in serialized status-bar snapshots.
pub const STATUS_BAR_SNAPSHOT_RECORD_KIND: &str = "status_bar_snapshot_record";
/// Schema version for the [`StatusBarSnapshot`] payload shape.
pub const STATUS_BAR_SNAPSHOT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried in serialized status-bar item rows.
pub const STATUS_BAR_ITEM_RECORD_KIND: &str = "status_bar_item_record";
/// Schema version for the [`StatusBarItemRecord`] payload shape.
pub const STATUS_BAR_ITEM_SCHEMA_VERSION: u32 = 1;

/// Stable item kinds rendered on the protected M1 status-bar seed.
///
/// The seed renders one row per kind when the upstream truth is active; items
/// are not duplicated and extension contributions are out of scope for the
/// seed. Their priority and stable slot keys mirror the contract frozen at
/// `docs/ux/status_bar_contract.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusBarItemKind {
    /// Current execution target (host class + reachability).
    Target,
    /// Active profile, deployment profile, and identity mode.
    Profile,
    /// Workspace trust posture.
    Trust,
    /// File source-fidelity tuple (encoding + BOM + newline mode + final newline).
    Encoding,
    /// Aggregated background-work summary.
    BackgroundState,
    /// Active efficiency state when power or thermal pressure changed behavior.
    EfficiencyState,
}

impl StatusBarItemKind {
    /// Stable status-item id used by the contract and surfaced to support exports.
    pub const fn status_item_id(self) -> &'static str {
        match self {
            Self::Target => "status.item.target.execution",
            Self::Profile => "status.item.profile.deployment",
            Self::Trust => "status.item.trust.workspace",
            Self::Encoding => "status.item.encoding.file",
            Self::BackgroundState => "status.item.background.work_summary",
            Self::EfficiencyState => "status.item.efficiency.state",
        }
    }

    /// Stable slot key from the status-bar contract.
    pub const fn stable_slot_key(self) -> &'static str {
        match self {
            Self::Target => "status.slot.context.execution",
            Self::Profile => "status.slot.context.workspace",
            Self::Trust => "status.slot.context.workspace",
            Self::Encoding => "status.slot.metadata.file",
            Self::BackgroundState => "status.slot.work.summary",
            Self::EfficiencyState => "status.slot.efficiency.state",
        }
    }

    /// Default item class. Items can be promoted to `RecoveryCritical` when
    /// the projected truth becomes consequence-bearing.
    pub const fn default_item_class(self) -> StatusItemClass {
        match self {
            Self::Target => StatusItemClass::ActiveContextTruth,
            Self::Profile => StatusItemClass::ActiveContextTruth,
            Self::Trust => StatusItemClass::ActiveContextTruth,
            Self::Encoding => StatusItemClass::AmbientMetadata,
            Self::BackgroundState => StatusItemClass::OngoingWork,
            Self::EfficiencyState => StatusItemClass::OngoingWork,
        }
    }

    /// Static label rendered next to the current value.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Target => "Target",
            Self::Profile => "Profile",
            Self::Trust => "Trust",
            Self::Encoding => "Encoding",
            Self::BackgroundState => "Work",
            Self::EfficiencyState => "Power",
        }
    }

    /// Primary command id wired to the item activation.
    ///
    /// Each id opens the narrowest useful inspector or settings detail per
    /// the action contract in `docs/ux/status_bar_contract.md`.
    pub const fn primary_command_id(self) -> &'static str {
        match self {
            Self::Target => "cmd:runtime.execution_context.inspect",
            Self::Profile => "cmd:settings.profile.inspect",
            Self::Trust => "cmd:workspace.trust.review",
            Self::Encoding => "cmd:editor.source_fidelity.inspect",
            Self::BackgroundState => "cmd:activity.work_summary.open",
            Self::EfficiencyState => "cmd:runtime.efficiency_state.inspect",
        }
    }

    /// Surface ref opened by the primary activation.
    pub const fn opens_surface_ref(self) -> &'static str {
        match self {
            Self::Target => "inspector.runtime.execution_context",
            Self::Profile => "inspector.settings.profile",
            Self::Trust => "inspector.workspace.trust_review",
            Self::Encoding => "inspector.editor.source_fidelity",
            Self::BackgroundState => "surface.activity_center.work_summary",
            Self::EfficiencyState => "surface.runtime.efficiency_state",
        }
    }
}

/// Class ordering used by the status-bar contract priority ladder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusItemClass {
    /// State that can block, lose, narrow, or misrepresent core work.
    RecoveryCritical,
    /// Current execution, trust, host, branch, profile, or route truth that
    /// changes command meaning.
    ActiveContextTruth,
    /// Long-running or repeated work that should remain inspectable without
    /// toast spam.
    OngoingWork,
    /// Low-risk environment facts useful for glanceable inspection.
    AmbientMetadata,
}

impl StatusItemClass {
    /// Lower bound (inclusive) of the contract priority band.
    pub const fn priority_rank_floor(self) -> u32 {
        match self {
            Self::RecoveryCritical => 0,
            Self::ActiveContextTruth => 100,
            Self::OngoingWork => 200,
            Self::AmbientMetadata => 300,
        }
    }

    /// Stable token used in serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecoveryCritical => "recovery_critical",
            Self::ActiveContextTruth => "active_context_truth",
            Self::OngoingWork => "ongoing_work",
            Self::AmbientMetadata => "ambient_metadata",
        }
    }
}

/// Compact target-truth slice the status bar renders. The caller derives this
/// from `aureline_runtime::ExecutionContext` (or the canonical PTY session
/// header for terminal-focused frames). The status bar never re-derives a
/// target identity of its own.
#[derive(Debug, Clone)]
pub struct TargetSnapshot<'a> {
    /// Stable host-class token (`local_host`, `ssh_remote`, `managed_workspace`,
    /// ...). Mirrors `ExecutionContext::target_identity::target_class`.
    pub target_class_token: &'a str,
    /// Display label rendered as the current value.
    pub target_label: &'a str,
    /// Stable reachability token (`reachable`, `warming`, `degraded`,
    /// `unreachable`, `policy_blocked`).
    pub reachability_token: &'a str,
    /// Canonical execution-context id when one has been minted.
    pub execution_context_ref: Option<&'a str>,
    /// True when the upstream record reports degraded fields. The caller
    /// owns this boolean so the bar does not have to inspect every field.
    pub has_degraded_field: bool,
}

/// Compact profile-truth slice the status bar renders.
#[derive(Debug, Clone)]
pub struct ProfileSnapshot<'a> {
    /// Profile display label (e.g. "Standard", "Safe mode").
    pub profile_label: &'a str,
    /// Stable profile-mode token (`standard`, `safe_mode`,
    /// `temporary_session`, `imported_profile`, `managed_policy_profile`,
    /// `support_recovery`).
    pub profile_mode_token: &'a str,
    /// Stable deployment-profile token (`individual_local`, `self_hosted`,
    /// `enterprise_online`, `air_gapped`, `managed_cloud`).
    pub deployment_profile_token: &'a str,
    /// Stable identity-mode token (`account_free_local`, `self_hosted_org`,
    /// `managed_workspace`).
    pub identity_mode_token: &'a str,
}

/// Compact background-state slice the status bar renders. The seed
/// aggregates many owners into one ambient row so the bar does not become a
/// per-job ticker.
#[derive(Debug, Clone)]
pub struct BackgroundStateSnapshot<'a> {
    /// Owners contributing visible work in the active workspace (e.g.
    /// `indexer`, `tests`, `sync`). The status bar renders a single
    /// summary; the surface list is preserved for the overflow menu.
    pub active_owners: &'a [&'a str],
    /// Optional degraded posture for the aggregate (for example `Offline`
    /// when sync transports are unreachable).
    pub aggregate_degraded: Option<DegradedStateToken>,
    /// Caller-owned monotonically-non-decreasing observation timestamp,
    /// used as `last_observed_at` so support exports can quote the same
    /// reading the live shell rendered.
    pub observed_at: &'a str,
}

/// Compact encoding-truth slice. Mirrors `SourceFidelityRecord` but is
/// `Option`-able so the bar can render an explicit "no file open" row
/// instead of inventing a fake default.
#[derive(Debug, Clone, Copy)]
pub struct EncodingSnapshot<'a> {
    /// Source-fidelity record borrowed from the active editor. `None` means
    /// no file is currently open in the active editor.
    pub source_fidelity: Option<&'a SourceFidelityRecord>,
}

/// Inputs the caller assembles from the upstream truth sources. The status
/// bar projection is a pure function of these inputs.
#[derive(Debug, Clone)]
pub struct StatusBarInputs<'a> {
    /// Active workspace identifier; carried verbatim in the snapshot so
    /// support exports can group rows by workspace.
    pub workspace_id: &'a str,
    /// Workspace trust posture from the canonical lifecycle machine.
    pub workspace_trust_state: WorkspaceTrustState,
    /// Target slice projected from the active execution-context record.
    pub target: TargetSnapshot<'a>,
    /// Profile slice projected from the title/context bar identity tuple.
    pub profile: ProfileSnapshot<'a>,
    /// Encoding slice projected from the active editor's source-fidelity record.
    pub encoding: EncodingSnapshot<'a>,
    /// Background-state slice aggregated from durable job rows / activity center.
    pub background: BackgroundStateSnapshot<'a>,
    /// Efficiency-state projection when power or thermal pressure changed behavior.
    pub efficiency: Option<EfficiencyStatusSnapshot>,
}

/// One projected status-bar row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarItemRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub status_item_id: String,
    pub item_kind: StatusBarItemKind,
    pub item_class: StatusItemClass,
    pub priority_rank: u32,
    pub stable_slot_key: String,
    pub label: String,
    pub current_value_label: String,
    pub explanation: String,
    pub primary_command_id: String,
    pub opens_surface_ref: String,
    pub keyboard_target_id: String,
    /// Degraded-state chip the chrome renders next to the row. `None` when
    /// the upstream truth source is not in a degraded posture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_token: Option<String>,
    /// True when the projected truth has been promoted to recovery-critical
    /// (e.g. trust restricted, encoding decode failure).
    pub is_recovery_critical: bool,
    /// Stable export-safe truth-source ref a support packet can quote.
    pub truth_source_ref: String,
}

/// One snapshot of the status bar.
///
/// Ordering: items are sorted by `(priority_rank ascending, kind
/// declaration order)` so a recovery-critical promotion always lands ahead
/// of ambient metadata, and the bar order is deterministic across frames.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarSnapshot {
    pub record_kind: String,
    pub schema_version: u32,
    pub workspace_id: String,
    pub items: Vec<StatusBarItemRecord>,
    pub observed_at: String,
}

impl StatusBarSnapshot {
    /// Project a status-bar snapshot from upstream truth.
    pub fn project(inputs: &StatusBarInputs<'_>) -> Self {
        let mut items = vec![
            project_target(&inputs.target),
            project_profile(&inputs.profile),
            project_trust(inputs.workspace_trust_state),
            project_encoding(&inputs.encoding),
            project_background(&inputs.background),
        ];
        if let Some(efficiency) = inputs.efficiency.as_ref() {
            items.push(project_efficiency(efficiency));
        }
        items.sort_by_key(|item| item.priority_rank);
        Self {
            record_kind: STATUS_BAR_SNAPSHOT_RECORD_KIND.to_owned(),
            schema_version: STATUS_BAR_SNAPSHOT_SCHEMA_VERSION,
            workspace_id: inputs.workspace_id.to_owned(),
            items,
            observed_at: inputs.background.observed_at.to_owned(),
        }
    }

    /// Find one row by item kind. Useful for chrome that highlights a
    /// specific row in response to a focus event.
    pub fn item(&self, kind: StatusBarItemKind) -> Option<&StatusBarItemRecord> {
        self.items.iter().find(|item| item.item_kind == kind)
    }

    /// True when at least one row is recovery-critical. The chrome uses this
    /// to keep the recovery slot reserved before any ambient row is allowed
    /// to claim space.
    pub fn has_recovery_critical(&self) -> bool {
        self.items.iter().any(|item| item.is_recovery_critical)
    }

    /// True when at least one row carries a degraded chip. A support export
    /// uses this signal to flag the snapshot for human review.
    pub fn has_degraded_state(&self) -> bool {
        self.items.iter().any(|item| item.degraded_token.is_some())
    }
}

fn build_item(
    kind: StatusBarItemKind,
    class: StatusItemClass,
    rank_offset: u32,
    current_value_label: String,
    explanation: String,
    degraded: Option<DegradedStateToken>,
    keyboard_target_id: String,
    truth_source_ref: &str,
) -> StatusBarItemRecord {
    let priority_rank = class.priority_rank_floor() + rank_offset;
    StatusBarItemRecord {
        record_kind: STATUS_BAR_ITEM_RECORD_KIND.to_owned(),
        schema_version: STATUS_BAR_ITEM_SCHEMA_VERSION,
        status_item_id: kind.status_item_id().to_owned(),
        item_kind: kind,
        item_class: class,
        priority_rank,
        stable_slot_key: kind.stable_slot_key().to_owned(),
        label: kind.label().to_owned(),
        current_value_label,
        explanation,
        primary_command_id: kind.primary_command_id().to_owned(),
        opens_surface_ref: kind.opens_surface_ref().to_owned(),
        keyboard_target_id,
        degraded_token: degraded.map(|d| d.token().to_owned()),
        is_recovery_critical: matches!(class, StatusItemClass::RecoveryCritical),
        truth_source_ref: truth_source_ref.to_owned(),
    }
}

fn project_target(target: &TargetSnapshot<'_>) -> StatusBarItemRecord {
    let degraded = match target.reachability_token {
        "reachable" => {
            if target.has_degraded_field {
                Some(DegradedStateToken::Limited)
            } else {
                None
            }
        }
        "warming" => Some(DegradedStateToken::Warming),
        "degraded" => Some(DegradedStateToken::Limited),
        "unreachable" => Some(DegradedStateToken::Offline),
        "policy_blocked" => Some(DegradedStateToken::PolicyBlocked),
        _ => Some(DegradedStateToken::Limited),
    };
    let class = if matches!(target.reachability_token, "unreachable" | "policy_blocked") {
        StatusItemClass::RecoveryCritical
    } else {
        StatusBarItemKind::Target.default_item_class()
    };
    let mut explanation = format!("Target class: {}.", target.target_class_token);
    if let Some(ec_ref) = target.execution_context_ref {
        explanation.push_str(&format!(" Execution context: {ec_ref}."));
    }
    if let Some(token) = degraded {
        explanation.push_str(&format!(" {}", token.default_description()));
    }
    build_item(
        StatusBarItemKind::Target,
        class,
        20,
        target.target_label.to_owned(),
        explanation,
        degraded,
        "status_bar_item_target".to_owned(),
        "execution_context.target_identity",
    )
}

fn project_profile(profile: &ProfileSnapshot<'_>) -> StatusBarItemRecord {
    let degraded = match profile.profile_mode_token {
        "safe_mode" => Some(DegradedStateToken::Limited),
        "support_recovery" => Some(DegradedStateToken::Limited),
        "temporary_session" => Some(DegradedStateToken::Experimental),
        _ => None,
    };
    let class = if profile.profile_mode_token == "safe_mode" {
        StatusItemClass::RecoveryCritical
    } else {
        StatusBarItemKind::Profile.default_item_class()
    };
    let explanation = format!(
        "Deployment: {}. Identity: {}. Profile mode: {}.",
        profile.deployment_profile_token, profile.identity_mode_token, profile.profile_mode_token,
    );
    build_item(
        StatusBarItemKind::Profile,
        class,
        40,
        profile.profile_label.to_owned(),
        explanation,
        degraded,
        "status_bar_item_profile".to_owned(),
        "title_context_bar.profile_identity",
    )
}

fn project_trust(trust: WorkspaceTrustState) -> StatusBarItemRecord {
    let (current_value, degraded, class, explanation) = match trust {
        WorkspaceTrustState::Trusted => (
            "Trusted",
            None,
            StatusItemClass::ActiveContextTruth,
            "Workspace is trusted for ordinary editing and execution.",
        ),
        WorkspaceTrustState::Restricted => (
            "Restricted",
            Some(DegradedStateToken::PolicyBlocked),
            StatusItemClass::RecoveryCritical,
            "Workspace is restricted; commands that mutate or execute are gated until trust is granted.",
        ),
        WorkspaceTrustState::PendingEvaluation => (
            "Trust evaluating",
            Some(DegradedStateToken::Warming),
            StatusItemClass::ActiveContextTruth,
            "Trust is being evaluated; mutations and execution are gated until the lifecycle settles.",
        ),
    };
    build_item(
        StatusBarItemKind::Trust,
        class,
        10,
        current_value.to_owned(),
        explanation.to_owned(),
        degraded,
        "status_bar_item_trust".to_owned(),
        "workspace.trust_state",
    )
}

fn project_encoding(encoding: &EncodingSnapshot<'_>) -> StatusBarItemRecord {
    let Some(record) = encoding.source_fidelity else {
        return build_item(
            StatusBarItemKind::Encoding,
            StatusItemClass::AmbientMetadata,
            60,
            "No file open".to_owned(),
            "No editor buffer is bound; encoding will appear when a file is opened.".to_owned(),
            None,
            "status_bar_item_encoding".to_owned(),
            "editor.source_fidelity_record",
        );
    };

    let encoding_label = encoding_display_label(record.detected_encoding);
    let newline_label = newline_display_label(record.newline_mode_detected);
    let final_newline_suffix = match record.final_newline_detected {
        FinalNewlineDetected::Present => "",
        FinalNewlineDetected::Absent => " · no final newline",
        FinalNewlineDetected::UnknownOrDegraded => " · final newline unknown",
    };
    let current_value = format!("{encoding_label} · {newline_label}{final_newline_suffix}");

    let (class, degraded) = classify_encoding(record);
    let explanation = format!(
        "Encoding: {}. Newline: {}. Final newline: {}. BOM: {}.",
        record.detected_encoding.as_str(),
        record.newline_mode_detected.as_str(),
        record.final_newline_detected.as_str(),
        record.bom_state_detected.as_str(),
    );
    build_item(
        StatusBarItemKind::Encoding,
        class,
        60,
        current_value,
        explanation,
        degraded,
        "status_bar_item_encoding".to_owned(),
        "editor.source_fidelity_record",
    )
}

fn project_background(background: &BackgroundStateSnapshot<'_>) -> StatusBarItemRecord {
    let active = background.active_owners.len();
    let degraded = background.aggregate_degraded;
    let (class, current_value) = match (active, degraded) {
        (0, None) => (StatusItemClass::AmbientMetadata, "Idle".to_owned()),
        (0, Some(token)) => (
            StatusItemClass::OngoingWork,
            format!("Idle · {}", token.label()),
        ),
        (n, None) => (StatusItemClass::OngoingWork, format!("{n} running")),
        (n, Some(token)) => (
            StatusItemClass::OngoingWork,
            format!("{n} running · {}", token.label()),
        ),
    };
    let explanation = if background.active_owners.is_empty() {
        "No background work is in flight.".to_owned()
    } else {
        format!(
            "Background work owners: {}.",
            background.active_owners.join(", ")
        )
    };
    build_item(
        StatusBarItemKind::BackgroundState,
        class,
        20,
        current_value,
        explanation,
        degraded,
        "status_bar_item_background".to_owned(),
        "activity_center.work_summary",
    )
}

fn project_efficiency(efficiency: &EfficiencyStatusSnapshot) -> StatusBarItemRecord {
    let degraded = efficiency
        .degraded_token
        .as_deref()
        .and_then(degraded_token_from_str);
    let class = if efficiency.is_recovery_critical {
        StatusItemClass::RecoveryCritical
    } else {
        StatusBarItemKind::EfficiencyState.default_item_class()
    };
    build_item(
        StatusBarItemKind::EfficiencyState,
        class,
        30,
        efficiency.current_value_label.clone(),
        efficiency.explanation.clone(),
        degraded,
        "status_bar_item_efficiency".to_owned(),
        "runtime.efficiency_state",
    )
}

fn degraded_token_from_str(token: &str) -> Option<DegradedStateToken> {
    match token {
        "Warming" => Some(DegradedStateToken::Warming),
        "Cached" => Some(DegradedStateToken::Cached),
        "Partial" => Some(DegradedStateToken::Partial),
        "Stale" => Some(DegradedStateToken::Stale),
        "Offline" => Some(DegradedStateToken::Offline),
        "PolicyBlocked" => Some(DegradedStateToken::PolicyBlocked),
        "Limited" => Some(DegradedStateToken::Limited),
        "Unsupported" => Some(DegradedStateToken::Unsupported),
        "Labs" => Some(DegradedStateToken::Labs),
        "Experimental" => Some(DegradedStateToken::Experimental),
        "RetestPending" => Some(DegradedStateToken::RetestPending),
        _ => None,
    }
}

fn classify_encoding(
    record: &SourceFidelityRecord,
) -> (StatusItemClass, Option<DegradedStateToken>) {
    if matches!(
        record.detected_encoding,
        DetectedEncoding::UnknownBinaryLike
    ) {
        return (
            StatusItemClass::RecoveryCritical,
            Some(DegradedStateToken::PolicyBlocked),
        );
    }
    if matches!(record.newline_mode_detected, NewlineModeDetected::Mixed) {
        return (
            StatusItemClass::ActiveContextTruth,
            Some(DegradedStateToken::Partial),
        );
    }
    if matches!(
        record.newline_mode_detected,
        NewlineModeDetected::UnknownOrDegraded
    ) || matches!(
        record.final_newline_detected,
        FinalNewlineDetected::UnknownOrDegraded
    ) {
        return (
            StatusItemClass::ActiveContextTruth,
            Some(DegradedStateToken::Stale),
        );
    }
    (StatusBarItemKind::Encoding.default_item_class(), None)
}

fn encoding_display_label(encoding: DetectedEncoding) -> &'static str {
    match encoding {
        DetectedEncoding::Utf8 => "UTF-8",
        DetectedEncoding::Utf8Bom => "UTF-8 (BOM)",
        DetectedEncoding::Utf16LeBom => "UTF-16 LE",
        DetectedEncoding::Utf16BeBom => "UTF-16 BE",
        DetectedEncoding::Utf32LeBom => "UTF-32 LE",
        DetectedEncoding::Utf32BeBom => "UTF-32 BE",
        DetectedEncoding::UnknownBinaryLike => "Binary-like",
    }
}

fn newline_display_label(newline: NewlineModeDetected) -> &'static str {
    match newline {
        NewlineModeDetected::Lf => "LF",
        NewlineModeDetected::Crlf => "CRLF",
        NewlineModeDetected::CrOnly => "CR",
        NewlineModeDetected::Mixed => "Mixed",
        NewlineModeDetected::UnknownOrDegraded => "Newline unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use aureline_workspace::save::{
        BomStateDetected, DetectedEncoding, DetectionSource, ExecutableIntent,
        FinalNewlineDetected, NewlineModeDetected, SourceFidelityRecord,
    };

    fn fidelity_utf8_lf() -> SourceFidelityRecord {
        SourceFidelityRecord {
            detected_encoding: DetectedEncoding::Utf8,
            detection_source: DetectionSource::Utf8Heuristic,
            bom_state_detected: BomStateDetected::Absent,
            newline_mode_detected: NewlineModeDetected::Lf,
            final_newline_detected: FinalNewlineDetected::Present,
            executable_intent: ExecutableIntent::NonExecutable,
        }
    }

    fn nominal_inputs<'a>(
        fidelity: Option<&'a SourceFidelityRecord>,
        owners: &'a [&'a str],
    ) -> StatusBarInputs<'a> {
        StatusBarInputs {
            workspace_id: "ws-test",
            workspace_trust_state: WorkspaceTrustState::Trusted,
            target: TargetSnapshot {
                target_class_token: "local_host",
                target_label: "Local",
                reachability_token: "reachable",
                execution_context_ref: Some("execution_context.local_desktop.workspace_root"),
                has_degraded_field: false,
            },
            profile: ProfileSnapshot {
                profile_label: "Standard",
                profile_mode_token: "standard",
                deployment_profile_token: "individual_local",
                identity_mode_token: "account_free_local",
            },
            encoding: EncodingSnapshot {
                source_fidelity: fidelity,
            },
            background: BackgroundStateSnapshot {
                active_owners: owners,
                aggregate_degraded: None,
                observed_at: "2026-05-10T12:00:00Z",
            },
            efficiency: None,
        }
    }

    #[test]
    fn protected_walk_renders_truthful_active_row_per_kind() {
        // Protected walk: open a workspace and a terminal; status items for
        // target, profile, trust, encoding, and background state must be
        // synchronized with the upstream truth without any degraded chip.
        let fidelity = fidelity_utf8_lf();
        let inputs = nominal_inputs(Some(&fidelity), &[]);

        let snapshot = StatusBarSnapshot::project(&inputs);
        assert_eq!(snapshot.items.len(), 5);
        assert!(!snapshot.has_recovery_critical());
        assert!(!snapshot.has_degraded_state());

        let trust = snapshot.item(StatusBarItemKind::Trust).expect("trust row");
        assert_eq!(trust.current_value_label, "Trusted");
        assert!(trust.degraded_token.is_none());

        let target = snapshot
            .item(StatusBarItemKind::Target)
            .expect("target row");
        assert_eq!(target.current_value_label, "Local");
        assert_eq!(target.stable_slot_key, "status.slot.context.execution");

        let profile = snapshot
            .item(StatusBarItemKind::Profile)
            .expect("profile row");
        assert_eq!(profile.current_value_label, "Standard");
        assert!(profile.explanation.contains("individual_local"));

        let encoding = snapshot
            .item(StatusBarItemKind::Encoding)
            .expect("encoding row");
        assert_eq!(encoding.current_value_label, "UTF-8 · LF");
        assert!(encoding.degraded_token.is_none());

        let background = snapshot
            .item(StatusBarItemKind::BackgroundState)
            .expect("background row");
        assert_eq!(background.current_value_label, "Idle");
    }

    #[test]
    fn snapshot_orders_items_by_priority_rank_with_recovery_critical_first() {
        let fidelity = fidelity_utf8_lf();
        let inputs = StatusBarInputs {
            workspace_trust_state: WorkspaceTrustState::Restricted,
            ..nominal_inputs(Some(&fidelity), &[])
        };
        let snapshot = StatusBarSnapshot::project(&inputs);
        assert_eq!(snapshot.items[0].item_kind, StatusBarItemKind::Trust);
        assert!(snapshot.items[0].is_recovery_critical);
        // Subsequent rows must be in non-decreasing priority order.
        for window in snapshot.items.windows(2) {
            assert!(window[0].priority_rank <= window[1].priority_rank);
        }
    }

    #[test]
    fn failure_drill_restricted_trust_promotes_recovery_critical_label() {
        // Failure drill: flip the upstream trust source to Restricted. The
        // status row must surface the degraded label and recovery-critical
        // promotion instead of a stale "Trusted" success state.
        let fidelity = fidelity_utf8_lf();
        let inputs = StatusBarInputs {
            workspace_trust_state: WorkspaceTrustState::Restricted,
            ..nominal_inputs(Some(&fidelity), &[])
        };
        let snapshot = StatusBarSnapshot::project(&inputs);
        assert!(snapshot.has_recovery_critical());
        assert!(snapshot.has_degraded_state());

        let trust = snapshot.item(StatusBarItemKind::Trust).expect("trust row");
        assert_eq!(trust.current_value_label, "Restricted");
        assert!(trust.is_recovery_critical);
        assert_eq!(trust.degraded_token.as_deref(), Some("PolicyBlocked"));
        assert_eq!(trust.priority_rank, 10);
        assert_eq!(trust.stable_slot_key, "status.slot.context.workspace");
    }

    #[test]
    fn unreachable_target_promotes_recovery_critical_offline_chip() {
        let fidelity = fidelity_utf8_lf();
        let mut inputs = nominal_inputs(Some(&fidelity), &[]);
        inputs.target = TargetSnapshot {
            target_class_token: "ssh_remote",
            target_label: "Remote (build-vm-01)",
            reachability_token: "unreachable",
            execution_context_ref: Some("execution_context.ssh_remote.build_vm_01"),
            has_degraded_field: true,
        };
        let snapshot = StatusBarSnapshot::project(&inputs);
        let target = snapshot
            .item(StatusBarItemKind::Target)
            .expect("target row");
        assert!(target.is_recovery_critical);
        assert_eq!(target.degraded_token.as_deref(), Some("Offline"));
        assert!(target.explanation.contains("ssh_remote"));
    }

    #[test]
    fn binary_like_encoding_promotes_recovery_critical_chip() {
        let record = SourceFidelityRecord {
            detected_encoding: DetectedEncoding::UnknownBinaryLike,
            detection_source: DetectionSource::DecodeFailedNoChoice,
            bom_state_detected: BomStateDetected::UnknownOrDegraded,
            newline_mode_detected: NewlineModeDetected::UnknownOrDegraded,
            final_newline_detected: FinalNewlineDetected::UnknownOrDegraded,
            executable_intent: ExecutableIntent::UnknownOrDegraded,
        };
        let inputs = nominal_inputs(Some(&record), &[]);
        let snapshot = StatusBarSnapshot::project(&inputs);
        let encoding = snapshot
            .item(StatusBarItemKind::Encoding)
            .expect("encoding row");
        assert!(encoding.is_recovery_critical);
        assert_eq!(encoding.degraded_token.as_deref(), Some("PolicyBlocked"));
        assert!(encoding.current_value_label.starts_with("Binary-like"));
    }

    #[test]
    fn mixed_newline_mode_marks_partial_without_recovery_critical() {
        let record = SourceFidelityRecord {
            detected_encoding: DetectedEncoding::Utf8,
            detection_source: DetectionSource::Utf8Heuristic,
            bom_state_detected: BomStateDetected::Absent,
            newline_mode_detected: NewlineModeDetected::Mixed,
            final_newline_detected: FinalNewlineDetected::Present,
            executable_intent: ExecutableIntent::NonExecutable,
        };
        let inputs = nominal_inputs(Some(&record), &[]);
        let snapshot = StatusBarSnapshot::project(&inputs);
        let encoding = snapshot
            .item(StatusBarItemKind::Encoding)
            .expect("encoding row");
        assert!(!encoding.is_recovery_critical);
        assert_eq!(encoding.degraded_token.as_deref(), Some("Partial"));
        assert!(encoding.current_value_label.contains("Mixed"));
    }

    #[test]
    fn background_state_aggregates_owners_without_per_owner_rows() {
        let fidelity = fidelity_utf8_lf();
        let owners: &[&str] = &["indexer", "tests", "sync"];
        let inputs = nominal_inputs(Some(&fidelity), owners);
        let snapshot = StatusBarSnapshot::project(&inputs);
        let background = snapshot
            .item(StatusBarItemKind::BackgroundState)
            .expect("background row");
        assert_eq!(background.current_value_label, "3 running");
        assert!(background.explanation.contains("indexer"));
        assert!(background.explanation.contains("sync"));
        // No per-owner rows allowed; the seed renders exactly five items.
        assert_eq!(snapshot.items.len(), 5);
    }

    #[test]
    fn background_state_offline_aggregate_degrades_without_invented_owners() {
        let fidelity = fidelity_utf8_lf();
        let mut inputs = nominal_inputs(Some(&fidelity), &[]);
        inputs.background.aggregate_degraded = Some(DegradedStateToken::Offline);
        let snapshot = StatusBarSnapshot::project(&inputs);
        let background = snapshot
            .item(StatusBarItemKind::BackgroundState)
            .expect("background row");
        assert_eq!(background.current_value_label, "Idle · Offline");
        assert_eq!(background.degraded_token.as_deref(), Some("Offline"));
    }

    #[test]
    fn efficiency_state_renders_when_power_or_thermal_changes_behavior() {
        let fidelity = fidelity_utf8_lf();
        let mut inputs = nominal_inputs(Some(&fidelity), &[]);
        inputs.efficiency = Some(EfficiencyStatusSnapshot {
            record_kind: crate::efficiency::EFFICIENCY_STATUS_RECORD_KIND.to_owned(),
            schema_version: 1,
            active_state: "ThermalConstrained".to_owned(),
            pressure_sources: vec!["thermal_pressure".to_owned()],
            behavior_changed: true,
            affected_capability_count: 3,
            current_value_label: "Thermal constrained · thermal pressure".to_owned(),
            explanation:
                "Thermal constrained changed background work because of thermal pressure; 3 capability rows name what paused or reduced."
                    .to_owned(),
            accessibility_label: "Efficiency state: Thermal constrained".to_owned(),
            primary_command_id: "cmd:runtime.efficiency_state.inspect".to_owned(),
            opens_surface_ref: "surface.runtime.efficiency_state".to_owned(),
            degraded_token: Some("Limited".to_owned()),
            is_recovery_critical: false,
        });
        let snapshot = StatusBarSnapshot::project(&inputs);
        let efficiency = snapshot
            .item(StatusBarItemKind::EfficiencyState)
            .expect("efficiency row");
        assert_eq!(
            efficiency.current_value_label,
            "Thermal constrained · thermal pressure"
        );
        assert_eq!(efficiency.stable_slot_key, "status.slot.efficiency.state");
        assert_eq!(efficiency.degraded_token.as_deref(), Some("Limited"));
    }

    #[test]
    fn encoding_row_renders_no_file_open_when_no_record_is_bound() {
        let inputs = nominal_inputs(None, &[]);
        let snapshot = StatusBarSnapshot::project(&inputs);
        let encoding = snapshot
            .item(StatusBarItemKind::Encoding)
            .expect("encoding row");
        assert_eq!(encoding.current_value_label, "No file open");
        assert!(encoding.degraded_token.is_none());
        assert!(!encoding.is_recovery_critical);
    }

    #[test]
    fn safe_mode_profile_promotes_recovery_critical_with_limited_chip() {
        let fidelity = fidelity_utf8_lf();
        let mut inputs = nominal_inputs(Some(&fidelity), &[]);
        inputs.profile.profile_label = "Safe mode";
        inputs.profile.profile_mode_token = "safe_mode";
        let snapshot = StatusBarSnapshot::project(&inputs);
        let profile = snapshot
            .item(StatusBarItemKind::Profile)
            .expect("profile row");
        assert!(profile.is_recovery_critical);
        assert_eq!(profile.degraded_token.as_deref(), Some("Limited"));
        assert_eq!(profile.current_value_label, "Safe mode");
    }
}
