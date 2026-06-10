# Add Chronology and Reverse-Step Controls, History Partiality Cues, and Import or Export Packets

This document is the reviewer-facing landing page for the M5 chronology,
reverse-step, history-partiality, and import/export packet lane.

## Scope

This lane governs how profiler and trace surfaces:

- show chronology controls (play, pause, step forward, step backward, jump to time,
  speed control, scrub) with enabled-state labels so users always know which
  navigation verbs are available and why;
- display reverse-step actions (reverse continue, reverse step over, reverse step
  into, reverse step out, reverse run to cursor) with target event refs and mapping
  quality so reverse-step surfaces never promise steps they cannot deliver;
- surface history partiality cues (truncated start/end, filtered out, missing mapping,
  sampling gap, policy redacted) with severity, time range, and explanation so
  incomplete, truncated, or filtered history is never shown as a complete canvas;
- handle import and export packets with direction, format kind, format version,
  provenance, integrity hash, and content summary so imported or exported evidence
  is always traceable to its origin and build;
- degrade honestly by showing degraded-state labels when chronology support,
  reverse-step availability, mapping fidelity, or packet integrity are weak.

## Canonical Artifacts

- **Implementation:** `crates/aureline-profiler/src/add_chronology_and_reverse_step_controls_history_partiality_cues_and_import_or_export_packets/`
- **Packet:** `artifacts/perf/m5/add-chronology-and-reverse-step-controls-history-partiality-cues-and-import-or-export-packets.json`
- **Schema:** `schemas/perf/add-chronology-and-reverse-step-controls-history-partiality-cues-and-import-or-export-packets.schema.json`
- **Fixtures:** `fixtures/performance/m5/add-chronology-and-reverse-step-controls-history-partiality-cues-and-import-or-export-packets/`

## Surfaces

| Surface | Claim | Rationale |
|---|---|---|
| Chronology control bar | Stable | Shows play, pause, step forward, step backward, jump-to-time, speed control, and scrub with enabled-state labels and mapping quality. |
| Reverse-step toolbar | Stable | Shows reverse-continue, reverse-step-over, reverse-step-into, reverse-step-out, and reverse-run-to-cursor with enabled state, target event refs, and mapping quality. |
| History partiality indicator | Stable | Shows truncated start/end, filtered-out events, missing mappings, sampling gaps, and policy redactions with severity, time range, and explanation. |
| Import packet dialog | Stable | Shows direction, format kind, format version, provenance, integrity hash, and content summary for imported evidence. |
| Export packet dialog | Preview | Redaction-safe export flows for chronology and reverse-step evidence are still under qualification. |
| Packet integrity review | Preview | Deep integrity verification and policy-blocked packet detection are still under qualification. |

## Chronology Control Kinds

The module carries a closed chronology-control vocabulary:

- `play` — play forward from current position;
- `pause` — pause at current position;
- `step_forward` — step forward one event or frame;
- `step_backward` — step backward one event or frame;
- `jump_to_time` — jump to an absolute timestamp;
- `speed_control` — adjust playback speed;
- `scrub` — scrub along the timeline.

Every chronology control MUST show its enabled state and a degraded-state label.

## Reverse-Step Action Kinds

The module carries a closed reverse-step vocabulary:

- `reverse_continue` — continue execution in reverse;
- `reverse_step_over` — step over one function call in reverse;
- `reverse_step_into` — step into one function call in reverse;
- `reverse_step_out` — step out of the current function in reverse;
- `reverse_run_to_cursor` — run backwards to the cursor position.

Every reverse-step action MUST show its enabled state, mapping quality, and a
 degraded-state label.

## History Partiality Cue Kinds

The module carries a closed partiality-cue vocabulary:

- `truncated_start` — history is truncated at the start;
- `truncated_end` — history is truncated at the end;
- `filtered_out` — some events were filtered out;
- `missing_mapping` — mapping is missing for some events;
- `sampling_gap` — sampling gap caused missing events;
- `policy_redacted` — events were redacted by policy.

Every history partiality cue MUST show its severity, an explanation, and a
degraded-state label when the severity is `warning` or `critical`.

## Partiality Severity Levels

- `info` — history is mostly complete;
- `warning` — a noticeable portion of history is missing;
- `critical` — history is substantially incomplete or misleading.

A `critical` severity blocks a stable claim for the surface.

## Import/Export Packet Format Kinds

The module carries a closed format-kind vocabulary:

- `trace_bundle` — trace bundle format;
- `profile_snapshot` — profile snapshot format;
- `regression_baseline` — regression baseline format;
- `notebook_archive` — notebook archive format;
- `replay_capture` — replay capture format.

Every import/export packet row MUST show its provenance, integrity hash, and
format version.

## Downgrade and Rollback

- Any surface that claims `stable` with an incomplete guard set is narrowed
  automatically by the validator.
- Chronology control rows MUST show enabled state and a degraded-state label;
  missing labels trigger a validation violation.
- Reverse-step action rows MUST show enabled state, mapping quality, and a
  degraded-state label; missing labels trigger a validation violation.
- History partiality cue rows MUST show severity, explanation, and a degraded-state
  label when applicable; missing labels trigger a validation violation.
- Import/export packet rows MUST show provenance, integrity hash, and format version;
  missing labels trigger a validation violation.

## Invariants

- Raw payload bytes, raw command lines, secrets, and ambient credentials do not
  cross this boundary.
- Every chronology control carries enabled state and shows it.
- Every reverse-step action carries mapping quality and shows it.
- Every history partiality cue carries severity, explanation, and shows a degraded-state
  label when applicable.
- Every import/export packet carries provenance, integrity hash, and format version.
- Trace bundles and profile snapshots are immutable once captured; derived controls,
  actions, cues, and packets are separate derived artifacts with their own provenance.
