use super::*;

fn packet() -> M5MutationPathFixFlowPacket {
    frozen_m5_mutation_path_fix_flow_packet()
}

fn path<'a>(
    packet: &'a M5MutationPathFixFlowPacket,
    path: M5MutationPath,
) -> &'a M5MutationPathProjection {
    packet
        .paths
        .iter()
        .find(|p| p.path == path)
        .expect("path present")
}

#[test]
fn frozen_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn covers_every_mutation_path_once() {
    let packet = packet();
    assert!(packet.covers_all_paths());
    let present: BTreeSet<_> = packet.paths.iter().map(|p| p.path).collect();
    for p in M5MutationPath::ALL {
        assert!(present.contains(&p), "missing {}", p.as_str());
    }
}

#[test]
fn every_path_blocks_silent_byte_rewrite() {
    let packet = packet();
    assert!(packet.all_block_silent_byte_rewrite());
    for p in &packet.paths {
        assert!(p.silent_byte_rewrite_blocked);
        assert!(p.bytes_change_only_after_preview);
    }
}

#[test]
fn every_fix_flow_requires_preview_before_bytes_change() {
    let packet = packet();
    assert!(packet.all_require_preview());
    for p in &packet.paths {
        assert!(p.fix_flow.preview_required);
        assert!(p.fix_flow.shows_raw_and_proposed);
        assert!(p.preview_required());
    }
}

#[test]
fn fix_flow_mode_matches_declared_route() {
    let packet = packet();
    // Local-edit paths preview a diff; AI-apply previews a review sheet.
    assert_eq!(
        path(&packet, M5MutationPath::Save).fix_flow.mode,
        M5FixFlowMode::PreviewableDiff
    );
    assert_eq!(
        path(&packet, M5MutationPath::Format).fix_flow.mode,
        M5FixFlowMode::PreviewableDiff
    );
    assert_eq!(
        path(&packet, M5MutationPath::OrganizeImports).fix_flow.mode,
        M5FixFlowMode::PreviewableDiff
    );
    assert_eq!(
        path(&packet, M5MutationPath::AiApply).fix_flow.mode,
        M5FixFlowMode::ReviewSheet
    );
    for p in &packet.paths {
        assert_eq!(p.fix_flow.mode, p.path.preview_route());
        assert_eq!(p.fix_flow.mode_token, p.fix_flow.mode.as_str());
    }
}

#[test]
fn bidi_save_offers_bidi_fix() {
    let packet = packet();
    let save = path(&packet, M5MutationPath::Save);
    assert!(save.findings_present);
    assert!(save.threat_classes.iter().any(|c| c == "bidi_control"));
    assert!(save.fix_flow.fix_kinds_offered.iter().any(|k| k == "bidi"));
    assert!(save.suspicious_excerpt_escaped.contains("\\u{202E}"));
}

#[test]
fn invisible_format_offers_invisible_fix() {
    let packet = packet();
    let format = path(&packet, M5MutationPath::Format);
    assert!(format.findings_present);
    assert!(format
        .threat_classes
        .iter()
        .any(|c| c == "invisible_formatting"));
    assert!(format
        .fix_flow
        .fix_kinds_offered
        .iter()
        .any(|k| k == "invisible"));
    assert!(format.suspicious_excerpt_escaped.contains("\\u{200B}"));
}

#[test]
fn confusable_organize_imports_offers_confusable_fix() {
    let packet = packet();
    let organize = path(&packet, M5MutationPath::OrganizeImports);
    assert!(organize.findings_present);
    assert!(organize
        .threat_classes
        .iter()
        .any(|c| c == "mixed_script_confusable"));
    assert!(organize
        .fix_flow
        .fix_kinds_offered
        .iter()
        .any(|k| k == "confusable"));
}

#[test]
fn ai_apply_offers_all_three_fix_kinds() {
    let packet = packet();
    let ai = path(&packet, M5MutationPath::AiApply);
    assert!(ai.findings_present);
    for kind in ["bidi", "invisible", "confusable"] {
        assert!(
            ai.fix_flow.fix_kinds_offered.iter().any(|k| k == kind),
            "missing {kind}"
        );
    }
}

