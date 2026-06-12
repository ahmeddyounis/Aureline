//! M5 rollout inventory for experiments, cohorts, expiry, and kill-switch truth.
//!
//! This packet is the canonical rollout source for the optional M5 command
//! families. It records the declared and effective lifecycle state, rollout
//! ring, cohort, owner, review/expiry date, promotion posture, kill-switch
//! path, and stable-surface disclosure coverage for every affected command
//! family so desktop, CLI, AI, help/About, diagnostics, settings inspectors,
//! release packets, and support exports can all quote the same bounded truth.

use std::error::Error;
use std::fmt;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Stable record-kind tag carried by [`M5RolloutInventoryPacket`].
pub const M5_ROLLOUT_INVENTORY_RECORD_KIND: &str = "m5_rollout_inventory_packet";

/// Schema version for M5 rollout inventory packets.
pub const M5_ROLLOUT_INVENTORY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the M5 rollout inventory schema.
pub const M5_ROLLOUT_INVENTORY_SCHEMA_REF: &str =
    "schemas/commands/m5_rollout_inventory.schema.json";

/// Repo-relative path of the companion doc.
pub const M5_ROLLOUT_INVENTORY_DOC_REF: &str = "docs/commands/m5_rollout_inventory.md";

/// Repo-relative path of the checked fixture directory.
pub const M5_ROLLOUT_INVENTORY_FIXTURE_DIR: &str = "fixtures/commands/m5_rollout_inventory";

/// Repo-relative path of the checked packet artifact.
pub const M5_ROLLOUT_INVENTORY_PACKET_REF: &str =
    "artifacts/commands/m5_rollout_inventory/packet.json";

/// Repo-relative path of the checked support export.
pub const M5_ROLLOUT_INVENTORY_SUPPORT_EXPORT_REF: &str =
    "artifacts/commands/m5_rollout_inventory/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_ROLLOUT_INVENTORY_SUMMARY_REF: &str =
    "artifacts/commands/m5_rollout_inventory/summary.md";

/// Stable packet id used by the seeded export.
pub const M5_ROLLOUT_INVENTORY_PACKET_ID: &str = "m5-rollout-inventory:stable:0001";

/// Stable support-export id used by [`M5RolloutInventorySupportExport`].
pub const M5_ROLLOUT_INVENTORY_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-rollout-inventory:0001";

const GENERATED_AT: &str = "2026-06-12T00:00:00Z";

static SEEDED_PACKET: OnceLock<M5RolloutInventoryPacket> = OnceLock::new();

/// Effective lifecycle state carried by rollout rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RolloutStateClass {
    /// Local opt-in or maintainer-only Labs surface.
    Labs,
    /// Visible preview cohort with expected churn.
    Preview,
    /// Supported beta cohort with bounded support intent.
    Beta,
    /// Stable-facing claim with no hidden flag dependency.
    Stable,
    /// Visible sunset path with migration guidance.
    Deprecated,
    /// Present but blocked by a winning policy or kill switch.
    DisabledByPolicy,
    /// Present but narrowed because proof freshness or review has lapsed.
    RetestPending,
    /// Preserved only as a tombstone or reference row.
    Removed,
}

impl M5RolloutStateClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Labs => "labs",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
            Self::Deprecated => "deprecated",
            Self::DisabledByPolicy => "disabled_by_policy",
            Self::RetestPending => "retest_pending",
            Self::Removed => "removed",
        }
    }

    /// Human-readable label shared by help, diagnostics, and support.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Labs => "Labs",
            Self::Preview => "Preview",
            Self::Beta => "Beta",
            Self::Stable => "Stable",
            Self::Deprecated => "Deprecated",
            Self::DisabledByPolicy => "DisabledByPolicy",
            Self::RetestPending => "RetestPending",
            Self::Removed => "Removed",
        }
    }
}

/// Promotion posture the rollout row is currently in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PromotionStateClass {
    /// Maintainer- or contributor-only Labs opt-in posture.
    LabsOptIn,
    /// Named preview cohort.
    PreviewNamedCohort,
    /// Design-partner or bounded beta cohort.
    BetaDesignPartner,
    /// Broad beta exposure.
    BetaBroad,
    /// Broad stable exposure.
    StableBroad,
    /// Stable row narrowed pending requalification.
    RetestRequired,
    /// Stable or beta row halted by policy or emergency disable.
    BlockedByPolicy,
    /// Sunset path remains visible but no new rollout broadening happens.
    SunsetOnly,
}

impl M5PromotionStateClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LabsOptIn => "labs_opt_in",
            Self::PreviewNamedCohort => "preview_named_cohort",
            Self::BetaDesignPartner => "beta_design_partner",
            Self::BetaBroad => "beta_broad",
            Self::StableBroad => "stable_broad",
            Self::RetestRequired => "retest_required",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::SunsetOnly => "sunset_only",
        }
    }
}

