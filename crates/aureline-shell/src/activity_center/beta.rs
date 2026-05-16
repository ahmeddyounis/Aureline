//! Beta-grade activity-center projection.
//!
//! This module is the page-level surface that promotes the durable
//! activity center to beta. It does not own the per-row activity
//! lifecycle truth (that still lives in [`super::alpha`] and the
//! routed-notification path under [`crate::notifications`]); it pins
//! the acceptance promises a beta reviewer needs to be able to inspect
//! on every claimed row:
//!
//! - **Authoritative reopen.** Every durable job row reopens an exact
//!   durable object or a truthful placeholder. The validator rejects a
//!   row that falls back to a generic home target.
//! - **Exact object routing.** The page carries the exact object
//!   identity that the row reopens, the row badge mirrors the row
//!   state, and the support-export row carries the same identity and
//!   resolution vocabulary.
//! - **Support-export parity.** The activity row, the activity badge,
//!   and the support-export row agree on `job_family`, `state`,
//!   `resolution`, and `exact_reopen_identity_ref`; the validator
//!   rejects any drift.
//! - **Toast independence.** Long-running and retryable work no longer
//!   relies on an ephemeral toast to remain recoverable; the row
//!   exposes durable acknowledge/retry/cancel affordances and a
//!   `recoverable_without_toast` invariant.
//!
//! The same projection feeds the live shell, the
//! `aureline_shell_activity_center` headless inspector, and the
//! support-export wrapper. UI rows, CLI rows, and support-export rows
//! always come from the same `case_id` and `shared_contract_ref`, so
//! the live shell, the review packet, and the support export report
//! the same activity-center truth.

use serde::{Deserialize, Serialize};

use super::alpha::{ActivityJobFamily, ActivityPartition, ActivityRowStateClass};
use crate::notifications::envelope::{PrivacyClass, SeverityClass, SourceSubsystem};

/// Beta activity-center schema version exported with every record.
pub const ACTIVITY_CENTER_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every beta activity-center row.
pub const ACTIVITY_CENTER_BETA_SHARED_CONTRACT_REF: &str = "shell:activity_center_beta:v1";

/// Stable record kind for [`ActivityCenterBetaPage`] payloads.
pub const ACTIVITY_CENTER_BETA_PAGE_RECORD_KIND: &str = "shell_activity_center_beta_page_record";

/// Stable record kind for [`ActivityCenterBetaRow`] payloads.
pub const ACTIVITY_CENTER_BETA_ROW_RECORD_KIND: &str = "shell_activity_center_beta_row_record";

/// Stable record kind for [`ActivityCenterBetaBadge`] payloads.
pub const ACTIVITY_CENTER_BETA_BADGE_RECORD_KIND: &str = "shell_activity_center_beta_badge_record";

/// Stable record kind for [`ActivityCenterBetaSupportExportRow`] payloads.
pub const ACTIVITY_CENTER_BETA_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "shell_activity_center_beta_support_export_row_record";

/// Stable record kind for [`ActivityCenterBetaSupportExport`] payloads.
pub const ACTIVITY_CENTER_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_activity_center_beta_support_export_record";

/// Class of reopen path the row promises.
///
/// The beta promise: every row reopens an exact durable object or a
/// truthful placeholder; a generic home fallback would silently strand
/// the row and is rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthoritativeReopenClass {
    /// Opens the canonical exact target ref. `exact_target_identity_ref`
    /// must be present on the row.
    ExactDurableObject,
    /// Opens a placeholder sheet that explains why the exact object is
    /// not directly reachable (object archived, retention elapsed,
    /// revalidation required); `placeholder_reason_label` must be
    /// present on the row.
    TruthfulPlaceholder,
    /// Opens a denial sheet that quotes the policy or trust reason the
    /// reopen was refused; `denial_reason_label` must be present on
    /// the row.
    DeniedAndExplained,
}

impl AuthoritativeReopenClass {
    /// Returns the stable schema token for this reopen class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactDurableObject => "exact_durable_object",
            Self::TruthfulPlaceholder => "truthful_placeholder",
            Self::DeniedAndExplained => "denied_and_explained",
        }
    }
}

/// Resolution posture for a durable activity row.
///
/// The resolution class is independent from the lifecycle state class:
/// a `Completed` row is still `Unresolved` until the user reviews it,
/// and a `Failed` row stays `Unresolved` until acknowledged or repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowResolutionClass {
    /// Row is unresolved and visible in `current_work` or
    /// `needs_attention`.
    Unresolved,
    /// Row has been acknowledged but not resolved. Acknowledgement
    /// preserves durable history while clearing the attention badge.
    Acknowledged,
    /// Row was resolved by a typed action (retry, cancel, repair, or
    /// user-accept).
    Resolved,
    /// Row is resolved and archived. Archive preserves history without
    /// keeping the row in the active partitions.
    ArchivedAfterResolve,
    /// Row requires explicit revalidation (typically due to trust,
    /// policy, or sleep/resume boundaries) before it can be reopened.
    RequiresRevalidation,
}

impl RowResolutionClass {
    /// Returns the stable schema token for this resolution class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unresolved => "unresolved",
            Self::Acknowledged => "acknowledged",
            Self::Resolved => "resolved",
            Self::ArchivedAfterResolve => "archived_after_resolve",
            Self::RequiresRevalidation => "requires_revalidation",
        }
    }
}

