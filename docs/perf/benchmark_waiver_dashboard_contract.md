# Benchmark-waiver dashboard, recurring-regression triage, and correction-trigger contract

This contract freezes one shared vocabulary for the benchmark-waiver
dashboard row, the recurring-regression triage envelope, and the
correction-trigger envelope that the perf side uses to treat repeated
benchmark drift and performance waivers as governed program signals
rather than one-off tolerated regressions. It exists so the dashboard,
the milestone-close review, the release-candidate shiproom packet, the
support-escalation export, the weekly governance review, and the
governance packet all consume the **same** rolled-up row shape with
the **same** stable IDs whenever a protected fitness-function row,
metric family, and corpus / profile / hardware envelope crosses into
drift, waiver, recurrence, or correction-trigger territory.

The contract is pre-implementation. It defines the reusable record
shape, the closed vocabularies, the projection rules, the export
parity floor, and the fixture corpus. It does not implement
performance tuning, regression bisect tooling, a nightly-job runner,
or a waiver-renewal workflow.

## Companion artifacts

- [`/schemas/perf/benchmark_waiver_row.schema.json`](../../schemas/perf/benchmark_waiver_row.schema.json)
  — boundary schema for one `benchmark_waiver_row_record`.
- [`/fixtures/perf/benchmark_waiver_cases/`](../../fixtures/perf/benchmark_waiver_cases/)
  — worked records covering a passing baseline row, a warning-band
  drift row, an active-waiver row nearing expiry, an expired-waiver
  row that triggered correction work, a recurring-cluster row that
  escalates to release-council correction, and a noisy isolated
  failure that the triage vocabulary keeps distinct from chronic
  drift.
- [`/docs/governance/nightly_report_and_waiver_queue_contract.md`](../governance/nightly_report_and_waiver_queue_contract.md)
  and its schemas
  [`/schemas/governance/nightly_report_row.schema.json`](../../schemas/governance/nightly_report_row.schema.json)
  and
  [`/schemas/governance/waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json)
  — the upstream nightly-row and waiver-expiry-queue vocabulary the
  benchmark-waiver dashboard reuses for run-state, compare-action,
  freshness, mitigation, and cluster classes.
- [`/docs/governance/waiver_register_contract.md`](../governance/waiver_register_contract.md)
  and its schema
  [`/schemas/governance/waiver_register.schema.json`](../../schemas/governance/waiver_register.schema.json)
  — register projection that every `waiver_register_entry_refs` entry
  resolves into.
- [`/schemas/benchmarks/run_result.schema.json`](../../schemas/benchmarks/run_result.schema.json)
  — per-run benchmark-lab record family. Every
  `last_qualifying_run_ref` and every `prior_baseline_packet_refs[]`
  resolves to a record on this schema.
- [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)
  and
  [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml)
  — protected fitness-function and threshold catalogs every dashboard
  row pins through `protected_path.fitness_function_row_ref` and
  `protected_path.protected_metrics_row_ref`.
- [`/artifacts/governance/correction_trigger_table.yaml`](../../artifacts/governance/correction_trigger_table.yaml)
  — current-milestone correction-trigger projection. Every
  `correction_work_refs` entry resolves through this table; the
  dashboard's `correction_trigger_class` mirrors the action-class
  vocabulary frozen there.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — DRI / lane / waiver register. Every `waiver_record_ref`,
  `correction_owner_dri_ref`, `correction_owner_lane_ref`, and
  `decision_forum_ref` resolves into this matrix.
- [`/artifacts/perf/reference_hardware_manifest.yaml`](../../artifacts/perf/reference_hardware_manifest.yaml)
  and
  [`/artifacts/perf/lab_image_manifest.yaml`](../../artifacts/perf/lab_image_manifest.yaml)
  — hardware / environment definition rows the
  `corpus_profile_identity` envelope pins through
  `hardware_definition_ref` and `environment_definition_ref`.
- [`/artifacts/governance/evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml)
  and
  [`/artifacts/governance/evidence_rerun_triggers.yaml`](../../artifacts/governance/evidence_rerun_triggers.yaml)
  — proof-class freshness ceilings and named rerun triggers; the
  dashboard's `evidence_freshness` envelope copies metadata from
  these registers.

## Normative sources projected here

- `.t2/docs/Aureline_Technical_Architecture_Document.md` §3.4
  quality-attribute scenarios and measurable SLOs, §22.9 release
  governance.
- `.t2/docs/Aureline_Technical_Design_Document.md` §8.36 release
  evidence, §8.41 supportability evidence.
- `.t2/docs/Aureline_PRD.md` performance, regression, waiver, and
  release-evidence requirements.

If this contract disagrees with those sources, those sources win and
this contract, the schema, and the fixtures update in the same
change.

## Why a benchmark-waiver dashboard contract exists