/// Kill-switch source class in precedence order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5KillSwitchSourceClass {
    /// Emergency security or safety response.
    EmergencySecurityResponse,
    /// Admin or enterprise policy ceiling.
    AdminPolicyCeiling,
    /// Release-channel or rollout-plan override.
    ReleaseChannelOrRolloutOverride,
    /// Cohort or ring assignment.
    CohortOrRingAssignment,
    /// User opt-in or local preview toggle.
    UserOptInOrLocalPreviewToggle,
}

impl M5KillSwitchSourceClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmergencySecurityResponse => "emergency_security_response",
            Self::AdminPolicyCeiling => "admin_policy_ceiling",
            Self::ReleaseChannelOrRolloutOverride => "release_channel_or_rollout_override",
            Self::CohortOrRingAssignment => "cohort_or_ring_assignment",
            Self::UserOptInOrLocalPreviewToggle => "user_opt_in_or_local_preview_toggle",
        }
    }
}

/// Stable-facing consumer that must disclose rollout narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RolloutConsumerSurfaceClass {
    /// Settings inspector or effective-settings detail surface.
    SettingsInspector,
    /// Help or About surface.
    HelpAbout,
    /// Diagnostics panel or troubleshooting summary.
    Diagnostics,
    /// Support export or incident packet.
    SupportExport,
    /// Docs/help or release-note publication surface.
    DocsRelease,
}

impl M5RolloutConsumerSurfaceClass {
    /// Required stable-facing coverage for every row.
    pub const ALL: [Self; 5] = [
        Self::SettingsInspector,
        Self::HelpAbout,
        Self::Diagnostics,
        Self::SupportExport,
        Self::DocsRelease,
    ];

    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsInspector => "settings_inspector",
            Self::HelpAbout => "help_about",
            Self::Diagnostics => "diagnostics",
            Self::SupportExport => "support_export",
            Self::DocsRelease => "docs_release",
        }
    }
}

/// One kill-switch or disable-source path on a rollout row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RolloutKillSwitchRecord {
    /// Source-class token in precedence order.
    pub source_class: M5KillSwitchSourceClass,
    /// Stable source ref for support and audit joins.
    pub source_ref: String,
    /// Whether this source currently wins or participates in the active block.
    pub active: bool,
    /// Copy-safe reason the source may narrow or block the row.
    pub reason: String,
    /// Whether user-authored durable state remains preserved when this source fires.
    pub preserve_user_data: bool,
    /// Scope of preserved data while narrowed or blocked.
    pub preserved_data_scope: String,
    /// Recovery or fallback path the user can still take.
    pub fallback_path: String,
}

/// One stable-facing consumer disclosure row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RolloutSurfaceDisclosureRecord {
    /// Consumer surface that must surface the same narrowing truth.
    pub surface_class: M5RolloutConsumerSurfaceClass,
    /// Stable projection or packet ref quoted by that surface.
    pub surface_ref: String,
    /// Whether narrowed or blocked behavior remains visibly disclosed.
    pub behavior_change_visible: bool,
}

/// One command-family rollout inventory row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RolloutInventoryRow {
    /// Stable command id anchoring the family.
    pub command_id: String,
    /// Stable capability id shared across help, docs, and support.
    pub capability_id: String,
    /// Human-facing family label.
    pub display_label: String,
    /// Owning person or team ref.
    pub owner_ref: String,
    /// Rollout ring this row currently rides.
    pub rollout_ring: String,
    /// Named cohort within the ring.
    pub cohort: String,
    /// Review or expiry date in `YYYY-MM-DD` form.
    pub review_or_expiry_date: String,
    /// Declared lifecycle state before rollout narrowing.
    pub declared_state_class: M5RolloutStateClass,
    /// Effective lifecycle state after rollout and freshness rules apply.
    pub effective_state_class: M5RolloutStateClass,
    /// Current promotion posture.
    pub promotion_state: M5PromotionStateClass,
    /// Stable rollout-state ref shared by parity, support, and docs.
    pub rollout_state_ref: String,
    /// Whether the row may still publish stable-facing wording.
    pub stable_claim_allowed: bool,
    /// Whether the row satisfies the explicit no-hidden-flag rule.
    pub no_hidden_flag_rule_satisfied: bool,
    /// Capability families visibly affected by the row.
    pub affected_capability_ids: Vec<String>,
    /// Kill-switch and disable-source paths.
    pub kill_switches: Vec<M5RolloutKillSwitchRecord>,
    /// Stable-facing consumers that must disclose the same narrowing.
    pub surfaced_in: Vec<M5RolloutSurfaceDisclosureRecord>,
    /// Copy-safe explanation of the current rollout posture.
    pub rationale: String,
}

impl M5RolloutInventoryRow {
    /// Returns the active kill-switch rows in precedence order.
    pub fn active_kill_switches(&self) -> Vec<&M5RolloutKillSwitchRecord> {
        let mut rows = self
            .kill_switches
            .iter()
            .filter(|row| row.active)
            .collect::<Vec<_>>();
        rows.sort_by_key(|row| match row.source_class {
            M5KillSwitchSourceClass::EmergencySecurityResponse => 0usize,
            M5KillSwitchSourceClass::AdminPolicyCeiling => 1,
            M5KillSwitchSourceClass::ReleaseChannelOrRolloutOverride => 2,
            M5KillSwitchSourceClass::CohortOrRingAssignment => 3,
            M5KillSwitchSourceClass::UserOptInOrLocalPreviewToggle => 4,
        });
        rows
    }
}

