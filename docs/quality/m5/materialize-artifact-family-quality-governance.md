# Artifact-family quality governance

This document describes the canonical artifact-family quality-governance packet
used by Aureline's quality, review, support-export, and release surfaces. It is
the user-facing companion to the governed artifact at
`artifacts/quality/m5/materialize-artifact-family-quality-governance.json` and
the typed model in the `aureline-runtime` crate
(`materialize_artifact_family_quality_governance`).

## Why this packet exists

The stable quality lane already resolves one effective profile, orders save
participants, classifies fix safety, and counts release-visible debt for source
files (see `crate::quality` and the effective-profile / save-participant
governance packet). This packet **extends that exact vocabulary** to the
code-adjacent artifact families that the source-file lane did not previously
cover end to end:

- **`notebook`** — notebook documents and their cell/companion exports.
- **`request_file`** — request files such as `.http` / `.rest` collections.
- **`scaffolded_output`** — outputs emitted by templates or new-project flows.
- **`framework_generator`** — generator outputs (routes, migrations, clients).
- **`data_adjacent_artifact`** — fixtures, schemas, and config bundles.

It does **not** invent a parallel rule set. It reuses the stable quality
vocabulary (profile source layers, save-participant phases, fix-safety classes,
preview/apply posture, mutation scope, and release-debt states) so the new
families read the same way everywhere.

## What each family materializes

For every artifact family, the packet answers four questions a user can read
back directly:

1. **Which quality profile won?** A profile-resolution row names the effective
   profile, the winning source ref, the winning **source layer** and **source
   state**, whether **imported tool config** was mapped (and how many keys were
   left unmapped), and whether **policy overrode** lower-layer keys.
2. **Which save participants ran, and in what order?** An ordered participant
   list, each carrying its **phase**, **action**, **fix-safety class**, preview
   requirement, apply posture, and mutation scope. The list is stored in
   execution order (by phase, then phase order).
3. **Which actions require preview?** Every participant declares whether it can
   `auto_apply`, must `preview_first`, or is `apply_blocked`.
4. **Which debt is suppressed, baselined, waived, or newly introduced?** Per
   family debt rows keyed to a debt-state class and linked to the governed
   suppression or baseline record that backs them.

## Suppressed is distinct from baselined

`suppressed` and `baselined` never collapse into the same record:

- A **suppressed** or **waived** debt row references a governed **suppression**
  record (`suppression_ref`) and carries no `baseline_ref`.
- A **baselined** debt row references a governed **baseline** record
  (`baseline_ref`) and carries no `suppression_ref`.
- A **new** debt row carries neither ref: it is newly introduced, ungoverned
  debt that remains release-visible.

A debt row that carries both refs, references a record the packet does not
declare, or whose state disagrees with its refs is a validation failure
(`DebtRowCarriesBothRefs`, `DanglingDebtRef`, `DebtStateRefMismatch`).

## Governed records keep scope and policy context

Every governed suppression carries scope, owner, reason, evidence, reopen rule,
and either an explicit `expires_at` **or** a `policy_lock_state_class` of
`expiry_managed_by_policy`. A suppression with no expiry that is not
policy-managed is a hidden permanent toggle and is denied
(`HiddenPermanentSuppression`). Baselines carry their compatible profile family,
accepted findings, owner, evidence/review refs, and compatibility state. Because
the records travel inside the packet, they round-trip into export, support, and
release projections without losing scope or policy context.

## Quality automation never becomes an invisible broad write

The save-participant guardrails keep fix-all and save-participant flows honest on
the new families:

- A participant that is **not** a safe local edit, or whose mutation scope is
  **broad** (whole-document, multi-file, generated family, or
  protected/policy-scoped), may never be marked `auto_apply`
  (`InvisibleBroadWrite`).
- Such a participant must route through preview or be blocked; it can never
  silently apply (`UnsafeWriteWithoutPreview`).
- A **generated-companion** update must run in the `generated_artifact_update`
  phase, not folded into a silent local format pass (`GeneratedPhaseMismatch`).
- A profile that reports policy overrides must surface them behind a
  `selected_winner` or `policy_overridden` winner state, never mask them behind
  an unrelated source state (`MaskedPolicyOverride`).
- The declared apply posture and preview requirement must agree with the boolean
  flags downstream surfaces render (`ApplyPostureInconsistent`,
  `PreviewFlagInconsistent`).

## Export and reconstruction

`export_projection()` produces a redaction-safe projection — per-family winning
profile, override/imported-config disclosure, ordered save action tokens, debt
counts by state, and the governed suppression/baseline refs — that support and
release packets can ingest directly instead of restating status by hand. The
model carries no raw source, raw tool arguments, raw paths, provider payloads, or
secrets.
