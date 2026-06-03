# Proof packet: Stabilize stable CLI/headless schemas, machine-readable output, and support/export compatibility promises

## What this register proves

This register proves that every CLI command surface, headless output schema, machine-readable output format, and support/export compatibility promise the M4 stable line claims is either:

1. **Finalized stable** — backed by a current proof packet, complete evidence, and an owner sign-off; or
2. **Finalized on waiver** — held on an active, unexpired waiver that covers a recorded gap with a reviewable rationale; or
3. **Narrowed below the cutline** — automatically downgraded because its proof packet aged out, its evidence is incomplete, its schema broke, its waiver expired, its owner sign-off is absent, or the public claim it backs is itself below the cutline.

The register does **not** allow an unbacked row to inherit an adjacent backed row's published label.

## Current snapshot (as of 2026-06-02)

| Entry | Kind | State | Release-blocking | Label | Gap reason |
|---|---|---|---|---|---|
| cli:core_commands | CLI command surface | Finalized stable | Yes | Stable | — |
| cli:search | CLI command surface | Finalized stable | No | Stable | — |
| headless:json_output | Headless output schema | Narrowed stale | Yes | Beta | Proof packet freshness breached |
| headless:xml_output | Headless output schema | Narrowed schema breaking | No | Preview | Schema breaking change |
| machine_readable:tab_separated | Machine-readable format | Finalized stable | No | Stable | — |
| machine_readable:json_lines | Machine-readable format | Narrowed evidence incomplete | No | Preview | Evidence incomplete |
| support_export:support_bundle | Support/export compatibility | Finalized on waiver | Yes | Stable | — (waiver covers incomplete migration hints) |
| support_export:diagnostics_json | Support/export compatibility | Narrowed stale | No | Preview | Proof packet freshness breached |
| support_export:export_format_compat | Support/export compatibility | Finalized stable | No | Stable | — |

## Qualification verdict

**Hold** — one release-blocking row is narrowed below the cutline:

- `headless:json_output` — proof packet breached its freshness SLO

## Re-verification steps

1. Parse the checked-in register at `artifacts/release/stabilize_stable_cli_headless_schemas_machine_readable_output.json` into the typed model.
2. Confirm `schema_version == 1` and `record_kind == "cli_headless_schema_machine_readable_output_stabilize"`.
3. Confirm every `CliHeadlessKind` variant has at least one covering row.
4. Confirm every declared `release_blocking_surface_ref` has at least one covering row.
5. Confirm the computed summary matches the stored summary.
6. Confirm the computed publication decision matches the stored decision.
7. Confirm the computed blocking rule ids and entry ids match the stored values.
8. Review any active gap reasons and confirm the narrowing automation has not left an unbacked row at or above the stable cutline.

## Companion artifacts

- [`/artifacts/release/stabilize_stable_cli_headless_schemas_machine_readable_output.json`](../../artifacts/release/stabilize_stable_cli_headless_schemas_machine_readable_output.json)
- [`/docs/m4/stabilize-stable-cli-headless-schemas-machine-readable-output.md`](../../docs/m4/stabilize-stable-cli-headless-schemas-machine-readable-output.md)
- [`/schemas/release/stabilize-stable-cli-headless-schemas-machine-readable-output.schema.json`](../../schemas/release/stabilize-stable-cli-headless-schemas-machine-readable-output.schema.json)
