# M5 clipboard contracts: plain-text-default copy, copy-with-context variants, and sensitive-copy labels

This contract makes clipboard behavior an explicit, reviewable product surface
across the new Milestone 5 artifact surfaces — editor, notebook, data/API,
preview, docs, review, runtime, and companion-adjacent panes — instead of ad hoc
per-surface copy. It binds the clipboard half of the frozen
[keyboard-continuity matrix](./freeze-the-m5-keyboard-mode-modal-sequence-clipboard-route-drag-drop-verb-and-grouped-hist.md):
*copy/export defaults preserve useful plain text and sensitive-copy warnings*.

The canonical packet is built by
`aureline_shell::implement_clipboard_contracts_with_plain_text_default_copy_with_context_variants_sensitive`.
Its boundary schema is
[`schemas/interaction/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive.schema.json`](../../../schemas/interaction/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive.schema.json),
the checked support export is
[`artifacts/interaction/m5/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive/support_export.json`](../../../artifacts/interaction/m5/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive/support_export.json),
and the protected fixtures live under
[`fixtures/interaction/m5/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive/`](../../../fixtures/interaction/m5/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive/).

## What one record binds

A `ClipboardContractRecord` binds one claimed M5 surface (keyed by a
`KeyboardSurfaceKind` and a non-display `KeyboardSurfaceSubject`) to one copy flow:

- the **copied object** (`CopyObjectRef`): a `CopyObjectClass`
  (`source_code_fragment`, `relative_path`, `permalink`, `command_id`,
  `diagnostic_detail`, `artifact_evidence_ref`, `support_link`) plus an opaque or
  workspace-relative `object_token` — never an absolute private path;
- the ordered exact **representation set** (`CopyRepresentation`), each keyed by a
  `CopyFlavorClass`. The set is preserved rather than collapsed into a single
  pretty rich blob, and always carries at least one plain-text flavor unless the
  copy is rejected;
- the content **sensitivity** (`CopySensitivityClass`): `non_sensitive`,
  `access_token_material`, `fingerprint_digest`, `private_absolute_path`, or
  `support_session_link`;
- a reopenable **verification proof** (`AxisVerification`); and
- the resolved **copy variant** (`CopyResolutionClass`).

## Copy variants and the safety floor

`CopyResolutionClass` is the canonical copy-variant vocabulary that product, help,
migration, and support all name:

| Resolution | Rank | Meaning |
| --- | --- | --- |
| `plain_text_default_copy` | 0 | Plain text is the default representation, pushed without extra labeling. |
| `copy_with_context_variant` | 1 | An explicit copy-with-context variant (permalink with repo + ref, diagnostic with detail, code with `file:line`, or a rich representation with a plain-text fallback) is exposed alongside plain text. |
| `sensitive_labeled_before_copy` | 2 | Sensitive content is visibly labeled / previewed before it reaches the clipboard. |
| `relativized_or_redacted_copy` | 3 | A private absolute path is relativized or a secret is redacted before copy. |
| `rejected_rich_only_or_unsafe` | 4 | A rich-only or otherwise unsafe copy is rejected. |

Only `plain_text_default_copy` pushes silently. A copy is held off that silent
lane whenever a `CopyContractTrigger` fires, and each trigger imposes a minimum
resolution rank (the **floor**):

| Trigger | Floor |
| --- | --- |
| `context_beyond_bare_object` | `copy_with_context_variant` |
| `stale_or_missing_copy_proof` | `copy_with_context_variant` |
| `sensitive_token_or_fingerprint` | `sensitive_labeled_before_copy` |
| `support_link_present` | `sensitive_labeled_before_copy` |
| `private_absolute_path` | `relativized_or_redacted_copy` |
| `rich_only_no_plain_text` | `rejected_rich_only_or_unsafe` |

The recorded trigger set must equal the set computed from the record's context,
sensitivity, representations, and proof, and the resolution must meet the floor.
The resolution must carry exactly the detail field it requires — a
`context_label`, `sensitive_label`, `transform_note`, or `rejection_reason_label`
— and that label may not be a generic non-answer (`error`, `sensitive`,
`rejected`, …).

## Invariants enforced by `validate`

- **Plain text by default.** Every non-rejected record preserves at least one
  plain-text representation; pretty rich text never becomes the only readable
  output. A representation set with no plain-text flavor fires
  `rich_only_no_plain_text`, raising the floor to a reject.
- **Sensitive copy is labeled, never silent.** Token / fingerprint / support-link
  / private-path content is labeled, relativized, or redacted before the
  clipboard. A silent sensitive push is rejected.
- **Representations are preserved.** The exact flavor set is kept; collapsing it
  into an opaque rich blob is rejected.
- **Provider copies never read as local.** A provider-linked or imported surface
  carries imported proof and never reads as a locally verified copy.
- **No raw boundary material.** Raw clipboard byte buffers, raw secret material,
  raw provider payloads, file contents, and absolute private paths never cross
  this boundary.

## Coverage the packet must prove

The seeded packet represents the six core artifact surfaces (`editor_core`,
`notebook_surface`, `data_api_surface`, `preview_surface`, `docs_surface`,
`review_surface`), the five parity-critical object classes (`relative_path`,
`permalink`, `command_id`, `diagnostic_detail`, `artifact_evidence_ref`), every
resolution class, at least one clean silent-default baseline, at least one copy
forced off the silent lane, at least one sensitive-labeled copy, and at least one
provider-linked / imported copy.

## Consumers

The packet is the canonical source so product, help / migration guidance, support
export, and release-control surfaces name the same copy variants and route classes
the product actually exposes — rather than re-deriving copy semantics per surface.
