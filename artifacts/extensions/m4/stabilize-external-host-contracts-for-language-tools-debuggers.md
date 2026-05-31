# Artifact: Stabilize external-host contracts for language tools, debuggers, CLIs, and database / infra adapters

**Task:** Stabilize the external-host contract for the four out-of-process host kinds — language tools, debuggers, CLIs, and database / infrastructure adapters. Make the database / infra data-plane contract (connection / target class, auth-source mode, read-only-versus-write-capable posture, local / tunneled / remote / managed origin, result / export safety, control-plane-boundary truth) first-class typed fields, add a reconnect / replay safety contract that keeps connection state honest and never silently re-runs a query, apply-capable action, or control-plane mutation after a host restart, and derive the stability qualification with automatic narrowing below Stable.
**Status:** Implemented
**Verification class:** Conformance / interoperability suite + Security / privacy review + Docs validation + Release evidence review

## Summary

This lane binds the stabilized runtime contract identity, the external-host kind (language tool / debug adapter / CLI / database adapter / infra adapter) with its execution locus and protocol, the enforced sandbox binding (with backend classification and fail-closed enforcement state), a non-widening capability envelope, an optional typed data-plane contract (required for database / infra adapters and forbidden otherwise), a reconnect / replay safety record, an activation budget, and the active-contribution inspector entries into one validated packet, and derives the stability qualification it may claim.

A `stable` external-host claim is only allowed when the row pins the published ABI contract version, is enforcement-backed, enforces its sandbox profile as published, keeps its publisher trust tier out of quarantine, stays runnable, keeps every contribution nominal, holds a bounded activation cost, is fully attributed, keeps an honest (non-dirty) connection, has no side-effecting reattach pending review, and — for an adapter — sources auth from a managed broker (never ambient), bounds its result / export, and never exposes an unguarded mutating control plane. When any condition fails the visible tier is automatically narrowed below Stable (`beta`, `preview`, or `withdrawn`) with machine-readable reasons. The checked-in packet is canonical: install review, the runtime inspector, the quarantine flow, diagnostics, and support exports ingest it instead of inventing a generic extension badge.

## What changed

- New Rust module: `crates/aureline-extensions/src/stabilize_external_host_contracts_for_language_tools_debuggers/mod.rs` (+ `tests.rs`)
- Re-exported from `crates/aureline-extensions/src/lib.rs`
- New schema: `schemas/extensions/stable_external_host_contract.schema.json`
- New fixtures: `fixtures/extensions/m4/stabilize-external-host-contracts-for-language-tools-debuggers/`
  - `database_adapter_read_only_stable_current.json` — a verified read-only relational-database adapter on a managed endpoint that enforces its published profile and holds Stable.
  - `infra_adapter_unguarded_control_plane_withdrawn.json` — an orchestrator control-plane adapter exposing an unguarded mutating control plane; the claim is `withdrawn` and a banner is raised.
  - `cli_tool_catalog_asserted_narrows_to_preview.json` — a CLI formatter claiming Stable on catalog assertion alone; it narrows to `preview`.
  - `language_tool_fail_closed_downgraded_narrows_to_beta.json` — a language server whose published seatbelt profile is unavailable; it falls back to a narrower process jail, narrows to `beta`, and raises a banner.
  - `debug_adapter_quarantined_contribution_narrows_to_preview.json` — a debug adapter with one quarantined session; it narrows to `preview` while keeping both sessions inspectable.
  - `database_adapter_dirty_reconnect_review_pending_narrows_to_preview.json` — a write-capable SQL runner with a dirty disconnect and a pending apply awaiting operator review; it narrows to `preview` and never silently replays the apply.
- New dump example: `crates/aureline-extensions/examples/dump_stable_external_host_contract_records.rs`
- New docs: `docs/extensions/m4/stabilize-external-host-contracts-for-language-tools-debuggers.md`

## Review-pass delta requirements

