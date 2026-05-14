# Project Doctor support-scenario matrix, diagnosis-latency scoreboard, repair-honesty contract, and escalation-packet completeness seed

This packet freezes one shared support contract for Project Doctor: the
scenario families Doctor MUST be judged against, the finding record a
probe returns, the repair-class vocabulary Doctor may suggest, the
recovery rung each finding maps onto, the scoreboard metrics that judge
whether diagnosis is timely and whether suggested repairs over-claim
safety, and the escalation-packet completeness check an exported issue
must pass. It exists so supportability is a measured platform contract
in the foundations phase instead of a later troubleshooting promise.

If this packet, the
[`scenario_matrix.yaml`](../../fixtures/support/scenario_matrix.yaml)
corpus, the
[`diagnosis_latency_scoreboard.yaml`](../../artifacts/support/diagnosis_latency_scoreboard.yaml)
scoreboard, the
[`escalation_packet_completeness_cases/`](../../fixtures/support/escalation_packet_completeness_cases/)
case set, and the frozen support vocabularies disagree, the frozen
support-bundle contract, object-handoff contract, and record-class
registry win for tooling and this packet must update in the same
change.

Companion artifacts:

- [`/docs/support/project_doctor_contract_alpha.md`](./project_doctor_contract_alpha.md),
  [`/schemas/project_doctor/probe.schema.json`](../../schemas/project_doctor/probe.schema.json),
  [`/schemas/project_doctor/finding.schema.json`](../../schemas/project_doctor/finding.schema.json),
  and
  [`/artifacts/support/project_doctor_probe_pack_alpha.yaml`](../../artifacts/support/project_doctor_probe_pack_alpha.yaml)
  - alpha Project Doctor probe/finding contract and read-only probe-pack
  baseline consumed by the support crate.
- [`/docs/support/probe_family_matrix.md`](./probe_family_matrix.md) and
  [`/artifacts/support/probe_families.yaml`](../../artifacts/support/probe_families.yaml)
  — published probe-family matrix and non-destructive diagnosis rules
  that bound what Doctor may inspect and what it must never do during
  diagnosis.
- [`/docs/support/project_doctor_probe_contract.md`](./project_doctor_probe_contract.md),
  [`/schemas/support/probe_catalog_entry.schema.json`](../../schemas/support/probe_catalog_entry.schema.json),
  [`/schemas/support/doctor_probe.schema.json`](../../schemas/support/doctor_probe.schema.json),
  [`/schemas/support/doctor_finding_card.schema.json`](../../schemas/support/doctor_finding_card.schema.json),
  and
  [`/schemas/support/doctor_explanation.schema.json`](../../schemas/support/doctor_explanation.schema.json)
  — concrete probe-catalog, probe-descriptor, finding-card, and
  finding-explanation packet contracts that project this scenario
  matrix into runnable Doctor admission rules, the closed probe-class
  taxonomy, the read-only-by-default finding card, evidence refs,
  no-hidden-side-effect handling, governed next actions, the
  desktop-versus-headless parity contract, and repair handoff
  anchors.
- [`/fixtures/support/scenario_matrix.yaml`](../../fixtures/support/scenario_matrix.yaml)
  — machine-readable scenario matrix covering missing toolchain,
  blocked trust state, broken watcher, incompatible cache/profile,
  extension regression, wrong target/environment, failed helper
  attach, and degraded docs/mirror paths. Every row classifies one
  finding by probe family, suggested recovery rung, and repair class.
- [`/artifacts/support/diagnosis_latency_scoreboard.yaml`](../../artifacts/support/diagnosis_latency_scoreboard.yaml)
  — scoreboard contract binding each seeded scenario to thresholds for
  time-to-first-diagnosis, finding accuracy, false-safe-repair rate,
  and escalation-packet completeness. Rows remain the scoreboard
  registry; current numeric targets live in the SLO target catalog.
- [`/artifacts/support/diagnosis_slo_targets.yaml`](../../artifacts/support/diagnosis_slo_targets.yaml)
  — current numeric SLO target catalog for first actionable diagnosis,
  escalation completeness, release-candidate drill coverage, and waiver
  behavior.
- [`/fixtures/support/drill_scenarios/`](../../fixtures/support/drill_scenarios/)
  — drill packets covering extension regression, stale toolchain
  context, proxy or certificate failure, and renderer/trace escalation.
