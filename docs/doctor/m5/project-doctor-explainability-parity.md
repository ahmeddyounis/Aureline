# Project Doctor explainability panes, evidence refs, and cross-surface parity

This document describes the canonical packet that turns each Project Doctor
finding into an inspectable **explainability pane** and pins
**desktop/CLI/headless/support parity** across the M5 feature lanes. It is the
user-facing companion to the governed artifact at
`artifacts/doctor/m5/project-doctor-explainability-parity.json`, the boundary
schema at `schemas/doctor/project-doctor-explainability-parity.schema.json`, and
the typed model in the `aureline-doctor` crate
(`ship_project_doctor_explainability_panes_evidence_refs_and_cross_surface_parity`).

It builds on the feature-lane probe packet
(`artifacts/doctor/m5/project-doctor-feature-lane-probes.json`): that packet pins
*which* findings each lane may emit; this packet pins *how a finding explains
itself* and *how it renders identically everywhere*.

## What an explainability pane exposes

Each `pane` answers, for one finding:

- **Which stable code fired?** A `finding_code` beginning with
  `doctor.finding.`.
- **Which probe produced it, at what version?** A `probe_id` and a
  `probe_version`, so support and release packets reference probe versions and
  finding ids instead of screenshots or prose.
- **What backs it?** One or more `evidence_refs`.
- **What is affected?** An `affected_scope` naming an opaque `scope_kind` and
  `scope_ref`.
- **Why is a candidate repair available — or not?** A `repair_availability` of
  `available`, `not_applicable_healthy`, or one of the explicit
  `blocked_unsupported_context`, `blocked_managed_policy`,
  `blocked_partial_evidence`, or `blocked_reversal_unproven` classes.
- **How can the repair be undone?** A `reversal_class` of
  `reversible_transactional`, `reversible_with_snapshot`, `irreversible_guarded`,
  or `not_applicable`.
- **What does the CLI return?** A canonical `cli_exit_class` (and stable exit
  code) derived from the diagnosis state.
- **What may localization touch?** The `machine_meaning_keys` name the
  locale-invariant JSON keys; the `explanation` is additive prose.

## Why a repair is or is not available

The central addition over the raw feature-lane packet is that every pane is
explicit about repair availability:

- An **`available`** pane names a `repair_candidate_id` (using the `repair.`
  prefix) and a real `reversal_class`, so the user can reason about how the
  change is undone before applying it. It carries **no** block reason.
- A **`blocked_*`** pane names a specific, non-generic
  `repair_unavailable_reason_code` (e.g.
  `api_auth_policy_denies_repair_in_managed_environment`,
  `profiler_capture_coverage_incomplete_for_repair`,
  `companion_reissue_reversal_path_unproven`). Empty or generic tokens
  (`unavailable`, `error`, `failed`, `unknown`, …) are rejected by validation. It
  carries **no** candidate id and its `reversal_class` is `not_applicable`.
- A **`not_applicable_healthy`** pane is reserved for the healthy state and
  carries neither a candidate nor a reason.

A user can therefore inspect, without leaving the Doctor flow, exactly why a
repair is blocked, unsupported, partial, or policy-denied.

## One machine meaning across every surface

Each pane records the `parity_surfaces` it renders on. Every pane must render on
the four **core** surfaces — `desktop_pane`, `cli_row`, `headless_json`, and
`support_export` — and may additionally render on `incident_packet` and
`public_truth`. Because the same pane carries the same finding code, diagnosis
state, repair availability, and exit class on every surface, desktop, CLI,
headless, support, incident, and public-truth views present the same finding and
repair-candidate identity.

The `machine_meaning_keys` (`finding_code`, `diagnosis_state`,
`repair_availability`, `cli_exit_class`) are the locale-invariant keys: localized
prose is additive only and may never change them.

## Exit semantics are canonical

The `cli_exit_class` is derived from the diagnosis state by a fixed map, so the
interactive CLI, headless runs, and the desktop product agree on the exit code
for every state:

| diagnosis state  | exit class          | exit code |
| ---------------- | ------------------- | --------- |
| `healthy`        | `ok_healthy`        | 0         |
| `partial`        | `advisory_findings` | 10        |
| `stale`          | `advisory_findings` | 10        |
| `target_mismatch`| `blocked`           | 20        |
| `unsupported`    | `unsupported`       | 30        |
| `policy_blocked` | `policy_denied`     | 40        |

Validation rejects any pane whose `cli_exit_class` does not match this map.

## Read-only and metadata-only

A pane is metadata about a finding: every field is a typed state or an opaque
ref. Each pane sets `redaction_class: metadata_safe_default` and
`raw_private_material_excluded: true`, and carries no credential bodies, raw
provider payloads, or mount/port/tunnel secrets.

## How downstream surfaces consume it

`export_projection()` produces a redaction-safe row set with the pane id, finding
code, probe id/version, diagnosis state, scope, repair availability, candidate
id, block reason, reversal class, exit class and stable exit code, a
cross-surface-stable flag, and the explanation, plus `available_repair_count`,
`blocked_repair_count`, and `cross_surface_stable_count`. Help/About, docs/help,
support exports, incident packets, and release/public-truth packets should ingest
this projection directly rather than restating explainability text by hand.

## Validation

`ProjectDoctorExplainabilityParity::validate()` reports every violation,
including unsupported schema version or record kind, non-canonical closed
vocabularies, empty required fields, a duplicate pane id, a finding code without
the Doctor prefix, a pane citing no evidence, a pane that is not stable across the
core surfaces, a missing locale-invariant machine-meaning key, a non-metadata-safe
redaction class, an exit class that does not match the canonical state map, a
healthy/availability disagreement, an available repair missing its candidate or
carrying a block reason, an unavailable repair carrying a candidate, a blocked
repair hiding behind a generic/empty reason, a repair candidate without the
`repair.` prefix, a repair-availability/reversal-class disagreement, and a summary
block that disagrees with the panes.
