# Stable external-host contracts for language tools, debuggers, CLIs, and database / infra adapters

**Status:** Stable extension-runtime lane — implemented in `crates/aureline-extensions`.

## Goal

Stabilize the **external-host contract** an extension carries when it drives an out-of-process language server, debug adapter, CLI helper, or database / infrastructure adapter. The runtime v1 *beta* admission contract owns the per-session admission decision, and the stable runtime-ABI lane owns the published runtime truth; this lane owns the layer those leave open for hosts that run as a separately supervised process or a remote / managed endpoint.

Two things make external hosts different from a Wasm capability sandbox, and both are first-class here:

1. A **typed data-plane contract** for database / infra adapters — connection / target class, auth-source mode, read-only-versus-write-capable posture, local / tunneled / remote / managed origin, result / export safety, and control-plane-boundary truth are typed fields, not adapter-local strings.
2. **Reconnect / replay honesty** — after a host restart or reconnect, an external host must never silently re-run a query, an apply-capable action, or a control-plane mutation. A reattach whose pending work has possible side effects must require an explicit replay or review path.

As with the rest of the stable line, the **stability qualification** is derived, not asserted: when the host can no longer back a `stable` external-host claim, the visible tier is **automatically narrowed below Stable** (`beta`, `preview`, or `withdrawn`) with machine-readable reasons. Install review, the runtime inspector, the quarantine flow, diagnostics, and support exports ingest this packet instead of inventing a generic extension badge.

## Design principles

1. **Published host-kind vocabulary** — Every packet carries one of `language_tool`, `debug_adapter`, `cli_tool`, `database_adapter`, `infra_adapter`, plus its execution locus and host protocol. No "runs somewhere" badge.
2. **Sandbox and backend truth** — Every external host publishes a `sandbox_profile_id` and a backend classification and names the enforcement state. The published profile is enforced, or the row fails closed.
3. **Fail-closed downgrade, never silent widening** — `fail_closed_downgraded` (narrower profile) narrows to `beta`; `unenforceable_refused` withdraws the claim. An external host can **never** set `widens_to_ambient_full_user` — that is rejected at construction.
4. **Capability envelopes never widen** — `granted ⊆ negotiated ⊆ declared` is enforced.
5. **Typed data-plane contract** — A database / infra adapter **must** carry a data-plane contract; any other host kind **must not**. `ambient_environment` auth, `unbounded_unsafe` export, and an `unguarded_mutating` control plane can never ride the stable line (the first narrows to `preview`; the latter two `withdrawn`). A `read_only` adapter declaring a mutating control plane is a self-contradiction and is rejected.
6. **Reconnect / replay honesty** — Connection state is reported truthfully. A reattach with `apply_capable` or `control_plane_mutation` pending may not claim a `stateless_safe_resume`; it must require an explicit replay or review path, and silently re-running side effects is rejected at construction. A dirty / quarantined connection or a side-effecting reattach pending review narrows below Stable.
7. **No prose / catalog-only stable claim** — A `stable` tier must be `enforcement_backed`; a `catalog_asserted_only` basis narrows to `preview`.
8. **Active-contribution inspector stays attributable** — Each contribution always carries source package, kind, trust tier, used permissions, and last-known-good host — even when quarantined, failed, or downgraded.
9. **Bounded activation cost** — An `unbounded_refused` budget withdraws the claim.

## Record kinds

| Record kind | Purpose |
|---|---|
| `stable_external_host_contract_packet` | Top-level packet consumed by install review, the runtime inspector, the quarantine flow, diagnostics, support export, docs/help, and release packets. |
| `stable_external_host_identity` | Stabilized runtime contract ref, pinned ABI version, source package, publisher trust tier, lifecycle state. |
| `stable_external_host_kind_declaration` | Host kind, execution locus, host protocol. |
| `stable_external_host_sandbox_binding` | `sandbox_profile_id`, backend classification, enforcement state, platform backend, ambient-widening flag (always false). |
| `stable_external_host_capability_envelope` | Declared / negotiated / granted capability refs with the no-widening invariant. |
| `stable_external_host_data_plane_contract` | Typed connection / target, auth-source, write posture, origin, result/export safety, and control-plane boundary for an adapter. |
| `stable_external_host_reconnect_replay_safety` | Connection state, restart posture, quarantine flag, pending reattach side-effect class, reattach policy, and the (always-false) silent-rerun flag. |
| `stable_external_host_activation_budget` | Activation-budget class plus declared/observed cost refs. |
| `stable_external_host_contribution_entry` | Per-contribution attribution that survives quarantine / failure / downgrade. |
| `stable_external_host_qualification_claim` | Claimed tier, effective tier after the posture is applied, support claim, narrowing reasons. |
| `stable_external_host_downgraded_banner` | Whether a downgraded-host banner must display, why, and the last-known-good host. |
| `stable_external_host_contract_inspection` | Compact boolean projection for CLI and inspector surfaces. |
| `stable_external_host_contract_support_export` | Metadata-safe support / partner export row. |

## Closed vocabularies

### Host kinds
`language_tool`, `debug_adapter`, `cli_tool`, `database_adapter`, `infra_adapter` (the last two require a data-plane contract)

### Host protocols
`language_server_protocol`, `debug_adapter_protocol`, `cli_invocation`, `database_wire_protocol`, `infra_control_api`, `custom_host_protocol`

