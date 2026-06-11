# Project Doctor feature-lane probes, finding codes, and unsupported-state reporting

This document describes the canonical packet that extends Project Doctor to the
M5 feature lanes and keeps every lane's diagnosis read-only, stably coded, and
explicit about unsupported, partial, stale, policy-blocked, and target-mismatch
states. It is the user-facing companion to the governed artifact at
`artifacts/doctor/m5/project-doctor-feature-lane-probes.json` and the typed model
in the `aureline-doctor` crate
(`extend_project_doctor_probes_finding_codes_and_unsupported_state_reporting_across_feature_lanes`).

## What this packet covers

The packet pins **one read-only probe family per feature lane**. The claimed
lanes are:

1. **`notebook_kernel`** — notebook kernel runtime, engine identity, attach
   readiness.
2. **`request_api`** — request/API auth authority and environment binding.
3. **`database_target`** — database target identity and schema alignment.
4. **`profiler_replay`** — profiler/replay instrumentation attachment and capture
   coverage.
5. **`preview_route`** — remote preview route, port, and tunnel scope.
6. **`sync_device_registry`** — sync, offboarding, and device-registry
   consistency.
7. **`companion_handoff`** — companion handoff packet completeness and continuity.
8. **`incident_packet`** — incident packet integrity and chain-of-custody
   readiness.

Each family names:

- a stable **`finding_code_prefix`** of the form `doctor.finding.<lane>.`;
- the **`supported_finding_codes`** it may emit;
- the **`supported_states`** it may report;
- the **`supported_scope_kinds`** it diagnoses (the lane's engine/route/target/
  session/registry/packet identity);
- a **`read_only_posture`** — either `read_only_no_mutation` or
  `metadata_local_evidence_only`. There is no mutating posture, so diagnosis is
  read-only by construction.

## Findings carry stable, explainable, scoped truth

Each `finding` answers, for its lane:

- **Which stable code fired?** A `finding_code` in the lane family's supported
  set, beginning with the lane's prefix.
- **What state is the lane in?** A `diagnosis_state` of `healthy`, `partial`,
  `stale`, `unsupported`, `policy_blocked`, or `target_mismatch`.
- **How sure is Doctor?** A `confidence_class` of `observed_authoritative`,
  `observed_with_gap`, `inferred_from_evidence`, or `unknown_requires_probe`.
- **What is affected?** An `affected_scope` naming the `scope_kind` identity and
  an opaque `scope_ref`.
- **What backs it?** One or more `evidence_refs`.
- **What can the user do first?** A `first_action` — the first actionable
  explanation, not a generic error string.
- **Is there a reviewed repair?** Zero or more `repair_candidate_ids`, only in
  lanes whose family sets `emits_repair_candidates`.

## Unsupported states are reported explicitly

The central guarantee is that an unsupported, partial, stale, policy-blocked, or
target-mismatch lane is never collapsed into a generic "unavailable" string.

- Every **non-healthy** finding must carry a specific, non-generic
  `state_detail_code` (e.g. `kernel_runtime_not_installed`,
  `api_auth_policy_denies_environment`, `database_target_schema_drift`,
  `preview_route_evidence_past_freshness_slo`). Empty or generic tokens
  (`unavailable`, `error`, `failed`, `unknown`, …) are rejected by validation.
- A **healthy** finding must carry no detail code.
- The `diagnosis_state` and `severity_class` must agree on the unsupported axis:
  an `unsupported` state requires an `unsupported` severity, and no lesser state
  may claim it.

## Read-only and repair-safety guardrails

- Diagnosis is read-only by construction — a family carries only a read-only
  posture and may not run repo-owned hooks, mutate external services, or silently
  re-enable a blocked/quarantined component to gather evidence.
- A lane whose family sets `emits_repair_candidates: false` may not attach any
  `repair_candidate_ids`, so no speculative remediation leaks into a read-only
  lane. Where present, each repair-candidate id uses the `repair.` prefix and
  names an explicit, reviewed candidate rather than an ambient auto-heal.

## One vocabulary across desktop, CLI/headless, and support

Every finding must render stably across the desktop card, the headless JSON row,
and the support-export row (`render_surfaces` must include `ui_finding_card`,
`headless_json_row`, and `support_export_row`). Because the same finding id and
finding code carry the same diagnosis state and detail everywhere, support,
automation, and users reason about the same finding across desktop and headless
contexts.

## How downstream surfaces consume it

`export_projection()` produces a redaction-safe row set with the finding id and
code, lane, diagnosis state, severity, confidence, explicit detail code, scope
kind/ref, repair candidates, first action, and a cross-context-stable flag, plus
`repair_candidate_count` and `cross_context_stable_count`. Help/About, docs/help,
support exports, and release/public-truth packets should ingest this projection
directly rather than restating feature-lane diagnosis text by hand.

## Validation

`ProjectDoctorFeatureLaneProbes::validate()` reports every violation, including
unsupported schema version or record kind, non-canonical closed vocabularies,
empty required fields, a missing or duplicate lane family, a family prefix that
does not match its lane, a finding code outside its family's supported set, a
diagnosis state or scope kind a family does not support, a non-healthy finding
hiding behind a generic/empty detail code, a healthy finding carrying a detail
code, a severity/state disagreement on the unsupported axis, a repair candidate
in a non-emitting lane or without the `repair.` prefix, a finding that is not
stable across desktop/headless/support, and a summary block that disagrees with
the families and findings.
