# Project Doctor probe-family matrix and non-destructive diagnosis rules

This document publishes the probe-family matrix used by Project Doctor
so reviewers can understand, at a glance, what each probe family may
inspect, what it must never do during diagnosis, which evidence classes
it is expected to produce, and the default redaction posture.

Diagnosis is **read-only by default**. Any mutation, trust widening, or
network-policy bypass is outside diagnosis and must route through a
governed repair or export flow.

Companion artifacts:

- [`/artifacts/support/probe_families.yaml`](../../artifacts/support/probe_families.yaml)
  — machine-readable probe-family matrix (row values).
- [`/schemas/support/probe_catalog_entry.schema.json`](../../schemas/support/probe_catalog_entry.schema.json)
  — canonical `probe_family_class` vocabulary plus evidence and side-
  effect vocabularies used by probes.
- [`/schemas/support/doctor_finding.schema.json`](../../schemas/support/doctor_finding.schema.json)
  — finding fields that carry `probe_family_class`, evidence refs,
  confidence, safety/no-touch attestations, and redaction posture.
- [`/docs/support/project_doctor_probe_contract.md`](./project_doctor_probe_contract.md)
  — per-probe catalog and probe-descriptor rules (admission, consent,
  parity, and no-hidden-side-effects).
- [`/fixtures/support/doctor_probe_cases/`](../../fixtures/support/doctor_probe_cases/)
  — seeded probe-descriptor + finding-card case pairs demonstrating
  safe diagnosis and safe next actions for each family.

Normative source anchors:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix DK.
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.6.3.1.
- `docs/architecture/supportability_adr.md` (shared vocabulary and
  non-destructive supportability boundary).

If this document disagrees with the machine-readable matrix or the
schemas, the schemas control shape and the matrix controls row values.

## Reviewer contract

For each `probe_family_class`, a reviewer should be able to answer:

| Question | Where to look |
|---|---|
| What inputs may this family inspect? | `primary_inputs` |
| What evidence classes should it emit? | `expected_evidence_source_classes` and `expected_signal_classes` |
| What is the default redaction posture? | `redaction_posture` |
| What actions are forbidden during diagnosis? | `forbidden_during_diagnosis` |
| What findings does it commonly produce? | `typical_findings` |
| Where is a concrete, reviewable example? | `seed_cases[]` |

## Non-destructive diagnosis rules (global)

The following rules apply to *all* probe families during diagnosis:

- Diagnosis MUST NOT execute repo-owned hooks, activators, tasks, build
  steps, scripts, post-checkout hooks, or extension activation paths
  merely to gather evidence.
- Diagnosis MUST NOT widen trust or identity scope (workspace trust,
  approvals, entitlements, managed policy epochs, credential grants) as
  part of evidence collection.
- Diagnosis MUST NOT bypass configured network controls. Network
  reachability checks (when allowed) MUST respect proxy configuration,
  endpoint-class rules, and CA posture and MUST NOT “try direct” as a
  fallback.
- Diagnosis MUST NOT activate untrusted third-party extensions or
  runtime hosts outside an explicitly declared safe inspection profile.
- Diagnosis MUST NOT delete or rewrite durable state (settings,
  histories, trust stores, credential stores, profiles, route/session
  objects). Even disposable cache clears are repair actions, not
  diagnosis actions.
- The only permitted writes during diagnosis are local evidence rows and
  local preview manifests that describe a repair but do not apply it.
- If a probe cannot establish truth, it MUST emit an explicit
  low-confidence or `unsupported`/insufficient-evidence outcome rather
  than silently omitting findings.

## Probe-family matrix (published)

The machine-readable matrix lives in
[`/artifacts/support/probe_families.yaml`](../../artifacts/support/probe_families.yaml).
This section is a human-readable rendering of the same rows.

### `execution_context_toolchains`

- Primary inputs: workspace manifests, capsule/prebuild metadata,
  resolved toolchain records, target fingerprints.
- Expected evidence classes: `existing_manifest`, `policy_decision`,
  `health_event`, `metric_counter`, `structured_log`.
- Default redaction posture: `max_data_class=environment_adjacent`,
  `default_redaction_class=metadata_safe_default`.
- Forbidden during diagnosis: `execute_repo_owned_code`,
  `mutate_cache_or_index`, `mutate_target_or_route`, `mutate_user_files`,
  `mutate_trust_policy_or_credentials`, `external_service_mutation`,
  `activate_third_party_extension`, `collect_high_risk_payload`.
- Typical findings: wrong interpreter/toolchain selected, blocked
  activator, stale capsule/prebuild, target mismatch, required component
  missing.
- Seed case: [`fixtures/support/doctor_probe_cases/finding_card_missing_toolchain.yaml`](../../fixtures/support/doctor_probe_cases/finding_card_missing_toolchain.yaml).

### `trust_identity_policy`

- Primary inputs: trust state, signed policy bundles, entitlement/session
  metadata, approval rules.
- Expected evidence classes: `existing_manifest`, `policy_decision`,
  `health_event`.