/// Packet summary for help, release, and support consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RolloutInventorySummary {
    /// Number of rollout rows under audit.
    pub row_count: usize,
    /// Number of rows still at Labs.
    pub labs_row_count: usize,
    /// Number of rows still at Preview.
    pub preview_row_count: usize,
    /// Number of rows still at Beta.
    pub beta_row_count: usize,
    /// Number of rows still at Stable.
    pub stable_row_count: usize,
    /// Number of rows narrowed to Deprecated.
    pub deprecated_row_count: usize,
    /// Number of rows blocked by policy or kill switch.
    pub disabled_by_policy_row_count: usize,
    /// Number of rows narrowed to RetestPending.
    pub retest_pending_row_count: usize,
    /// Number of rows preserved only as Removed references.
    pub removed_row_count: usize,
    /// Number of rows with at least one active kill switch.
    pub active_kill_switch_row_count: usize,
    /// Number of rows barred from stable wording.
    pub narrowed_row_count: usize,
    /// Number of rows that still satisfy stable wording.
    pub stable_claim_allowed_row_count: usize,
    /// Number of rows satisfying the no-hidden-flag rule.
    pub no_hidden_flag_row_count: usize,
}

/// Canonical M5 rollout inventory packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RolloutInventoryPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Schema ref for this packet.
    pub schema_ref: String,
    /// Companion doc ref.
    pub doc_ref: String,
    /// Ordered rollout rows.
    pub rows: Vec<M5RolloutInventoryRow>,
    /// Roll-up counts.
    pub summary: M5RolloutInventorySummary,
}

impl M5RolloutInventoryPacket {
    /// Renders a compact Markdown summary for checked artifacts.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Rollout Inventory\n\n");
        out.push_str("| Metric | Value |\n|---|---:|\n");
        out.push_str(&format!("| Rows | {} |\n", self.summary.row_count));
        out.push_str(&format!("| Labs rows | {} |\n", self.summary.labs_row_count));
        out.push_str(&format!(
            "| Preview rows | {} |\n",
            self.summary.preview_row_count
        ));
        out.push_str(&format!("| Beta rows | {} |\n", self.summary.beta_row_count));
        out.push_str(&format!(
            "| Stable rows | {} |\n",
            self.summary.stable_row_count
        ));
        out.push_str(&format!(
            "| Policy-blocked rows | {} |\n",
            self.summary.disabled_by_policy_row_count
        ));
        out.push_str(&format!(
            "| RetestPending rows | {} |\n",
            self.summary.retest_pending_row_count
        ));
        out.push_str(&format!(
            "| Active kill-switch rows | {} |\n",
            self.summary.active_kill_switch_row_count
        ));
        out.push_str(&format!(
            "| Narrowed rows | {} |\n",
            self.summary.narrowed_row_count
        ));
        out.push_str(&format!(
            "| No-hidden-flag rows | {} |\n\n",
            self.summary.no_hidden_flag_row_count
        ));
        out.push_str("| Command | Effective state | Ring | Cohort | Promotion | Owner | Active kill switch |\n");
        out.push_str("|---|---|---|---|---|---|---|\n");
        for row in &self.rows {
            let active = row
                .active_kill_switches()
                .first()
                .map(|kill| kill.source_class.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                row.command_id,
                row.effective_state_class.display_label(),
                row.rollout_ring,
                row.cohort,
                row.promotion_state.as_str(),
                row.owner_ref,
                active
            ));
        }
        out.push('\n');
        out
    }
}

/// Support-export wrapper for the M5 rollout inventory packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RolloutInventorySupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet schema ref.
    pub schema_ref: String,
    /// Case ids useful for support joins.
    pub case_ids: Vec<String>,
    /// Quoted rollout packet.
    pub packet: M5RolloutInventoryPacket,
}

impl M5RolloutInventorySupportExport {
    /// Builds a deterministic support-export wrapper from a packet.
    pub fn from_packet(support_export_id: String, packet: M5RolloutInventoryPacket) -> Self {
        let mut case_ids = vec![packet.packet_id.clone()];
        for row in &packet.rows {
            case_ids.push(row.command_id.clone());
            case_ids.push(row.capability_id.clone());
            case_ids.push(row.owner_ref.clone());
            case_ids.push(row.rollout_state_ref.clone());
            case_ids.extend(row.affected_capability_ids.iter().cloned());
            for source in &row.kill_switches {
                case_ids.push(source.source_ref.clone());
            }
            for surface in &row.surfaced_in {
                case_ids.push(surface.surface_ref.clone());
            }
        }
        case_ids.sort();
        case_ids.dedup();
        Self {
            record_kind: "m5_rollout_inventory_support_export".to_string(),
            schema_version: 1,
            support_export_id,
            schema_ref: M5_ROLLOUT_INVENTORY_SCHEMA_REF.to_string(),
            case_ids,
            packet,
        }
    }
}

