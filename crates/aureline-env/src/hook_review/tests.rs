use std::collections::BTreeSet;

use super::seed::{seeded_hook_review_drills, seeded_hook_review_fixtures, seeded_lifecycle_hooks};
use super::*;
use crate::capsules::{CapsuleTargetClass, TrustGateState};
use crate::m5_env_governance::EnvironmentProfile;

fn trusted_context() -> HookReviewContext {
    HookReviewContext {
        profile: EnvironmentProfile::Starter,
        target_class: CapsuleTargetClass::Local,
        trusted: true,
        restricted_mode: false,
        supported_activators: HookActivator::ALL.to_vec(),
        denied_activators: Vec::new(),
        denied_hook_kinds: Vec::new(),
        projected_secrets: vec!["DATABASE_URL".to_owned()],
        failed_actions: Vec::new(),
        policy_ref: "artifacts/policy/lifecycle_hook_policy.yaml".to_owned(),
        approval_lineage_ref: "artifacts/approvals/approval_lineage.yaml".to_owned(),
        support_export_ref: "crates/aureline-support/src/bundle/mod.rs".to_owned(),
    }
}

#[test]
fn every_seeded_hook_validates() {
    for hook in seeded_lifecycle_hooks() {
        validate_lifecycle_hook(&hook)
            .unwrap_or_else(|err| panic!("hook {} must validate: {err}", hook.hook_id));
    }
}

