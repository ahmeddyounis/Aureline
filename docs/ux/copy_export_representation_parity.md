# Copy/preview/export representation parity contract

Copy, preview, and export actions often touch the same underlying bytes
through different surfaces (editor, diff, docs preview, rich preview,
notebook output, output/log viewers, install/review prompts, and support
export). This contract prevents those surfaces from drifting into
incompatible or misleading semantics (e.g. “Copy” meaning raw source in
one place, rendered text in another, and a sanitized snapshot elsewhere).

This contract is normative. Where it disagrees with upstream contracts
it cites, the upstream contract wins and this document MUST be corrected
in the same change.

Machine-readable companions:

- [`/schemas/ux/representation_copy_export.schema.json`](../../schemas/ux/representation_copy_export.schema.json)
  — boundary schema for worked parity cases and copy/export metadata
  records.
- [`/fixtures/ux/representation_copy_export_cases/`](../../fixtures/ux/representation_copy_export_cases/)
  — worked cases spanning suspicious text, raw/rendered divergence,
  sanitized preview, oversized/truncated output, binary excerpts, and
  generated artifacts.

This contract composes with (and does not replace):

- [`/docs/security/safe_preview_trust_classes.md`](../security/safe_preview_trust_classes.md)
  and [`/schemas/security/trust_class.schema.json`](../../schemas/security/trust_class.schema.json)
  for the four trust classes and their allowed behaviors.
- [`/docs/security/suspicious_content_packet.md`](../security/suspicious_content_packet.md)
  for detector outcomes and cross-surface suspicious-content evidence.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  and [`/schemas/ux/interaction_safety.schema.json`](../../schemas/ux/interaction_safety.schema.json)
  for the shell-wide representation-class vocabulary and the
  representation-labeled copy/export audit record.
- [`/docs/ux/clipboard_history_contract.md`](./clipboard_history_contract.md)
  for named copy variants and clipboard-specific truthfulness posture.
- [`/docs/ux/output_log_viewer_contract.md`](./output_log_viewer_contract.md)
  plus [`/schemas/ux/live_set_state.schema.json`](../../schemas/ux/live_set_state.schema.json)
  for copy/export scope truth and windowing/truncation disclosure on
  streamed or oversized output.
- [`/docs/notebooks/output_viewer_truth_contract.md`](../notebooks/output_viewer_truth_contract.md)
  and [`/schemas/notebooks/output_include_policy.schema.json`](../../schemas/notebooks/output_include_policy.schema.json)
  for notebook output inclusion, omission, truncation, and support-capture
  policy.
- [`/docs/accessibility/locale_fallback_and_copy_representation_contract.md`](../accessibility/locale_fallback_and_copy_representation_contract.md)
  for representation labels and peer actions staying reachable under
  dense layouts, high zoom, keyboard, and assistive technology.

## Scope

This contract applies anywhere the user can:

- preview content that might be unsafe, ambiguous, or heavy;
- copy content to the clipboard; or
- export content to files, bundles, issue reports, or support artifacts.

The goal is not “every surface supports every representation”. The goal
is **parity**: if two surfaces expose the same content class, they MUST
use the same representation vocabulary and MUST disclose the same
transform semantics so reviewers can compare them mechanically.

## Definitions

### Trust class (safe preview)

Trust class names the **render/execution posture** of the surface (e.g.
`RawText` vs `SanitizedRich`). Trust class is not a copy label.

The trust-class vocabulary is owned by the safe-preview contract and is
not redefined here.

### Representation action id (transfer action)

Representation action ids name **what is leaving the product**:
`copy_raw`, `copy_rendered`, `copy_escaped`, `export_sanitized_snapshot`,
and `export_metadata_only`.

Generic unlabeled “Copy” / “Export” is non-conforming when multiple
representations exist and differ materially.

### Representation class (representation label)

Representation class names the **representation of the transferred
payload** (or, for on-screen state, the currently rendered posture).

This contract uses the shell-wide spellings:

- `raw` — exact source bytes/text representation.
- `rendered` — the current rendered view (preview text, rich rendering,
  or viewer rendering), explicitly not source-identical.
- `escaped` — source representation made safe for inspection/paste by
  escaping controls/metacharacters.
- `sanitized` — an inert snapshot with active/scriptable content removed
  or disabled.
- `sandboxed` — on-screen rendered content confined to an isolation
  boundary (not a transfer output).
- `generated` — on-screen model-produced content (transfer still uses a
  representation-labeled action; quoted authoritative content requires
  citation anchors).
- `blocked_metadata_only` — body withheld entirely; only typed metadata
  leaves.

### Transform semantics

When bytes shown or transferred are not source-identical, the surface
MUST disclose **which transforms occurred**. Common transforms include:

- truncation/windowing (“showing last N”, “visible rows only”);
- sanitization (removal/disablement of active content);
- escaping (control/metacharacter escaping for safe inspection);
- decoding replacement (malformed bytes replaced with U+FFFD or similar);
- normalization (newline normalization, whitespace folding);
- excerpting (binary/text excerpt, structured excerpt, sampled rows);
- redaction (secret/PII removal).

Transforms are not “implementation details” when they change what a
reviewer can infer from copied/exported text.

## Contract rules

### 1) Parity vocabulary is shared across surfaces

Every surface that offers preview/copy/export MUST speak in the shared
vocabulary:

- trust class (render posture);
- representation action id (what leaves);
- representation class (what the payload materially is);
- scope truth (what subset is included when windowed/buffered); and
- transform semantics (what changed vs source).

If a surface cannot express a distinction (e.g. it cannot tell whether
output is truncated), it MUST narrow to `export_metadata_only` /
`blocked_metadata_only` rather than overclaim fidelity.

