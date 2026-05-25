//! Fixture-replay and invariant tests for the stable badge-aggregate corpus.
//!
//! The records live under
//! `fixtures/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression/`
//! and are minted by the `aureline_shell_badge_aggregate_stable` emitter so the
//! checked-in JSON stays a literal projection of the in-code corpus, which is
//! itself a projection of the live attention router.
//!
//! What this guards (the acceptance criteria for this lane):
//!
//! - Each scenario's record on disk matches the in-code projection bit-for-bit.
//!   Regenerate with:
//!
//!   ```sh
//!   cargo run -q -p aureline-shell \
//!     --bin aureline_shell_badge_aggregate_stable -- emit-fixtures \
//!     fixtures/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression
//!   ```
//!
//! - Every badge count is typed by count class, and the required classes are
//!   exercised.
//! - The dock/taskbar, title-bar, in-shell, and companion projections derive from
//!   the same durable object set as the activity center; no Stable surface
//!   inflates a class.
//! - Cross-client / cross-window dedupe collapses copies and preserves
//!   count-class integrity.
//! - A zero active badge means no current durable objects of that class — held
//!   items are tracked separately with lineage.
//! - Every admin-suppressed, quiet-hours-muted, or per-class-disabled badge
//!   difference carries export-safe lineage that preserves the durable object and
//!   reopen target.
//! - The persistent attention summary is durable and inspectable.
//! - No snapshot over-claims; a snapshot that cannot prove a pillar or sits on a
//!   below-Stable surface is narrowed below Stable with a named reason.
//! - The same aggregate opens from the activity center, command palette, status
//!   bar, and a menu command, keyboard-first, across normal / high-contrast /
//!   zoomed layouts.
//! - Every snapshot stays available without an account or managed services.

use aureline_shell::badge_aggregate_stable::{
    badge_aggregate_corpus, AggregateCountClass, BadgeAggregateRecord, BadgeRecoveryAction,
    BadgeSurface, DurableItemDisposition, SuppressionScope,
};
use aureline_shell::notification_attention_stable::{
    is_canonical_object_ref, AttentionRouteSurface, LayoutMode, StableClaimClass,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression",
);

