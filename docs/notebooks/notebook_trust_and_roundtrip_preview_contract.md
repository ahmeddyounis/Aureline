# Notebook trust ladder, structured round-trip preview sheet, and irreversibility disclosure contract

This contract freezes the **trust ladder** and the **round-trip preview
sheet** that notebook and rich-structured-asset surfaces MUST render
**before** any lossy, privileged, or otherwise non-trivial mutation is
applied.

It exists so reviewers can tell, at preview time:

- which trust boundary applies (document vs kernel/runtime vs output vs widget);
- whether the change is lossless, metadata-lossy, representation-lossy, structurally lossy, or irreversible; and
- which safe next actions exist (compare-first, preserve raw, rerun/regenerate, refuse).

It is normative. A surface that edits notebooks, edits structured
artifacts through a structured form, or rewrites structured artifacts as
an incidental side effect (export, “fix”, “format”, “apply”, “repair”)
MUST emit the machine-readable record defined in the companion schema.

## Companion artifacts

- [`/schemas/notebooks/roundtrip_preview.schema.json`](../../schemas/notebooks/roundtrip_preview.schema.json)
  defines `structured_round_trip_preview_sheet_record`.
- [`/fixtures/notebooks/roundtrip_preview_cases/`](../../fixtures/notebooks/roundtrip_preview_cases/)
  contains worked YAML cases covering mixed-trust notebooks and common
  structured-artifact round-trip hazards (manifests, env files, lockfiles).

## Composition, not redefinition

This contract composes with:

- [`/docs/adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md`](../adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md)
  — canonical four-axis notebook trust posture and kernel transport
  vocabulary (document trust, kernel trust, output trust, widget trust).
- [`/docs/architecture/generated_artifact_safe_edit_policy.md`](../architecture/generated_artifact_safe_edit_policy.md)
  — generated/mirrored/imported artifact edit posture, rebuild intent,
  and structured-viewer fallback policy (lockfiles, manifests, outputs).
- [`/docs/ux/preview_apply_revert_contract.md`](../ux/preview_apply_revert_contract.md)
  — preview-first obligations, basis-drift rules, and undo/rollback
  honesty (no overclaiming reversibility).
- [`/docs/editor/decode_recovery_and_save_consequence_contract.md`](../editor/decode_recovery_and_save_consequence_contract.md)
  — raw-byte preservation postures and “block until reviewed” semantics.
- [`/docs/runtime/target_discovery_and_install_review_taxonomy.md`](../runtime/target_discovery_and_install_review_taxonomy.md)
  — taxonomy slots that quote `notebook_trust_rung` and
  `structured_round_trip_risk_packet_ref` on install-review and other
  cross-surface review packets.

When these sources disagree, the source contract wins and this document
MUST be updated in the same change.

## Part A: Trust ladder + four-axis trust (never one banner)

Notebook and rich-structured previews MUST keep trust visibly
multi-dimensional.

### A.1 Four-axis trust posture (re-export)

Notebook trust is **four independent axes** that MUST NOT be collapsed
into a single “trusted/untrusted” state:

1. **Document trust** — whether the notebook’s stored content is trusted
   to drive privileged actions (execution admission, widget binding).
2. **Kernel/runtime trust** — whether launching or attaching to a kernel
   is admissible for this context (local vs remote vs managed; policy
   ceilings).
3. **Output trust** — whether a specific output is live from the current
   session, captured evidence from a prior session, replayed, orphaned,
   or widget-gated.
4. **Widget trust** — whether widget live binding is denied, admitted
   after preview, suppressed by trust or policy, content-class blocked,
   or runtime unavailable.

These axes (and their enumerations) are owned by ADR-0022 and are
re-exported in the preview sheet record so that every preview can show
all four without inventing a surface-local vocabulary.

### A.2 Notebook trust ladder (`notebook_trust_rung`)

In addition to the four axes, notebook surfaces MUST render the
**notebook trust ladder** rung that answers: “how far up the trust
ladder is this notebook (or this set of cells) allowed to climb right
now?”

The rung exists so mixed-trust notebooks (imported public notebook,
browser-handoff-return notebook, AI-generated cells, partially reviewed
cells) stay reviewable **without** collapsing into one bit.

Frozen `notebook_trust_rung` tokens:

| Token | Definition |
|---|---|
| `untrusted_tainted` | Notebook or cell is untrusted/tainted (imported from an untrusted source, AI-authored without review, or returned from a browser-handoff surface). Outputs render behind a tainted fence. |
| `untrusted_quarantined_for_review` | Notebook is in a review workspace for evaluation. Cells are not executed; outputs are not re-rendered; only the reviewer’s preview renders. |
| `structural_only_trusted` | Notebook structure (cell graph, metadata, markdown) is trusted; code cells and outputs remain untrusted. Navigation/search/outline are allowed without executing code. |
| `selective_cell_trust` | Individual cells are trusted; siblings are not. Cell-level authority rides the rung; aggregation MUST follow the dependency-marker downgrade rule. |
| `fully_trusted_user` | User opted in to trust the whole notebook (workspace-trusted and user-confirmed). Activators that require trust may fire; outputs may re-render live. |
| `fully_trusted_workspace_policy` | Admin policy pins the notebook to trusted (curated workspace notebooks). Trust survives reopen; still narrowed by capability-lifecycle rules. |
| `trust_revoked_pending_review` | Notebook was trusted but a revocation event demoted it (policy change, signature mismatch, provenance drift). Outputs render with a revocation notice; user action required. |

Rules (frozen):