/// Validation error raised by [`validate_m5_rollout_inventory_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5RolloutInventoryValidationError {
    /// The packet has no rows.
    NoRows,
    /// A row is missing required metadata.
    MissingMetadata {
        /// Command id that regressed.
        command_id: String,
        /// Field name that regressed.
        field_name: String,
    },
    /// A row stopped exposing a kill-switch path.
    NoKillSwitchPath {
        /// Command id that regressed.
        command_id: String,
    },
    /// A row stopped naming an affected capability family.
    NoAffectedCapabilities {
        /// Command id that regressed.
        command_id: String,
    },
    /// A row stopped covering a required stable-facing surface.
    MissingSurfaceDisclosure {
        /// Command id that regressed.
        command_id: String,
        /// Surface that regressed.
        surface_class: String,
    },
    /// A stable claim still depended on hidden flag state.
    HiddenFlagRuleFailed {
        /// Command id that regressed.
        command_id: String,
    },
    /// A row claimed stability while its effective state was narrower.
    StableClaimMismatch {
        /// Command id that regressed.
        command_id: String,
        /// Effective state that blocked the claim.
        effective_state: String,
    },
}

impl fmt::Display for M5RolloutInventoryValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoRows => write!(f, "m5 rollout inventory packet has no rows"),
            Self::MissingMetadata {
                command_id,
                field_name,
            } => write!(f, "command {command_id} is missing rollout metadata field {field_name}"),
            Self::NoKillSwitchPath { command_id } => {
                write!(f, "command {command_id} has no kill-switch path")
            }
            Self::NoAffectedCapabilities { command_id } => {
                write!(f, "command {command_id} names no affected capability families")
            }
            Self::MissingSurfaceDisclosure {
                command_id,
                surface_class,
            } => write!(
                f,
                "command {command_id} is missing rollout disclosure for {surface_class}"
            ),
            Self::HiddenFlagRuleFailed { command_id } => write!(
                f,
                "command {command_id} still depends on hidden rollout or flag state"
            ),
            Self::StableClaimMismatch {
                command_id,
                effective_state,
            } => write!(
                f,
                "command {command_id} claims stable wording while effective state is {effective_state}"
            ),
        }
    }
}

impl Error for M5RolloutInventoryValidationError {}

fn surfaced_in_refs(canonical_verb: &str) -> Vec<M5RolloutSurfaceDisclosureRecord> {
    M5RolloutConsumerSurfaceClass::ALL
        .into_iter()
        .map(|surface_class| M5RolloutSurfaceDisclosureRecord {
            surface_class,
            surface_ref: format!("m5-rollout:{}:{}", canonical_verb, surface_class.as_str()),
            behavior_change_visible: true,
        })
        .collect()
}

fn kill_switch(
    source_class: M5KillSwitchSourceClass,
    source_ref: &str,
    active: bool,
    reason: &str,
    preserved_data_scope: &str,
    fallback_path: &str,
) -> M5RolloutKillSwitchRecord {
    M5RolloutKillSwitchRecord {
        source_class,
        source_ref: source_ref.to_string(),
        active,
        reason: reason.to_string(),
        preserve_user_data: true,
        preserved_data_scope: preserved_data_scope.to_string(),
        fallback_path: fallback_path.to_string(),
    }
}

fn row(
    command_id: &str,
    capability_id: &str,
    display_label: &str,
    owner_ref: &str,
    rollout_ring: &str,
    cohort: &str,
    review_or_expiry_date: &str,
    declared_state_class: M5RolloutStateClass,
    effective_state_class: M5RolloutStateClass,
    promotion_state: M5PromotionStateClass,
    stable_claim_allowed: bool,
    affected_capability_ids: &[&str],
    kill_switches: Vec<M5RolloutKillSwitchRecord>,
    rationale: &str,
) -> M5RolloutInventoryRow {
    let canonical_verb = command_id.trim_start_matches("cmd:");
    M5RolloutInventoryRow {
        command_id: command_id.to_string(),
        capability_id: capability_id.to_string(),
        display_label: display_label.to_string(),
        owner_ref: owner_ref.to_string(),
        rollout_ring: rollout_ring.to_string(),
        cohort: cohort.to_string(),
        review_or_expiry_date: review_or_expiry_date.to_string(),
        declared_state_class,
        effective_state_class,
        promotion_state,
        rollout_state_ref: format!("rollout:{canonical_verb}:{}", effective_state_class.as_str()),
        stable_claim_allowed,
        no_hidden_flag_rule_satisfied: true,
        affected_capability_ids: affected_capability_ids
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        kill_switches,
        surfaced_in: surfaced_in_refs(canonical_verb),
        rationale: rationale.to_string(),
    }
}

