# Token-export and token-drift fixtures

Worked fixtures for the token-conformance audit format frozen in
[`/docs/design/token_conformance_audit.md`](../../../docs/design/token_conformance_audit.md)
and the fail-gate rules in
[`/artifacts/design/token_drift_rules.yaml`](../../../artifacts/design/token_drift_rules.yaml).

Each JSON file in this directory pairs:

- a per-surface `audit_row` carrying the fields the audit format
  reserves (`audited_surface_class`, `surface_owner_role_class`,
  `manifest_ref`, `consumed_*_refs`, `gap_records`, `findings`,
  `gate_state_class`, `policy_context`, `redaction_class`,
  `minted_at`);
- where applicable, the `design_token_export_manifest_record` the
  surface resolved against, conforming to
  [`/schemas/design/token_export_manifest.schema.json`](../../../schemas/design/token_export_manifest.schema.json);
  and
- the resulting `token_export_audit_event_record` emitted on the
  design-token-vocabulary audit stream.

Fixtures exist so design review, conformance tooling, and release
evidence packets can cite the same row shape instead of reinventing
local audit checklists.

## Fixtures

- [`shell_chrome_first_party_manifest_clean.json`](./shell_chrome_first_party_manifest_clean.json)
  — `pass`. First-party shell-chrome surface resolves a complete
  manifest cleanly; every per-entity invariant holds.
- [`manifest_missing_token_family_axis_refused.json`](./manifest_missing_token_family_axis_refused.json)
  — `block`. Manifest emission refused because a token-family axis
  is missing; routes to `token_family_class_unresolved`.
- [`extension_partial_high_contrast_inheritance_pass.json`](./extension_partial_high_contrast_inheritance_pass.json)
  — `pass_with_disclosed_gap`. Extension-contributed surface ships
  dark and light reference theme rows and inherits high-contrast
  support from the host; gap is declared.
- [`embedded_surface_inert_placeholder_pass.json`](./embedded_surface_inert_placeholder_pass.json)
  — `pass_with_disclosed_gap`. Embedded surface uses the host's
  `inert_placeholder` for an unmapped semantic role; gap is
  declared.
- [`first_party_token_fork_blocked.json`](./first_party_token_fork_blocked.json)
  — `block`. First-party shell minted a parallel `gold-v2` family;
  routes to `token_family_repurposed_without_decision_row`.
- [`notification_color_alone_state_blocked.json`](./notification_color_alone_state_blocked.json)
  — `block`. Notification surface conveyed a stale state by colour
  alone; routes to `color_alone_conveyed_required_meaning`.
- [`palette_canvas_hard_coded_z_index_blocked.json`](./palette_canvas_hard_coded_z_index_blocked.json)
  — `block`. Palette canvas emitted a hard-coded z-index; routes to
  `layer_order_hard_coded`.
- [`density_changed_information_architecture_blocked.json`](./density_changed_information_architecture_blocked.json)
  — `block`. A density mode changed column visibility; routes to
  `density_changed_information_architecture`.

## Intended usage

- **Schema conformance:** every embedded `design_token_export_manifest_record`
  and `token_export_audit_event_record` validates against the
  boundary schema.
- **Drift evaluator inputs:** the conformance evaluator reads
  `audit_row.findings` and the `rule_id` it cites against
  `token_drift_rules.yaml` to confirm the gate verdict mechanically.
- **Design review:** reviewers cite `audit_finding_class` and the
  resolved `gate_state_class` instead of describing screenshots.
- **Release evidence:** release packets attach the
  `token_export_audit_event_record` ids without re-deriving
  verdicts.