/// Retry posture exposed by the row.
///
/// The beta promise: retry is a durable affordance on the row, not a
/// toast button. `NotApplicable` is used while the row is still
/// in-flight or already terminal-successful; `Denied` is reserved for
/// trust/policy/source-integrity denials and must carry an explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableRetryPosture {
    /// Retry is not meaningful at the current state.
    NotApplicable,
    /// Retry is offered as a durable row action.
    DurableRetryOffered,
    /// Retry has already been issued from the row and is in flight or
    /// queued.
    DurableRetryInFlight,
    /// Retry is denied by upstream context.
    Denied,
}

impl DurableRetryPosture {
    /// Returns the stable schema token for this retry posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::DurableRetryOffered => "durable_retry_offered",
            Self::DurableRetryInFlight => "durable_retry_in_flight",
            Self::Denied => "denied",
        }
    }
}

/// Beta toast-independence posture for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToastIndependencePosture {
    /// True when the row is recoverable without an active toast. Beta
    /// rejects any value other than `true`.
    pub recoverable_without_toast: bool,
    /// True when the row exposes a durable acknowledge action distinct
    /// from a toast dismissal.
    pub durable_acknowledge_available: bool,
    /// True when retry, when offered, is bound to a durable row action
    /// rather than a toast button.
    pub retry_bound_to_durable_action: bool,
    /// True when the row preserves a reopenable detail link even after
    /// the originating toast is dismissed or expires.
    pub reopenable_after_toast_expiry: bool,
}

impl ToastIndependencePosture {
    /// Builds a toast-independence posture that satisfies the beta
    /// invariants for a durable row.
    pub fn durable() -> Self {
        Self {
            recoverable_without_toast: true,
            durable_acknowledge_available: true,
            retry_bound_to_durable_action: true,
            reopenable_after_toast_expiry: true,
        }
    }
}

/// One activity-center beta badge that mirrors a row.
///
/// Beta requires badges, rows, and support exports to agree on the
/// closed vocabulary. The badge carries the parity tokens the
/// validator compares back to the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityCenterBetaBadge {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable row id this badge mirrors.
    pub row_id: String,
    /// Mirrored job family token.
    pub job_family: ActivityJobFamily,
    /// Mirrored lifecycle state token.
    pub state_class: ActivityRowStateClass,
    /// Mirrored resolution token.
    pub resolution_class: RowResolutionClass,
    /// Mirrored activity partition.
    pub activity_partition: ActivityPartition,
    /// Short reviewer-facing badge label.
    pub badge_label: String,
    /// True when the badge counts toward the attention chip in the
    /// status strip.
    pub counts_toward_attention: bool,
}

/// Support-export projection row.
///
/// The export row carries the same identity, family, state, and
/// resolution vocabulary as the row and the badge. Beta forbids
/// support-export rows that quote raw private material (paths,
/// credentials, raw URLs); only typed refs and labels cross this
/// boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityCenterBetaSupportExportRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable row id this export quotes.
    pub row_id: String,
    /// Stable durable-job id this export quotes.
    pub durable_job_id: String,
    /// Mirrored job family token.
    pub job_family: ActivityJobFamily,
    /// Mirrored lifecycle state token.
    pub state_class: ActivityRowStateClass,
    /// Mirrored resolution token.
    pub resolution_class: RowResolutionClass,
    /// Exact reopen identity (when applicable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_reopen_identity_ref: Option<String>,
    /// Reopen class.
    pub reopen_class: AuthoritativeReopenClass,
    /// Stable support-pack item id.
    pub support_pack_item_id: String,
    /// True when no raw private material is exported.
    pub raw_private_material_excluded: bool,
}

/// One durable beta activity-center row.
///
/// The beta row pins the four acceptance promises: authoritative
/// reopen, exact object routing, badge / support-export parity, and
/// toast-independence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityCenterBetaRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable case id used to pivot across surfaces.
    pub case_id: String,
    /// Stable activity-row id consumed by chrome and headless inspector.
    pub row_id: String,
    /// Stable durable-job id.
    pub durable_job_id: String,
    /// Stable canonical event id shared with the routing layer.
    pub canonical_event_id: String,
    /// Job family represented by this row.
    pub job_family: ActivityJobFamily,
    /// Source subsystem owning the job.
    pub source_subsystem: SourceSubsystem,
    /// Severity class projected by the row.
    pub severity_class: SeverityClass,
    /// Privacy class projected by the row.
    pub privacy_class: PrivacyClass,
    /// Lifecycle state class.
    pub state_class: ActivityRowStateClass,
    /// Resolution class for this row.
    pub resolution_class: RowResolutionClass,
    /// Activity partition the row belongs to.
    pub activity_partition: ActivityPartition,
    /// Reviewer-facing one-line summary.
    pub summary_label: String,
    /// Reviewer-facing target label (workspace, package, branch, etc.).
    pub target_label: String,
    /// Reopen class.
    pub reopen_class: AuthoritativeReopenClass,
    /// Exact object identity ref reopened by the row when
    /// [`AuthoritativeReopenClass::ExactDurableObject`] is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_target_identity_ref: Option<String>,
    /// Placeholder reason label when
    /// [`AuthoritativeReopenClass::TruthfulPlaceholder`] is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_reason_label: Option<String>,
    /// Denial reason label when
    /// [`AuthoritativeReopenClass::DeniedAndExplained`] is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason_label: Option<String>,
    /// True when this row represents long-running or retryable work,
    /// not a one-shot fire-and-forget event.
    pub is_long_running_or_retryable: bool,
    /// Retry posture exposed by the row.
    pub retry_posture: DurableRetryPosture,
    /// Toast-independence posture.
    pub toast_independence: ToastIndependencePosture,
    /// Stable command id surfaced as the open-details affordance.
    pub open_details_command_id: String,
    /// Optional stable command id surfaced as the retry affordance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_command_id: Option<String>,
    /// Reviewer-facing narrative used in fixtures and a11y exports.
    pub narrative: String,
}