fn seeded_rows() -> Vec<M5RolloutInventoryRow> {
    vec![
        row(
            "cmd:notebook.run_all_cells",
            "m5.notebook.execution",
            "Notebook execution and result lineage",
            "@ahmeddyounis",
            "design_partner_beta",
            "notebook_beta_seed",
            "2026-07-15",
            M5RolloutStateClass::Beta,
            M5RolloutStateClass::Beta,
            M5PromotionStateClass::BetaDesignPartner,
            false,
            &["m5.notebook.execution", "m5.notebook.support_export"],
            vec![kill_switch(
                M5KillSwitchSourceClass::ReleaseChannelOrRolloutOverride,
                "policy:m5.notebook.execution.pause",
                false,
                "Release owner may pause notebook execution promotion while kernel lineage evidence is refreshed.",
                "Notebook files and recorded outputs remain available for inspection and export.",
                "Reopen notebooks locally and export the last durable execution packet.",
            )],
            "Notebook execution is beta-only for the current M5 cohort and must keep lifecycle markers on support and docs surfaces.",
        ),
        row(
            "cmd:data_api.send_request",
            "m5.data.request_handoff",
            "Data request, result, and handoff surfaces",
            "@ahmeddyounis",
            "design_partner_beta",
            "data_api_beta_seed",
            "2026-07-18",
            M5RolloutStateClass::Beta,
            M5RolloutStateClass::Beta,
            M5PromotionStateClass::BetaDesignPartner,
            false,
            &["m5.data.request_handoff", "m5.data.support_export"],
            vec![kill_switch(
                M5KillSwitchSourceClass::ReleaseChannelOrRolloutOverride,
                "policy:m5.data.request_handoff.pause",
                false,
                "Release owner may pause broadening while database handoff evidence is refreshed.",
                "Captured requests, result packets, and local notebooks remain available.",
                "Continue with local query inspection and export the result handoff packet.",
            )],
            "Data request surfaces stay beta while notebook, chart, and AI handoff evidence remains cohort-bound.",
        ),
        row(
            "cmd:profiler.start_capture",
            "m5.profiler.capture",
            "Profiler capture and export lineage",
            "@ahmeddyounis",
            "beta_broad",
            "desktop_dogfood_beta",
            "2026-07-20",
            M5RolloutStateClass::Beta,
            M5RolloutStateClass::Beta,
            M5PromotionStateClass::BetaBroad,
            false,
            &["m5.profiler.capture", "m5.profiler.support_export"],
            vec![kill_switch(
                M5KillSwitchSourceClass::ReleaseChannelOrRolloutOverride,
                "policy:m5.profiler.capture.pause",
                false,
                "Release owner may pause profiler capture broadening if trace export evidence drifts.",
                "Saved traces and local profiles remain available for review.",
                "Use local performance inspection and export the last trace artifact manually.",
            )],
            "Profiler capture is broadly beta but still requires visible beta posture and export lineage.",
        ),
        row(
            "cmd:trace_replay.replay_session",
            "m5.trace_replay.session",
            "Trace replay and chronology follow-up",
            "@ahmeddyounis",
            "public_preview",
            "trace_replay_preview_ring",
            "2026-06-27",
            M5RolloutStateClass::Preview,
            M5RolloutStateClass::Preview,
            M5PromotionStateClass::PreviewNamedCohort,
            false,
            &["m5.trace_replay.session", "m5.trace_replay.support_export"],
            vec![kill_switch(
                M5KillSwitchSourceClass::CohortOrRingAssignment,
                "rollout:m5.trace_replay.preview_ring",
                true,
                "Trace replay remains in the named preview ring until replay support-class evidence clears.",
                "Recorded chronology artifacts remain available for read-only inspection.",
                "Review exported chronology packets locally or hand off to desktop replay.",
            )],
            "Trace replay remains preview-only and browser/companion consumers must disclose the desktop handoff path instead of widening authority.",
        ),
        row(
            "cmd:docs_browser.open_external",
            "m5.docs.external_browser_handoff",
            "Docs browser external handoff",
            "@ahmeddyounis",
            "beta_broad",
            "docs_browser_beta",
            "2026-06-09",
            M5RolloutStateClass::Beta,
            M5RolloutStateClass::RetestPending,
            M5PromotionStateClass::RetestRequired,
            false,
            &["m5.docs.external_browser_handoff", "m5.docs.release_copy"],
            vec![kill_switch(
                M5KillSwitchSourceClass::ReleaseChannelOrRolloutOverride,
                "policy:m5.docs.external_handoff.retest",
                false,
                "The release owner may freeze broadening while docs-browser proof freshness is stale.",
                "Local docs packs and cached release notes remain available.",
                "Use local docs packs or export the docs-browser continuity packet.",
            )],
            "Docs-browser external handoff is temporarily narrowed to RetestPending because the freshness proof for claimed help/docs behavior has lapsed.",
        ),
        row(
            "cmd:template_scaffold.scaffold_project",
            "m5.scaffold.project_creation",
            "Template scaffold planner and creation flow",
            "@ahmeddyounis",
            "beta_broad",
            "template_scaffold_beta",
            "2026-07-10",
            M5RolloutStateClass::Beta,
            M5RolloutStateClass::Beta,
            M5PromotionStateClass::BetaBroad,
            false,
            &["m5.scaffold.project_creation", "m5.scaffold.support_export"],
            vec![kill_switch(
                M5KillSwitchSourceClass::ReleaseChannelOrRolloutOverride,
                "policy:m5.scaffold.project_creation.pause",
                false,
                "Scaffold broadening may pause while framework-pack certification catches up.",
                "Generated previews, rollback handles, and local review sheets remain available.",
                "Use preview-only scaffold planning and defer apply until the row requalifies.",
            )],
            "Scaffold creation is beta and must keep preview, rollback, and generation-diff disclosure visible.",
        ),
        row(
            "cmd:review_pipeline.run_pipeline",
            "m5.review.pipeline_actions",
            "Review pipeline actions and reruns",
            "@ahmeddyounis",
            "beta_broad",
            "review_pipeline_beta",
            "2026-07-12",
            M5RolloutStateClass::Beta,
            M5RolloutStateClass::Beta,
            M5PromotionStateClass::BetaBroad,
            false,
            &["m5.review.pipeline_actions", "m5.review.support_export"],
            vec![kill_switch(
                M5KillSwitchSourceClass::ReleaseChannelOrRolloutOverride,
                "policy:m5.review.pipeline_actions.pause",
                false,
                "Pipeline actions may pause while provider or CI compatibility evidence is refreshed.",
                "Review artifacts, logs, and rerun receipts remain available.",
                "Open the review workspace locally and export the current pipeline packet.",
            )],
            "Pipeline actions are beta-only and must keep provider, preview, and rerun truth visible across review surfaces.",
        ),
        row(
            "cmd:preview.open_live_preview",
            "m5.preview.live_runtime",
            "Live preview runtime",
            "@ahmeddyounis",
            "labs_local",
            "preview_runtime_opt_in",
            "2026-06-28",
            M5RolloutStateClass::Labs,
            M5RolloutStateClass::Labs,
            M5PromotionStateClass::LabsOptIn,
            false,
            &["m5.preview.live_runtime", "m5.preview.support_export"],
            vec![kill_switch(
                M5KillSwitchSourceClass::UserOptInOrLocalPreviewToggle,
                "setting:m5.preview.live_runtime.enabled",
                true,
                "Live preview remains a local Labs opt-in surface and may not publish stable wording.",
                "Local project files and preview state snapshots remain stored.",
                "Use generated preview diagnostics and static render packets without opening live preview.",
            )],
            "Live preview is Labs-only and every stable-facing consumer must say so instead of inheriting generic preview copy.",
        ),
        row(
            "cmd:companion.handoff_session",
            "m5.companion.session_handoff",
            "Companion session handoff",
            "@ahmeddyounis",
            "beta_broad",
            "companion_beta",
            "2026-07-14",
            M5RolloutStateClass::Beta,
            M5RolloutStateClass::Beta,
            M5PromotionStateClass::BetaBroad,
            false,
            &["m5.companion.session_handoff", "m5.companion.support_export"],
            vec![kill_switch(
                M5KillSwitchSourceClass::ReleaseChannelOrRolloutOverride,
                "policy:m5.companion.session_handoff.pause",
                false,
                "Companion-session broadening may pause while continuity and redaction proof catches up.",
                "Desktop context and local drafts remain available.",
                "Continue on desktop and export the companion handoff packet.",
            )],
            "Companion handoff is beta and must keep bounded scope, redaction, and offline continuity visible.",
        ),
        row(
            "cmd:incident.open_incident",
            "m5.incident.workspace",
            "Incident workspace and evidence slices",
            "@ahmeddyounis",
            "beta_broad",
            "incident_beta",
            "2026-07-08",
            M5RolloutStateClass::Beta,
            M5RolloutStateClass::Beta,
            M5PromotionStateClass::BetaBroad,
            false,
            &["m5.incident.workspace", "m5.incident.support_export"],
            vec![kill_switch(
                M5KillSwitchSourceClass::ReleaseChannelOrRolloutOverride,
                "policy:m5.incident.workspace.pause",
                false,
                "Incident workspace rollout may pause while evidence-slice exports are refreshed.",
                "Captured incident packets and local evidence remain available.",
                "Use local incident packets and export the bounded support bundle.",
            )],
            "Incident workspace surfaces stay beta until the evidence and offboarding paths complete their current proof window.",
        ),
        row(
            "cmd:sync.push_workspace_state",
            "m5.sync.workspace_push",
            "Managed workspace-state push",
            "@ahmeddyounis",
            "managed_beta",
            "managed_sync_out_of_scope",
            "2026-06-24",
            M5RolloutStateClass::Beta,
            M5RolloutStateClass::DisabledByPolicy,
            M5PromotionStateClass::BlockedByPolicy,
            false,
            &["m5.sync.workspace_push", "m5.sync.support_export"],
            vec![
                kill_switch(
                    M5KillSwitchSourceClass::UserOptInOrLocalPreviewToggle,
                    "setting:m5.sync.workspace_push.requested",
                    true,
                    "A local opt-in request exists but cannot widen the current managed rollout boundary.",
                    "Local workspace state remains intact and exportable.",
                    "Keep local state and export the sync review packet without pushing managed state.",
                ),
                kill_switch(
                    M5KillSwitchSourceClass::AdminPolicyCeiling,
                    "policy:m5.sync.workspace_push.managed_ceiling",
                    true,
                    "The managed policy ceiling blocks workspace-state push for the current cohort.",
                    "Local workspace state and conflict review packets remain available.",
                    "Keep local continuity or review managed sync state from the desktop inspector.",
                ),
            ],
            "Workspace-state push is policy-blocked for the current managed cohort and every route must deny with the same rollout explanation.",
        ),
        row(
            "cmd:offboarding.export_and_wipe",
            "m5.offboarding.export_and_wipe",
            "Offboarding export and wipe flow",
            "@ahmeddyounis",
            "beta_broad",
            "offboarding_beta",
            "2026-07-22",
            M5RolloutStateClass::Beta,
            M5RolloutStateClass::Beta,
            M5PromotionStateClass::BetaBroad,
            false,
            &["m5.offboarding.export_and_wipe", "m5.offboarding.support_export"],
            vec![kill_switch(
                M5KillSwitchSourceClass::ReleaseChannelOrRolloutOverride,
                "policy:m5.offboarding.export_and_wipe.pause",
                false,
                "Offboarding broadening may pause while delete proof and export coverage is refreshed.",
                "User-authored exports and rollback checkpoints remain available.",
                "Use export-only mode and defer wipe until approval and rollback posture requalify.",
            )],
            "Offboarding remains beta because deletion, export, and rollback truth must stay visibly coupled on every surface.",
        ),
        row(
            "cmd:secret_broker.open_credential_review",
            "m5.secret_broker.review",
            "Secret broker credential review",
            "@ahmeddyounis",
            "beta_broad",
            "secret_broker_beta",
            "2026-07-16",
            M5RolloutStateClass::Beta,
            M5RolloutStateClass::Beta,
            M5PromotionStateClass::BetaBroad,
            false,
            &["m5.secret_broker.review", "m5.secret_broker.support_export"],
            vec![kill_switch(
                M5KillSwitchSourceClass::ReleaseChannelOrRolloutOverride,
                "policy:m5.secret_broker.review.pause",
                false,
                "Credential-review broadening may pause while secret-boundary evidence is refreshed.",
                "Handle metadata remains available; secret bodies remain excluded.",
                "Inspect credential metadata and export the redacted secret-boundary packet.",
            )],
            "Credential review is beta and must keep approval, redaction, and managed-boundary truth visible.",
        ),
        row(
            "cmd:secret_broker.open_credential_rotation",
            "m5.secret_broker.rotation",
            "Secret broker credential rotation",
            "@ahmeddyounis",
            "beta_broad",
            "secret_broker_beta",
            "2026-07-16",
            M5RolloutStateClass::Beta,
            M5RolloutStateClass::Beta,
            M5PromotionStateClass::BetaBroad,
            false,
            &["m5.secret_broker.rotation", "m5.secret_broker.support_export"],
            vec![kill_switch(
                M5KillSwitchSourceClass::ReleaseChannelOrRolloutOverride,
                "policy:m5.secret_broker.rotation.pause",
                false,
                "Credential-rotation broadening may pause while approval and rollback proof is refreshed.",
                "Credential handles and review metadata remain available.",
                "Inspect rotation requirements locally and export the redacted rotation packet.",
            )],
            "Credential rotation remains beta and approval-gated; automation and stable wording may not outrun the rollout row.",
        ),
        row(
            "cmd:infrastructure.reconcile_workspace",
            "m5.infrastructure.reconcile_workspace",
            "Infrastructure reconcile workspace flow",
            "@ahmeddyounis",
            "beta_broad",
            "infra_reconcile_beta",
            "2026-07-19",
            M5RolloutStateClass::Beta,
            M5RolloutStateClass::Beta,
            M5PromotionStateClass::BetaBroad,
            false,
            &["m5.infrastructure.reconcile_workspace", "m5.infrastructure.support_export"],
            vec![kill_switch(
                M5KillSwitchSourceClass::ReleaseChannelOrRolloutOverride,
                "policy:m5.infrastructure.reconcile_workspace.pause",
                false,
                "Infrastructure reconcile broadening may pause while preview and authority proof is refreshed.",
                "Plans, dry runs, and local diagnostics remain available.",
                "Use preview-only reconcile plans and export the infrastructure review packet.",
            )],
            "Infrastructure reconcile stays beta and must preserve preview, authority, and rollout honesty on every route.",
        ),
    ]
}

