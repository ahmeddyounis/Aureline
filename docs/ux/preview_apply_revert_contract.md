# Preview/apply/revert lifecycle, checkpoint expectation, and rollback/undo honesty contract

This document freezes how Aureline names and stages **reversible change**
so that **preview**, **apply**, **undo**, **rollback**, and **checkpoint**
semantics remain truthful and comparable across subsystems.

This is not a domain-specific diff engine spec. It defines the shared
vocabulary, invariants, and minimum disclosure axes that every
consequence-bearing change surface MUST honour.

Where this contract disagrees with the PRD, architecture document,
technical design, UI/UX spec, design-system style guide, or an existing
frozen schema, those sources win and this contract plus its schema(s)
and fixtures update in the same change.

## Machine-readable schemas

- `/schemas/ux/interaction_safety.schema.json`
  - owns `preview_apply_revert_phase` and `revert_class`;
  - owns `preview_apply_revert_record` (per-phase lineage record).
- `/schemas/ux/staged_review_state.schema.json`
  - owns `apply_timing_class` for **immediate/live**, **staged**, and
    **preview-first** form flows.
- `/schemas/ux/batch_action_review.schema.json`
  - owns the batch-action `review_requirement_class` and the required
    pre-commit included/excluded/blocked/hidden/not-loaded counts.
- `/schemas/ux/review_decision.schema.json`
  - owns the shared “review decision” packet emitted when a user (or
    policy-managed actor) admits, cancels, defers, or re-reviews a
    proposed change.

Worked fixtures live at:

- `/fixtures/ux/preview_apply_cases/`

## Companion contracts (authoritative inputs)

This contract consumes (does not re-mint) existing vocabulary:

- `/docs/ux/shell_interaction_safety_contract.md` — consequence class,
  phase, revert class, basis-drift invalidation, and required-visible
  fields for consequential interactions.
- `/docs/reliability/local_history_contract.md` — checkpoint group
  expectations, group identity, and local-history truth-source
  distinctions.
- `/docs/workspace/mutation_lineage_model.md` and
  `/schemas/workspace/mutation_journal.schema.json` — mutation/group
  identity and reversal-class attribution.
- `/docs/ux/selection_and_batch_action_contract.md` — batch review-sheet
  requirements and forbidden count collapses.
- `/docs/ux/forms_validation_contract.md` — staged-review form semantics.
- `/docs/verification/source_fidelity_and_undo_packet.md` — save/restore
  recovery-class honesty rules (no “Undo” overclaiming).

## Terminology (frozen meanings)

### Preview-first

**Preview-first** means the surface MUST produce a reviewable preview
manifest and (when applicable) a diff before **any durable mutation**
lands, and MUST require an explicit admit action before commit.

Preview-first is mandatory for:

- multi-file mutations;
- provider-backed mutations with server-authoritative truth;
- destructive mutations;
- share/export/publish actions that move bytes across a boundary;
- AI-assisted or machine-generated patches when they touch user code or
  workspace durable state.

### Immediate / live-apply

**Immediate/live-apply** means the value applies as the user changes it
without a separate “Apply” step. Live-apply is admitted only when the
surface can truthfully provide:

- a visible “live change” cue;
- an honest recovery affordance (`revert_class`) for the resulting state
  before the user navigates away;
- stable identity of what was changed (the target does not silently
  rebind).

Live-apply MUST NOT be used for provider-backed, destructive, share/
export, or multi-file mutations. Those MUST be staged or preview-first.

### Staged-apply

**Staged-apply** means the user edits a draft state that is not yet
committed. The surface MUST expose:

- which values are live vs staged;
- what validation is current vs stale/skipped;
- the apply timing class (`staged_apply_required` vs
  `preview_first_apply_required`);
- the same rollback/undo honesty rules as preview-first flows.

### Revert vs rollback vs undo (honesty contract)

User-facing recovery MUST map to the frozen `revert_class` vocabulary:

- `exact_undo` — the system can restore the prior state byte-for-byte
  for the declared scope.
- `compensating_action` — the system can attempt a bounded compensating
  action, but it may not exactly invert the original effect.
- `regenerate_from_source` — recovery is by regenerating/recomputing
  from canonical inputs (not by inverting bytes).
- `restore_from_checkpoint` — recovery is by restoring a minted
  checkpoint/snapshot group.
- `evidence_only_no_rerun` — recovery is evidence/inspection only;
  rerunning is explicitly not claimed.
- `no_recovery_available` — admitted only when the consequence class is
  `irreversible_high_blast`.

Rules (frozen):

1. **“Undo” is forbidden as a generic label** unless `revert_class =
   exact_undo`. Every other class MUST name the class in the UI copy
   (e.g. “Rollback (compensating)”, “Regenerate”, “Restore checkpoint”,
   “View evidence”).