1. **Tolerated regressions become invisible without one shared row.**
   A waiver minted on a release-floor breach becomes a queue item on
   the governance side, but the recurring-evidence run keeps
   producing fresh nightly rows on the same protected path. Without
   one rolled-up dashboard row, the weekly review sees an isolated
   waiver renewal and the milestone-close review sees an isolated
   blocker; nobody sees the chronic drift across both. The dashboard
   row is the rolled-up projection both reviews quote.
2. **Chronic drift cannot collapse into a "recurring" label.** A
   protected path that breaches the warning band three weeks running
   is a different signal from one that breaches once and then holds.
   The `drift_direction_class` and `noise_class` vocabularies
   distinguish chronic drift (consistent direction, low coefficient
   of variation) from isolated noisy failures (high variance, no
   consistent direction). A surface that collapses the two into a
   single "recurring" chip is non-conforming.
3. **Recurrence count is data, not a chip.** A row with three
   waivers across distinct quarters needs the integer count and the
   typed `recurrence_window_class` so the weekly review can sort by
   recurrence and route the correction work, not just see a "yellow
   chip" that hides the count.
4. **Correction triggers must name who opens staffed work.** A
   chronic-drift row that has crossed the correction-trigger
   threshold is owed staffed correction work, not another waiver
   renewal. The `correction_trigger_class` and
   `correction_owner_role_class` name which authority is owed the
   work; a row that fires the trigger without naming the owner is
   non-conforming.
5. **The dashboard, the milestone-close review, and the release-
   candidate review must consume the same row.** Without one shared
   row shape, the milestone-close review re-derives drift from
   scorecard tiles and the release-candidate review re-derives it
   from release-evidence packets — and the two derivations
   inevitably disagree. The `review_cycle_export_fields` parity
   floor enforces that every consuming review and every consuming
   packet renders the same envelope.
6. **An expired waiver cannot ride into release / claim
   publication.** The schema's `allOf` block forbids the
   `drift_breaches_release_floor_expired_waiver` row from being
   rendered on the release packet or the claim manifest unless it
   is also rendered on the dashboard and the governance packet, so
   an expired waiver cannot be filed away from the dashboard while
   the release surface still reads as clean.

## 1. Benchmark-waiver dashboard row shape

A `benchmark_waiver_row_record` carries:

- `benchmark_waiver_row_id` — stable, machine-readable id quoted by
  every consuming surface.
- `evaluated_at` — projection timestamp; distinct from the per-run
  `measured_on` on the underlying benchmark run-result.
- `headline_label` — bounded reviewable headline rendered onto the
  row.
- `protected_path` — typed envelope pinning the fitness-function
  row, the metric family, the protected-metrics row, and the catalog
  / metrics revisions (§3).
- `corpus_profile_identity` — typed envelope pinning the corpus,
  the profile, the hardware definition, the environment definition,
  the council-baseline flag, and a bounded reviewable identity
  summary (§4).
- `current_state` — typed `dashboard_row_state_class`, the latest
  qualifying run ref, the latest qualifying run timestamp, the next
  expected run timestamp, and a bounded reviewable state summary
  (§5).
- `evidence_freshness` — typed freshness class plus captured-at,
  stale-after, expires-at, rerun-trigger ref, and freshness summary;
  reuses the freshness vocabulary from the governance-side
  nightly-row contract.
- `waiver_envelope` — typed `waiver_cause_class`,
  `expiry_proximity_class`, `mitigation_status_class`, the waiver
  record ref, the waiver-expiry-item ref, the waiver-authority ref,
  the expiry timestamp, and a bounded reviewable waiver summary
  (§6).
- `drift_triage` — typed `drift_direction_class`, `noise_class`,
  `compare_action_class`, `compare_result_class`,
  `comparison_envelope_class`, the baseline-window span, the
  baseline-run count, the prior-baseline packet refs, and a bounded
  reviewable compare summary (§7).
- `recurrence_envelope` — typed `recurrence_window_class`, the
  integer recurrence count, the recurrence-window span, the typed
  recurring-waiver and repeated-protected-path-regression cluster
  classes, the cluster-member refs, and a bounded reviewable
  recurrence summary (§8).
- `correction_trigger` — typed `correction_trigger_class`,
  `correction_owner_role_class`, the DRI / lane / decision-forum
  refs, the correction-work refs, the response-window business
  days, and a bounded reviewable correction-trigger summary (§9).
- `release_or_milestone_impact` — typed
  `affected_release_or_milestone_refs`, linked claim refs, linked
  compatibility-surface refs, and a bounded reviewable impact
  summary (§10).
- `linked_governance_refs` — refs into the upstream nightly rows,
  the governance-side waiver-expiry queue, the waiver-register
  entries, the support-export packets, and the release-evidence
  packets (§11).
- `review_cycle_export_fields` — typed booleans for every review
  cycle and every consuming packet (§12).
- `review_cycles_covered` — non-empty list of review-cycle classes
  the row was assembled for; consistent with the export-field
  booleans.

## 2. Stable IDs and human-readable copy

Rows carry both:

- machine-stable ids — `benchmark_waiver_row_id`, the protected-
  path refs, the corpus / hardware / environment refs, the waiver
  refs, the recurrence-cluster refs, the correction-work refs, the
  release / milestone refs, and the linked-governance refs; and
- bounded reviewable copy — `headline_label`, `identity_summary`,
  `last_qualifying_run_state_summary`, `freshness_summary`,
  `waiver_summary`, `compare_summary`, `recurrence_summary`,
  `correction_trigger_summary`, and `impact_summary`.

Surfaces render the copy verbatim and quote the IDs as refs. Tooling
consumes the IDs and the typed class fields without parsing the
copy. Raw measurement bytes, raw evidence bodies, raw waiver
justifications, and raw user identifiers MUST NOT appear; the record
carries opaque refs, typed vocabulary, integer counters, and bounded
reviewable summaries only.

## 3. Protected-path envelope

`protected_path` carries:

- `fitness_function_row_ref` — opaque ref into
  [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)
  `rows[].id` (e.g. `ff.warm_start_to_first_paint`,
  `ff.first_paint`, `ff.input_to_paint`, `ff.buffer_operations`,
  `ff.vfs_save_conflict_handling`, `ff.benchmark_lab_health`,
  `ff.power_thermal_posture`, `ff.restore_fidelity`,
  `ff.command_parity`).
- `metric_family_class` — typed metric family (§3.1).
- `protected_metrics_row_ref` — opaque ref into
  [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml)
  threshold rows.
- `fitness_function_catalog_revision` and
  `protected_metrics_revision` — integer revision pins so the row
  cannot float free of the catalog the metric was promised against.

### 3.1 Metric-family vocabulary

| Class | Meaning |
| --- | --- |
| `latency_warm_start_to_first_paint` | Warm-start to first-paint latency family (`ff.warm_start_to_first_paint`). |
| `latency_first_paint` | First-paint latency family on first-run / cold-start surfaces (`ff.first_paint`). |
| `latency_input_to_paint` | Input-to-paint latency family (`ff.input_to_paint`). |
| `latency_buffer_operations` | Buffer / large-file latency family (`ff.buffer_operations`). |
| `throughput_save_pipeline` | VFS save / conflict-handling throughput family (`ff.vfs_save_conflict_handling`). |
| `ratio_command_parity` | Command-parity ratio family (`ff.command_parity`). |
| `ratio_restore_fidelity` | Restore-fidelity ratio family (`ff.restore_fidelity`). |
| `count_benchmark_lab_health` | Benchmark-lab health count family (`ff.benchmark_lab_health`). |
| `boolean_or_count_power_thermal_posture` | Power / thermal posture family (`ff.power_thermal_posture`). |
| `structural_digest_other` | Structural-digest rows (no numeric value). |
| `other_named_in_summary` | Family not yet typed; admissible only when `headline_label` and `identity_summary` name the family. |

A row whose metric family cannot be typed denies with
`metric_family_class_unresolved` rather than defaulting.

## 4. Corpus / profile / hardware identity envelope

`corpus_profile_identity` carries the same five identity classes
frozen on the governance-side
[`fitness_dashboard_contract.md` §6](../governance/fitness_dashboard_contract.md):
`reference_hardware`, `design_partner_workspace`, `air_gapped_lab`,
`managed_saas_ring`, and `general_corpus_no_profile_pin`. The perf-
side envelope additionally pins:

- `corpus_refs` — corpus ids the rolling baseline window resolved
  against.
- `corpus_manifest_revision` — integer revision pin for
  [`/fixtures/benchmarks/corpus_manifest.yaml`](../../fixtures/benchmarks/corpus_manifest.yaml).
- `profile_pin_ref` — opaque ref pinning the named reference
  surface.
- `hardware_definition_ref` and `hardware_definition_revision` —
  hardware-definition row pinned through
  [`/artifacts/perf/reference_hardware_manifest.yaml`](../../artifacts/perf/reference_hardware_manifest.yaml).
- `environment_definition_ref` and
  `environment_definition_revision` — benchmark-environment row
  pinned through
  [`/artifacts/perf/lab_image_manifest.yaml`](../../artifacts/perf/lab_image_manifest.yaml).
- `is_council_approved_baseline` — true when the hardware /
  environment is on the benchmark-council-approved baseline list.
  False rows MUST NOT claim
  `dashboard_row_state_class = drift_breaches_release_floor_no_waiver`
  against a council-approved baseline; they may only warn or carry
  an explicit waiver.
- `identity_summary` — one-sentence reviewable identity summary.

Schema-enforced rules:

- `corpus_profile_identity_class` in `{reference_hardware,
  design_partner_workspace, air_gapped_lab, managed_saas_ring}`
  requires non-null `profile_pin_ref`, `hardware_definition_ref`,
  `hardware_definition_revision`, `environment_definition_ref`,
  `environment_definition_revision`, and a non-empty `corpus_refs`.
- `comparison_envelope_class` (on `drift_triage`) MUST equal
  `corpus_profile_identity_class`; a baseline comparison cannot
  widen across reference surfaces.