/// Aggregate summary banner for the beta page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ActivityCenterBetaSummary {
    /// Number of rows on the page.
    pub row_count: usize,
    /// Rows reopening an exact durable object.
    pub exact_reopen_row_count: usize,
    /// Rows reopening a truthful placeholder.
    pub placeholder_reopen_row_count: usize,
    /// Rows reopening a denial sheet.
    pub denial_reopen_row_count: usize,
    /// Rows representing long-running or retryable work.
    pub long_running_or_retryable_row_count: usize,
    /// Rows offering a durable retry affordance.
    pub durable_retry_offered_row_count: usize,
    /// Rows in `needs_attention` that mirror to an attention-counting
    /// badge.
    pub attention_badge_row_count: usize,
    /// Job families present on the page.
    pub job_families_present: Vec<ActivityJobFamily>,
}

impl ActivityCenterBetaSummary {
    fn from_rows(rows: &[ActivityCenterBetaRow], badges: &[ActivityCenterBetaBadge]) -> Self {
        let row_count = rows.len();
        let mut exact = 0usize;
        let mut placeholder = 0usize;
        let mut denial = 0usize;
        let mut long_running = 0usize;
        let mut retry_offered = 0usize;
        let mut families: Vec<ActivityJobFamily> = Vec::new();
        for row in rows {
            match row.reopen_class {
                AuthoritativeReopenClass::ExactDurableObject => exact += 1,
                AuthoritativeReopenClass::TruthfulPlaceholder => placeholder += 1,
                AuthoritativeReopenClass::DeniedAndExplained => denial += 1,
            }
            if row.is_long_running_or_retryable {
                long_running += 1;
            }
            if matches!(row.retry_posture, DurableRetryPosture::DurableRetryOffered) {
                retry_offered += 1;
            }
            if !families.contains(&row.job_family) {
                families.push(row.job_family);
            }
        }
        families.sort();
        let attention_badge_row_count = badges
            .iter()
            .filter(|badge| badge.counts_toward_attention)
            .count();
        Self {
            row_count,
            exact_reopen_row_count: exact,
            placeholder_reopen_row_count: placeholder,
            denial_reopen_row_count: denial,
            long_running_or_retryable_row_count: long_running,
            durable_retry_offered_row_count: retry_offered,
            attention_badge_row_count,
            job_families_present: families,
        }
    }
}

/// Top-level beta activity-center page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityCenterBetaPage {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Reviewer-facing page label.
    pub page_label: String,
    /// Aggregate summary banner.
    pub summary: ActivityCenterBetaSummary,
    /// Beta rows on the page.
    pub rows: Vec<ActivityCenterBetaRow>,
    /// Badge mirror for each row on the page.
    pub badges: Vec<ActivityCenterBetaBadge>,
}

impl ActivityCenterBetaPage {
    /// Builds a beta page from the row and badge lists.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        rows: Vec<ActivityCenterBetaRow>,
        badges: Vec<ActivityCenterBetaBadge>,
    ) -> Self {
        let summary = ActivityCenterBetaSummary::from_rows(&rows, &badges);
        Self {
            record_kind: ACTIVITY_CENTER_BETA_PAGE_RECORD_KIND.to_owned(),
            schema_version: ACTIVITY_CENTER_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACTIVITY_CENTER_BETA_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            summary,
            rows,
            badges,
        }
    }

    /// True when every claimed beta job family appears in the page.
    pub fn covers_required_families(&self) -> bool {
        [
            ActivityJobFamily::Indexing,
            ActivityJobFamily::Restore,
            ActivityJobFamily::InstallUpdate,
            ActivityJobFamily::TaskRun,
            ActivityJobFamily::TestRun,
            ActivityJobFamily::GitReview,
        ]
        .iter()
        .all(|family| self.summary.job_families_present.contains(family))
    }
}

/// Support-export wrapper that quotes the beta page plus a row-aligned
/// export row for every page row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityCenterBetaSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Generated-at timestamp.
    pub generated_at: String,
    /// Embedded beta page.
    pub page: ActivityCenterBetaPage,
    /// Per-row export rows in stable page order.
    pub rows: Vec<ActivityCenterBetaSupportExportRow>,
    /// Case ids of the rows quoted by the export, in stable page order.
    pub case_ids: Vec<String>,
    /// True when no raw private material crosses the export boundary.
    pub raw_private_material_excluded: bool,
}

impl ActivityCenterBetaSupportExport {
    /// Builds a support-export wrapper from a beta page.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: ActivityCenterBetaPage,
    ) -> Self {
        let rows: Vec<ActivityCenterBetaSupportExportRow> = page
            .rows
            .iter()
            .map(|row| ActivityCenterBetaSupportExportRow {
                record_kind: ACTIVITY_CENTER_BETA_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
                schema_version: ACTIVITY_CENTER_BETA_SCHEMA_VERSION,
                shared_contract_ref: ACTIVITY_CENTER_BETA_SHARED_CONTRACT_REF.to_owned(),
                row_id: row.row_id.clone(),
                durable_job_id: row.durable_job_id.clone(),
                job_family: row.job_family,
                state_class: row.state_class,
                resolution_class: row.resolution_class,
                exact_reopen_identity_ref: row.exact_target_identity_ref.clone(),
                reopen_class: row.reopen_class,
                support_pack_item_id: format!("support.item.activity.beta.{}", row.row_id),
                raw_private_material_excluded: true,
            })
            .collect();
        let case_ids: Vec<String> = page.rows.iter().map(|row| row.case_id.clone()).collect();
        Self {
            record_kind: ACTIVITY_CENTER_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ACTIVITY_CENTER_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACTIVITY_CENTER_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            rows,
            case_ids,
            raw_private_material_excluded: true,
        }
    }
}

