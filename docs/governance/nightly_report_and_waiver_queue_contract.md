# Nightly-governance report row, waiver-expiry queue, and recurring-evidence compare contract

This contract freezes one shared vocabulary for the nightly-governance
report row, the waiver-expiry queue item, and the compare action that
recurring evidence runs use when they grade a fresh capture against a
prior baseline. It exists so looming waiver expiry and stale recurring
evidence remain visible to the dashboard, the milestone scorecard, the
release-evidence shiproom packet, the support bundle, the public claim
manifest, and the weekly governance review **before** any of those
surfaces silently inherits the prior pass.

The contract is pre-implementation. It defines the reusable record
shapes, the closed vocabularies, the projection rules, the export
parity floor, and the fixture corpus. It does not implement a
nightly-job runner, an issue-routing pipeline, or a waiver-renewal
workflow.

## Companion artifacts

- [`/schemas/governance/nightly_report_row.schema.json`](../../schemas/governance/nightly_report_row.schema.json)
  — boundary schema for one `nightly_report_row_record`.
- [`/schemas/governance/waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json)
  — boundary schema for one `waiver_expiry_item_record`.
- [`/fixtures/governance/nightly_report_cases/`](../../fixtures/governance/nightly_report_cases/)
  — worked records covering fresh recurring pass, partial-run subset,
  waiver nearing expiry, expired-waiver, recurring-waiver cluster, and
  repeated-protected-path regression.
- [`/docs/governance/fitness_dashboard_contract.md`](./fitness_dashboard_contract.md)
  and [`/schemas/governance/fitness_tile.schema.json`](../../schemas/governance/fitness_tile.schema.json)
  — sibling vocabulary for live fitness-tile rendering. The nightly
  report row reuses the corpus / profile identity, evidence freshness,
  waiver authority, and mitigation-note vocabularies frozen there.
- [`/artifacts/governance/evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml)
  and
  [`/artifacts/governance/evidence_rerun_triggers.yaml`](../../artifacts/governance/evidence_rerun_triggers.yaml)
  — proof-class freshness ceilings and named rerun triggers. The
  nightly row's evidence-freshness envelope copies metadata from these
  registers.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — DRI, backup-owner, decision-forum, and waiver register. Every
  `waiver_ref` resolves into this matrix.
- [`/artifacts/policy/waiver_expiry_dashboard_contract.yaml`](../../artifacts/policy/waiver_expiry_dashboard_contract.yaml)
  — upstream policy waiver-expiry dashboard register that the
  governance-side queue items quote and extend with corpus / profile
  and recurring-evidence context.
- [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml),
  [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml),
  and
  [`/artifacts/governance/claim_manifest_seed.yaml`](../../artifacts/governance/claim_manifest_seed.yaml)
  — catalogs that `linked_rule_or_claim_refs` resolve into.

## Normative sources projected here

- `.t2/docs/Aureline_Technical_Architecture_Document.md` §3.4 quality-
  attribute scenarios and measurable SLOs, §22.9 release governance.
- `.t2/docs/Aureline_Technical_Design_Document.md` §8.36 release
  evidence, §8.41 supportability evidence.
- `.t2/docs/Aureline_PRD.md` verification, release-evidence, waiver,
  and compatibility-publication requirements.

If this contract disagrees with those sources, those sources win and
this contract, the schemas, and the fixtures update in the same change.

## Why a nightly-report and waiver-queue contract exists

1. **Late surprises are the failure mode.** Without one shared row
   shape, recurring evidence runs publish their own ad-hoc pass / fail
   counters, waiver expiry becomes a reminder buried in someone's
   calendar, and the release / support reviews discover the gap on the
   morning they need a clean signoff. The contract exists so the
   nightly run, the queue item, the weekly review, the release-evidence
   packet, the support packet, and the governance packet all quote the
   **same** row shape with the **same** stable IDs.
2. **Recurring runs cannot collapse into anonymous pass / fail
   history.** A recurring benchmark capture on the macOS reference-
   laptop profile is not interchangeable with a recurring capture on
   the air-gapped lab. The row carries the corpus / profile identity
   so a "passing" run on one profile cannot widen silently into a
   full-corpus claim.