## 5. Current-state vocabulary

The closed nine-class `dashboard_row_state_class`:

| Class | Meaning |
| --- | --- |
| `passing_no_waiver_no_drift` | Latest qualifying run met every threshold; no active waiver, no drift inside the warning band, no chronic-drift cluster. |
| `drift_inside_warning_band_no_waiver` | Drift inside the declared warning band against the recent-baseline window; no active waiver. |
| `drift_inside_failure_band_no_waiver` | Drift past the warning band but not yet a release blocker; no active waiver. |
| `drift_breaches_release_floor_no_waiver` | Release-floor breach with no active waiver; the row is gated as a release blocker. |
| `drift_breaches_release_floor_active_waiver` | Release-floor breach held open by an active waiver; row MUST cite a non-null `waiver_envelope.waiver_record_ref`. |
| `drift_breaches_release_floor_expired_waiver` | Release-floor breach on a row whose waiver expired without renewal; row MUST cite the previous waiver and MUST NOT render as passing on any consuming surface. |
| `evidence_stale_pending_rerun` | Recent-baseline window did not capture qualifying evidence within the freshness ceiling, or a named rerun trigger fired. |
| `not_run_due_to_blocker` | Recurring run did not execute (broken corpus, frozen lane, missing capability, hardware-definition drift). |
| `provisional_threshold_pending_seed` | Protected-metrics threshold for this row is still flagged `to_be_set_by_benchmark_council`; the row may only warn, never fail. |

Schema-enforced pairings:

- `passing_no_waiver_no_drift` requires
  `waiver_cause_class = no_active_waiver`,
  `expiry_proximity_class = no_active_waiver`,
  `mitigation_status_class = no_mitigation_required`,
  `drift_direction_class` in `{improving, stable_within_band}`,
  `noise_class = not_applicable_passing_row`,
  `correction_trigger_class = no_correction_required`, and
  `evidence_freshness_class` in `{fresh, near_expiry}`.
- `drift_breaches_release_floor_no_waiver` requires
  `compare_result_class = drift_breaches_release_floor`,
  `waiver_cause_class = no_active_waiver`, and a non-empty
  `linked_governance_refs.nightly_report_row_refs`.
- `drift_breaches_release_floor_active_waiver` requires
  `compare_result_class = drift_breaches_release_floor`,
  `waiver_cause_class != no_active_waiver`,
  `expiry_proximity_class` out of
  `{no_active_waiver, expired_past_due}`, a non-null
  `waiver_record_ref`, and a non-empty
  `linked_governance_refs.waiver_expiry_item_refs`.
- `drift_breaches_release_floor_expired_waiver` requires
  `compare_result_class = drift_breaches_release_floor`,
  `expiry_proximity_class = expired_past_due`, a non-null
  `waiver_record_ref`, and a non-empty
  `linked_governance_refs.waiver_expiry_item_refs`.
- `evidence_stale_pending_rerun` requires `evidence_freshness_class`
  in `{stale_by_time, stale_by_trigger, missing}`.
- `provisional_threshold_pending_seed` requires
  `evidence_freshness_class = not_applicable_provisional` and
  `waiver_cause_class` in
  `{threshold_provisional_pending_council, no_active_waiver}`.

## 6. Waiver envelope

The waiver envelope captures the protected-function-level rolled-up
view of the waiver lifecycle. A passing row carries
`waiver_cause_class = no_active_waiver`,
`expiry_proximity_class = no_active_waiver`,
`mitigation_status_class = no_mitigation_required`, and null
`waiver_record_ref` / `waiver_expiry_item_ref` /
`waiver_authority_ref` / `expires_at`.

### 6.1 Waiver-cause vocabulary

The closed thirteen-class `waiver_cause_class`:

| Class | Meaning |
| --- | --- |
| `no_active_waiver` | No waiver covers the row. Admissible only on passing / no-waiver dashboard states. |
| `corpus_revision_drift` | Waiver minted because a corpus-manifest revision change widened the row's spread on the same hardware. |
| `hardware_definition_drift` | Waiver minted because the hardware definition changed (laptop refresh, SKU substitution, council-rebaselining). |
| `environment_definition_drift` | Waiver minted because the lab-image, calibration, or power / thermal posture changed. |
| `toolchain_pin_drift` | Waiver minted because the rust-toolchain pin or Cargo lockfile drifted between otherwise-identical runs. |
| `threshold_provisional_pending_council` | Waiver minted because the protected-metrics threshold is still flagged `to_be_set_by_benchmark_council`. |
| `dependency_blocker_pending` | Waiver minted because an upstream dependency blocker prevents capture or fix. |
| `third_party_runtime_blocker` | Waiver minted because a third-party runtime regression (rustc, system framework, GPU stack) gates the protected metric. |
| `infrastructure_outage` | Waiver minted because the benchmark-lab infrastructure is degraded (host reboot, network outage, storage failure). |
| `claim_attestation_pending` | Waiver minted because a public claim attestation is in flight; the row is held open until the attestation lands. |
| `review_outcome_incomplete` | Waiver minted because a council review is in flight and the outcome is incomplete. |
| `single_maintainer_backup` | Standing waiver under the solo-maintainer backup posture; no expiry pinned. |
| `other_named_in_summary` | Cause not yet typed; admissible only when `waiver_summary` names the cause. |

