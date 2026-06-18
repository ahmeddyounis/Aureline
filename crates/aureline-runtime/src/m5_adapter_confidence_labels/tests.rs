//! Inline unit coverage for the adapter-confidence audit: the seed validates
//! clean, the surface bindings keep source class and confidence distinct, the
//! no-lower-confidence-overwrite arbitration blocks weaker re-reports, and the
//! source-quality-change vocabulary survives the support / CLI / AI projections.

use super::*;
use crate::build_test_event_interoperability::{
    BuildTestEventConfidence, BuildTestEventSourceKind, BuildTestInteropPromotionState,
};

fn seed() -> AdapterConfidenceAudit {
    seeded_adapter_confidence_audit()
}

#[test]
fn seed_validates_clean_and_is_stable() {
    let audit = seed();
    assert!(
        audit.validate().is_empty(),
        "seed must validate clean: {:?}",
        audit
            .validate()
            .iter()
            .map(|f| f.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        audit.promotion_state,
        BuildTestInteropPromotionState::Stable
    );
    assert!(validate_adapter_confidence_audit(&audit).is_ok());
}

#[test]
fn every_claimed_surface_has_a_binding() {
    let audit = seed();
    let tokens = audit.surface_tokens();
    for surface in ConfidenceLabelSurface::ALL {
        assert!(
            tokens.contains(&surface.as_str()),
            "missing binding for {}",
            surface.as_str()
        );
    }
}

#[test]
fn seed_exercises_every_source_quality_change() {
    let audit = seed();
    let tokens = audit.source_quality_change_tokens();
    for change in SourceQualityChange::ALL {
        assert!(
            tokens.contains(&change.as_str()),
            "seed must exercise {}",
            change.as_str()
        );
    }
}

#[test]
fn label_keeps_source_and_confidence_separate() {
    let label = ConfidenceLabel::new(
        BuildTestEventSourceKind::Native,
        BuildTestEventConfidence::High,
    );
    assert_eq!(label.source_chip(), "native");
    assert_eq!(label.confidence_chip(), "high");
    assert!(!label.heuristic_fallback_banner);
    assert!(label.banner_text().is_none());
    assert!(label.is_authoritative());
}

#[test]
fn heuristic_label_always_carries_a_banner() {
    let label = ConfidenceLabel::new(
        BuildTestEventSourceKind::HeuristicParser,
        BuildTestEventConfidence::Low,
    );
    assert!(label.heuristic_fallback_banner);
    assert_eq!(
        label.fallback_reason,
        Some(DowngradeReason::HeuristicFallback)
    );
    assert!(label.banner_text().is_some());
    assert!(!label.is_authoritative());
}

#[test]
fn weaker_overwrite_attempt_is_blocked() {
    let audit = seed();
    let resolution = audit
        .subjects
        .iter()
        .find(|s| s.subject.subject_id == "subject:test:finish")
        .expect("test finish subject present");
    assert_eq!(
        resolution.current_authoritative_source,
        BuildTestEventSourceKind::Native
    );
    assert_eq!(
        resolution.source_quality_change,
        SourceQualityChange::OverwriteBlocked
    );
    let heuristic_decision = resolution
        .overwrite_decisions
        .iter()
        .find(|row| row.claim_id == "claim:test:finish:heuristic")
        .expect("heuristic decision present");
    assert_eq!(
        heuristic_decision.decision,
        OverwriteDecision::BlockedLowerConfidence
    );
    assert_eq!(
        heuristic_decision.reason,
        Some(OverwriteReason::WeakerSourceClass)
    );
}

#[test]
fn enrich_only_claim_is_kept_not_blocked() {
    let audit = seed();
    let resolution = audit
        .subjects
        .iter()
        .find(|s| s.subject.subject_id == "subject:coverage:artifact")
        .expect("coverage subject present");
    assert_eq!(
        resolution.source_quality_change,
        SourceQualityChange::EnrichedWithoutOverwrite
    );
    let heuristic_decision = resolution
        .overwrite_decisions
        .iter()
        .find(|row| row.claim_id == "claim:coverage:heuristic")
        .expect("heuristic decision present");
    assert_eq!(
        heuristic_decision.decision,
        OverwriteDecision::EnrichedContextOnly
    );
    assert_eq!(
        heuristic_decision.reason,
        Some(OverwriteReason::NeverClaimedAuthority)
    );
    // Lineage is retained, never dropped to resolve the conflict.
    assert_eq!(resolution.claims.len(), 2);
}

#[test]
fn heuristic_can_be_authoritative_but_flagged_on_downgrade() {
    let audit = seed();
    let resolution = audit
        .subjects
        .iter()
        .find(|s| s.subject.subject_id == "subject:pipeline:diagnostic")
        .expect("pipeline subject present");
    assert_eq!(
        resolution.current_authoritative_source,
        BuildTestEventSourceKind::HeuristicParser
    );
    assert_eq!(
        resolution.source_quality_change,
        SourceQualityChange::DowngradedToFallback
    );
    let claim = &resolution.claims[0];
    assert!(claim.label.heuristic_fallback_banner);
}

#[test]
fn accepted_lower_confidence_overwrite_blocks_stable() {
    let mut audit = seed();
    let resolution = audit
        .subjects
        .iter_mut()
        .find(|s| s.subject.subject_id == "subject:test:finish")
        .expect("test finish subject present");
    let decision = resolution
        .overwrite_decisions
        .iter_mut()
        .find(|row| row.claim_id == "claim:test:finish:heuristic")
        .expect("heuristic decision present");
    decision.decision = OverwriteDecision::EnrichedContextOnly;
    decision.reason = Some(OverwriteReason::NeverClaimedAuthority);
    audit.refresh_findings();

    assert!(!audit.is_stable());
    assert!(audit
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ConfidenceAuditFindingKind::LowerConfidenceOverwriteAccepted));
}

#[test]
fn collapsed_surface_badge_blocks_stable() {
    let mut audit = seed();
    audit.surface_bindings[0].keeps_source_and_confidence_distinct = false;
    audit.refresh_findings();
    assert!(
        audit
            .validation_findings
            .iter()
            .any(|f| f.finding_kind
                == ConfidenceAuditFindingKind::SurfaceCollapsesSourceAndConfidence)
    );
}

#[test]
fn support_export_round_trips_and_is_safe() {
    let audit = seed();
    let export = audit.support_export("support-export:test", "2026-06-17T00:01:00Z");
    assert!(export.is_export_safe());
    assert_eq!(export.audit, audit);
    let json = serde_json::to_string(&export).expect("serialize");
    let back: AdapterConfidenceAuditSupportExport =
        serde_json::from_str(&json).expect("round-trip");
    assert_eq!(back, export);
}

#[test]
fn cli_and_ai_views_preserve_label_and_lineage() {
    let audit = seed();
    let cli = audit.cli_headless_view("cli:test", "2026-06-17T00:01:00Z");
    assert!(cli.every_row_keeps_label());
    assert_eq!(cli.label_digest, audit.label_digest);

    let ai = audit.ai_evidence_view("ai:test", "2026-06-17T00:01:00Z");
    assert!(ai.keeps_lineage());
    assert_eq!(ai.label_digest, audit.label_digest);
    // The contested test-finish subject keeps both claims in evidence.
    let contested = ai
        .subjects
        .iter()
        .find(|s| s.subject_id == "subject:test:finish")
        .expect("contested subject present");
    assert_eq!(contested.claims.len(), 2);
}