3. **Comparison against a prior baseline must stay scoped.** Drift is
   only meaningful inside the same corpus / profile envelope. The
   `compare_action_class` and `compare_result_class` vocabularies pin
   how the comparison was made and refuse to let "drift within the
   warning band on macOS" widen into a "drift band breach across the
   declared profile set."
4. **Waivers must surface as queue items before the release / support
   review needs them.** A waiver expiring next week behaves differently
   from a waiver that already lapsed; they cannot share a "needs
   action" chip. The `expiry_proximity_class` carries that distinction
   and the `mitigation_status_class` names what is in flight, what is
   blocked, and what has not yet started.
5. **Chronic drift must be visible as a pattern, not only as isolated
   incidents.** The `recurring_waiver_cluster_class` and the
   `repeated_protected_path_regression_class` flag waivers that keep
   coming back on the same protected path so the weekly review can
   escalate the cluster into correction work rather than rubber-
   stamping each isolated renewal.
6. **Export parity keeps screenshots out of the workflow.** Release
   packets, support bundles, and governance packets need to ship the
   same row data as the dashboard. The schema's `export_fields` block
   enforces a parity floor so the release packet cannot drop the
   corpus / profile identity, the support packet cannot drop the
   waiver expiry, and the governance packet cannot drop the cluster
   indicator.

## 1. Nightly-report row shape

A `nightly_report_row_record` carries:

- `nightly_report_row_id` — stable, machine-readable id quoted by
  every consuming surface.
- `report_type_class` — typed report type (recurring benchmark report,
  recurring compatibility report, recurring supportability drill,
  recurring security packet, recurring migration packet, recurring
  claim attestation; §3).
- `report_run_chronology` — `started_at`, `completed_at`, and the run
  id from the upstream evidence packet.
- `run_state_class` — typed run state (§4).
- `corpus_profile_identity` — the same envelope frozen in
  [`fitness_dashboard_contract.md` §6](./fitness_dashboard_contract.md):
  `corpus_profile_identity_class`, `corpus_refs`, `profile_pin_ref`,
  `partial_profile_result_class`, and a bounded reviewable
  `identity_summary`.
- `compare_snapshot` — typed `compare_action_class`, typed
  `compare_result_class`, the `prior_baseline_packet_ref`, the named
  `comparison_envelope_class`, and a bounded reviewable
  `compare_summary` (§5).
- `evidence_freshness` — typed freshness class plus packet ref,
  `captured_at`, `stale_after`, and the computed `expires_at`. Reuses
  the freshness vocabulary from
  [`fitness_dashboard_contract.md` §5](./fitness_dashboard_contract.md).
- `linked_rule_or_claim_refs` — typed refs into the protected fitness-
  function catalog, the protected-metrics register, the policy /
  signoff matrix, the claim manifest, the compatibility surface
  inventory, or the certified-archetype report. A row that names no
  upstream rule or claim is non-conforming.
- `linked_evidence_refs` — refs into the underlying recurring evidence
  packets the row was assembled from.
- `waiver_queue_item_refs` — refs into the waiver-expiry queue items
  attached to this row (zero, one, or many). Required when
  `run_state_class` is `waived_failure`, `waiver_expired_failure`, or
  any state whose `compare_result_class` is gated by an active waiver.
- `mitigation_note` — typed mitigation class plus three bounded
  reviewable sentences (§7) following the same shape as the
  fitness-tile mitigation note.
- `cluster_indicators` — `recurring_waiver_cluster_class` and
  `repeated_protected_path_regression_class` (§8).
- `export_fields` — typed booleans for dashboard, scorecard, release
  packet, support export, governance packet, and claim manifest (§9).

## 2. Stable IDs and human-readable copy

Rows carry both:

- machine-stable ids — `nightly_report_row_id`,
  `evidence_packet_ref`, `prior_baseline_packet_ref`,
  `linked_rule_or_claim_refs`, `waiver_queue_item_refs`,
  `profile_pin_ref`, `corpus_refs`; and
- bounded reviewable copy — `headline_label`, `compare_summary`,
  `freshness_summary`, `identity_summary`, `mitigation_summary`,
  `what_users_should_do`, `what_operators_should_do`.

