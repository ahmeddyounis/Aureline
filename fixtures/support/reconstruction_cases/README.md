# Cross-packet reconstruction cases

These are the worked reconstruction cases the drill at
[`docs/support/reconstruction_drill.md`](../../../docs/support/reconstruction_drill.md)
defines. Each case:

- maps to exactly one `scenario_class` row in
  [`artifacts/support/reconstruction_checklist.yaml`](../../../artifacts/support/reconstruction_checklist.yaml);
- names the exported packet family the reviewer opens first
  (typically an `object_handoff_packet_record` or a
  `support_bundle_record`);
- lists the typed refs and typed-absent tokens the reviewer reads to
  resolve each of the six correlation axes (command, route and target
  and origin and exposure, docs and source-version applicability,
  exact-build identity, claim row, known-limit note);
- declares the `reconstructed_sentence` the drill produces, built from
  typed fields only — no free-text paraphrase is permitted; and
- declares the `reconstruction_outcome` the case is shaped to produce
  (`reconstructable_without_gap`, `reconstructable_with_typed_absence`,
  or `reconstruction_blocked_with_escalation`).

These cases are contracts over the exported packet set, not full
packet bodies. They point at the artifacts the reviewer must open, and
at the field-level joins between them, so a reconstruction reviewer
can pivot from one scenario class → one case → the exact packet refs
and typed fields the drill expects without reading raw source code or
relying on oral history.

Case list:

- `local_only_workspace_format_action.yaml`
- `provider_bearing_remote_attach.yaml`
- `mirrored_offline_docs_pack_import.yaml`
- `wrong_target_remote_attach_corrected.yaml`

Every case cites its scenario class by stable id so the
[`reconstruction_checklist.yaml`](../../../artifacts/support/reconstruction_checklist.yaml)
rows can bind 1:1.
