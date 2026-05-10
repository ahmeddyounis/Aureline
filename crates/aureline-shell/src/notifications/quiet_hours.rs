//! Shell-level quiet-hours, do-not-disturb, and privacy-safe badge behavior.
//!
//! This module owns the runtime posture the shell carries between routing
//! calls. It augments [`NotificationEnvelope`]s with the shell's currently-
//! active quiet-hours / do-not-disturb / focus-mode / presentation /
//! screen-share / privacy-mode / reduced-attention / power-saver / admin-
//! suppression modes BEFORE the envelope reaches the
//! [`super::router::NotificationRouter`], so the router stays the single
//! truth lane while the shell's posture decides what counts as "non-critical
//! interruption" right now.
//!
//! It also derives privacy-safe badge counts from durable items projected by
//! the router, so the badge a user sees on a durable activity row, the OS
//! app-icon, or the lock-screen summary never inflates from duplicate
//! deliveries and never names a workspace, object, or actor identifier.
//!
//! ## Truth invariants (non-negotiable)
//!
//! 1. **Critical-safety severity always interrupts.** Quiet hours, DND,
//!    presentation, privacy-mode, admin-suppression — none of these may
//!    suppress a [`SeverityClass::Critical`] envelope. The posture refuses
//!    to set `suppressed = true` for critical severity.
//! 2. **Durable history is preserved.** The posture writes only the
//!    suppression state field; it never strips `recommended_surfaces[]`.
//!    The router still delivers durable surfaces (durable_job_row,
//!    status_item, status_strip, activity_center_digest_card) under hold —
//!    that contract lives in [`super::router`].
//! 3. **The posture narrows; it never widens.** If the envelope was minted
//!    with quiet-hours modes already, the posture's modes are unioned in.
//!    The posture never removes a mode an upstream subsystem knew about.
//! 4. **Badges count deduped durable items, not raw events.** The badge
//!    projection collapses by `canonical_event_id` so a retry storm does
//!    not inflate the count.
//! 5. **OS-bound surfaces redact under policy.** When an active mode
//!    suppresses the OS app-icon badge, the projection records
//!    `os_badge_visible = false` so the platform adapter knows not to
//!    paint a count on the dock / taskbar / lock-screen summary. The
//!    in-product badge still renders so durable truth survives.
//! 6. **Summary labels are privacy-safe.** Labels carry category-class
//!    text, counts, and severity buckets only — never raw paths, raw URLs,
//!    actor identities, or workspace identifiers.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::envelope::{
    FanoutSurfaceClass, NotificationEnvelope, QuietHoursMode, SeverityClass, SuppressionReason,
    SuppressionState,
};
use super::router::RoutedNotification;

/// Schema version of [`DurableBadgeProjection`] snapshots.
pub const DURABLE_BADGE_PROJECTION_SCHEMA_VERSION: u32 = 1;
/// Stable record-kind tag used in serialized [`DurableBadgeProjection`]s.
pub const DURABLE_BADGE_PROJECTION_RECORD_KIND: &str = "durable_badge_projection_record";

/// The shell's currently-active quiet-hours posture.
///
/// One [`QuietHoursPosture`] is meant to live on the shell's notification
/// truth lane next to the [`super::router::NotificationRouter`]. It is read
/// when an envelope is about to route, when a badge is about to render, and
/// when the activity center summarizes held items.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QuietHoursPosture {
    active_modes: BTreeSet<QuietHoursMode>,
}

impl QuietHoursPosture {
    /// No mode active. Routing proceeds unmodified.
    pub fn none() -> Self {
        Self::default()
    }

    /// Build a posture with one or more active modes. `mode_none` entries
    /// are stripped — the empty set is the canonical "no quiet mode"
    /// representation.
    pub fn with_modes(modes: impl IntoIterator<Item = QuietHoursMode>) -> Self {
        Self {
            active_modes: modes
                .into_iter()
                .filter(|mode| !matches!(mode, QuietHoursMode::ModeNone))
                .collect(),
        }
    }

