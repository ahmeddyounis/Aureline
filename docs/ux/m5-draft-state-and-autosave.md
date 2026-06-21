# M5 draft state, autosave journals, and recover-draft semantics

One canonical, export-safe model for what happens *across an interruption* to a
mutation-capable surface: how its edits autosave to a **local draft journal**, how
it keeps **draft-versus-applied** state explicit, and how a **recover-draft**
action restores work after a crash, restart, reconnect, or missing-dependency
condition — without ever implying that local draft state was written to a remote
target, provider, or protected file.

Where [the field/control-row model](m5-field-control-rows.md) freezes a single
field's label, source, and validation anchor,
[the form-validation model](m5-form-validation-and-blocked-submit.md) freezes how
field validity rolls up into a blocked-submit reason, and
[the structured-input / staged-review model](m5-structured-input-and-staged-review.md)
freezes the per-surface honesty claim of a whole form, this model freezes the
truth a form must hold *between sessions*: a draft is recoverable, draft is never
confused with applied, the autosave indicator never overclaims its reach, and
recovery never destroys unrelated work.

- Schema: [`schemas/ux/m5-draft-state-and-autosave.schema.json`](../../schemas/ux/m5-draft-state-and-autosave.schema.json)
- Canonical support export: [`artifacts/ux/m5-draft-state-and-autosave/support_export.json`](../../artifacts/ux/m5-draft-state-and-autosave/support_export.json)
- Report: [`artifacts/ux/m5-draft-state-and-autosave/report.md`](../../artifacts/ux/m5-draft-state-and-autosave/report.md)
- Perturbation corpus: [`fixtures/ux/m5-draft-state-and-autosave/`](../../fixtures/ux/m5-draft-state-and-autosave/)
- Rust truth source: `crates/aureline-ui/src/m5_draft_state_and_autosave`
- Validator: `tools/release/draft_state_and_autosave.py`

## What a surface carries

Each `DraftJournalRecord` (`#/$defs/surface`) is one mutation-capable surface,
identified by `surface_id`, its `lane`, its `origin`, and its `claim_posture`,
plus the draft-state contract it must hold:

- **Autosave journal** (`journal`) — the `persistence_tier` the draft actually
  lives in (`unsaved_in_memory`, `local_journal`, `local_durable_checkpoint`,
  `committed_local`, `committed_remote`), the `autosave_status` indicator
  (`idle`, `saving`, `saved`, `failed`, `disabled`), and the
  `autosave_claim_scope` the indicator *claims* (`claims_local_only`,
  `claims_remote_synced`, `claims_none`). The indicator can never claim a
  remote/synced target while the draft only reached a local tier.
- **Draft-versus-applied state** (`draft_state`) — an explicit
  `draft_applied_state` (`draft_only`, `partially_applied`, `applied`,
  `not_distinguished`), whether draft and applied are `draft_distinct_from_applied`,
  the unsaved/applied/draft field counts, and whether an applied state has its
  `applied_target_named`. A draft-tier value never reads as `applied`, and an
  applied state always names where it went.
- **Recover-draft semantics** (`recovery`) — `availability` (`recoverable`,
  `recovered`, `no_journal`), the `interruption_kind` it survived (`crash`,
  `restart`, `reconnect`, `missing_dependency`, or `none`), whether a
  `recover_action_present` exists when a journal does, whether recovery
  `recover_preserves_unrelated_state`, whether the restore surface
  `enumerates_affected_surfaces`, and the guarantee that
  `recover_implies_remote_write` is false.
- **Submit gate** (`submit_gate`) — `submit_allowed` must be closed while draft and
  applied state are ambiguous, `draft_applied_disambiguated_before_submit` records
  that the disambiguation happened first, and `commit_action_is_specific` names the
  scope/effect rather than a generic Continue.
- **Backing freshness and proof** — `declared_freshness_state` with
  `freshness_state_visible` and `superseded_state_marked`, plus a `verification`
  proof, so a stale, superseded, or unproven backing source narrows the surface
  instead of reading as fresh.

`declared_blocked_fallback` declares the submit-control presentation a floored
surface drops to, `lineage` (including `structured_input_ref` and `journal_ref`)
attributes the surface to its structured-input surface and autosave journal, and
`renderings[]` lists the consumer surfaces and the claim each one shows.

## Effective claim

`DraftJournalRecord::narrow` re-derives a `DraftClaim` per surface so a surface can
never read wider than its evidence:

