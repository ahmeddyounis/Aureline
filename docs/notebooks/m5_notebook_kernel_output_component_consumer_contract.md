# M5 notebook-kernel-output component consumer contract (M05-1090)

This is the consumer-adoption contract for the frozen M5 notebook-kernel-output
component matrix (M05-1084) and its four B129 implement lanes (M05-1085 through
M05-1088). It proves that the eight reusable notebook document / kernel / output
component families are adopted as **primitives** across the claimed M5 notebook
and data surfaces, rather than being reinvented as per-feature notebook chrome.

- **Crate:** `aureline-notebook`
- **Module:**
  `wire_editor_diff_review_debug_ai_support_and_export_consumers_so_notebook_document_kernel_and_output_components_keep_one_vocabulary_across_claimed_m5_notebook_and_data_surfaces`
- **Schema:** `schemas/ui/m5-notebook-kernel-output-component-consumer.schema.json`
- **Release proof:**
  `artifacts/release/m5-notebook-kernel-output-component-consumer-proof/`
- **Fixtures:** `fixtures/ui/m5-notebook-kernel-output-component-consumers/`

## Component families and controls lanes

The eight frozen families group into the four B129 controls contracts. Every
consumer must point back to the one canonical family (its per-family matrix
schema) and the one canonical controls lane, never a feature-local clone.

| Controls lane | Component families |
| --- | --- |
| `document_kernel` | notebook-document header, kernel-state strip |
| `kernel_choice` | kernel-picker row, kernel-origin pill |
| `output_trust` | output-trust banner, output-provenance chip group |
| `restart_recovery` | restart-consequence card, kernel-recovery card |

## Consumer classes

Six claimed M5 notebook / data consumer classes each adopt at least one canonical
family:

1. **notebook editor** — the first claimed editor consumer (AC1 anchor).
2. **diff / review**
3. **debug**
4. **AI context**
5. **CLI / headless**
6. **support / export + release packet** (support packet + release evidence; AC2).

Future notebook / data surfaces must register a row against this shared component
matrix before claiming parity: a new surface is audited against the one shared
component registry rather than a bespoke translation table.

## Preserved truth pillars

Every consumer — even a read-only, inspect-only, export-only, or support replay —
keeps the identical controlled labels and the identical frozen kernel/output
disposition vocabulary:

- `document_identity`
- `kernel_state`
- `kernel_selection`
- `kernel_origin`
- `output_trust`
- `output_provenance`
- `restart_consequence`
- `recovery_continuity`

A narrower consumer discloses the reduction with a reduced-capability banner (and,
when it punts to another surface, a desktop / kernel-manager / browser /
support-packet note) rather than renaming or dropping governed notebook truth.

## Guardrails (must all stay false per row)

1. `recovery_card_implies_rerun` — a reused kernel-recovery card must never imply
   that outputs silently rerun on reconnect, restart-clean, or
   choose-another-kernel recovery.
2. `presents_stale_output_as_live` — a stale or captured output is never rendered
   as live truth.
3. `hides_trust_class_behind_hover_only` — the raw / sanitized / active output
   trust class is never buried behind a hover-only affordance.
4. `collapses_kernel_origins_into_one_badge` — local, SSH, container, managed, and
   browser-bridge kernels are never collapsed into one unlabeled badge.

## Acceptance criteria

- **AC1** — the first claimed consumers all render the same notebook, kernel,
  output-trust, and recovery language (one vocabulary, one component family).
- **AC2** — support / export and release artifacts no longer need feature-local
  translation tables for notebook / kernel / output state.
- New notebook / data consumers can be audited against one shared component
  registry.

## Metadata-only boundary

The packet carries only typed class tokens, opaque notebook-state refs, booleans,
and redacted labels. Raw kernel connection strings, credential material, and
bearer secrets never cross this boundary.

## Regenerating the proof

```
GEN_NOTEBOOK_KERNEL_OUTPUT_CONSUMER_ARTIFACTS=1 \
  cargo test -p aureline-notebook generate_artifacts
```