/// Validation errors raised when the beta page fails an acceptance
/// invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivityCenterBetaValidationError {
    /// A row promised an exact durable-object reopen but carried no
    /// exact identity ref.
    ExactReopenMissingIdentity {
        /// Row id.
        row_id: String,
    },
    /// A row promised a truthful placeholder but carried no
    /// placeholder reason label.
    PlaceholderReopenMissingReason {
        /// Row id.
        row_id: String,
    },
    /// A row promised an explained denial but carried no denial reason
    /// label.
    DenialReopenMissingReason {
        /// Row id.
        row_id: String,
    },
    /// A row's badge did not echo the row's `job_family`, `state`,
    /// `resolution`, or `activity_partition`.
    BadgeRowParityDrift {
        /// Row id.
        row_id: String,
        /// Field that drifted.
        field: String,
    },
    /// The badge mirror is missing a row that exists on the page.
    BadgeMissingForRow {
        /// Row id.
        row_id: String,
    },
    /// A support-export row drifted from the page row on `job_family`,
    /// `state`, `resolution`, or `exact_reopen_identity_ref`.
    SupportExportParityDrift {
        /// Row id.
        row_id: String,
        /// Field that drifted.
        field: String,
    },
    /// The support-export row is missing for a page row.
    SupportExportMissingForRow {
        /// Row id.
        row_id: String,
    },
    /// A row declared it was recoverable without a toast but the
    /// posture admitted a toast-only path.
    ToastDependenceAdmitted {
        /// Row id.
        row_id: String,
        /// Reason label.
        reason: String,
    },
    /// A long-running or retryable row did not offer a durable
    /// affordance.
    LongRunningRowMissingDurableAffordance {
        /// Row id.
        row_id: String,
    },
    /// A page did not cover every required beta job family.
    JobFamilyCoverageIncomplete {
        /// Missing family token.
        missing_family: String,
    },
}

impl std::fmt::Display for ActivityCenterBetaValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExactReopenMissingIdentity { row_id } => write!(
                f,
                "row {row_id} promises exact_durable_object reopen but carries no exact_target_identity_ref"
            ),
            Self::PlaceholderReopenMissingReason { row_id } => write!(
                f,
                "row {row_id} promises truthful_placeholder reopen but carries no placeholder_reason_label"
            ),
            Self::DenialReopenMissingReason { row_id } => write!(
                f,
                "row {row_id} promises denied_and_explained reopen but carries no denial_reason_label"
            ),
            Self::BadgeRowParityDrift { row_id, field } => write!(
                f,
                "badge for row {row_id} drifted from the row on field {field}"
            ),
            Self::BadgeMissingForRow { row_id } => {
                write!(f, "row {row_id} has no badge mirror on the page")
            }
            Self::SupportExportParityDrift { row_id, field } => write!(
                f,
                "support-export row for {row_id} drifted from the page row on field {field}"
            ),
            Self::SupportExportMissingForRow { row_id } => write!(
                f,
                "row {row_id} has no support-export row in the wrapper"
            ),
            Self::ToastDependenceAdmitted { row_id, reason } => write!(
                f,
                "row {row_id} admitted a toast-only recovery path: {reason}"
            ),
            Self::LongRunningRowMissingDurableAffordance { row_id } => write!(
                f,
                "long-running or retryable row {row_id} did not expose a durable retry, acknowledge, or open-details affordance"
            ),
            Self::JobFamilyCoverageIncomplete { missing_family } => write!(
                f,
                "beta activity-center page did not cover required job family {missing_family}"
            ),
        }
    }
}

impl std::error::Error for ActivityCenterBetaValidationError {}