- Default redaction posture: `max_data_class=metadata_only`,
  `default_redaction_class=metadata_safe_default`.
- Forbidden during diagnosis: `execute_repo_owned_code`,
  `mutate_trust_policy_or_credentials`, `external_service_mutation`,
  `collect_high_risk_payload`.
- Typical findings: restricted-mode block, stale policy bundle, missing
  approval path, expired entitlement.
- Seed case: [`fixtures/support/doctor_probe_cases/finding_card_trust_policy_block.yaml`](../../fixtures/support/doctor_probe_cases/finding_card_trust_policy_block.yaml).

### `filesystem_watchers`

- Primary inputs: root capability map, alias/canonical identity graph,
  watcher health stats, low-disk signals.
- Expected evidence classes: `existing_manifest`, `health_event`,
  `metric_counter`, `structured_log`.
- Default redaction posture: `max_data_class=environment_adjacent`,
  `default_redaction_class=metadata_safe_default`.
- Forbidden during diagnosis: `execute_repo_owned_code`,
  `mutate_cache_or_index`, `mutate_user_files`, `external_service_mutation`,
  `collect_high_risk_payload`.
- Typical findings: watcher exhaustion/backlog, wrong-root hazard,
  permission failure, low-disk pressure.
- Seed case: [`fixtures/support/doctor_probe_cases/finding_card_filesystem_watcher_exhaustion.yaml`](../../fixtures/support/doctor_probe_cases/finding_card_filesystem_watcher_exhaustion.yaml).

### `network_proxy_ca_transport`

- Primary inputs: transport inventory, proxy config, CA summaries,
  endpoint-class metadata.
- Expected evidence classes: `existing_manifest`, `policy_decision`,
  `route_decision`, `probe_observation`, `structured_log`.
- Default redaction posture: `max_data_class=environment_adjacent`,
  `default_redaction_class=metadata_safe_default`.
- Forbidden during diagnosis: `execute_repo_owned_code`,
  `mutate_trust_policy_or_credentials`, `external_service_mutation`,
  `collect_high_risk_payload`.
- Typical findings: proxy mismatch, CA chain failure, blocked egress
  class, offline route.
- Seed case: [`fixtures/support/doctor_probe_cases/finding_card_proxy_or_ca_failure.yaml`](../../fixtures/support/doctor_probe_cases/finding_card_proxy_or_ca_failure.yaml).

### `extension_runtime_health`

- Primary inputs: extension inventories, host health events, quarantine
  records, budget history.
- Expected evidence classes: `existing_manifest`, `health_event`,
  `metric_counter`, `structured_log`, `crash_envelope`.
- Default redaction posture: `max_data_class=environment_adjacent`,
  `default_redaction_class=metadata_safe_default`.
- Forbidden during diagnosis: `execute_repo_owned_code`,
  `activate_third_party_extension`, `mutate_cache_or_index`,
  `mutate_user_files`, `mutate_trust_policy_or_credentials`,
  `external_service_mutation`, `collect_high_risk_payload`.
- Typical findings: crash loop, budget violation, compatibility mismatch,
  quarantine state.
- Seed case: [`fixtures/support/doctor_probe_cases/finding_card_extension_regression.yaml`](../../fixtures/support/doctor_probe_cases/finding_card_extension_regression.yaml).

### `caches_schema_local_state`

- Primary inputs: storage-class inventory, schema versions, integrity
  markers, migration state.
- Expected evidence classes: `existing_manifest`, `health_event`,
  `metric_counter`, `structured_log`.
- Default redaction posture: `max_data_class=environment_adjacent`,
  `default_redaction_class=metadata_safe_default`.
- Forbidden during diagnosis: `execute_repo_owned_code`,
  `mutate_cache_or_index`, `mutate_user_files`,
  `mutate_trust_policy_or_credentials`, `external_service_mutation`,
  `collect_high_risk_payload`.
- Typical findings: schema drift, failed migration, disposable cache
  incompatibility, suspected corruption.
- Seed case: [`fixtures/support/doctor_probe_cases/finding_card_schema_drift_repair_preview.yaml`](../../fixtures/support/doctor_probe_cases/finding_card_schema_drift_repair_preview.yaml).

### `remote_routes_collaboration`

- Primary inputs: agent envelopes, route/session objects, drift records,
  relay health.
- Expected evidence classes: `existing_manifest`, `route_decision`,
  `policy_decision`, `health_event`, `metric_counter`, `structured_log`.
- Default redaction posture: `max_data_class=environment_adjacent`,
  `default_redaction_class=metadata_safe_default`.
- Forbidden during diagnosis: `execute_repo_owned_code`,
  `mutate_target_or_route`, `mutate_trust_policy_or_credentials`,
  `external_service_mutation`, `collect_high_risk_payload`.
- Typical findings: agent skew/drift, route expiry, session-policy
  mismatch, helper attach required.
- Seed case: [`fixtures/support/doctor_probe_cases/finding_card_remote_target_mismatch.yaml`](../../fixtures/support/doctor_probe_cases/finding_card_remote_target_mismatch.yaml).