fn build_summary(rows: &[M5RolloutInventoryRow]) -> M5RolloutInventorySummary {
    M5RolloutInventorySummary {
        row_count: rows.len(),
        labs_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5RolloutStateClass::Labs)
            .count(),
        preview_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5RolloutStateClass::Preview)
            .count(),
        beta_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5RolloutStateClass::Beta)
            .count(),
        stable_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5RolloutStateClass::Stable)
            .count(),
        deprecated_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5RolloutStateClass::Deprecated)
            .count(),
        disabled_by_policy_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5RolloutStateClass::DisabledByPolicy)
            .count(),
        retest_pending_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5RolloutStateClass::RetestPending)
            .count(),
        removed_row_count: rows
            .iter()
            .filter(|row| row.effective_state_class == M5RolloutStateClass::Removed)
            .count(),
        active_kill_switch_row_count: rows
            .iter()
            .filter(|row| !row.active_kill_switches().is_empty())
            .count(),
        narrowed_row_count: rows.iter().filter(|row| !row.stable_claim_allowed).count(),
        stable_claim_allowed_row_count: rows.iter().filter(|row| row.stable_claim_allowed).count(),
        no_hidden_flag_row_count: rows
            .iter()
            .filter(|row| row.no_hidden_flag_rule_satisfied)
            .count(),
    }
}