### 6.2 Expiry-proximity vocabulary

The closed seven-class `expiry_proximity_class` mirrors the
governance-side waiver-expiry-queue contract and adds a
`no_active_waiver` entry for passing rows:

| Class | Meaning |
| --- | --- |
| `no_active_waiver` | Row carries no active waiver; admissible only on passing / no-waiver dashboard states. |
| `expired_past_due` | `expires_at` strictly in the past; row gates release / claim publication. |
| `due_today_or_within_24h` | `expires_at` within 24 hours of `evaluated_at`. |
| `nearing_expiry_within_seven_days` | `expires_at` between 24 hours and seven days from `evaluated_at`. |
| `nearing_expiry_within_thirty_days` | `expires_at` between seven and thirty days from `evaluated_at`. |
| `not_yet_due_review_window_open` | `expires_at` more than thirty days from `evaluated_at`. |
| `no_expiry_pinned` | Standing waiver with no expiry; admissible only for `single_maintainer_backup` or `other_named_in_summary`. |

### 6.3 Mitigation-status vocabulary

The closed six-class `mitigation_status_class` mirrors the
governance-side waiver-expiry-queue contract and adds a
`no_mitigation_required` entry for passing rows:

| Class | Meaning |
| --- | --- |
| `no_mitigation_required` | Row is passing or held by a standing audit-only waiver; no mitigation owed. |
| `mitigation_in_flight_owner_named` | Mitigation work is in flight; primary DRI is named and the work is moving. |
| `mitigation_blocked_pending_decision` | Mitigation blocked pending a council decision. |
| `mitigation_not_started_pending_owner` | Mitigation has not started; ownership unresolved or the named owner has not yet acked. |
| `mitigation_complete_renewal_or_retire_pending` | Mitigation work complete; the queue item is open only because the renewal / retirement has not yet been minted. |
| `mitigation_not_required_audit_only` | Row open only for audit visibility (e.g. standing single-maintainer-backup waiver). |

## 7. Drift-triage envelope

The drift-triage envelope carries the rolled-up trend direction over
the recent-baseline window, the typed noise class, the compare
action / result against the rolling baseline, the comparison
envelope, the baseline-window span and run count, and the prior-
baseline packet refs.

### 7.1 Drift-direction vocabulary

The closed six-class `drift_direction_class`:

| Class | Meaning |
| --- | --- |
| `improving` | Rolled-up trend over the baseline window is improving. |
| `regressing` | Rolled-up trend over the baseline window is regressing. |
| `stable_within_band` | Rolled-up trend is flat inside the warning band. |
| `noisy_high_variance` | Rolled-up trend is dominated by high variance with no consistent direction. |
| `unknown_insufficient_history` | Baseline window has fewer qualifying runs than the dashboard requires for a direction. |
| `unknown_not_comparable` | Baseline window cannot be compared (corpus / profile / hardware drift). |

### 7.2 Noise vocabulary

The closed six-class `noise_class`:

| Class | Meaning |
| --- | --- |
| `low_noise_chronic_drift` | Coefficient of variation low enough that the rolled-up direction is reliable; the drift is chronic. |
| `high_noise_isolated_failure` | Coefficient of variation high enough that the rolled-up direction is dominated by an isolated failure rather than a chronic pattern. |
| `mixed_chronic_drift_with_noise` | Both signals present; surfaces render the direction with a "watch" hint. |
| `insufficient_samples_to_evaluate` | `baseline_run_count = 0`; the noise class cannot be evaluated. |
| `not_evaluated_provisional_threshold` | Threshold is provisional; noise is not evaluated until the council seeds the threshold. |
| `not_applicable_passing_row` | Row is on a passing / provisional state; admissible only when `dashboard_row_state_class` is in `{passing_no_waiver_no_drift, provisional_threshold_pending_seed}`. |

A row that collapses chronic drift and isolated noisy failures into
a single `recurring` chip is non-conforming.

### 7.3 Compare-action and compare-result vocabulary

The drift-triage envelope reuses the compare-action and compare-
result vocabularies from the governance-side nightly-row contract,
specialised to the rolling baseline window the dashboard renders:

- `compare_action_class`:
  `compare_against_prior_baseline_window`,
  `compare_against_release_floor`,
  `compare_against_corpus_revision_floor`,
  `compare_against_threshold_only_no_baseline`,
  `compare_skipped_baseline_drift`,
  `compare_blocked_by_drift_in_corpus_or_profile`.