    /// Convenience: a user-scheduled quiet-hours posture.
    pub fn quiet_hours_user() -> Self {
        Self::with_modes([QuietHoursMode::ModeQuietHoursUser])
    }

    /// Convenience: a user do-not-disturb posture (narrower than quiet
    /// hours; also holds tier_actionable severity).
    pub fn do_not_disturb() -> Self {
        Self::with_modes([QuietHoursMode::ModeDoNotDisturbUser])
    }

    /// Convenience: focus mode (app-icon badges still render).
    pub fn focus_mode() -> Self {
        Self::with_modes([QuietHoursMode::ModeFocusModeUser])
    }

    /// Convenience: presentation / screen-share posture (audience-visible
    /// surfaces dimmed).
    pub fn presentation() -> Self {
        Self::with_modes([QuietHoursMode::ModePresentation])
    }

    /// Convenience: privacy mode (every OS-level and lock-screen surface
    /// suppressed; in-product preserved).
    pub fn privacy_mode() -> Self {
        Self::with_modes([QuietHoursMode::ModePrivacyMode])
    }

    /// Convenience: admin policy suppression (managed surface).
    pub fn admin_suppression() -> Self {
        Self::with_modes([QuietHoursMode::ModeAdminSuppression])
    }

    /// True when at least one non-trivial mode is active.
    pub fn has_active_quiet_mode(&self) -> bool {
        !self.active_modes.is_empty()
    }

    /// Borrow the typed active mode set (without `mode_none`).
    pub fn active_modes(&self) -> &BTreeSet<QuietHoursMode> {
        &self.active_modes
    }

    /// Stable, sorted list of modes for snapshot/export use.
    pub fn active_modes_sorted(&self) -> Vec<QuietHoursMode> {
        self.active_modes.iter().copied().collect()
    }

    /// True when the posture would suppress the OS app-icon badge under any
    /// active mode. Per the upstream policy matrix, the modes that suppress
    /// `os_badge_app_icon` are quiet-hours-user, DND, presentation,
    /// screen-share, privacy-mode, and admin-suppression. Focus-mode,
    /// reduced-attention, and power-saver intentionally preserve the OS
    /// badge so glanceable truth survives the hold.
    pub fn suppresses_os_app_icon_badge(&self) -> bool {
        self.active_modes
            .iter()
            .any(|mode| mode_suppresses_os_app_icon_badge(*mode))
    }

    /// True when the posture would suppress the lock-screen summary surface.
    /// Under privacy-mode lock-screen renders are denied outright; the other
    /// suppressing modes simply hold the surface, which still records a
    /// receipt at the router boundary.
    pub fn suppresses_lock_screen_summary(&self) -> bool {
        self.active_modes
            .iter()
            .any(|mode| mode_suppresses_lock_screen_summary(*mode))
    }

    /// True when the posture HOLDS attention-grabbing surfaces for an
    /// envelope of the given severity. Critical severity always interrupts;
    /// blocking severity bypasses every mode except admin-suppression.
    pub fn holds_attention_for(&self, severity: SeverityClass) -> bool {
        if severity_always_interrupts(severity) {
            return false;
        }
        if matches!(severity, SeverityClass::Blocking) {
            // Blocking severity passes through user-scheduled holds; admin
            // policy is the only suppressor strong enough to hold it, and
            // even then the admin posture cannot block tier_critical_safety
            // (enforced by the upstream contract — this module never lets
            // admin-suppression hold Critical).
            return self
                .active_modes
                .contains(&QuietHoursMode::ModeAdminSuppression);
        }
        self.has_active_quiet_mode()
    }

    /// Project this posture as the [`SuppressionState`] block an envelope
    /// would carry. Critical severity always returns an unsuppressed state
    /// (modes recorded for audit, but `suppressed = false`).
    pub fn project_suppression_state(&self, severity: SeverityClass) -> SuppressionState {
        let suppressed = self.holds_attention_for(severity);
        let active_modes_at_mint: Vec<QuietHoursMode> = if self.active_modes.is_empty() {
            vec![QuietHoursMode::ModeNone]
        } else {
            self.active_modes_sorted()
        };
        let suppression_reasons: Vec<SuppressionReason> = if suppressed {
            self.active_modes
                .iter()
                .filter_map(|mode| suppression_reason_for_mode(*mode))
                .collect()
        } else {
            Vec::new()
        };
        SuppressionState {
            active_modes_at_mint,
            suppression_reasons,
            suppressed,
        }
    }

