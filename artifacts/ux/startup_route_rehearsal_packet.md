# Startup-route rehearsal packet template

<!--
Copy this template when assembling a startup-route rehearsal
packet. The packet is the **single shared format** every
protected startup route is scored under, so reviewers can read
"the route reached first-useful-work" the same way regardless of
which entry verb / target kind / profile is being exercised.

Companion artifacts:
- /artifacts/ux/first_useful_work_corpus/   — qualification corpus
                                              (one fuw_row:* per
                                              protected route ×
                                              case category)
- /artifacts/ux/no_account_switching_scoreboard.yaml
                                            — aggregates rehearsal
                                              outcomes by entry
                                              route × deployment
                                              profile
- /fixtures/ux/first_useful_work_cases/      — worked seed fixtures
- /artifacts/product/task_success_corpus_seed.yaml
                                              — upstream task-success
                                              scenarios
- /artifacts/product/no_account_switching_scoreboard_seed.yaml
                                              — upstream scoreboard
                                              seed
- /docs/product/onboarding_measurement_plan.md
                                              — entry-route taxonomy,
                                              readiness buckets,
                                              measurement surfaces
- /docs/ux/entry_restore_truth_audit.md       — startup-state token
                                              set
- /artifacts/ux/startup_state_copy_review.yaml
                                              — copy-review rows
                                              every audited state
                                              resolves through

This packet is intentionally structured around stable refs
(`fuw_row:*`, `scoreboard_row:*`, `er.*`, `startup_state:*`,
`tsc.*`, fixture paths). Do not substitute free-text route or
state names where a stable ref exists. Do not invent new
qualification classes outside the closed set
{`exact`, `compatible`, `partial`, `failed`}; rows that need a
new class open a decision row instead of widening this template.
-->

## 0. Packet header

Every rehearsal packet SHOULD embed this header verbatim. Fields
without a stable ref use the closed vocabularies named in the
qualification corpus README §1.

- **Packet id:** `<startup-route-rehearsal-packet-id>`
- **Packet state:** `draft` | `in_review` | `accepted` | `blocked` | `superseded`
- **Reviewer (DRI):** `@<handle>`
- **Co-reviewer:** `@<handle>` or `none`
- **Rehearsal date:** `YYYY-MM-DDTHH:MM:SSZ`
- **Build identity:** `<exact_build_identity_ref>` from
  `schemas/build/exact_build_identity.schema.json`
- **Deployment profile under test:** one of
  `individual_local` | `self_hosted` | `enterprise_online` |
  `air_gapped` | `managed_cloud` from
  `artifacts/governance/deployment_profiles.yaml`
- **Network posture:** `online` | `offline` | `mirror_only`
- **Sign-in posture:** `no_sign_in` | `sign_in_offered_skipped` |
  `sign_in_required_completed` | `sign_in_required_skipped`
- **Service opt-in posture:** `none_offered` |
  `offered_all_declined` | `offered_some_accepted` |
  `offered_all_accepted`
- **Corpus rows in scope:** list of `fuw_row:*` ids from
  `artifacts/ux/first_useful_work_corpus/`
- **Scoreboard rows in scope:** list of `scoreboard_row:*` ids
  from `artifacts/ux/no_account_switching_scoreboard.yaml`
- **Seed scoreboard rows referenced:** list of
  `scoreboard_row:*` ids from
  `artifacts/product/no_account_switching_scoreboard_seed.yaml`
- **Active waiver packet refs:** waiver packet ids or `none`
- **Notes:** one-paragraph reviewer comment; never a substitute
  for a typed field.

## 1. Executive summary

Two or three sentences naming:

1. how many rows were exercised in this packet,
2. how many scored `exact`, `compatible`, `partial`, or `failed`,
3. whether any row violated its `local_first_claim_class`.

This summary feeds
`artifacts/ux/no_account_switching_scoreboard.yaml`
without restating per-row thresholds.

## 2. Per-row rehearsal table

Reviewers fill **one row per qualification-corpus row** under
test. Every column resolves to a closed vocabulary. Free-text
goes only in the `notes` column at the end of each row.

| Field | Vocabulary / source |
|---|---|
| `corpus_row_ref` | `fuw_row:*` from `artifacts/ux/first_useful_work_corpus/` |
| `case_category` | corpus README §1.3 |
| `entry_route_id` | onboarding measurement plan §4 |
| `deployment_profile_id` | `artifacts/governance/deployment_profiles.yaml` |
| `startup_state_token` | `artifacts/ux/startup_state_copy_review.yaml` startup_state_token_vocabulary |
| `expected_blocker_class` | corpus README §1.1 |
| `observed_blocker_class` | corpus README §1.1 (use `no_blocker` if none) |
| `expected_qualification_class` | `{exact, compatible, partial, failed}` |
| `observed_qualification_class` | `{exact, compatible, partial, failed}` |
| `time_to_first_useful_work_class` | `{within_envelope, within_envelope_after_safe_exit, outside_envelope, not_reached}` |
| `local_first_claim_held` | `{yes, narrowed_advertised, account_required_continue_local_offered, no_violation_recorded}` |
| `safe_exit_action_invoked` | `next_step_decision_hook` value or `none_required` |
| `notes` | free-text rationale; must not mint state names |

