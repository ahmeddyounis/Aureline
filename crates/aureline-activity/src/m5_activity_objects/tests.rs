//! Unit tests for the durable activity object model, row rendering, archive /
//! expiry policy, invariants, and export-safety rules.

use super::*;

fn bundle() -> ActivityObjectsBundle {
    activity_objects_bundle()
}

fn object_for(slug: &str) -> ActivityObject {
    bundle()
        .object(&format!("activity_job:{slug}:0001"))
        .unwrap_or_else(|| panic!("object {slug} present"))
        .clone()
}

#[test]
fn bundle_validates_and_all_invariants_hold() {
    let b = bundle();
    b.validate().expect("canonical bundle validates");
    assert!(b.all_invariants_hold());
    assert!(!b.invariants.is_empty());
}

#[test]
fn bundle_is_deterministic() {
    assert_eq!(activity_objects_bundle(), activity_objects_bundle());
}

#[test]
fn bundle_is_support_export_safe() {
    let b = bundle();
    assert!(b.raw_payload_excluded);
    assert!(b.is_support_export_safe());
}

#[test]
fn every_family_has_a_durable_object() {
    let b = bundle();
    for family in JobFamilyClass::ALL {
        let entry = b
            .family(family)
            .unwrap_or_else(|| panic!("family {} has an entry", family.as_str()));
        assert!(!entry.spinner_or_toast_only);
        assert!(
            b.objects.iter().any(|o| o.job_family == family),
            "family {} has an object",
            family.as_str()
        );
    }
}

#[test]
fn every_object_is_durable_and_reopen_safe() {
    let b = bundle();
    for o in &b.objects {
        assert!(o.durable);
        assert!(o.survives_focus_change);
        assert!(!o.toast_only);
        assert!(o.can_open_details);
        assert!(!o.evidence_refs.is_empty());
        assert!(!o.reopen_anchor.object_ref.is_empty());
    }
}

#[test]
fn each_object_has_exactly_one_reproducible_row() {
    let b = bundle();
    assert_eq!(b.rows.len(), b.objects.len());
    for o in &b.objects {
        let row = b.row(&o.activity_job_id).expect("row present");
        assert_eq!(&render_row(o), row);
        assert_eq!(row.activity_job_id, o.activity_job_id);
        assert_eq!(row.reopen_target_id, o.reopen_anchor.reopen_target.as_str());
    }
}

#[test]
fn phase_is_derived_from_progress_state() {
    let b = bundle();
    for o in &b.objects {
        assert_eq!(o.progress.phase, phase_for(o.progress.progress_state));
    }
}

#[test]
fn archive_state_recomputes_from_retention_policy() {
    let b = bundle();
    for o in &b.objects {
        assert_eq!(
            o.archive_state,
            archive_state_for(o.progress.progress_state, &o.retention, o.age_days)
        );
    }
}

#[test]
fn archive_expiry_thresholds_apply() {
    // A recent failure is retained as active.
    let failed = object_for("task.failed");
    assert_eq!(failed.progress.progress_state, AttentionStateClass::Failed);
    assert_eq!(failed.archive_state, ArchiveStateClass::Active);

    // A completed sync past the archive horizon is archived.
    let archived = object_for("sync.backup");
    assert_eq!(archived.archive_state, ArchiveStateClass::Archived);
    assert!(archived.age_days >= archived.retention.archive_after_days);
    assert!(archived.age_days < archived.retention.expire_after_days);

    // A completed offboarding past the expiry horizon is expired.
    let expired = object_for("offboarding.export");
    assert_eq!(expired.archive_state, ArchiveStateClass::Expired);
    assert!(expired.age_days >= expired.retention.expire_after_days);
    assert!(!expired.archive_state.retains_full_record());
}

#[test]
fn completion_and_failure_history_is_retained() {
    let b = bundle();
    for o in &b.objects {
        if is_terminal_for_retention(o.progress.progress_state)
            && o.age_days < o.retention.archive_after_days
        {
            assert_eq!(
                o.archive_state,
                ArchiveStateClass::Active,
                "recent terminal {} must stay active",
                o.activity_job_id
            );
        }
    }
    // The corpus exercises all three retention dispositions.
    assert!(b
        .objects
        .iter()
        .any(|o| o.archive_state == ArchiveStateClass::Active));
    assert!(b
        .objects
        .iter()
        .any(|o| o.archive_state == ArchiveStateClass::Archived));
    assert!(b
        .objects
        .iter()
        .any(|o| o.archive_state == ArchiveStateClass::Expired));
}

#[test]
fn affordances_match_family_and_state() {
    // A failed retryable task offers retry, not cancel.
    let failed = object_for("task.failed");
    assert!(failed.can_retry);
    assert!(!failed.can_cancel);
    assert!(failed
        .affordances
        .contains(&ActivityAffordanceClass::OpenDetails));

    // A running notebook offers cancel, not retry.
    let running = object_for("notebook.run");
    assert!(running.can_cancel);
    assert!(!running.can_retry);

    // A failed offboarding-style non-retryable family would not offer retry — proven
    // here via the operator handoff (non-retryable) awaiting review.
    let handoff = object_for("operator.handoff");
    assert!(!handoff.can_retry);
    assert!(handoff
        .affordances
        .contains(&ActivityAffordanceClass::ReviewApprove));
}