#[test]
fn every_seeded_fixture_validates() {
    for fixture in seeded_hook_review_fixtures() {
        validate_hook_review_fixture(&fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn every_seeded_drill_validates() {
    for drill in seeded_hook_review_drills() {
        validate_hook_review_drill(&drill)
            .unwrap_or_else(|err| panic!("drill {} must validate: {err}", drill.drill_id));
    }
}

#[test]
fn trusted_unrestricted_review_clears_every_hook() {
    let hooks = seeded_lifecycle_hooks();
    let packet = review_hooks(&hooks, &trusted_context());
    assert_eq!(packet.posture, HookReviewPosture::AllCleared);
    assert_eq!(packet.runnable_hooks as usize, hooks.len());
    assert!(packet.repairs.is_empty());
    assert!(packet.reason_tokens.is_empty());
    assert_eq!(packet.trust_hooks_evidence_state, EvidenceState::Current);
    assert!(packet
        .entries
        .iter()
        .all(|e| e.runnable && e.repair.is_none()));
}

#[test]
fn ungated_hook_is_never_runnable_even_when_trusted() {
    // The marquee guardrail: an ungated hook is held for review, never run
    // merely because the workspace is trusted.
    let mut hooks = seeded_lifecycle_hooks();
    hooks
        .iter_mut()
        .find(|h| h.hook_id == "hook.nix_activate")
        .expect("nix hook")
        .gate_state = TrustGateState::Ungated;
    let packet = review_hooks(&hooks, &trusted_context());
    let entry = packet
        .entries
        .iter()
        .find(|e| e.hook_id == "hook.nix_activate")
        .expect("nix entry");
    assert!(!entry.runnable);
    assert_eq!(entry.disposition, HookDisposition::ReviewRequired);
    assert_eq!(entry.hold_reason, Some(HookHoldReason::UngatedAuthority));
    assert_eq!(packet.posture, HookReviewPosture::ReviewPending);
}

#[test]
fn restricted_mode_holds_auto_run_but_keeps_a_safe_subset_statement() {
    let hooks = seeded_lifecycle_hooks();
    let mut context = trusted_context();
    context.restricted_mode = true;
    let packet = review_hooks(&hooks, &context);
    assert_eq!(packet.posture, HookReviewPosture::ReviewPending);
    assert_eq!(packet.runnable_hooks, 0);
    assert!(packet
        .entries
        .iter()
        .all(|e| e.disposition == HookDisposition::Restricted));
    assert!(packet
        .reason_tokens
        .contains(&HookHoldReason::RestrictedMode.as_str().to_owned()));
    // Each held hook is offered a manual-run repair with a next step.
    for repair in &packet.repairs {
        assert_eq!(repair.repair_kind, RepairKind::RunManuallyAfterReview);
        assert!(!repair.next_step.is_empty());
    }
}

#[test]
fn policy_denied_activator_is_surfaced_and_safe_subset_runs() {
    let hooks = seeded_lifecycle_hooks();
    let mut context = trusted_context();
    context.denied_activators = vec![HookActivator::Nix];
    let packet = review_hooks(&hooks, &context);
    assert_eq!(packet.posture, HookReviewPosture::PartiallyBlocked);
    assert_eq!(
        packet.denied_hook_tokens,
        vec!["hook.nix_activate".to_owned()]
    );
    assert!(packet.runnable_hooks > 0, "the safe subset still runs");
    let entry = packet
        .entries
        .iter()
        .find(|e| e.hook_id == "hook.nix_activate")
        .expect("nix entry");
    assert_eq!(entry.disposition, HookDisposition::Denied);
    assert_eq!(entry.hold_reason, Some(HookHoldReason::PolicyDenied));
    assert!(entry.reason_tokens.contains(&"activator_nix".to_owned()));
    let repair = entry.repair.as_ref().expect("denied hook has a repair");
    assert_eq!(repair.repair_kind, RepairKind::RequestPolicyException);
}

#[test]
fn missing_secret_blocks_and_names_the_secret() {
    let hooks = seeded_lifecycle_hooks();
    let mut context = trusted_context();
    context.projected_secrets = Vec::new();
    let packet = review_hooks(&hooks, &context);
    let entry = packet
        .entries
        .iter()
        .find(|e| e.hook_id == "hook.post_create_seed")
        .expect("seed entry");
    assert_eq!(entry.disposition, HookDisposition::Blocked);
    assert_eq!(entry.hold_reason, Some(HookHoldReason::MissingSecret));
    assert!(entry
        .reason_tokens
        .contains(&"secret_DATABASE_URL".to_owned()));
}

#[test]
fn unsupported_activator_is_blocked_not_dropped() {
    let hooks = seeded_lifecycle_hooks();
    let mut context = trusted_context();
    context.supported_activators = HookActivator::ALL
        .into_iter()
        .filter(|a| *a != HookActivator::Nix)
        .collect();
    let packet = review_hooks(&hooks, &context);
    let entry = packet
        .entries
        .iter()
        .find(|e| e.hook_id == "hook.nix_activate")
        .expect("nix entry");
    assert_eq!(entry.disposition, HookDisposition::Blocked);
    assert_eq!(
        entry.hold_reason,
        Some(HookHoldReason::UnsupportedActivator)
    );
}

#[test]
fn failed_bootstrap_cascades_to_the_dependent_post_create_hook() {
    let hooks = seeded_lifecycle_hooks();
    let mut context = trusted_context();
    context.failed_actions = vec!["hook.bootstrap".to_owned()];
    let packet = review_hooks(&hooks, &context);
    let bootstrap = packet
        .entries
        .iter()
        .find(|e| e.hook_id == "hook.bootstrap")
        .expect("bootstrap entry");
    assert_eq!(bootstrap.hold_reason, Some(HookHoldReason::BootstrapFailed));
    let post_create = packet
        .entries
        .iter()
        .find(|e| e.hook_id == "hook.devcontainer_post_create")
        .expect("post-create entry");
    assert_eq!(post_create.disposition, HookDisposition::Blocked);
    assert_eq!(
        post_create.hold_reason,
        Some(HookHoldReason::UpstreamBlocked)
    );
    assert!(post_create
        .reason_tokens
        .contains(&"upstream_hook.bootstrap".to_owned()));
    assert!(packet
        .blocked_hook_tokens
        .contains(&"hook.devcontainer_post_create".to_owned()));
}

#[test]
fn fully_blocked_when_every_activator_is_denied() {
    let hooks = seeded_lifecycle_hooks();
    let mut context = trusted_context();
    context.denied_activators = HookActivator::ALL.to_vec();
    let packet = review_hooks(&hooks, &context);
    assert_eq!(packet.posture, HookReviewPosture::FullyBlocked);
    assert_eq!(packet.runnable_hooks, 0);
    assert_eq!(packet.trust_hooks_evidence_state, EvidenceState::Missing);
    assert!(packet.what_still_works.to_lowercase().contains("no hook"));
}

#[test]
fn desktop_headless_ai_and_support_share_one_object() {
    let hooks = seeded_lifecycle_hooks();
    let context = trusted_context();
    let desktop = desktop_hook_review(&hooks, &context);
    let headless = headless_hook_review(&hooks, &context);
    let ai = ai_hook_review(&hooks, &context);
    let support = support_hook_review(&hooks, &context);
    assert_eq!(desktop, headless);
    assert_eq!(desktop, ai);
    assert_eq!(support.packet, desktop);
}

#[test]
fn export_is_metadata_first_and_preserves_repairs() {
    let hooks = seeded_lifecycle_hooks();
    let mut context = trusted_context();
    context.denied_activators = vec![HookActivator::Nix];
    let packet = review_hooks(&hooks, &context);
    let export = export_hook_review(&packet);
    assert_eq!(export.redaction_class, RedactionClass::MetadataOnly);
    assert_eq!(export.repairs, packet.repairs);
    assert_eq!(export.packet, packet);
    // Hook identity, reason, and next step survive the export verbatim.
    for repair in &export.repairs {
        assert!(!repair.hook_id.is_empty());
        assert!(!repair.next_step.is_empty());
    }
}

#[test]
fn fixtures_cover_every_posture_and_disposition() {
    let fixtures = seeded_hook_review_fixtures();
    let postures: BTreeSet<HookReviewPosture> = fixtures.iter().map(|f| f.packet.posture).collect();
    for required in HookReviewPosture::ALL {
        assert!(
            postures.contains(&required),
            "fixtures must cover {required:?}"
        );
    }
    let dispositions: BTreeSet<HookDisposition> = fixtures
        .iter()
        .flat_map(|f| f.packet.entries.iter().map(|e| e.disposition))
        .collect();
    for required in HookDisposition::ALL {
        assert!(
            dispositions.contains(&required),
            "fixtures must exercise disposition {required:?}"
        );
    }
}

#[test]
fn drills_cover_the_required_failure_reasons() {
    let drills = seeded_hook_review_drills();
    let reasons: BTreeSet<HookHoldReason> = drills.iter().map(|d| d.injected_reason).collect();
    for required in [
        HookHoldReason::UpstreamBlocked,
        HookHoldReason::BootstrapFailed,
        HookHoldReason::MissingSecret,
        HookHoldReason::UnsupportedActivator,
        HookHoldReason::PolicyDenied,
    ] {
        assert!(
            reasons.contains(&required),
            "drills must cover {required:?}"
        );
    }
    for drill in &drills {
        assert_eq!(drill.recovers_to_posture, HookReviewPosture::AllCleared);
    }
}

#[test]
fn packet_round_trips_through_json() {
    let packet = review_hooks(&seeded_lifecycle_hooks(), &trusted_context());
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let back: HookReviewPacket = serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(packet, back);
}