/// Validates the beta page against the four acceptance promises.
pub fn validate_activity_center_beta_page(
    page: &ActivityCenterBetaPage,
) -> Result<(), Vec<ActivityCenterBetaValidationError>> {
    let mut errors: Vec<ActivityCenterBetaValidationError> = Vec::new();

    for row in &page.rows {
        match row.reopen_class {
            AuthoritativeReopenClass::ExactDurableObject => {
                if row
                    .exact_target_identity_ref
                    .as_deref()
                    .map_or(true, str::is_empty)
                {
                    errors.push(
                        ActivityCenterBetaValidationError::ExactReopenMissingIdentity {
                            row_id: row.row_id.clone(),
                        },
                    );
                }
            }
            AuthoritativeReopenClass::TruthfulPlaceholder => {
                if row
                    .placeholder_reason_label
                    .as_deref()
                    .map_or(true, str::is_empty)
                {
                    errors.push(
                        ActivityCenterBetaValidationError::PlaceholderReopenMissingReason {
                            row_id: row.row_id.clone(),
                        },
                    );
                }
            }
            AuthoritativeReopenClass::DeniedAndExplained => {
                if row
                    .denial_reason_label
                    .as_deref()
                    .map_or(true, str::is_empty)
                {
                    errors.push(
                        ActivityCenterBetaValidationError::DenialReopenMissingReason {
                            row_id: row.row_id.clone(),
                        },
                    );
                }
            }
        }

        let badge = page.badges.iter().find(|b| b.row_id == row.row_id);
        match badge {
            None => errors.push(ActivityCenterBetaValidationError::BadgeMissingForRow {
                row_id: row.row_id.clone(),
            }),
            Some(badge) => {
                if badge.job_family != row.job_family {
                    errors.push(ActivityCenterBetaValidationError::BadgeRowParityDrift {
                        row_id: row.row_id.clone(),
                        field: "job_family".to_owned(),
                    });
                }
                if badge.state_class != row.state_class {
                    errors.push(ActivityCenterBetaValidationError::BadgeRowParityDrift {
                        row_id: row.row_id.clone(),
                        field: "state_class".to_owned(),
                    });
                }
                if badge.resolution_class != row.resolution_class {
                    errors.push(ActivityCenterBetaValidationError::BadgeRowParityDrift {
                        row_id: row.row_id.clone(),
                        field: "resolution_class".to_owned(),
                    });
                }
                if badge.activity_partition != row.activity_partition {
                    errors.push(ActivityCenterBetaValidationError::BadgeRowParityDrift {
                        row_id: row.row_id.clone(),
                        field: "activity_partition".to_owned(),
                    });
                }
            }
        }

        if !row.toast_independence.recoverable_without_toast {
            errors.push(ActivityCenterBetaValidationError::ToastDependenceAdmitted {
                row_id: row.row_id.clone(),
                reason: "recoverable_without_toast=false".to_owned(),
            });
        }
        if !row.toast_independence.reopenable_after_toast_expiry {
            errors.push(ActivityCenterBetaValidationError::ToastDependenceAdmitted {
                row_id: row.row_id.clone(),
                reason: "reopenable_after_toast_expiry=false".to_owned(),
            });
        }

        if row.is_long_running_or_retryable {
            let offers_durable_affordance = row.toast_independence.durable_acknowledge_available
                || matches!(
                    row.retry_posture,
                    DurableRetryPosture::DurableRetryOffered
                        | DurableRetryPosture::DurableRetryInFlight
                );
            if !offers_durable_affordance {
                errors.push(
                    ActivityCenterBetaValidationError::LongRunningRowMissingDurableAffordance {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if matches!(row.retry_posture, DurableRetryPosture::DurableRetryOffered)
                && !row.toast_independence.retry_bound_to_durable_action
            {
                errors.push(ActivityCenterBetaValidationError::ToastDependenceAdmitted {
                    row_id: row.row_id.clone(),
                    reason: "durable_retry_offered without retry_bound_to_durable_action"
                        .to_owned(),
                });
            }
        }
    }

    for required in [
        ActivityJobFamily::Indexing,
        ActivityJobFamily::Restore,
        ActivityJobFamily::InstallUpdate,
        ActivityJobFamily::TaskRun,
        ActivityJobFamily::TestRun,
        ActivityJobFamily::GitReview,
    ] {
        if !page.summary.job_families_present.contains(&required) {
            errors.push(
                ActivityCenterBetaValidationError::JobFamilyCoverageIncomplete {
                    missing_family: required.as_str().to_owned(),
                },
            );
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates a support-export wrapper for parity with its embedded page.
pub fn validate_activity_center_beta_support_export(
    export: &ActivityCenterBetaSupportExport,
) -> Result<(), Vec<ActivityCenterBetaValidationError>> {
    let mut errors: Vec<ActivityCenterBetaValidationError> = Vec::new();
    for row in &export.page.rows {
        let export_row = export.rows.iter().find(|r| r.row_id == row.row_id);
        match export_row {
            None => errors.push(
                ActivityCenterBetaValidationError::SupportExportMissingForRow {
                    row_id: row.row_id.clone(),
                },
            ),
            Some(export_row) => {
                if export_row.job_family != row.job_family {
                    errors.push(
                        ActivityCenterBetaValidationError::SupportExportParityDrift {
                            row_id: row.row_id.clone(),
                            field: "job_family".to_owned(),
                        },
                    );
                }
                if export_row.state_class != row.state_class {
                    errors.push(
                        ActivityCenterBetaValidationError::SupportExportParityDrift {
                            row_id: row.row_id.clone(),
                            field: "state_class".to_owned(),
                        },
                    );
                }
                if export_row.resolution_class != row.resolution_class {
                    errors.push(
                        ActivityCenterBetaValidationError::SupportExportParityDrift {
                            row_id: row.row_id.clone(),
                            field: "resolution_class".to_owned(),
                        },
                    );
                }
                if export_row.reopen_class != row.reopen_class {
                    errors.push(
                        ActivityCenterBetaValidationError::SupportExportParityDrift {
                            row_id: row.row_id.clone(),
                            field: "reopen_class".to_owned(),
                        },
                    );
                }
                if export_row.exact_reopen_identity_ref != row.exact_target_identity_ref {
                    errors.push(
                        ActivityCenterBetaValidationError::SupportExportParityDrift {
                            row_id: row.row_id.clone(),
                            field: "exact_reopen_identity_ref".to_owned(),
                        },
                    );
                }
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn make_badge(row: &ActivityCenterBetaRow, badge_label: &str) -> ActivityCenterBetaBadge {
    let counts_toward_attention =
        matches!(row.activity_partition, ActivityPartition::NeedsAttention)
            && matches!(row.resolution_class, RowResolutionClass::Unresolved);
    ActivityCenterBetaBadge {
        record_kind: ACTIVITY_CENTER_BETA_BADGE_RECORD_KIND.to_owned(),
        schema_version: ACTIVITY_CENTER_BETA_SCHEMA_VERSION,
        shared_contract_ref: ACTIVITY_CENTER_BETA_SHARED_CONTRACT_REF.to_owned(),
        row_id: row.row_id.clone(),
        job_family: row.job_family,
        state_class: row.state_class,
        resolution_class: row.resolution_class,
        activity_partition: row.activity_partition,
        badge_label: badge_label.to_owned(),
        counts_toward_attention,
    }
}

fn make_row(
    case_id: &str,
    row_suffix: &str,
    job_family: ActivityJobFamily,
    source_subsystem: SourceSubsystem,
    severity_class: SeverityClass,
    privacy_class: PrivacyClass,
    state_class: ActivityRowStateClass,
    resolution_class: RowResolutionClass,
    reopen_class: AuthoritativeReopenClass,
    exact_target_identity_ref: Option<String>,
    placeholder_reason_label: Option<String>,
    denial_reason_label: Option<String>,
    summary_label: &str,
    target_label: &str,
    is_long_running_or_retryable: bool,
    retry_posture: DurableRetryPosture,
    open_details_command_id: &str,
    retry_command_id: Option<&str>,
    narrative: &str,
) -> ActivityCenterBetaRow {
    ActivityCenterBetaRow {
        record_kind: ACTIVITY_CENTER_BETA_ROW_RECORD_KIND.to_owned(),
        schema_version: ACTIVITY_CENTER_BETA_SCHEMA_VERSION,
        shared_contract_ref: ACTIVITY_CENTER_BETA_SHARED_CONTRACT_REF.to_owned(),
        case_id: case_id.to_owned(),
        row_id: format!("ux:activity-row:beta:{row_suffix}"),
        durable_job_id: format!("ux:durable-job:beta:{row_suffix}"),
        canonical_event_id: format!("ux:event:beta:{row_suffix}"),
        job_family,
        source_subsystem,
        severity_class,
        privacy_class,
        state_class,
        resolution_class,
        activity_partition: state_class.partition(),
        summary_label: summary_label.to_owned(),
        target_label: target_label.to_owned(),
        reopen_class,
        exact_target_identity_ref,
        placeholder_reason_label,
        denial_reason_label,
        is_long_running_or_retryable,
        retry_posture,
        toast_independence: ToastIndependencePosture::durable(),
        open_details_command_id: open_details_command_id.to_owned(),
        retry_command_id: retry_command_id.map(ToOwned::to_owned),
        narrative: narrative.to_owned(),
    }
}

/// Seeded fixture builder used by the headless inspector and the
/// integration test. The seed is the only mint-from-truth path for the
/// JSON checked in under `fixtures/ux/m3/activity_center/`, so the
/// live shell records, the CLI rows, and the support-export rows
/// cannot drift.
pub fn seeded_activity_center_beta_page() -> ActivityCenterBetaPage {
    let indexing = make_row(
        "shell:activity-center:beta:indexing-running:01",
        "indexing:hot-set",
        ActivityJobFamily::Indexing,
        SourceSubsystem::Indexer,
        SeverityClass::Info,
        PrivacyClass::WorkspaceSensitive,
        ActivityRowStateClass::Running,
        RowResolutionClass::Unresolved,
        AuthoritativeReopenClass::ExactDurableObject,
        Some("ux:durable-job:beta:indexing:hot-set".to_owned()),
        None,
        None,
        "Indexing active workspace hot set",
        "Active workspace",
        true,
        DurableRetryPosture::NotApplicable,
        "cmd:activity.open_job_details",
        None,
        "Indexing is running; the row exposes a durable open-details affordance and a cancel command — no toast is required to keep it recoverable.",
    );

    let restore = make_row(
        "shell:activity-center:beta:restore-completed:01",
        "restore:last-session",
        ActivityJobFamily::Restore,
        SourceSubsystem::Shell,
        SeverityClass::Success,
        PrivacyClass::WorkspaceSensitive,
        ActivityRowStateClass::Completed,
        RowResolutionClass::Resolved,
        AuthoritativeReopenClass::ExactDurableObject,
        Some("ux:durable-job:beta:restore:last-session".to_owned()),
        None,
        None,
        "Restore completed for last session",
        "Last session workspace",
        true,
        DurableRetryPosture::NotApplicable,
        "cmd:activity.open_job_details",
        None,
        "Restore completed; the row remains reopenable through the durable open-details affordance after the success toast expires.",
    );

    let install_update = make_row(
        "shell:activity-center:beta:install-needs-review:01",
        "install:package-update",
        ActivityJobFamily::InstallUpdate,
        SourceSubsystem::InstallUpdateAttach,
        SeverityClass::Warning,
        PrivacyClass::WorkspaceSensitive,
        ActivityRowStateClass::PartiallyCompleted,
        RowResolutionClass::Unresolved,
        AuthoritativeReopenClass::ExactDurableObject,
        Some("ux:durable-job:beta:install:package-update".to_owned()),
        None,
        None,
        "Package update partially applied; needs review",
        "Workspace extensions",
        true,
        DurableRetryPosture::DurableRetryOffered,
        "cmd:activity.open_job_details",
        Some("cmd:install_update.retry_failed_items"),
        "Package update reached partial completion across policy, network, and trust boundaries; the row exposes a durable retry-failed-items affordance bound to a typed command, not a toast button.",
    );

    let task_run = make_row(
        "shell:activity-center:beta:task-queued:01",
        "task:dev-server",
        ActivityJobFamily::TaskRun,
        SourceSubsystem::TaskRunner,
        SeverityClass::Info,
        PrivacyClass::WorkspaceSensitive,
        ActivityRowStateClass::QueuedWaiting,
        RowResolutionClass::Unresolved,
        AuthoritativeReopenClass::ExactDurableObject,
        Some("ux:durable-job:beta:task:dev-server".to_owned()),
        None,
        None,
        "Development server queued for execution profile",
        "tasks.json: dev-server",
        true,
        DurableRetryPosture::NotApplicable,
        "cmd:activity.open_job_details",
        None,
        "Task is queued waiting for an execution profile; the row stays reopenable to inspect the queue reason without a toast being on screen.",
    );

    let test_run = make_row(
        "shell:activity-center:beta:test-failed-retry:01",
        "test:pytest-suite",
        ActivityJobFamily::TestRun,
        SourceSubsystem::TestRunner,
        SeverityClass::Error,
        PrivacyClass::WorkspaceSensitive,
        ActivityRowStateClass::Failed,
        RowResolutionClass::Unresolved,
        AuthoritativeReopenClass::ExactDurableObject,
        Some("ux:durable-job:beta:test:pytest-suite".to_owned()),
        None,
        None,
        "Test run failed; durable retry offered",
        "pytest: tests/",
        true,
        DurableRetryPosture::DurableRetryOffered,
        "cmd:activity.open_job_details",
        Some("cmd:test_run.retry_failed"),
        "Test run failed; retry is a durable row action bound to cmd:test_run.retry_failed and is recoverable even after the failure toast expires.",
    );

    let git_review = make_row(
        "shell:activity-center:beta:git-publish-denied:01",
        "git:publish-feature-branch",
        ActivityJobFamily::GitReview,
        SourceSubsystem::ProviderBearing,
        SeverityClass::Blocking,
        PrivacyClass::ManagedSensitive,
        ActivityRowStateClass::Failed,
        RowResolutionClass::RequiresRevalidation,
        AuthoritativeReopenClass::DeniedAndExplained,
        None,
        None,
        Some("Publish denied by managed policy; explicit revalidation required.".to_owned()),
        "Publish denied by managed policy",
        "feature-branch → upstream",
        true,
        DurableRetryPosture::Denied,
        "cmd:activity.open_job_details",
        None,
        "Publish was denied by managed policy; the row opens an explained-denial sheet rather than retrying silently, and remains reopenable for revalidation review.",
    );

    let restore_placeholder = make_row(
        "shell:activity-center:beta:restore-placeholder:01",
        "restore:archived-session",
        ActivityJobFamily::Restore,
        SourceSubsystem::Shell,
        SeverityClass::Info,
        PrivacyClass::WorkspaceSensitive,
        ActivityRowStateClass::Superseded,
        RowResolutionClass::ArchivedAfterResolve,
        AuthoritativeReopenClass::TruthfulPlaceholder,
        None,
        Some(
            "Source session was archived after restore; placeholder explains retention and points at archive."
                .to_owned(),
        ),
        None,
        "Older restore was archived; placeholder retains history",
        "Archived session 2026-05-08",
        true,
        DurableRetryPosture::NotApplicable,
        "cmd:activity.open_job_details",
        None,
        "Source session has been archived; the row opens a truthful placeholder explaining retention rather than falling back to a generic home target.",
    );

    let rows = vec![
        indexing,
        restore,
        install_update,
        task_run,
        test_run,
        git_review,
        restore_placeholder,
    ];

    let badges: Vec<ActivityCenterBetaBadge> = rows
        .iter()
        .map(|row| {
            let label = match (row.activity_partition, row.resolution_class) {
                (ActivityPartition::CurrentWork, _) => "In progress",
                (ActivityPartition::NeedsAttention, RowResolutionClass::Unresolved) => {
                    "Needs review"
                }
                (ActivityPartition::NeedsAttention, RowResolutionClass::RequiresRevalidation) => {
                    "Revalidation required"
                }
                (ActivityPartition::NeedsAttention, _) => "Attention cleared",
                (ActivityPartition::Completed, RowResolutionClass::Resolved) => "Resolved",
                (ActivityPartition::Completed, _) => "Reviewable",
                (ActivityPartition::SuppressedHeld, _) => "Held",
            };
            make_badge(row, label)
        })
        .collect();

    ActivityCenterBetaPage::new(
        "shell:activity-center:beta:page:default",
        "Activity center (beta): durable reopen, exact routing, support-export parity",
        rows,
        badges,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_passes_validation() {
        let page = seeded_activity_center_beta_page();
        validate_activity_center_beta_page(&page).expect("seeded page must validate");
    }

    #[test]
    fn seeded_page_covers_required_families() {
        let page = seeded_activity_center_beta_page();
        assert!(page.covers_required_families());
    }

    #[test]
    fn seeded_summary_matches_rows() {
        let page = seeded_activity_center_beta_page();
        assert_eq!(page.summary.row_count, page.rows.len());
        let exact = page
            .rows
            .iter()
            .filter(|r| matches!(r.reopen_class, AuthoritativeReopenClass::ExactDurableObject))
            .count();
        assert_eq!(page.summary.exact_reopen_row_count, exact);
        let placeholder = page
            .rows
            .iter()
            .filter(|r| {
                matches!(
                    r.reopen_class,
                    AuthoritativeReopenClass::TruthfulPlaceholder
                )
            })
            .count();
        assert_eq!(page.summary.placeholder_reopen_row_count, placeholder);
        let denial = page
            .rows
            .iter()
            .filter(|r| matches!(r.reopen_class, AuthoritativeReopenClass::DeniedAndExplained))
            .count();
        assert_eq!(page.summary.denial_reopen_row_count, denial);
        let long_running = page
            .rows
            .iter()
            .filter(|r| r.is_long_running_or_retryable)
            .count();
        assert_eq!(
            page.summary.long_running_or_retryable_row_count,
            long_running
        );
    }

    #[test]
    fn validation_flags_exact_reopen_missing_identity() {
        let mut page = seeded_activity_center_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| matches!(r.reopen_class, AuthoritativeReopenClass::ExactDurableObject))
            .expect("seed has an exact reopen row");
        row.exact_target_identity_ref = None;
        let errors = validate_activity_center_beta_page(&page)
            .expect_err("must flag missing exact reopen identity");
        assert!(errors.iter().any(|e| matches!(
            e,
            ActivityCenterBetaValidationError::ExactReopenMissingIdentity { .. }
        )));
    }

    #[test]
    fn validation_flags_placeholder_missing_reason() {
        let mut page = seeded_activity_center_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| {
                matches!(
                    r.reopen_class,
                    AuthoritativeReopenClass::TruthfulPlaceholder
                )
            })
            .expect("seed has a placeholder row");
        row.placeholder_reason_label = None;
        let errors = validate_activity_center_beta_page(&page)
            .expect_err("must flag missing placeholder reason");
        assert!(errors.iter().any(|e| matches!(
            e,
            ActivityCenterBetaValidationError::PlaceholderReopenMissingReason { .. }
        )));
    }

    #[test]
    fn validation_flags_denial_missing_reason() {
        let mut page = seeded_activity_center_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| matches!(r.reopen_class, AuthoritativeReopenClass::DeniedAndExplained))
            .expect("seed has a denial row");
        row.denial_reason_label = None;
        let errors =
            validate_activity_center_beta_page(&page).expect_err("must flag missing denial reason");
        assert!(errors.iter().any(|e| matches!(
            e,
            ActivityCenterBetaValidationError::DenialReopenMissingReason { .. }
        )));
    }

    #[test]
    fn validation_flags_badge_parity_drift() {
        let mut page = seeded_activity_center_beta_page();
        page.badges[0].state_class = ActivityRowStateClass::Failed;
        let errors =
            validate_activity_center_beta_page(&page).expect_err("must flag badge parity drift");
        assert!(errors.iter().any(|e| matches!(
            e,
            ActivityCenterBetaValidationError::BadgeRowParityDrift { field, .. } if field == "state_class"
        )));
    }

    #[test]
    fn validation_flags_missing_badge() {
        let mut page = seeded_activity_center_beta_page();
        let removed_row_id = page.rows[0].row_id.clone();
        page.badges.retain(|b| b.row_id != removed_row_id);
        let errors =
            validate_activity_center_beta_page(&page).expect_err("must flag missing badge");
        assert!(errors.iter().any(|e| matches!(
            e,
            ActivityCenterBetaValidationError::BadgeMissingForRow { .. }
        )));
    }

    #[test]
    fn validation_flags_toast_dependence() {
        let mut page = seeded_activity_center_beta_page();
        page.rows[0].toast_independence.recoverable_without_toast = false;
        let errors =
            validate_activity_center_beta_page(&page).expect_err("must flag toast dependence");
        assert!(errors.iter().any(|e| matches!(
            e,
            ActivityCenterBetaValidationError::ToastDependenceAdmitted { .. }
        )));
    }

    #[test]
    fn validation_flags_long_running_without_durable_affordance() {
        let mut page = seeded_activity_center_beta_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.is_long_running_or_retryable)
            .expect("seed has a long-running row");
        row.toast_independence.durable_acknowledge_available = false;
        row.retry_posture = DurableRetryPosture::NotApplicable;
        let errors = validate_activity_center_beta_page(&page)
            .expect_err("must flag long-running row without durable affordance");
        assert!(errors.iter().any(|e| matches!(
            e,
            ActivityCenterBetaValidationError::LongRunningRowMissingDurableAffordance { .. }
        )));
    }

    #[test]
    fn validation_flags_job_family_coverage_gap() {
        let mut page = seeded_activity_center_beta_page();
        let dropped = ActivityJobFamily::GitReview;
        page.rows.retain(|r| r.job_family != dropped);
        page.badges.retain(|b| b.job_family != dropped);
        page.summary = ActivityCenterBetaSummary::from_rows(&page.rows, &page.badges);
        let errors =
            validate_activity_center_beta_page(&page).expect_err("must flag missing job family");
        assert!(errors.iter().any(|e| matches!(
            e,
            ActivityCenterBetaValidationError::JobFamilyCoverageIncomplete { .. }
        )));
    }

    #[test]
    fn support_export_quotes_every_row_and_case_id() {
        let page = seeded_activity_center_beta_page();
        let export = ActivityCenterBetaSupportExport::from_page(
            "support-export:activity-center-beta:001",
            "2026-05-15T00:00:00Z",
            page.clone(),
        );
        assert_eq!(
            export.shared_contract_ref,
            ACTIVITY_CENTER_BETA_SHARED_CONTRACT_REF
        );
        assert_eq!(export.rows.len(), page.rows.len());
        assert_eq!(export.case_ids.len(), page.rows.len());
        for (page_row, export_row) in page.rows.iter().zip(export.rows.iter()) {
            assert_eq!(page_row.row_id, export_row.row_id);
            assert_eq!(page_row.job_family, export_row.job_family);
            assert_eq!(page_row.state_class, export_row.state_class);
            assert_eq!(page_row.resolution_class, export_row.resolution_class);
            assert_eq!(page_row.reopen_class, export_row.reopen_class);
            assert_eq!(
                page_row.exact_target_identity_ref,
                export_row.exact_reopen_identity_ref
            );
        }
        validate_activity_center_beta_support_export(&export)
            .expect("support export must validate against its embedded page");
    }

    #[test]
    fn support_export_validation_flags_drift() {
        let page = seeded_activity_center_beta_page();
        let mut export = ActivityCenterBetaSupportExport::from_page(
            "support-export:activity-center-beta:001",
            "2026-05-15T00:00:00Z",
            page,
        );
        export.rows[0].state_class = ActivityRowStateClass::Failed;
        let errors =
            validate_activity_center_beta_support_export(&export).expect_err("must flag drift");
        assert!(errors.iter().any(|e| matches!(
            e,
            ActivityCenterBetaValidationError::SupportExportParityDrift { field, .. } if field == "state_class"
        )));
    }
}
