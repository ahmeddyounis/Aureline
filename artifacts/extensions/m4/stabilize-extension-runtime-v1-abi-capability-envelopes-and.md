# Artifact: Stabilize extension runtime v1 ABI, capability envelopes, and host isolation for the stable line

**Task:** Promote the extension runtime v1 beta admission contract into the stable line — pin the ABI contract version, publish the runtime-class vocabulary and sandbox-profile/backend truth with a fail-closed downgrade, enforce capability envelopes that never widen, and expose an active-contribution inspector and a downgraded-host banner — and derive the stability qualification with automatic narrowing below Stable.
**Status:** Implemented
**Verification class:** Conformance / interoperability suite + Security / privacy review + Docs validation + Release evidence review

## Summary

This lane binds the stabilized runtime contract identity, the published runtime class, the enforced sandbox profile (with its backend classification and fail-closed enforcement state), a non-widening capability envelope, the host-isolation posture, an activation budget, and the active-contribution inspector entries into one validated packet, and derives the stability qualification it may claim. A `stable` runtime claim is only allowed when the row pins the published ABI contract version, is enforcement-backed, enforces its sandbox profile as published, keeps its publisher trust tier out of quarantine, stays on a runnable lifecycle, keeps every contribution nominal, holds a bounded activation cost, and is fully attributed. A claimed sandboxed runtime class can never silently widen to ambient full-user execution, and a platform/backend that cannot enforce the published profile fails closed (`fail_closed_downgraded` or `unenforceable_refused`) and raises a downgraded-host banner. When any condition fails the visible tier is automatically narrowed below Stable (to `beta`, `preview`, or `withdrawn`) with machine-readable reasons. The checked-in packet is canonical: install review, the runtime inspector, the quarantine flow, diagnostics, and support exports ingest it instead of inventing a generic extension badge.

## What changed

- New Rust module: `crates/aureline-extensions/src/stabilize_extension_runtime_v1_abi_capability_envelopes_and/mod.rs` (+ `tests.rs`)
- Re-exported from `crates/aureline-extensions/src/lib.rs`
- New schema: `schemas/extensions/stable_runtime_abi.schema.json`
- New fixtures: `fixtures/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and/`
  - `wasm_capability_sandbox_stable_current.json` — a Wasm capability-sandbox formatter that enforces its published profile and holds Stable.
  - `external_host_fail_closed_downgraded_narrows_to_beta.json` — an external-host language server whose published seatbelt profile is unavailable; it falls back to a narrower process jail, narrows to `beta`, and raises a downgraded-host banner.
  - `remote_side_component_unenforceable_withdrawn.json` — a remote-side component whose host cannot attest the enforcement envelope; the claim is `withdrawn` and a banner is raised.
  - `compatibility_bridge_quarantined_contribution_narrows_to_preview.json` — a compatibility-bridge theme with a quarantined contribution; it narrows to `preview` while keeping the bridged and quarantined contributions inspectable.
  - `declarative_view_catalog_asserted_narrows_to_preview.json` — a host-rendered declarative view claiming Stable on catalog assertion alone; it narrows to `preview`.
- New dump example: `crates/aureline-extensions/examples/dump_stable_runtime_abi_records.rs`
- New docs: `docs/extensions/m4/stabilize-extension-runtime-v1-abi-capability-envelopes-and.md`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet are current and self-describing (schema, fixtures, and docs reference one another) rather than ad hoc notes. (`stable_runtime_abi.schema.json`, fixtures dir, this packet)
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable, with machine-readable reasons, instead of inheriting an adjacent green row. (`external_host_fail_closed_downgraded_narrows_to_beta.json`, `remote_side_component_unenforceable_withdrawn.json`, `compatibility_bridge_quarantined_contribution_narrows_to_preview.json`, `declarative_view_catalog_asserted_narrows_to_preview.json`, `abi_version_mismatch_narrows_below_stable`)
- [x] Users and admins can inspect permissions, compatibility range (ABI version + runtime class), activation cost, lifecycle label, publisher provenance, and rollback/revocation state (lifecycle + quarantine) for the touched ecosystem row. (`stable_runtime_abi_inspection`, `stable_active_contribution_inspector_entry`)
- [x] Install review, runtime inspector, quarantine flow, diagnostics, and support export all name the enforced sandbox profile/backend class or a narrower-than-stable downgrade. (`consumer_surfaces`, `stable_sandbox_profile_binding`, `stable_runtime_abi_support_export`)
- [x] Runtime-class audits prove stable surfaces, diagnostics, and support exports preserve contribution attribution and downgraded-host truth instead of collapsing into a generic extension badge. (`every_fixture_builds_validates_and_matches_expectations`, `support_export_quotes_runtime_class_and_profile`)

## Guardrails honored

- No ambient extension privilege: a claimed sandboxed runtime class with `widens_to_ambient_full_user == true` is rejected at construction (`sandboxed_class_widening_to_ambient_is_rejected`), and the flag is surfaced (always `false`) on the inspection row.
- Fail-closed, never silent: a platform/backend that cannot enforce the published profile fails closed (`fail_closed_downgraded` narrows to `beta`; `unenforceable_refused` withdraws the claim) and always raises a downgraded-host banner (`quarantined_trust_tier_raises_banner_and_narrows`, `honest_beta_claim_passes_through`).
- No catalog-only trust: a `catalog_asserted_only` basis can never back a stable claim (`declarative_view_catalog_asserted_narrows_to_preview.json`, `no_catalog_only_stable_claim`).
- No unbounded activation cost: an `unbounded_refused` budget withdraws the claim (`unbounded_activation_cost_withdraws_the_claim`).
- No capability widening: `granted ⊆ negotiated ⊆ declared` is enforced (`capability_envelope_widening_is_rejected`).
- A narrower stable claim is published rather than papered over: the effective tier, downgrade flag, reasons, and banner are re-derived from the posture at validation time, so the packet cannot drift.

## How to verify

```bash
cargo test -p aureline-extensions stabilize_extension_runtime_v1_abi
cargo run -q -p aureline-extensions --example dump_stable_runtime_abi_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_runtime_abi.schema.json` (checked with a Draft 2020-12 validator).

## Risks / follow-ups

- The sandbox enforcement state and activation-budget class are supplied by the producing host. When a wall-clock enforcement probe and a budget meter land, the narrowing should be derived from the live probe versus the published profile rather than a producer-supplied class.
- Runtime-class, backend, locus, trust, and lifecycle vocabularies are closed string sets shared with the beta runtime, SDK v1, and supervision lanes; when those crates stabilize typed enums, these should be narrowed to share them rather than re-declared as strings.
- The capability envelope consumes producer-supplied declared/negotiated/granted ref sets; a later revision should source the negotiated set directly from the runtime v1 admission contract this packet stabilizes instead of accepting a parallel copy.
