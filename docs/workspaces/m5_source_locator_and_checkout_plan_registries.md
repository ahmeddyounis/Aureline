# M5 source-locator and checkout-plan registries

This lane is the first implement lane over the frozen
[M5 repository-bootstrap matrix](./m5_repository_bootstrap_contract.md). It turns the *source-locator* grammar
(open-local / open-archive) and the *checkout-plan* grammar (clone-remote) into registry resolvers that produce
export-safe, honest projections, so the shell, entry, diagnostics, admin, workspace, git, trust, docs, CLI, and
support surfaces resolve one canonical acquisition truth instead of a per-entry, hand-copied reconstruction.
The source locator and the checkout plan are separated in runtime and serialized state: the literal target,
resolved checkout root or archive container, staged-trust metadata, disclosed credential posture, and signer /
mirror provenance live on the source locator, while the ref selection, mode, depth / filter, submodule mode,
LFS posture, destination path, and cost band live on the checkout plan, and open and clone stay distinct verbs
so a clone is never silently rewritten into an open over an existing local checkout.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_source_locator_and_checkout_plan_registries` (the authoritative validator).
- **Combined schema:**
  `schemas/workspaces/m5-source-locator-and-checkout-plan-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/workspaces/m5-source-locator.schema.json`](../../schemas/workspaces/m5-source-locator.schema.json)
  and
  [`schemas/workspaces/m5-checkout-plan.schema.json`](../../schemas/workspaces/m5-checkout-plan.schema.json)
  as its canonical domain contracts.
- **Checked proof:** `artifacts/release/m5-source-locator-and-checkout-plan-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/workspaces/m5-source-locator-and-checkout-plan-registries/`
  (`local_path_source_beta_narrowed.json`, `sparse_checkout_preview_narrowed.json`).

## Two registries

1. **Source locator** (`resolve_source_locator_entry`) — publishes one stable source-locator object per entry
   flow: the source-locator kind and canonical locator mode, the literal target preserved verbatim, the
   resolved checkout root or archive container, the staged-trust metadata, the disclosed credential posture, the
   signer / mirror provenance, and the distinct mirror / air-gap hint. A clean entry names a canonical registry
   token, a classified source-locator kind, and a repository-bootstrap role, covers the canonical / accessible /
   audit resolution forms, publishes a complete object, preserves the literal target as a verb-faithful locator,
   and discloses the bootstrap credential posture before a network or mirror fetch. Otherwise it degrades
   honestly — a literal target rewritten into a different verb (or a network locator that hides its credential
   posture) degrades to `source_locator_rewrites_verb_or_hides_credential_posture`.
2. **Checkout plan** (`resolve_checkout_plan_entry`) — keeps the checkout plan safe. A clean entry names a
   classified checkout mode and provides the complete ref-selection / depth-filter / submodule-mode /
   LFS-posture / destination-path / cost-band checkout-plan object; a plan that would run a repo-owned action
   (hook, task, extension, package restore, submodule or LFS hydration, generator install) without staging it,
   hides checkout cost or topology before mutation, or asserts an implicit mutation it cannot explain degrades to
   `checkout_plan_runs_repo_owned_action_or_hides_cost`.

## Per-entry acquisition reference

The source-locator kind carries its canonical locator mode, and the resolver publishes the full locator object,
so the registry — never a hand-copied per-entry assumption — is the single source of truth.
`source_locator_object_is_complete` rejects an object missing any field,
`literal_target_stays_verb_preserving` rejects a verb rewrite or a hidden credential posture, and
`checkout_plan_stays_honest` rejects a plan that has become an implicit bootstrap.

| source-locator kind | locator mode | literal target | resolved root / container | trust-stage metadata | credential posture | signer / mirror provenance |
| --- | --- | --- | --- | --- | --- | --- |
| local path | local_path_locator | `local-path.acme/repo` | `checkout-root.acme/repo` | `trust-stage.staged.v3` | `credential-posture.not-required` | `signer-provenance.acme.v3` |
| remote forge / URL | remote_forge_url_locator | `remote-forge.acme/org/repo` | `checkout-root.acme/repo` | `trust-stage.staged.v3` | `credential-posture.disclosed` | `signer-provenance.acme.v3` |
| mirror source | mirror_source_locator | `mirror.acme/org/repo` | `checkout-root.acme/repo` | `trust-stage.staged.v3` | `credential-posture.disclosed` | `mirror-provenance.acme.v3` |

A verb rewrite degrades to `source_locator_rewrites_verb_or_hides_credential_posture`, an incomplete object
degrades to `source_locator_object_incomplete`, and an implicit bootstrap degrades to
`checkout_plan_runs_repo_owned_action_or_hides_cost`, so a verb rewrite, an incomplete object, or an implicit
bootstrap can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Every claimed entry flow resolves to one stable source-locator object with literal-target / resolved-root /
  trust-stage / credential-posture / provenance fields.** Clean locator entries cover the canonical local-path /
  remote-forge / archive / mirror / managed-snapshot kinds and the first shell / entry / diagnostics / admin /
  support surfaces, an object-incomplete example degrades, and no clean locator entry published an incomplete
  object.
- **Open and clone stay distinct verbs; the literal target is never rewritten into a different verb.** A
  verb-rewrite example and an unbound example degrade, a clean verb-preserving locator entry is present, and no
  clean entry lost the literal target.
- **The suite fails when a checkout plan collapses into an implicit bootstrap.** Clean checkout-plan entries
  cover the full / partial / sparse modes with full resolution-form coverage while providing the complete plan
  object, and a plan that would run a repo-owned action implicitly degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_source_locator_and_checkout_plan_registries -- support-export
cargo run -p aureline-ui --example dump_m5_source_locator_and_checkout_plan_registries -- csv
cargo run -p aureline-ui --example dump_m5_source_locator_and_checkout_plan_registries -- report
cargo run -p aureline-ui --example dump_m5_source_locator_and_checkout_plan_registries -- source-acquisition-table
cargo run -p aureline-ui --example dump_m5_source_locator_and_checkout_plan_registries -- fixture-local-path-source-beta-narrowed
cargo run -p aureline-ui --example dump_m5_source_locator_and_checkout_plan_registries -- fixture-sparse-checkout-preview-narrowed
```