- [x] The external-host capability contract for database / infra adapters carries connection / target class, auth-source mode, read-only-versus-write-capable posture, local / tunneled / remote / managed origin, result / export safety, and control-plane-boundary truth as first-class typed fields (`stable_external_host_data_plane_contract`), not adapter-local strings. (`ExternalHostDataPlaneContract`, `database_adapter_read_only_stable_current.json`)
- [x] Quarantine / restart / reconnect behavior preserves connection-state honesty and never silently re-runs a query, apply-capable action, or control-plane mutation after host restart; a reattach with possible side effects requires an explicit replay or review path. (`ExternalHostReconnectReplaySafety`; `silent_side_effect_replay_is_rejected`, `side_effecting_reattach_with_stateless_resume_is_rejected`, `database_adapter_dirty_reconnect_review_pending_narrows_to_preview.json`)

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet are current and self-describing (schema, fixtures, and docs reference one another) rather than ad hoc notes. (`stable_external_host_contract.schema.json`, fixtures dir, this packet)
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable, with machine-readable reasons, instead of inheriting an adjacent green row. (every narrowing fixture + `abi_version_mismatch_narrows_below_stable`, `ambient_auth_source_narrows_below_stable`, `unbounded_result_export_withdraws_the_claim`)
- [x] Users and admins can inspect permissions, compatibility range (ABI version + host kind/protocol), activation cost, lifecycle label, publisher provenance, data-plane posture, connection state, and rollback/revocation state (lifecycle + quarantine) for the touched ecosystem row. (`stable_external_host_contract_inspection`, `stable_external_host_contribution_entry`, `stable_external_host_data_plane_contract`)
- [x] Install review, runtime inspector, quarantine flow, diagnostics, and support export all name the enforced sandbox profile/backend, the data-plane posture, and the connection/reattach state or a narrower-than-stable downgrade. (`consumer_surfaces`, `stable_external_host_contract_support_export`)

## Guardrails honored

- No ambient extension privilege: an external host with `widens_to_ambient_full_user == true` is rejected (`external_host_widening_to_ambient_is_rejected`); ambient-environment adapter auth narrows below Stable (`ambient_auth_source_narrows_below_stable`).
- Fail-closed, never silent: a backend that cannot enforce the published profile fails closed (`fail_closed_downgraded` → `beta`; `unenforceable_refused` → `withdrawn`) and raises a banner.
- No catalog-only trust: a `catalog_asserted_only` basis can never back a stable claim (`cli_tool_catalog_asserted_narrows_to_preview.json`, `no_catalog_only_stable_claim`).
- No unbounded activation cost or export: an `unbounded_refused` budget or `unbounded_unsafe` export withdraws the claim.
- No unguarded control plane: an `unguarded_mutating` control plane withdraws the claim; a `read_only` adapter declaring a mutating control plane is rejected at construction.
- No silent reattach: silently re-running side effects is rejected, and a side-effecting reattach claiming a stateless safe resume is rejected.
- A narrower stable claim is published rather than papered over: the effective tier, downgrade flag, reasons, and banner are re-derived from the posture at validation time, so the packet cannot drift.

## How to verify

```bash
cargo test -p aureline-extensions stabilize_external_host
cargo run -q -p aureline-extensions --example dump_stable_external_host_contract_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_external_host_contract.schema.json` (checked with a Draft 2020-12 validator).

## Risks / follow-ups

- The sandbox enforcement state, connection state, and pending-reattach side-effect class are supplied by the producing host. When a live enforcement probe, a connection monitor, and a replay-journal meter land, the narrowing should be derived from the live signals rather than producer-supplied classes.
- Host-kind, protocol, backend, trust, lifecycle, data-plane, and reconnect vocabularies are closed string sets shared with the runtime, supervision, and marketplace lanes; when those crates stabilize typed enums, these should be narrowed to share them rather than re-declared as strings.
- The data-plane contract describes the declared posture; a later revision should reconcile the declared write posture and control-plane boundary against the actually-negotiated capability envelope so a `write_capable` posture cannot outrun its granted capabilities.