2. A surface MUST NOT overclaim exactness. If drift, policy, provider
   state, or a missing basis makes recovery inexact, the surface MUST
   downgrade the advertised recovery claim **before** commit rather than
   attempting a hidden best-effort inverse.
3. A surface MUST NOT collapse “revert”, “restore”, “regenerate”, and
   “evidence-only” into one ambiguous affordance.

## Diff-scope identity and basis-drift rules

Preview and apply MUST preserve the same scope identity:

- the same basis snapshot (or basis query) used to compute scope;
- the same target identity set (not “whatever matches now”);
- the same group identity for multi-target operations.

Apply MUST NOT silently widen or materially change scope after preview.
If the basis drifts, the surface MUST invalidate the preview and require
re-review; it MUST NOT perform a hidden best-effort apply.

## Checkpoint expectation (when ordinary undo is insufficient)

Before commit, a surface MUST mint or bind a checkpoint when:

- the operation is multi-file;
- the operation has a compensating or regenerate-only recovery claim;
- the operation crosses a provider/remote boundary with partial-failure
  risk;
- the operation is destructive or has non-trivial side effects (package
  install/update, scaffolding write/overwrite, migration/import, repair/
  reset).

Checkpoint-backed flows MUST disclose:

- checkpoint scope (what is covered vs explicitly out-of-scope);
- checkpoint durability/retention posture (survives restart, policy
  retention class, exportability);
- the recovery affordance the user will see after apply.

## Consequence summary, blocked/skip truth, and post-apply evidence links

Every review sheet or preview-first surface MUST make the following
reviewable on one surface/packet:

- **Consequence summary** (what will change, what is risky, what cannot
  be undone exactly).
- **Blocked vs unavailable vs skipped truth**:
  - blocked items are pre-commit ineligible and remain reviewable;
  - unavailable items are absent from evaluation (provider offline,
    redaction, unsupported filter);
  - skipped items are post-commit no-ops.
  These MUST NOT be collapsed into one “unavailable” bucket.
- **Evidence links after apply**, at least one of:
  - mutation journal entry/group id;
  - local-history checkpoint/group;
  - durable job row;
  - history row / evidence packet;
  - provider audit record ref (for provider-backed actions).

## When a review sheet is mandatory

This contract reuses the batch-action rule and extends it beyond
collections:

Review sheets (or dedicated review surfaces) are mandatory for:

- destructive mutations;
- provider-backed or remote-authoritative mutations;
- share/export/publish actions;
- multi-file workspace mutations (rename, bulk replace, refactor apply);
- AI patch apply against code/durable workspace state;
- package install/update/remove and bundle changes that write durable
  state;
- scaffolding/generation that writes/overwrites files;
- migrations/imports/restores/resets that alter durable state outside a
  single local edit buffer.

Lightweight inline preview is sufficient only when:

- scope is a single local target with stable identity;
- the change is reversible with `exact_undo`;
- no provider/remote boundary is crossed; and
- no destructive or export/share side effect occurs.

## Flow-specific minimum expectations (non-exhaustive)

The following flows MUST all honour the same preview/apply/revert
lineage and honesty rules:

- **AI patch review.** Preview MUST show target identity set, diff, any
  blocked/protected paths, and `revert_class`. Apply MUST bind the
  approved preview id and emit evidence links (mutation journal + local
  history checkpoint or declared alternative).
- **Bulk replace.** Preview MUST disclose match counts, excluded scopes,
  blocked/protected/generated-path counts, and grouped identity.
  Multi-file replace MUST create a checkpoint-backed recovery path.
- **Package install/update.** Preview MUST disclose which packages and
  durable artifacts change (locks/config), side effects (post-install
  scripts, tool resolution), and the honest recovery class (uninstall,
  restore checkpoint, or compensating).
- **Scaffold generation.** Preview MUST disclose create vs overwrite,
  co-resident user data risk, and recovery posture (checkpoint restore,
  delete-generated compensation, or manual recovery).
- **Rename/refactor apply.** Preview MUST preserve object identity
  mapping (old→new), disclose blocked/protected items, and keep the same
  group id through apply/validate/revert.
- **Migration/import.** Preview MUST disclose lossy mapping, excluded
  machine-local state, and checkpoint/rollback posture before any durable
  promotion.
- **Recovery restore/reset.** Review MUST disclose what is preserved vs
  cleared, whether recovery is exact vs compensating, and must never
  describe a destructive reset as a lossless “Undo”.

## Worked fixtures

`/fixtures/ux/preview_apply_cases/` carries worked `review_decision`
records that exercise:

- AI patch review (checkpoint-backed restore);
- workspace bulk replace (multi-file group identity preserved);
- package install/update (compensating rollback claim);
- scaffold generation (overwrite risk + checkpoint requirement);
- restore/reset (evidence-only or checkpoint restore, never a generic
  “Undo”).