Surfaces render the copy verbatim and quote the IDs as refs. Tooling
consumes the IDs and the typed class fields without parsing the copy.
Raw measurement bytes, raw evidence bodies, raw waiver justifications,
and raw user identifiers MUST NOT appear; the record carries opaque
refs, typed vocabulary, and bounded reviewable summaries only.

## 3. Report-type vocabulary

The closed six-class report-type vocabulary:

| Class | Meaning |
| --- | --- |
| `recurring_benchmark_report` | Recurring benchmark publication run against the protected fitness-function catalog. |
| `recurring_compatibility_report` | Recurring compatibility-window report run against the compatibility-surface inventory. |
| `recurring_supportability_drill_report` | Recurring supportability-drill capture (issue-report packet, support-export shape, recovery rehearsal). |
| `recurring_security_packet_report` | Recurring security / transport packet capture (signed-pack provenance, code-signing chain, transport posture). |
| `recurring_migration_packet_report` | Recurring migration-helper handoff or compatibility-window report run. |
| `recurring_claim_attestation_report` | Recurring re-attestation of a public claim row in the claim manifest. |

A row whose report type cannot be typed denies with
`report_type_class_unresolved` rather than defaulting.

## 4. Run-state vocabulary

The closed nine-class run-state vocabulary:

| Class | Meaning |
| --- | --- |
| `pass_full_corpus_full_profile` | Recurring run met every threshold and covered the declared corpus / profile set. |
| `pass_partial_profile_named_gap` | Recurring run met every threshold but covered only a subset of the declared profile set; the partial-profile class names the gap. |
| `partial_run_subset_only` | Recurring run completed only a subset of the declared corpus rows or profile slices (lab outage, time budget overrun, infrastructure incident); compare action MUST cite the subset. |
| `blocked_by_failure` | Recurring run captured a fresh threshold breach with no active waiver. |
| `blocked_by_drift` | Recurring run captured drift past the warning band against the prior baseline; threshold not yet breached but trending toward it. |
| `waived_failure` | Recurring run captured a failure or near-failure held open by an active waiver; the row MUST cite a non-null `waiver_queue_item_refs` entry. |
| `waiver_expired_failure` | Recurring run captured a failure on a row whose waiver expired without renewal; the row MUST cite the previous waiver and MUST NOT render as passing on any consuming surface. |
| `evidence_stale` | Recurring run did not capture fresh evidence within the freshness window or a named rerun trigger fired; the prior numeric pass result MUST NOT be projected as authoritative. |
| `not_run_due_to_blocker` | Recurring run did not execute because an upstream blocker (broken corpus, frozen lane, missing capability) prevented capture. |

`pass_full_corpus_full_profile` requires `partial_profile_result_class
= not_partial_full_corpus_run`. Every other state admits or requires a
non-default partial-profile class as documented in §6.

## 5. Compare action and compare result

`compare_snapshot` carries:

- `compare_action_class` — how the run was compared (closed six-class
  vocabulary):

  | Class | Meaning |
  | --- | --- |
  | `compare_against_prior_baseline` | Compared against the prior recurring run on the same corpus / profile envelope. `prior_baseline_packet_ref` required. |
  | `compare_against_release_floor` | Compared against the release-bar threshold from the protected fitness-function catalog. |
  | `compare_against_corpus_revision_floor` | Compared against the floor declared for the named corpus revision. |
  | `compare_against_threshold_only_no_baseline` | First run on this row with no prior baseline; only the catalog threshold was applied. |
  | `compare_skipped_baseline_drift` | Comparison skipped because the prior baseline is on a different corpus or profile and a comparable baseline does not yet exist. |
  | `compare_blocked_by_drift_in_corpus_or_profile` | Comparison blocked because the corpus or profile pin changed since the prior baseline; a new baseline must be seeded. |