1. A surface MUST NOT render any `fully_trusted_*` rung without at least
   one evidence ref (signature, provenance anchor, explicit user grant,
   or policy pin).
2. The rung MUST NOT widen silently. Any upward move requires explicit
   user/admin action and must be auditable.
3. AI-authored and browser-handoff-return cells MUST enter at
   `untrusted_tainted`. A write path that lands at a higher rung is
   non-conforming.
4. A rung downgrade MUST cancel or suppress live execution and widget
   binding rather than trying to “keep running” under stale trust.

## Part B: Structured round-trip preview sheet

### B.1 What a preview sheet must answer

Before a surface applies any structured mutation, it MUST answer all of:

- **Authored source**: what is the canonical authored source being
  changed (and what is preserved as raw evidence)?
- **Effective preview**: what structured/resolved form is actually being
  shown (schema defaults, resolution, normalization, degradation)?
- **Round-trip risk**: what will be lost or rewritten if the structured
  form is committed?
- **Irreversibility disclosure**: can the change be reversed exactly; if
  not, what is the honest recovery posture (preserved raw copy, rerun,
  regeneration, checkpoint restore, or no recovery)?
- **Safe next actions**: what the user can do next that is safe
  (compare-first, preserve raw, rerun/regenerate, refuse).

These answers MUST be encoded in one
`structured_round_trip_preview_sheet_record` so notebooks, structured
editors, docs/help, support exports, and future automation can quote one
stable truth instead of inventing bespoke copy.

### B.2 Structured round-trip risk class (`structured_round_trip_risk_class`)

Frozen `structured_round_trip_risk_class` tokens:

| Token | Definition |
|---|---|
| `lossless_roundtrip` | Round-trip preserves every declared structural field and attachment. |
| `lossy_metadata_only` | Non-essential metadata (timestamps, author hints, display-only annotations, formatting comments) is dropped. |
| `lossy_output_representation` | Output representations (rendered images, MIME alternates, rich-text alt forms) are normalised. Cell inputs are preserved. |
| `lossy_structural` | Structural elements (cell order, grouping, attachments, linked ids) change. The surface MUST enumerate affected fields before allowing commit. |
| `lossy_irreversible` | Commit triggers an irreversible action (external side effect, remote mutation, signature re-sign). Typed confirmation is mandatory. |
| `round_trip_unavailable` | The surface cannot determine risk (renderer unreachable, schema mismatch, unsupported attachment class). Commit MUST be denied rather than guessed. |
| `round_trip_policy_blocked` | Admin policy blocks the round-trip. Commit MUST be denied with a typed policy disclosure. |

Acceptance mapping (must be obvious before apply):

- **Source-exact**: `lossless_roundtrip`.
- **Structurally safe**: `lossy_metadata_only` (no structural meaning
  changes; metadata/formatting loss is disclosed).
- **Lossy but reversible**: `lossy_output_representation` and
  `lossy_structural` *only when* a declared recovery path exists
  (preserved raw, rerun/regenerate, or checkpoint restore).
- **Irreversible without raw preservation**: any case where the preview
  cannot name a recovery path (including `lossy_irreversible`, or
  `lossy_structural` with missing raw preservation).

### B.3 Preview representation class (`preview_representation_class`)

The preview sheet MUST say what the reviewer actually saw:

- `full_fidelity_preview` — no normalization/degradation beyond stable
  rendering.
- `normalised_preview` — formatting/representation was normalized
  (sorting keys, canonicalizing whitespace, collapsing env duplicates,
  choosing one MIME alternative, etc).
- `summary_only_preview` — only a summary is shown (size, policy, or
  availability reasons).
- `tombstone_preview` — no previewable structure exists; the sheet is a
  typed refusal with safe next actions.

### B.4 Apply-gating rules (refuse / warn / compare-first / preserve raw)

The preview sheet’s `apply_gate_class` MUST follow this matrix:

| Risk class | Minimum gate | Notes |
|---|---|---|
| `lossless_roundtrip` | `allow_apply` | Compare-first is optional, but identity/basis drift rules still apply. |
| `lossy_metadata_only` | `warn_allow_apply` | Must disclose what metadata/formatting will not survive. |
| `lossy_output_representation` | `warn_allow_apply` | Must disclose what outputs/representations are normalized; MUST NOT claim outputs are “refreshed” without rerun. |
| `lossy_structural` | `require_compare_first_review` | Must enumerate `affected_fields`. Must preserve raw before mutation or deny. |
| `lossy_irreversible` | `require_typed_confirmation` | Must name `irreversibility_flag = true` and cite the irreversible disclosure flags. |
| `round_trip_unavailable` | `refuse_rewrite` | The safe path is inspect/compare-only; do not guess. |
| `round_trip_policy_blocked` | `refuse_rewrite` | Deny apply with typed policy disclosure; do not silently downgrade. |

Raw preservation rule:

- If the risk class is `lossy_structural` or `lossy_irreversible`, the
  surface MUST preserve an exact raw copy (buffer, journal, or detached
  artifact) before any mutation. If policy forbids raw capture, apply
  MUST be denied.

## Fixture coverage expectation

`/fixtures/notebooks/roundtrip_preview_cases/` includes worked cases for:

- notebook cells and mixed-trust rungs;
- JSON/YAML manifests and unknown/unsupported fields;
- `.env`/environment file normalization and duplicate-key hazards;
- lockfile structured-edit refusal vs regenerate-first posture;
- rich outputs, widgets, and downgraded widget trust states;
- mixed local/remote kernels with kernel-unavailable execution trust.