fn load_record(filename: &str) -> BadgeAggregateRecord {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in badge_aggregate_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-shell --bin aureline_shell_badge_aggregate_stable -- emit-fixtures fixtures/ux/m4/finalize-badge-semantics-cross-client-dedupe-admin-suppression`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn pinned_rollups_match_each_scenario() {
    for scenario in badge_aggregate_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(
            record.posture_id, scenario.expected_posture,
            "{} posture",
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
    }
}

#[test]
fn required_count_classes_are_exercised_across_the_matrix() {
    let mut seen = std::collections::BTreeSet::new();
    for scenario in badge_aggregate_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for class in &record.class_aggregates {
            seen.insert(class.count_class);
        }
    }
    for required in AggregateCountClass::REQUIRED {
        assert!(
            seen.contains(&required),
            "missing count class {}",
            required.as_str()
        );
    }
}

#[test]
fn surfaces_derive_from_one_durable_set() {
    for scenario in badge_aggregate_corpus() {
        let record = load_record(&scenario.fixture_filename);
        // Every required surface must be projected.
        for required in BadgeSurface::REQUIRED {
            assert!(
                record
                    .surface_projections
                    .iter()
                    .any(|p| p.surface == required),
                "{} missing surface {}",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        // The activity center is the authoritative durable surface: it never
        // disables a class and never diverges.
        let activity = record
            .surface_projections
            .iter()
            .find(|p| p.surface == BadgeSurface::ActivityCenter)
            .expect("activity center projection");
        assert!(
            activity.matches_durable_set,
            "{} activity center diverges",
            scenario.scenario_id
        );
        assert!(
            activity.disabled_classes.is_empty(),
            "{} activity center disables a class",
            scenario.scenario_id
        );
        assert!(
            !activity.inflates_any_class,
            "{} activity center inflates",
            scenario.scenario_id
        );

        // A Stable snapshot proves the pillar; a narrowed one names a reason.
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            assert!(
                record.pillars.one_durable_set_holds,
                "{}",
                scenario.scenario_id
            );
            for projection in &record.surface_projections {
                assert!(
                    !projection.inflates_any_class,
                    "{} surface {} inflates on a Stable row",
                    scenario.scenario_id,
                    projection.surface.as_str(),
                );
            }
        }
    }
}

#[test]
fn cross_client_dedupe_preserves_class_integrity() {
    for scenario in badge_aggregate_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let dedupe = &record.cross_client_dedupe;
        assert!(
            dedupe.class_integrity_preserved,
            "{} dedupe lost class integrity",
            scenario.scenario_id
        );
        assert_eq!(
            dedupe.deduped_object_count,
            record.deduped_objects.len() as u32,
            "{} deduped count drift",
            scenario.scenario_id,
        );
        assert_eq!(
            dedupe.raw_appearance_count,
            dedupe.deduped_object_count + dedupe.cross_client_collapsed,
            "{} dedupe arithmetic drift",
            scenario.scenario_id,
        );
        // Repeated copies of one object never multiply its class count: the
        // sum of active objects per class equals the class' active count.
        for class in &record.class_aggregates {
            let active_objects = record
                .deduped_objects
                .iter()
                .filter(|o| {
                    o.count_class == class.count_class
                        && o.disposition == DurableItemDisposition::Active
                })
                .count() as u32;
            assert_eq!(
                active_objects,
                class.active_count,
                "{} class {} active count not derived from durable objects",
                scenario.scenario_id,
                class.count_class.as_str(),
            );
        }
    }
}

#[test]
fn zero_active_means_no_active_durable_objects() {
    for scenario in badge_aggregate_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            record.pillars.zero_means_no_durable_items,
            "{}",
            scenario.scenario_id
        );
        for class in &record.class_aggregates {
            let active_objects = record
                .deduped_objects
                .iter()
                .filter(|o| {
                    o.count_class == class.count_class
                        && o.disposition == DurableItemDisposition::Active
                })
                .count();
            // A zero active count corresponds to zero active durable objects —
            // not a hidden toast or a failed fanout.
            assert_eq!(
                class.active_count == 0,
                active_objects == 0,
                "{} class {} zero-means-none violated",
                scenario.scenario_id,
                class.count_class.as_str(),
            );
        }
    }
}

#[test]
fn suppression_lineage_is_export_safe_and_complete() {
    for scenario in badge_aggregate_corpus() {
        let record = load_record(&scenario.fixture_filename);
        // Every held / suppressed object has an object-scoped lineage entry.
        for object in &record.deduped_objects {
            if object.disposition != DurableItemDisposition::HeldOrSuppressed {
                continue;
            }
            let covered = record.suppression_lineage.iter().any(|entry| {
                entry.scope == SuppressionScope::Object
                    && entry.object_ref.as_deref() == Some(object.object_ref.as_str())
            });
            assert!(
                covered,
                "{} held object {} has no lineage",
                scenario.scenario_id, object.object_ref,
            );
        }
        // Every per-class disablement on a surface has a lineage entry.
        for projection in &record.surface_projections {
            for class in &projection.disabled_classes {
                let covered = record.suppression_lineage.iter().any(|entry| {
                    entry.scope == SuppressionScope::SurfaceClass
                        && entry.surface == Some(projection.surface)
                        && entry.count_class == Some(*class)
                });
                assert!(
                    covered,
                    "{} surface {} disabled class {} has no lineage",
                    scenario.scenario_id,
                    projection.surface.as_str(),
                    class.as_str(),
                );
            }
        }
        // Every lineage entry preserves the durable object and reopen target.
        for entry in &record.suppression_lineage {
            assert!(
                entry.durable_object_preserved && entry.reopen_target_preserved,
                "{} lineage entry erases durable truth",
                scenario.scenario_id,
            );
            assert!(
                !entry.export_safe_summary.trim().is_empty(),
                "{} empty lineage summary",
                scenario.scenario_id
            );
        }
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            assert!(
                record.pillars.suppression_lineage_export_safe,
                "{}",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn summary_is_persistent_and_inspectable() {
    for scenario in badge_aggregate_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let summary = &record.summary_digest;
        let total_active: u32 = record.class_aggregates.iter().map(|c| c.active_count).sum();
        let total_held: u32 = record
            .class_aggregates
            .iter()
            .map(|c| c.held_or_suppressed_count)
            .sum();
        assert_eq!(
            summary.total_active, total_active,
            "{} summary active drift",
            scenario.scenario_id
        );
        assert_eq!(
            summary.total_held_or_suppressed, total_held,
            "{} summary held drift",
            scenario.scenario_id,
        );
        assert!(
            summary.inspectable_in_product,
            "{} summary not inspectable",
            scenario.scenario_id
        );
        if record.stable_qualification.claim_class == StableClaimClass::Stable {
            assert!(
                record.pillars.summary_persistent_inspectable,
                "{}",
                scenario.scenario_id
            );
            assert!(summary.durable_and_persistent, "{}", scenario.scenario_id);
        }
    }
}

#[test]
fn matrix_spans_stable_and_narrowed_rows() {
    let corpus = badge_aggregate_corpus();
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
    for scenario in badge_aggregate_corpus() {
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
    for scenario in badge_aggregate_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let ceiling = record.claim_ceiling;
        let pillars = record.pillars;
        if ceiling.asserts_one_durable_set {
            assert!(
                pillars.one_durable_set_holds,
                "{} overclaims one durable set",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_cross_client_dedupe {
            assert!(
                pillars.cross_client_dedupe_holds,
                "{} overclaims dedupe",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_suppression_lineage_export_safe {
            assert!(
                pillars.suppression_lineage_export_safe,
                "{} overclaims lineage",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_zero_means_no_durable_items {
            assert!(
                pillars.zero_means_no_durable_items,
                "{} overclaims zero-means-none",
                scenario.scenario_id
            );
        }
        if ceiling.asserts_summary_persistent_inspectable {
            assert!(
                pillars.summary_persistent_inspectable,
                "{} overclaims summary",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn recovery_routes_are_complete_and_keyboard_reachable() {
    for scenario in badge_aggregate_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let route_ids: Vec<&str> = record
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in BadgeRecoveryAction::REQUIRED {
            assert!(
                route_ids.contains(&required.as_str()),
                "{} missing recovery route {}",
                scenario.scenario_id,
                required.as_str(),
            );
        }
        for route in &record.recovery_routes {
            assert!(
                route.keyboard_reachable,
                "{} recovery route {} not keyboard reachable",
                scenario.scenario_id, route.action_id,
            );
        }
    }
}

#[test]
fn routes_reach_every_surface_keyboard_first() {
    for scenario in badge_aggregate_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for required in AttentionRouteSurface::REQUIRED {
            let route = record
                .routes
                .iter()
                .find(|route| route.surface == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing route surface {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                route.keyboard_reachable,
                "{} route {} not keyboard reachable",
                scenario.scenario_id,
                required.as_str()
            );
            assert!(
                route.activates_same_item,
                "{} route {} activates a different item",
                scenario.scenario_id,
                required.as_str()
            );
            assert!(
                is_canonical_object_ref(&route.route_ref),
                "{} route {} ref {:?} not canonical",
                scenario.scenario_id,
                required.as_str(),
                route.route_ref,
            );
        }
    }
}

#[test]
fn accessibility_holds_in_every_layout() {
    for scenario in badge_aggregate_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(
            record.accessibility.action_labels.len(),
            record.recovery_routes.len(),
            "{} action labels drift from recovery routes",
            scenario.scenario_id,
        );
        for (label, route) in record
            .accessibility
            .action_labels
            .iter()
            .zip(record.recovery_routes.iter())
        {
            assert_eq!(
                label, &route.action_label,
                "{} action label drift",
                scenario.scenario_id
            );
        }
        for required in LayoutMode::REQUIRED {
            let mode = record
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
                .unwrap_or_else(|| {
                    panic!(
                        "{} missing layout mode {}",
                        scenario.scenario_id,
                        required.as_str()
                    )
                });
            assert!(
                mode.row_narration_available && mode.recovery_affordances_reachable,
                "{} layout mode {} unreachable",
                scenario.scenario_id,
                required.as_str(),
            );
        }
    }
}

#[test]
fn rows_stay_available_without_account_or_managed_services() {
    for scenario in badge_aggregate_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            record.available_without_account,
            "{} hidden without an account",
            scenario.scenario_id
        );
        assert!(
            record.available_without_managed_services,
            "{} hidden without managed services",
            scenario.scenario_id,
        );
    }
}

#[test]
fn minted_refs_are_canonical_durable_objects() {
    for scenario in badge_aggregate_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for (label, value) in [
            ("diagnostics_export_ref", &record.diagnostics_export_ref),
            ("support_export_ref", &record.support_export_ref),
        ] {
            assert!(
                is_canonical_object_ref(value),
                "{} {label} {value:?} not canonical",
                scenario.scenario_id,
            );
        }
        for object in &record.deduped_objects {
            assert!(
                is_canonical_object_ref(&object.object_ref),
                "{} object ref {:?} not canonical",
                scenario.scenario_id,
                object.object_ref,
            );
        }
        for value in record
            .evidence_refs
            .iter()
            .chain(record.narrative_refs.iter())
        {
            assert!(
                is_canonical_object_ref(value),
                "{} ref {value:?} not canonical",
                scenario.scenario_id,
            );
        }
    }
}