- `compare_result_class` — the compare outcome (closed six-class
  vocabulary):

  | Class | Meaning |
  | --- | --- |
  | `no_drift_inside_warning_band` | Drift inside the declared warning band on the same corpus / profile envelope. |
  | `drift_within_warning_band` | Drift inside the warning band but trending toward the failure band; surfaces MAY decorate with a watch hint. |
  | `drift_within_failure_band_no_waiver` | Drift past the warning band but not yet a release blocker; no active waiver. |
  | `drift_breaches_release_floor` | Drift breaches the release-floor threshold; row MUST be `blocked_by_failure`, `waived_failure`, or `waiver_expired_failure`. |
  | `new_baseline_required_seed_pending` | The prior baseline is no longer comparable; a new baseline must be seeded before drift can be evaluated again. |
  | `compare_skipped` | Compare action was skipped (`compare_skipped_baseline_drift`, `compare_blocked_by_drift_in_corpus_or_profile`, `compare_against_threshold_only_no_baseline`). |

- `comparison_envelope_class` — the corpus / profile envelope the
  comparison was made against (closed five-class vocabulary mirroring
  `corpus_profile_identity_class`): `reference_hardware`,
  `design_partner_workspace`, `air_gapped_lab`, `managed_saas_ring`,
  `general_corpus_no_profile_pin`. The compare envelope MUST equal the
  row's `corpus_profile_identity_class` so a comparison cannot drift
  silently across reference surfaces.

- `prior_baseline_packet_ref` — required when `compare_action_class =
  compare_against_prior_baseline`; null otherwise.

- `compare_summary` — one bounded reviewable sentence naming the
  comparison outcome and the envelope; raw measurement bytes do not
  appear.

Schema-enforced rules:

- `compare_action_class = compare_against_prior_baseline` requires a
  non-null `prior_baseline_packet_ref`.
- `compare_action_class = compare_skipped_baseline_drift`,
  `compare_blocked_by_drift_in_corpus_or_profile`, or
  `compare_against_threshold_only_no_baseline` forces
  `compare_result_class` to `compare_skipped` or
  `new_baseline_required_seed_pending`.
- `run_state_class = blocked_by_drift` requires
  `compare_result_class` in
  `{drift_within_warning_band, drift_within_failure_band_no_waiver}`.
- `run_state_class` in `{blocked_by_failure, waived_failure,
  waiver_expired_failure}` requires `compare_result_class =
  drift_breaches_release_floor`.

## 6. Corpus / profile identity and partial-profile rules

The corpus / profile identity envelope is identical to the one frozen
in [`fitness_dashboard_contract.md` §6](./fitness_dashboard_contract.md):
the same five identity classes, the same four partial-profile classes,
and the same pairing rules.

Schema-enforced pairings on the nightly row:

- `partial_profile_result_class = not_partial_full_corpus_run`
  requires `run_state_class` to be one of
  `pass_full_corpus_full_profile`, `blocked_by_failure`,
  `blocked_by_drift`, `waived_failure`, `waiver_expired_failure`,
  `evidence_stale`, or `not_run_due_to_blocker`.
- `partial_profile_result_class` in
  `{single_profile_only, subset_of_profiles_covered,
  subset_of_corpus_rows_covered}` forbids
  `run_state_class = pass_full_corpus_full_profile` and forces the
  run state to `pass_partial_profile_named_gap`,
  `partial_run_subset_only`, `blocked_by_drift`, `waived_failure`,
  `waiver_expired_failure`, or `evidence_stale`.
- `corpus_profile_identity_class` in
  `{reference_hardware, design_partner_workspace, air_gapped_lab,
  managed_saas_ring}` requires a non-null `profile_pin_ref` and a
  non-empty `corpus_refs` array, mirroring the fitness-tile rule.

## 7. Mitigation note

The mitigation note carries the same shape as the fitness-tile
mitigation block: one typed `mitigation_note_class`,
`what_users_should_do`, `what_operators_should_do`, and
`mitigation_summary`. The closed nine-class vocabulary is the same
one frozen in [`fitness_dashboard_contract.md` §8](./fitness_dashboard_contract.md):
`no_mitigation_required_passing`, `narrower_scope_until_refresh`,
`slower_path_active_until_refresh`, `less_portable_temporarily`,
`temporarily_blocked_pending_owner_action`,
`waiver_holds_release_until_expiry`,
`partial_profile_result_pending_full_capture`,
`provisional_no_action_until_seeded`,
`early_signal_drift_owner_inspecting`.