    /// Apply this posture's suppression state to an envelope before the
    /// router sees it. The envelope's existing modes are kept; the
    /// posture's modes are unioned in. `suppressed` is recomputed: a
    /// critical-severity envelope can never end up suppressed.
    ///
    /// Returns `true` when the envelope's suppression state was changed.
    pub fn apply_to_envelope(&self, envelope: &mut NotificationEnvelope) -> bool {
        let severity = envelope.severity_class;
        let mut union: BTreeSet<QuietHoursMode> = envelope
            .suppression_state
            .non_trivial_modes()
            .into_iter()
            .collect();
        union.extend(self.active_modes.iter().copied());

        let new_modes_at_mint: Vec<QuietHoursMode> = if union.is_empty() {
            vec![QuietHoursMode::ModeNone]
        } else {
            union.iter().copied().collect()
        };

        // Critical severity always interrupts: never suppress, never carry
        // suppression reasons. Audit modes still recorded.
        let suppressed = if severity_always_interrupts(severity) {
            false
        } else if matches!(severity, SeverityClass::Blocking) {
            union.contains(&QuietHoursMode::ModeAdminSuppression)
        } else {
            !union.is_empty()
        };

        // Union existing reasons with reasons newly contributed by the
        // posture's modes. Skip reasons when the suppression won't take
        // effect (e.g., critical severity bypassing the hold).
        let mut reasons: BTreeSet<SuppressionReason> = if suppressed {
            envelope
                .suppression_state
                .suppression_reasons
                .iter()
                .copied()
                .collect()
        } else {
            BTreeSet::new()
        };
        if suppressed {
            for mode in &self.active_modes {
                if let Some(reason) = suppression_reason_for_mode(*mode) {
                    reasons.insert(reason);
                }
            }
        }
        let mut sorted_reasons: Vec<SuppressionReason> = reasons.into_iter().collect();
        // The receipt's stable order matches insertion order in tests.
        sorted_reasons.sort();

        let new_state = SuppressionState {
            active_modes_at_mint: new_modes_at_mint,
            suppression_reasons: sorted_reasons,
            suppressed,
        };

        if envelope.suppression_state == new_state {
            return false;
        }
        envelope.suppression_state = new_state;
        true
    }
}

/// Severity buckets the durable badge projection exposes. Counts are derived
/// from deduped durable items, not raw deliveries.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeSeverityCounts {
    #[serde(default)]
    pub info: u32,
    #[serde(default)]
    pub success: u32,
    #[serde(default)]
    pub warning: u32,
    #[serde(default)]
    pub degraded: u32,
    #[serde(default)]
    pub error: u32,
    #[serde(default)]
    pub blocking: u32,
    #[serde(default)]
    pub critical: u32,
}

impl BadgeSeverityCounts {
    pub fn total(&self) -> u32 {
        self.info
            + self.success
            + self.warning
            + self.degraded
            + self.error
            + self.blocking
            + self.critical
    }

    fn bump(&mut self, severity: SeverityClass) {
        match severity {
            SeverityClass::Info => self.info += 1,
            SeverityClass::Success => self.success += 1,
            SeverityClass::Warning => self.warning += 1,
            SeverityClass::Degraded => self.degraded += 1,
            SeverityClass::Error => self.error += 1,
            SeverityClass::Blocking => self.blocking += 1,
            SeverityClass::Critical => self.critical += 1,
        }
    }
}

