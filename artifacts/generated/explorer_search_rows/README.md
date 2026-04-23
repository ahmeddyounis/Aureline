# Explorer / search row lineage-hint examples

Short, reviewer-facing `row_lineage_hint_record` examples for the row
contract frozen in
[`/docs/generated/lineage_hint_packet.md`](../../../docs/generated/lineage_hint_packet.md).

Every example projects a posture record under
[`/artifacts/generated/viewer_fallback_examples/`](../viewer_fallback_examples/)
onto explorer, quick-open, full-search, symbol-jump, docs-search,
cross-repo, graph-overlay, AI-citation, and support / export row
surfaces. The row hint carries enough truth that a surface listing the
object does not accidentally imply it is authored source; it does not
replace the posture record or the full generated-artifact lineage
record.

## What each example shows

| File | Case id | Artifact class / origin | Drift state / default edit posture | Highlights |
|---|---|---|---|---|
| [`build_output_release_binary_row.json`](./build_output_release_binary_row.json) | `generated.row_hint.build_output.release_binary` | `build_output` / `generated_artifact` | `in_sync` / `inspect_read_only` | Read-only row, rebuild action points back to the source tree plus manifests. |
| [`codegen_source_sibling_clean_row.json`](./codegen_source_sibling_clean_row.json) | `generated.row_hint.codegen_sibling.clean` | `codegen_source_sibling` / `generated_artifact` | `in_sync` / `edit_canonical_source` | Explorer and search rows keep the OpenAPI source visible; regeneration command stays on the row. |
| [`codegen_source_sibling_diverged_row.json`](./codegen_source_sibling_diverged_row.json) | `generated.row_hint.codegen_sibling.diverged` | `codegen_source_sibling` / `generated_artifact` | `diverged_from_generator` / `inspect_read_only` | Divergence chip with active override provenance reference; row action set stays read-only. |
| [`structured_lockfile_stale_row.json`](./structured_lockfile_stale_row.json) | `generated.row_hint.structured_lockfile.stale` | `structured_lockfile` / `generated_artifact` | `stale_inputs` / `structured_safe_edit` | Structured viewer is the default open; regenerate action is co-equal. |
| [`mirrored_docs_pack_drifted_row.json`](./mirrored_docs_pack_drifted_row.json) | `generated.row_hint.mirrored_pack.drifted` | `mirrored_pack_artifact` / `mirrored_artifact` | `mirror_drift` / `replace_by_mirror_promotion` | Refresh-mirror action, upstream ref preserved separately from local mirror version. |
| [`preview_snapshot_mock_row.json`](./preview_snapshot_mock_row.json) | `generated.row_hint.preview_snapshot.mock` | `preview_render_snapshot` / `preview_projection` | `mock_provenance` / `inspect_read_only` | Canonical-source redirect action, viewer-fallback token surfaces on the row. |
| [`notebook_output_fallback_row.json`](./notebook_output_fallback_row.json) | `generated.row_hint.notebook_output.fallback` | `notebook_output` / `generated_artifact` | `in_sync` / `clear_or_reexecute` | Structured-viewer-with-raw-text fallback; raw-text option stays reachable from the row. |

## Scope rules

- Every example carries `row_lineage_hint_schema_version: 1` and uses
  only the tokens frozen in
  [`artifact_edit_posture.schema.json`](../../../schemas/generated/artifact_edit_posture.schema.json)
  and the row-hint vocabulary in
  [`lineage_hint_manifest.yaml`](../../../fixtures/generated/lineage_hint_examples/lineage_hint_manifest.yaml).
- Rows reference the posture record by id via `artifact_posture_ref`;
  they do not inline override policy, active override provenance,
  structured-viewer fallback, or the full surface-projection array.
- Raw absolute paths, raw credentials, raw policy bodies, raw mirror
  payloads, and raw artifact bytes never appear. Redaction class is
  `metadata_safe_default` across this seed.
- Opaque ids and timestamps are chosen for review clarity rather than
  to mirror a real machine.

## Coverage contract

This directory MUST keep:

- at least one row example per required artifact class
  (`build_output`, `codegen_source_sibling`, `structured_lockfile`,
  `mirrored_pack_artifact`, `preview_render_snapshot`,
  `notebook_output`);
- at least one row example exercising each of
  `in_sync`, `stale_inputs`, `diverged_from_generator`,
  `mock_provenance`, and `mirror_drift`;
- at least one row exercising a non-`not_needed`
  `viewer_fallback_state` so the row chip renders the fallback token;
- at least one row that forbids `offer_unreviewed_direct_edit` on an
  AI-citation surface (see
  [`codegen_source_sibling_clean_row.json`](./codegen_source_sibling_clean_row.json));
- at least one row that forbids `export_body_without_provenance` on a
  support / export surface (see
  [`build_output_release_binary_row.json`](./build_output_release_binary_row.json)).

Adding a row that covers a new artifact class, provenance state,
default edit posture, rebuild intent class, viewer-fallback state, or
row surface class is additive-minor. Removing a row this directory
already covers is a breaking change.
