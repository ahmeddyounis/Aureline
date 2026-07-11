# M5 notebook-kernel-output component matrix contract

This document is the human-readable companion to the frozen M5 notebook-kernel-output
component matrix. The authoritative gate is the Rust validator in
`crates/aureline-notebook/src/freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix`.
The checked-in support export under `artifacts/release/m5-notebook-kernel-output-proof/` is the
single source of truth; the schemas under `schemas/ui/` document the shape.

## Purpose

The matrix freezes the reusable notebook kernel / output components so notebook, kernel-manager,
output-viewer, debugger, review, and CLI surfaces stop drifting on document, runtime, trust, and
recovery language across claimed M5 notebook workflows. It names each component family once and
binds it to canonical `.ipynb` identity, selected kernel origin / class, execution and output
trust state, stale-versus-live output honesty, restart / reconnect consequences,
preserved-versus-lost state, and choose-another-kernel recovery before widening consumer
coverage.

## Component families

- `notebook_document_header` — where a notebook came from (local, remote, managed-workspace,
  imported, scratch/untitled, unknown-source `.ipynb`) and where its canonical identity stands
  (saved clean, unsaved changes, autosaved, conflicted, read-only, recovered).
- `kernel_state_strip` — where a kernel stands in execution (idle ready, queued pending, busy
  running, interrupted, dead / no kernel, disconnected / reconnecting) and how it is connected
  (connected local, connected remote, reconnecting, disconnected, connection lost, never
  connected).
- `kernel_picker_row` — what kind of kernel a candidate is (local interpreter, virtual env,
  conda env, container kernel, remote kernel, managed kernel) and where its selection stands
  (selected, available, recommended, incompatible, unavailable, needs install).
- `kernel_origin_pill` — where a kernel physically runs (local host, SSH remote, container,
  devcontainer, managed workspace, browser bridge) and how trusted that origin is (trusted,
  first-party, third-party, unverified, restricted, unknown).
- `output_trust_banner` — an output's trust class (trusted, sanitized, sandboxed, raw / active,
  blocked, unknown) and its freshness (live, stale, cached, cleared, superseded, no output).
- `output_provenance_chip_group` — what produced an output (cell, run, imported, restored,
  external, unknown) and how completely its execution lineage resolves (complete, partial,
  missing, execution count pinned / drifted, provenance stale).
- `restart_consequence_card` — which restart / interrupt action it describes (restart kernel,
  restart and run all, interrupt kernel, shutdown kernel, reconnect kernel, clear outputs) and
  what survives it (state preserved / lost, variables cleared, outputs retained / cleared, no
  consequence).
- `kernel_recovery_card` — which recovery action it offers (reconnect, restart clean, choose
  another kernel, reattach session, start local fallback, wait for managed) and where recovery
  stands (recoverable, reconnect available, restart required, no kernel available, recovery
  blocked, recovered).

## The one disposition vocabulary

Every component binds the same controlled disposition vocabulary — `no_kernel`, `queued`,
`busy`, `ready`, `disconnected`, `managed`, `remote`, `stale_output`, `sanitized`, `active`,
`reconnect`, `restart_clean`, `choose_another_kernel` — so no surface invents a parallel word
for a kernel-free, busy, disconnected, managed, remote, stale-output, sanitized, active,
reconnect, restart-clean, or choose-another-kernel state.

## Hard invariants

Every row asserts these are `false`:

- `recovery_card_implies_rerun` — a kernel recovery card never implies a rerun.
- `presents_stale_output_as_live` — no component presents stale output as live truth.
- `hides_trust_class_behind_hover_only` — no output banner hides a raw / sanitized / active
  trust class behind a hover-only affordance.
- `collapses_kernel_origins_into_one_badge` — no origin pill collapses local, SSH, container,
  managed, or browser-bridge kernels into one unlabeled badge.

## Mandatory and truth labels

Every component must be able to show `identity`, `state`, and `keyboard_route`. The remaining
labels — `kernel_origin_and_class`, `output_trust_and_freshness`, and `restart_and_recovery` —
close the acceptance-criteria ambiguity about kernel origin / class, output trust / freshness,
and restart / recovery.

## Deployment lines and accessibility

Every component keeps the same truth across the `local_oss`, `self_hosted`, `managed`,
`air_gapped`, and `mirror_offline` deployment lines, and offers every non-visual accessibility
route (`keyboard_focusable`, `screen_reader_announced`, `non_hover_reachable`,
`pointer_optional`, `high_contrast_safe`, `support_exportable`) so no notebook, kernel, or
output truth is hover-only, pointer-only, or visually encoded alone.

## Export safety

The packet is metadata-only. Raw notebook JSON, raw cell source, raw output bytes, raw
kernel-protocol frames, and private endpoints never cross the export boundary.

## Regenerating the artifacts

The headless emitter is the only mint-from-truth path:

```sh
BIN=aureline_notebook_m5_notebook_kernel_output_component_matrix
cargo run -q -p aureline-notebook --bin $BIN -- support-export > artifacts/release/m5-notebook-kernel-output-proof/support_export.json
cargo run -q -p aureline-notebook --bin $BIN -- csv > artifacts/release/m5-notebook-kernel-output-proof/matrix.csv
cargo run -q -p aureline-notebook --bin $BIN -- report > artifacts/design/m5-notebook-kernel-output-component-matrix.md
cargo run -q -p aureline-notebook --bin $BIN -- fixture-kernel-recovery-card-beta-narrowed > fixtures/ui/m5-notebook-kernel-output-components/kernel_recovery_card_beta_narrowed.json
cargo run -q -p aureline-notebook --bin $BIN -- fixture-output-trust-banner-preview-narrowed > fixtures/ui/m5-notebook-kernel-output-components/output_trust_banner_preview_narrowed.json
```

The inline tests assert the checked-in export byte-for-byte matches the seed builder, so any
drift between code and artifact fails `cargo test -p aureline-notebook`.