- [`/fixtures/support/project_doctor_cases/`](../../fixtures/support/project_doctor_cases/)
  — focused cases for probe-catalog admission, mutating-probe repair
  promotion, finding explainability, and repair handoff linkage.
- [`/fixtures/support/escalation_packet_completeness_cases/`](../../fixtures/support/escalation_packet_completeness_cases/)
  — one completeness case per scenario family showing which exact-
  build, route/boundary, redaction, and fixture-id fields an exported
  issue handoff must carry, and which fields may be typed-unknown.
- [`/docs/support/support_intake_and_escalation_contract.md`](./support_intake_and_escalation_contract.md),
  [`/schemas/support/scenario_picker.schema.json`](../../schemas/support/scenario_picker.schema.json),
  [`/schemas/support/escalation_packet.schema.json`](../../schemas/support/escalation_packet.schema.json),
  and
  [`/fixtures/support/scenario_cases/`](../../fixtures/support/scenario_cases/)
  — support-intake scenario picker, capability-card, escalation-
  packet, and field-readiness contract that reuses the Doctor finding,
  repair-class, recovery-rung, and no-touch boundary vocabularies
  this packet pins.
- [`/docs/support/support_center_concept.md`](./support_center_concept.md)
  — product-facing Support Center concept that reserves Project Doctor
  as the canonical diagnosis surface and binds it to the recovery
  ladder, repair preview, bundle preview, and object-specific issue
  handoff vocabularies this packet reuses.
- [`/docs/support/support_bundle_contract.md`](./support_bundle_contract.md)
  and
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  — support-bundle recovery-rung, fault-domain, and artifact-manifest
  vocabulary Doctor findings quote by stable ref.
- [`/docs/support/object_handoff_packet.md`](./object_handoff_packet.md)
  and
  [`/schemas/support/object_handoff_packet.schema.json`](../../schemas/support/object_handoff_packet.schema.json)
  — escalation-packet field set the completeness cases project over.
- [`/schemas/support/support_packet_index.schema.json`](../../schemas/support/support_packet_index.schema.json)
  — support-packet family registry that already names the
  `object_issue_handoff` family the escalation packet belongs to.
- [`/docs/governance/dogfood_issue_taxonomy.md`](../governance/dogfood_issue_taxonomy.md)
  — dogfood category, severity rubric, `finding_codes`, and
  `repair_candidate_ids` conventions this packet reuses rather than
  re-minting.
