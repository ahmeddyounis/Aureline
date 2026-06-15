# M5 sideload review sheets — human-readable rendering

Human-readable rendering of the canonical M5 sideload review sheets. This row is a
depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at `artifacts/ecosystem/m5/m5-sideload-review.json`.

## Per-family sideload review sheet

| Family | Source | Signing | Binding | Claimed → rendered tier | Runtime / host-ABI | Triggers | Disposition |
| --- | --- | --- | --- | --- | --- | --- | --- |
| first_party_framework_pack | unpacked / workspace_relative_path | signed_verified | stay_local | enterprise_approved → **unsigned_local_only** | wasm_capability_sandbox / local_machine | — | reviewed_install_ready |
| docs_pack | archive / removable_media | signed_unverified | bind_to_registry_later | registry_bound → **unsigned_local_only** | passive_package / no_code_execution | — | reviewed_install_ready |
| local_model_pack | unpacked / user_home_relative_path | unsigned_sideload | stay_local | registry_bound → unsigned_local_only | external_host / external_process | external_executable_introduced | fresh_review_required |
| signed_recipe_pack | unpacked / workspace_relative_path | signed_verified | stay_local | verified_publisher → **unsigned_local_only** | wasm_capability_sandbox / local_machine | permission_widening, runtime_class_changed, host_or_abi_rebound | fresh_review_required |
| template_artifact | unpacked / workspace_relative_path | signed_unverified | bound_to_registry_identity | registry_bound → registry_bound | declarative_host_rendered_view / browser_runtime | — | reviewed_install_ready |
| bridge_backed_package | archive / network_mount | signed_verified | bind_to_registry_later | verified_publisher → **unsigned_local_only** | compatibility_bridge / managed_host | update_binding_changed, release_channel_changed | fresh_review_required |
| side_loaded_package | archive / removable_media | revoked_signature | stay_local | enterprise_approved → **unsigned_local_only** | remote_side_component / remote_target | — | **blocked** |
| mirrored_registry_variant | archive / process_stream | unsigned_sideload | stay_local | registry_bound → unsigned_local_only | wasm_capability_sandbox / managed_host | — | **blocked** |

## Non-inheritance and fresh-review holds

- **framework pack** — signed and verified, but built locally and staying local;
  renders `unsigned_local_only`, proving a local build never inherits a trusted badge
  just because the machine holds a trusted key.
- **recipe pack** — signed and verified, but its reload widens network egress and
  rebinds the runtime class and host; recomputes to `fresh_review_required`, so the
  widening cannot apply through a silent hot reload.
- **model pack** — its reload introduces a new external executable; held for a fresh
  review.
- **bridge-backed pack** — its reload changes the registry-binding decision and release
  channel; rebinding holds for a fresh review.
- **template artifact** — bound to a registry identity, so it may render
  `registry_bound` — but never a verified or enterprise badge.
- **side-loaded package** — signature revoked; `blocked` with the accept action
  disabled.
- **mirrored variant** — quarantined under anti-abuse review; `blocked` regardless of
  source.

## Summary

- 8 families, one sideload review sheet each.
- Dispositions: 3 `reviewed_install_ready`, 3 `fresh_review_required`, 2 `blocked`.
- 7 sheets render `unsigned_local_only`; only the bound-to-registry template renders
  `registry_bound`; none render a trusted-publisher badge.
- Bindings: 5 `stay_local`, 2 `bind_to_registry_later`, 1 `bound_to_registry_identity`.
- Every review trigger is exercised by at least one reload sheet.