A row is filled with the stable ref and the observed values. A
row whose `observed_qualification_class` differs from the corpus
row's declared `rehearsal_qualification_class` is non-conforming
unless the difference resolves to a typed `observed_blocker_class`
the row was not predicted to encounter — in which case the row
is filed as a packet-level finding (§5).

### 2.1 Empty row template

Copy and fill one block per corpus row.

```yaml
- corpus_row_ref: fuw_row:<case_category>.<slug>
  case_category: <case_category>
  entry_route_id: <er.*>
  deployment_profile_id: <profile>
  startup_state_token: <startup_state:*>
  expected_blocker_class: <blocker_class>
  observed_blocker_class: <blocker_class>
  expected_qualification_class: <exact|compatible|partial|failed>
  observed_qualification_class: <exact|compatible|partial|failed>
  time_to_first_useful_work_class: <within_envelope|within_envelope_after_safe_exit|outside_envelope|not_reached>
  local_first_claim_held: <yes|narrowed_advertised|account_required_continue_local_offered|no_violation_recorded>
  safe_exit_action_invoked: <next_step_decision_hook | none_required>
  notes: >
    <one-paragraph rationale; closed-vocab fields above are the
    actual record of truth>
```

## 3. Hidden-setup failures

Reviewers list every undisclosed setup work the route surfaced.
A clean rehearsal records `hidden_setup_failures: none`. Each
entry resolves to a closed vocabulary or names the boundary row
the setup affected.

| Field | Vocabulary / source |
|---|---|
| `corpus_row_ref` | `fuw_row:*` |
| `hidden_setup_class` | `{undisclosed_extension_activation, undisclosed_dependency_install, undisclosed_toolchain_mutation, undisclosed_devcontainer_attach, undisclosed_credential_persistence, undisclosed_remote_state_write, undisclosed_telemetry_emission, undisclosed_managed_policy_application, other}` |
| `boundary_manifest_row_id` | `docs/product/boundary_manifest_strawman.md` |
| `discovered_when_class` | `{at_admission, after_admission_pre_first_useful_work, post_first_useful_work, post_apply_only}` |
| `notes` | free-text rationale |

Empty entry template:

```yaml
- corpus_row_ref: fuw_row:<case_category>.<slug>
  hidden_setup_class: <class>
  boundary_manifest_row_id: <row_id>
  discovered_when_class: <when>
  notes: >
    <reviewer rationale>
```

## 4. Startup-route drift

Drift = the route the user took diverged from the corpus row's
declared `entry_route_id`, `entry_verb`, `target_kind`,
`resulting_mode`, or `first_useful_work_target_surface`. A clean
rehearsal records `startup_route_drift: none`.

