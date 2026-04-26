# Component-contract fixtures

Worked fixtures for the component-contract packet frozen in
[`/docs/ux/component_contract_template.md`](../../../docs/ux/component_contract_template.md).
Every JSON file in this directory is intended to conform to
[`/schemas/design/component_contract.schema.json`](../../../schemas/design/component_contract.schema.json).

The fixtures exist so design review, implementation, QA, docs, and
extension-conformance work can all cite one packet shape instead of
inventing local component checklists. Each example keeps the same
sections:

- anatomy;
- explicit state machine;
- content and keyboard rules;
- accessibility notes;
- token, density, motion, and degraded-state bindings;
- theme / icon / motion / localization hooks;
- extension-parity guidance; and
- review gate refs for checklist, evidence-pack, and waiver linkage;
  and
- typed evidence hooks.

## Fixtures

- [`command_palette_result_row.json`](./command_palette_result_row.json)
  — command / result-row packet for command-discovery surfaces.
- [`permission_trust_prompt.json`](./permission_trust_prompt.json)
  — host-owned trust / permission prompt packet with raw identifier
  visibility and safe-local fallback behavior.
- [`durable_job_row.json`](./durable_job_row.json)
  — durable job-row packet for long-running or throttled work that must
  not degrade into toast-only delivery.
- [`policy_locked_settings_row.json`](./policy_locked_settings_row.json)
  — settings-row packet demonstrating shared taxonomy refs for locked,
  read-only, pending, degraded, current, and disabled states plus the
  expanded evidence hooks.

## Intended usage

- **Schema conformance:** the JSON shape is the contract of record.
- **Design review:** reviewers can walk from component packet to theme,
  accessibility, localization, and evidence refs without relying on
  implied conventions.
- **Extension guidance:** extensions can tell whether the host component
  must match, may narrow with disclosure, or should hand off.
- **Evidence planning:** keyboard, AT, token-drift, screenshot, and
  state-machine hooks stay typed instead of drifting into ad hoc notes.
- **Gate wiring:** `review_gate_refs` points each packet at the reusable
  component checklist, review gate manifest, design evidence-pack id,
  and waiver state.