Schema-enforced pairings:

- `pass_full_corpus_full_profile` MUST resolve to
  `no_mitigation_required_passing`.
- `pass_partial_profile_named_gap` and `partial_run_subset_only` MUST
  resolve to `partial_profile_result_pending_full_capture` (or
  `waiver_holds_release_until_expiry` when an active waiver covers the
  gap).
- `blocked_by_drift` MUST resolve to
  `early_signal_drift_owner_inspecting`,
  `narrower_scope_until_refresh`, or
  `partial_profile_result_pending_full_capture`.
- `blocked_by_failure` MUST resolve to
  `temporarily_blocked_pending_owner_action`,
  `narrower_scope_until_refresh`,
  `slower_path_active_until_refresh`, or
  `less_portable_temporarily`.
- `waived_failure` MUST resolve to
  `waiver_holds_release_until_expiry`.
- `waiver_expired_failure` MUST resolve to
  `temporarily_blocked_pending_owner_action`.
- `evidence_stale` MUST resolve to
  `narrower_scope_until_refresh`,
  `slower_path_active_until_refresh`, `less_portable_temporarily`, or
  `partial_profile_result_pending_full_capture`.
- `not_run_due_to_blocker` MUST resolve to
  `temporarily_blocked_pending_owner_action`.

## 8. Cluster and regression indicators

`cluster_indicators` carries two typed axes that the weekly governance
review uses to escalate chronic drift into correction work.

`recurring_waiver_cluster_class`:

| Class | Meaning |
| --- | --- |
| `not_recurring_first_observation` | First observation of this waiver authority on this row in the current observation window. |
| `recurring_within_quarter` | Two or more waivers minted on the same row inside the same quarter. |
| `recurring_across_quarters` | Three or more waivers minted on the same row across distinct quarters; the council MUST review the cluster as a pattern. |
| `recurring_across_protected_paths` | The same waiver authority has minted waivers on at least two distinct protected paths within the observation window; surfaces render the cluster as a portfolio risk. |
| `not_applicable_no_waiver_history` | The row has never carried a waiver. Admissible only when `run_state_class` is not in `{waived_failure, waiver_expired_failure}`. |

`repeated_protected_path_regression_class`:

| Class | Meaning |
| --- | --- |
| `no_regression_first_observation` | First observation of a regression on this protected path. |
| `repeated_protected_path_within_milestone` | Same protected path regressed at least twice inside the current milestone. |
| `repeated_protected_path_across_milestones` | Same protected path regressed across distinct milestones; surfaces render this as chronic drift. |
| `repeated_protected_path_across_release_lines` | Same protected path regressed across distinct release lines (stable / LTS); the cluster MUST be escalated to the release council. |
| `not_applicable_no_protected_path_pin` | Row is not pinned to a protected path (e.g. supportability drill on an internal tooling lane). |

Schema-enforced pairings:

- `recurring_waiver_cluster_class != not_applicable_no_waiver_history`
  requires a non-empty `cluster_member_refs` array citing the other
  nightly-row ids that share the cluster.
- `repeated_protected_path_regression_class !=
  not_applicable_no_protected_path_pin` requires a non-null
  `protected_path_ref`.
- `repeated_protected_path_regression_class =
  repeated_protected_path_across_release_lines` requires the row's
  `mitigation_note_class` to be
  `temporarily_blocked_pending_owner_action` or
  `waiver_holds_release_until_expiry` and the `escalation_path_class`
  on every linked waiver-queue item to be `escalate_to_release_council`
  or `escalate_to_shiproom`.

## 9. Export parity

Every surface that renders the nightly row MUST render the same fields.
The parity floor is enforced by the schema's `allOf` block.

Required on every consuming surface:

- `nightly_report_row_id`, `report_type_class`,
  `run_state_class`, `headline_label`;
- `report_run_chronology.started_at`,
  `report_run_chronology.completed_at`;
- `corpus_profile_identity.corpus_profile_identity_class`,
  `corpus_profile_identity.partial_profile_result_class`;
