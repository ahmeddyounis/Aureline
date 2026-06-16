//! Inline unit tests binding the typed matrix to the checked-in artifact and
//! exercising the open-baseline guardrail, narrowing consistency, and the
//! publication verdict against mutated copies.

use super::*;

fn matrix() -> BoundaryDurabilityMatrix {
    current_m5_boundary_and_upstream_durability().expect("checked-in matrix parses")
}

#[test]
fn embedded_matrix_parses_and_validates() {
    let m = matrix();
    assert_eq!(
        m.schema_version,
        M5_BOUNDARY_AND_UPSTREAM_DURABILITY_SCHEMA_VERSION
    );
    assert_eq!(
        m.record_kind,
        M5_BOUNDARY_AND_UPSTREAM_DURABILITY_RECORD_KIND
    );
    assert_eq!(m.validate(), Vec::new());
    assert!(!m.rows.is_empty());
}

#[test]
fn every_asset_lane_is_covered() {
    let m = matrix();
    for lane in AssetLane::ALL {
        assert!(
            !m.rows_for_lane(lane).is_empty(),
            "asset lane {} must have a row",
            lane.as_str()
        );
    }
}

#[test]
fn must_remain_open_lanes_keep_an_open_baseline_or_narrow() {
    let m = matrix();
    for row in &m.rows {
        if row.must_remain_open && !row.boundary_posture.is_open_baseline() {
            assert!(
                row.has_active_reason(DurabilityReason::BoundaryBaselineViolated),
                "must-remain-open lane {} drifted off the baseline without narrowing",
                row.entry_id
            );
        }
    }
}

#[test]
fn states_are_per_axis_not_one_global_flag() {
    let m = matrix();
    let states: BTreeSet<DurabilityState> = m.rows.iter().map(|r| r.durability_state).collect();
    // The matrix keeps distinct narrowing axes, never one green/red flag.
    assert!(states.contains(&DurabilityState::Durable));
    assert!(
        states.len() >= 3,
        "expected several distinct durability states"
    );

    let reasons: BTreeSet<DurabilityReason> = m
        .rows
        .iter()
        .flat_map(|r| r.active_reasons.iter().copied())
        .collect();
    assert!(!reasons.is_empty(), "narrowed rows must name reasons");
}

#[test]
fn summary_matches_rows() {
    let m = matrix();
    assert_eq!(m.summary, m.computed_summary());
    assert_eq!(
        m.summary.rows_durable + m.summary.rows_narrowed + m.summary.state_withdrawn,
        m.rows.len()
    );
}

#[test]
fn reuse_projection_covers_every_row() {
    let m = matrix();
    let projection = m.reuse_projection();
    assert_eq!(projection.len(), m.rows.len());
    for projected in &projection {
        assert!(
            !projected.reuse_destinations.is_empty(),
            "projected row {} must carry reuse destinations",
            projected.entry_id
        );
    }
}

#[test]
fn durability_layer_failure_holds_promotion_inherited_does_not() {
    let m = matrix();
    // At least one release-blocking lane narrows under a still-stable declared
    // label and holds promotion.
    assert_eq!(m.publication.decision, m.computed_decision());
    let blocking = m.computed_blocking_row_ids();
    for id in &blocking {
        let row = m.row(id).expect("blocking row exists");
        assert!(row.release_blocking);
        assert!(row.declares_at_or_above_cutline());
        assert!(!row.is_waived());
        assert!(row.durability_state.is_narrowed());
    }
    // A lane that is narrowed but already below the cutline, or held by a valid
    // waiver, is gated upstream and never blocks.
    for row in &m.rows {
        if row.durability_state.is_narrowed()
            && (!row.declares_at_or_above_cutline() || row.is_waived())
        {
            assert!(
                !blocking.contains(&row.entry_id),
                "inherited/waived narrowing on {} must not hold promotion",
                row.entry_id
            );
        }
    }
}

#[test]
fn validate_flags_a_must_remain_open_baseline_violation() {
    let mut m = matrix();
    let row = m
        .rows
        .iter_mut()
        .find(|r| r.must_remain_open && r.boundary_posture.is_open_baseline())
        .expect("an open must-remain-open lane exists");
    // Blur the open baseline with a managed posture but leave it durable.
    row.boundary_posture = BoundaryPosture::ManagedService;
    row.support_class = SupportClass::Managed;
    assert!(m
        .validate()
        .iter()
        .any(|x| matches!(x, MatrixViolation::MustRemainOpenViolated { .. })));
}

#[test]
fn validate_flags_a_durable_row_with_a_gap() {
    let mut m = matrix();
    let row = m
        .rows
        .iter_mut()
        .find(|r| r.is_durable())
        .expect("a durable row exists");
    row.active_reasons
        .push(DurabilityReason::OwnerSignoffMissing);
    assert!(m
        .validate()
        .iter()
        .any(|x| matches!(x, MatrixViolation::DurableWithActiveReason { .. })));
}

#[test]
fn validate_flags_a_narrowed_row_that_stays_above_the_cutline() {
    let mut m = matrix();
    let row = m
        .rows
        .iter_mut()
        .find(|r| r.durability_state.is_narrowed())
        .expect("a narrowed row exists");
    row.effective_label = LifecycleLabel::Stable;
    assert!(m.validate().iter().any(|x| matches!(
        x,
        MatrixViolation::NarrowedAboveCutline { .. }
            | MatrixViolation::EffectiveLabelMismatch { .. }
    )));
}

#[test]
fn validate_flags_a_proceed_verdict_while_a_rule_fires() {
    let mut m = matrix();
    if m.computed_decision() == PublicationDecision::Hold {
        m.publication.decision = PublicationDecision::Proceed;
        assert!(m
            .validate()
            .iter()
            .any(|x| matches!(x, MatrixViolation::PublicationDecisionInconsistent)));
    }
}