/// Builds the seeded M5 rollout inventory packet.
pub fn seeded_m5_rollout_inventory_packet() -> M5RolloutInventoryPacket {
    SEEDED_PACKET
        .get_or_init(|| {
            let rows = seeded_rows();
            M5RolloutInventoryPacket {
                record_kind: M5_ROLLOUT_INVENTORY_RECORD_KIND.to_string(),
                schema_version: M5_ROLLOUT_INVENTORY_SCHEMA_VERSION,
                packet_id: M5_ROLLOUT_INVENTORY_PACKET_ID.to_string(),
                generated_at: GENERATED_AT.to_string(),
                schema_ref: M5_ROLLOUT_INVENTORY_SCHEMA_REF.to_string(),
                doc_ref: M5_ROLLOUT_INVENTORY_DOC_REF.to_string(),
                summary: build_summary(&rows),
                rows,
            }
        })
        .clone()
}

/// Returns one rollout row for the named command id, when present.
pub fn rollout_inventory_row(command_id: &str) -> Option<M5RolloutInventoryRow> {
    seeded_m5_rollout_inventory_packet()
        .rows
        .into_iter()
        .find(|row| row.command_id == command_id)
}

/// Returns the current seeded packet after validating it.
pub fn current_m5_rollout_inventory_export(
) -> Result<M5RolloutInventoryPacket, Vec<M5RolloutInventoryValidationError>> {
    let packet = seeded_m5_rollout_inventory_packet();
    validate_m5_rollout_inventory_packet(&packet)?;
    Ok(packet)
}