- `compare_result_class`: `no_drift_inside_warning_band`,
  `drift_within_warning_band`,
  `drift_within_failure_band_no_waiver`,
  `drift_breaches_release_floor`,
  `new_baseline_required_seed_pending`, `compare_skipped`.

Schema-enforced rules:

- `compare_action_class = compare_against_prior_baseline_window`
  requires a non-empty `prior_baseline_packet_refs` and
  `baseline_run_count >= 1`.
- `compare_action_class` in `{compare_skipped_baseline_drift,
  compare_blocked_by_drift_in_corpus_or_profile,
  compare_against_threshold_only_no_baseline}` forces
  `compare_result_class` to `compare_skipped` or
  `new_baseline_required_seed_pending`.
- `baseline_run_count = 0` forces
  `noise_class = insufficient_samples_to_evaluate` and
  `drift_direction_class` in
  `{unknown_insufficient_history, unknown_not_comparable}`.

## 8. Recurrence envelope

The recurrence envelope carries the typed
`recurrence_window_class`, the integer `recurrence_count`, the
`recurrence_window_span` (ISO 8601 duration), the typed
`recurring_waiver_cluster_class` and
`repeated_protected_path_regression_class` mirrored from the
governance-side nightly-row contract, and the `cluster_member_refs`
naming the other rows that share the cluster.

### 8.1 Recurrence-window vocabulary

The closed five-class `recurrence_window_class`:

| Class | Meaning |
| --- | --- |
| `first_observation_in_window` | First observation of this protected path in the recurrence window; `recurrence_count = 1`. |
| `recurring_within_milestone` | Two or more waivers / breach observations on the same protected path inside the current milestone. |
| `recurring_across_milestones` | Two or more observations across distinct milestones; chronic drift. |
| `recurring_across_release_lines` | Two or more observations across distinct release lines (stable, LTS, preview); the row MUST be escalated to release-council or shiproom-executive correction work. |
| `no_history_window_pending_baseline` | The recurrence window is empty because the row is being seeded; `recurrence_count = 0`. |

Schema-enforced rules:

- `recurrence_window_class` in `{recurring_within_milestone,
  recurring_across_milestones, recurring_across_release_lines}`
  requires `recurrence_count >= 2` and a non-empty
  `cluster_member_refs`.
- `recurrence_window_class = recurring_across_release_lines`
  requires `correction_owner_role_class` in
  `{release_council, shiproom_executive}`.

### 8.2 Recurring-waiver and repeated-protected-path-regression
clusters

The recurrence envelope mirrors the governance-side cluster
vocabularies so the perf dashboard, the nightly row, and the
waiver-expiry queue agree on whether a row is recurring:

- `recurring_waiver_cluster_class`: `not_recurring_first_observation`,
  `recurring_within_quarter`, `recurring_across_quarters`,
  `recurring_across_protected_paths`,
  `not_applicable_no_waiver_history`.
- `repeated_protected_path_regression_class`:
  `no_regression_first_observation`,
  `repeated_protected_path_within_milestone`,
  `repeated_protected_path_across_milestones`,
  `repeated_protected_path_across_release_lines`,
  `not_applicable_no_protected_path_pin`.

A non-default value on either cluster class requires a non-empty
`cluster_member_refs`.

## 9. Correction-trigger envelope

The correction-trigger envelope is the contract's primary
mechanism for escalating chronic drift and repeated waivers into
staffed correction work. It mirrors the action-class vocabulary on
[`/artifacts/governance/correction_trigger_table.yaml`](../../artifacts/governance/correction_trigger_table.yaml)
and adds typed owner-role and response-window fields.

### 9.1 Correction-trigger vocabulary

The closed six-class `correction_trigger_class`:

| Class | Meaning |
| --- | --- |
| `no_correction_required` | Row is passing or its drift / waiver state has not crossed the correction-trigger threshold. |
| `correction_owed_lane_dri` | Correction work owed by the lane DRI (single-lane fix, rebaseline against new corpus). |
| `correction_owed_decision_forum` | Correction work owed by a decision forum (performance council, architecture council, benchmark council). |
| `correction_owed_release_council` | Correction work owed by the release council; cross-release impact. |
| `correction_owed_shiproom_executive` | Correction work owed by shiproom executive scope review; cross-release-line / cross-product impact. |
| `correction_complete_pending_retire` | Correction work landed; the row is open only until the waiver renewal or retirement is minted. |

### 9.2 Correction-owner role vocabulary

The closed eight-class `correction_owner_role_class`:

| Class | Meaning |
| --- | --- |
| `no_owner_no_correction_required` | No correction owner; admissible only when `correction_trigger_class = no_correction_required`. |
| `lane_dri` | Lane DRI is the correction owner; non-null `correction_owner_dri_ref` and `correction_owner_lane_ref` required. |
| `lane_dri_with_backup_owner` | Lane DRI plus a named backup owner (multi-maintainer lane). |
| `performance_council` | Performance council is the correction owner. |
| `architecture_council` | Architecture council is the correction owner. |
| `release_council` | Release council is the correction owner. |
| `shiproom_executive` | Shiproom executive scope review is the correction owner. |
| `benchmark_council` | Benchmark council is the correction owner. |

