# Project Doctor alpha contract

This document publishes the alpha Project Doctor diagnosis contract and
the read-only probe-pack baseline. The checked-in schemas and probe pack
are the canonical source for the lane:

- [`/schemas/project_doctor/probe.schema.json`](../../schemas/project_doctor/probe.schema.json)
  defines the probe-pack envelope and one read-only probe record.
- [`/schemas/project_doctor/finding.schema.json`](../../schemas/project_doctor/finding.schema.json)
  defines the finding record and shared finding-vocabulary row.
- [`/artifacts/support/project_doctor_probe_pack_alpha.yaml`](../../artifacts/support/project_doctor_probe_pack_alpha.yaml)
  is the alpha read-only probe-pack baseline.
- [`/crates/aureline-support/src/project_doctor/mod.rs`](../../crates/aureline-support/src/project_doctor/mod.rs)
  is the first support/export consumer. It parses the pack, validates the
  read-only invariants, and projects machine-readable and human-readable
  outputs from the same finding vocabulary.

The contract narrows the earlier supportability artifacts into one
runtime-consumable shape. It reuses these existing sources instead of
restating their vocabularies:

- [`project_doctor_probe_contract.md`](./project_doctor_probe_contract.md)
  for probe admission, no-hidden-side-effect rules, and repair handoff.
- [`project_doctor_packet.md`](./project_doctor_packet.md)
  for scenario families, diagnosis latency, and escalation completeness.
- [`probe_family_matrix.md`](./probe_family_matrix.md)
  for probe-family inputs, forbidden diagnosis actions, and seed cases.
- [`supportability_slo_and_pack_contract.md`](./supportability_slo_and_pack_contract.md)
  for evidence-pack classes, redaction defaults, and first actionable
  diagnosis fields.

## Contract Shape

The probe-pack artifact is a `project_doctor_probe_pack_record`. It
contains:

- schema refs for the Project Doctor probe and finding schemas;
- source contract refs for the support docs and probe-family matrix;
- a default execution policy with `read_only_by_default: true`;
- vocabulary bindings back to `schemas/support/*` definitions;
- redaction-safe output routing for headless JSON, human summary rows,
  support-bundle refs, runbook handoff, and escalation packets;
- one shared `finding_vocabulary[]` list; and
- one `project_doctor_probe_record` per alpha probe.

Each probe record carries the fields support, UI, CLI/headless, and
release review need before running or rendering the probe:

| Field group | Required contract |
|---|---|
| Identity | `probe_id`, `probe_code`, `probe_family_class`, `probe_version`, lifecycle status |
| Scope | explicit `target_scope` with scope class, opaque scope ref, and support contexts |
| Read-only posture | `read_only_default`, `mutability_class`, `doctor_admission_class`, allowed and forbidden side effects |
| Evidence | evidence key, source class, signal class, data class, redaction class, inclusion class, replayability, output routes, and schema ref |
| Finding output | stable finding codes plus explicit unknown and unsupported finding codes |
| Repair handoff | whether repair is available, preview-only, handoff-only, or unsupported |
| Human output | stable text keys that point back to the same finding vocabulary |

## Read-Only Baseline

Every alpha probe in the baseline is read-only by default. A probe may:

- read existing manifests, hashes, health events, counters, and
  redaction-safe summaries;
- write a local evidence row or local preview manifest only when a
  governed repair/export surface owns that write; and
- issue a consent-gated read-only reachability check for network,
  proxy, or certificate diagnosis.

A probe may not:

- execute repo-owned hooks or activators;
- mutate cache or index state;
- retarget, reapprove, reattach, republish, or join routes/sessions;
- create, rewrite, or delete user files or durable state;
- widen trust, credentials, policy, or entitlement state;
- write to external services;
- activate third-party extensions; or
- collect high-risk payload bodies.

Any mutation is represented as a repair handoff, repair preview,
support-bundle export, runbook handoff, or escalation packet. It is not
part of diagnosis.

## Finding Vocabulary

Machine-readable output and human-readable output use the same
`finding_vocabulary[]` rows. Finding codes and keys are stable and never
localized. Human text may localize later, but it must keep the same
finding code, severity, confidence, repair availability, unsupported
state, and evidence refs.

The alpha vocabulary explicitly covers:

- toolchain missing or out of range;
- trust or policy blocks;
- watcher degradation;
- proxy or certificate failure;
- extension host crash loops;
- cache/schema drift;
- local-history integrity states with no supported repair;
- wrong remote target or route uncertainty; and
- insufficient evidence.

Unsupported and unknown states are first-class outputs. They must be
emitted as typed finding rows instead of being hidden or collapsed into
generic failure text.

## Proof Path

The protected proof path is the `aureline-support` consumer test suite.
It verifies that:

- the pack parses from the checked-in YAML artifact;
- every probe is read-only by default;
- every probe declares target scope, evidence classes, redaction-safe
  output routing, and repair handoff state;
- every emitted finding code resolves through the shared vocabulary;
- at least one finding demonstrates repair available, preview-only, and
  unsupported repair states; and
- the human and headless projections are derived from the same
  vocabulary rows.

Run:

```sh
cargo test -p aureline-support project_doctor
```
