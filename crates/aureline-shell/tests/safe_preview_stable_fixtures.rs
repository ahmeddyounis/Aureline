//! Fixture-replay and invariant tests for the stable safe-preview corpus.
//!
//! The records live under
//! `fixtures/ux/m4/certify-suspicious-content-safe-preview-copy-export-and/` and
//! are minted by the `aureline_shell_safe_preview_stable` emitter so the checked-in
//! JSON stays a literal projection of the in-code corpus, which is itself a
//! projection of the live content-safety detector and the trust-class /
//! representation vocabulary.
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_safe_preview_stable -- emit-fixtures \
//!     fixtures/ux/m4/certify-suspicious-content-safe-preview-copy-export-and
//!   ```
//!
//! - The matrix covers every carrier surface and at least one trust-sensitive
//!   action, and spans Stable and narrowed rows.
//! - The trust class is consumed from the content-safety ladder rather than a
//!   re-spelled vocabulary; copy/export choices use the content-safety action ids.
//! - Representation cues stay explicit, suspicious findings keep reveal + escaped
//!   paths, copy/export offers a raw and a safe path, and cues survive all five
//!   carriers on Stable rows; the carrier-flatten and reveal-missing drills narrow.
//! - Trust-sensitive actions show a stricter boundary before commit; the
//!   boundary drill narrows.
//! - Accessibility cues hold across normal / high-contrast / zoomed layouts;
//!   per-OS conformance covers macOS, Windows, and Linux with current proof.
//! - The shell surface, activity center, CLI inspect, Help/About, and support
//!   export all bind the shared record; the same posture opens from the activity
//!   center, command palette, status bar, and a menu command, keyboard-first,
//!   without an account or managed services.

use aureline_shell::notification_attention_stable::{
    AttentionRouteSurface, LayoutMode, StableClaimClass,
};
use aureline_shell::shell_safe_preview_stable::{
    safe_preview_corpus, CueCarrier, SafePreviewRecord, SafePreviewRecoveryAction,
    SafePreviewTruthSurface, ShellAdjacentSurface,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/certify-suspicious-content-safe-preview-copy-export-and",
);