Schema-enforced rules:

- `correction_trigger_class = no_correction_required` requires
  `correction_owner_role_class = no_owner_no_correction_required`,
  an empty `correction_work_refs`, and a null
  `response_window_business_days`.
- `correction_trigger_class` in `{correction_owed_lane_dri,
  correction_owed_decision_forum, correction_owed_release_council,
  correction_owed_shiproom_executive,
  correction_complete_pending_retire}` requires a non-empty
  `correction_work_refs` and a non-null
  `response_window_business_days`.
- `correction_trigger_class = correction_owed_lane_dri` requires
  `correction_owner_role_class` in `{lane_dri,
  lane_dri_with_backup_owner}` and non-null
  `correction_owner_dri_ref` + `correction_owner_lane_ref`.
- `correction_trigger_class = correction_owed_decision_forum`
  requires `correction_owner_role_class` in `{performance_council,
  architecture_council, benchmark_council}` and a non-null
  `decision_forum_ref`.
- `correction_trigger_class = correction_owed_release_council`
  requires `correction_owner_role_class = release_council`.
- `correction_trigger_class = correction_owed_shiproom_executive`
  requires `correction_owner_role_class = shiproom_executive`.
- `recurrence_window_class = recurring_across_release_lines`
  requires `correction_owner_role_class` in
  `{release_council, shiproom_executive}`.

## 10. Release / milestone impact

`release_or_milestone_impact` carries:

- `affected_release_or_milestone_refs` — non-empty refs into
  release-line ids, milestone ids, or LTS-train ids the row gates.
- `linked_claim_refs` — refs into
  [`/artifacts/governance/claim_manifest_seed.yaml`](../../artifacts/governance/claim_manifest_seed.yaml)
  rows the row gates.
- `linked_compatibility_surface_refs` — refs into the compatibility-
  surface inventory rows the metric family pairs with.
- `impact_summary` — one-sentence reviewable impact summary.

## 11. Linked-governance refs

`linked_governance_refs` carries refs into the upstream nightly-row
ids, the governance-side waiver-expiry-queue items, the waiver-
register entries, the support-export packets, and the release-
evidence shiproom packets. The schema's `allOf` block enforces:

- `dashboard_row_state_class` in
  `{drift_inside_warning_band_no_waiver,
  drift_inside_failure_band_no_waiver,
  drift_breaches_release_floor_no_waiver,
  drift_breaches_release_floor_active_waiver,
  drift_breaches_release_floor_expired_waiver,
  evidence_stale_pending_rerun, not_run_due_to_blocker}` requires a
  non-empty `nightly_report_row_refs`.
- `dashboard_row_state_class` in
  `{drift_breaches_release_floor_active_waiver,
  drift_breaches_release_floor_expired_waiver}` requires a non-empty
  `waiver_expiry_item_refs`.

## 12. Review-cycle export parity

Every consuming review and every consuming packet MUST render the
same fields. The parity floor is enforced by the schema's `allOf`
block. `review_cycle_export_fields` carries typed booleans for:

- `render_on_nightly_review`,
- `render_on_weekly_governance_review`,
- `render_on_release_candidate_review`,
- `render_on_milestone_close_review`,
- `render_on_support_escalation_review`,
- `render_on_release_packet`,
- `render_on_support_export`,
- `render_on_governance_packet`,
- `render_on_claim_manifest`.

`review_cycles_covered` carries the closed list of review-cycle
classes the row was assembled for and MUST be consistent with the
booleans (a row in `review_cycles_covered` MUST have the
corresponding `render_on_*` boolean set to true).

Required on every consuming surface:

- `benchmark_waiver_row_id`, `headline_label`, `evaluated_at`;
- `protected_path.fitness_function_row_ref`,
  `protected_path.metric_family_class`,
  `protected_path.protected_metrics_row_ref`;
- `corpus_profile_identity.corpus_profile_identity_class`,
  `corpus_profile_identity.profile_pin_ref`,
  `corpus_profile_identity.hardware_definition_ref`,
  `corpus_profile_identity.environment_definition_ref`,
  `corpus_profile_identity.is_council_approved_baseline`;
- `current_state.dashboard_row_state_class`,
  `current_state.last_qualifying_run_ref`,
  `current_state.last_qualifying_run_at`,
  `current_state.next_expected_run_at`;
- `evidence_freshness.evidence_freshness_class`,
  `evidence_freshness.captured_at`,
  `evidence_freshness.expires_at`;
- `waiver_envelope.waiver_cause_class`,
  `waiver_envelope.expiry_proximity_class`,
  `waiver_envelope.mitigation_status_class`,
  `waiver_envelope.expires_at`,
  `waiver_envelope.waiver_summary`;
- `drift_triage.drift_direction_class`,
  `drift_triage.noise_class`,
  `drift_triage.compare_action_class`,
  `drift_triage.compare_result_class`,
  `drift_triage.comparison_envelope_class`,
  `drift_triage.compare_summary`;