| Field | Vocabulary / source |
|---|---|
| `corpus_row_ref` | `fuw_row:*` |
| `drift_axis` | `{entry_route_id, entry_verb, target_kind, resulting_mode, first_useful_work_target_surface, restore_class, account_prompt_timing, recovery_class}` |
| `declared_value` | value from the cited corpus row |
| `observed_value` | value the rehearsal actually committed |
| `drift_class` | `{narrowed_to_advertised, narrowed_unadvertised, widened_unadvertised, collapsed_into_generic_get_started, recovery_class_free_form}` |
| `routing_disclosure_held` | `{yes, no, partial}` (was the narrowing advertised before commit per the corpus row's `local_first_claim_class`?) |
| `notes` | free-text rationale |

Empty entry template:

```yaml
- corpus_row_ref: fuw_row:<case_category>.<slug>
  drift_axis: <axis>
  declared_value: <value>
  observed_value: <value>
  drift_class: <class>
  routing_disclosure_held: <yes|no|partial>
  notes: >
    <reviewer rationale>
```

## 5. Local-first claim qualification

Reviewers state whether Aureline preserved a credible
local-first path when sign-in or managed services were absent
or declined. The packet records:

| Field | Vocabulary / source |
|---|---|
| `account_present_during_rehearsal` | `{none, offered_skipped, offered_completed, required_completed, required_skipped}` |
| `managed_service_reachable` | `{none_required, all_reachable, some_unreachable, none_reachable}` |
| `network_posture_observed` | `{online, offline, mirror_only}` |
| `local_first_floor_held` | `yes` / `narrowed_advertised` / `no_with_typed_reason` |
| `narrowing_advertised_before_commit` | `yes` / `no` / `not_applicable` |
| `decline_path_offered_same_weight` | `yes` / `no` / `not_applicable` |
| `retroactive_degradation_observed` | `yes` / `no` |
| `summary` | one paragraph naming every (`fuw_row:*`, `local_first_claim_class`) pair the rehearsal exercised and how it resolved |

A rehearsal that records `retroactive_degradation_observed:
yes` MUST raise at least one packet-level finding (§7) regardless
of the per-row qualification scores.

## 6. Per-route qualification roll-up

For each `entry_route_id` × `deployment_profile_id` pair the
rehearsal exercised, fill one block. The block aggregates the §2
table into the format the no-account switching scoreboard
ingests by id.

```yaml
- entry_route_id: <er.*>
  deployment_profile_id: <profile>
  scoreboard_row_refs:
    - scoreboard_row:fuw_no_account_by_route.<...>
    - scoreboard_row:fuw_service_opt_in_decline_outcomes.<...>
    - scoreboard_row:fuw_managed_sign_in_skipped.<...>
    - scoreboard_row:fuw_account_required_exception.<...>
  observed_qualification_class: <exact|compatible|partial|failed>
  worst_observed_blocker_class: <blocker_class>
  declined_opt_ins:
    - <boundary_manifest_row_id>
  forced_sign_in_observed: <yes|no>
  network_required_for_local_entry_observed: <yes|no>
  notes: >
    <one-paragraph reviewer rationale; the typed fields above
    are the record of truth>
```

A roll-up block whose `observed_qualification_class` falls
outside the cited scoreboard rows'
`acceptable_qualification_classes` set MUST raise at least one
packet-level finding (§7).

## 7. Packet-level findings

Every finding resolves to one of three outcomes: `pass`,
`narrowed_pass`, or `fail`. A `narrowed_pass` is permitted only
when narrowing was advertised before commit per the cited
corpus row.

```yaml
- finding_id: <packet-id>.finding.<short-slug>
  finding_class: <forced_sign_in_before_useful_local_work
                  | network_required_for_local_entry
                  | resulting_mode_silently_downgraded
                  | restore_level_promised_higher_than_delivered
                  | rollback_checkpoint_missing
                  | retroactive_degradation_after_decline
                  | hidden_setup_post_apply
                  | startup_route_drift_unadvertised
                  | other>
  outcome: <pass|narrowed_pass|fail>
  related_corpus_row_refs:
    - fuw_row:<case_category>.<slug>
  related_scoreboard_row_refs:
    - scoreboard_row:<...>
  remediation_owner_lane: <ownership-matrix lane id>
  remediation_target: <next packet | next decision row | next milestone review>
  notes: >
    <reviewer rationale; closed-vocab fields above are the
    record of truth>
```

A packet that lists any `fail` finding cannot be marked
`accepted` and is routed to the lane named in
`remediation_owner_lane`.

## 8. How this packet feeds downstream artifacts

When the packet reaches `accepted` state, the assembling lane:

1. Updates each cited
   [`scoreboard_row:fuw_*`](./no_account_switching_scoreboard.yaml)
   row's `latest_observed_qualification_class` (when this field
   is present) and the supporting fixture refs.
2. Files the per-row table from §2 into the lane's UX evidence
   packet for the milestone review.
3. Cross-references any §5 narrowing that crossed the
   `local_first_claim_class` boundary into the
   [no-account local-entry contract](../../docs/ux/no_account_local_entry_contract.md)
   review queue.
4. Files §7 findings against the
   [decision index](../../artifacts/governance/decision_index.yaml)
   when the finding is `fail` and reopens the cited corpus row
   under
   [`/artifacts/ux/first_useful_work_corpus/`](./first_useful_work_corpus/)
   when the row's declared `rehearsal_qualification_class`
   needs to narrow.

## 9. Worked example (informational)

The following sketch shows a minimal packet covering one row
from each row family. It is illustrative, not exhaustive; a
real packet exercises every protected route the lane owns.