/// Privacy-safe badge projection derived from a stream of routed
/// notifications. The projection is the truth a glance-only badge surface
/// (durable activity row pill, OS app-icon badge, lock-screen summary)
/// reads when it decides what number to show.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableBadgeProjection {
    pub record_kind: String,
    pub schema_version: u32,
    /// Total deduped durable items (one per canonical event id with at
    /// least one delivered durable surface route).
    pub durable_count: u32,
    /// Per-severity counts.
    pub severity_counts: BadgeSeverityCounts,
    /// Number of items whose severity is critical-safety. These items
    /// always render even under quiet hours / DND / privacy-mode, so the
    /// platform adapter can treat this as a "must show" subcount.
    pub critical_safety_count: u32,
    /// Number of items currently held by the active quiet-hours posture.
    /// Held items still appear in `durable_count` because durable rows
    /// always preserve history; this field exists so an exit-from-quiet-
    /// hours digest can quote the held subcount without inventing one.
    pub held_under_posture_count: u32,
    /// Modes contributing to the redaction of OS-bound surfaces, sorted
    /// stably for snapshots and support exports.
    pub active_modes: Vec<QuietHoursMode>,
    /// True when the OS app-icon badge MAY render the count. False when
    /// any active mode suppresses the OS badge (per the upstream policy
    /// matrix). The in-product durable row badge still renders either way.
    pub os_app_icon_badge_visible: bool,
    /// True when the lock-screen summary surface MAY render. False under
    /// privacy-mode and the broader quiet-hours posture.
    pub lock_screen_summary_visible: bool,
    /// Privacy-safe summary label suitable for OS / lock-screen / companion
    /// consumption. Carries category-class tokens and counts only — never
    /// names a workspace, object, actor, raw path, or raw URL.
    pub privacy_safe_summary_label: String,
    /// Time the projection was minted (from the latest contributing
    /// envelope's mint time when present, otherwise empty).
    pub minted_at: String,
}

impl DurableBadgeProjection {
    /// Build a projection from a slice of routed notifications and the
    /// current shell posture. Items are deduped by `canonical_event_id`
    /// across the slice so a retry storm cannot inflate the badge.
    pub fn from_routed(routed: &[RoutedNotification], posture: &QuietHoursPosture) -> Self {
        // Collect the most recent routing snapshot per canonical event id.
        // The router's own dedupe keeps the "first delivered" surfaces on
        // the snapshot; using BTreeMap gives a stable iteration order that
        // matches snapshot/export expectations.
        let mut by_event: BTreeMap<String, &RoutedNotification> = BTreeMap::new();
        for r in routed {
            by_event.insert(r.canonical_event_id.clone(), r);
        }

        let mut severity_counts = BadgeSeverityCounts::default();
        let mut durable_count: u32 = 0;
        let mut critical_safety_count: u32 = 0;
        let mut held_under_posture_count: u32 = 0;
        let mut latest_minted_at = String::new();

        for r in by_event.values() {
            // Only count items that have at least one durable surface
            // route in `surface_routes[]`. Held / suppressed / deduped
            // routes still represent durable truth at the activity center,
            // so we do not require `Delivered` — a held durable row is
            // still durable.
            if !has_durable_surface(r) {
                continue;
            }
            durable_count += 1;
            severity_counts.bump(r.severity_class);
            if matches!(r.severity_class, SeverityClass::Critical) {
                critical_safety_count += 1;
            }
            if posture.holds_attention_for(r.severity_class) {
                held_under_posture_count += 1;
            }
            if r.minted_at > latest_minted_at {
                latest_minted_at = r.minted_at.clone();
            }
        }

        let os_app_icon_badge_visible = !posture.suppresses_os_app_icon_badge();
        let lock_screen_summary_visible = !posture.suppresses_lock_screen_summary();
        let privacy_safe_summary_label =
            privacy_safe_summary_label(durable_count, critical_safety_count);

        Self {
            record_kind: DURABLE_BADGE_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: DURABLE_BADGE_PROJECTION_SCHEMA_VERSION,
            durable_count,
            severity_counts,
            critical_safety_count,
            held_under_posture_count,
            active_modes: posture.active_modes_sorted(),
            os_app_icon_badge_visible,
            lock_screen_summary_visible,
            privacy_safe_summary_label,
            minted_at: latest_minted_at,
        }
    }
}