#[test]
fn archive_state_is_shared_across_every_surface() {
    let b = bundle();
    for r in &b.rows {
        assert!(!r.surface_projections.is_empty());
        for p in &r.surface_projections {
            assert_eq!(
                p.archive_state, r.archive_state,
                "row {} surface {} archive state diverged",
                r.activity_job_id, p.consumer_token
            );
        }
    }
}

#[test]
fn row_is_shared_across_desktop_support_and_companion() {
    // A workspace-private completed preview route renders on desktop, CLI, support
    // export, and the companion (redacted), proving the row is shared across
    // clients.
    let b = bundle();
    let row = b.row("activity_job:preview.route:0001").expect("row");
    for consumer in [
        AttentionConsumerClass::ShellActivityCenter,
        AttentionConsumerClass::CliHeadless,
        AttentionConsumerClass::SupportExport,
        AttentionConsumerClass::CompanionCrossClient,
    ] {
        let p = row.projection(consumer).expect("projection present");
        assert!(p.included, "{} must include the row", p.consumer_token);
    }
}

#[test]
fn managed_sensitive_rows_never_reach_the_companion() {
    let b = bundle();
    for o in &b.objects {
        if o.privacy_class == NotificationPrivacyClass::ManagedSensitive {
            let row = b.row(&o.activity_job_id).expect("row");
            let companion = row
                .projection(AttentionConsumerClass::CompanionCrossClient)
                .expect("companion projection");
            assert!(
                !companion.included,
                "managed-sensitive {} must not reach the companion",
                o.activity_job_id
            );
        }
    }
}

#[test]
fn privacy_never_widens_on_a_surface() {
    let b = bundle();
    for r in &b.rows {
        let o = b.object(&r.activity_job_id).expect("object");
        for p in &r.surface_projections {
            assert!(
                redaction_rank(p.applied_redaction) >= redaction_rank(o.default_redaction),
                "surface {} widened past the object default for {}",
                p.consumer_token,
                o.activity_job_id
            );
        }
    }
}

#[test]
fn non_shell_surfaces_offer_open_details_only() {
    let b = bundle();
    for r in &b.rows {
        for p in &r.surface_projections {
            if p.consumer == AttentionConsumerClass::ShellActivityCenter {
                continue;
            }
            assert_eq!(
                p.shown_affordances,
                vec![ActivityAffordanceClass::OpenDetails],
                "surface {} must reopen rather than act inline",
                p.consumer_token
            );
        }
    }
}

#[test]
fn badge_counts_only_durable_pending_rows() {
    let b = bundle();
    for r in &b.rows {
        let o = b.object(&r.activity_job_id).expect("object");
        assert_eq!(r.badge_counts_toward, o.badge_bearing());
        if r.badge_counts_toward {
            assert!(o.durable && !o.toast_only);
            assert_eq!(o.archive_state, ArchiveStateClass::Active);
            assert!(is_attention_pending(o.progress.progress_state));
        }
    }
    // An archived completed row never counts toward the badge.
    let archived = object_for("sync.backup");
    assert!(!archived.badge_bearing());
}

#[test]
fn progress_states_and_reopen_targets_are_matrix_bound() {
    use crate::m5_attention_routing::{attention_routing_matrix, AttentionObjectClass};
    let matrix = attention_routing_matrix();
    let entry = matrix
        .object(AttentionObjectClass::ActivityObject)
        .expect("activity object entry");
    let b = bundle();
    for o in &b.objects {
        assert!(entry.can_show(o.progress.progress_state));
        assert!(entry.can_reopen(o.reopen_anchor.reopen_target));
        assert_eq!(
            o.retention.retention_class,
            entry.retention_rule.retention_class
        );
    }
}

#[test]
fn validate_rejects_a_raw_payload_flag_flip() {
    let mut b = bundle();
    b.raw_payload_excluded = false;
    assert!(b.validate().is_err());
    assert!(!b.is_support_export_safe());
}

#[test]
fn validate_rejects_an_unsafe_ref() {
    let mut b = bundle();
    b.objects[0].scope_ref = "https://internal.example.com/scope".to_owned();
    assert!(!b.is_support_export_safe());
    assert!(b.validate().is_err());
}

#[test]
fn validate_rejects_a_toast_only_object() {
    let mut b = bundle();
    b.objects[0].toast_only = true;
    assert!(b.validate().is_err());
}

#[test]
fn validate_rejects_an_inconsistent_archive_state() {
    let mut b = bundle();
    b.objects[0].archive_state = ArchiveStateClass::Expired;
    assert!(b.validate().is_err());
}

#[test]
fn validate_rejects_a_missing_evidence_link() {
    let mut b = bundle();
    b.objects[0].evidence_refs.clear();
    assert!(b.validate().is_err());
}

#[test]
fn human_readable_projection_renders() {
    let b = bundle();
    let lines = activity_objects_lines(&b);
    assert!(lines.iter().any(|l| l.contains("Activity-objects bundle")));
    assert!(lines.iter().any(|l| l.contains("Families:")));
    assert!(lines.iter().any(|l| l.contains("Objects:")));
    assert!(lines.iter().any(|l| l.contains("Rows:")));
    for family in JobFamilyClass::ALL {
        assert!(
            lines.iter().any(|l| l.contains(family.as_str())),
            "projection must mention family {}",
            family.as_str()
        );
    }
}