- `compare_snapshot.compare_action_class`,
  `compare_snapshot.compare_result_class`,
  `compare_snapshot.comparison_envelope_class`,
  `compare_snapshot.compare_summary`;
- `evidence_freshness.evidence_freshness_class`,
  `evidence_freshness.captured_at`,
  `evidence_freshness.expires_at`;
- `mitigation_note.mitigation_note_class`,
  `mitigation_note.mitigation_summary`;
- `cluster_indicators.recurring_waiver_cluster_class`,
  `cluster_indicators.repeated_protected_path_regression_class`.

Per-state required extras:

- `waived_failure` and `waiver_expired_failure` — at least one entry
  in `waiver_queue_item_refs`.
- `evidence_stale` — `evidence_freshness.rerun_trigger_ref` (when the
  freshness class is `stale_by_trigger`),
  `evidence_freshness.freshness_summary`.
- `partial_run_subset_only` and `pass_partial_profile_named_gap` —
  `compare_snapshot.compare_summary` MUST name the captured subset.

Forbidden collapses on release-packet, support-packet, and claim-
manifest surfaces:

- rendering `waived_failure` or `waiver_expired_failure` as a clean
  pass to unblock publication;
- dropping `corpus_profile_identity` when widening a row across
  profiles;
- dropping `cluster_indicators` so chronic drift looks like an
  isolated incident;
- omitting the named rerun trigger on a stale-by-trigger row;
- dropping `compare_snapshot.compare_summary` so a "drift breaches
  release floor" outcome reads as ambiguous "review pending."

## 10. Waiver-expiry queue item shape

A `waiver_expiry_item_record` carries:

- `waiver_expiry_item_id` — stable, machine-readable id quoted by
  every consuming surface.
