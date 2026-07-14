# M5 Install-Topology Shared Consumers: One Registry Across Surfaces

**Status:** Stable · B140 consumer-adoption lane
**Module:** `aureline_ui::m5_install_topology_shared_consumers_one_registry_across_surfaces`
**Schema:** [`schemas/install/m5-install-topology-shared-consumers.schema.json`](../../schemas/install/m5-install-topology-shared-consumers.schema.json)
**Proof:** [`artifacts/release/m5-install-topology-shared-consumers-proof/`](../../artifacts/release/m5-install-topology-shared-consumers-proof/)
**Fixtures:** [`fixtures/install/m5-install-topology-shared-consumers/`](../../fixtures/install/m5-install-topology-shared-consumers/)

This lane is the consumer-adoption capstone for the five reusable delivery-topology families frozen in
the [install-topology matrix](m5_install_topology_contract.md) and implemented by the install-topology /
state-root registries, portable-mode state-containment, managed-deployment operations, and side-by-side
channel-isolation lanes. It binds each shared install-topology family to the concrete installer /
package-manager, About / shell, update-center / updater, diagnostics, admin, docs / help, CLI / export,
support-export, and general product / fleet-rollout consumers that render it, and proves — by fixtures,
not screenshots — that the same delivery profile presents the **same registry** everywhere it appears.

## Why this exists

The batch already hardens release artifact graphs, native desktop handler ownership, install-profile
and deployment-summary components, and the managed / self-hosted / offline continuity model, but it left
Aureline's actual install-topology behavior too implicit for each install-facing surface. This lane wires
those rules into the daily-driver distribution surfaces so channel, updater owner, state root, rollback
target, and rollout ring cannot drift between installer flows, About / update copy, diagnostics, admin
consoles, docs / screenshots, and support packets: every install-facing surface consumes the shared
registry rather than private wording or hand-copied installer notes.

## The three honesty axes

1. **Reuse.** Each of the five install-topology families is adopted by **at least two distinct
   consumers**, so a family is proven shared delivery-topology infrastructure rather than a one-surface
   fork of install-mode, updater-ownership, state-root, or rollback copy.
2. **One registry / no drift.** For a given delivery profile every consumer surface presents the
   identical six-word grammar — `install_topology_role_word`, `family_word`, `registry_reference_word`,
   `channel_word`, `surface_context_word`, and `ownership_identity_word`. The role word must be a token
   from the frozen `M5InstallTopologyRole` vocabulary (`install_mode`, `updater_owner`, `binary_root`,
   `writable_state_roots`, `policy_roots`, `rollback_target`, `rollout_ring`), so no surface rewrites a
   role in its own words. A surface may narrow *how much* it shows across desktop, compact, remote, and
   exported representations, but never reword the grammar per surface — and a role that carries
   updater-ownership or state-isolation meaning may never let a topology change hide who owns the
   updater, spill durable state into hidden machine-global paths, reuse a stable state namespace without
   a handoff, narrow rollback below the full artifact graph, or publish a deployment claim that outpaces
   ring or repair / verify evidence.
3. **Map back to one family.** Support and CLI/export consumers point at the canonical per-domain schema
   and the frozen matrix by id, so an exported packet always maps an install-topology surface back to
   one shared contract family.

## Guardrails (each MUST be false on every binding)

- `portable_mode_writes_hidden_machine_global_durable_state`
- `preview_channel_reuses_stable_state_namespace_without_handoff`
- `rollback_targets_primary_executable_while_sidecars_drift`
- `hides_updater_ownership_or_admin_control_in_managed_flow`
- `publishes_deployment_claim_outpacing_ring_or_repair_verify_evidence`

## Narrowing is disclosed, never hidden

A compact, remote, or exported representation carries an explicit `narrow_note` naming the reason, the
preserved grammar, and the next action; a remote representation names its remote source, and an exported
representation names its export-safe detail boundary rather than collapsing the profile out of view.
Stale proof or a missing canonical reference **narrows** the claim via an
`InstallTopologySharedConsumersDowngradeTrigger` rather than hiding the family.

## Seeded coverage

Five delivery profiles — one per family — fan out to fifteen consumer bindings covering all nine
consumers and all four representations:

| Family | Role | Consumers |
| --- | --- | --- |
| `per_user_managed` | `updater_owner` | update center, About / shell, CLI export |
| `per_machine_managed` | `policy_roots` | admin, installer, support export |
| `side_by_side_stable_preview` | `writable_state_roots` | diagnostics, About / shell, product |
| `portable_mode` | `install_mode` | docs/help, diagnostics, product |
| `offline_airgap_bundle` | `rollback_target` | admin, installer, support export |

Two checked narrowed fixtures prove the grammar survives compact / remote and exported / redacted forms
without rewording.

## Regenerating the proof

```text
cargo run -p aureline-ui --example dump_m5_install_topology_shared_consumers -- support-export
cargo run -p aureline-ui --example dump_m5_install_topology_shared_consumers -- csv
cargo run -p aureline-ui --example dump_m5_install_topology_shared_consumers -- report
cargo run -p aureline-ui --example dump_m5_install_topology_shared_consumers -- fixture-compact-remote-narrowed
cargo run -p aureline-ui --example dump_m5_install_topology_shared_consumers -- fixture-exported-redaction-narrowed
```

The example is the only mint-from-truth path for the checked support export, matrix CSV, Markdown
summary, and narrowed fixtures; the module tests fail if any drifts from the seed builder.