/// True when the routed notification has at least one durable surface route.
/// Durable surfaces (durable_job_row, activity_center_digest_card,
/// digest_group_row) preserve truth across reloads; only those rows
/// contribute to the badge count so transient toasts and OS notifications
/// never inflate it.
fn has_durable_surface(r: &RoutedNotification) -> bool {
    r.surface_routes.iter().any(|route| {
        matches!(
            route.fanout_surface_class,
            FanoutSurfaceClass::DurableJobRow
                | FanoutSurfaceClass::ActivityCenterDigestCard
                | FanoutSurfaceClass::DigestGroupRow
        )
    })
}

/// Per the upstream `quiet_hours_policy_matrix.yaml`, these modes suppress
/// `os_badge_app_icon`. Focus-mode, reduced-attention, and power-saver
/// preserve the OS badge so the user can still glance at durable truth.
fn mode_suppresses_os_app_icon_badge(mode: QuietHoursMode) -> bool {
    matches!(
        mode,
        QuietHoursMode::ModeQuietHoursUser
            | QuietHoursMode::ModeDoNotDisturbUser
            | QuietHoursMode::ModePresentation
            | QuietHoursMode::ModeScreenShare
            | QuietHoursMode::ModePrivacyMode
            | QuietHoursMode::ModeAdminSuppression
    )
}

/// Per the upstream `quiet_hours_policy_matrix.yaml`, these modes suppress
/// `lock_screen_summary`. Privacy mode denies lock-screen renders outright;
/// quiet hours / DND / presentation / screen-share / admin all hold or
/// suppress the surface.
fn mode_suppresses_lock_screen_summary(mode: QuietHoursMode) -> bool {
    matches!(
        mode,
        QuietHoursMode::ModeQuietHoursUser
            | QuietHoursMode::ModeDoNotDisturbUser
            | QuietHoursMode::ModeFocusModeUser
            | QuietHoursMode::ModePresentation
            | QuietHoursMode::ModeScreenShare
            | QuietHoursMode::ModePrivacyMode
            | QuietHoursMode::ModeAdminSuppression
    )
}

/// Critical severity (tier_critical_safety in upstream taxonomy) bypasses
/// every quiet-hours mode. The upstream contract is explicit: a managed
/// admin suppression that silently blocks a critical-safety delivery is
/// non-conforming.
fn severity_always_interrupts(severity: SeverityClass) -> bool {
    matches!(severity, SeverityClass::Critical)
}

/// Map a quiet-hours mode to its canonical [`SuppressionReason`]. Modes
/// that do not contribute their own reason (e.g., `mode_none`) return
/// `None`.
fn suppression_reason_for_mode(mode: QuietHoursMode) -> Option<SuppressionReason> {
    match mode {
        QuietHoursMode::ModeNone => None,
        QuietHoursMode::ModeQuietHoursUser => Some(SuppressionReason::QuietHoursUserPolicy),
        QuietHoursMode::ModeDoNotDisturbUser => Some(SuppressionReason::DoNotDisturbUserPolicy),
        QuietHoursMode::ModeFocusModeUser => Some(SuppressionReason::FocusModeUserPolicy),
        QuietHoursMode::ModePresentation => Some(SuppressionReason::PresentationModeActive),
        QuietHoursMode::ModeScreenShare => Some(SuppressionReason::ScreenShareActive),
        QuietHoursMode::ModePrivacyMode => Some(SuppressionReason::PrivacyModeActive),
        QuietHoursMode::ModeReducedAttentionPolicy => {
            Some(SuppressionReason::ReducedAttentionPosture)
        }
        QuietHoursMode::ModePowerSaverRuntime => Some(SuppressionReason::PowerSaverBackgroundPause),
        QuietHoursMode::ModeAdminSuppression => Some(SuppressionReason::AdminPolicySuppression),
    }
}