fn valid_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(idx, byte)| idx == 4 || idx == 7 || byte.is_ascii_digit())
}

/// Validates the canonical M5 rollout inventory packet.
pub fn validate_m5_rollout_inventory_packet(
    packet: &M5RolloutInventoryPacket,
) -> Result<(), Vec<M5RolloutInventoryValidationError>> {
    let mut errors = Vec::new();
    if packet.rows.is_empty() {
        errors.push(M5RolloutInventoryValidationError::NoRows);
    }

    for row in &packet.rows {
        for (field_name, value) in [
            ("command_id", row.command_id.as_str()),
            ("capability_id", row.capability_id.as_str()),
            ("display_label", row.display_label.as_str()),
            ("owner_ref", row.owner_ref.as_str()),
            ("rollout_ring", row.rollout_ring.as_str()),
            ("cohort", row.cohort.as_str()),
            ("rollout_state_ref", row.rollout_state_ref.as_str()),
            ("rationale", row.rationale.as_str()),
        ] {
            if value.trim().is_empty() {
                errors.push(M5RolloutInventoryValidationError::MissingMetadata {
                    command_id: row.command_id.clone(),
                    field_name: field_name.to_string(),
                });
            }
        }
        if !valid_date(&row.review_or_expiry_date) {
            errors.push(M5RolloutInventoryValidationError::MissingMetadata {
                command_id: row.command_id.clone(),
                field_name: "review_or_expiry_date".to_string(),
            });
        }
        if row.kill_switches.is_empty() {
            errors.push(M5RolloutInventoryValidationError::NoKillSwitchPath {
                command_id: row.command_id.clone(),
            });
        }
        if row.affected_capability_ids.is_empty() {
            errors.push(M5RolloutInventoryValidationError::NoAffectedCapabilities {
                command_id: row.command_id.clone(),
            });
        }
        for required_surface in M5RolloutConsumerSurfaceClass::ALL {
            if !row
                .surfaced_in
                .iter()
                .any(|surface| surface.surface_class == required_surface)
            {
                errors.push(M5RolloutInventoryValidationError::MissingSurfaceDisclosure {
                    command_id: row.command_id.clone(),
                    surface_class: required_surface.as_str().to_string(),
                });
            }
        }
        if !row.no_hidden_flag_rule_satisfied {
            errors.push(M5RolloutInventoryValidationError::HiddenFlagRuleFailed {
                command_id: row.command_id.clone(),
            });
        }
        if row.stable_claim_allowed && row.effective_state_class != M5RolloutStateClass::Stable {
            errors.push(M5RolloutInventoryValidationError::StableClaimMismatch {
                command_id: row.command_id.clone(),
                effective_state: row.effective_state_class.as_str().to_string(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
