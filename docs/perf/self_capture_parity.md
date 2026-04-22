# Self-capture parity for benchmark packets

This document is the normative guide for relating a local
`self_capture` benchmark run to Aureline's benchmark-lab reference rows
without pretending the local machine is the lab.

Companion artifacts:

- [`/artifacts/perf/reference_hardware_manifest.yaml`](../../artifacts/perf/reference_hardware_manifest.yaml)
  — canonical hardware rows and display classes for benchmark packets.
- [`/artifacts/perf/lab_image_manifest.yaml`](../../artifacts/perf/lab_image_manifest.yaml)
  — lab-image revisions, environment rows, calibration checklists, and
  comparability-reset rules.
- [`/docs/benchmarks/benchmark_lab_run_results.md`](../benchmarks/benchmark_lab_run_results.md)
  — shared run-result schema used by both `reference_capture` and
  `self_capture`.
- [`/docs/benchmarks/benchmark_publication_pack_template.md`](../benchmarks/benchmark_publication_pack_template.md)
  — publication packet template that quotes the same hardware and
  environment ids.
- [`/docs/benchmarks/public_comparison_rules.md`](../benchmarks/public_comparison_rules.md)
  — external-claim rules and withdrawal triggers.
- [`/docs/perf/power_thermal_methodology.md`](./power_thermal_methodology.md)
  — power, thermal, and efficiency methodology that reuses the same
  display, power-posture, and calibration vocabulary.

If this document disagrees with the PRD, Technical Architecture
Document, Technical Design Document, or UI / UX Spec, those documents
win and this file updates in the same change.

## Why this exists

`reference_capture` and `self_capture` share one benchmark run-result
schema on purpose. The difference between them is evidentiary posture,
not packet family.

A self-capture is useful for:

- local diagnosis,
- directional regression confirmation,
- confirming whether a user report is plausibly real on another host,
  and
- narrowing which reference row a future protected rerun should target.

A self-capture is not, by itself:

- a protected release gate,
- a public head-to-head claim,
- a substitute for a benchmark-lab rerun on frozen hardware and lab
  image, or
- permission to silently widen a result from "current machine" to
  "reference hardware."

## Required disclosures on every self-capture packet

Every self-capture benchmark packet or dashboard card MUST surface:

- `run_context = self_capture`;
- one `hardware_definition.definition_id`;
- one `environment_definition.definition_id`;
- one `environment_definition.display_class_id`;
- one `environment_definition.lab_image_id` plus
  `environment_definition.lab_image_revision`;
- exact-build identity and workspace version;
- whether the run was cold, warm, or already-loaded;
- extension/config posture when it differs from the claimed default;
- power and thermal posture; and
- a comparability note before any number is compared with a reference
  row.

If the current machine has not been promoted to a benchmark-council
reference row, the packet SHOULD cite
`hardware_definition.self_capture.current_machine_reported` and
`environment_definition.self_capture.current_machine_default` rather
than inventing a private label.

## Parity postures

These are disclosure postures, not a second schema. A dashboard or
packet may use these labels in prose while still rendering the same
run-result fields.

| Posture | When to use | What it allows |
|---|---|---|
| `same_reference_row_replay` | The run used the same hardware row and environment row as the approved reference, but happened outside the protected nightly lane. | Compare directionally to the reference row, but do not promote the result to release or public-proof status until a protected `reference_capture` reruns it. |
| `same_family_narrowed` | OS and architecture match a reference family, but display class, lab-image revision, power policy, thermal posture, or calibration revision differ. | Compare trend direction only. The packet MUST name every drifted field and say the result is narrowed. |
| `cross_family_informational_only` | OS, architecture, display class, or environment family differs materially from every approved reference row. | Use for local diagnosis only. Do not treat the result as evidence for a reference-row claim. |

## How to compare a self-capture honestly

1. Identify the actual run context first.
   If the run was captured on a developer or user machine, keep
   `run_context = self_capture` even if the hardware happens to match a
   reference laptop.

2. Cite the actual hardware row and environment row.
   If the host is not an approved reference row, cite the self-capture
   placeholder rows instead of pretending a match.

3. Name the nearest reference row only when the family really matches.
   Matching means at least OS family, architecture, and workload scope
   line up. A macOS ARM64 self-capture should not be described as
   "matching" a Windows x86_64 reference row just because the percentile
   looks favorable.

4. Treat display, lab image, power, thermal, and calibration drift as
   first-class.
   If any of those differ, the packet MUST say so before presenting the
   comparison. This is the normal path, not a footnote.

5. Keep the conclusion narrow.
   A self-capture may say "directionally faster than the current macOS
   ARM64 reference row under a different unmanaged local environment."
   It may not say "Aureline meets the published reference-hardware bar"
   unless a protected `reference_capture` proves it.

## Mandatory comparability notes

The following changes always require an explicit comparability note, and
usually reset a wider claim until the reference row is rerun:

- lab-image revision changes;
- power-policy or AC versus battery posture changes;
- display-class or screen-configuration changes;
- thermal-posture changes; and
- calibration-rule-set changes.

These reset triggers are machine-readable in
[`/artifacts/perf/lab_image_manifest.yaml`](../../artifacts/perf/lab_image_manifest.yaml).
Packets and dashboards should read those ids rather than inventing new
drift labels.

## Examples

- A developer reruns the benchmark on the same Apple Silicon reference
  laptop and same lab image after a local code change.
  This is `same_reference_row_replay`: useful directional evidence, but
  still not the protected nightly reference until the governed lane
  records it.

- A contributor runs the benchmark on a Linux x86_64 laptop with the
  same OS family as the Ubuntu reference row but on battery saver with a
  different display setup.
  This is `same_family_narrowed`: compare direction only and name the
  battery/display drift up front.

- A user captures a Windows run on an unmanaged gaming desktop with a
  high-refresh HDR monitor.
  This is `cross_family_informational_only`: useful for diagnosis, not
  for reference-row claims.

## Relationship to the shared schema

The benchmark dashboard, benchmark-publication packet, and local export
should all project from the same run-result schema:

- `hardware_definition.*` names the hardware row,
- `environment_definition.*` names the display class, lab-image
  revision, power posture, thermal posture, and calibration revision,
  and
- `run_context` says whether the packet is protected reference evidence
  or local self-capture.

That is enough to render "reference row vs self-capture" honestly
without minting a second comparison packet family.