fn load_record(filename: &str) -> SafePreviewRecord {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in safe_preview_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_safe_preview_stable -- emit-fixtures fixtures/ux/m4/certify-suspicious-content-safe-preview-copy-export-and`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in safe_preview_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(
            record.surface_class, scenario.expected_surface_class,
            "{} surface class",
            scenario.scenario_id
        );
        assert_eq!(
            record.stable_qualification.claim_class, scenario.expected_claim_class,
            "{} claim_class",
            scenario.scenario_id
        );
        assert_eq!(
            record.stable_qualification.qualifies_stable, scenario.expected_qualifies_stable,
            "{} qualifies_stable",
            scenario.scenario_id
        );
        assert_eq!(
            record.surface_lifecycle_marker, scenario.expected_surface_marker,
            "{} surface marker",
            scenario.scenario_id
        );
        assert_eq!(
            record.posture_id, scenario.scenario_id,
            "{} posture id",
            scenario.scenario_id
        );
    }
}

#[test]
fn matrix_covers_carriers_and_a_trust_sensitive_action() {
    let corpus = safe_preview_corpus();
    let carriers = [
        ShellAdjacentSurface::Notification,
        ShellAdjacentSurface::ActivityCenter,
        ShellAdjacentSurface::BrowserHandoff,
        ShellAdjacentSurface::SupportExport,
        ShellAdjacentSurface::ScreenshotEvidence,
    ];
    for carrier in carriers {
        assert!(
            corpus.iter().any(|s| s.expected_surface_class == carrier),
            "matrix must cover carrier surface {}",
            carrier.as_str(),
        );
    }
    assert!(
        corpus
            .iter()
            .any(|s| s.expected_surface_class.is_trust_sensitive_action()),
        "matrix must cover at least one trust-sensitive action",
    );
}

#[test]
fn consumes_content_safety_trust_class_ladder() {
    let trust_ladder = [
        "RawText",
        "SanitizedRich",
        "TrustedLocalActive",
        "IsolatedRemoteActive",
    ];
    let action_ids = [
        "copy_raw",
        "copy_rendered",
        "copy_escaped",
        "export_sanitized_snapshot",
        "export_metadata_only",
    ];
    for scenario in safe_preview_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let value = serde_json::to_value(&record).expect("serializes");
        let trust = value["trust_class"].as_str().expect("trust_class string");
        assert!(
            trust_ladder.contains(&trust),
            "{} trust class {trust} is not on the content-safety ladder",
            scenario.scenario_id,
        );
        // Upstream provenance keeps the content-safety schema versions.
        assert!(
            record.upstream.trust_class_schema_version >= 1
                && record.upstream.representation_policy_schema_version >= 1,
            "{} drops content-safety schema-version provenance",
            scenario.scenario_id,
        );
        for choice in value["representation_choices"]
            .as_array()
            .expect("choices array")
        {
            let id = choice["action_id"].as_str().expect("action_id string");
            assert!(
                action_ids.contains(&id),
                "{} copy/export choice {id} is not a content-safety action id",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn representation_cues_explicit_on_stable() {
    for scenario in safe_preview_corpus() {
        let record = load_record(&scenario.fixture_filename);
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            let cues = &record.representation;
            assert!(
                cues.raw_reveal_available
                    && cues.representation_label_present
                    && cues.copy_raw_present
                    && cues.explicit_when_meaning_differs,
                "{} Stable but representation cues not explicit",
                scenario.scenario_id,
            );
            // Whenever a rendered view exists, Copy rendered must be explicit too.
            if record.renders_rich_content {
                assert!(
                    cues.copy_rendered_present,
                    "{} renders rich content but Copy rendered missing",
                    scenario.scenario_id,
                );
            }
            assert!(
                record.pillars.representation_cues_explicit,
                "{} Stable but representation pillar false",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn reveal_missing_drill_narrows_on_representation() {
    let record = load_record("rendered_reveal_missing_drill.json");
    assert!(!record.representation.raw_reveal_available);
    assert!(!record.representation.explicit_when_meaning_differs);
    assert!(!record.pillars.representation_cues_explicit);
    assert!(record
        .stable_qualification
        .narrowing_reasons
        .iter()
        .any(|r| r.as_str() == "representation_cues_not_explicit"));
}

#[test]
fn copy_export_offers_raw_and_safe_paths() {
    for scenario in safe_preview_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            record
                .representation_choices
                .iter()
                .any(|c| c.is_exact_raw()),
            "{} offers no raw copy path",
            scenario.scenario_id,
        );
        assert!(
            record
                .representation_choices
                .iter()
                .any(|c| c.is_safe_inspection()),
            "{} offers no safe-inspection path",
            scenario.scenario_id,
        );
        for choice in &record.representation_choices {
            assert!(
                !choice.label.trim().is_empty(),
                "{} representation choice is unlabeled",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn suspicious_findings_keep_reveal_and_escaped_paths() {
    for scenario in safe_preview_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for finding in &record.suspicious_findings {
            assert!(
                finding.holds(),
                "{} finding {} is flattened (no reveal or no escaped path)",
                scenario.scenario_id,
                finding.finding_id,
            );
        }
        // suspicious_content_present must agree with the finding list.
        assert_eq!(
            record.suspicious_content_present,
            !record.suspicious_findings.is_empty(),
            "{} suspicious_content_present disagrees with findings",
            scenario.scenario_id,
        );
    }
}

#[test]
fn cue_carriers_complete_and_unflattened_on_stable() {
    for scenario in safe_preview_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in CueCarrier::REQUIRED {
            assert!(
                record
                    .cue_survival
                    .iter()
                    .any(|row| row.carrier == required),
                "{} missing cue carrier {}",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            for row in &record.cue_survival {
                assert!(
                    row.holds(),
                    "{} Stable but carrier {} flattens a cue",
                    scenario.scenario_id,
                    row.carrier.as_str(),
                );
            }
            assert!(
                record.pillars.cues_survive_all_carriers,
                "{} Stable but cue-survival pillar false",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn carrier_flatten_drill_actually_flattens() {
    let record = load_record("support_export_flattened_preview_drill.json");
    assert!(record.cue_survival.iter().any(|row| !row.holds()));
    assert!(!record.pillars.cues_survive_all_carriers);
    assert!(record
        .stable_qualification
        .narrowing_reasons
        .iter()
        .any(|r| r.as_str() == "cues_flattened_on_carrier"));
}

#[test]
fn trust_sensitive_actions_show_boundary_before_commit_on_stable() {
    for scenario in safe_preview_corpus() {
        let record = load_record(&scenario.fixture_filename);
        if record.is_trust_sensitive_action {
            let boundary = record
                .stricter_boundary
                .as_ref()
                .unwrap_or_else(|| panic!("{} action lacks a boundary", scenario.scenario_id));
            assert_eq!(
                boundary.action, record.surface_class,
                "{} boundary names a different action",
                scenario.scenario_id,
            );
            if record.stable_qualification.claim_class == StableClaimClass::Stable {
                assert!(
                    boundary.holds(),
                    "{} Stable action does not show the boundary before commit",
                    scenario.scenario_id,
                );
            }
        } else {
            assert!(
                record.stricter_boundary.is_none(),
                "{} non-action carries a stricter boundary",
                scenario.scenario_id,
            );
        }
    }
}

#[test]
fn boundary_not_shown_drill_narrows() {
    let record = load_record("install_boundary_not_shown_drill.json");
    let boundary = record.stricter_boundary.as_ref().expect("boundary present");
    assert!(!boundary.shows_boundary_before_commit);
    assert!(!record.pillars.stricter_boundary_shown_before_commit);
    assert!(record
        .stable_qualification
        .narrowing_reasons
        .iter()
        .any(|r| r.as_str() == "stricter_boundary_not_shown_before_commit"));
}

#[test]
fn accessibility_cues_complete_on_stable() {
    for scenario in safe_preview_corpus() {
        let record = load_record(&scenario.fixture_filename);
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            let cues = &record.a11y_cues;
            assert!(
                cues.warning_announced_not_color_only
                    && cues.representation_label_announced
                    && cues.trust_class_announced
                    && cues.reveal_affordance_keyboard_reachable,
                "{} Stable but accessibility cues incomplete",
                scenario.scenario_id,
            );
            assert!(
                record.pillars.accessibility_cues_complete,
                "{} Stable but a11y pillar false",
                scenario.scenario_id,
            );
        }
        for required in LayoutMode::REQUIRED {
            let disclosure = record
                .accessibility
                .layout_modes
                .iter()
                .find(|m| m.mode == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing layout mode {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                disclosure.row_narration_available && disclosure.recovery_affordances_reachable,
                "{} layout mode {} unreachable",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        assert_eq!(
            record.accessibility.action_labels.len(),
            record.recovery_routes.len(),
            "{} action-label / recovery-route count drift",
            scenario.scenario_id,
        );
    }
}

#[test]
fn per_os_conformance_is_complete_with_proof() {
    use aureline_shell::shell_safe_preview_stable::PlatformProfileClass;
    for scenario in safe_preview_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in PlatformProfileClass::REQUIRED {
            let row = record
                .platform_conformance
                .iter()
                .find(|row| row.profile == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing per-OS profile {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                row.covered && !row.proof_ref.trim().is_empty(),
                "{} profile {} lacks current proof",
                scenario.scenario_id,
                required.as_str(),
            );
            assert!(
                !row.named_behaviors.is_empty(),
                "{} profile {} names no behavior",
                scenario.scenario_id,
                required.as_str(),
            );
        }
    }
}

#[test]
fn surfaces_bind_the_shared_record() {
    for scenario in safe_preview_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in SafePreviewTruthSurface::REQUIRED {
            let projection = record
                .surface_projections
                .iter()
                .find(|p| p.surface == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing binding surface {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                projection.reads_shared_record,
                "{} surface {} clones prose",
                scenario.scenario_id,
                required.as_str(),
            );
            assert!(
                !projection.summary_line.trim().is_empty(),
                "{} surface {} empty summary",
                scenario.scenario_id,
                required.as_str(),
            );
        }
    }
}

#[test]
fn entry_routes_reach_the_same_posture_keyboard_first() {
    for scenario in safe_preview_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in AttentionRouteSurface::REQUIRED {
            let route = record
                .routes
                .iter()
                .find(|r| r.surface == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing entry route {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                route.keyboard_reachable && route.activates_same_item,
                "{} route {} not keyboard-first or targets a different item",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        for required in SafePreviewRecoveryAction::REQUIRED {
            assert!(
                record
                    .recovery_routes
                    .iter()
                    .any(|r| r.action_id == required.as_str()),
                "{} missing recovery action {}",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        assert!(
            record.available_without_account && record.available_without_managed_services,
            "{} not available without an account or managed services",
            scenario.scenario_id,
        );
    }
}

#[test]
fn matrix_spans_stable_and_narrowed_rows() {
    let corpus = safe_preview_corpus();
    let stable = corpus
        .iter()
        .filter(|s| s.expected_claim_class == StableClaimClass::Stable)
        .count();
    let narrowed = corpus.len() - stable;
    assert!(stable >= 1, "matrix must include a Stable row");
    assert!(narrowed >= 1, "matrix must include a narrowed row");
}

#[test]
fn narrowed_rows_drop_below_cutline_and_name_a_reason() {
    for scenario in safe_preview_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let qualification = &record.stable_qualification;
        if qualification.claim_class == StableClaimClass::Stable {
            assert!(
                qualification.qualifies_stable && qualification.narrowing_reasons.is_empty(),
                "{} Stable row carries a narrowing reason",
                scenario.scenario_id,
            );
        } else {
            assert!(
                !qualification.qualifies_stable,
                "{} narrowed row still qualifies",
                scenario.scenario_id
            );
            assert!(
                !qualification.claim_class.at_or_above_cutline(),
                "{} narrowed claim sits at or above the cutline",
                scenario.scenario_id,
            );
            assert!(
                !qualification.narrowing_reasons.is_empty(),
                "{} narrowed without a named reason",
                scenario.scenario_id,
            );
            assert!(
                record.honesty_marker_present,
                "{} hides the honesty marker",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn claim_ceiling_never_overclaims() {
    for scenario in safe_preview_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let ceiling = record.claim_ceiling;
        let pillars = record.pillars;
        if ceiling.asserts_representation_cues_explicit {
            assert!(
                pillars.representation_cues_explicit,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_suspicious_findings_surfaced {
            assert!(
                pillars.suspicious_findings_surfaced,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_copy_export_labeled {
            assert!(pillars.copy_export_labeled, "{}", scenario.scenario_id);
        }
        if ceiling.asserts_cues_survive_all_carriers {
            assert!(
                pillars.cues_survive_all_carriers,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_stricter_boundary_shown_before_commit {
            assert!(
                pillars.stricter_boundary_shown_before_commit,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_accessibility_cues_complete {
            assert!(
                pillars.accessibility_cues_complete,
                "{}",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_platform_conformance_complete {
            assert!(
                pillars.platform_conformance_complete,
                "{}",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn preview_help_surface_narrows_to_preview() {
    let record = load_record("preview_help_surface_posture.json");
    assert_eq!(
        record.stable_qualification.claim_class,
        StableClaimClass::Preview
    );
    // Every pillar holds; the only narrowing is the below-Stable binding surface.
    assert!(record.pillars.representation_cues_explicit);
    assert!(record.pillars.cues_survive_all_carriers);
    assert_eq!(
        record.stable_qualification.narrowing_reasons.len(),
        1,
        "preview row should narrow only on the surface marker",
    );
    assert_eq!(
        record.stable_qualification.narrowing_reasons[0].as_str(),
        "surface_not_yet_stable"
    );
}