- `recurrence_envelope.recurrence_window_class`,
  `recurrence_envelope.recurrence_count`,
  `recurrence_envelope.recurring_waiver_cluster_class`,
  `recurrence_envelope.repeated_protected_path_regression_class`,
  `recurrence_envelope.recurrence_summary`;
- `correction_trigger.correction_trigger_class`,
  `correction_trigger.correction_owner_role_class`,
  `correction_trigger.response_window_business_days`,
  `correction_trigger.correction_trigger_summary`;
- `release_or_milestone_impact.affected_release_or_milestone_refs`,
  `release_or_milestone_impact.impact_summary`.

Forbidden collapses on release-packet, support-export, governance-
packet, and claim-manifest surfaces:

- rendering `drift_breaches_release_floor_active_waiver` or
  `drift_breaches_release_floor_expired_waiver` as a clean pass to
  unblock publication;
- dropping `corpus_profile_identity` when widening a row across
  reference surfaces;
- dropping `recurrence_envelope` so chronic drift looks like an
  isolated incident;
- collapsing `low_noise_chronic_drift` and
  `high_noise_isolated_failure` into a single "recurring" label;
- dropping `correction_trigger` so the row reads as "tolerated"
  when staffed correction work has actually been triggered;
- omitting the named rerun trigger on a stale-by-trigger row;
- dropping `compare_summary` so a "drift breaches release floor"
  outcome reads as ambiguous "review pending."

The schema additionally forbids
`drift_breaches_release_floor_expired_waiver` rows from rendering on
the release packet or the claim manifest unless they also render on
the dashboard and the governance packet, so an expired waiver
cannot ride into release / claim publication while being filed away
from the dashboard.

## 13. Authoring rules

When a recurring benchmark run lands fresh evidence and the
benchmark-waiver dashboard re-projects:

1. Resolve `protected_path` against the live fitness-function
   catalog and protected-metrics revisions; copy the typed
   `metric_family_class`.
2. Resolve `corpus_profile_identity` against the corpus manifest,
   reference-hardware manifest, and lab-image manifest; copy the
   typed identity class, the council-baseline flag, and the
   reviewable identity summary.
3. Resolve `current_state.dashboard_row_state_class` against the
   latest qualifying run plus the active-waiver state; the schema's
   `allOf` block enforces every required pairing.
4. Resolve `evidence_freshness` from the proof-class freshness
   ceiling (`P14D` for benchmark publication packs by default).
5. Resolve `waiver_envelope` from the active waiver record (or set
   the no-active-waiver defaults).
6. Resolve `drift_triage` from the rolling baseline window;
   `comparison_envelope_class` MUST equal
   `corpus_profile_identity_class`.
7. Resolve `recurrence_envelope` from the rolling recurrence window;
   `recurrence_count` is the integer count of distinct waiver
   mintings or release-floor breach observations.
8. Resolve `correction_trigger` from the recurrence envelope plus
   the protected-path correction-trigger table; the schema's
   `allOf` block enforces every required pairing between
   `correction_trigger_class`, `correction_owner_role_class`, and
   the DRI / lane / forum refs.
9. Resolve `release_or_milestone_impact` from the upstream
   release-evidence packets, the claim manifest, and the
   compatibility-surface inventory.
10. Resolve `linked_governance_refs` from the upstream nightly
    rows, the governance-side waiver-expiry queue, the waiver-
    register entries, and the consuming release / support
    packets.
11. Resolve `review_cycle_export_fields` and
    `review_cycles_covered` from the consuming review surfaces; the
    two MUST be consistent.
12. Surfaces render the bounded reviewable summaries verbatim; they
    MUST NOT substitute free-text fear copy or "all good" / "all
    bad" placeholders.

## 14. Out of scope

This contract does not implement:

- performance tuning, regression bisect tooling, or a benchmark-
  capture harness;
- a nightly-job runner, an issue-routing automation, a waiver-
  renewal workflow, or a correction-work tracker;
- the protected fitness-function catalog, the protected-metrics
  register, or the correction-trigger table itself (those live in
  [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml),
  [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml),
  and
  [`/artifacts/governance/correction_trigger_table.yaml`](../../artifacts/governance/correction_trigger_table.yaml));
- the underlying waiver records, the governance-side waiver-expiry
  queue, or the waiver register (those live in
  [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml),
  [`/schemas/governance/waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json),
  and
  [`/schemas/governance/waiver_register.schema.json`](../../schemas/governance/waiver_register.schema.json));
- the per-run benchmark record (that lives on
  [`/schemas/benchmarks/run_result.schema.json`](../../schemas/benchmarks/run_result.schema.json)).

This contract is the projection vocabulary that those upstream
artifacts flow through when they reach the benchmark-waiver
dashboard, the milestone-close review, the release-candidate
shiproom packet, the support-escalation export, the weekly
governance review, and the governance packet.