#[test]
fn fix_kinds_cover_findings_everywhere() {
    let packet = packet();
    assert!(packet.fix_kinds_cover_findings_everywhere());
    for p in &packet.paths {
        assert!(p.fix_kinds_cover_findings());
        assert_eq!(p.findings_present, !p.fix_flow.fix_kinds_offered.is_empty());
    }
}

#[test]
fn every_suppression_is_scope_aware_and_auditable() {
    let packet = packet();
    assert!(packet.suppressions_auditable_everywhere());
    for p in &packet.paths {
        assert!(p.suppression_audit.scope_aware);
        assert_eq!(
            p.suppression_audit.allowed_scopes.len(),
            M5SuppressionScope::ALL.len()
        );
        assert!(!p.suppression_audit.hidden_per_pane_state);
        if let Some(s) = &p.recorded_suppression {
            assert!(s.is_auditable());
            assert!(!s.hidden_per_pane_state);
            assert!(!s.actor_ref.trim().is_empty());
            assert!(!s.reason.trim().is_empty());
            assert!(!s.recorded_at.trim().is_empty());
            assert!(!s.audit_log_ref.trim().is_empty());
        }
    }
}

#[test]
fn recorded_suppressions_cover_every_scope() {
    let packet = packet();
    assert_eq!(packet.suppressed_path_count, 4);
    let scopes: BTreeSet<_> = packet
        .paths
        .iter()
        .filter_map(|p| p.recorded_suppression.as_ref().map(|s| s.scope))
        .collect();
    for scope in M5SuppressionScope::ALL {
        assert!(scopes.contains(&scope), "missing {}", scope.as_str());
    }
}

#[test]
fn admin_policy_suppression_carries_expiry() {
    let packet = packet();
    let ai = path(&packet, M5MutationPath::AiApply);
    let suppression = ai.recorded_suppression.as_ref().expect("suppression");
    assert_eq!(suppression.scope, M5SuppressionScope::AdminPolicy);
    assert!(suppression.expires_at.is_some());
}

#[test]
fn counts_match_projections() {
    let packet = packet();
    assert_eq!(packet.path_count, packet.paths.len());
    assert_eq!(packet.paths_with_findings_count, 4);
    assert_eq!(
        packet.paths_with_findings_count,
        packet.paths.iter().filter(|p| p.findings_present).count()
    );
    assert_eq!(
        packet.suppressed_path_count,
        packet
            .paths
            .iter()
            .filter(|p| p.recorded_suppression.is_some())
            .count()
    );
}

#[test]
fn guard_preserved_across_carriers() {
    let packet = packet();
    assert!(packet.preserved_everywhere());
    for p in &packet.paths {
        assert!(p.preserved_in_product);
        assert!(p.preserved_in_exported_review_packet);
        assert!(p.preserved_in_support_handoff);
    }
}

#[test]
fn silent_rewrite_unblocked_fails_validation() {
    let mut packet = packet();
    packet.paths[0].silent_byte_rewrite_blocked = false;
    assert!(packet
        .validate()
        .contains(&M5MutationPathFixFlowViolation::SilentRewriteNotBlocked));
}

#[test]
fn preview_not_required_fails_validation() {
    let mut packet = packet();
    packet.paths[0].fix_flow.preview_required = false;
    assert!(packet
        .validate()
        .contains(&M5MutationPathFixFlowViolation::PreviewNotRequired));
}

#[test]
fn fix_flow_mode_mismatch_fails_validation() {
    let mut packet = packet();
    // Force the save path onto the review-sheet mode it should not use.
    packet.paths[0].fix_flow.mode = M5FixFlowMode::ReviewSheet;
    packet.paths[0].fix_flow.mode_token = M5FixFlowMode::ReviewSheet.as_str().to_owned();
    assert!(packet
        .validate()
        .contains(&M5MutationPathFixFlowViolation::FixFlowModeMismatch));
}

#[test]
fn missing_fix_kind_fails_validation() {
    let mut packet = packet();
    // A path with findings that offers no fix kind violates coverage.
    let save = packet
        .paths
        .iter_mut()
        .find(|p| p.path == M5MutationPath::Save)
        .expect("save");
    save.fix_flow.fix_kinds_offered.clear();
    assert!(packet
        .validate()
        .contains(&M5MutationPathFixFlowViolation::FixKindsMissing));
}