### 1.1) Open behaviors are representation-bearing too

“Open” actions (open in detail, open raw view, open rendered preview,
open in system browser, open downloaded bytes, open static snapshot)
MUST follow the same representation vocabulary:

- the open target MUST disclose whether it is raw source, rendered view,
  escaped inspection view, sanitized snapshot, sandboxed/isolated view,
  or metadata-only.
- any downgrade (e.g. isolate → sanitize → metadata-only) MUST be
  visible and MUST not keep a stronger label than it can still justify.

### 2) Copy/export MUST not imply source identity when it is not true

Whenever `representation_class != raw`, the surface MUST:

- label the action and the resulting payload with the representation
  used; and
- disclose transforms that make it non-source-identical (sanitized,
  escaped, truncated, normalized, excerpted, decoded with replacement).

If a surface shows a rendered preview and allows copying it, it MUST
also keep a path to copy raw source when raw source exists and is safe
to transfer.

### 3) Oversized and buffered output has scope truth, not “best effort”

When content is oversized, streamed, buffered, or virtualized:

- copy/export MUST name the scope (`visible_rows_or_events`,
  `loaded_materialized_set`, `named_snapshot_only`, `metadata_only`);
- copy/export MUST disclose whether buffered/unseen content is excluded
  until the user explicitly reveals it; and
- truncation/windowing MUST be explicit (no silent “last N”).

### 4) Binary content never pretends to be text

If the subject is binary (or includes binary segments), a surface MUST
not pretend that a text excerpt is “the file”.

Conforming paths include:

- `export_metadata_only` by default on support/export boundaries;
- explicit “Copy text excerpt” / “Copy escaped excerpt” actions whose
  metadata states `excerpted` and the excerpt bounds; and
- explicit “Export raw bytes” only when policy and trust posture permit
  it (never as the default when the surface cannot verify safety).

### 5) Generated content stays attributable and honesty-preserving

When content is model-produced:

- on-screen state MUST remain representation-labeled (`generated`) per
  shell interaction-safety rules;
- any transfer that quotes authoritative material MUST carry citation
  anchors; and
- copy/export MUST not re-label generated text as raw source.

### 6) Support export is sanitized or metadata-only by default

Support-export surfaces MUST default to `export_sanitized_snapshot` or
`export_metadata_only` and MUST never export active content as active.
If raw bodies are excluded by policy, the export MUST still carry the
typed omission reason and a stable identity handle so support can reason
about absence without inventing bytes.

## Surface mapping (frozen)

The table below is a parity-audit map: a reviewer SHOULD be able to
compare two surfaces and see whether they are exposing the same
representation class and transform semantics for the same content class.

| Surface family | Typical trust class | Default copy posture | Default export posture | Notes |
|---|---|---|---|---|
| editor / raw diff | `RawText` | `copy_raw` primary; `copy_escaped` required when suspicious content is present | export posture is surface-dependent; support/export boundaries narrow to `export_metadata_only` | never silently substitute rendered/sanitized output for raw selection |
| docs/help preview | `SanitizedRich` | `copy_rendered` primary; `copy_raw` and `copy_escaped` stay reachable when source exists or divergence/suspicious content is present | `export_sanitized_snapshot` | rendered copy must not imply it is raw Markdown/source |
| install / package review | `RawText` (strict display mode) | `copy_raw` primary for identifiers; `copy_escaped` for confusables/invisibles | `export_metadata_only` or `export_sanitized_snapshot` | active rendering of untrusted package bodies never becomes the default |
| rich preview / embedded webview | `SanitizedRich` by default; `IsolatedRemoteActive` only while guarantees hold | `copy_rendered` allowed only while verified; raw/escaped available only when the source contract provides it | `export_sanitized_snapshot` or `export_metadata_only` | origin/identity drift, disconnect, or sandbox loss forces downgrade to static snapshot or metadata-only |
| notebook rich output / viewers | `SanitizedRich` by default | per notebook include-policy: copy/export actions MUST be representation-labeled and scope-true | per notebook include-policy, often `export_sanitized_snapshot` or metadata-only | omission/truncation/redaction are first-class and must be reflected in include-policy records |
| output/log viewers | depends on origin + active-content policy | copy/export MUST cite scope truth (visible/loaded/snapshot/metadata-only) and representation label | export MUST not overclaim full-body when truncated/windowed | oversized/buffered output must not silently change what copy/export includes |
| support export | `SanitizedRich` or metadata-only | representation-labeled only; defaults narrow | `export_sanitized_snapshot` or `export_metadata_only` by default | raw bodies and raw URLs are excluded by default; omission reasons must remain explicit |

## Required copy/export metadata (frozen)

Whenever bytes leave the product (clipboard copy, export file, support
bundle item, or evidence packet excerpt), the exporting surface MUST be
able to produce a structured metadata record that answers:

- **Representation used**: action id + representation class.
- **Transforms applied**: sanitized/escaped/truncated/normalized/
  decoded-with-replacement/excerpted/redacted (as applicable).
- **Trust posture**: trust class of the source surface at the time of
  transfer.
- **Scope truth**: whether the payload is a full body, a windowed slice,
  a snapshot-only capture, or metadata-only.
- **Hidden/omitted segments**: whether any content was omitted (buffered
  unseen content, truncated tail/head, policy-omitted rows/fields,
  redacted segments) and why.
- **Share safety**: whether the payload is safe to paste into an issue
  report and/or safe to embed in a support bundle under the default
  redaction posture.

The schema in `schemas/ux/representation_copy_export.schema.json`
freezes the required fields for these metadata records.
