# Implement the notebook header, kernel bar, execution-locus chips, and paired-export state

## Purpose

This document describes the M05-013 composed UI surface that backs the notebook header, kernel bar, execution-locus chips, and paired-export state. It reuses the closed vocabularies and backing records from the `runtime_truth` module and adds the `ExecutionLocusChip` record so execution locus is visible wherever the user can run, restart, debug, or export.

## Principles

- The notebook header should answer five questions without requiring a click: what file is this, is it trusted, which kernel is selected, where does that kernel run, and is `.ipynb` still the canonical source of truth?
- Document trust, kernel availability, execution locus, and paired-export state are distinct labels and may differ without collapsing into one generic status.
- A trusted notebook can still have no runnable kernel; an untrusted notebook can still be editable.
- Local vs remote vs container vs managed kernel boundaries stay visible anywhere run, restart, debug, inspect, or export actions appear.
- Paired script/Markdown views are interoperability helpers, not hidden replacements for the canonical `.ipynb`.
- Execution-locus chips must never imply an active kernel when the state is degraded, disconnected, reconnecting, or policy-blocked.

## Model overview

### ExecutionLocusChip

The compact chip that communicates execution locus:

- `chip_id` — stable opaque chip identity.
- `document_id_ref` — opaque ref to the notebook document.
- `chip_class` — `local_host`, `local_container`, `ssh_remote`, `managed_workspace`, `browser_bridge`, `service_plane`, or `no_kernel`.
- `chip_state` — `active`, `degraded`, `disconnected`, `reconnecting`, or `policy_blocked`.
- `target_name_label` — export-safe target name (e.g. `devcontainer:ml-cuda`).
- `tooltip_label` — export-safe tooltip.
- `boundary_cue_visible` — whether the local-vs-remote boundary cue is shown.

### NotebookHeaderKernelBarState

The composed UI-consumable state that binds:

- `document_id_ref`, `document_path_token_ref`, `document_title_label` — document identity.
- `document_trust_class` — projected document-trust chip.
- `dirty_state_class` — dirty/clean/unreconciled state.
- `kernel_selection_state` — selected, pending, narrowed, or unavailable.
- `kernel_origin_class` — where the kernel runs.
- `execution_locus_chips` — one or more chips describing execution locus.
- `paired_export_posture` — not applicable, derived to script, or derived to Markdown.
- `available_actions` — select, change, restart, interrupt, reconnect, shutdown.
- `last_successful_run` — optional recency cue.
- `auto_rerun_forbidden` — invariant field, always `true`.

## Closed vocabularies

| Vocabulary | Variants | Location |
|---|---|---|
| `ExecutionLocusChipClass` | `local_host`, `local_container`, `ssh_remote`, `managed_workspace`, `browser_bridge`, `service_plane`, `no_kernel` | `aureline-notebook` crate |
| `ExecutionLocusChipState` | `active`, `degraded`, `disconnected`, `reconnecting`, `policy_blocked` | `aureline-notebook` crate |

Reused from `runtime_truth`:

| Vocabulary | Variants | Location |
|---|---|---|
| `NotebookDocumentTrustClass` | `document_trust_inherited_from_workspace`, `document_trust_elevated_on_explicit_grant`, `document_trust_restricted_by_policy`, `document_trust_revoked` | `aureline-notebook` crate |
| `NotebookDirtyStateClass` | `clean_saved`, `dirty_user_edited`, `dirty_cell_ordering_changed`, `dirty_external_change_detected`, `unreconciled_canonical_mismatch` | `aureline-notebook` crate |
| `KernelOriginClass` | `no_kernel`, `local_managed_toolchain_kernel`, `local_provisioned_kernel`, `remote_agent_primary_kernel`, `managed_workspace_agent_kernel`, `provider_side_remote_kernel`, `compatibility_bridge_remote_kernel` | `aureline-notebook` crate |
| `KernelSelectionState` | `no_kernel_selected`, `selected_kernel_resolved`, `selection_pending_user`, `selection_pending_resolver`, `selection_narrowed_by_policy`, `selection_unavailable_no_compatible_kernel` | `aureline-notebook` crate |
| `KernelBarActionClass` | `select_kernel`, `change_kernel`, `restart_kernel`, `restart_kernel_and_run_all`, `interrupt_kernel`, `reconnect_kernel`, `shutdown_kernel` | `aureline-notebook` crate |
| `NotebookPairedExportPosture` | `paired_text_export_not_applicable`, `paired_text_export_derived_notebook_to_script`, `paired_text_export_derived_notebook_to_markdown` | `aureline-notebook` crate |

## Schema

The boundary schema lives at:

```
/schemas/notebook/implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state.schema.json
```

## Checked-in artifact

The typed header-kernel-bar packet is checked in at:

```
artifacts/notebook/m5/implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state.json
```

It is embedded in the `aureline-notebook` crate via `include_str!` so consumers and CI agree on every row without a cargo build in CI.

## Fixtures

Worked fixture cases live under:

```
fixtures/notebook/m5/implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state/
```

Cases cover:
- `local_kernel` — local managed-toolchain kernel with active local-host chip.
- `container_kernel` — local provisioned kernel inside a devcontainer with active container chip.
- `no_kernel` — no kernel selected, disconnected chip, select_kernel action only.
- `remote_kernel` — remote agent primary kernel with managed-workspace chip and paired script export.
- `ssh_remote_kernel` — provider-side remote kernel over SSH in reconnecting state with paired Markdown export.
- `degraded_remote` — compatibility-bridge remote kernel in degraded state with browser-bridge chip.
- `service_plane_kernel` — managed workspace agent kernel blocked by policy with service-plane chip.

## Integration

The `aureline-notebook` crate exposes:

- `ExecutionLocusChip`, `NotebookHeaderKernelBarState`, `NotebookHeaderKernelBarPacket`
- `current_notebook_header_kernel_bar_packet()`
- Validation methods on every record that return typed `HeaderKernelBarFinding` findings
- Closed vocabularies with `as_str()` tokens and `ALL` arrays

## Risks and downgrade behavior

- If the embedded JSON artifact is missing or malformed, `current_notebook_header_kernel_bar_packet()` returns an error and CI must treat the lane as underqualified.
- If a fixture case fails validation, the corpus is incomplete and promotion should narrow the claim.
- Remote kernel origins must always render the local-vs-remote boundary cue; hiding it is a trust violation.
- No-kernel states must expose `select_kernel` and must not expose `restart` or `interrupt`.
- `auto_rerun_forbidden` is an invariant; any state that sets it to `false` is non-conforming.