#[test]
fn hidden_per_pane_suppression_fails_validation() {
    let mut packet = packet();
    let save = packet
        .paths
        .iter_mut()
        .find(|p| p.path == M5MutationPath::Save)
        .expect("save");
    save.recorded_suppression
        .as_mut()
        .unwrap()
        .hidden_per_pane_state = true;
    assert!(packet
        .validate()
        .contains(&M5MutationPathFixFlowViolation::SuppressionNotAuditable));
}

#[test]
fn unauditable_suppression_fails_validation() {
    let mut packet = packet();
    let save = packet
        .paths
        .iter_mut()
        .find(|p| p.path == M5MutationPath::Save)
        .expect("save");
    save.recorded_suppression.as_mut().unwrap().audit_log_ref = String::new();
    assert!(packet
        .validate()
        .contains(&M5MutationPathFixFlowViolation::SuppressionNotAuditable));
}

#[test]
fn suppression_on_clean_path_fails_validation() {
    let mut packet = packet();
    // Make the save path clean but keep its suppression.
    let save = packet
        .paths
        .iter_mut()
        .find(|p| p.path == M5MutationPath::Save)
        .expect("save");
    save.findings_present = false;
    save.fix_flow.fix_kinds_offered.clear();
    save.threat_classes.clear();
    save.finding_count = 0;
    assert!(packet
        .validate()
        .contains(&M5MutationPathFixFlowViolation::SuppressionWithoutFinding));
}

#[test]
fn missing_path_fails_validation() {
    let mut packet = packet();
    packet.paths.retain(|p| p.path != M5MutationPath::AiApply);
    packet.path_count = packet.paths.len();
    assert!(packet
        .validate()
        .contains(&M5MutationPathFixFlowViolation::MutationPathMissing));
}

#[test]
fn normalization_flag_fails_validation() {
    let mut packet = packet();
    packet.normalization_applied = true;
    assert!(packet
        .validate()
        .contains(&M5MutationPathFixFlowViolation::NormalizationApplied));
}

#[test]
fn incomplete_review_fails_validation() {
    let mut packet = packet();
    packet.review.all_mutation_paths_block_silent_byte_rewrite = false;
    assert!(packet
        .validate()
        .contains(&M5MutationPathFixFlowViolation::ReviewIncomplete));
}

#[test]
fn markdown_summary_lists_every_path() {
    let summary = packet().render_markdown_summary();
    for p in M5MutationPath::ALL {
        assert!(summary.contains(p.as_str()), "missing {}", p.as_str());
    }
}

#[test]
fn packet_round_trips_via_serde() {
    let packet = packet();
    let json = packet.export_safe_json();
    let back: M5MutationPathFixFlowPacket = serde_json::from_str(&json).expect("parse");
    assert_eq!(back, packet);
    assert_eq!(back.record_kind, M5_MUTATION_PATH_FIX_FLOW_RECORD_KIND);
}

#[test]
fn checked_support_export_matches_frozen_packet() {
    let checked = current_m5_mutation_path_fix_flow_export()
        .expect("checked M5 mutation-path fix-flow export validates");
    assert_eq!(checked.packet_id, M5_MUTATION_PATH_FIX_FLOW_PACKET_ID);
    assert_eq!(
        checked,
        frozen_m5_mutation_path_fix_flow_packet(),
        "checked-in support export drifted from the frozen in-code packet; regenerate with the bin"
    );
}

#[test]
fn checked_clean_fixture_blocks_silent_rewrite_without_findings() {
    let raw = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5/m5_mutation_path_fix_flow/all_clean_no_findings.json"
    ));
    let packet: M5MutationPathFixFlowPacket =
        serde_json::from_str(raw).expect("fixture parses as mutation-path fix-flow packet");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.paths_with_findings_count, 0);
    assert_eq!(packet.suppressed_path_count, 0);
    assert!(packet.paths.iter().all(|p| !p.findings_present));
    assert!(packet
        .paths
        .iter()
        .all(|p| p.recorded_suppression.is_none()));
    // The silent-rewrite guard and preview-first fix flow hold even with nothing
    // flagged, so a clean run never silently rewrites bytes.
    assert!(packet.all_block_silent_byte_rewrite());
    assert!(packet.all_require_preview());
}