### Execution loci
`dedicated_subprocess`, `helper_binary`, `remote_agent`, `managed_service_endpoint`

### Backend classifications
`os_process_sandbox`, `seatbelt_sandbox_profile`, `landlock_seccomp_profile`, `app_container_profile`, `remote_enforced_envelope`

### Data-plane axes
- **Connection / target:** `relational_database`, `document_database`, `key_value_store`, `message_broker`, `object_store`, `container_runtime`, `orchestrator_control_plane`, `cloud_resource_manager`, `secrets_manager`, `generic_endpoint`
- **Auth-source mode:** `workspace_credential_broker`, `host_managed_keychain`, `external_secret_reference`, `ephemeral_session_token`, `interactive_user_prompt`, `ambient_environment` (never stable)
- **Write posture:** `read_only`, `write_capable`, `control_plane_mutating`
- **Origin:** `local`, `tunneled`, `remote`, `managed`
- **Result / export safety:** `bounded_redacted`, `bounded_full`, `streamed_capped`, `unbounded_unsafe` (never stable)
- **Control-plane boundary:** `no_control_plane`, `read_only_observability`, `gated_apply_with_review`, `unguarded_mutating` (never stable)

### Reconnect / replay axes
- **Connection state:** `connected_nominal`, `disconnected_clean`, `reconnect_pending`, `disconnected_dirty`, `quarantined`
- **Reattach policy:** `stateless_safe_resume`, `explicit_replay_required`, `explicit_review_required`, `blocked_pending_operator`
- **Pending reattach side effect:** `none`, `query_results_only`, `apply_capable`, `control_plane_mutation` (the last two are side-effecting)

### Stability tiers
`stable`, `beta`, `preview`, `withdrawn` (only `stable` is a stable external-host claim)

### Narrowing reasons
`abi_version_mismatch`, `catalog_only_trust_not_enforcement_backed`, `sandbox_fail_closed_downgraded`, `sandbox_unenforceable`, `trust_tier_quarantined`, `lifecycle_not_runnable`, `contribution_quarantined`, `contribution_host_downgraded`, `activation_cost_over_budget`, `activation_cost_unbounded`, `attribution_incomplete`, `ambient_auth_source`, `unbounded_result_export`, `unguarded_control_plane_mutation`, `connection_state_dirty`, `reattach_review_pending`

## Key invariants

- A `stable` effective tier requires the published ABI version, `enforcement_backed`, `enforced_as_published`, a non-quarantined trust tier, a runnable lifecycle, no quarantined/failed/downgraded contribution, a bounded activation cost, complete attribution, an honest (non-dirty) connection, no side-effecting reattach pending review, and — for an adapter — non-ambient auth, a bounded export, and a non-`unguarded_mutating` control plane.
- The host-kind → execution-locus, host-kind → protocol, and locus → backend mappings are enforced (a managed endpoint must use a remote-enforced envelope, a CLI cannot pretend to speak the debug-adapter protocol, etc.).
- An external host with `widens_to_ambient_full_user == true`, a capability envelope that widens, `silently_reruns_side_effects == true`, a side-effecting reattach claiming `stateless_safe_resume`, a missing/forbidden data-plane contract, or a `read_only` adapter with a mutating control plane is rejected at construction.
- The effective tier, downgrade flag, narrowing reasons, and the downgraded-host banner are re-derived from the posture at validation time, so a stored packet cannot drift from its evidence.
- `allows_ambient_full_user_widening`, `allows_catalog_only_trust`, `allows_unbounded_activation_cost`, `allows_silent_host_downgrade`, and `allows_silent_side_effect_replay` are forced `false` and validated.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-extensions/src/stabilize_external_host_contracts_for_language_tools_debuggers/mod.rs` |
| Schema | `schemas/extensions/stable_external_host_contract.schema.json` |
| Fixtures | `fixtures/extensions/m4/stabilize-external-host-contracts-for-language-tools-debuggers/` |
| Tests | `crates/aureline-extensions/src/stabilize_external_host_contracts_for_language_tools_debuggers/tests.rs` |
| Dump example | `crates/aureline-extensions/examples/dump_stable_external_host_contract_records.rs` |
| Proof packet | `artifacts/extensions/m4/stabilize-external-host-contracts-for-language-tools-debuggers.md` |

## Integration with existing lanes

- Sits above the runtime v1 *beta* admission contract (`crates/aureline-extensions/src/runtime/`) and beside the stable runtime-ABI lane (`crates/aureline-extensions/src/stabilize_extension_runtime_v1_abi_capability_envelopes_and/`): those own the runtime class and host isolation truth; this lane specializes the `external_host` runtime class for the four host kinds and adds the typed data-plane and reconnect / replay contracts. The `identity.runtime_contract_ref` points back at the beta contract (`runtime_v1_beta:` prefix).
- Reuses the same trust-tier, lifecycle, sandbox-enforcement, activation-budget, and claim-basis vocabularies the runtime, supervision, and marketplace lanes carry, so support and review surfaces share one external-host vocabulary.

## Verification

```bash
cargo test -p aureline-extensions stabilize_external_host
cargo run -q -p aureline-extensions --example dump_stable_external_host_contract_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_external_host_contract.schema.json` (checked with a Draft 2020-12 validator).
