# Stable extension runtime v1 ABI, capability envelopes, and host isolation

**Status:** Stable extension-runtime lane — implemented in `crates/aureline-extensions`.

## Goal

Promote the extension runtime v1 *beta* admission contract into the **stable line**. Every claimed stable ecosystem row carries one canonical, checked-in runtime truth: the pinned ABI contract version, the published runtime class, the enforced sandbox profile and its backend classification, a capability envelope that never widens, the host-isolation posture, an activation budget, and an active-contribution inspector. The **stability qualification** that truth is allowed to claim is derived, not asserted: when the host can no longer back a `stable` runtime claim, the visible tier is **automatically narrowed below Stable** (`beta`, `preview`, or `withdrawn`) with machine-readable reasons. Install review, the runtime inspector, the quarantine flow, diagnostics, and support exports ingest this packet instead of inventing a generic extension badge.

## Design principles

1. **Published runtime-class vocabulary** — Every record and every contribution carries one of `passive_package`, `wasm_capability_sandbox`, `declarative_host_rendered_view`, `external_host`, `compatibility_bridge`, `remote_side_component`. No "runs somewhere" badge.
2. **Sandbox-profile and backend truth** — A sandboxed runtime class publishes a `sandbox_profile_id` and a backend classification, and names the enforcement state. The published profile is enforced, or the row fails closed.
3. **Fail-closed downgrade, never silent widening** — When a platform/backend cannot enforce the published profile the enforcement state is `fail_closed_downgraded` (still sandboxed, narrower profile) or `unenforceable_refused` (no safe profile, the row is withdrawn). A claimed sandboxed runtime class can **never** set `widens_to_ambient_full_user` — that is rejected at construction.
4. **Capability envelopes never widen** — `granted ⊆ negotiated ⊆ declared` is enforced; granting beyond the negotiated set is rejected.
5. **No prose/catalog-only stable claim** — A `stable` runtime tier must be `enforcement_backed`; a `catalog_asserted_only` basis narrows below Stable.
6. **Active-contribution inspector stays attributable** — Each contribution always carries source package, runtime class, execution locus, trust tier, used permissions, and last-known-good host — even when quarantined, bridged, or running on a narrower profile.
7. **Downgraded-host banner** — Any fail-closed downgrade, quarantine, contribution downgrade, or quarantined trust tier raises a banner that names the reason and the last-known-good host.
8. **Bounded activation cost** — The activation budget is instrumented; an `unbounded_refused` budget can never ride the stable line (it withdraws the claim).

## Record kinds

| Record kind | Purpose |
|---|---|
| `stable_runtime_abi_packet` | Top-level packet consumed by install review, the runtime inspector, the quarantine flow, diagnostics, support export, docs/help, and release packets. |
| `stable_runtime_abi_identity` | Stabilized runtime contract ref, pinned ABI contract version, source package, publisher trust tier, lifecycle state. |
| `stable_runtime_class_declaration` | Published runtime class and execution locus. |
| `stable_sandbox_profile_binding` | `sandbox_profile_id`, backend classification, enforcement state, platform backend, ambient-widening flag (always false). |
| `stable_capability_envelope` | Declared / negotiated / granted capability refs with the no-widening invariant. |
| `stable_host_isolation_posture` | Isolation boundary, restart posture, restart attempts. |
| `stable_runtime_activation_budget` | Activation-budget class plus declared/observed cost refs. |
| `stable_active_contribution_inspector_entry` | Per-contribution attribution that survives quarantine/bridge/downgrade. |
| `stable_runtime_abi_qualification_claim` | Claimed tier, effective tier after the posture is applied, support claim, narrowing reasons. |
| `stable_downgraded_host_banner` | Whether a downgraded-host banner must display, why, and the last-known-good host. |
| `stable_runtime_abi_inspection` | Compact boolean projection for CLI and inspector surfaces. |
| `stable_runtime_abi_support_export` | Metadata-safe support/partner export row. |

## Closed vocabularies

### Runtime classes
`passive_package`, `wasm_capability_sandbox`, `declarative_host_rendered_view`, `external_host`, `compatibility_bridge`, `remote_side_component`

### Execution loci
`editor_in_process_isolated`, `dedicated_subprocess`, `helper_binary`, `remote_agent`, `bridged_foreign_runtime`, `host_rendered_no_extension_code`, `passive_no_execution`

### Backend classifications
`wasm_component_model`, `wasm_core_module`, `os_process_sandbox`, `seatbelt_sandbox_profile`, `landlock_seccomp_profile`, `app_container_profile`, `remote_enforced_envelope`, `bridge_translated_profile`, `none_passive`

### Sandbox enforcement states
`enforced_as_published`, `fail_closed_downgraded`, `unenforceable_refused`

### Stability tiers
`stable`, `beta`, `preview`, `withdrawn` (only `stable` is a stable runtime claim)

### Narrowing reasons
`abi_version_mismatch`, `catalog_only_trust_not_enforcement_backed`, `sandbox_fail_closed_downgraded`, `sandbox_unenforceable`, `trust_tier_quarantined`, `lifecycle_not_runnable`, `contribution_quarantined`, `contribution_host_downgraded`, `activation_cost_over_budget`, `activation_cost_unbounded`, `attribution_incomplete`

## Key invariants

- A `stable` effective tier requires `abi_contract_version == published`, `claim_basis_class == enforcement_backed`, `enforcement_state_class == enforced_as_published`, a non-quarantined trust tier, a runnable lifecycle, no quarantined/failed/downgraded contribution, a bounded activation cost, and complete attribution.
- The runtime-class → execution-locus mapping and the runtime-class → backend-classification mapping are enforced (a Wasm sample cannot pretend to run under a remote envelope, etc.).
- A claimed sandboxed runtime class with `widens_to_ambient_full_user == true`, or a capability envelope that widens, is rejected at construction.
- The effective tier, downgrade flag, narrowing reasons, and the downgraded-host banner are re-derived from the posture at validation time, so a stored packet cannot drift from its evidence.
- `allows_ambient_full_user_widening`, `allows_catalog_only_trust`, `allows_unbounded_activation_cost`, and `allows_silent_host_downgrade` are forced `false` and validated.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-extensions/src/stabilize_extension_runtime_v1_abi_capability_envelopes_and/mod.rs` |
| Schema | `schemas/extensions/stable_runtime_abi.schema.json` |
| Fixtures | `fixtures/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and/` |
| Tests | `crates/aureline-extensions/src/stabilize_extension_runtime_v1_abi_capability_envelopes_and/tests.rs` |
| Dump example | `crates/aureline-extensions/examples/dump_stable_runtime_abi_records.rs` |
| Proof packet | `artifacts/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and.md` |

## Integration with existing lanes

- Sits above the runtime v1 *beta* admission contract (`crates/aureline-extensions/src/runtime/`): that module owns the per-session admission decision; this module owns the published, stable runtime-ABI truth and its stability qualification. The `identity.runtime_contract_ref` points back at the beta contract (`runtime_v1_beta:` prefix).
- Reuses the same runtime-class shapes that the SDK v1 starter pack, the host-isolation supervision lane, and the marketplace fact grid carry, so support and review surfaces share one runtime vocabulary.

## Verification

```bash
cargo test -p aureline-extensions stabilize_extension_runtime_v1_abi
cargo run -q -p aureline-extensions --example dump_stable_runtime_abi_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_runtime_abi.schema.json` (checked with a Draft 2020-12 validator).