- `waiver_ref` — opaque ref to the waiver in
  [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  or to a `waiver_expiry_record` on
  [`/schemas/governance/waiver_expiry.schema.json`](../../schemas/governance/waiver_expiry.schema.json).
- `waiver_owner` — primary DRI handle, owning lane, and decision
  forum. No silent blanks.
- `waiver_authority_class` — the same closed five-class vocabulary
  frozen on the fitness tile (`performance_council`,
  `architecture_council`, `release_council`,
  `shiproom_executive_scope_review`, `not_applicable_no_active_waiver`).
- `waiver_kind_class` — typed waiver kind mirroring
  [`waiver_expiry.schema.json`](../../schemas/governance/waiver_expiry.schema.json):
  `freeze_exception`, `policy_narrowing`, `retention_exception`,
  `release_floor_breach_waiver`, `support_drill_pending_waiver`,
  `compatibility_window_waiver`, `claim_attestation_pending_waiver`,
  `single_maintainer_backup`, `other_named_in_summary`.
- `expires_at` — RFC 3339 expiry timestamp from the waiver record.
  Required when `expiry_proximity_class != no_expiry_pinned`.
- `expiry_proximity_class` — typed proximity class (§11).
- `affected_release_or_milestone_refs` — non-empty refs into
  release-line ids, milestone ids, or LTS-train ids.
- `affected_protected_path_refs` — non-empty refs into the protected
  fitness-function catalog, the protected-metrics register, the
  compatibility-surface inventory, the certified-archetype list, or
  the policy / signoff matrix. A queue item that names no protected
  path is non-conforming.
- `mitigation_status_class` — typed mitigation status (§12).
- `linked_evidence_refs` — refs into the evidence packets the queue
  item depends on.
- `linked_nightly_report_row_refs` — refs into the nightly-row ids
  that triggered or are gated by this queue item.
- `recurring_waiver_cluster_class` and
  `repeated_protected_path_regression_class` — same vocabularies as
  on the nightly row (§8); the queue item carries them so the weekly
  review surface can sort by cluster without joining back to the
  nightly row.
- `cluster_member_refs` — refs into other queue items that share the
  cluster.
- `open_detail_action_class` — typed action surface the user opens
  (§13).
- `escalation_path_class` — typed escalation path mirroring
  [`waiver_expiry.schema.json`](../../schemas/governance/waiver_expiry.schema.json):
  `escalate_to_dri`, `escalate_to_backup_owner`,
  `escalate_to_decision_forum`, `escalate_to_release_council`,
  `escalate_to_shiproom`, `escalate_to_support`, `no_escalation_required`.
- `summary` — one bounded reviewable sentence naming the authority,
  the expiry, and the held release condition. Raw waiver justification
  text MUST NOT appear.
- `export_fields` — typed booleans for dashboard, weekly governance
  review, release packet, support export, and governance packet.

## 11. Expiry proximity vocabulary

Closed six-class vocabulary on the queue item:

| Class | Meaning |
| --- | --- |
| `expired_past_due` | `expires_at` in the past with no renewal recorded. The queue item MUST be gated on a `mitigation_status_class` other than `mitigation_complete_renewal_or_retire_pending`. |
| `due_today_or_within_24h` | `expires_at` within the next 24 hours or already today. |
| `nearing_expiry_within_seven_days` | `expires_at` between 24 hours and seven days from now. |
| `nearing_expiry_within_thirty_days` | `expires_at` between seven and thirty days from now. |
| `not_yet_due_review_window_open` | `expires_at` more than thirty days from now. |
| `no_expiry_pinned` | Standing waiver with no expiry timestamp; admissible only when `waiver_kind_class = single_maintainer_backup` or `waiver_kind_class = other_named_in_summary` and the summary names why no expiry is pinned. |

Schema-enforced pairings:

- `expired_past_due` requires an `expires_at` strictly in the past.
- `due_today_or_within_24h`,
  `nearing_expiry_within_seven_days`,
  `nearing_expiry_within_thirty_days`, and
  `not_yet_due_review_window_open` require a non-null `expires_at`.
- `no_expiry_pinned` requires `expires_at` to be null.

## 12. Mitigation-status vocabulary

Closed five-class vocabulary on the queue item:

| Class | Meaning |
| --- | --- |
| `mitigation_in_flight_owner_named` | Mitigation work is in flight; primary DRI is named and the work is moving. |
| `mitigation_blocked_pending_decision` | Mitigation is blocked pending a council decision; `escalation_path_class` MUST cite the relevant council. |
| `mitigation_not_started_pending_owner` | Mitigation has not started; ownership is unresolved or the named owner has not yet acked the queue item. The queue item is degraded and MUST NOT be filtered out by "owner inspecting" affordances. |
| `mitigation_complete_renewal_or_retire_pending` | Mitigation work is complete; the queue item is open only because the waiver renewal or retirement has not yet been minted. |
| `mitigation_not_required_audit_only` | The queue item is open only for audit visibility (e.g. standing single-maintainer-backup waiver). No mitigation action is owed. |

Schema-enforced pairings:

- `expiry_proximity_class = expired_past_due` forbids
  `mitigation_status_class = mitigation_not_required_audit_only`.
- `mitigation_status_class = mitigation_blocked_pending_decision`
  requires `escalation_path_class` in
  `{escalate_to_decision_forum, escalate_to_release_council,
  escalate_to_shiproom}`.
- `mitigation_status_class = mitigation_not_started_pending_owner`
  requires `escalation_path_class` in
  `{escalate_to_dri, escalate_to_backup_owner,
  escalate_to_decision_forum}`.

## 13. Open-detail action vocabulary

Closed five-class vocabulary on the queue item:

| Class | Meaning |
| --- | --- |
| `open_waiver_record_inspector` | Open the waiver record inspector pinned to `waiver_ref`. |
| `open_nightly_report_row_inspector` | Open the nightly-report-row inspector pinned to one of the linked nightly rows. |
| `open_protected_path_inspector` | Open the protected-path inspector pinned to one of `affected_protected_path_refs`. |
| `open_release_or_milestone_inspector` | Open the release / milestone inspector pinned to one of `affected_release_or_milestone_refs`. |
| `open_cluster_inspector` | Open the cluster inspector when `recurring_waiver_cluster_class` or `repeated_protected_path_regression_class` flags a pattern. |

Every open-detail action is inspect-only: it pins surfaces to the
record IDs and MUST NOT mint, renew, retire, or extend a waiver in
place. Mutating actions live on the upstream waiver workflow surface
and require the council authority and signoff matrix.

## 14. Recurring-waiver clustering and repeated-protected-path
regression

The cluster indicators are the contract's primary mechanism for
surfacing chronic drift to the weekly governance review.

Detection rules (frozen here so consuming surfaces compute the same
cluster classes):

- A `waiver_expiry_item` is in `recurring_within_quarter` when the
  same `affected_protected_path_refs[*]` carries at least two
  waivers with the same `waiver_authority_class` minted inside the
  current quarter.
- The cluster is in `recurring_across_quarters` when at least three
  such waivers exist across distinct quarters in the
  twelve-month window.
- The cluster is in `recurring_across_protected_paths` when the same
  `waiver_authority_class` has minted waivers on at least two
  distinct entries in `affected_protected_path_refs` inside the
  current quarter.
- A nightly row is in `repeated_protected_path_within_milestone` when
  the same `linked_rule_or_claim_refs[*]` row records at least two
  `blocked_by_failure`, `blocked_by_drift`, `waived_failure`, or
  `waiver_expired_failure` outcomes inside the current milestone.
- The row is in `repeated_protected_path_across_milestones` when the
  pattern persists across distinct milestones.
- The row is in `repeated_protected_path_across_release_lines` when
  the pattern persists across distinct release lines (stable, LTS,
  preview).

Both indicators are required fields. A row whose cluster cannot be
typed denies with `cluster_class_unresolved` rather than defaulting to
"isolated."

## 15. Authoring rules

When a recurring run lands a fresh evidence packet:

1. Mint a `nightly_report_row_record` with the typed `report_type_class`
   and the run chronology copied from the packet header.
2. Resolve `corpus_profile_identity` against the corpus / profile
   manifest used by the run.
3. Resolve `compare_snapshot` against the prior baseline packet on the
   same corpus / profile envelope; when no comparable baseline exists,
   resolve to `compare_against_threshold_only_no_baseline` plus
   `compare_skipped`.
4. Wire `evidence_freshness` from the packet header.
5. Resolve `linked_rule_or_claim_refs` against the protected fitness-
   function catalog, the claim manifest, the compatibility-surface
   inventory, the certified-archetype list, or the policy / signoff
   matrix. A row that names no upstream rule is non-conforming.
6. Resolve `mitigation_note_class` against the live `run_state_class`;
   the class is bounded by the schema's `allOf` block.
7. Resolve `cluster_indicators` against the rolling observation window;
   the schema requires both axes to be typed even when both are
   `not_recurring_first_observation` /
   `no_regression_first_observation`.

When a waiver is minted, renewed, retired, or expires:

1. Mint a `waiver_expiry_item_record` and link it from every nightly
   row whose `run_state_class` is gated by the waiver.
2. Resolve `expiry_proximity_class` against the live evaluation time;
   the class is recomputed every time the queue is rendered.
3. Resolve `mitigation_status_class` from the upstream mitigation
   work item; surfaces MUST NOT collapse "blocked pending decision"
   and "not started pending owner" into a single "needs review" chip.
4. Resolve `recurring_waiver_cluster_class` and
   `repeated_protected_path_regression_class` against the rolling
   observation window.
5. Resolve `escalation_path_class` against the council, the lane DRI,
   and the decision-rights matrix.
6. Surfaces render `summary` verbatim; they MUST NOT substitute
   free-text fear copy.

## 16. Out of scope

This contract does not implement:

- a nightly-job runner, an issue-routing automation, or a waiver-
  renewal workflow;
- the council decision rights themselves (those live in
  [`/docs/governance/decision_rights_and_signoff_matrix.md`](./decision_rights_and_signoff_matrix.md)
  and the signoff matrix);
- the protected fitness-function catalog or the protected-metrics
  register (those live in
  [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)
  and
  [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml));
- the underlying waiver records (those live in
  [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  and on
  [`/schemas/governance/waiver_expiry.schema.json`](../../schemas/governance/waiver_expiry.schema.json)).

This contract is the projection vocabulary that those upstream
artifacts flow through when they reach the dashboard, the milestone
scorecard, the weekly governance review, the release-evidence
shiproom packet, the support bundle, the public claim manifest, and
the governance packet.