```yaml
packet_id: startup-route-rehearsal-2026-foundations-spike-0001
packet_state: draft
reviewer_dri: "@ahmeddyounis"
co_reviewer: none
rehearsal_date: "2026-05-01T00:00:00Z"
build_identity: build-identity-seed-fuw-rehearsal-example
deployment_profile_under_test: individual_local
network_posture: online
sign_in_posture: no_sign_in
service_opt_in_posture: offered_all_declined

executive_summary: >
  Three corpus rows exercised on individual_local. Two scored
  exact, one scored compatible (managed sign-in offered and
  skipped). No row violated its local_first_claim_class.

per_row_rehearsal_table:
  - corpus_row_ref: fuw_row:local_open.first_run_start_center_local_folder
    case_category: local_open
    entry_route_id: er.start_center
    deployment_profile_id: individual_local
    startup_state_token: startup_state:first_run
    expected_blocker_class: no_blocker
    observed_blocker_class: no_blocker
    expected_qualification_class: exact
    observed_qualification_class: exact
    time_to_first_useful_work_class: within_envelope
    local_first_claim_held: yes
    safe_exit_action_invoked: none_required
    notes: >
      Start Center first-run reached first-useful-edit on
      individual_local with no opt-in card, no managed sign-in
      prompt, and no network call.
  - corpus_row_ref: fuw_row:service_opt_in_declined.telemetry_optional_at_first_run
    case_category: service_opt_in_declined
    entry_route_id: er.start_center
    deployment_profile_id: individual_local
    startup_state_token: startup_state:first_run
    expected_blocker_class: no_blocker
    observed_blocker_class: no_blocker
    expected_qualification_class: compatible
    observed_qualification_class: compatible
    time_to_first_useful_work_class: within_envelope
    local_first_claim_held: yes
    safe_exit_action_invoked: set_up_later
    notes: >
      Telemetry opt-in offered same-weight with Continue local;
      decline preserved local-first floor.
  - corpus_row_ref: fuw_row:managed_sign_in_available_but_skipped.individual_local_first_run_managed_sign_in_offered_skipped
    case_category: managed_sign_in_available_but_skipped
    entry_route_id: er.start_center
    deployment_profile_id: individual_local
    startup_state_token: startup_state:first_run
    expected_blocker_class: no_blocker
    observed_blocker_class: no_blocker
    expected_qualification_class: compatible
    observed_qualification_class: compatible
    time_to_first_useful_work_class: within_envelope
    local_first_claim_held: yes
    safe_exit_action_invoked: set_up_later
    notes: >
      Managed-org sign-in card surfaced same-weight with
      Continue local; decline preserved local-first floor.

hidden_setup_failures: none

startup_route_drift: none

local_first_claim_qualification:
  account_present_during_rehearsal: offered_skipped
  managed_service_reachable: none_required
  network_posture_observed: online
  local_first_floor_held: yes
  narrowing_advertised_before_commit: not_applicable
  decline_path_offered_same_weight: yes
  retroactive_degradation_observed: no
  summary: >
    On individual_local, the no-account floor held across all
    three rows. Continue local was offered same-weight with
    every opt-in card; declines preserved first-useful-edit.

per_route_qualification_rollup:
  - entry_route_id: er.start_center
    deployment_profile_id: individual_local
    scoreboard_row_refs:
      - scoreboard_row:fuw_no_account_by_route.start_center_individual_local
      - scoreboard_row:fuw_service_opt_in_decline_outcomes.telemetry_individual_local
      - scoreboard_row:fuw_managed_sign_in_skipped.individual_local_local_flow_continued
    observed_qualification_class: compatible
    worst_observed_blocker_class: no_blocker
    declined_opt_ins:
      - telemetry_support_pipeline
      - identity_policy_service
    forced_sign_in_observed: no
    network_required_for_local_entry_observed: no
    notes: >
      Worst observed class is compatible (because
      managed-sign-in and telemetry rows narrowed by decline,
      not by drift). All declines preserved local flow.

findings: []

packet_state_at_close: accepted
```

## 10. Schema references

- Closed vocabularies for every typed field in this packet:
  [`/artifacts/ux/first_useful_work_corpus/README.md`](./first_useful_work_corpus/README.md).
- Scoreboard the packet feeds:
  [`/artifacts/ux/no_account_switching_scoreboard.yaml`](./no_account_switching_scoreboard.yaml).
- Audited startup-state copy-review:
  [`/artifacts/ux/startup_state_copy_review.yaml`](./startup_state_copy_review.yaml).
- Entry / restore object model:
  [`/docs/workspace/entry_restore_object_model.md`](../../docs/workspace/entry_restore_object_model.md).
- Onboarding measurement plan:
  [`/docs/product/onboarding_measurement_plan.md`](../../docs/product/onboarding_measurement_plan.md).
- No-account local-entry contract:
  [`/docs/ux/no_account_local_entry_contract.md`](../../docs/ux/no_account_local_entry_contract.md).
- Boundary manifest:
  [`/docs/product/boundary_manifest_strawman.md`](../../docs/product/boundary_manifest_strawman.md).
- Deployment-profile register:
  [`/artifacts/governance/deployment_profiles.yaml`](../governance/deployment_profiles.yaml).
- Protected metrics (envelope refs `time_to_first_useful_work_class` resolves against):
  [`/artifacts/bench/protected_metrics.yaml`](../bench/protected_metrics.yaml).