- [`/docs/runtime/target_discovery_and_install_review_taxonomy.md`](../runtime/target_discovery_and_install_review_taxonomy.md)
  — target-discovery confidence, host-boundary cue, and managed-
  workspace lifecycle vocabulary the target/helper probes cite.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md` §10.15 (diagnostics), §10.22 (support
  export), and §10.23 (recovery ladder).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §8.10 (fault
  domain and supervisor), §24.2.2 (recovery rungs), §24.4 (repair
  preview), and Appendix I (support packet posture).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §22.20 (Support Center)
  and §23.26 (Doctor surface).
- `.t2/docs/Aureline_Milestones_Document.md` §3.20 (supportability),
  §3.21 (evidence), and §7.4 (blocked-user recovery).

If this document disagrees with those sources, those sources win and
this packet plus the companion artifacts update in the same change.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: support_doctor_packet
packet_id: support.project_doctor.seed
evidence_id: evidence.support.project_doctor.packet
title: Project Doctor support-scenario matrix, diagnosis-latency scoreboard, repair-honesty contract, and escalation-packet completeness seed
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - REL-SUPPORT-001
    - REL-REPAIR-015
    - OPS-SUP-005
    - GOV-EVID-901
    - GOV-CORPUS-901
  claim_row_refs:
    - packet_row:project_doctor.scenario_matrix_contract
    - packet_row:project_doctor.finding_record_contract
    - packet_row:project_doctor.repair_class_honesty
    - packet_row:project_doctor.recovery_rung_projection
    - packet_row:project_doctor.diagnosis_latency_scoreboard
    - packet_row:project_doctor.false_safe_repair_rate
    - packet_row:project_doctor.escalation_packet_completeness
    - packet_row:project_doctor.no_touch_boundary_contract
  covered_lanes:
    - support_export
    - release_evidence
    - governance_packets
    - docs_public_truth
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-23T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: project_doctor_seed@1
  trigger_revision: project_doctor_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen support-bundle, object-handoff, record-
    class registry, recovery-rung, target-discovery, and host-boundary
    vocabularies already landed in the repository. No live probe
    runtime, Doctor implementation, or support-portal is wired to this
    packet yet. Claims are structural: every scenario row reuses
    existing frozen tokens rather than minting per-surface finding
    language.
artifact_links:
  supporting_evidence_ids:
    - evidence.support.project_doctor.scenario_matrix
    - evidence.support.project_doctor.diagnosis_latency_scoreboard
    - evidence.support.project_doctor.escalation_packet_completeness
    - evidence.support.support_bundle_contract
    - evidence.support.object_handoff_packet
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/support/scenario_matrix.yaml
    - fixtures/support/escalation_packet_completeness_cases/
    - fixtures/support/object_handoff_examples/
    - fixtures/support/support_bundle_examples/recovery_ladder_remote_connector_loss.json
  archetype_refs: []
  source_anchor_refs:
    - docs/support/support_bundle_contract.md
    - docs/support/support_center_concept.md
    - docs/support/object_handoff_packet.md
    - docs/governance/dogfood_issue_taxonomy.md
    - docs/runtime/target_discovery_and_install_review_taxonomy.md
    - schemas/support/support_bundle.schema.json
    - schemas/support/object_handoff_packet.schema.json
    - schemas/support/support_packet_index.schema.json
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This seed packet freezes:

- one reviewer-facing `doctor_finding_record` shape every Project
  Doctor probe (desktop, CLI, headless, inspector) emits, with probe
  family, stable finding code, severity, confidence, scope, evidence
  refs, suggested repair class, and a typed `remaining_unknowns` list
  when diagnosis is partial;
- one closed `probe_family_class` vocabulary naming the eight scenario
  families in this milestone and one closed `repair_class` vocabulary
  pinned to the narrowest-safe-first ordering required by the Support
  Center concept;
- one mapping from every seeded finding code to its canonical
  `probe_family_class`, `repair_class`, `recovery_rung_class`, and
  explicit `no_touch_boundary_set` so Doctor does not invent
  case-by-case semantics for scenarios already governed elsewhere;
- one scoreboard contract binding each scenario to rows for time-to-
  first-diagnosis, finding accuracy, false-safe-repair rate, and
  escalation-packet completeness, with threshold values reserved to
  the benchmark council per the existing promotion pattern;
- one completeness-check rule set for escalation packets tied to
  exact-build identity, route and host-boundary truth, redaction
  defaults, and relevant fixture ids; and
- one per-scenario worked escalation case showing the minimum fields
  an export must carry (and which fields may be typed-unknown) before
  the packet leaves the machine.

It does not claim a live Project Doctor, a live probe scheduler, or a
hosted support portal is wired up. It claims only that the scenario
matrix, the scoreboard contract, the repair-honesty rules, and the
escalation-packet completeness cases now exist in one reviewable form
and reuse the frozen support vocabulary already landed in this
repository.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:project_doctor.scenario_matrix_contract` | `REL-SUPPORT-001`, `GOV-CORPUS-901` | `seed_only` | `internal` | `evidence.support.project_doctor.scenario_matrix` | Scenario matrix covers the eight required families; every row classifies finding by probe family, suggested recovery rung, and repair class. |
| `packet_row:project_doctor.finding_record_contract` | `REL-SUPPORT-001`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.support.project_doctor.scenario_matrix` | One finding-record shape every probe emits across desktop, CLI, headless, and inspector flows. |
| `packet_row:project_doctor.repair_class_honesty` | `REL-REPAIR-015`, `REL-SUPPORT-001` | `seed_only` | `internal` | `evidence.support.project_doctor.scenario_matrix`, `evidence.support.support_bundle_contract` | Repair-class vocabulary is closed, ordered narrowest-safe-first, and pinned to preview/rollback expectations. |
| `packet_row:project_doctor.recovery_rung_projection` | `REL-SUPPORT-001`, `REL-REPAIR-015` | `seed_only` | `internal` | `evidence.support.support_bundle_contract` | Every seeded finding names one `recovery_rung_class` from the frozen support-bundle vocabulary. |
| `packet_row:project_doctor.diagnosis_latency_scoreboard` | `OPS-SUP-005`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.support.project_doctor.diagnosis_latency_scoreboard` | Scoreboard rows freeze time-to-first-diagnosis and accuracy metric shape; thresholds reserved to the benchmark council. |
| `packet_row:project_doctor.false_safe_repair_rate` | `REL-REPAIR-015`, `OPS-SUP-005` | `seed_only` | `internal` | `evidence.support.project_doctor.diagnosis_latency_scoreboard` | False-safe-repair metric distinguishes over-claimed safety (`safety_overclaim`) from under-claim (`repair_undersold`). |
| `packet_row:project_doctor.escalation_packet_completeness` | `REL-SUPPORT-001`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.support.project_doctor.escalation_packet_completeness` | Completeness rules tie the exported packet to exact-build identity, route/boundary truth, redaction defaults, and fixture ids. |
| `packet_row:project_doctor.no_touch_boundary_contract` | `REL-SUPPORT-001`, `REL-REPAIR-015` | `seed_only` | `internal` | `evidence.support.project_doctor.scenario_matrix` | Every scenario row names its explicit no-touch boundary so diagnosis never mutates the surface under review. |

## Scenario matrix

Every row in the
[`scenario_matrix.yaml`](../../fixtures/support/scenario_matrix.yaml)
corpus describes one scenario family and at least one seeded finding
row. Each finding row freezes:

- `finding_code` — stable code for the finding (for example
  `doctor.finding.toolchain_missing_required_component`). Finding
  codes are additive-only; repurposing is breaking.
- `probe_family_class` — one of the closed tokens below.
- `severity_class` — `daily_blocker`, `major`, `scoped`, or
  `clarity_gap` from the dogfood severity rubric.
- `confidence_class` — `observed_authoritative`, `observed_with_gap`,
  `inferred_from_evidence`, or `unknown_requires_probe`.
- `scope_class` — `workspace`, `user`, `device`, `target`,
  `extension_host`, `helper_attach`, or `docs_pack`.
- `evidence_ref_fields` — stable field ids from the dogfood taxonomy
  the finding quotes (`exact_build_identity_ref`,
  `support_bundle_ref`, `route_context_*`, `checkpoint_ref`, and so
  on).
- `suggested_repair_class` — one of the closed repair-class tokens
  below; `observe_only_no_repair` is a first-class value, not a
  fallback for "unknown".
- `preview_required` — boolean. If true, the repair MUST run the
  repair-preview transaction grammar (review → preview → checkpoint →
  apply → verify → rollback/compensate) before touching state.
- `rollback_expectation_class` — `exact_rollback_available`,
  `compensating_action_available`, `no_exact_rollback_prefer_export`,
  or `not_applicable_read_only_finding`.
- `recovery_rung_class` — the rung Doctor maps the finding onto from
  the frozen support-bundle `recovery_rung_class` vocabulary.
- `no_touch_boundary_set` — explicit list of state classes Doctor MUST
  NOT mutate during diagnosis (for example the trust store, remote
  helpers that require reapproval, or policy-authored settings).
- `remaining_unknowns` — typed list drawn from the unknown-class
  vocabulary below. Populated on partial diagnoses; empty on
  authoritative findings.

Rule: a Doctor surface that cannot fill `finding_code`,
`probe_family_class`, `severity_class`, `confidence_class`,
`suggested_repair_class`, or `recovery_rung_class` MUST emit the
reserved finding code
`doctor.finding.probe_evidence_insufficient_for_diagnosis` at
`confidence_class = unknown_requires_probe` with populated
`remaining_unknowns` rather than invent a generic "something is
wrong" row.

### `probe_family_class` (frozen)

| Token | Job |
|---|---|
| `toolchain_probe` | Verify required interpreters, compilers, language services, and package managers against the declared target. |
| `trust_state_probe` | Verify workspace-trust posture and approval-ticket freshness against the declared surface. |
| `filesystem_watcher_probe` | Verify watcher liveness, backlog, and reseed state against the active workspace. |
| `cache_profile_probe` | Verify cache and profile compatibility, including disposable-derived caches, authoritative profile stores, and import provenance. |
| `extension_host_probe` | Verify extension and runtime-host health, quarantine posture, activation budget, and regression indicators. |
| `target_and_route_probe` | Verify target discovery, route class, host boundary, wrong-target correction, and managed-workspace lifecycle. |
| `helper_attach_probe` | Verify remote helper attach, adapter confidence, reapproval state, and suspend/resume posture. |
| `docs_mirror_probe` | Verify docs-pack freshness, version-match state, mirror catalog truth, and known-limit coverage. |

### `repair_class` (frozen, ordered narrowest-safe-first)

| Token | Job | Default preview/rollback expectation |
|---|---|---|
| `observe_only_no_repair` | Doctor reports the finding without proposing a write. | `preview_required: false`, `not_applicable_read_only_finding`. |
| `reacquire_trust_approval` | Re-request the workspace-trust or approval-ticket that expired or drifted. | `preview_required: true`, `compensating_action_available`. |
| `restart_watcher_with_reseed` | Restart the watcher and reseed its backlog without touching user files. | `preview_required: true`, `exact_rollback_available`. |
| `reset_ephemeral_cache` | Reset a disposable derived cache (`execution_context_cache`, extension-activation cache, resolver cache) named in the finding. | `preview_required: true`, `exact_rollback_available`. |
| `install_or_repair_toolchain` | Install, pin, or repair the missing toolchain component; never rewrite user source. | `preview_required: true`, `compensating_action_available`. |
| `quarantine_and_bisect_extension` | Quarantine a suspect extension and offer bisect; no auto-reenable. | `preview_required: true`, `compensating_action_available`. |
| `reapprove_target_or_route` | Re-request target approval or wrong-target correction; never auto-retarget. | `preview_required: true`, `compensating_action_available`. |
| `reattach_helper_with_new_approval` | Reattach a remote helper after explicit reapproval; never silently rebind a session. | `preview_required: true`, `compensating_action_available`. |
| `refresh_docs_or_mirror_pack` | Refresh docs pack, mirror snapshot, or known-limit index; never rewrite authored user content. | `preview_required: true`, `exact_rollback_available`. |
| `defer_to_escalation_packet` | No safe repair exists locally; Doctor prepares an escalation packet instead of applying a repair. | `preview_required: false`, `no_exact_rollback_prefer_export`. |

Rule: a finding whose `rollback_expectation_class =
no_exact_rollback_prefer_export` MUST set `suggested_repair_class =
defer_to_escalation_packet` unless a stricter repair class is
available that runs entirely in `observe_only_no_repair` mode.

### `remaining_unknowns` (closed vocabulary)

- `toolchain_version_unreadable`
- `trust_store_not_inspectable_without_unlock`
- `watcher_backlog_depth_not_observable`
- `cache_schema_version_drift_indeterminate`
- `extension_crash_lineage_incomplete`
- `target_identity_unresolved_requires_user`
- `helper_adapter_confidence_below_floor`
- `docs_pack_revision_not_matched_to_build`
- `mirror_snapshot_revision_unknown`
- `remote_route_reachability_intermittent`

Rule: a finding may populate `remaining_unknowns` only with tokens
from this list. Free-form unknown text is non-conforming.

## Finding-to-rung, no-touch, and repair mapping

Every seeded finding in
[`scenario_matrix.yaml`](../../fixtures/support/scenario_matrix.yaml)
is explained as:

> **probe family → finding code → repair class → recovery rung**,
> with an explicit `no_touch_boundary_set`.

The frozen `recovery_rung_class` tokens Doctor may project (from
`schemas/support/support_bundle.schema.json`):

- `safe_mode`
- `extension_bisect`
- `extension_quarantine`
- `open_without_restore`
- `cache_reset_candidate`
- `restricted_reopen`
- `rollback_reinstall_candidate`
- `typed_repair_flow`

Rule: Doctor MUST NOT invent a new rung token. A finding whose best
rung is not covered by this list is non-conforming and must be routed
through `defer_to_escalation_packet`.

### No-touch boundary vocabulary (closed)

Every finding row names at least one boundary token that Doctor MAY
NOT mutate during diagnosis or repair preview:

- `workspace_trust_store` — never silently widen or reset trust.
- `user_authored_files` — never rewrite authored buffers under any
  repair class.
- `managed_policy_overrides` — never alter admin-authored policy
  rows.
- `remote_helper_authorization` — never reattach without explicit
  reapproval.
- `managed_workspace_control_plane` — never mutate orchestrated
  lifecycle state.
- `installed_extension_marketplace_state` — never silently
  reinstall, reenable, or downgrade an extension.
- `docs_pack_authoring` — never edit docs content; refresh only
  governed mirror snapshots.
- `credential_store` — never read, copy, or rotate secrets.

Rule: `workspace_trust_store`, `managed_policy_overrides`,
`remote_helper_authorization`, `managed_workspace_control_plane`, and
`credential_store` appear in the no-touch list of every finding row
that targets a managed or remote boundary, even when the repair class
is narrower.

## Scoreboard contract

The scoreboard in
[`diagnosis_latency_scoreboard.yaml`](../../artifacts/support/diagnosis_latency_scoreboard.yaml)
has four families:

- `diagnosis_latency` — time-to-first-diagnosis per scenario.
- `finding_accuracy` — confusion-matrix shape per probe family, with
  false-positive and false-negative rates.
- `false_safe_repair_rate` — rate at which Doctor suggested a repair
  class whose advertised `rollback_expectation_class` was not in fact
  available on the observed system (`safety_overclaim`), separately
  from the under-claim rate (`repair_undersold`).
- `escalation_packet_completeness` — pass rate of the completeness
  check defined below, per scenario.

Every row cites exactly one `scenario_row_id` from the scenario
matrix, one `measurement_surface_class`
(`doctor_probe_run_local`, `doctor_probe_run_headless`,
`doctor_probe_run_inspector`, `doctor_probe_run_managed`), one
`primary_dri_lane`, and a `threshold_state` drawn from:

- `to_be_set_by_benchmark_council`
- `must_complete_under_diagnosis_latency_budget`
- `must_not_exceed_false_positive_budget`
- `must_not_claim_exact_rollback_without_evidence`
- `must_export_complete_escalation_packet`

The scoreboard remains the per-scenario row registry. Current release
targets for first actionable diagnosis, drill coverage, escalation
completeness, and waiver behavior live in
[`diagnosis_slo_targets.yaml`](../../artifacts/support/diagnosis_slo_targets.yaml).
Rows not bound by that target catalog remain shape-only until the
benchmark council ratifies their threshold state. Widening a threshold
opens a new decision row in `artifacts/governance/decision_index.yaml`.

Rule: a scoreboard row may move off
`to_be_set_by_benchmark_council` only through the benchmark council or
an explicit target-catalog binding. A false-safe-repair row may move only to
`must_not_claim_exact_rollback_without_evidence` or stricter; it may
not move sideways to a latency or completeness threshold.

## Escalation-packet completeness

An escalation packet exported by Doctor MUST be an
`object_handoff_packet_record` (schema:
`schemas/support/object_handoff_packet.schema.json`) whose fields
pass the completeness check defined here and worked case-by-case in
[`escalation_packet_completeness_cases/`](../../fixtures/support/escalation_packet_completeness_cases/).

### Required fields

- `build_context.exact_build_identity_ref` — non-null or carries a
  typed `exact_build_identity_unknown` reason from the handoff
  schema.
- `build_context.docs_pack_ref` and `docs_version_match_state` —
  populated when `probe_family_class = docs_mirror_probe`; may be
  null with `docs_pack_ref_unknown` otherwise.
- `route_context.command_id`, `invocation_session_id`,
  `action_origin_class`, `action_target_class`, `action_route_class`,
  `action_exposure_class` — populated when the finding touched a
  command or route; each field may resolve to the reserved unknown
  token from the origin/target/route taxonomy.
- `scope_and_boundary_context.host_boundary_class` — populated for
  every finding whose `probe_family_class` is
  `target_and_route_probe`, `helper_attach_probe`,
  `trust_state_probe`, or `docs_mirror_probe`.
- `scope_and_boundary_context.target_identity_ref` — populated when
  the finding touches a remote or managed target.
- `evidence_and_recovery_context.support_bundle_refs` — non-empty
  whenever the finding suggests a non-`observe_only_no_repair` class.
- `evidence_and_recovery_context.recovery_rung_class` — mirrors the
  finding's `recovery_rung_class`.
- `evidence_and_recovery_context.repair_transaction_refs` and
  `checkpoint_refs` — populated when the packet is exported after a
  preview or apply; empty only when `suggested_repair_class =
  observe_only_no_repair` or `defer_to_escalation_packet`.
- `evidence_and_recovery_context.redaction_choice_class` — one of
  the frozen handoff redaction-choice tokens; the default for a
  scenario whose probe family is `trust_state_probe`,
  `helper_attach_probe`, or `target_and_route_probe` is
  `support_bundle_by_reference` (no raw credentials, no raw route
  body).
- `provenance_bindings` — includes every applicable scenario fixture
  id (from `fixtures/support/scenario_matrix.yaml`) as a
  `scenario_row_ref` binding so support review can pivot from packet
  back to scenario row in O(1).
- `destination_context.support_route_ref` — non-null for
  non-local-only deliveries; null with
  `delivery_state_class = local_only_review` for local-only exports.

### Completeness outcomes

The completeness check returns one of:

- `complete` — all required fields populated; any typed-unknown
  placeholders are drawn from the reserved unknown vocabulary; all
  relevant fixture ids bind.
- `complete_with_typed_unknowns` — one or more required fields are
  resolved to a typed-unknown token; the packet still exports but the
  scoreboard records the typed-unknown list.
- `incomplete_refused_export` — at least one required field is
  missing without a typed-unknown placeholder; Doctor MUST NOT export
  and instead emits the finding code
  `doctor.finding.escalation_packet_incomplete_refused_export` and
  writes the typed gap list back into the packet draft.

Rule: an escalation packet whose
`redaction_choice_class` widens past the default for its probe family
without an explicit user consent marker is `incomplete_refused_export`
regardless of the rest of the field set.

## Seeded scenario families

The eight scenario families in the corpus and their default mappings:

| Scenario family | Probe family | Default suggested repair | Default recovery rung | Default redaction choice |
|---|---|---|---|---|
| `missing_toolchain` | `toolchain_probe` | `install_or_repair_toolchain` | `typed_repair_flow` | `metadata_only_embedded` |
| `blocked_trust_state` | `trust_state_probe` | `reacquire_trust_approval` | `restricted_reopen` | `support_bundle_by_reference` |
| `broken_watcher` | `filesystem_watcher_probe` | `restart_watcher_with_reseed` | `typed_repair_flow` | `metadata_only_embedded` |
| `incompatible_cache_profile` | `cache_profile_probe` | `reset_ephemeral_cache` | `cache_reset_candidate` | `metadata_only_embedded` |
| `extension_regression` | `extension_host_probe` | `quarantine_and_bisect_extension` | `extension_quarantine` | `metadata_only_embedded` |
| `wrong_target_environment` | `target_and_route_probe` | `reapprove_target_or_route` | `restricted_reopen` | `support_bundle_by_reference` |
| `failed_helper_attach` | `helper_attach_probe` | `reattach_helper_with_new_approval` | `restricted_reopen` | `support_bundle_by_reference` |
| `degraded_docs_mirror` | `docs_mirror_probe` | `refresh_docs_or_mirror_pack` | `typed_repair_flow` | `metadata_only_embedded` |

Every scenario family MUST carry at least one finding row whose
`suggested_repair_class = observe_only_no_repair` so the matrix
covers diagnosis-only outcomes alongside repair-candidate outcomes.

## What this seed does not promise

- No live Project Doctor implementation is wired up. The matrix, the
  scoreboard, and the escalation-packet cases are reviewable objects
  only.
- No live measurement result is claimed. Numeric targets are contract
  budgets in the SLO target catalog, not proof that a runtime already
  meets them.
- No production telemetry pipeline, live support portal, or hosted
  ticket system is in scope.
- No schema changes to `support_bundle.schema.json` or
  `object_handoff_packet.schema.json` are required; this packet
  projects onto existing vocabularies rather than minting new record
  kinds.
- No scenario row commits to a milestone date. Rows are additive.
  Repurposing a `finding_code`, `probe_family_class`, `repair_class`,
  or `recovery_rung_class` token is breaking and requires a new
  decision row in `artifacts/governance/decision_index.yaml`.
