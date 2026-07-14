# M5 Repository-Bootstrap Shared Consumers: One Registry Across Surfaces

**Status:** Stable · B142 consumer-adoption lane
**Module:** `aureline_ui::m5_repository_bootstrap_shared_consumers_one_registry_across_surfaces`
**Schema:** [`schemas/workspaces/m5-repository-bootstrap-shared-consumers.schema.json`](../../schemas/workspaces/m5-repository-bootstrap-shared-consumers.schema.json)
**Proof:** [`artifacts/release/m5-repository-bootstrap-shared-consumers-proof/`](../../artifacts/release/m5-repository-bootstrap-shared-consumers-proof/)
**Fixtures:** [`fixtures/workspaces/m5-repository-bootstrap-shared-consumers/`](../../fixtures/workspaces/m5-repository-bootstrap-shared-consumers/)

This lane is the consumer-adoption capstone for the five governed acquisition families frozen in the
[repository-bootstrap matrix](m5_repository_bootstrap_contract.md) and implemented by the source-locator /
checkout-plan, credential-posture / fetch-route, staged-trust / post-open-queue, and acquisition-evidence /
partial-recovery lanes. It binds each shared repository-bootstrap family to the concrete acquisition-engine,
shell, workspace, git-service, trust-service, diagnostics, docs / help, CLI / export, and support-export
consumers that render it — the start-center, OS-open / system-association, CLI / headless, browser /
deep-link, and import entry surfaces — and proves, by fixtures rather than screenshots, that the same
acquisition profile presents the **same registry** everywhere it appears.

## Why this exists

The sheet already hardens workflow-bundle and workspace-admission behavior, project-entry components,
repository-topology and sparse/partial checkout disclosure, starter/scaffold preflight, and native desktop
external-path continuity, but it left Aureline's actual repository acquisition and bootstrap engine too
implicit for each claimed entry surface. This lane wires those rules into the daily-driver entry surfaces
so entry verbs, trust stages, and resumable-partial-root behavior cannot drift between the start center,
OS-open handoff, CLI / headless entry, deep-link handoff, import flow, help / docs, and support / export:
every entry surface consumes the shared registry rather than private wording or hand-copied bootstrap
copy. When two consumers describe the same acquisition state differently, the regression suite fails.

## The three honesty axes

1. **Reuse.** Each of the five repository-bootstrap families is adopted by **at least two distinct
   consumers**, so a family is proven shared acquisition-engine infrastructure rather than a one-surface
   fork of source-locator, checkout-plan, or bootstrap-evidence copy.
2. **One registry / no drift.** For a given acquisition profile every consumer surface presents the
   identical six-word grammar — `repository_bootstrap_role_word`, `family_word`,
   `registry_reference_word`, `entry_context_word`, `surface_context_word`, and
   `trust_stage_continuity_word`. The role word must be a token from the frozen
   `M5RepositoryBootstrapRole` vocabulary (`source_locator`, `checkout_plan`, `credential_posture`,
   `evidence_packet`, `staged_trust`, `resumable_acquisition`, `post_open_queue`), so no surface rewrites a
   role in its own words. A surface may narrow *how much* it shows across desktop, compact, remote, and
   exported representations, but never reword the grammar per surface — and a role that carries
   credential-posture, evidence-packet, staged-trust, or post-open-queue meaning may never let a surface
   rewrite clone into open because a local checkout already exists, run a repo-owned action implicitly
   during acquisition, lose signer or mirror provenance across an offline or mirrored fetch, strand a
   partial acquisition without Resume / Discard / open-read-only-partial-root choices, or hide the
   bootstrap credential posture behind generic connected-state copy.
3. **Map back to one family.** Support and CLI/export consumers point at the canonical per-domain schema
   and the frozen matrix by id, so an exported packet always maps an entry surface back to one shared
   contract family.

## Guardrails (each MUST be false on every binding)

- `rewrites_clone_into_open_when_local_checkout_already_exists`
- `runs_repo_owned_actions_implicitly_during_acquisition`
- `loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches`
- `strands_partial_acquisition_without_resume_discard_or_readonly_choices`
- `hides_bootstrap_credential_posture_behind_generic_connected_state_copy`

## Narrowing is disclosed, never hidden

A compact, remote, or exported representation carries an explicit `narrow_note` naming the reason, the
preserved grammar, and the next action; a remote representation names its remote source, and an exported
representation names its export-safe detail boundary rather than collapsing the profile out of view. When a
route only supports layout/context reopen, local inspect, or a read-only partial root rather than a full
resumable acquisition, the narrowing is surfaced consistently. Stale proof or a missing canonical reference
**narrows** the claim via a `RepositoryBootstrapSharedConsumersDowngradeTrigger` rather than hiding the
family.

## Seeded coverage

Five acquisition profiles — one per family — fan out to fifteen consumer bindings covering all nine
consumers and all four representations:

| Family | Role | Consumers |
| --- | --- | --- |
| `open_local` | `source_locator` | acquisition engine, shell, CLI export |
| `clone_remote` | `credential_posture` | git service, shell, support export |
| `open_archive` | `evidence_packet` | diagnostics, acquisition engine, workspace service |
| `import_bundle` | `staged_trust` | trust service, diagnostics, docs/help |
| `resume_snapshot` | `post_open_queue` | docs/help, workspace service, support export |

Two checked narrowed fixtures prove the grammar survives compact / remote and exported / redacted forms
without rewording.

## Regenerating the proof

```text
cargo run -p aureline-ui --example dump_m5_repository_bootstrap_shared_consumers -- support-export
cargo run -p aureline-ui --example dump_m5_repository_bootstrap_shared_consumers -- csv
cargo run -p aureline-ui --example dump_m5_repository_bootstrap_shared_consumers -- report
cargo run -p aureline-ui --example dump_m5_repository_bootstrap_shared_consumers -- fixture-compact-remote-narrowed
cargo run -p aureline-ui --example dump_m5_repository_bootstrap_shared_consumers -- fixture-exported-redaction-narrowed
```

The example is the only mint-from-truth path for the checked support export, matrix CSV, Markdown summary,
and narrowed fixtures; the module tests fail if any drifts from the seed builder.
