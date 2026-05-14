# Support Bundle Alpha Manifest

This document describes the alpha support-bundle manifest path used by
the local preview and shell support export surfaces. The machine shape is
owned by
[`schemas/support/support_bundle_manifest.schema.json`](../../schemas/support/support_bundle_manifest.schema.json);
this document only explains the intended joins.

## Contract

Every preview/export manifest carries:

- exact build identity and exact-build refs copied from the running
  build;
- collection schema version, active redaction profile, policy context,
  and policy notes;
- preview rows with included and excluded classes before export;
- per-row redaction controls that can narrow capture but cannot export
  raw content from the alpha path;
- typed action reconstruction contexts for reviewed commands and
  externalized flows.

The action reconstruction context records the command id, command
descriptor ref, invocation session id, target identity ref,
origin/target/route/exposure classes, policy source, redaction class,
and exact-build refs. Support and incident readers must use those fields
instead of scraping activity-center, palette, or preview text.

## Redaction Posture

Local-first defaults remain conservative:

| Class | Default |
| --- | --- |
| `metadata_only` | included by default |
| `environment_adjacent` | included as metadata after preview |
| `code_adjacent` | omitted pending explicit item review |
| `high_risk` secret-bearing content | prohibited and represented by an omission marker |
| `high_risk` traces or dumps | retained locally unless a separate reviewed packet is produced |

`redaction_controls[].raw_content_export_allowed` is pinned to `false`
for the alpha manifest. If a later incident packet needs a broader raw
capture, it must use a separate reviewed path with its own manifest and
policy source.

## First Consumer

The first runtime consumer is
[`crates/aureline-shell/src/support_seed/mod.rs`](../../crates/aureline-shell/src/support_seed/mod.rs).
`SupportSeedSurface::reviewed_command_route_preview` consumes the
reviewed command enforcement row and the command invocation session, then
adds a `route_and_execution_truth` preview row plus one
`action_reconstruction_context`.

That preview is covered by
[`crates/aureline-shell/tests/support_bundle_alpha_manifest.rs`](../../crates/aureline-shell/tests/support_bundle_alpha_manifest.rs).
The protected support-bundle fixtures under
[`fixtures/support/support_seed_cases/`](../../fixtures/support/support_seed_cases/)
also include the preview classification summary and redaction controls.

## Reconstruction Rules

Support readers reconstruct a reviewed action in this order:

1. Read `build_identity.exact_build_refs`.
2. Read the matching `preview_items[]` row and
   `review_decisions[]` entry.
3. Read `action_reconstruction_contexts[]` for the command,
   invocation, target, route, exposure, policy source, and redaction
   class.
4. Read `redaction_report` and `redaction_controls[]` to confirm what
   was omitted, retained locally, or prohibited.

If a required command, route, target, policy source, or exact-build field
is missing for a reviewed action, the bundle is incomplete; readers
should record a reconstruction gap rather than infer from prose.