/// Privacy-safe summary label. Carries category-class tokens and counts
/// only. NEVER includes raw paths, raw URLs, actor identities, or
/// workspace identifiers — that rule comes from the upstream
/// `privacy_safe_payload_rule_record` set in
/// `os_notification_and_quiet_hours_contract.md`.
fn privacy_safe_summary_label(durable_count: u32, critical_safety_count: u32) -> String {
    match (durable_count, critical_safety_count) {
        (0, _) => "No background items".to_owned(),
        (1, 0) => "1 background item".to_owned(),
        (1, 1) => "1 background item, 1 critical".to_owned(),
        (n, 0) => format!("{n} background items"),
        (n, 1) => format!("{n} background items, 1 critical"),
        (n, c) => format!("{n} background items, {c} critical"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notifications::envelope::{
        DedupeKeyScheme, NotificationEnvelope, PrivacyClass, PrivacyPayloadClass, RedactionClass,
        ReopenTarget, ReopenTargetKind, SourceSubsystem, StableAction, SuppressionState,
        NOTIFICATION_ENVELOPE_SCHEMA_VERSION,
    };
    use crate::notifications::router::NotificationRouter;

    fn baseline_envelope() -> NotificationEnvelope {
        NotificationEnvelope {
            record_kind: "notification_envelope_record".into(),
            notification_envelope_schema_version: NOTIFICATION_ENVELOPE_SCHEMA_VERSION,
            notification_envelope_id: "ux:notif-env:test:01".into(),
            canonical_event_id: "ux:event:test:01".into(),
            event_lineage_id_ref: "ux:lineage:test:01".into(),
            source_subsystem: SourceSubsystem::Indexer,
            source_event_ref: "test:event:01".into(),
            actor_identity_ref: "id:actor:system:test".into(),
            canonical_object_target_ref: "obj:test:01".into(),
            severity_class: SeverityClass::Warning,
            privacy_class: PrivacyClass::WorkspaceSensitive,
            privacy_payload_class: PrivacyPayloadClass::LockScreenSafeGeneric,
            redaction_class: RedactionClass::OperatorOnlyRestricted,
            dedupe_key_scheme: DedupeKeyScheme::CanonicalEventId,
            dedupe_key_ref: "ux:event:test:01".into(),
            grouped_burst_id_ref: None,
            recommended_surfaces: vec![
                FanoutSurfaceClass::DurableJobRow,
                FanoutSurfaceClass::StatusItem,
                FanoutSurfaceClass::Toast,
            ],
            summary_label: "Test event".into(),
            reopen_target: ReopenTarget {
                reopen_target_ref: "ux:reopen:test:01".into(),
                reopen_target_kind: ReopenTargetKind::DurableActivityRow,
                exact_target_identity_ref: Some("obj:test:01".into()),
                placeholder_announcement_label: None,
                revalidation_required_reason_label: None,
            },
            actions: vec![StableAction {
                action_id: "ux:action:test:open:01".into(),
                label: "Open".into(),
                command_id: "cmd:test.open".into(),
                target_identity_ref: "obj:test:01".into(),
                reopen_target_kind: ReopenTargetKind::DurableActivityRow,
                is_destructive: false,
            }],
            suppression_state: SuppressionState {
                active_modes_at_mint: vec![QuietHoursMode::ModeNone],
                suppression_reasons: vec![],
                suppressed: false,
            },
            fanout_receipts: vec![],
            minted_at: "2026-05-10T10:00:00Z".into(),
        }
    }

    #[test]
    fn no_quiet_mode_leaves_envelope_unchanged() {
        let posture = QuietHoursPosture::none();
        let mut env = baseline_envelope();
        let changed = posture.apply_to_envelope(&mut env);
        assert!(!changed);
        assert!(!env.suppression_state.suppressed);
        assert_eq!(
            env.suppression_state.active_modes_at_mint,
            vec![QuietHoursMode::ModeNone]
        );
    }

    #[test]
    fn quiet_hours_user_holds_warning_severity_envelope() {
        let posture = QuietHoursPosture::quiet_hours_user();
        let mut env = baseline_envelope();
        assert_eq!(env.severity_class, SeverityClass::Warning);
        let changed = posture.apply_to_envelope(&mut env);
        assert!(changed);
        assert!(env.suppression_state.suppressed);
        assert!(env
            .suppression_state
            .active_modes_at_mint
            .contains(&QuietHoursMode::ModeQuietHoursUser));
        assert!(env
            .suppression_state
            .suppression_reasons
            .contains(&SuppressionReason::QuietHoursUserPolicy));
    }

    #[test]
    fn critical_severity_bypasses_every_quiet_mode() {
        for posture in [
            QuietHoursPosture::quiet_hours_user(),
            QuietHoursPosture::do_not_disturb(),
            QuietHoursPosture::presentation(),
            QuietHoursPosture::privacy_mode(),
            QuietHoursPosture::admin_suppression(),
        ] {
            let mut env = baseline_envelope();
            env.severity_class = SeverityClass::Critical;
            posture.apply_to_envelope(&mut env);
            assert!(
                !env.suppression_state.suppressed,
                "critical severity must never be held: posture {posture:?}"
            );
            assert!(
                env.suppression_state.suppression_reasons.is_empty(),
                "critical severity must not record suppression reasons"
            );
        }
    }

    #[test]
    fn blocking_severity_bypasses_user_modes_but_admin_holds_it() {
        // Quiet-hours-user must NOT hold blocking-trust severity.
        let user = QuietHoursPosture::quiet_hours_user();
        let mut env = baseline_envelope();
        env.severity_class = SeverityClass::Blocking;
        user.apply_to_envelope(&mut env);
        assert!(!env.suppression_state.suppressed);

        // Admin suppression DOES hold blocking severity (managed posture).
        let admin = QuietHoursPosture::admin_suppression();
        let mut env = baseline_envelope();
        env.severity_class = SeverityClass::Blocking;
        admin.apply_to_envelope(&mut env);
        assert!(env.suppression_state.suppressed);
        assert!(env
            .suppression_state
            .suppression_reasons
            .contains(&SuppressionReason::AdminPolicySuppression));
    }

    #[test]
    fn posture_unions_with_existing_envelope_modes() {
        let posture = QuietHoursPosture::with_modes([QuietHoursMode::ModePresentation]);
        let mut env = baseline_envelope();
        env.suppression_state = SuppressionState {
            active_modes_at_mint: vec![QuietHoursMode::ModeQuietHoursUser],
            suppression_reasons: vec![SuppressionReason::QuietHoursUserPolicy],
            suppressed: true,
        };
        posture.apply_to_envelope(&mut env);
        let modes: BTreeSet<_> = env
            .suppression_state
            .active_modes_at_mint
            .iter()
            .copied()
            .collect();
        assert!(modes.contains(&QuietHoursMode::ModeQuietHoursUser));
        assert!(modes.contains(&QuietHoursMode::ModePresentation));
    }

    #[test]
    fn os_app_icon_badge_suppressed_under_quiet_hours_user() {
        let posture = QuietHoursPosture::quiet_hours_user();
        assert!(posture.suppresses_os_app_icon_badge());
        assert!(posture.suppresses_lock_screen_summary());
    }

    #[test]
    fn os_app_icon_badge_visible_under_focus_mode() {
        // Focus mode preserves the app-icon badge per the policy matrix.
        let posture = QuietHoursPosture::focus_mode();
        assert!(!posture.suppresses_os_app_icon_badge());
    }

    #[test]
    fn badge_projection_dedupes_across_repeat_emissions() {
        let mut router = NotificationRouter::new();
        let env = baseline_envelope();
        let r1 = router.route(&env).unwrap();
        let r2 = router.route(&env).unwrap();
        let r3 = router.route(&env).unwrap();
        let projection =
            DurableBadgeProjection::from_routed(&[r1, r2, r3], &QuietHoursPosture::none());
        // Three emissions of the same canonical event collapse to one
        // durable item — the badge does NOT inflate.
        assert_eq!(projection.durable_count, 1);
        assert_eq!(projection.severity_counts.warning, 1);
        assert_eq!(projection.critical_safety_count, 0);
        assert!(projection.os_app_icon_badge_visible);
    }

    #[test]
    fn badge_projection_counts_critical_safety_separately() {
        let mut router = NotificationRouter::new();
        let mut warn = baseline_envelope();
        warn.canonical_event_id = "ux:event:warn:01".into();
        warn.dedupe_key_ref = warn.canonical_event_id.clone();
        warn.notification_envelope_id = "ux:notif-env:warn:01".into();
        let mut crit = baseline_envelope();
        crit.canonical_event_id = "ux:event:crit:01".into();
        crit.dedupe_key_ref = crit.canonical_event_id.clone();
        crit.notification_envelope_id = "ux:notif-env:crit:01".into();
        crit.severity_class = SeverityClass::Critical;
        crit.privacy_class = PrivacyClass::SecurityCritical;
        crit.privacy_payload_class = PrivacyPayloadClass::RedactedMetadataOnly;
        crit.redaction_class = RedactionClass::InternalSupportRestricted;
        let r1 = router.route(&warn).unwrap();
        let r2 = router.route(&crit).unwrap();

        // Even with quiet hours active, the projection still reports the
        // critical-safety subcount so a glance surface knows what must show.
        let posture = QuietHoursPosture::quiet_hours_user();
        let projection = DurableBadgeProjection::from_routed(&[r1, r2], &posture);
        assert_eq!(projection.durable_count, 2);
        assert_eq!(projection.severity_counts.warning, 1);
        assert_eq!(projection.severity_counts.critical, 1);
        assert_eq!(projection.critical_safety_count, 1);
        assert_eq!(projection.held_under_posture_count, 1); // only the warning is held
        assert!(!projection.os_app_icon_badge_visible);
        assert_eq!(
            projection.privacy_safe_summary_label,
            "2 background items, 1 critical"
        );
    }

    #[test]
    fn badge_projection_skips_routed_notifications_without_durable_surfaces() {
        let mut router = NotificationRouter::new();
        let mut env = baseline_envelope();
        // Drop the durable_job_row — only ambient surfaces remain.
        env.recommended_surfaces = vec![FanoutSurfaceClass::Toast, FanoutSurfaceClass::StatusItem];
        let r = router.route(&env).unwrap();
        let projection = DurableBadgeProjection::from_routed(&[r], &QuietHoursPosture::none());
        assert_eq!(projection.durable_count, 0);
        assert_eq!(projection.privacy_safe_summary_label, "No background items");
    }

    #[test]
    fn privacy_safe_summary_label_never_names_objects() {
        // Sanity check that the formatter never echoes an object/actor ref.
        let posture = QuietHoursPosture::privacy_mode();
        let mut env = baseline_envelope();
        env.canonical_object_target_ref = "obj:secret-project:01".into();
        env.actor_identity_ref = "id:actor:user:secret-collaborator".into();
        env.summary_label = "Secret review".into();
        let mut router = NotificationRouter::new();
        let r = router.route(&env).unwrap();
        let projection = DurableBadgeProjection::from_routed(&[r], &posture);
        let label = &projection.privacy_safe_summary_label;
        assert!(!label.contains("secret-project"));
        assert!(!label.contains("secret-collaborator"));
        assert!(!label.contains("Secret review"));
    }

    #[test]
    fn applied_posture_routes_through_router_as_held_quiet_hours() {
        // End-to-end: shell posture applies to envelope, router still
        // delivers durable surface but holds the toast.
        let posture = QuietHoursPosture::quiet_hours_user();
        let mut env = baseline_envelope();
        posture.apply_to_envelope(&mut env);
        let mut router = NotificationRouter::new();
        let routed = router.route(&env).unwrap();

        let mut by_surface = std::collections::HashMap::new();
        for r in &routed.surface_routes {
            by_surface.insert(r.fanout_surface_class, r.receipt_state);
        }
        assert_eq!(
            by_surface.get(&FanoutSurfaceClass::DurableJobRow),
            Some(&super::super::envelope::FanoutReceiptState::Delivered)
        );
        assert_eq!(
            by_surface.get(&FanoutSurfaceClass::Toast),
            Some(&super::super::envelope::FanoutReceiptState::HeldQuietHours)
        );
    }
}