| Claim | Meaning |
| --- | --- |
| `draft_certified` | Full draft-versus-applied-honest, autosave-truthful, recoverable contract. A surface applied to a local target with a local-only indicator is certified; a remote-committed surface may honestly claim a remote sync. |
| `draft_narrowed` | A first-party surface held below certified by a labelled, recoverable gap (an unlabeled autosave indicator, an in-flight save, unsaved in-memory edits, stale/superseded backing, stale/missing proof); the draft stays recoverable and reopenable. |
| `draft_review_overlay` | A read-only review of an imported/migrated/restored state; attributable but never a local submit. |
| `draft_blocked` | The draft/autosave/recovery contract is broken; the surface falls back to an explicit blocked state that names the reason instead of a clean-but-false submit. |
| `draft_labs_not_claimed` | Labs/unadvertised; makes no public claim and is never widened. |

A higher-rank claim asserts more authority, so a narrowing or floor moves strictly
lower, and a rendering that shows wider than the effective claim is itself a floor
(`rendering_overclaims`).

### Floor reasons (drop to `draft_blocked`)

These break the draft-state contract outright: `autosave_overclaims_remote`,
`draft_applied_ambiguous`, `local_draft_reads_as_applied`,
`recover_implies_remote_write`, `submit_from_ambiguous_state`,
`recovery_deletes_unrelated_state`, `applied_target_unnamed`,
`recover_action_lost`, `affected_surfaces_unenumerable`,
`imported_draft_reads_as_applied`, `rendering_overclaims`, and
`journal_backing_missing`. A floored surface keeps its `declared_blocked_fallback`
(`shows_reason_on_submit` or `disabled_with_hint`) rather than a misleading clean
submit.

### Narrowing reasons (hold at `draft_narrowed`, stay usable)

`autosave_state_unlabeled`, `autosave_pending`, `draft_unsaved_pending`,
`freshness_unlabeled`, `superseded_state_not_marked`, `draft_stale`,
`verification_proof_stale`, `verification_proof_missing`, and `reopen_path_lost`.
On an imported/restore review overlay, any non-floor gap drops the surface below
the overlay because the overlay is already the minimal honest claim.

## Guardrails enforced by the validator

`M5DraftStateSetPacket::validate` (Rust) and
`tools/release/draft_state_and_autosave.py validate` (the CI gate) both refuse a
packet that:

- lets an autosave indicator claim a remote/synced target while only local draft
  state was saved;
- leaves draft and applied state ambiguous, or lets a local (draft-tier) value
  read as applied, or leaves an applied state without a named target;
- lets a recover-draft action imply a remote write, delete unrelated
  workspace/profile state, or go missing while a journal exists, or leaves a
  crash-recovery surface unable to enumerate the affected forms/sheets;
- lets a submit proceed from an ambiguous draft/applied state, lets an
  imported/restore review read as a local submit, or lets a rendering overclaim;
- floors a surface to a silent submit control with no reason/hint;
- fails to represent every form lane, persistence tier, recovery availability,
  interruption kind, autosave claim scope, or consumer render surface, or contains
  no surface that demonstrates the auto-narrowing rule;
- leaks raw credential/secret material across the export boundary.

## Draft is not applied, and local is not remote

The model keeps two axes explicit and independent. `persistence_tier` records
*where the edits are journaled* — and `is_local_only` is true for every tier
except `committed_remote`, so the autosave indicator can never claim a remote sync
for a local draft. `draft_applied_state` records *how much has reached the target*
— and a draft tier may never be labelled fully `applied`, while any applied or
partially-applied state must name its target. Recovery restores the draft journal
without ever implying the draft was committed: `recover_implies_remote_write` must
stay false, and recovery must preserve unrelated state and enumerate exactly the
surfaces an interruption affected.

## Regenerating the artifacts

```bash
# Canonical support export + report (Rust seed is the source of truth)
cargo run -p aureline-ui --example dump_m5_draft_state_and_autosave \
  > artifacts/ux/m5-draft-state-and-autosave/support_export.json
cargo run -p aureline-ui --example dump_m5_draft_state_and_autosave report \
  > artifacts/ux/m5-draft-state-and-autosave/report.md

# Perturbation corpus
python3 tools/release/draft_state_and_autosave.py emit-corpus

# Verify everything (schema, re-derivation, corpus)
python3 tools/release/draft_state_and_autosave.py self-test
cargo test -p aureline-ui m5_draft_state
```

The Rust seed builder, the checked-in support export, and the Python re-derivation
are kept byte-aligned: a Rust test asserts the checked-in export equals the
in-crate builder, and the Python `self-test` re-derives every surface and corpus
case so the artifacts can never imply a wider claim than the current evidence
backs.
